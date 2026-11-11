/* S7Service.cpp — Siemens S7comm (RFC1006) client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/S7Service.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

namespace tplc {

namespace {
constexpr uint64_t NS_PER_MS = 1000000ull;

/* TPKT + COTP + S7 codec helpers (all big-endian on the wire) */

void put16(uint8_t* p, uint16_t v) { p[0] = uint8_t(v >> 8); p[1] = uint8_t(v); }
void put24(uint8_t* p, uint32_t v) { p[0] = uint8_t(v >> 16); p[1] = uint8_t(v >> 8); p[2] = uint8_t(v); }
uint16_t get16(const uint8_t* p) { return uint16_t(p[0]) << 8 | p[1]; }

/* wrap payload in TPKT: `03 00 <len16>` */
size_t wrap_tpkt(uint8_t* out, size_t cap, const uint8_t* payload, size_t n) {
    if (n + 4 > cap) return 0;
    out[0] = 0x03; out[1] = 0x00;
    put16(out + 2, uint16_t(n + 4));
    memcpy(out + 4, payload, n);
    return n + 4;
}

/* COTP Connection Request for S7 rack/slot.
   called TSAP = 0x0100 | (rack<<5) | slot  (snap7 convention) */
size_t build_cotp_cr(uint8_t* out, size_t cap, uint16_t rack, uint16_t slot) {
    if (cap < 23) return 0;
    uint16_t tsa = uint16_t(0x0100 | ((rack & 0x0F) << 5) | (slot & 0x1F));
    static const uint8_t pre[] = {
        0x06, 0xE0, 0x00, 0x00, 0x00, 0x01, 0x00,
        0xC1, 0x02, 0x01, 0x00,             /* calling TSAP 0x0100 */
        0xC2, 0x02, 0x00, 0x00,             /* called TSAP          */
        0xC0, 0x01, 0x0A};                  /* PDU size 2^10=1024   */
    memcpy(out, pre, sizeof(pre));
    out[13] = uint8_t(tsa >> 8);
    out[14] = uint8_t(tsa);
    return sizeof(pre);
}

/* COTP Data Transfer: `02 F0 80` + payload */
size_t build_cotp_dt(uint8_t* out, size_t cap, const uint8_t* payload, size_t n) {
    if (n + 3 > cap) return 0;
    out[0] = 0x02; out[1] = 0xF0; out[2] = 0x80;
    memcpy(out + 3, payload, n);
    return n + 3;
}

/* S7 Job PDU: 10-byte header `32 01 0000 ref plen dlen` +
   param + optional data section */
size_t build_s7_job(uint8_t* out, size_t cap, uint16_t ref,
                    const uint8_t* param, size_t plen,
                    const uint8_t* data, size_t dlen) {
    if (plen + dlen + 10 > cap) return 0;
    out[0] = 0x32; out[1] = 0x01;
    out[2] = 0; out[3] = 0;
    put16(out + 4, ref);
    put16(out + 6, uint16_t(plen));
    put16(out + 8, uint16_t(dlen));
    memcpy(out + 10, param, plen);
    if (dlen) memcpy(out + 10 + plen, data, dlen);
    return 10 + plen + dlen;
}

/* S7-ANY address spec item: `12 0A 10 ts count16 db16 area addr3`
   (12 bytes). The transport byte is the S7WordLen type code:
   0x01=BIT 0x02=BYTE 0x04=WORD ... — byte access uses BYTE so
   count is a byte count. bit access: ts=0x01 count=1
   addr=byte*8+bit; word access: ts=0x02 count=len addr=byte*8 */
size_t build_addr_item(uint8_t* out, const TplcS7Addr& a) {
    bool bit = a.bit_off != 0xFFFFFFFFu;
    uint8_t ts = bit ? 0x01 : 0x02;
    uint16_t count = bit ? 1 : uint16_t(a.len);
    uint32_t addr = a.byte_off * 8 + (bit ? a.bit_off : 0);
    out[0] = 0x12; out[1] = 0x0A; out[2] = 0x10;
    out[3] = ts;
    put16(out + 4, count);
    put16(out + 6, uint16_t(a.db));
    out[8] = uint8_t(a.area);
    put24(out + 9, addr & 0xFFFFFFu);
    return 12;
}

static bool parse_v4(const char* host, struct in_addr* out) {
    unsigned a, b, c, d;
    char tail;
    if (!host || sscanf(host, "%u.%u.%u.%u%c", &a, &b, &c, &d, &tail) != 4 ||
        a > 255 || b > 255 || c > 255 || d > 255)
        return false;
    out->s_addr = htonl((a << 24) | (b << 16) | (c << 8) | d);
    return true;
}

static int nonblock(int fd) {
    int fl = fcntl(fd, F_GETFL, 0);
    return fl < 0 ? -1 : fcntl(fd, F_SETFL, fl | O_NONBLOCK);
}

bool valid_area(uint32_t a) {
    return a == TPLC_S7_AREA_PE || a == TPLC_S7_AREA_PA ||
           a == TPLC_S7_AREA_MK || a == TPLC_S7_AREA_DB ||
           a == TPLC_S7_AREA_CT || a == TPLC_S7_AREA_TM;
}
} /* namespace */

S7Service* S7Service::self(void* p) { return static_cast<S7Service*>(p); }

S7Service::S7Service(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcS7Service);
    svc_.abi_version = TPLC_S7_ABI_VERSION;
    svc_.host = this;
    svc_.s7_open = [](void* s, const TplcS7OpenReq* r,
                      TplcS7ConnHandle* o) {
        return self(s)->open(s, r, o);
    };
    svc_.s7_conn_poll = [](void* s, TplcS7ConnHandle ch,
                           TplcS7ConnInfo* i) {
        return self(s)->connPoll(s, ch, i);
    };
    svc_.s7_read = [](void* s, TplcS7ConnHandle ch,
                      const TplcS7ReadReq* r, TplcS7IoHandle* o) {
        return self(s)->read(s, ch, r, o);
    };
    svc_.s7_write = [](void* s, TplcS7ConnHandle ch,
                       const TplcS7WriteReq* r, TplcS7IoHandle* o) {
        return self(s)->write(s, ch, r, o);
    };
    svc_.s7_poll = [](void* s, TplcS7IoHandle h, TplcS7RespInfo* i,
                      uint8_t* v, uint32_t vc) {
        return self(s)->poll(s, h, i, v, vc);
    };
    svc_.s7_abort = [](void* s, TplcS7IoHandle h) {
        return self(s)->abort(s, h);
    };
    svc_.s7_close = [](void* s, TplcS7ConnHandle ch) {
        return self(s)->connClose(s, ch);
    };
}

void S7Service::failConn(Conn& c, int err) {
    c.stage = Stage::Failed;
    c.err = err;
    if (c.active_io) {
        auto ii = ios_.find(c.active_io);
        if (ii != ios_.end()) {
            ii->second.info.status = TPLC_S7_ST_ERROR;
            ii->second.info.err = err;
            ii->second.finished = true;
        }
    }
}

bool S7Service::pumpTx(Conn& c) {
    while (c.tx_off < c.tx_len) {
        ssize_t w = send(c.fd, c.tx + c.tx_off, c.tx_len - c.tx_off,
                         MSG_DONTWAIT);
        if (w > 0) {
            c.tx_off += size_t(w);
        } else if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK ||
                             errno == EINTR)) {
            return true;   /* would block — retry next poll */
        } else {
            return false;  /* real error */
        }
    }
    c.tx_len = c.tx_off = 0;
    return true;
}

/* extract one complete TPKT frame's COTP payload; returns payload
   length, 0 = incomplete, -1 = malformed/overflow */
int S7Service::pumpRx(Conn& c, uint8_t* frame, size_t cap, size_t* flen) {
    for (;;) {
        if (c.rx_len >= 4) {
            if (c.rx[0] != 0x03) return -1;
            size_t total = size_t(get16(c.rx + 2));
            if (total < 7 || total > sizeof(c.rx)) return -1;
            if (c.rx_len >= total) {
                size_t n = total - 4;
                if (n > cap) return -1;
                memcpy(frame, c.rx + 4, n);
                size_t rem = c.rx_len - total;
                memmove(c.rx, c.rx + total, rem);
                c.rx_len = rem;
                *flen = n;
                return int(n);
            }
        }
        ssize_t r = recv(c.fd, c.rx + c.rx_len,
                         sizeof(c.rx) - c.rx_len, MSG_DONTWAIT);
        if (r > 0) {
            c.rx_len += size_t(r);
            if (c.rx_len >= sizeof(c.rx)) return -1;
            continue;
        }
        if (r == 0) return -2;         /* peer closed */
        if (errno != EAGAIN && errno != EWOULDBLOCK && errno != EINTR)
            return -2;
        return 0;                      /* nothing more right now */
    }
}

void S7Service::handleHandshake(Conn& c, const uint8_t* f, size_t n) {
    /* COTP payload: byte0=LI, byte1=type */
    if (n >= 2 && f[1] == 0xD0) {           /* Connection Confirm */
        /* queue S7 Setup Communication job */
        static const uint8_t param[] = {
            0xF0, 0x00, 0x00, 0x01, 0x00, 0x01, 0x01, 0xE0};
        uint8_t pdu[64];
        size_t pn = build_s7_job(pdu, sizeof(pdu), ++c.pdu_ref,
                                 param, sizeof(param), nullptr, 0);
        /* build into tx: DT(S7) wrapped in TPKT */
        uint8_t tmp[128];
        size_t tn = build_cotp_dt(tmp, sizeof(tmp), pdu, pn);
        size_t wn = wrap_tpkt(c.tx, sizeof(c.tx), tmp, tn);
        if (!wn) { failConn(c, TPLC_S7_E_RANGE); return; }
        c.tx_len = wn;
        c.stage = Stage::SetupWait;
        return;
    }
    if (n >= 3 && f[0] == 0x02 && f[1] == 0xF0 && f[2] == 0x80 &&
        c.stage == Stage::SetupWait) {
        /* S7 response PDU */
        const uint8_t* pdu = f + 3;
        size_t pn = n - 3;
        if (pn < 10 || pdu[0] != 0x32) { failConn(c, TPLC_S7_E_IO); return; }
        bool resp12 = pdu[1] == 0x02 || pdu[1] == 0x03;
        size_t hlen = resp12 ? 12 : 10;
        if (pn < hlen) { failConn(c, TPLC_S7_E_IO); return; }
        uint16_t plen = get16(pdu + 6), dlen = get16(pdu + 8);
        uint32_t errw = resp12 ? (uint32_t(pdu[10]) << 8 | pdu[11]) : 0;
        (void)plen; (void)dlen;
        if (errw != 0 || (pdu[1] != 0x02 && pdu[1] != 0x03)) {
            failConn(c, int(errw ? errw : TPLC_S7_E_IO));
            return;
        }
        c.stage = Stage::Ready;
    }
}

void S7Service::handleIoResp(Io& io, Conn& c, const uint8_t* f, size_t n) {
    if (n < 3 || f[0] != 0x02 || f[1] != 0xF0 || f[2] != 0x80) {
        io.info.status = TPLC_S7_ST_BADRESP;
        io.finished = true;
        c.active_io = 0;
        return;
    }
    const uint8_t* p = f + 3;
    size_t pn = n - 3;
    if (pn < 12 || p[0] != 0x32 ||
        (p[1] != 0x02 && p[1] != 0x03)) {
        io.info.status = TPLC_S7_ST_BADRESP;
        io.finished = true;
        c.active_io = 0;
        return;
    }
    uint16_t ref = get16(p + 4);
    uint16_t plen = get16(p + 6), dlen = get16(p + 8);
    uint32_t errw = uint32_t(p[10]) << 8 | p[11];
    if (ref != io.pdu_ref) {
        /* stale/unsolicited response — keep waiting */
        return;
    }
    if (errw != 0) {
        io.info.status = TPLC_S7_ST_S7ERR;
        io.info.err = int(errw);
        io.finished = true;
        c.active_io = 0;
        return;
    }
    const uint8_t* param = p + 12;
    const uint8_t* data = param + plen;
    size_t dn = pn - 12 - plen;
    if (plen < 2 || dn != size_t(dlen)) {
        io.info.status = TPLC_S7_ST_BADRESP;
        io.finished = true;
        c.active_io = 0;
        return;
    }
    uint8_t func = param[0];
    uint8_t nitems = param[1];
    (void)nitems;
    io.val_len = 0;

    if (!io.is_write) {
        if (func != 0x04 || dn < 4) {
            io.info.status = TPLC_S7_ST_BADRESP;
            io.finished = true;
            c.active_io = 0;
            return;
        }
        /* data items: rc ts len16 payload (+fill byte parity) */
        const uint8_t* q = data;
        size_t left = dn;
        uint8_t rc = q[0], ts = q[1];
        uint16_t ilen = get16(q + 2);
        q += 4; left -= 4;
        /* data-section transport byte: length is in bits for
           0x04 (byte/word/dword) and 0x05 (int/dint), in bytes
           otherwise (0x03 bit, 0x07 real, 0x09 octet string) */
        size_t nbytes;
        if (ts == 0x04 || ts == 0x05) {
            nbytes = (ilen + 7) / 8;
        } else {
            nbytes = ilen;
        }
        if (nbytes > left) {
            io.info.status = TPLC_S7_ST_BADRESP;
            io.finished = true;
            c.active_io = 0;
            return;
        }
        size_t cp = nbytes > TPLC_S7_MAX_DATA ? TPLC_S7_MAX_DATA : nbytes;
        memcpy(io.val, q, cp);
        io.val_len = uint32_t(cp);
        io.info.data_len = uint32_t(cp);
        io.info.item_rc = rc;
        if (rc != 0xFF) {
            io.info.status = TPLC_S7_ST_S7ERR;
        } else {
            io.info.status = TPLC_S7_ST_OK;
        }
    } else {
        if (func != 0x05 || dn < 1) {
            io.info.status = TPLC_S7_ST_BADRESP;
            io.finished = true;
            c.active_io = 0;
            return;
        }
        /* data section = per-item return codes */
        io.info.item_rc = data[0];
        io.info.status = data[0] == 0xFF ? TPLC_S7_ST_OK
                                       : TPLC_S7_ST_S7ERR;
    }
    io.finished = true;
    c.active_io = 0;
}

int32_t S7Service::open(void*, const TplcS7OpenReq* req,
                        TplcS7ConnHandle* out) {
    if (!req || !out) return TPLC_S7_E_PARAM;
    if (req->rack > 15 || req->slot > 31) return TPLC_S7_E_PARAM;
    struct in_addr da{};
    if (!parse_v4(req->host, &da)) return TPLC_S7_E_PARAM;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_S7_E_IO;
    if (nonblock(fd) < 0) { close(fd); return TPLC_S7_E_IO; }
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));

    sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_addr = da;
    dst.sin_port = htons(req->port ? req->port : TPLC_S7_PORT);
    if (connect(fd, reinterpret_cast<sockaddr*>(&dst), sizeof(dst)) < 0 &&
        errno != EINPROGRESS) {
        close(fd);
        return TPLC_S7_E_IO;
    }

    uint32_t h = next_conn_++;
    if (h == 0) h = next_conn_++;
    Conn c;
    c.fd = fd;
    c.req = *req;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 5000;
    if (to > 60000) to = 60000;
    c.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    c.self = h;
    conns_.emplace(h, std::move(c));
    *out = h;
    return TPLC_S7_OK;
}

int32_t S7Service::connPoll(void*, TplcS7ConnHandle ch,
                            TplcS7ConnInfo* info) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_S7_E_STATE;
    Conn& c = it->second;
    TplcS7ConnInfo zero{};
    if (info) *info = zero;
    if (c.stage == Stage::Ready) {
        if (info) info->status = TPLC_S7_CONN_READY;
        return TPLC_S7_OK;
    }
    if (c.stage == Stage::Failed) {
        if (info) {
            info->status = TPLC_S7_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_S7_OK;
    }
    uint64_t now = clock_(clock_ctx_);
    if (now >= c.deadline_ns) {
        failConn(c, TPLC_S7_E_IO);
        if (info) {
            info->status = TPLC_S7_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_S7_OK;
    }
    if (!pumpTx(c)) {
        failConn(c, errno ? errno : TPLC_S7_E_IO);
        if (info) {
            info->status = TPLC_S7_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_S7_OK;
    }
    if (c.stage == Stage::TcpConnect) {
        int err = 0;
        socklen_t l = sizeof(err);
        if (getsockopt(c.fd, SOL_SOCKET, SO_ERROR, &err, &l) == 0) {
            if (err == 0) {
                /* queue COTP CR */
                uint8_t cotp[32];
                size_t cn = build_cotp_cr(cotp, sizeof(cotp),
                                          c.req.rack, c.req.slot);
                size_t wn = wrap_tpkt(c.tx, sizeof(c.tx), cotp, cn);
                if (!wn) { failConn(c, TPLC_S7_E_RANGE); }
                else {
                    c.tx_len = wn;
                    c.stage = Stage::CotpWaitCc;
                }
            } else if (err != EINPROGRESS && err != EALREADY &&
                       err != EWOULDBLOCK) {
                failConn(c, err);
            }
        }
    } else {
        uint8_t frame[1024];
        size_t flen = 0;
        int fr = pumpRx(c, frame, sizeof(frame), &flen);
        if (fr == -1 || fr == -2) {
            failConn(c, fr == -1 ? TPLC_S7_E_IO : (errno ? errno : TPLC_S7_E_IO));
        } else if (fr > 0) {
            handleHandshake(c, frame, flen);
        }
    }
    if (info) {
        info->status = c.stage == Stage::Ready ? TPLC_S7_CONN_READY
                    : c.stage == Stage::Failed ? TPLC_S7_CONN_ERROR
                    : TPLC_S7_CONN_PENDING;
        info->err = c.err;
    }
    return TPLC_S7_OK;
}

bool S7Service::sendJob(Conn& c, bool is_write, const TplcS7Addr& a,
                        const uint8_t* data, uint32_t dlen) {
    uint8_t param[32];
    uint8_t item[16];
    size_t in = build_addr_item(item, a);
    size_t pn = 0;
    param[pn++] = is_write ? 0x05 : 0x04;
    param[pn++] = 0x01;                 /* one item */
    memcpy(param + pn, item, in);
    pn += in;

    uint8_t dbuf[320];
    size_t dn = 0;
    if (is_write) {
        bool bit = a.bit_off != 0xFFFFFFFFu;
        uint8_t ts = bit ? 0x03 : 0x04;
        uint16_t dsize = bit ? uint16_t(dlen) : uint16_t(dlen * 8);
        dbuf[dn++] = 0x00;
        dbuf[dn++] = ts;
        put16(dbuf + dn, dsize);
        dn += 2;
        if (dlen > TPLC_S7_MAX_DATA) return false;
        memcpy(dbuf + dn, data, dlen);
        dn += dlen;
    }

    uint8_t pdu[384];
    size_t jn = build_s7_job(pdu, sizeof(pdu), ++c.pdu_ref,
                             param, pn, dbuf, dn);
    if (!jn) return false;
    uint8_t dt[448];
    size_t tn = build_cotp_dt(dt, sizeof(dt), pdu, jn);
    if (!tn) return false;
    size_t wn = wrap_tpkt(c.tx, sizeof(c.tx), dt, tn);
    if (!wn) return false;
    c.tx_len = wn;
    return true;
}

int32_t S7Service::read(void*, TplcS7ConnHandle ch,
                        const TplcS7ReadReq* req,
                        TplcS7IoHandle* out) {
    if (!req || !out) return TPLC_S7_E_PARAM;
    if (!valid_area(req->addr.area)) return TPLC_S7_E_PARAM;
    if (req->addr.byte_off > 0xFFFFFFu) return TPLC_S7_E_RANGE;
    bool bit = req->addr.bit_off != 0xFFFFFFFFu;
    if (bit && req->addr.bit_off > 7) return TPLC_S7_E_PARAM;
    if (!bit && (req->addr.len == 0 || req->addr.len > TPLC_S7_MAX_DATA))
        return TPLC_S7_E_RANGE;

    auto ci = conns_.find(ch);
    if (ci == conns_.end()) return TPLC_S7_E_STATE;
    Conn& c = ci->second;
    if (c.stage != Stage::Ready) return TPLC_S7_E_STATE;
    if (c.active_io) return TPLC_S7_E_BUSY;
    if (!sendJob(c, false, req->addr, nullptr, 0)) return TPLC_S7_E_RANGE;

    uint32_t h = next_io_++;
    if (h == 0) h = next_io_++;
    Io io;
    io.conn = ch;
    io.is_write = false;
    io.addr = req->addr;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 2000;
    if (to > 60000) to = 60000;
    io.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    io.pdu_ref = c.pdu_ref;
    io.info.status = TPLC_S7_ST_PENDING;
    ios_.emplace(h, std::move(io));
    c.active_io = h;
    *out = h;
    return TPLC_S7_OK;
}

int32_t S7Service::write(void*, TplcS7ConnHandle ch,
                         const TplcS7WriteReq* req,
                         TplcS7IoHandle* out) {
    if (!req || !out) return TPLC_S7_E_PARAM;
    if (!valid_area(req->addr.area)) return TPLC_S7_E_PARAM;
    if (req->addr.byte_off > 0xFFFFFFu) return TPLC_S7_E_RANGE;
    bool bit = req->addr.bit_off != 0xFFFFFFFFu;
    if (bit && req->addr.bit_off > 7) return TPLC_S7_E_PARAM;
    if (req->dlen == 0 || req->dlen > TPLC_S7_MAX_DATA)
        return TPLC_S7_E_RANGE;
    if (bit && req->dlen != 1) return TPLC_S7_E_PARAM;

    auto ci = conns_.find(ch);
    if (ci == conns_.end()) return TPLC_S7_E_STATE;
    Conn& c = ci->second;
    if (c.stage != Stage::Ready) return TPLC_S7_E_STATE;
    if (c.active_io) return TPLC_S7_E_BUSY;
    if (!sendJob(c, true, req->addr, req->data, req->dlen))
        return TPLC_S7_E_RANGE;

    uint32_t h = next_io_++;
    if (h == 0) h = next_io_++;
    Io io;
    io.conn = ch;
    io.is_write = true;
    io.addr = req->addr;
    memcpy(io.data, req->data, req->dlen);
    io.dlen = req->dlen;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 2000;
    if (to > 60000) to = 60000;
    io.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    io.pdu_ref = c.pdu_ref;
    io.info.status = TPLC_S7_ST_PENDING;
    ios_.emplace(h, std::move(io));
    c.active_io = h;
    *out = h;
    return TPLC_S7_OK;
}

int32_t S7Service::poll(void*, TplcS7IoHandle h,
                        TplcS7RespInfo* info, uint8_t* val,
                        uint32_t val_cap) {
    auto it = ios_.find(h);
    if (it == ios_.end()) return TPLC_S7_E_STATE;
    Io& io = it->second;
    auto ci = conns_.find(io.conn);
    if (ci == conns_.end()) {
        ios_.erase(it);
        return TPLC_S7_E_STATE;
    }
    Conn& c = ci->second;
    if (!io.finished) {
        uint64_t now = clock_(clock_ctx_);
        if (now >= io.deadline_ns) {
            io.info.status = TPLC_S7_ST_TIMEOUT;
            io.finished = true;
            c.active_io = 0;
        } else {
            if (!pumpTx(c)) {
                io.info.status = TPLC_S7_ST_ERROR;
                io.info.err = errno ? errno : TPLC_S7_E_IO;
                io.finished = true;
                c.active_io = 0;
            } else {
                uint8_t frame[1024];
                size_t flen = 0;
                int fr = pumpRx(c, frame, sizeof(frame), &flen);
                if (fr == -1 || fr == -2) {
                    io.info.status = TPLC_S7_ST_ERROR;
                    io.info.err = errno ? errno : TPLC_S7_E_IO;
                    io.finished = true;
                    c.active_io = 0;
                } else if (fr > 0) {
                    handleIoResp(io, c, frame, flen);
                }
            }
        }
    }
    if (info) *info = io.info;
    if (val && val_cap && !io.is_write) {
        size_t cp = io.val_len < val_cap ? io.val_len : val_cap;
        memcpy(val, io.val, cp);
    }
    if (io.finished) {
        ios_.erase(it);
    }
    return TPLC_S7_OK;
}

int32_t S7Service::abort(void*, TplcS7IoHandle h) {
    auto it = ios_.find(h);
    if (it == ios_.end()) return TPLC_S7_E_STATE;
    TplcS7ConnHandle ch = it->second.conn;
    ios_.erase(it);
    auto ci = conns_.find(ch);
    if (ci != conns_.end()) {
        /* the wire state is unknown mid-transaction — close the
           connection rather than risk a stale response matching
           the next job's pdu_ref */
        if (ci->second.fd >= 0) close(ci->second.fd);
        conns_.erase(ci);
    }
    return TPLC_S7_OK;
}

int32_t S7Service::connClose(void*, TplcS7ConnHandle ch) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_S7_E_STATE;
    if (it->second.fd >= 0) close(it->second.fd);
    uint32_t aio = it->second.active_io;
    conns_.erase(it);
    if (aio) ios_.erase(aio);
    return TPLC_S7_OK;
}

} /* namespace tplc */
