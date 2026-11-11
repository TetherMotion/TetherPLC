/* FtpService.cpp — FTP client (RFC 959), PASV data connections.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/FtpService.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

namespace tplc {

namespace {
constexpr uint64_t NS_PER_MS = 1000000ull;

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

/* parse "NNN-x" or "NNN x" reply line; returns code or 0 */
static uint32_t reply_code(const char* line) {
    if (!line || line[0] < '0' || line[0] > '9') return 0;
    uint32_t c = uint32_t(line[0] - '0') * 100;
    if (line[1] < '0' || line[1] > '9') return 0;
    c += uint32_t(line[1] - '0') * 10;
    if (line[2] < '0' || line[2] > '9') return 0;
    return c + uint32_t(line[2] - '0');
}

static bool reply_final(const char* line) {
    /* final line has ' ' (or EOL) at index 3, continuation '-' */
    return line[3] == ' ' || line[3] == '\0' || line[3] == '\r';
}

/* extract reply text after "NNN " prefix */
static void reply_text(const char* line, char* out, size_t cap) {
    size_t n = 0;
    if (line[3] == ' ') {
        const char* p = line + 4;
        while (*p && *p != '\r' && *p != '\n' && n + 1 < cap) out[n++] = *p++;
    }
    out[n] = 0;
}
} /* namespace */

FtpService* FtpService::self(void* p) { return static_cast<FtpService*>(p); }

FtpService::FtpService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcFtpService);
    svc_.abi_version = TPLC_FTP_ABI_VERSION;
    svc_.host = this;
    svc_.ftp_open = [](void* s, const TplcFtpOpenReq* r,
                       TplcFtpConnHandle* o) {
        return self(s)->open(s, r, o);
    };
    svc_.ftp_conn_poll = [](void* s, TplcFtpConnHandle ch,
                            TplcFtpConnInfo* i) {
        return self(s)->connPoll(s, ch, i);
    };
    svc_.ftp_cmd = [](void* s, TplcFtpConnHandle ch,
                      const TplcFtpCmdReq* r, TplcFtpTxnHandle* o) {
        return self(s)->cmd(s, ch, r, o);
    };
    svc_.ftp_txn_poll = [](void* s, TplcFtpTxnHandle h,
                           TplcFtpTxnInfo* i, char* t, uint32_t tc) {
        return self(s)->txnPoll(s, h, i, t, tc);
    };
    svc_.ftp_xfer_begin = [](void* s, TplcFtpConnHandle ch,
                             const TplcFtpXferReq* r,
                             TplcFtpTxnHandle* o) {
        return self(s)->xferBegin(s, ch, r, o);
    };
    svc_.ftp_xfer_poll = [](void* s, TplcFtpTxnHandle h,
                            TplcFtpTxnInfo* i, uint8_t* v, uint32_t vc) {
        return self(s)->xferPoll(s, h, i, v, vc);
    };
    svc_.ftp_xfer_feed = [](void* s, TplcFtpTxnHandle h,
                            const uint8_t* b, uint32_t l) {
        return self(s)->xferFeed(s, h, b, l);
    };
    svc_.ftp_xfer_finish = [](void* s, TplcFtpTxnHandle h) {
        return self(s)->xferFinish(s, h);
    };
    svc_.ftp_abort = [](void* s, TplcFtpTxnHandle h) {
        return self(s)->abort(s, h);
    };
    svc_.ftp_close = [](void* s, TplcFtpConnHandle ch) {
        return self(s)->connClose(s, ch);
    };
}

void FtpService::failConn(Conn& c, int err) {
    c.stage = ConnStage::Failed;
    c.err = err;
    if (c.active_txn) {
        auto it = txns_.find(c.active_txn);
        if (it != txns_.end()) failTxn(it->second, TPLC_FTP_ST_ERROR, err);
    }
}

void FtpService::failTxn(Txn& t, int status, int err) {
    t.info.status = status;
    t.info.err = err;
    t.finished = true;
}

bool FtpService::pumpTx(Conn& c) {
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

/* extract one CRLF/LF-terminated reply line; returns 1 with
   line in out, 0 incomplete, negative on error/overflow */
int FtpService::pumpRxLine(Conn& c, char* line, size_t cap, size_t* llen) {
    for (;;) {
        for (size_t i = 0; i < c.rx_len; i++) {
            if (c.rx[i] == '\n') {
                size_t n = i;
                if (n > 0 && c.rx[n - 1] == '\r') n--;
                if (n >= cap) return -3;
                memcpy(line, c.rx, n);
                line[n] = 0;
                size_t rem = c.rx_len - (i + 1);
                memmove(c.rx, c.rx + i + 1, rem);
                c.rx_len = rem;
                *llen = n;
                return 1;
            }
        }
        if (c.rx_len >= sizeof(c.rx)) return -3;
        ssize_t r = recv(c.fd, c.rx + c.rx_len, sizeof(c.rx) - c.rx_len,
                         MSG_DONTWAIT);
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

bool FtpService::sendLine(Conn& c, const char* s) {
    size_t n = strlen(s);
    if (n + 2 > sizeof(c.tx)) return false;
    memcpy(c.tx, s, n);
    c.tx[n] = '\r';
    c.tx[n + 1] = '\n';
    c.tx_len = n + 2;
    c.tx_off = 0;
    return true;
}

int32_t FtpService::open(void*, const TplcFtpOpenReq* req,
                         TplcFtpConnHandle* out) {
    if (!req || !out) return TPLC_FTP_E_PARAM;
    struct in_addr da{};
    if (!parse_v4(req->host, &da)) return TPLC_FTP_E_PARAM;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_FTP_E_IO;
    if (nonblock(fd) < 0) { close(fd); return TPLC_FTP_E_IO; }
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));

    sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_addr = da;
    dst.sin_port = htons(req->port ? req->port : TPLC_FTP_PORT);
    if (connect(fd, reinterpret_cast<sockaddr*>(&dst), sizeof(dst)) < 0 &&
        errno != EINPROGRESS) {
        close(fd);
        return TPLC_FTP_E_IO;
    }

    uint32_t h = next_conn_++;
    if (h == 0) h = next_conn_++;
    Conn c;
    c.fd = fd;
    c.req = *req;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 10000;
    if (to > 120000) to = 120000;
    c.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    conns_.emplace(h, std::move(c));
    *out = h;
    return TPLC_FTP_OK;
}

int32_t FtpService::connPoll(void*, TplcFtpConnHandle ch,
                             TplcFtpConnInfo* info) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_FTP_E_STATE;
    Conn& c = it->second;
    TplcFtpConnInfo zero{};
    if (info) *info = zero;
    if (c.stage == ConnStage::Ready) {
        if (info) {
            info->status = TPLC_FTP_CONN_READY;
            info->resp_code = c.resp_code;
        }
        return TPLC_FTP_OK;
    }
    if (c.stage == ConnStage::Failed) {
        if (info) {
            info->status = TPLC_FTP_CONN_ERROR;
            info->err = c.err;
            info->resp_code = c.resp_code;
        }
        return TPLC_FTP_OK;
    }
    uint64_t now = clock_(clock_ctx_);
    if (now >= c.deadline_ns) {
        failConn(c, TPLC_FTP_E_IO);
        if (info) {
            info->status = TPLC_FTP_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_FTP_OK;
    }
    if (!pumpTx(c)) {
        failConn(c, errno ? errno : TPLC_FTP_E_IO);
        if (info) {
            info->status = TPLC_FTP_CONN_ERROR;
            info->err = c.err;
        }
        return TPLC_FTP_OK;
    }

    if (c.stage == ConnStage::TcpConnect) {
        int err = 0;
        socklen_t l = sizeof(err);
        if (getsockopt(c.fd, SOL_SOCKET, SO_ERROR, &err, &l) == 0) {
            if (err == 0) {
                /* wait for the 220 greeting */
                c.stage = ConnStage::UserWait;
                /* mark: greeting expected before USER; use
                   user_sent=false and a flag via rx state — we
                   handle greeting inside the reply loop below */
            } else if (err != EINPROGRESS && err != EALREADY &&
                       err != EWOULDBLOCK) {
                failConn(c, err);
            }
        }
    }

    /* drain any pending login replies */
    if (c.stage == ConnStage::UserWait || c.stage == ConnStage::PassWait) {
        char line[512];
        size_t llen = 0;
        int lr = pumpRxLine(c, line, sizeof(line), &llen);
        if (lr == -1 || lr == -2 || lr == -3) {
            failConn(c, lr == -3 ? TPLC_FTP_E_IO : (errno ? errno : TPLC_FTP_E_IO));
        } else if (lr == 1) {
            uint32_t code = reply_code(line);
            c.resp_code = code;
            bool fin = reply_final(line);
            if (c.stage == ConnStage::UserWait && !c.user_sent) {
                /* this is the greeting (220) — now send USER */
                if (fin && code >= 200 && code < 400) {
                    char cmd[160];
                    snprintf(cmd, sizeof(cmd), "USER %s",
                             c.req.user[0] ? c.req.user : "anonymous");
                    sendLine(c, cmd);
                    c.user_sent = true;
                    /* stay in UserWait: next reply is USER's */
                } else if (fin) {
                    failConn(c, TPLC_FTP_E_IO);
                }
            } else if (c.user_sent && c.stage == ConnStage::UserWait) {
                /* USER reply: expect 230 (logged in, no PASS)
                   or 331 (need password) */
                if (fin && code == 230) {
                    c.stage = ConnStage::Ready;
                } else if (fin && code == 331) {
                    char cmd[160];
                    snprintf(cmd, sizeof(cmd), "PASS %s",
                             c.req.pass[0] ? c.req.pass : "");
                    sendLine(c, cmd);
                    c.stage = ConnStage::PassWait;
                } else if (fin) {
                    failConn(c, TPLC_FTP_E_IO);
                }
            } else if (c.stage == ConnStage::PassWait) {
                /* PASS reply: expect 230 */
                if (fin && code == 230) {
                    c.stage = ConnStage::Ready;
                } else if (fin) {
                    failConn(c, TPLC_FTP_E_IO);
                }
            }
        }
    }

    if (info) {
        info->status = c.stage == ConnStage::Ready ? TPLC_FTP_CONN_READY
                    : c.stage == ConnStage::Failed ? TPLC_FTP_CONN_ERROR
                    : TPLC_FTP_CONN_PENDING;
        info->err = c.err;
        info->resp_code = c.resp_code;
    }
    return TPLC_FTP_OK;
}

int32_t FtpService::cmd(void*, TplcFtpConnHandle ch,
                        const TplcFtpCmdReq* req,
                        TplcFtpTxnHandle* out) {
    if (!req || !out) return TPLC_FTP_E_PARAM;
    if (!req->cmd[0]) return TPLC_FTP_E_PARAM;
    /* forbid CR/LF injection in the command line */
    for (const char* p = req->cmd; *p; p++)
        if (*p == '\r' || *p == '\n') return TPLC_FTP_E_PARAM;

    auto ci = conns_.find(ch);
    if (ci == conns_.end()) return TPLC_FTP_E_STATE;
    Conn& c = ci->second;
    if (c.stage != ConnStage::Ready) return TPLC_FTP_E_STATE;
    if (c.active_txn) return TPLC_FTP_E_BUSY;
    if (!sendLine(c, req->cmd)) return TPLC_FTP_E_RANGE;

    uint32_t h = next_txn_++;
    if (h == 0) h = next_txn_++;
    Txn t;
    t.conn = ch;
    t.is_xfer = false;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 10000;
    if (to > 120000) to = 120000;
    t.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    t.info.status = TPLC_FTP_ST_PENDING;
    txns_.emplace(h, std::move(t));
    c.active_txn = h;
    *out = h;
    return TPLC_FTP_OK;
}

int32_t FtpService::txnPoll(void*, TplcFtpTxnHandle h,
                            TplcFtpTxnInfo* info, char* textbuf,
                            uint32_t text_cap) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_FTP_E_STATE;
    Txn& t = it->second;
    auto ci = conns_.find(t.conn);
    if (ci == conns_.end()) {
        txns_.erase(it);
        return TPLC_FTP_E_STATE;
    }
    Conn& c = ci->second;
    if (!t.finished) {
        if (clock_(clock_ctx_) >= t.deadline_ns) {
            failTxn(t, TPLC_FTP_ST_TIMEOUT, 0);
            c.active_txn = 0;
        } else if (!pumpTx(c)) {
            failTxn(t, TPLC_FTP_ST_ERROR, errno ? errno : TPLC_FTP_E_IO);
            c.active_txn = 0;
        } else {
            char line[512];
            size_t llen = 0;
            int lr = pumpRxLine(c, line, sizeof(line), &llen);
            if (lr == -1 || lr == -2 || lr == -3) {
                failTxn(t, TPLC_FTP_ST_ERROR,
                        lr == -3 ? TPLC_FTP_E_IO
                                 : (errno ? errno : TPLC_FTP_E_IO));
                c.active_txn = 0;
            } else if (lr == 1) {
                uint32_t code = reply_code(line);
                if (code) {
                    t.info.resp_code = code;
                    c.resp_code = code;
                    reply_text(line, t.text, sizeof(t.text));
                    if (reply_final(line)) {
                        t.info.status = code >= 200 && code < 300
                                            ? TPLC_FTP_ST_OK
                                            : TPLC_FTP_ST_FAIL;
                        t.finished = true;
                        c.active_txn = 0;
                    }
                }
            }
        }
    }
    if (info) *info = t.info;
    if (textbuf && text_cap) {
        size_t n = strlen(t.text);
        if (n >= text_cap) n = text_cap - 1;
        memcpy(textbuf, t.text, n);
        textbuf[n] = 0;
    }
    if (t.finished) txns_.erase(it);
    return TPLC_FTP_OK;
}

int32_t FtpService::xferBegin(void*, TplcFtpConnHandle ch,
                              const TplcFtpXferReq* req,
                              TplcFtpTxnHandle* out) {
    if (!req || !out) return TPLC_FTP_E_PARAM;
    if (req->op > TPLC_FTP_XFER_LIST) return TPLC_FTP_E_PARAM;
    if (!req->path[0]) return TPLC_FTP_E_PARAM;
    for (const char* p = req->path; *p; p++)
        if (*p == '\r' || *p == '\n') return TPLC_FTP_E_PARAM;

    auto ci = conns_.find(ch);
    if (ci == conns_.end()) return TPLC_FTP_E_STATE;
    Conn& c = ci->second;
    if (c.stage != ConnStage::Ready) return TPLC_FTP_E_STATE;
    if (c.active_txn) return TPLC_FTP_E_BUSY;

    uint32_t h = next_txn_++;
    if (h == 0) h = next_txn_++;
    Txn t;
    t.conn = ch;
    t.is_xfer = true;
    t.op = req->op;
    memcpy(t.path, req->path, TPLC_FTP_MAX_STR - 1);
    uint32_t to = req->timeout_ms ? req->timeout_ms : 30000;
    if (to > 300000) to = 300000;
    t.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    t.info.status = TPLC_FTP_ST_PENDING;
    /* kick off: TYPE I */
    if (!sendLine(c, "TYPE I")) return TPLC_FTP_E_RANGE;
    t.xs = XferStage::TypeCmd;
    txns_.emplace(h, std::move(t));
    c.active_txn = h;
    *out = h;
    return TPLC_FTP_OK;
}

int32_t FtpService::xferPoll(void*, TplcFtpTxnHandle h,
                             TplcFtpTxnInfo* info, uint8_t* val,
                             uint32_t val_cap) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_FTP_E_STATE;
    Txn& t = it->second;
    if (!t.is_xfer) return TPLC_FTP_E_STATE;
    auto ci = conns_.find(t.conn);
    if (ci == conns_.end()) {
        dropTxn(t);
        txns_.erase(it);
        return TPLC_FTP_E_STATE;
    }
    Conn& c = ci->second;
    uint32_t chunk = 0;

    if (!t.finished) {
        if (clock_(clock_ctx_) >= t.deadline_ns) {
            failTxn(t, TPLC_FTP_ST_TIMEOUT, 0);
        } else {
            bool yield = false;
            while (!yield && !t.finished) {
                switch (t.xs) {
                case XferStage::TypeCmd:
                case XferStage::PasvCmd:
                case XferStage::XferCmd: {
                    /* awaiting a final control reply */
                    if (!pumpTx(c)) {
                        failTxn(t, TPLC_FTP_ST_ERROR,
                                errno ? errno : TPLC_FTP_E_IO);
                        break;
                    }
                    char line[512];
                    size_t llen = 0;
                    int lr = pumpRxLine(c, line, sizeof(line), &llen);
                    if (lr < 0) {
                        failTxn(t, TPLC_FTP_ST_ERROR,
                                lr == -3 ? TPLC_FTP_E_IO
                                         : (errno ? errno : TPLC_FTP_E_IO));
                        break;
                    }
                    if (lr == 0) { yield = true; break; }
                    uint32_t code = reply_code(line);
                    if (!code || !reply_final(line)) continue;
                    t.info.resp_code = code;
                    c.resp_code = code;
                    reply_text(line, t.text, sizeof(t.text));
                    if (t.xs == XferStage::TypeCmd) {
                        if (code >= 200 && code < 300) {
                            t.xs = XferStage::PasvCmd;
                            if (!sendLine(c, "PASV")) {
                                failTxn(t, TPLC_FTP_ST_ERROR,
                                        TPLC_FTP_E_IO);
                            }
                        } else {
                            failTxn(t, TPLC_FTP_ST_FAIL, 0);
                        }
                    } else if (t.xs == XferStage::PasvCmd) {
                        if (code != 227) {
                            failTxn(t, TPLC_FTP_ST_FAIL, 0);
                            break;
                        }
                        /* parse "(h1,h2,h3,h4,p1,p2)" */
                        const char* lp = strchr(t.text, '(');
                        const char* rp = strchr(t.text, ')');
                        unsigned n[6] = {0};
                        int cnt = 0;
                        bool okp = lp && rp && rp > lp + 1;
                        if (okp) {
                            const char* p = lp + 1;
                            for (int i = 0; i < 6 && okp; i++) {
                                unsigned v = 0;
                                if (*p < '0' || *p > '9') { okp = false; break; }
                                while (*p >= '0' && *p <= '9') {
                                    v = v * 10 + unsigned(*p - '0');
                                    p++;
                                }
                                n[i] = v;
                                if (i < 5) {
                                    if (*p != ',') { okp = false; break; }
                                    p++;
                                } else if (*p != ')') {
                                    okp = false;
                                }
                                cnt++;
                            }
                        }
                        if (!okp || cnt != 6 || n[4] > 255 || n[5] > 255) {
                            failTxn(t, TPLC_FTP_ST_BADRESP, 0);
                            break;
                        }
                        t.pasv_addr.s_addr = htonl(
                            (n[0] << 24) | (n[1] << 16) |
                            (n[2] << 8) | n[3]);
                        t.pasv_port = uint16_t((n[4] << 8) | n[5]);
                        t.xs = XferStage::DataConnect;
                    } else { /* XferCmd */
                        if (code >= 100 && code < 200) {
                            t.xs = XferStage::Transfer;
                        } else if (code >= 200 && code < 300) {
                            /* completed without a data phase */
                            if (t.data_fd >= 0) {
                                close(t.data_fd);
                                t.data_fd = -1;
                            }
                            t.info.status = TPLC_FTP_ST_OK;
                            t.finished = true;
                        } else {
                            failTxn(t, TPLC_FTP_ST_FAIL, 0);
                        }
                    }
                    break;
                }
                case XferStage::DataConnect: {
                    if (t.data_fd < 0) {
                        int fd = socket(AF_INET, SOCK_STREAM, 0);
                        if (fd < 0) {
                            failTxn(t, TPLC_FTP_ST_ERROR, TPLC_FTP_E_IO);
                            break;
                        }
                        nonblock(fd);
                        sockaddr_in dst{};
                        dst.sin_family = AF_INET;
                        dst.sin_addr = t.pasv_addr;
                        dst.sin_port = htons(t.pasv_port);
                        if (connect(fd, reinterpret_cast<sockaddr*>(&dst),
                                    sizeof(dst)) < 0 &&
                            errno != EINPROGRESS) {
                            close(fd);
                            failTxn(t, TPLC_FTP_ST_ERROR, errno);
                            break;
                        }
                        t.data_fd = fd;
                    }
                    int err = 0;
                    socklen_t l = sizeof(err);
                    if (getsockopt(t.data_fd, SOL_SOCKET, SO_ERROR, &err,
                                   &l) != 0 || err == EINPROGRESS ||
                        err == EALREADY || err == EWOULDBLOCK) {
                        yield = true;
                        break;
                    }
                    if (err != 0) {
                        failTxn(t, TPLC_FTP_ST_ERROR,
                                err ? err : TPLC_FTP_E_IO);
                        break;
                    }
                    /* connected — issue the transfer command */
                    char cmd[200];
                    const char* verb = t.op == TPLC_FTP_XFER_RETR
                                           ? "RETR "
                                           : t.op == TPLC_FTP_XFER_STOR
                                                 ? "STOR "
                                                 : "LIST ";
                    snprintf(cmd, sizeof(cmd), "%s%s", verb, t.path);
                    if (!sendLine(c, cmd)) {
                        failTxn(t, TPLC_FTP_ST_ERROR, TPLC_FTP_E_IO);
                        break;
                    }
                    t.xs = XferStage::XferCmd;
                    break;
                }
                case XferStage::Transfer: {
                    if (t.op == TPLC_FTP_XFER_STOR) {
                        /* drain pending upload bytes */
                        while (t.up_off < t.up_len && t.data_fd >= 0) {
                            ssize_t w = send(t.data_fd,
                                             t.up_pending + t.up_off,
                                             t.up_len - t.up_off,
                                             MSG_DONTWAIT);
                            if (w > 0) {
                                t.up_off += size_t(w);
                                t.data_total += uint64_t(w);
                                t.info.total = t.data_total;
                            } else if (w < 0 && errno != EAGAIN &&
                                       errno != EWOULDBLOCK &&
                                       errno != EINTR) {
                                failTxn(t, TPLC_FTP_ST_ERROR, errno);
                                break;
                            } else {
                                break;
                            }
                        }
                        if (t.finished) break;
                        if (t.up_off >= t.up_len) {
                            t.up_len = t.up_off = 0;
                        }
                        if (t.feed_done) {
                            if (t.data_fd >= 0) {
                                shutdown(t.data_fd, SHUT_WR);
                                close(t.data_fd);
                                t.data_fd = -1;
                            }
                            t.xs = XferStage::FinishWait;
                        } else {
                            yield = true;
                        }
                    } else {
                        /* RETR/LIST: one recv per poll yields a
                           chunk to the caller */
                        if (t.data_fd >= 0 && val && val_cap) {
                            ssize_t r = recv(t.data_fd, val, val_cap,
                                             MSG_DONTWAIT);
                            if (r > 0) {
                                chunk = uint32_t(r);
                                t.data_total += uint64_t(r);
                                t.info.total = t.data_total;
                                yield = true;
                            } else if (r == 0) {
                                close(t.data_fd);
                                t.data_fd = -1;
                                t.xs = XferStage::FinishWait;
                            } else if (errno != EAGAIN &&
                                       errno != EWOULDBLOCK &&
                                       errno != EINTR) {
                                failTxn(t, TPLC_FTP_ST_ERROR, errno);
                            } else {
                                yield = true;
                            }
                        } else {
                            yield = true;
                        }
                    }
                    break;
                }
                case XferStage::FinishWait: {
                    char line[512];
                    size_t llen = 0;
                    int lr = pumpRxLine(c, line, sizeof(line), &llen);
                    if (lr < 0) {
                        failTxn(t, TPLC_FTP_ST_ERROR,
                                lr == -3 ? TPLC_FTP_E_IO
                                         : (errno ? errno : TPLC_FTP_E_IO));
                        break;
                    }
                    if (lr == 0) { yield = true; break; }
                    uint32_t code = reply_code(line);
                    if (!code || !reply_final(line)) continue;
                    t.info.resp_code = code;
                    c.resp_code = code;
                    reply_text(line, t.text, sizeof(t.text));
                    t.info.status = code >= 200 && code < 300
                                        ? TPLC_FTP_ST_OK
                                        : TPLC_FTP_ST_FAIL;
                    t.finished = true;
                    break;
                }
                case XferStage::PasvParse:
                default:
                    yield = true;
                    break;
                }
            }
        }
    }

    /* while streaming data the txn reports MORE (6) so the FB
       layer keeps BUSY without treating it as terminal */
    if (!t.finished && t.xs == XferStage::Transfer)
        t.info.status = TPLC_FTP_ST_MORE;
    if (t.finished) {
        c.active_txn = 0;
        if (t.data_fd >= 0) {
            close(t.data_fd);
            t.data_fd = -1;
        }
    }
    t.info.chunk = chunk;
    t.info.total = t.data_total;
    if (info) *info = t.info;
    if (t.finished) txns_.erase(it);
    return TPLC_FTP_OK;
}


int32_t FtpService::xferFeed(void*, TplcFtpTxnHandle h,
                             const uint8_t* buf, uint32_t len) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_FTP_E_STATE;
    Txn& t = it->second;
    if (!t.is_xfer || t.op != TPLC_FTP_XFER_STOR) return TPLC_FTP_E_STATE;
    if (t.finished || t.xs != XferStage::Transfer) return TPLC_FTP_E_STATE;
    if (!buf || len == 0) return 0;
    /* compact pending buffer then append what fits */
    if (t.up_off >= t.up_len) t.up_len = t.up_off = 0;
    size_t space = sizeof(t.up_pending) - t.up_len;
    size_t n = len < space ? len : space;
    if (n == 0) return 0;
    memcpy(t.up_pending + t.up_len, buf, n);
    t.up_len += n;
    return int32_t(n);
}

int32_t FtpService::xferFinish(void*, TplcFtpTxnHandle h) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_FTP_E_STATE;
    Txn& t = it->second;
    if (!t.is_xfer || t.op != TPLC_FTP_XFER_STOR) return TPLC_FTP_E_STATE;
    t.feed_done = true;
    return TPLC_FTP_OK;
}

void FtpService::dropTxn(Txn& t) {
    if (t.data_fd >= 0) close(t.data_fd);
    t.data_fd = -1;
}

int32_t FtpService::abort(void*, TplcFtpTxnHandle h) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_FTP_E_STATE;
    Txn& t = it->second;
    TplcFtpConnHandle ch = t.conn;
    dropTxn(t);
    txns_.erase(it);
    auto ci = conns_.find(ch);
    if (ci != conns_.end()) ci->second.active_txn = 0;
    return TPLC_FTP_OK;
}

int32_t FtpService::connClose(void*, TplcFtpConnHandle ch) {
    auto it = conns_.find(ch);
    if (it == conns_.end()) return TPLC_FTP_E_STATE;
    if (it->second.fd >= 0) close(it->second.fd);
    uint32_t at = it->second.active_txn;
    conns_.erase(it);
    if (at) {
        auto ti = txns_.find(at);
        if (ti != txns_.end()) {
            dropTxn(ti->second);
            txns_.erase(ti);
        }
    }
    return TPLC_FTP_OK;
}

} /* namespace tplc */
