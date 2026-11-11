/* Iec870Service.cpp — IEC 60870-5-104 client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Iec870Service.hpp>

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

/* k = max unconfirmed I-frames we may send, w = I-frames we
   receive before sending an S-frame ack (IEC 60870-5-104
   typical values) */
constexpr uint32_t K_WINDOW = 12;
constexpr uint32_t W_WINDOW = 8;
constexpr uint32_t SEQ_MOD = 32768;

/* U-format control octets */
constexpr uint8_t U_STARTDT_ACT = 0x07;
constexpr uint8_t U_STARTDT_CON = 0x0B;
constexpr uint8_t U_STOPDT_ACT  = 0x13;
constexpr uint8_t U_STOPDT_CON  = 0x23;
constexpr uint8_t U_TESTFR_ACT  = 0x43;
constexpr uint8_t U_TESTFR_CON  = 0x83;
constexpr uint8_t S_TESTFR      = 0x01;

/* fixed 6-byte U/S frames: 68 04 <cf1> 00 00 00 for U;
   68 04 01 00 <recv*2 LE16> for S */
void build_u_frame(uint8_t* out, uint8_t func) {
    out[0] = 0x68; out[1] = 0x04; out[2] = func;
    out[3] = 0; out[4] = 0; out[5] = 0;
}

void build_s_frame(uint8_t* out, uint16_t recv_count) {
    uint32_t v = uint32_t(recv_count) * 2;
    out[0] = 0x68; out[1] = 0x04; out[2] = 0x01; out[3] = 0x00;
    out[4] = uint8_t(v & 0xFF);
    out[5] = uint8_t((v >> 8) & 0xFF);
}

/* decode a 15-bit sequence number from two control octets
   (stored shifted left 1, little-endian) */
uint32_t decode_seq(const uint8_t* p) {
    return ((uint32_t(p[1]) << 8) | (uint32_t(p[0]) & 0xFE)) / 2;
}

void encode_seq(uint8_t* p, uint16_t seq) {
    uint32_t v = uint32_t(seq) * 2;
    p[0] = uint8_t(v & 0xFF);
    p[1] = uint8_t((v >> 8) & 0xFF);
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
} /* namespace */

Iec870Service* Iec870Service::self(void* p) {
    return static_cast<Iec870Service*>(p);
}

Iec870Service::Iec870Service(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcIec870Service);
    svc_.abi_version = TPLC_IEC870_ABI_VERSION;
    svc_.host = this;
    svc_.iec870_open = [](void* s, const TplcIec870OpenReq* r,
                          TplcIec870ConnHandle* o) {
        return self(s)->open(s, r, o);
    };
    svc_.iec870_conn_poll = [](void* s, TplcIec870ConnHandle ch,
                               TplcIec870ConnInfo* i) {
        return self(s)->connPoll(s, ch, i);
    };
    svc_.iec870_send = [](void* s, TplcIec870ConnHandle ch,
                          const TplcIec870TxReq* r,
                          TplcIec870TxHandle* o) {
        return self(s)->send(s, ch, r, o);
    };
    svc_.iec870_tx_poll = [](void* s, TplcIec870TxHandle h,
                             TplcIec870TxInfo* i) {
        return self(s)->txPoll(s, h, i);
    };
    svc_.iec870_tx_abort = [](void* s, TplcIec870TxHandle h) {
        return self(s)->txAbort(s, h);
    };
    svc_.iec870_rx_poll = [](void* s, TplcIec870ConnHandle ch,
                             TplcIec870RxInfo* i, uint8_t* p,
                             uint32_t cap) {
        return self(s)->rxPoll(s, ch, i, p, cap);
    };
    svc_.iec870_close = [](void* s, TplcIec870ConnHandle ch) {
        return self(s)->connClose(s, ch);
    };
}

void Iec870Service::failConn(Conn& c, int err) {
    c.stage = Stage::Failed;
    c.err = err;
    if (c.active_tx) {
        auto it = txs_.find(c.active_tx);
        if (it != txs_.end()) {
            it->second.info.status = TPLC_IEC870_TX_ERROR;
            it->second.info.err = err;
            it->second.finished = true;
        }
        c.active_tx = 0;
    }
}

/* queue a fixed 6-byte U/S frame. If an I-frame is mid-write
   the frame is stashed in ctrl_pending and sent once tx drains
   (latest pending wins — TESTFR con / S-ack are idempotent). */
void Iec870Service::sendUFrame(Conn& c, uint8_t func) {
    if (c.stage != Stage::Active &&
        !(c.stage == Stage::StartDtWait &&
          (func == U_STARTDT_CON || func == U_STOPDT_CON)))
        return;
    uint8_t f[6];
    build_u_frame(f, func);
    if (c.tx_off == c.tx_len) {
        memcpy(c.tx, f, 6);
        c.tx_len = 6;
        c.tx_off = 0;
        c.tx_frame = false;
    } else {
        memcpy(c.ctrl_pending, f, 6);
        c.ctrl_pending_len = 6;
    }
}

void Iec870Service::sendSFrame(Conn& c) {
    uint8_t f[6];
    build_s_frame(f, c.recv_count);
    if (c.tx_off == c.tx_len) {
        memcpy(c.tx, f, 6);
        c.tx_len = 6;
        c.tx_off = 0;
        c.tx_frame = false;
    } else {
        memcpy(c.ctrl_pending, f, 6);
        c.ctrl_pending_len = 6;
    }
}

void Iec870Service::finishTx(Conn& c, int status, int err) {
    if (!c.active_tx) return;
    auto it = txs_.find(c.active_tx);
    if (it != txs_.end()) {
        it->second.info.status = status;
        it->second.info.err = err;
        it->second.finished = true;
    }
    c.active_tx = 0;
}

bool Iec870Service::pumpTx(Conn& c) {
    while (c.tx_off < c.tx_len) {
        ssize_t w = ::send(c.fd, c.tx + c.tx_off, c.tx_len - c.tx_off,
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
    if (c.tx_frame) {
        /* I-frame fully sent: consume the send sequence number */
        c.send_count = uint16_t((c.send_count + 1) % SEQ_MOD);
        finishTx(c, TPLC_IEC870_TX_OK, 0);
    }
    c.tx_len = c.tx_off = 0;
    c.tx_frame = false;
    /* a stashed control frame goes next */
    if (c.ctrl_pending_len) {
        memcpy(c.tx, c.ctrl_pending, c.ctrl_pending_len);
        c.tx_len = c.ctrl_pending_len;
        c.ctrl_pending_len = 0;
    }
    return true;
}

/* recv + APCI frame extraction. Returns one complete frame's
   length (>=6) in `flen`, 0 when none ready, -1 on overflow /
   peer close. Consumes the frame from rx. */
int Iec870Service::pumpRx(Conn& c, uint8_t* frame, size_t cap,
                          size_t* flen) {
    for (;;) {
        /* resync: drop bytes until the 0x68 start char */
        while (c.rx_len && c.rx[0] != 0x68) {
            memmove(c.rx, c.rx + 1, --c.rx_len);
            c.rx_dropped++;
        }
        if (c.rx_len >= 2) {
            size_t total = size_t(c.rx[1]) + 2;
            if (total < 6 || total > sizeof(c.rx)) return -1;
            if (c.rx_len >= total) {
                if (total > cap) return -1;
                memcpy(frame, c.rx, total);
                size_t rem = c.rx_len - total;
                memmove(c.rx, c.rx + total, rem);
                c.rx_len = rem;
                *flen = total;
                return int(total);
            }
        }
        ssize_t r = recv(c.fd, c.rx + c.rx_len,
                         sizeof(c.rx) - c.rx_len, MSG_DONTWAIT);
        if (r > 0) {
            c.rx_len += size_t(r);
            if (c.rx_len >= sizeof(c.rx)) return -1;
            continue;
        }
        if (r == 0) return -2;
        if (errno != EAGAIN && errno != EWOULDBLOCK && errno != EINTR)
            return -2;
        return 0;
    }
}

void Iec870Service::queueAsdu(Conn& c, const uint8_t* asdu, size_t n,
                              uint32_t i_seq) {
    uint16_t ca_size = c.req.ca_size ? c.req.ca_size : 2;
    uint16_t cot_size = c.req.cot_size ? c.req.cot_size : 2;
    /* header: type(1) vsq(1) cot(1|2) ca(1|2) */
    size_t hlen = 2 + cot_size + ca_size;
    if (n < hlen) { c.rx_dropped++; return; }
    RxAsdu a{};
    a.type_id = asdu[0];
    a.vsq = asdu[1];
    a.cot = asdu[2];
    size_t p = 3;
    if (cot_size == 2) { a.cot |= uint32_t(asdu[p]) << 8; p++; }
    a.ca = 0;
    for (uint16_t i = 0; i < ca_size; i++)
        a.ca |= uint32_t(asdu[p + i]) << (8 * i);
    p += ca_size;
    a.payload_len = uint16_t(n - p);
    if (a.payload_len > TPLC_IEC870_MAX_ASDU) {
        c.rx_dropped++;
        return;
    }
    memcpy(a.payload, asdu + p, a.payload_len);
    a.i_frame_seq = i_seq;

    if (c.q_count == TPLC_IEC870_RX_QUEUE) {
        /* drop the oldest entry, keep the freshest */
        memmove(&c.queue[0], &c.queue[1],
                (TPLC_IEC870_RX_QUEUE - 1) * sizeof(RxAsdu));
        c.q_head = TPLC_IEC870_RX_QUEUE - 1;
        c.q_count--;
        c.rx_dropped++;
    }
    uint32_t tail = (c.q_head + c.q_count) % TPLC_IEC870_RX_QUEUE;
    c.queue[tail] = a;
    c.q_count++;
}

void Iec870Service::handleFrame(Conn& c, const uint8_t* f, size_t n) {
    uint64_t now = clock_(clock_ctx_);
    c.last_rx_ns = now;
    uint8_t cf1 = f[2];

    if ((cf1 & 1) == 0) {
        /* I format */
        if (n < 7) { failConn(c, TPLC_IEC870_E_IO); return; }
        uint32_t sseq = decode_seq(f + 2);
        uint32_t rseq = decode_seq(f + 4);
        if (sseq != c.recv_count) {
            failConn(c, TPLC_IEC870_E_IO);   /* sequence error */
            return;
        }
        c.recv_count = uint16_t((c.recv_count + 1) % SEQ_MOD);
        c.confirmed_send = uint16_t(rseq % SEQ_MOD);
        c.unconfirmed_rx++;
        queueAsdu(c, f + 6, n - 6, sseq);
        if (c.unconfirmed_rx >= W_WINDOW) {
            c.unconfirmed_rx = 0;
            sendSFrame(c);
        }
        return;
    }
    if ((cf1 & 3) == 3) {
        /* U format */
        switch (cf1) {
        case U_TESTFR_ACT:
            sendUFrame(c, U_TESTFR_CON);
            break;
        case U_TESTFR_CON:
            c.testfr_outstanding = false;
            break;
        case U_STARTDT_ACT:
            sendUFrame(c, U_STARTDT_CON);
            break;
        case U_STARTDT_CON:
            if (c.stage == Stage::StartDtWait) {
                c.stage = Stage::Active;
                c.last_rx_ns = now;
            }
            break;
        case U_STOPDT_ACT:
            sendUFrame(c, U_STOPDT_CON);
            break;
        case U_STOPDT_CON:
        default:
            break;
        }
        return;
    }
    if (cf1 == S_TESTFR) {
        /* S format: receive sequence number at f[4..5] */
        c.confirmed_send = uint16_t(decode_seq(f + 4) % SEQ_MOD);
    }
}

int32_t Iec870Service::open(void*, const TplcIec870OpenReq* req,
                            TplcIec870ConnHandle* out) {
    if (!req || !out) return TPLC_IEC870_E_PARAM;
    if (req->ca_size && req->ca_size != 1 && req->ca_size != 2)
        return TPLC_IEC870_E_PARAM;
    if (req->cot_size && req->cot_size != 1 && req->cot_size != 2)
        return TPLC_IEC870_E_PARAM;
    struct in_addr da{};
    if (!parse_v4(req->host, &da)) return TPLC_IEC870_E_PARAM;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_IEC870_E_IO;
    if (nonblock(fd) < 0) { close(fd); return TPLC_IEC870_E_IO; }
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));

    sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_addr = da;
    dst.sin_port = htons(req->port ? req->port : TPLC_IEC870_PORT);
    if (connect(fd, reinterpret_cast<sockaddr*>(&dst), sizeof(dst)) < 0 &&
        errno != EINPROGRESS) {
        close(fd);
        return TPLC_IEC870_E_IO;
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
    return TPLC_IEC870_OK;
}

/* shared Active-stage pump: tx, rx, keepalive. Returns false on
   transport/protocol failure (conn already failed). */
bool Iec870Service::driveActive(Conn& c) {
    uint64_t now = clock_(clock_ctx_);
    if (!pumpTx(c)) { failConn(c, errno ? errno : TPLC_IEC870_E_IO); return false; }
    for (;;) {
        uint8_t frame[320];
        size_t flen = 0;
        int fr = pumpRx(c, frame, sizeof(frame), &flen);
        if (fr == -1 || fr == -2) {
            failConn(c, fr == -1 ? TPLC_IEC870_E_IO
                                 : (errno ? errno : TPLC_IEC870_E_IO));
            return false;
        }
        if (fr <= 0) break;
        handleFrame(c, frame, flen);
        if (c.stage != Stage::Active) return true;
    }
    /* TESTFR keepalive on receive silence */
    uint32_t ka = c.req.keepalive_ms ? c.req.keepalive_ms : 10000;
    if (ka > 60000) ka = 60000;
    if (c.last_rx_ns && !c.testfr_outstanding &&
        now - c.last_rx_ns > uint64_t(ka) * NS_PER_MS) {
        sendUFrame(c, U_TESTFR_ACT);
        c.testfr_outstanding = true;
        c.testfr_deadline_ns = now + uint64_t(c.req.timeout_ms ?
            c.req.timeout_ms : 5000) * NS_PER_MS;
    }
    if (c.testfr_outstanding && now >= c.testfr_deadline_ns) {
        failConn(c, TPLC_IEC870_E_IO);
        return false;
    }
    return true;
}

int32_t Iec870Service::connPoll(void*, TplcIec870ConnHandle ch,
                                TplcIec870ConnInfo* info) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_IEC870_E_STATE;
    Conn& c = it->second;
    TplcIec870ConnInfo zero{};
    if (info) *info = zero;

    int32_t status, substate;
    switch (c.stage) {
    case Stage::Failed:
        status = TPLC_IEC870_CONN_ERROR;
        substate = TPLC_IEC870_SUB_NONE;
        break;
    case Stage::Active:
        driveActive(c);
        status = c.stage == Stage::Failed ? TPLC_IEC870_CONN_ERROR
                                        : TPLC_IEC870_CONN_READY;
        substate = TPLC_IEC870_SUB_ACTIVE;
        break;
    case Stage::StartDtWait:
        /* STARTDT act is queued as a control frame; pump tx/rx */
        if (!pumpTx(c)) { failConn(c, errno ? errno : TPLC_IEC870_E_IO); }
        if (c.stage != Stage::Failed) {
            for (;;) {
                uint8_t frame[320];
                size_t flen = 0;
                int fr = pumpRx(c, frame, sizeof(frame), &flen);
                if (fr == -1 || fr == -2) {
                    failConn(c, TPLC_IEC870_E_IO);
                    break;
                }
                if (fr <= 0) break;
                handleFrame(c, frame, flen);
                if (c.stage == Stage::Active) break;
            }
        }
        status = c.stage == Stage::Active ? TPLC_IEC870_CONN_READY
                : c.stage == Stage::Failed ? TPLC_IEC870_CONN_ERROR
                : TPLC_IEC870_CONN_PENDING;
        substate = c.stage == Stage::Active ? TPLC_IEC870_SUB_ACTIVE
                 : TPLC_IEC870_SUB_STARTDT;
        break;
    case Stage::TcpConnect:
    default: {
        uint64_t now = clock_(clock_ctx_);
        if (now >= c.deadline_ns) {
            failConn(c, TPLC_IEC870_E_IO);
        } else {
            int err = 0;
            socklen_t l = sizeof(err);
            if (getsockopt(c.fd, SOL_SOCKET, SO_ERROR, &err, &l) == 0) {
                if (err == 0) {
                    /* queue STARTDT act, move to StartDtWait */
                    uint8_t f[6];
                    build_u_frame(f, U_STARTDT_ACT);
                    memcpy(c.tx, f, 6);
                    c.tx_len = 6;
                    c.tx_off = 0;
                    c.tx_frame = false;
                    c.stage = Stage::StartDtWait;
                    c.last_rx_ns = now;
                } else if (err != EINPROGRESS && err != EALREADY &&
                           err != EWOULDBLOCK) {
                    failConn(c, err);
                }
            }
        }
        status = c.stage == Stage::Failed ? TPLC_IEC870_CONN_ERROR
                                        : TPLC_IEC870_CONN_PENDING;
        substate = TPLC_IEC870_SUB_TCP;
        break;
    }
    }
    if (c.stage == Stage::Failed && status != TPLC_IEC870_CONN_ERROR)
        status = TPLC_IEC870_CONN_ERROR;
    if (info) {
        info->status = status;
        info->substate = substate;
        info->err = c.err;
        info->rx_dropped = c.rx_dropped;
    }
    return TPLC_IEC870_OK;
}

int32_t Iec870Service::send(void*, TplcIec870ConnHandle ch,
                            const TplcIec870TxReq* req,
                            TplcIec870TxHandle* out) {
    if (!req || !out) return TPLC_IEC870_E_PARAM;
    if (req->plen > TPLC_IEC870_MAX_ASDU) return TPLC_IEC870_E_RANGE;
    if (req->type_id > 0xFF || req->vsq > 0xFF) return TPLC_IEC870_E_PARAM;

    auto ci = conns_.find(ch);
    if (ci == conns_.end()) return TPLC_IEC870_E_STATE;
    Conn& c = ci->second;
    if (c.stage != Stage::Active) return TPLC_IEC870_E_STATE;
    if (c.active_tx) return TPLC_IEC870_E_BUSY;

    /* k-window: unacked = send_count - confirmed_send (mod 32768) */
    uint32_t unacked = (uint32_t(c.send_count) + SEQ_MOD -
                        uint32_t(c.confirmed_send)) % SEQ_MOD;
    if (unacked >= K_WINDOW) return TPLC_IEC870_E_BUSY;

    uint16_t ca_size = c.req.ca_size ? c.req.ca_size : 2;
    uint16_t cot_size = c.req.cot_size ? c.req.cot_size : 2;

    /* ASDU: type(1) vsq(1) cot(1|2) ca(1|2) payload */
    size_t asdu_len = 2 + cot_size + ca_size + req->plen;
    size_t frame_len = 6 + asdu_len;
    if (frame_len > sizeof(c.tx) - 1) return TPLC_IEC870_E_RANGE;

    uint8_t* f = c.tx;
    f[0] = 0x68;
    f[1] = uint8_t(frame_len - 2);
    encode_seq(f + 2, c.send_count);
    encode_seq(f + 4, c.recv_count);
    size_t p = 6;
    f[p++] = uint8_t(req->type_id);
    f[p++] = uint8_t(req->vsq);
    f[p++] = uint8_t(req->cot & 0xFF);
    if (cot_size == 2) f[p++] = uint8_t((req->cot >> 8) & 0xFF);
    f[p++] = uint8_t(req->ca & 0xFF);
    if (ca_size == 2) f[p++] = uint8_t((req->ca >> 8) & 0xFF);
    if (req->plen) memcpy(f + p, req->payload, req->plen);

    c.tx_len = frame_len;
    c.tx_off = 0;
    c.tx_frame = true;

    uint32_t h = next_tx_++;
    if (h == 0) h = next_tx_++;
    Tx t;
    t.conn = ch;
    t.info.status = TPLC_IEC870_TX_PENDING;
    txs_.emplace(h, std::move(t));
    c.active_tx = h;
    *out = h;
    return TPLC_IEC870_OK;
}

int32_t Iec870Service::txPoll(void*, TplcIec870TxHandle h,
                              TplcIec870TxInfo* info) {
    auto it = txs_.find(h);
    if (it == txs_.end()) return TPLC_IEC870_E_STATE;
    Tx& t = it->second;
    auto ci = conns_.find(t.conn);
    if (ci == conns_.end()) {
        txs_.erase(it);
        return TPLC_IEC870_E_STATE;
    }
    Conn& c = ci->second;
    if (!t.finished && c.stage == Stage::Active) {
        driveActive(c);
        if (c.stage == Stage::Failed && !t.finished) {
            t.info.status = TPLC_IEC870_TX_ERROR;
            t.info.err = c.err;
            t.finished = true;
        }
    } else if (!t.finished && c.stage == Stage::Failed) {
        t.info.status = TPLC_IEC870_TX_ERROR;
        t.info.err = c.err;
        t.finished = true;
    }
    if (info) *info = t.info;
    if (t.finished) txs_.erase(it);
    return TPLC_IEC870_OK;
}

int32_t Iec870Service::txAbort(void*, TplcIec870TxHandle h) {
    auto it = txs_.find(h);
    if (it == txs_.end()) return TPLC_IEC870_E_STATE;
    TplcIec870ConnHandle ch = it->second.conn;
    txs_.erase(it);
    auto ci = conns_.find(ch);
    if (ci != conns_.end()) {
        Conn& c = ci->second;
        if (c.active_tx == h) {
            /* the wire state is unknown mid-frame — drop the conn
               rather than risk a corrupted I-frame stream */
            if (c.fd >= 0) close(c.fd);
            c.fd = -1;
            failConn(c, TPLC_IEC870_E_IO);
        }
    }
    return TPLC_IEC870_OK;
}

int32_t Iec870Service::rxPoll(void*, TplcIec870ConnHandle ch,
                              TplcIec870RxInfo* info,
                              uint8_t* payload, uint32_t cap) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_IEC870_E_STATE;
    Conn& c = it->second;
    TplcIec870RxInfo zero{};
    if (info) *info = zero;

    if (c.stage == Stage::Active) driveActive(c);

    if (c.q_count == 0) {
        if (info) {
            info->status = TPLC_IEC870_RX_NODATA;
            info->err = c.stage == Stage::Failed ? c.err : 0;
        }
        return TPLC_IEC870_OK;
    }
    RxAsdu& a = c.queue[c.q_head];
    uint32_t cp = a.payload_len < cap ? a.payload_len : cap;
    if (payload && cp) memcpy(payload, a.payload, cp);
    if (info) {
        info->status = TPLC_IEC870_RX_ASDU;
        info->type_id = a.type_id;
        info->vsq = a.vsq;
        info->cot = a.cot;
        info->ca = a.ca;
        info->payload_len = a.payload_len;
        info->i_frame_seq = a.i_frame_seq;
    }
    /* consume */
    c.q_head = (c.q_head + 1) % TPLC_IEC870_RX_QUEUE;
    c.q_count--;
    return TPLC_IEC870_OK;
}

int32_t Iec870Service::connClose(void*, TplcIec870ConnHandle ch) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_IEC870_E_STATE;
    Conn& c = it->second;
    /* best-effort STOPDT act, then close — the con is not awaited */
    if (c.stage == Stage::Active && c.fd >= 0) {
        uint8_t f[6];
        build_u_frame(f, U_STOPDT_ACT);
        ::send(c.fd, f, 6, MSG_DONTWAIT);
    }
    if (c.fd >= 0) close(c.fd);
    for (auto ti = txs_.begin(); ti != txs_.end();) {
        if (ti->second.conn == ch) ti = txs_.erase(ti);
        else ++ti;
    }
    conns_.erase(it);
    return TPLC_IEC870_OK;
}

} /* namespace tplc */
