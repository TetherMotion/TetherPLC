/* SmtpService.cpp — SMTP client (RFC 5321/5322), plain text only.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/SmtpService.hpp>

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
constexpr size_t BODY_CAP = 4096;

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

static uint32_t reply_code(const char* line) {
    if (!line || line[0] < '0' || line[0] > '9') return 0;
    uint32_t c = uint32_t(line[0] - '0') * 100;
    if (line[1] < '0' || line[1] > '9') return 0;
    c += uint32_t(line[1] - '0') * 10;
    if (line[2] < '0' || line[2] > '9') return 0;
    return c + uint32_t(line[2] - '0');
}

static bool reply_final(const char* line) {
    return line[3] == ' ' || line[3] == '\0' || line[3] == '\r';
}

/* RFC 4648 base64 (standard alphabet, no padding issues) */
static size_t b64enc(const char* in, size_t n, char* out, size_t cap) {
    static const char* A =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    size_t o = 0;
    for (size_t i = 0; i < n; i += 3) {
        uint32_t v = uint32_t(uint8_t(in[i])) << 16;
        bool b1 = i + 1 < n, b2 = i + 2 < n;
        if (b1) v |= uint32_t(uint8_t(in[i + 1])) << 8;
        if (b2) v |= uint32_t(uint8_t(in[i + 2]));
        char e[4] = {A[(v >> 18) & 63], A[(v >> 12) & 63],
                     b1 ? A[(v >> 6) & 63] : '=',
                     b2 ? A[v & 63] : '='};
        for (int k = 0; k < 4; k++) {
            if (o + 1 >= cap) return 0;
            out[o++] = e[k];
        }
    }
    out[o] = 0;
    return o;
}

/* copy a request string field into a NUL-terminated buffer,
   stripping CR/LF (header/command injection guard) */
static void clean_str(const char* src, char* dst, size_t cap) {
    size_t n = 0;
    if (src) {
        for (const char* p = src; *p && n + 1 < cap; p++) {
            if (*p == '\r' || *p == '\n') continue;
            dst[n++] = *p;
        }
    }
    dst[n] = 0;
}
} /* namespace */

SmtpService* SmtpService::self(void* p) { return static_cast<SmtpService*>(p); }

SmtpService::SmtpService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcSmtpService);
    svc_.abi_version = TPLC_SMTP_ABI_VERSION;
    svc_.host = this;
    svc_.smtp_begin = [](void* s, const TplcSmtpReq* r,
                         TplcSmtpHandle* o) {
        return self(s)->begin(s, r, o);
    };
    svc_.smtp_poll = [](void* s, TplcSmtpHandle h, TplcSmtpInfo* i) {
        return self(s)->poll(s, h, i);
    };
    svc_.smtp_feed = [](void* s, TplcSmtpHandle h, const uint8_t* b,
                        uint32_t l) {
        return self(s)->feed(s, h, b, l);
    };
    svc_.smtp_feed_finish = [](void* s, TplcSmtpHandle h) {
        return self(s)->feedFinish(s, h);
    };
    svc_.smtp_abort = [](void* s, TplcSmtpHandle h) {
        return self(s)->abort(s, h);
    };
}

void SmtpService::failTxn(Txn& t, int status, int err) {
    t.info.status = status;
    t.info.err = err;
    t.finished = true;
}

void SmtpService::dropTxn(Txn& t) {
    if (t.fd >= 0) close(t.fd);
    t.fd = -1;
}

bool SmtpService::pumpTx(Txn& t) {
    while (t.tx_off < t.tx_len) {
        ssize_t w = send(t.fd, t.tx + t.tx_off, t.tx_len - t.tx_off,
                         MSG_DONTWAIT);
        if (w > 0) {
            t.tx_off += size_t(w);
        } else if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK ||
                             errno == EINTR)) {
            return true;
        } else {
            return false;
        }
    }
    t.tx_len = t.tx_off = 0;
    return true;
}

int SmtpService::pumpRxLine(Txn& t, char* line, size_t cap, size_t* llen) {
    for (;;) {
        for (size_t i = 0; i < t.rx_len; i++) {
            if (t.rx[i] == '\n') {
                size_t n = i;
                if (n > 0 && t.rx[n - 1] == '\r') n--;
                if (n >= cap) return -3;
                memcpy(line, t.rx, n);
                line[n] = 0;
                size_t rem = t.rx_len - (i + 1);
                memmove(t.rx, t.rx + i + 1, rem);
                t.rx_len = rem;
                *llen = n;
                return 1;
            }
        }
        if (t.rx_len >= sizeof(t.rx)) return -3;
        ssize_t r = recv(t.fd, t.rx + t.rx_len, sizeof(t.rx) - t.rx_len,
                         MSG_DONTWAIT);
        if (r > 0) {
            t.rx_len += size_t(r);
            continue;
        }
        if (r == 0) return -2;
        if (errno != EAGAIN && errno != EWOULDBLOCK && errno != EINTR)
            return -2;
        return 0;
    }
}

bool SmtpService::sendLine(Txn& t, const char* s) {
    size_t n = strlen(s);
    if (n + 2 > sizeof(t.tx)) return false;
    memcpy(t.tx, s, n);
    t.tx[n] = '\r';
    t.tx[n + 1] = '\n';
    t.tx_len = n + 2;
    t.tx_off = 0;
    return true;
}

void SmtpService::queueRaw(Txn& t, const char* s, size_t n) {
    if (t.tx_len + n > sizeof(t.tx)) n = sizeof(t.tx) - t.tx_len;
    if (n == 0) return;
    memcpy(t.tx + t.tx_len, s, n);
    t.tx_len += n;
}

void SmtpService::queueLine(Txn& t, const char* s) {
    queueRaw(t, s, strlen(s));
    queueRaw(t, "\r\n", 2);
}

/* move complete body lines through RFC 5321 dot-stuffing into tx.
   flush=true also emits an unterminated tail as the final line
   (used once feed_finish has been called). */
void SmtpService::pumpBody(Txn& t, bool flush) {
    while (t.body_off < t.body_len) {
        /* find next '\n' */
        size_t nl = t.body_off;
        while (nl < t.body_len && t.body[nl] != '\n') nl++;
        bool complete = nl < t.body_len;
        if (!complete && !flush) break; /* wait for more feed */
        size_t linelen = nl - t.body_off;
        /* strip a trailing '\r' from the line content */
        size_t content = linelen;
        if (content > 0 && t.body[t.body_off + content - 1] == '\r')
            content--;
        /* worst-case tx need: dot + content + CRLF */
        size_t need = content + 3;
        if (t.tx_len + need > sizeof(t.tx)) break; /* drain first */
        bool dot = content > 0 && t.body[t.body_off] == '.';
        if (dot) queueRaw(t, ".", 1);
        if (content > 0)
            queueRaw(t, reinterpret_cast<const char*>(t.body + t.body_off),
                     content);
        queueRaw(t, "\r\n", 2);
        t.body_off = complete ? nl + 1 : t.body_len;
    }
    if (t.body_off >= t.body_len) {
        t.body_off = t.body_len = 0;
    }
}

void SmtpService::queueHeaders(Txn& t) {
    char h[1024];
    int n = snprintf(h, sizeof(h),
                     "From: <%s>\r\n"
                     "To: <%s>\r\n"
                     "Subject: %s\r\n"
                     "MIME-Version: 1.0\r\n"
                     "Content-Type: text/plain; charset=us-ascii\r\n"
                     "\r\n",
                     t.req.from[0] ? t.req.from : "plc@localhost",
                     t.req.to,
                     t.req.subject);
    if (n > 0) queueRaw(t, h, size_t(n) < sizeof(h) ? size_t(n) : sizeof(h));
}

int32_t SmtpService::begin(void*, const TplcSmtpReq* req,
                           TplcSmtpHandle* out) {
    if (!req || !out) return TPLC_SMTP_E_PARAM;
    struct in_addr da{};
    if (!parse_v4(req->host, &da)) return TPLC_SMTP_E_PARAM;
    if (!req->from[0] || !req->to[0]) return TPLC_SMTP_E_PARAM;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_SMTP_E_IO;
    if (nonblock(fd) < 0) { close(fd); return TPLC_SMTP_E_IO; }
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));

    sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_addr = da;
    dst.sin_port = htons(req->port ? req->port : TPLC_SMTP_PORT);
    if (connect(fd, reinterpret_cast<sockaddr*>(&dst), sizeof(dst)) < 0 &&
        errno != EINPROGRESS) {
        close(fd);
        return TPLC_SMTP_E_IO;
    }

    uint32_t h = next_++;
    if (h == 0) h = next_++;
    Txn t;
    t.fd = fd;
    clean_str(req->host, t.req.host, sizeof(t.req.host));
    t.req.port = req->port;
    t.req.timeout_ms = req->timeout_ms;
    clean_str(req->user, t.req.user, sizeof(t.req.user));
    clean_str(req->pass, t.req.pass, sizeof(t.req.pass));
    clean_str(req->from, t.req.from, sizeof(t.req.from));
    clean_str(req->to, t.req.to, sizeof(t.req.to));
    clean_str(req->subject, t.req.subject, sizeof(t.req.subject));
    uint32_t to = req->timeout_ms ? req->timeout_ms : 30000;
    if (to > 300000) to = 300000;
    t.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    t.info.status = TPLC_SMTP_ST_PENDING;
    /* pre-encode AUTH LOGIN credentials */
    if (t.req.user[0]) {
        b64enc(t.req.user, strlen(t.req.user), t.b64_user,
               sizeof(t.b64_user));
        b64enc(t.req.pass, strlen(t.req.pass), t.b64_pass,
               sizeof(t.b64_pass));
    }
    txns_.emplace(h, std::move(t));
    *out = h;
    return TPLC_SMTP_OK;
}

int32_t SmtpService::poll(void*, TplcSmtpHandle h,
                          TplcSmtpInfo* info) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_SMTP_E_STATE;
    Txn& t = it->second;

    if (!t.finished) {
        if (clock_(clock_ctx_) >= t.deadline_ns) {
            failTxn(t, TPLC_SMTP_ST_TIMEOUT, 0);
        } else {
            /* TcpConnect: wait for SO_ERROR then the 220 */
            if (t.stage == Stage::TcpConnect) {
                int err = 0;
                socklen_t l = sizeof(err);
                if (getsockopt(t.fd, SOL_SOCKET, SO_ERROR, &err, &l) == 0) {
                    if (err == 0) {
                        t.stage = Stage::GreetWait;
                    } else if (err != EINPROGRESS && err != EALREADY &&
                               err != EWOULDBLOCK) {
                        failTxn(t, TPLC_SMTP_ST_ERROR, err);
                    }
                }
            }

            if (!t.finished && !pumpTx(t)) {
                failTxn(t, TPLC_SMTP_ST_ERROR,
                        errno ? errno : TPLC_SMTP_E_IO);
            }

            /* BodySend drains body->tx->socket every poll */
            if (!t.finished && t.stage == Stage::BodySend) {
                pumpBody(t, false);
                if (!pumpTx(t)) {
                    failTxn(t, TPLC_SMTP_ST_ERROR,
                            errno ? errno : TPLC_SMTP_E_IO);
                }
            }

            /* control reply stages */
            if (!t.finished && t.stage != Stage::TcpConnect &&
                t.stage != Stage::BodySend && t.stage != Stage::Done) {
                char line[512];
                size_t llen = 0;
                int lr = pumpRxLine(t, line, sizeof(line), &llen);
                if (lr < 0) {
                    failTxn(t, TPLC_SMTP_ST_ERROR,
                            lr == -3 ? TPLC_SMTP_E_IO
                                     : (errno ? errno : TPLC_SMTP_E_IO));
                } else if (lr == 1) {
                    uint32_t code = reply_code(line);
                    /* keep the transaction result code: the QUIT
                       221 would otherwise mask the 250 accept */
                    if (code && t.stage != Stage::QuitWait)
                        t.info.resp_code = code;
                    if (!reply_final(line)) {
                        /* multiline continuation — keep reading;
                           fall through to next poll */
                    } else {
                        auto bad = [&](void) {
                            failTxn(t, TPLC_SMTP_ST_FAIL, 0);
                        };
                        auto ok2 = [&](void) { return code >= 200 && code < 300; };
                        switch (t.stage) {
                        case Stage::GreetWait:
                            if (code == 220) {
                                sendLine(t, "EHLO tplc");
                                t.stage = Stage::EhloWait;
                            } else bad();
                            break;
                        case Stage::EhloWait:
                            if (ok2()) {
                                if (t.req.user[0]) {
                                    sendLine(t, "AUTH LOGIN");
                                    t.stage = Stage::AuthWait;
                                    t.auth_step = 0;
                                } else {
                                    char m[160];
                                    snprintf(m, sizeof(m), "MAIL FROM:<%s>",
                                             t.req.from);
                                    sendLine(t, m);
                                    t.stage = Stage::MailWait;
                                }
                            } else bad();
                            break;
                        case Stage::AuthWait:
                            if (t.auth_step == 0) {
                                if (code == 334) {
                                    sendLine(t, t.b64_user);
                                    t.auth_step = 1;
                                } else bad();
                            } else if (t.auth_step == 1) {
                                if (code == 334) {
                                    sendLine(t, t.b64_pass);
                                    t.auth_step = 2;
                                } else bad();
                            } else {
                                if (code == 235) {
                                    char m[160];
                                    snprintf(m, sizeof(m), "MAIL FROM:<%s>",
                                             t.req.from);
                                    sendLine(t, m);
                                    t.stage = Stage::MailWait;
                                } else bad();
                            }
                            break;
                        case Stage::MailWait:
                            if (ok2()) {
                                char m[160];
                                snprintf(m, sizeof(m), "RCPT TO:<%s>",
                                         t.req.to);
                                sendLine(t, m);
                                t.stage = Stage::RcptWait;
                            } else bad();
                            break;
                        case Stage::RcptWait:
                            if (ok2()) {
                                sendLine(t, "DATA");
                                t.stage = Stage::DataWait;
                            } else bad();
                            break;
                        case Stage::DataWait:
                            if (code == 354) {
                                queueHeaders(t);
                                t.headers_sent = true;
                                t.stage = Stage::BodySend;
                            } else bad();
                            break;
                        case Stage::EndDataWait:
                            if (ok2()) {
                                sendLine(t, "QUIT");
                                t.stage = Stage::QuitWait;
                            } else bad();
                            break;
                        case Stage::QuitWait:
                            if (code == 221 || ok2()) {
                                t.info.status = TPLC_SMTP_ST_OK;
                                t.finished = true;
                            } else {
                                t.info.status = TPLC_SMTP_ST_OK;
                                t.finished = true;
                            }
                            break;
                        default:
                            break;
                        }
                    }
                }
            }

        }
    }

    /* BodySend terminator: flush any unterminated tail, then once
       every fed byte is consumed and the tx queue is fully
       drained, send the "." line */
    if (!t.finished && t.stage == Stage::BodySend && t.feed_done) {
        pumpBody(t, true);
        if (t.body_off >= t.body_len && t.tx_len == 0 &&
            t.tx_off == 0 && t.tx_len + 3 <= sizeof(t.tx)) {
            queueRaw(t, ".\r\n", 3);
            t.stage = Stage::EndDataWait;
            if (!pumpTx(t)) {
                failTxn(t, TPLC_SMTP_ST_ERROR,
                        errno ? errno : TPLC_SMTP_E_IO);
            }
        }
    }

    t.info.body_total = t.body_total;
    /* streaming status while in BodySend before terminator */
    if (!t.finished && t.stage == Stage::BodySend)
        t.info.status = TPLC_SMTP_ST_MORE;
    if (info) *info = t.info;
    if (t.finished) {
        dropTxn(t);
        txns_.erase(it);
    }
    return TPLC_SMTP_OK;
}

int32_t SmtpService::feed(void*, TplcSmtpHandle h, const uint8_t* buf,
                          uint32_t len) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_SMTP_E_STATE;
    Txn& t = it->second;
    if (t.finished || t.feed_done) return TPLC_SMTP_E_STATE;
    if (!buf || len == 0) return 0;
    /* compact consumed bytes first */
    if (t.body_off > 0 && t.body_off == t.body_len) {
        t.body_off = t.body_len = 0;
    } else if (t.body_off > 0) {
        memmove(t.body, t.body + t.body_off, t.body_len - t.body_off);
        t.body_len -= t.body_off;
        t.body_off = 0;
    }
    size_t space = BODY_CAP - t.body_len;
    size_t n = len < space ? len : space;
    if (n == 0) return 0;
    memcpy(t.body + t.body_len, buf, n);
    t.body_len += n;
    t.body_total += n;
    return int32_t(n);
}

int32_t SmtpService::feedFinish(void*, TplcSmtpHandle h) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_SMTP_E_STATE;
    it->second.feed_done = true;
    return TPLC_SMTP_OK;
}

int32_t SmtpService::abort(void*, TplcSmtpHandle h) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_SMTP_E_STATE;
    dropTxn(it->second);
    txns_.erase(it);
    return TPLC_SMTP_OK;
}

} /* namespace tplc */
