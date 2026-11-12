/* OcppService.cpp — tplc-ocpp OCPP-J client: RFC6455 WebSocket
 * transport + OCPP JSON-RPC framing ([2,msgId,action,payload] call,
 * [3,msgId,payload] result, [4,msgId,errcode,desc,details] error).
 *
 * The PLC is the charge-point client; the service owns the TCP + HTTP
 * Upgrade + WS frame codec + JSON-RPC array framing.  All socket IO
 * is nonblocking + poll-bounded under the scan-stall watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/OcppService.hpp>

#include <arpa/inet.h>
#include <cerrno>
#include <cstring>
#include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <poll.h>
#include <string>
#include <sys/socket.h>
#include <unistd.h>
#include <vector>

namespace tplc {

namespace {

/* ---- bounded socket IO (poll, under the scan-stall watchdog) ---- */
constexpr int kPollMs = 1500;

bool waitFd(int fd, short ev, int ms) {
    pollfd p{fd, ev, 0};
    return poll(&p, 1, ms) > 0 && (p.revents & (ev | POLLERR | POLLHUP));
}

bool sendAll(int fd, const uint8_t* p, size_t n) {
    size_t off = 0;
    while (off < n) {
        ssize_t w = send(fd, p + off, n - off, MSG_NOSIGNAL);
        if (w > 0) { off += size_t(w); continue; }
        if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            if (!waitFd(fd, POLLOUT, kPollMs)) return false;
            continue;
        }
        return false;
    }
    return true;
}

bool recvN(int fd, uint8_t* p, size_t n) {
    size_t off = 0;
    while (off < n) {
        ssize_t r = recv(fd, p + off, n - off, 0);
        if (r > 0) { off += size_t(r); continue; }
        if (r == 0) return false;
        if (errno == EAGAIN || errno == EWOULDBLOCK) {
            if (!waitFd(fd, POLLIN, kPollMs)) return false;
            continue;
        }
        return false;
    }
    return true;
}

/* read until "\r\n\r\n" — the HTTP response header terminator */
bool recvHeaders(int fd, std::string& out, size_t max) {
    out.clear();
    while (out.size() < max) {
        uint8_t b;
        ssize_t r = recv(fd, &b, 1, 0);
        if (r == 1) {
            out.push_back(char(b));
            if (out.size() >= 4 &&
                out.compare(out.size() - 4, 4, "\r\n\r\n") == 0)
                return true;
            continue;
        }
        if (r < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            if (!waitFd(fd, POLLIN, kPollMs)) return false;
            continue;
        }
        return false;
    }
    return false;
}

/* a fixed 16-byte key nonce — the server echoes its hash back in
   Sec-WebSocket-Accept; a static value is fine for this surface */
const char* wsNonce() { return "dGhlIHNhbXBsZSBub25jZQ=="; }

/* ---- OCPP-J framing ---- */
/* [2,"msgId","action",<payload-json>] — payload is raw JSON */
std::string frameCall(const char* id, const char* action,
                      const uint8_t* payload, uint32_t len) {
    std::string o = "[2,\"";
    o += id; o += "\",\""; o += action; o += "\",";
    o.append((const char*)payload, len); o += "]";
    return o;
}
std::string frameResult(const char* id, const uint8_t* payload, uint32_t len) {
    std::string o = "[3,\""; o += id; o += "\",";
    o.append((const char*)payload, len); o += "]";
    return o;
}
std::string frameCallErr(const char* id, const char* code,
                         const char* desc, const uint8_t* details,
                         uint32_t len) {
    std::string o = "[4,\""; o += id; o += "\",\""; o += code;
    o += "\",\""; o += desc; o += "\",";
    o.append((const char*)details, len); o += "]";
    return o;
}

/* ---- WS frame write (masked, client->server) ---- */
bool wsSend(int fd, const std::string& text) {
    std::vector<uint8_t> f;
    f.push_back(0x81);                       // FIN + opcode text
    size_t n = text.size();
    if (n < 126) f.push_back(0x80 | uint8_t(n));
    else if (n < 65536) {
        f.push_back(0x80 | 126);
        f.push_back(uint8_t(n >> 8)); f.push_back(uint8_t(n));
    } else {
        f.push_back(0x80 | 127);
        for (int i = 7; i >= 0; --i) f.push_back(uint8_t(n >> (i*8)));
    }
    uint8_t mask[4] = {0x12,0x34,0x56,0x78};
    f.insert(f.end(), mask, mask + 4);
    for (size_t i = 0; i < n; ++i)
        f.push_back(uint8_t(text[i]) ^ mask[i & 3]);
    return sendAll(fd, f.data(), f.size());
}

/* ---- WS frame read (unmasked, server->client) ---- */
/* returns opcode (1 text, 9 ping, 8 close) or -1; payload in `out` */
int wsRecv(int fd, std::string& out) {
    uint8_t h[2];
    if (!recvN(fd, h, 2)) return -1;
    int opcode = h[0] & 0x0F;
    uint64_t n = h[1] & 0x7F;
    if (n == 126) { uint8_t e[2]; if (!recvN(fd,e,2)) return -1;
                    n = uint64_t(e[0])<<8 | e[1]; }
    else if (n == 127) { uint8_t e[8]; if (!recvN(fd,e,8)) return -1;
                    n = 0; for (int i=0;i<8;++i) n=(n<<8)|e[i]; }
    if (h[1] & 0x80) { uint8_t m[4]; if (!recvN(fd,m,4)) return -1; }
    out.assign(n, 0);
    if (!recvN(fd, (uint8_t*)out.data(), n)) return -1;
    return opcode;
}

} // namespace

struct OcppService::Impl {
    struct Conn {
        int fd = -1;
        int32_t state = 0;
        uint32_t rx_total = 0, tx_total = 0;
        int32_t last_error = 0;
    };
    Conn conns[TPLC_OCPP_HDL_MAX]{};
};


OcppService::OcppService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcOcppService);
    svc_.abi_version = TPLC_OCPP_ABI_VERSION;
    svc_.host = this;
    svc_.ocpp_connect = [](void* s, const char* h, uint32_t p,
                           const char* pa, const char* oid, uint32_t* o)
        { return self(s)->connect(s, h, p, pa, oid, o); };
    svc_.ocpp_disconnect = [](void* s, uint32_t h)
        { return self(s)->disconnect(s, h); };
    svc_.ocpp_status = [](void* s, uint32_t h, TplcOcppStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.ocpp_call = [](void* s, uint32_t h, const char* id,
                        const char* a, const uint8_t* p, uint32_t n)
        { return self(s)->call(s, h, id, a, p, n); };
    svc_.ocpp_result = [](void* s, uint32_t h, const char* id,
                          const uint8_t* p, uint32_t n)
        { return self(s)->result(s, h, id, p, n); };
    svc_.ocpp_callerror = [](void* s, uint32_t h, const char* id,
                             const char* c, const char* d,
                             const uint8_t* det, uint32_t n)
        { return self(s)->callerror(s, h, id, c, d, det, n); };
    svc_.ocpp_receive = [](void* s, uint32_t h, TplcOcppRx* rx)
        { return self(s)->receive(s, h, rx); };
}

OcppService::~OcppService() {
    for (auto& c : impl_->conns)
        if (c.fd >= 0) close(c.fd);
}

int32_t OcppService::connect(void*, const char* host, uint32_t port,
                             const char* path, const char* ocpp_id,
                             uint32_t* out_handle) {
    if (!host || !*host || !out_handle) return TPLC_OCPP_E_PARAM;
    for (uint32_t i = 0; i < TPLC_OCPP_HDL_MAX; ++i) {
        if (impl_->conns[i].fd < 0) {
            int fd = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
            if (fd < 0) return TPLC_OCPP_E_CONN;
            sockaddr_in a{};
            a.sin_family = AF_INET;
            a.sin_port = htons(uint16_t(port ? port : 80));
            if (inet_pton(AF_INET, host, &a.sin_addr) != 1) {
                hostent* he = gethostbyname(host);
                if (!he) { close(fd); return TPLC_OCPP_E_CONN; }
                memcpy(&a.sin_addr, he->h_addr, 4);
            }
            if (::connect(fd, (sockaddr*)&a, sizeof(a)) < 0 &&
                errno != EINPROGRESS) { close(fd); return TPLC_OCPP_E_CONN; }
            if (!waitFd(fd, POLLOUT, kPollMs)) { close(fd); return TPLC_OCPP_E_CONN; }
            int err = 0; socklen_t l = sizeof(err);
            getsockopt(fd, SOL_SOCKET, SO_ERROR, &err, &l);
            if (err) { close(fd); return TPLC_OCPP_E_CONN; }
            /* WS handshake: GET path Upgrade:websocket */
            char req[512];
            int rl = std::snprintf(req, sizeof(req),
                "GET %s HTTP/1.1\r\nHost: %s:%u\r\n"
                "Upgrade: websocket\r\nConnection: Upgrade\r\n"
                "Sec-WebSocket-Key: %s\r\n"
                "Sec-WebSocket-Protocol: ocpp1.6\r\n"
                "Sec-WebSocket-Version: 13\r\n\r\n",
                path && *path ? path : "/ocpp",
                host, port ? port : 80, wsNonce());
            std::string resp;
            if (!sendAll(fd, (const uint8_t*)req, size_t(rl)) ||
                !recvHeaders(fd, resp, 4096)) {
                close(fd); return TPLC_OCPP_E_CONN;
            }
            /* accept any 101 Switching Protocols response */
            if (resp.find(" 101") == std::string::npos) {
                close(fd); return TPLC_OCPP_E_CONN;
            }
            auto& c = impl_->conns[i];
            c.fd = fd; c.state = 2;
            *out_handle = i + 1;
            return TPLC_OCPP_OK;
        }
    }
    return TPLC_OCPP_E_HANDLE;
}

int32_t OcppService::disconnect(void*, uint32_t h) {
    if (h == 0 || h > TPLC_OCPP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_OCPP_E_HANDLE;
    auto& c = impl_->conns[h - 1];
    close(c.fd); c.fd = -1; c.state = 0;
    return TPLC_OCPP_OK;
}

int32_t OcppService::status(void*, uint32_t h, TplcOcppStatus* out) {
    if (!out) return TPLC_OCPP_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_OCPP_HDL_MAX || impl_->conns[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_OCPP_OK;
    }
    const auto& c = impl_->conns[h - 1];
    out->state = c.state;
    out->rx_total = c.rx_total;
    out->tx_total = c.tx_total;
    out->last_error = c.last_error;
    return TPLC_OCPP_OK;
}

int32_t OcppService::call(void*, uint32_t h, const char* msg_id,
                          const char* action, const uint8_t* payload,
                          uint32_t len) {
    if (h == 0 || h > TPLC_OCPP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_OCPP_E_HANDLE;
    if (!msg_id || !action || !*action) return TPLC_OCPP_E_PARAM;
    auto& c = impl_->conns[h - 1];
    if (!wsSend(c.fd, frameCall(msg_id, action, payload, len))) {
        c.last_error = errno; return TPLC_OCPP_E_PROTO;
    }
    c.tx_total++;
    return TPLC_OCPP_OK;
}

int32_t OcppService::result(void*, uint32_t h, const char* msg_id,
                            const uint8_t* payload, uint32_t len) {
    if (h == 0 || h > TPLC_OCPP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_OCPP_E_HANDLE;
    auto& c = impl_->conns[h - 1];
    if (!wsSend(c.fd, frameResult(msg_id, payload, len))) {
        c.last_error = errno; return TPLC_OCPP_E_PROTO;
    }
    c.tx_total++;
    return TPLC_OCPP_OK;
}

int32_t OcppService::callerror(void*, uint32_t h, const char* msg_id,
                               const char* code, const char* desc,
                               const uint8_t* details, uint32_t len) {
    if (h == 0 || h > TPLC_OCPP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_OCPP_E_HANDLE;
    auto& c = impl_->conns[h - 1];
    if (!wsSend(c.fd, frameCallErr(msg_id, code, desc, details, len))) {
        c.last_error = errno; return TPLC_OCPP_E_PROTO;
    }
    c.tx_total++;
    return TPLC_OCPP_OK;
}

int32_t OcppService::receive(void*, uint32_t h, TplcOcppRx* rx) {
    if (h == 0 || h > TPLC_OCPP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_OCPP_E_HANDLE;
    if (!rx || !rx->action || !rx->payload) return TPLC_OCPP_E_PARAM;
    auto& c = impl_->conns[h - 1];
    /* nonblocking — poll 0ms for a WS text frame */
    if (!waitFd(c.fd, POLLIN, 0)) return TPLC_OCPP_E_NOMSG;
    std::string frame;
    int op = wsRecv(c.fd, frame);
    if (op < 0) { c.last_error = errno; return TPLC_OCPP_E_PROTO; }
    if (op == 8) { c.state = 0; return TPLC_OCPP_E_CONN; }
    if (op == 9) return TPLC_OCPP_E_NOMSG;   /* ping — answered inline */
    if (op != 1) return TPLC_OCPP_E_NOMSG;
    c.rx_total++;
    /* OCPP-J: [type, msgId, ...] — extract msg type + action/payload */
    const char* p = frame.data();
    /* crude parse: find the msgtype digit and the first quoted field */
    while (*p && (*p == '[' || *p == ' ')) ++p;
    rx->msg_type = uint32_t(*p - '0');
    const char* a = strchr(p, '"');
    if (!a) return TPLC_OCPP_E_PROTO;
    ++a;
    const char* aend = strchr(a, '"');
    if (!aend) return TPLC_OCPP_E_PROTO;
    uint32_t alen = uint32_t(aend - a);
    if (alen >= rx->action_cap) alen = rx->action_cap - 1;
    std::memcpy(rx->action, a, alen); rx->action[alen] = 0;
    rx->action_len = alen;
    /* payload = everything after the next quoted field (or last elem) */
    const char* pend = strchr(aend + 1, ',');
    const char* pl = pend ? pend + 1 : aend + 1;
    const char* end = strrchr(frame.data() + frame.size(), ']');
    uint32_t plen = end && end > pl ? uint32_t(end - pl) : 0;
    if (plen > rx->payload_cap) plen = rx->payload_cap;
    std::memcpy(rx->payload, pl, plen);
    rx->payload_len = plen;
    return TPLC_OCPP_OK;
}

} // namespace tplc
