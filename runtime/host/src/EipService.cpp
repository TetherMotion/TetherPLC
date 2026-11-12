/* EipService.cpp — tplc-eip EtherNet/IP explicit-messaging client.
 *
 * Wire model (TCP 44818):
 *   Encapsulation header (24 B): command u16, length u16,
 *     sessionHandle u32, status u32, senderContext[8], options u32.
 *   Commands: ListIdentity 0x63, RegisterSession 0x65,
 *     UnRegisterSession 0x66, SendRRData 0x6F.
 *   RegisterSession body: protocolVersion u16, optionFlags u16.
 *   SendRRData body: interfaceHandle u32(0), timeout u16, itemCount
 *     u16, [NullAddr type=0 len=0], [UnconnData type=0xB2 len+N: CIP].
 *   CIP request: service u8, reqPathSize u8(words), path, data.
 *   CIP response: replyService u8, reserved u8, generalStatus u8,
 *     statusSize u8(words), additionalStatus, responseData.
 *   Services: Get_Attribute_Single 0x0E, Set_Attribute_Single 0x10.
 *   Path segments: 0x20/0x21 class, 0x24/0x25 instance, 0x30/0x31
 *   attribute (8-bit vs 16-bit form).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/EipService.hpp>

#include <arpa/inet.h>
#include <cstring>
#include <errno.h>
#include <netdb.h>
#include <netinet/tcp.h>
#include <poll.h>
#include <sys/socket.h>
#include <unistd.h>

#include <string>
#include <vector>

namespace tplc {

namespace {

constexpr uint16_t CMD_LIST_IDENTITY   = 0x63;
constexpr uint16_t CMD_REGISTER_SESS   = 0x65;
constexpr uint16_t CMD_UNREGISTER_SESS = 0x66;
constexpr uint16_t CMD_SEND_RR_DATA    = 0x6F;

constexpr uint16_t ITEM_NULL_ADDR   = 0x0000;
constexpr uint16_t ITEM_UNCONNECTED = 0x00B2;

constexpr uint8_t SVC_GET_ATTR = 0x0E;
constexpr uint8_t SVC_SET_ATTR = 0x10;

/* one open encapsulation session. */
struct Conn {
    int      fd = -1;
    uint32_t session = 0;
    int32_t  state = 0;
    uint32_t last_cip = 0;
    uint32_t seq = 1;
};

void put16(std::vector<uint8_t>& b, uint16_t v) {
    b.push_back(uint8_t(v)); b.push_back(uint8_t(v >> 8));
}
void put32(std::vector<uint8_t>& b, uint32_t v) {
    put16(b, uint16_t(v)); put16(b, uint16_t(v >> 16));
}

/* CIP path segment for class/instance/attribute — 8-bit (<256) or
   16-bit form. tag: 0x20 class, 0x24 instance, 0x30 attribute. */
void seg(std::vector<uint8_t>& b, uint8_t tag, uint16_t v) {
    if (v < 256) { b.push_back(tag); b.push_back(uint8_t(v)); }
    else {
        b.push_back(tag | 0x01); b.push_back(0);
        put16(b, v);
    }
}

int tcpConn(const char* host, uint16_t port, int timeout_ms) {
    char portbuf[8];
    std::snprintf(portbuf, sizeof(portbuf), "%u", port);
    addrinfo hints{};
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    addrinfo* res = nullptr;
    if (getaddrinfo(host, portbuf, &hints, &res) != 0 || !res) return -1;
    int fd = -1;
    /* Keep every blocking op well under the PLC scan-stall watchdog
       (~2 s) — a stuck service must fail fast, not stall the task.
       Connect uses nonblocking + poll so it is bounded too. */
    for (addrinfo* ai = res; ai && fd < 0; ai = ai->ai_next) {
        fd = ::socket(ai->ai_family, ai->ai_socktype | SOCK_NONBLOCK,
                      ai->ai_protocol);
        if (fd < 0) continue;
        int one = 1;
        setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));
        if (::connect(fd, ai->ai_addr, ai->ai_addrlen) == 0) {
            /* connected immediately — restore blocking with timeouts */
            timeval tv{0, 150 * 1000};
            setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
            setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv));
            break;
        }
        if (errno == EINPROGRESS) {
            pollfd pfd{fd, POLLOUT, 0};
            if (::poll(&pfd, 1, 400) > 0 && (pfd.revents & POLLOUT)) {
                int err = 0; socklen_t el = sizeof(err);
                getsockopt(fd, SOL_SOCKET, SO_ERROR, &err, &el);
                if (err == 0) {
                    timeval tv{0, 150 * 1000};
                    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
                    setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv));
                    break;
                }
            }
        }
        ::close(fd); fd = -1;
    }
    freeaddrinfo(res);
    /* socket is nonblocking; recv/send may return EAGAIN — xact handles
       it by polling. Keep the SO_* timeouts as documentation. */
    if (fd >= 0) {
        timeval tv{0, 150 * 1000};
        setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
    }
    (void)timeout_ms;
    return fd;
}

/* bounded nonblocking IO: poll up to timeout_ms on EAGAIN so a stuck
   peer fails fast instead of stalling the PLC scan. */
bool pollFd(int fd, short ev, int ms) {
    pollfd pfd{fd, ev, 0};
    return ::poll(&pfd, 1, ms) > 0 && (pfd.revents & ev);
}

bool sendAll(int fd, const uint8_t* p, size_t n) {
    size_t off = 0;
    while (off < n) {
        ssize_t w = ::send(fd, p + off, n - off, MSG_NOSIGNAL);
        if (w > 0) { off += size_t(w); continue; }
        if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            if (!pollFd(fd, POLLOUT, 150)) return false;
            continue;
        }
        return false;
    }
    return true;
}

bool recvN(int fd, uint8_t* p, size_t n) {
    size_t got = 0;
    while (got < n) {
        ssize_t r = ::recv(fd, p + got, n - got, 0);
        if (r > 0) { got += size_t(r); continue; }
        if (r < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            if (!pollFd(fd, POLLIN, 150)) return false;
            continue;
        }
        return false;
    }
    return true;
}

/* full encapsulation frame: header + body. */
bool xact(int fd, uint16_t cmd, uint32_t session,
          const std::vector<uint8_t>& body,
          std::vector<uint8_t>& resp, uint32_t& status) {
    std::vector<uint8_t> f;
    put16(f, cmd);
    put16(f, uint16_t(body.size()));
    put32(f, session);
    put32(f, 0);                     /* status */
    for (int i = 0; i < 8; ++i) f.push_back(0);   /* senderContext */
    put32(f, 0);                     /* options */
    f.insert(f.end(), body.begin(), body.end());
    if (!sendAll(fd, f.data(), f.size())) return false;
    uint8_t hdr[24];
    if (!recvN(fd, hdr, 24)) return false;
    const uint16_t len = uint16_t(hdr[2] | (hdr[3] << 8));
    status = uint32_t(hdr[8] | (hdr[9] << 8) | (hdr[10] << 16) | (hdr[11] << 24));
    resp.assign(len, 0);
    if (!recvN(fd, resp.data(), len)) return false;
    return true;
}

} // namespace

struct EipService::Impl {
    Conn conns[TPLC_EIP_HDL_MAX]{};
};

EipService::EipService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcEipService);
    svc_.abi_version = TPLC_EIP_ABI_VERSION;
    svc_.host = this;
    svc_.eip_open = [](void* s, const char* h, uint32_t p, uint32_t* o)
        { return self(s)->open(s, h, p, o); };
    svc_.eip_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.eip_status = [](void* s, uint32_t h, TplcEipStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.eip_list_identity = [](void* s, const char* h, uint32_t p,
                                char* o, uint32_t cap)
        { return self(s)->listIdentity(s, h, p, o, cap); };
    svc_.eip_get_attr = [](void* s, uint32_t h, uint32_t c, uint32_t i,
                           uint32_t a, uint8_t* b, uint32_t cap)
        { return self(s)->getAttr(s, h, c, i, a, b, cap); };
    svc_.eip_set_attr = [](void* s, uint32_t h, uint32_t c, uint32_t i,
                           uint32_t a, const uint8_t* b, uint32_t n)
        { return self(s)->setAttr(s, h, c, i, a, b, n); };
}

EipService::~EipService() {
    for (Conn& c : impl_->conns)
        if (c.fd >= 0) ::close(c.fd);
}

int32_t EipService::open(void*, const char* host, uint32_t port,
                         uint32_t* handle) {
    if (!host || !*host || !handle) return TPLC_EIP_E_PARAM;
    for (uint32_t i = 0; i < TPLC_EIP_HDL_MAX; ++i) {
        if (impl_->conns[i].fd < 0) {
            int fd = tcpConn(host, port ? uint16_t(port) : 44818, 3000);
            if (fd < 0) return TPLC_EIP_E_CONN;
            /* RegisterSession: protocolVersion=1, optionFlags=0. */
            std::vector<uint8_t> body, resp;
            put16(body, 1); put16(body, 0);
            uint32_t st = 0;
            /* sessionHandle is 0 on the request; the response header
               carries the assigned id — read it from the frame. */
            std::vector<uint8_t> f;
            put16(f, CMD_REGISTER_SESS);
            put16(f, uint16_t(body.size()));
            put32(f, 0); put32(f, 0);
            for (int k = 0; k < 8; ++k) f.push_back(0);
            put32(f, 0);
            f.insert(f.end(), body.begin(), body.end());
            if (!sendAll(fd, f.data(), f.size())) {
                ::close(fd); return TPLC_EIP_E_IO;
            }
            uint8_t hdr[24];
            if (!recvN(fd, hdr, 24)) {
                ::close(fd); return TPLC_EIP_E_IO;
            }
            const uint16_t rcmd = uint16_t(hdr[0] | (hdr[1] << 8));
            const uint16_t len = uint16_t(hdr[2] | (hdr[3] << 8));
            const uint32_t sess = uint32_t(hdr[4] | (hdr[5] << 8) |
                                     (hdr[6] << 16) | (hdr[7] << 24));
            st = uint32_t(hdr[8] | (hdr[9] << 8) | (hdr[10] << 16) | (hdr[11] << 24));
            std::vector<uint8_t> drain(len);
            if (!recvN(fd, drain.data(), len)) { /* drain is best-effort */ }
            if (rcmd != CMD_REGISTER_SESS || st != 0 || sess == 0) {
                ::close(fd); return TPLC_EIP_E_CONN;
            }
            Conn& c = impl_->conns[i];
            c.fd = fd; c.session = sess; c.state = 1; c.last_cip = 0;
            *handle = i + 1;
            return TPLC_EIP_OK;
        }
    }
    return TPLC_EIP_E_HANDLE;
}

int32_t EipService::close(void*, uint32_t h) {
    if (h == 0 || h > TPLC_EIP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_EIP_E_HANDLE;
    Conn& c = impl_->conns[h - 1];
    std::vector<uint8_t> resp;
    uint32_t st = 0;
    xact(c.fd, CMD_UNREGISTER_SESS, c.session, {}, resp, st);
    ::close(c.fd);
    c.fd = -1; c.session = 0; c.state = 0;
    return TPLC_EIP_OK;
}

int32_t EipService::status(void*, uint32_t h, TplcEipStatus* out) {
    if (!out) return TPLC_EIP_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_EIP_HDL_MAX || impl_->conns[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_EIP_OK;
    }
    const Conn& c = impl_->conns[h - 1];
    out->state = c.state;
    out->session = c.session;
    out->last_cip = c.last_cip;
    return TPLC_EIP_OK;
}

int32_t EipService::listIdentity(void*, const char* host, uint32_t port,
                                 char* out, uint32_t cap) {
    if (!host || !*host || !out || cap == 0) return TPLC_EIP_E_PARAM;
    int fd = tcpConn(host, port ? uint16_t(port) : 44818, 3000);
    if (fd < 0) return TPLC_EIP_E_CONN;
    std::vector<uint8_t> resp;
    uint32_t st = 0;
    const bool ok = xact(fd, CMD_LIST_IDENTITY, 0, {}, resp, st);
    ::close(fd);
    if (!ok || resp.size() < 2) return TPLC_EIP_E_IO;
    /* ListIdentity body: itemCount u16 + CIP Identity item(s). Parse the
       first item: type u16, len u16, then the identity struct —
       encapsProtocolVer u16, socketAddr(16B), vendorId u16, deviceType
       u16, productCode u16, revision[2], status u16, serial u32,
       productNameLen u8 + name, state u8. */
    if (resp.size() < 4) return TPLC_EIP_E_IO;
    size_t off = 2;
    if (off + 4 > resp.size()) return TPLC_EIP_E_IO;
    const uint16_t ilen = uint16_t(resp[off+2] | (resp[off+3] << 8));
    off += 4;
    if (off + ilen > resp.size() || ilen < 33) return TPLC_EIP_E_IO;
    const uint8_t* d = resp.data() + off;
    const uint16_t vendor = uint16_t(d[20] | (d[21] << 8));
    const uint8_t nlen = d[32];
    const size_t n = nlen < (ilen - 33) ? nlen : (ilen - 33);
    const std::string name(reinterpret_cast<const char*>(d + 33), n);
    std::snprintf(out, cap, "vendor=%u name=%.*s", vendor,
                  int(cap > 64 ? 64 : cap - 1), name.c_str());
    return TPLC_EIP_OK;
}

namespace {

/* one unconnected SendRRData carrying a CIP request; returns the CIP
   responseData or a negative TPLC_EIP_E_*. */
int32_t sendCip(Conn& c, uint8_t svc, uint16_t cls, uint16_t inst,
                uint16_t attr, const uint8_t* data, uint32_t dlen,
                uint8_t* out, uint32_t cap) {
    std::vector<uint8_t> cip;
    cip.push_back(svc);
    std::vector<uint8_t> path;
    seg(path, 0x20, cls);
    seg(path, 0x24, inst);
    seg(path, 0x30, attr);
    cip.push_back(uint8_t(path.size() / 2));
    cip.insert(cip.end(), path.begin(), path.end());
    if (data && dlen) cip.insert(cip.end(), data, data + dlen);

    std::vector<uint8_t> body;
    put32(body, 0);                  /* interfaceHandle */
    put16(body, 10);                 /* timeout */
    put16(body, 2);                  /* itemCount */
    put16(body, ITEM_NULL_ADDR); put16(body, 0);
    put16(body, ITEM_UNCONNECTED);
    put16(body, uint16_t(cip.size()));
    body.insert(body.end(), cip.begin(), cip.end());

    std::vector<uint8_t> resp;
    uint32_t st = 0;
    if (!xact(c.fd, CMD_SEND_RR_DATA, c.session, body, resp, st) || st != 0)
        return TPLC_EIP_E_IO;
    /* resp: interfaceHandle u32, timeout u16, itemCount u16, then
       [item: type u16 len u16 data...] — second item is the UnconnData
       CIP reply. */
    if (resp.size() < 8) return TPLC_EIP_E_IO;
    size_t off = 6;
    const uint16_t items = uint16_t(resp[6] | (resp[7] << 8));
    (void)items;
    /* first item (null addr) */
    off = 8;
    if (off + 4 > resp.size()) return TPLC_EIP_E_IO;
    const uint16_t l1 = uint16_t(resp[off+2] | (resp[off+3] << 8));
    off += 4 + l1;
    if (off + 4 > resp.size()) return TPLC_EIP_E_IO;
    const uint16_t l2 = uint16_t(resp[off+2] | (resp[off+3] << 8));
    off += 4;
    if (off + l2 > resp.size() || l2 < 4) return TPLC_EIP_E_IO;
    const uint8_t* cipr = resp.data() + off;
    const uint8_t gstat = cipr[2];
    c.last_cip = gstat;
    if (gstat != 0) return TPLC_EIP_E_CIP;
    const uint8_t asz = cipr[3];
    const size_t dlen2 = l2 - 4 - size_t(asz) * 2;
    const uint8_t* dd = cipr + 4 + size_t(asz) * 2;
    const uint32_t n = dlen2 < cap ? uint32_t(dlen2) : cap;
    if (out && n) std::memcpy(out, dd, n);
    return int32_t(n);
}

} // namespace

int32_t EipService::getAttr(void*, uint32_t h, uint32_t cls,
                            uint32_t inst, uint32_t attr,
                            uint8_t* buf, uint32_t cap) {
    if (h == 0 || h > TPLC_EIP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_EIP_E_HANDLE;
    if (!buf && cap) return TPLC_EIP_E_PARAM;
    return sendCip(impl_->conns[h - 1], SVC_GET_ATTR, uint16_t(cls),
                   uint16_t(inst), uint16_t(attr), nullptr, 0, buf, cap);
}

int32_t EipService::setAttr(void*, uint32_t h, uint32_t cls,
                            uint32_t inst, uint32_t attr,
                            const uint8_t* buf, uint32_t len) {
    if (h == 0 || h > TPLC_EIP_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_EIP_E_HANDLE;
    if (!buf && len) return TPLC_EIP_E_PARAM;
    return sendCip(impl_->conns[h - 1], SVC_SET_ATTR, uint16_t(cls),
                   uint16_t(inst), uint16_t(attr), buf, len,
                   nullptr, 0);
}

} // namespace tplc
