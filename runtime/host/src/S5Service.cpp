/* S5Service.cpp — Siemens S5 client service (RK512/3964 TCP).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/S5Service.hpp>

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

constexpr uint8_t RK_STX = 0x02;
constexpr uint8_t RK_ETX = 0x03;
constexpr uint8_t RK_TYPE_FETCH = 'E';   /* 0x45 */
constexpr uint8_t RK_TYPE_SEND = 'A';    /* 0x41 */
constexpr uint8_t RK_AREA_DB = 'D';      /* 0x44 */
constexpr uint8_t RK_FLAG_NONE = 0xFF;

/* wrap an RK512 message in a 3964 frame: STX <msg> BCC ETX,
   BCC = XOR of msg bytes plus the ETX byte. */
size_t wrap_3964(uint8_t* out, size_t cap, const uint8_t* msg,
                 size_t n) {
    if (n + 4 > cap) return 0;
    out[0] = RK_STX;
    memcpy(out + 1, msg, n);
    uint8_t bcc = RK_ETX;
    for (size_t i = 0; i < n; ++i) bcc ^= msg[i];
    out[1 + n] = bcc;
    out[2 + n] = RK_ETX;
    return n + 3;
}

/* RK512 FETCH/SEND command telegram (10-byte header + optional
   SEND payload). addr fields are already validated to the
   one-byte wire ranges. */
size_t build_rk512(uint8_t* out, size_t cap, bool is_write,
                   const TplcS5Addr& a, const uint8_t* data,
                   uint32_t dlen) {
    uint32_t woff = a.byte_off / 2;
    uint32_t nwords = (a.len + 1) / 2;
    size_t payload = is_write ? size_t(nwords * 2) : 0;
    if (10 + payload > cap) return 0;
    out[0] = 0x00; out[1] = 0x00;             /* telegram id     */
    out[2] = is_write ? RK_TYPE_SEND : RK_TYPE_FETCH;
    out[3] = RK_AREA_DB;
    out[4] = uint8_t(a.db);
    out[5] = uint8_t(woff);
    out[6] = uint8_t(nwords >> 8);
    out[7] = uint8_t(nwords);
    out[8] = RK_FLAG_NONE;
    out[9] = RK_FLAG_NONE;
    if (is_write) {
        /* payload = nwords*2 bytes; copy dlen, zero-pad the rest
           (an odd byte tail occupies the high byte of its word) */
        memset(out + 10, 0, payload);
        memcpy(out + 10, data, dlen);
    }
    return 10 + payload;
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

/* byte_off must be even and inside the 255-word window; len in
   bytes maps to ceil(len/2) words capped at 255 */
bool valid_addr(const TplcS5Addr& a) {
    if (a.db > 255) return false;
    if (a.byte_off > 510 || (a.byte_off & 1)) return false;
    if (a.len == 0 || a.len > TPLC_S5_MAX_DATA) return false;
    return (a.byte_off / 2 + (a.len + 1) / 2) <= 255;
}
} /* namespace */

S5Service* S5Service::self(void* p) { return static_cast<S5Service*>(p); }

S5Service::S5Service(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcS5Service);
    svc_.abi_version = TPLC_S5_ABI_VERSION;
    svc_.host = this;
    svc_.s5_open = [](void* s, const TplcS5OpenReq* r,
                      TplcS5ConnHandle* o) {
        return self(s)->open(s, r, o);
    };
    svc_.s5_conn_poll = [](void* s, TplcS5ConnHandle ch,
                           TplcS5ConnInfo* i) {
        return self(s)->connPoll(s, ch, i);
    };
    svc_.s5_read = [](void* s, TplcS5ConnHandle ch,
                      const TplcS5ReadReq* r, TplcS5IoHandle* o) {
        return self(s)->read(s, ch, r, o);
    };
    svc_.s5_write = [](void* s, TplcS5ConnHandle ch,
                       const TplcS5WriteReq* r, TplcS5IoHandle* o) {
        return self(s)->write(s, ch, r, o);
    };
    svc_.s5_poll = [](void* s, TplcS5IoHandle h, TplcS5RespInfo* i,
                      uint8_t* v, uint32_t vc) {
        return self(s)->poll(s, h, i, v, vc);
    };
    svc_.s5_abort = [](void* s, TplcS5IoHandle h) {
        return self(s)->abort(s, h);
    };
    svc_.s5_close = [](void* s, TplcS5ConnHandle ch) {
        return self(s)->connClose(s, ch);
    };
}

void S5Service::failConn(Conn& c, int err) {
    c.stage = Stage::Failed;
    c.err = err;
    if (c.active_io) {
        auto ii = ios_.find(c.active_io);
        if (ii != ios_.end()) {
            ii->second.info.status = TPLC_S5_ST_ERROR;
            ii->second.info.err = err;
            ii->second.finished = true;
        }
    }
}

bool S5Service::pumpTx(Conn& c) {
    while (c.tx_off < c.tx_len) {
        ssize_t w = send(c.fd, c.tx + c.tx_off, c.tx_len - c.tx_off,
                         MSG_DONTWAIT);
        if (w > 0) {
            c.tx_off += size_t(w);
        } else if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK ||
                             errno == EINTR)) {
            return true;
        } else {
            return false;
        }
    }
    c.tx_len = c.tx_off = 0;
    return true;
}

/* extract one complete 3964 frame's RK512 payload; returns
   payload length, 0 = incomplete, -1 = malformed/overflow,
   -2 = peer closed */
int S5Service::pumpRx(Conn& c, uint8_t* frame, size_t cap,
                      size_t* flen) {
    for (;;) {
        /* resync: drop bytes until STX at the head */
        if (c.rx_len == 0 || c.rx[0] != RK_STX) {
            ssize_t r = recv(c.fd, c.rx, 1, MSG_DONTWAIT);
            if (r > 0) {
                if (c.rx[0] == RK_STX) c.rx_len = 1;
                continue;
            }
            if (r == 0) return -2;
            if (errno != EAGAIN && errno != EWOULDBLOCK && errno != EINTR)
                return -2;
            return 0;
        }
        /* complete frame: STX <msg..> BCC ETX. Payload bytes may
           contain 0x03, so each ETX candidate is verified by its
           BCC before it is accepted as the terminator. */
        for (size_t i = 2; i < c.rx_len; ++i) {
            if (c.rx[i] != RK_ETX) continue;
            if (i < 3) return -1;
            uint8_t want = RK_ETX;
            for (size_t j = 1; j + 1 < i; ++j) want ^= c.rx[j];
            if (c.rx[i - 1] != want) continue;   /* false ETX */
            size_t msglen = i - 2;              /* bytes 1..i-2   */
            if (msglen > cap) return -1;
            memcpy(frame, c.rx + 1, msglen);
            size_t rem = c.rx_len - (i + 1);
            memmove(c.rx, c.rx + i + 1, rem);
            c.rx_len = rem;
            *flen = msglen;
            return int(msglen);
        }
        if (c.rx_len >= sizeof(c.rx)) return -1;
        ssize_t r = recv(c.fd, c.rx + c.rx_len,
                         sizeof(c.rx) - c.rx_len, MSG_DONTWAIT);
        if (r > 0) {
            c.rx_len += size_t(r);
            continue;
        }
        if (r == 0) return -2;
        if (errno != EAGAIN && errno != EWOULDBLOCK && errno != EINTR)
            return -2;
        return 0;
    }
}

void S5Service::handleIoResp(Io& io, Conn& c, const uint8_t* f,
                             size_t n) {
    /* response header: id[2] type status — type echoes the
       request, status 0x00 = ok */
    if (n < 4 || f[0] != 0x00 || f[1] != 0x00 ||
        f[2] != (io.is_write ? RK_TYPE_SEND : RK_TYPE_FETCH)) {
        io.info.status = TPLC_S5_ST_BADRESP;
        io.finished = true;
        c.active_io = 0;
        return;
    }
    uint8_t status = f[3];
    if (status != 0) {
        io.info.status = TPLC_S5_ST_S5ERR;
        io.info.err = int(status);
        io.finished = true;
        c.active_io = 0;
        return;
    }
    if (!io.is_write) {
        size_t dn = n - 4;
        size_t want = size_t((io.addr.len + 1) / 2 * 2);
        if (dn < want) {
            io.info.status = TPLC_S5_ST_BADRESP;
            io.finished = true;
            c.active_io = 0;
            return;
        }
        size_t cp = io.addr.len < TPLC_S5_MAX_DATA
                        ? io.addr.len
                        : TPLC_S5_MAX_DATA;
        memcpy(io.val, f + 4, cp);
        io.val_len = uint32_t(cp);
        io.info.data_len = uint32_t(cp);
    }
    io.info.status = TPLC_S5_ST_OK;
    io.finished = true;
    c.active_io = 0;
}

bool S5Service::sendJob(Conn& c, bool is_write, const TplcS5Addr& a,
                        const uint8_t* data, uint32_t dlen) {
    uint8_t msg[600];
    size_t mn = build_rk512(msg, sizeof(msg), is_write, a, data,
                            dlen);
    if (!mn) return false;
    size_t wn = wrap_3964(c.tx, sizeof(c.tx), msg, mn);
    if (!wn) return false;
    c.tx_len = wn;
    return true;
}

int32_t S5Service::open(void*, const TplcS5OpenReq* req,
                        TplcS5ConnHandle* out) {
    if (!req || !out) return TPLC_S5_E_PARAM;
    struct in_addr da{};
    if (!parse_v4(req->host, &da)) return TPLC_S5_E_PARAM;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_S5_E_IO;
    if (nonblock(fd) < 0) { close(fd); return TPLC_S5_E_IO; }
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));

    sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_addr = da;
    dst.sin_port = htons(req->port ? req->port : TPLC_S5_PORT);
    if (connect(fd, reinterpret_cast<sockaddr*>(&dst), sizeof(dst)) < 0 &&
        errno != EINPROGRESS) {
        close(fd);
        return TPLC_S5_E_IO;
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
    return TPLC_S5_OK;
}

int32_t S5Service::connPoll(void*, TplcS5ConnHandle ch,
                            TplcS5ConnInfo* info) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_S5_E_STATE;
    Conn& c = it->second;
    TplcS5ConnInfo zero{};
    if (info) *info = zero;
    if (c.stage == Stage::Ready) {
        if (info) info->status = TPLC_S5_CONN_READY;
        return TPLC_S5_OK;
    }
    if (c.stage == Stage::Failed) {
        if (info) {
            info->status = TPLC_S5_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_S5_OK;
    }
    uint64_t now = clock_(clock_ctx_);
    if (now >= c.deadline_ns) {
        failConn(c, TPLC_S5_E_IO);
        if (info) {
            info->status = TPLC_S5_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_S5_OK;
    }
    if (!pumpTx(c)) {
        failConn(c, errno ? errno : TPLC_S5_E_IO);
        if (info) {
            info->status = TPLC_S5_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_S5_OK;
    }
    if (c.stage == Stage::TcpConnect) {
        int err = 0;
        socklen_t l = sizeof(err);
        if (getsockopt(c.fd, SOL_SOCKET, SO_ERROR, &err, &l) == 0) {
            if (err == 0) {
                c.stage = Stage::Ready;
            } else if (err != EINPROGRESS && err != EALREADY &&
                       err != EWOULDBLOCK) {
                failConn(c, err);
            }
        }
    }
    if (info) {
        info->status = c.stage == Stage::Ready ? TPLC_S5_CONN_READY
                    : c.stage == Stage::Failed ? TPLC_S5_CONN_ERROR
                    : TPLC_S5_CONN_PENDING;
        info->err = c.err;
    }
    return TPLC_S5_OK;
}

int32_t S5Service::read(void*, TplcS5ConnHandle ch,
                        const TplcS5ReadReq* req,
                        TplcS5IoHandle* out) {
    if (!req || !out) return TPLC_S5_E_PARAM;
    if (!valid_addr(req->addr)) return TPLC_S5_E_PARAM;

    auto ci = conns_.find(ch);
    if (ci == conns_.end()) return TPLC_S5_E_STATE;
    Conn& c = ci->second;
    if (c.stage != Stage::Ready) return TPLC_S5_E_STATE;
    if (c.active_io) return TPLC_S5_E_BUSY;
    if (!sendJob(c, false, req->addr, nullptr, 0))
        return TPLC_S5_E_RANGE;

    uint32_t h = next_io_++;
    if (h == 0) h = next_io_++;
    Io io;
    io.conn = ch;
    io.is_write = false;
    io.addr = req->addr;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 2000;
    if (to > 60000) to = 60000;
    io.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    io.info.status = TPLC_S5_ST_PENDING;
    ios_.emplace(h, std::move(io));
    c.active_io = h;
    *out = h;
    return TPLC_S5_OK;
}

int32_t S5Service::write(void*, TplcS5ConnHandle ch,
                         const TplcS5WriteReq* req,
                         TplcS5IoHandle* out) {
    if (!req || !out) return TPLC_S5_E_PARAM;
    if (!valid_addr(req->addr)) return TPLC_S5_E_PARAM;
    if (req->dlen == 0 || req->dlen > TPLC_S5_MAX_DATA ||
        req->dlen != req->addr.len)
        return TPLC_S5_E_RANGE;

    auto ci = conns_.find(ch);
    if (ci == conns_.end()) return TPLC_S5_E_STATE;
    Conn& c = ci->second;
    if (c.stage != Stage::Ready) return TPLC_S5_E_STATE;
    if (c.active_io) return TPLC_S5_E_BUSY;
    if (!sendJob(c, true, req->addr, req->data, req->dlen))
        return TPLC_S5_E_RANGE;

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
    io.info.status = TPLC_S5_ST_PENDING;
    ios_.emplace(h, std::move(io));
    c.active_io = h;
    *out = h;
    return TPLC_S5_OK;
}

int32_t S5Service::poll(void*, TplcS5IoHandle h,
                        TplcS5RespInfo* info, uint8_t* val,
                        uint32_t val_cap) {
    auto it = ios_.find(h);
    if (it == ios_.end()) return TPLC_S5_E_STATE;
    Io& io = it->second;
    auto ci = conns_.find(io.conn);
    if (ci == conns_.end()) {
        ios_.erase(it);
        return TPLC_S5_E_STATE;
    }
    Conn& c = ci->second;
    if (!io.finished) {
        uint64_t now = clock_(clock_ctx_);
        if (now >= io.deadline_ns) {
            io.info.status = TPLC_S5_ST_TIMEOUT;
            io.finished = true;
            c.active_io = 0;
        } else {
            if (!pumpTx(c)) {
                io.info.status = TPLC_S5_ST_ERROR;
                io.info.err = errno ? errno : TPLC_S5_E_IO;
                io.finished = true;
                c.active_io = 0;
            } else {
                uint8_t frame[1088];
                size_t flen = 0;
                int fr = pumpRx(c, frame, sizeof(frame), &flen);
                if (fr == -1 || fr == -2) {
                    io.info.status = TPLC_S5_ST_ERROR;
                    io.info.err = fr == -1 ? TPLC_S5_E_IO
                                           : (errno ? errno
                                                    : TPLC_S5_E_IO);
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
    return TPLC_S5_OK;
}

int32_t S5Service::abort(void*, TplcS5IoHandle h) {
    auto it = ios_.find(h);
    if (it == ios_.end()) return TPLC_S5_E_STATE;
    TplcS5ConnHandle ch = it->second.conn;
    ios_.erase(it);
    auto ci = conns_.find(ch);
    if (ci != conns_.end()) {
        /* the wire state is unknown mid-transaction — close the
           connection rather than risk a stale response matching
           the next job's frame */
        if (ci->second.fd >= 0) close(ci->second.fd);
        conns_.erase(ci);
    }
    return TPLC_S5_OK;
}

int32_t S5Service::connClose(void*, TplcS5ConnHandle ch) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_S5_E_STATE;
    if (it->second.fd >= 0) close(it->second.fd);
    uint32_t aio = it->second.active_io;
    conns_.erase(it);
    if (aio) ios_.erase(aio);
    return TPLC_S5_OK;
}

} /* namespace tplc */
