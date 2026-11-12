/* Aes70Service.cpp — tplc-aes70 OCA client: OSCP+OCP codec over
 * TCP.  Frame: sync "OCP1" + version + dataLen + pduType + nrPdus +
 * PDUs; each PDU is an OcaMessageHeader (pduSize + headerLen +
 * msgType + handle + targetONo + methodID) + parameter bytes.
 * All socket IO nonblocking + poll-bounded under the watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Aes70Service.hpp>

#include <arpa/inet.h>
#include <cerrno>
#include <cstring>
#include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <poll.h>
#include <string>
#include <sys/socket.h>
#include <unistd.h>
#include <vector>

namespace tplc {

namespace {

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

/* big-endian helpers */
void be16(uint8_t* p, uint16_t v) { p[0] = uint8_t(v >> 8); p[1] = uint8_t(v); }
void be32(uint8_t* p, uint32_t v) {
    p[0] = uint8_t(v >> 24); p[1] = uint8_t(v >> 16);
    p[2] = uint8_t(v >> 8);  p[3] = uint8_t(v);
}
uint16_t rd16(const uint8_t* p) { return uint16_t(p[0] << 8 | p[1]); }
uint32_t rd32(const uint8_t* p) {
    return uint32_t(p[0] << 24 | p[1] << 16 | p[2] << 8 | p[3]);
}

/* ---- OCP frame: "OCP1" + version(1) + dataLen(2) + pduType(1) +
 *      nrPdus(2) + PDUs.  PDU = pduSize(2) + headerLen(1=9) +
 *      msgType(1) + handle(4) + targetONo(4) + methodID(2) + params. */
constexpr uint8_t PDU_CMD   = 0;
constexpr uint8_t PDU_KA    = 1;
constexpr uint8_t PDU_NOTIF = 2;
constexpr uint8_t PDU_RESP  = 3;

std::vector<uint8_t> buildCmd(uint32_t handle, uint32_t target_ono,
                              uint32_t method_id, const uint8_t* params,
                              uint32_t plen) {
    std::vector<uint8_t> f;
    /* pdu: 2(pduSize) + 9(hdr) + plen */
    uint16_t pduSize = uint16_t(2 + 9 + plen);
    /* OCP1 header: sync(3) + ver(1) + dlen(2) + ptype(1) + npdus(2) = 9 */
    uint16_t dlen = uint16_t(pduSize);
    f.insert(f.end(), {'O','C','P'});
    f.push_back(1);
    uint8_t b2[2]; be16(b2, dlen); f.insert(f.end(), b2, b2 + 2);
    f.push_back(PDU_CMD);
    uint8_t n2[2]; be16(n2, 1);    f.insert(f.end(), n2, n2 + 2);
    /* OcaMessageHeader */
    uint8_t sz[2]; be16(sz, pduSize); f.insert(f.end(), sz, sz + 2);
    f.push_back(9);                  /* headerLen */
    f.push_back(0);                  /* msgType = OcaCmdReq */
    uint8_t h4[4]; be32(h4, handle);     f.insert(f.end(), h4, h4 + 4);
    uint8_t t4[4]; be32(t4, target_ono); f.insert(f.end(), t4, t4 + 4);
    uint8_t m2[2]; be16(m2, uint16_t(method_id)); f.insert(f.end(), m2, m2 + 2);
    f.insert(f.end(), params, params + plen);
    return f;
}

std::vector<uint8_t> buildKeepalive() {
    /* OCP1 keepalive: sync+ver+dlen(0)+ptype(1)+npdus(0) */
    std::vector<uint8_t> f = {'O','C','P',1,0,0,PDU_KA,0,0};
    return f;
}

} // namespace

struct Aes70Service::Impl {
    struct Conn {
        int fd = -1;
        int32_t state = 0;
        uint32_t rx_total = 0, tx_total = 0;
        int32_t last_error = 0;
        uint32_t next_handle = 1;
    };
    Conn conns[TPLC_AES70_HDL_MAX]{};
};

Aes70Service::Aes70Service() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcAes70Service);
    svc_.abi_version = TPLC_AES70_ABI_VERSION;
    svc_.host = this;
    svc_.aes70_connect = [](void* s, const char* h, uint32_t p, uint32_t* o)
        { return self(s)->connect(s, h, p, o); };
    svc_.aes70_disconnect = [](void* s, uint32_t h)
        { return self(s)->disconnect(s, h); };
    svc_.aes70_status = [](void* s, uint32_t h, TplcAes70Status* o)
        { return self(s)->status(s, h, o); };
    svc_.aes70_call = [](void* s, uint32_t h, uint32_t t, uint32_t m,
                         const uint8_t* p, uint32_t n, uint32_t* o2)
        { return self(s)->call(s, h, t, m, p, n, o2); };
    svc_.aes70_keepalive = [](void* s, uint32_t h)
        { return self(s)->keepalive(s, h); };
    svc_.aes70_receive = [](void* s, uint32_t h, uint32_t* o2,
                            uint32_t* sc, uint8_t* pl, uint32_t cap)
        { return self(s)->receive(s, h, o2, sc, pl, cap); };
}

Aes70Service::~Aes70Service() {
    for (auto& c : impl_->conns)
        if (c.fd >= 0) close(c.fd);
}

int32_t Aes70Service::connect(void*, const char* host, uint32_t port,
                              uint32_t* out_handle) {
    if (!host || !*host || !out_handle) return TPLC_AES70_E_PARAM;
    for (uint32_t i = 0; i < TPLC_AES70_HDL_MAX; ++i) {
        if (impl_->conns[i].fd < 0) {
            int fd = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
            if (fd < 0) return TPLC_AES70_E_CONN;
            sockaddr_in a{};
            a.sin_family = AF_INET;
            a.sin_port = htons(uint16_t(port ? port : 65000));
            if (inet_pton(AF_INET, host, &a.sin_addr) != 1) {
                hostent* he = gethostbyname(host);
                if (!he) { close(fd); return TPLC_AES70_E_CONN; }
                memcpy(&a.sin_addr, he->h_addr, 4);
            }
            if (::connect(fd, (sockaddr*)&a, sizeof(a)) < 0 &&
                errno != EINPROGRESS) { close(fd); return TPLC_AES70_E_CONN; }
            if (!waitFd(fd, POLLOUT, kPollMs)) { close(fd); return TPLC_AES70_E_CONN; }
            int err = 0; socklen_t l = sizeof(err);
            getsockopt(fd, SOL_SOCKET, SO_ERROR, &err, &l);
            if (err) { close(fd); return TPLC_AES70_E_CONN; }
            auto& c = impl_->conns[i];
            c.fd = fd; c.state = 1; c.next_handle = 1;
            *out_handle = i + 1;
            return TPLC_AES70_OK;
        }
    }
    return TPLC_AES70_E_HANDLE;
}

int32_t Aes70Service::disconnect(void*, uint32_t h) {
    if (h == 0 || h > TPLC_AES70_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_AES70_E_HANDLE;
    auto& c = impl_->conns[h - 1];
    close(c.fd); c.fd = -1; c.state = 0;
    return TPLC_AES70_OK;
}

int32_t Aes70Service::status(void*, uint32_t h, TplcAes70Status* out) {
    if (!out) return TPLC_AES70_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_AES70_HDL_MAX || impl_->conns[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_AES70_OK;
    }
    const auto& c = impl_->conns[h - 1];
    out->state = c.state;
    out->rx_total = c.rx_total;
    out->tx_total = c.tx_total;
    out->last_error = c.last_error;
    return TPLC_AES70_OK;
}

int32_t Aes70Service::call(void*, uint32_t h, uint32_t target_ono,
                           uint32_t method_id, const uint8_t* params,
                           uint32_t plen, uint32_t* out_handle2) {
    if (h == 0 || h > TPLC_AES70_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_AES70_E_HANDLE;
    if (!params && plen) return TPLC_AES70_E_PARAM;
    auto& c = impl_->conns[h - 1];
    uint32_t handle = c.next_handle++;
    auto frame = buildCmd(handle, target_ono, method_id, params, plen);
    if (!sendAll(c.fd, frame.data(), frame.size())) {
        c.last_error = errno; return TPLC_AES70_E_PROTO;
    }
    c.tx_total++;
    if (out_handle2) *out_handle2 = handle;
    return TPLC_AES70_OK;
}

int32_t Aes70Service::keepalive(void*, uint32_t h) {
    if (h == 0 || h > TPLC_AES70_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_AES70_E_HANDLE;
    auto& c = impl_->conns[h - 1];
    auto f = buildKeepalive();
    if (!sendAll(c.fd, f.data(), f.size())) {
        c.last_error = errno; return TPLC_AES70_E_PROTO;
    }
    c.tx_total++;
    return TPLC_AES70_OK;
}

int32_t Aes70Service::receive(void*, uint32_t h, uint32_t* out_handle2,
                              uint32_t* out_status, uint8_t* payload,
                              uint32_t cap) {
    if (h == 0 || h > TPLC_AES70_HDL_MAX || impl_->conns[h - 1].fd < 0)
        return TPLC_AES70_E_HANDLE;
    auto& c = impl_->conns[h - 1];
    if (!waitFd(c.fd, POLLIN, 0)) return TPLC_AES70_E_NOMSG;
    /* OCP1 sync header: 3+1+2+1+2 = 9 bytes */
    uint8_t hdr[9];
    if (!recvN(c.fd, hdr, 9)) { c.last_error = errno; return TPLC_AES70_E_PROTO; }
    if (hdr[0] != 'O' || hdr[1] != 'C' || hdr[2] != 'P' || hdr[3] != 1) {
        c.last_error = -1; return TPLC_AES70_E_PROTO;
    }
    uint16_t dlen = rd16(hdr + 4);
    uint8_t ptype = hdr[6];
    uint16_t npdus = rd16(hdr + 7);
    c.rx_total++;
    if (ptype == PDU_KA || npdus == 0) {
        if (dlen > 0) { uint8_t skip[64]; if (!recvN(c.fd, skip, dlen)) return TPLC_AES70_E_PROTO; }
        return TPLC_AES70_E_NOMSG;
    }
    /* read the PDU payload (dlen bytes) */
    if (dlen > 65535) return TPLC_AES70_E_PROTO;
    std::vector<uint8_t> pdu(dlen);
    if (!recvN(c.fd, pdu.data(), dlen)) { c.last_error = errno; return TPLC_AES70_E_PROTO; }
    /* OcaMessageHeader: pduSize(2)+hdrLen(1)+msgType(1)+handle(4)+
       targetONo(4)+methodID(2)+status(1)+params */
    if (dlen < 14) return TPLC_AES70_E_PROTO;
    uint16_t hdrLen = pdu[2];
    uint8_t msgType = pdu[3];
    uint32_t handle = rd32(pdu.data() + 4);
    /* status code is the byte after the 2B methodID at hdrLen */
    uint8_t status = pdu[hdrLen - 1];   /* last byte of header = status? */
    /* params start after the header */
    uint32_t plen = dlen > hdrLen ? dlen - hdrLen : 0;
    const uint8_t* params = pdu.data() + hdrLen;
    if (msgType != 3 && msgType != 2) return TPLC_AES70_E_NOMSG;
    if (out_handle2) *out_handle2 = handle;
    if (out_status) *out_status = status;
    uint32_t n = plen < cap ? plen : cap;
    if (payload && n) std::memcpy(payload, params, n);
    return int32_t(n);
}

} // namespace tplc
