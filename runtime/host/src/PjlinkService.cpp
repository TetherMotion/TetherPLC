/* PjlinkService.cpp — PJLink (JBMIA) projector client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/PjlinkService.hpp>

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
constexpr uint64_t GREET_WINDOW_NS = 400ull * 1000ull * 1000ull;
constexpr size_t CMD_CAP = 384;

uint32_t rol(uint32_t v, int s) { return (v << s) | (v >> (32 - s)); }

/* compact MD5 (RFC 1321) — only used for the PJLink auth digest */
struct Md5 {
    uint32_t a = 0x67452301, b = 0xEFCDAB89, c = 0x98BADCFE,
             d = 0x10325476;
    uint64_t bits = 0;
    uint8_t buf[64];
    size_t n = 0;

    void block(const uint8_t* p) {
        static const uint32_t K[64] = {
            0xd76aa478,0xe8c7b756,0x242070db,0xc1bdceee,0xf57c0faf,0x4787c62a,
            0xa8304613,0xfd469501,0x698098d8,0x8b44f7af,0xffff5bb1,0x895cd7be,
            0x6b901122,0xfd987193,0xa679438e,0x49b40821,0xf61e2562,0xc040b340,
            0x265e5a51,0xe9b6c7aa,0xd62f105d,0x02441453,0xd8a1e681,0xe7d3fbc8,
            0x21e1cde6,0xc33707d6,0xf4d50d87,0x455a14ed,0xa9e3e905,0xfcefa3f8,
            0x676f02d9,0x8d2a4c8a,0xfffa3942,0x8771f681,0x6d9d6122,0xfde5380c,
            0xa4beea44,0x4bdecfa9,0xf6bb4b60,0xbebfbc70,0x289b7ec6,0xeaa127fa,
            0xd4ef3085,0x04881d05,0xd9d4d039,0xe6db99e5,0x1fa27cf8,0xc4ac5665,
            0xf4292244,0x432aff97,0xab9423a7,0xfc93a039,0x655b59c3,0x8f0ccc92,
            0xffeff47d,0x85845dd1,0x6fa87e4f,0xfe2ce6e0,0xa3014314,0x4e0811a1,
            0xf7537e82,0xbd3af235,0x2ad7d2bb,0xeb86d391};
        static const uint8_t S[64] = {
            7,12,17,22,7,12,17,22,7,12,17,22,7,12,17,22,
            5,9,14,20,5,9,14,20,5,9,14,20,5,9,14,20,
            4,11,16,23,4,11,16,23,4,11,16,23,4,11,16,23,
            6,10,15,21,6,10,15,21,6,10,15,21,6,10,15,21};
        uint32_t m[16];
        for (int i = 0; i < 16; i++)
            m[i] = uint32_t(p[i*4]) | (uint32_t(p[i*4+1]) << 8) |
                   (uint32_t(p[i*4+2]) << 16) | (uint32_t(p[i*4+3]) << 24);
        uint32_t A = a, B = b, C = c, D = d;
        for (int i = 0; i < 64; i++) {
            uint32_t f; int g;
            if (i < 16)      { f = (B & C) | (~B & D); g = i; }
            else if (i < 32) { f = (D & B) | (~D & C); g = (5*i+1) & 15; }
            else if (i < 48) { f = B ^ C ^ D;          g = (3*i+5) & 15; }
            else             { f = C ^ (B | ~D);       g = (7*i) & 15; }
            uint32_t t = D;
            D = C; C = B;
            B = B + rol(A + f + K[i] + m[g], S[i]);
            A = t;
        }
        a += A; b += B; c += C; d += D;
    }
    void update(const void* data, size_t len) {
        const uint8_t* p = static_cast<const uint8_t*>(data);
        bits += uint64_t(len) * 8;
        while (len) {
            size_t take = 64 - n < len ? 64 - n : len;
            memcpy(buf + n, p, take);
            n += take; p += take; len -= take;
            if (n == 64) { block(buf); n = 0; }
        }
    }
    void final(uint8_t out[16]) {
        uint64_t lb = bits;
        uint8_t pad = 0x80, z = 0;
        update(&pad, 1);
        while (n != 56) update(&z, 1);
        uint8_t lb8[8];
        for (int i = 0; i < 8; i++) lb8[i] = uint8_t(lb >> (8*i));
        update(lb8, 8);   /* bits accounting after lb is irrelevant */
        uint32_t h[4] = {a, b, c, d};
        for (int i = 0; i < 4; i++)
            for (int j = 0; j < 4; j++)
                out[i*4+j] = uint8_t(h[i] >> (8*j));
    }
};

void md5hex(const char* s, char out[33]) {
    static const char* H = "0123456789abcdef";
    Md5 m;
    m.update(s, strlen(s));
    uint8_t d[16];
    m.final(d);
    for (int i = 0; i < 16; i++) {
        out[i*2]   = H[d[i] >> 4];
        out[i*2+1] = H[d[i] & 15];
    }
    out[32] = 0;
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

/* build `%<cls><cmd4> <param>\r` into t.out; with auth the 32-char
   digest is prepixed (computed by caller via md5hex) */
static bool cmdLine(char* dst, size_t cap, const TplcPjlinkReq& r) {
    int n = snprintf(dst, cap, "%%%u%.4s %s\r", r.cls, r.cmd, r.param);
    return n > 0 && size_t(n) < cap;
}
} /* namespace */

PjlinkService* PjlinkService::self(void* p) {
    return static_cast<PjlinkService*>(p);
}

PjlinkService::PjlinkService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcPjlinkService);
    svc_.abi_version = TPLC_PJLINK_ABI_VERSION;
    svc_.host = this;
    svc_.pjl_begin = [](void* s, const TplcPjlinkReq* r,
                        TplcPjlinkHandle* o) {
        return self(s)->begin(s, r, o);
    };
    svc_.pjl_poll = [](void* s, TplcPjlinkHandle h,
                       TplcPjlinkRespInfo* i, char* v, uint32_t vc) {
        return self(s)->poll(s, h, i, v, vc);
    };
    svc_.pjl_abort = [](void* s, TplcPjlinkHandle h) {
        return self(s)->abort(s, h);
    };
}

void PjlinkService::finish(Txn& t, int32_t status, int32_t err) {
    t.info.status = status;
    t.info.err = err;
    t.finished = true;
}

int32_t PjlinkService::begin(void*, const TplcPjlinkReq* req,
                             TplcPjlinkHandle* out) {
    if (!req || !out) return TPLC_PJLINK_E_PARAM;
    if (req->cls > 9 || strlen(req->cmd) != TPLC_PJLINK_CMD_LEN)
        return TPLC_PJLINK_E_PARAM;
    for (uint32_t i = 0; i < TPLC_PJLINK_CMD_LEN; i++)
        if (req->cmd[i] < 0x20 || req->cmd[i] > 0x7E)
            return TPLC_PJLINK_E_PARAM;
    if (strlen(req->param) > TPLC_PJLINK_MAX_PARAM ||
        strlen(req->password) > TPLC_PJLINK_MAX_PASS)
        return TPLC_PJLINK_E_RANGE;
    /* printable ASCII only — a CR/LF in param would break the
       line framing */
    for (const char* p = req->param; *p; p++)
        if (*p < 0x20 || *p > 0x7E) return TPLC_PJLINK_E_PARAM;
    for (const char* p = req->password; *p; p++)
        if (*p < 0x20 || *p > 0x7E) return TPLC_PJLINK_E_PARAM;

    struct in_addr da{};
    if (!parse_v4(req->host, &da)) return TPLC_PJLINK_E_PARAM;

    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_PJLINK_E_IO;
    if (nonblock(fd) < 0) { close(fd); return TPLC_PJLINK_E_IO; }
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));

    sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_addr = da;
    dst.sin_port = htons(req->port ? req->port : TPLC_PJLINK_PORT);
    if (connect(fd, reinterpret_cast<sockaddr*>(&dst), sizeof(dst)) < 0 &&
        errno != EINPROGRESS) {
        close(fd);
        return TPLC_PJLINK_E_IO;
    }

    uint32_t h = next_handle_++;
    if (h == 0) h = next_handle_++;
    Txn t;
    t.fd = fd;
    t.req = *req;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 2000;
    if (to > 60000) to = 60000;
    uint64_t now = clock_(clock_ctx_);
    t.deadline_ns = now + uint64_t(to) * NS_PER_MS;
    t.info.status = TPLC_PJLINK_ST_PENDING;
    txns_.emplace(h, std::move(t));
    *out = h;
    return TPLC_PJLINK_OK;
}

/* response line `%<cls><cmd4>=<value>` -> info + value text */
void PjlinkService::handleLine(Txn& t) {
    const char* p = t.in;
    size_t n = strlen(p);
    if (n >= 7 && strncmp(p, "PJLINK ", 7) == 0) {
        finish(t, strncmp(p, "PJLINK ERRA", 11) == 0
                      ? TPLC_PJLINK_ST_AUTHFAIL
                      : TPLC_PJLINK_ST_BADRESP);
        return;
    }
    if (n < 7 || p[0] != '%' || p[6] != '=' || p[1] < '0' || p[1] > '9') {
        finish(t, TPLC_PJLINK_ST_BADRESP);
        return;
    }
    t.info.resp_cls = uint32_t(p[1] - '0');
    t.info.resp_cmd = (uint32_t(uint8_t(p[2])) << 24) |
                      (uint32_t(uint8_t(p[3])) << 16) |
                      (uint32_t(uint8_t(p[4])) << 8) |
                      uint32_t(uint8_t(p[5]));
    const char* v = p + 7;
    size_t vl = strlen(v);
    t.value_total = uint32_t(vl);
    t.info.value_len = uint32_t(vl);
    size_t cp = vl > TPLC_PJLINK_MAX_VALUE ? TPLC_PJLINK_MAX_VALUE : vl;
    memcpy(t.value, v, cp);
    t.value[cp] = 0;
    finish(t, TPLC_PJLINK_ST_OK);
}

int32_t PjlinkService::poll(void*, TplcPjlinkHandle h,
                            TplcPjlinkRespInfo* info, char* val,
                            uint32_t val_cap) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_PJLINK_E_STATE;
    Txn& t = it->second;
    if (!t.finished) {
        uint64_t now = clock_(clock_ctx_);
        if (now >= t.deadline_ns) {
            finish(t, TPLC_PJLINK_ST_TIMEOUT);
        } else {
            switch (t.stage) {
            case Stage::Connect: {
                int err = 0;
                socklen_t l = sizeof(err);
                if (getsockopt(t.fd, SOL_SOCKET, SO_ERROR, &err, &l) == 0) {
                    if (err == 0) {
                        t.stage = Stage::Greet;
                        t.greet_until_ns =
                            now + GREET_WINDOW_NS < t.deadline_ns
                                ? now + GREET_WINDOW_NS
                                : t.deadline_ns;
                    } else if (err != EINPROGRESS && err != EALREADY &&
                               err != EWOULDBLOCK) {
                        finish(t, TPLC_PJLINK_ST_ERROR, err);
                    }
                }
                break;
            }
            case Stage::Greet: {
                /* feed available bytes; greeting is one CR line */
                char buf[256];
                ssize_t rn = recv(t.fd, buf, sizeof(buf), MSG_DONTWAIT);
                if (rn > 0) {
                    for (ssize_t i = 0; i < rn && !t.finished; i++) {
                        if (buf[i] == '\r') {
                            t.in[t.in_len] = 0;
                            if (strncmp(t.in, "PJLINK 1 ", 9) == 0 &&
                                strlen(t.in) >= 17) {
                                /* password auth: prepend
                                   md5(nonce + password) hex */
                                char din[64];
                                snprintf(din, sizeof(din), "%.*s%s",
                                         8, t.in + 9, t.req.password);
                                char dig[33];
                                md5hex(din, dig);
                                char line[CMD_CAP];
                                if (!cmdLine(line, sizeof(line), t.req) ||
                                    32 + strlen(line) >= sizeof(t.out)) {
                                    finish(t, TPLC_PJLINK_ST_BADRESP);
                                    break;
                                }
                                memcpy(t.out, dig, 32);
                                strcpy(t.out + 32, line);
                                t.out_len = 32 + strlen(line);
                                t.authed = true;
                                t.info.authed = 1;
                            } else if (strncmp(t.in, "PJLINK", 6) == 0) {
                                /* PJLINK 0 / unknown -> no auth */
                                if (!cmdLine(t.out, sizeof(t.out), t.req)) {
                                    finish(t, TPLC_PJLINK_ST_BADRESP);
                                    break;
                                }
                                t.out_len = strlen(t.out);
                            } else {
                                finish(t, TPLC_PJLINK_ST_BADRESP);
                                break;
                            }
                            t.stage = Stage::Send;
                            t.in_len = 0;
                            t.out_off = 0;
                            break;
                        }
                        if (t.in_len + 1 < sizeof(t.in))
                            t.in[t.in_len++] = buf[i];
                    }
                }
                /* greeting window elapsed -> legacy no-auth */
                if (!t.finished && t.stage == Stage::Greet &&
                    now >= t.greet_until_ns) {
                    if (!cmdLine(t.out, sizeof(t.out), t.req)) {
                        finish(t, TPLC_PJLINK_ST_BADRESP);
                        break;
                    }
                    t.out_len = strlen(t.out);
                    t.stage = Stage::Send;
                    t.in_len = 0;
                    t.out_off = 0;
                }
                break;
            }
            case Stage::Send: {
                if (t.out_off < t.out_len) {
                    ssize_t w = send(t.fd, t.out + t.out_off,
                                     t.out_len - t.out_off,
                                     MSG_DONTWAIT);
                    if (w > 0) {
                        t.out_off += size_t(w);
                    } else if (w < 0 && errno != EAGAIN &&
                               errno != EWOULDBLOCK && errno != EINTR) {
                        finish(t, TPLC_PJLINK_ST_ERROR, errno);
                    }
                }
                if (!t.finished && t.out_off >= t.out_len) {
                    t.stage = Stage::Recv;
                    t.in_len = 0;
                }
                break;
            }
            case Stage::Recv: {
                char buf[512];
                ssize_t rn = recv(t.fd, buf, sizeof(buf), MSG_DONTWAIT);
                if (rn > 0) {
                    for (ssize_t i = 0; i < rn; i++) {
                        if (buf[i] == '\r') {
                            t.in[t.in_len] = 0;
                            handleLine(t);
                            break;
                        }
                        if (buf[i] != '\n' &&
                            t.in_len + 1 < sizeof(t.in))
                            t.in[t.in_len++] = buf[i];
                    }
                } else if (rn == 0) {
                    finish(t, TPLC_PJLINK_ST_ERROR, 0);
                }
                break;
            }
            }
        }
    }
    if (info) *info = t.info;
    if (val && val_cap) {
        size_t cp = t.value_total < val_cap - 1 ? t.value_total
                                                : val_cap - 1;
        memcpy(val, t.value, cp);
        val[cp] = 0;
    }
    if (t.finished) {
        if (t.fd >= 0) close(t.fd);
        txns_.erase(it);
    }
    return TPLC_PJLINK_OK;
}

int32_t PjlinkService::abort(void*, TplcPjlinkHandle h) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_PJLINK_E_STATE;
    if (it->second.fd >= 0) close(it->second.fd);
    txns_.erase(it);
    return TPLC_PJLINK_OK;
}

} /* namespace tplc */
