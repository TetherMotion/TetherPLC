/* AdsService.cpp — AMS/TCP ADS client service.
 * Wire format (Beckhoff ADS spec): 6-byte TCP header (u16 reserved,
 * u32 ams_len), 32-byte AMS header, ADS payload. Little-endian.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/AdsService.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

namespace tplc {

namespace {
constexpr size_t AMS_HDR_LEN = 32;
constexpr size_t FRAME_HDR  = 6 + AMS_HDR_LEN; /* tcp + ams header   */
constexpr size_t MAX_FRAME  = 8192;
constexpr uint16_t ST_COMMAND = 0x0004;  /* request state flag    */
constexpr uint16_t ST_RESPOND = 0x0001;  /* response state flag   */
/* source AMS identity: placeholder NetID; real AMS routing needs a
   configured local NetID (documented limitation) */
const char* SRC_NETID = "127.0.0.1.1.1";
constexpr uint16_t SRC_PORT = 0;
} /* namespace */

static AdsService* self(void* p) { return static_cast<AdsService*>(p); }

AdsService::AdsService() {
    svc_.struct_size        = sizeof(TplcAdsService);
    svc_.abi_version        = TPLC_ADS_ABI_VERSION;
    svc_.host               = this;
    svc_.ads_open           = [](void* s, const char* h, uint16_t p,
                                 TplcAdsHandle* o) {
        return self(s)->adsOpen(s, h, p, o);
    };
    svc_.ads_read           = [](void* s, TplcAdsHandle h, uint32_t g,
                                 uint32_t o, uint32_t l, void* b,
                                 uint32_t c, uint32_t* rl, uint32_t* ae) {
        return self(s)->adsRead(s, h, g, o, l, b, c, rl, ae);
    };
    svc_.ads_write          = [](void* s, TplcAdsHandle h, uint32_t g,
                                 uint32_t o, const void* d, uint32_t l,
                                 uint32_t* ae) {
        return self(s)->adsWrite(s, h, g, o, d, l, ae);
    };
    svc_.ads_read_state     = [](void* s, TplcAdsHandle h, uint32_t* as,
                                 uint32_t* ds, uint32_t* ae) {
        return self(s)->adsReadState(s, h, as, ds, ae);
    };
    svc_.ads_write_control  = [](void* s, TplcAdsHandle h, uint32_t as,
                                 uint32_t ds, uint32_t* ae) {
        return self(s)->adsWriteControl(s, h, as, ds, ae);
    };
    svc_.ads_close          = [](void* s, TplcAdsHandle h) {
        return self(s)->adsClose(s, h);
    };
}

void AdsService::put16(std::vector<uint8_t>& v, uint16_t x) {
    v.push_back(x & 0xff);
    v.push_back((x >> 8) & 0xff);
}
void AdsService::put32(std::vector<uint8_t>& v, uint32_t x) {
    for (int i = 0; i < 4; i++) v.push_back((x >> (8 * i)) & 0xff);
}
uint16_t AdsService::get16(const uint8_t* p) {
    return (uint16_t)(p[0] | (p[1] << 8));
}
uint32_t AdsService::get32(const uint8_t* p) {
    return (uint32_t)p[0] | ((uint32_t)p[1] << 8) |
           ((uint32_t)p[2] << 16) | ((uint32_t)p[3] << 24);
}
void AdsService::putNetId(std::vector<uint8_t>& v, const char* netid) {
    /* "a.b.c.d.e.f" -> six raw bytes */
    unsigned vals[6] = {};
    if (netid) {
        for (int i = 0; i < 6 && *netid; i++) {
            char* end = nullptr;
            vals[i] = (unsigned)strtoul(netid, &end, 10);
            netid = (*end == '.') ? end + 1 : end;
        }
    }
    for (int i = 0; i < 6; i++) v.push_back((uint8_t)(vals[i] & 0xff));
}

int32_t AdsService::mapErr(int e) {
    switch (e) {
    case EAGAIN:         return TPLC_ADS_E_AGAIN;
    case EBADF:
    case ENOTCONN:       return TPLC_ADS_E_CLOSED;
    case ECONNRESET:
    case EPIPE:          return TPLC_ADS_E_CLOSED;
    case ECONNREFUSED:
    case EHOSTUNREACH:
    case ENETUNREACH:    return TPLC_ADS_E_CONNECT;
    case EINVAL:         return TPLC_ADS_E_PARAM;
    default:             return TPLC_ADS_E_IO;
    }
}

AdsService::Conn* AdsService::find(TplcAdsHandle h) {
    auto it = conns_.find(h);
    return it == conns_.end() ? nullptr : &it->second;
}

void AdsService::drop(TplcAdsHandle h) {
    auto it = conns_.find(h);
    if (it != conns_.end()) {
        if (it->second.fd >= 0) ::close(it->second.fd);
        conns_.erase(it);
    }
}

int32_t AdsService::ensureConnected(Conn& c) {
    if (c.cstate != 0) return TPLC_ADS_OK;
    int err = 0;
    socklen_t l = sizeof(err);
    if (getsockopt(c.fd, SOL_SOCKET, SO_ERROR, &err, &l) < 0)
        return mapErr(errno);
    if (err == 0) {
        c.cstate = 1;
        return TPLC_ADS_OK;
    }
    if (err == EINPROGRESS || err == EALREADY)
        return TPLC_ADS_E_AGAIN;
    return TPLC_ADS_E_CONNECT;
}

/* drive one request/response exchange on conn c. req_payload is the
   ADS command data; on success resp holds the response payload and
   the function returns TPLC_ADS_OK. */
int32_t AdsService::exchange(Conn& c, uint16_t cmd,
                             const std::vector<uint8_t>& req_payload,
                             std::vector<uint8_t>& resp) {
    int32_t rc = ensureConnected(c);
    if (rc != TPLC_ADS_OK) return rc;

    if (c.cstate == 1) {
        /* build request: tcp hdr + ams hdr + payload */
        c.pending.clear();
        c.pending.reserve(FRAME_HDR + req_payload.size());
        put16(c.pending, 0);                    /* reserved */
        put32(c.pending, (uint32_t)(AMS_HDR_LEN + req_payload.size()));
        putNetId(c.pending, c.tnetid.c_str());
        put16(c.pending, c.tport);
        putNetId(c.pending, SRC_NETID);
        put16(c.pending, SRC_PORT);
        put16(c.pending, cmd);
        put16(c.pending, ST_COMMAND);
        put32(c.pending, (uint32_t)req_payload.size());
        put32(c.pending, 0);                    /* error code */
        c.invoke++;
        if (c.invoke == 0) c.invoke = 1;
        put32(c.pending, c.invoke);
        c.pending.insert(c.pending.end(),
                         req_payload.begin(), req_payload.end());
        c.sent = 0;
        c.cmd = cmd;
        c.cstate = 2;
    }

    /* finish sending */
    while (c.sent < c.pending.size()) {
        ssize_t n = ::send(c.fd, c.pending.data() + c.sent,
                           c.pending.size() - c.sent, MSG_NOSIGNAL);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK)
                return TPLC_ADS_E_AGAIN;
            return mapErr(errno);
        }
        c.sent += (size_t)n;
    }

    /* receive until a complete frame or EAGAIN */
    for (;;) {
        if (c.rx.size() >= 6) {
            uint32_t ams_len = get32(c.rx.data() + 2);
            if (ams_len < AMS_HDR_LEN || ams_len > MAX_FRAME)
                return TPLC_ADS_E_PROTO;
            size_t need = 6 + (size_t)ams_len;
            if (c.rx.size() >= need) {
                /* validate + extract payload */
                const uint8_t* h = c.rx.data() + 6;
                uint16_t rcmd = get16(h + 16);
                uint16_t rflags = get16(h + 18);
                uint32_t rinvoke = get32(h + 28);
                uint32_t rdlen = get32(h + 20);
                if (rcmd != c.cmd || rinvoke != c.invoke ||
                    !(rflags & ST_RESPOND))
                    return TPLC_ADS_E_PROTO;
                if (rdlen > ams_len - AMS_HDR_LEN)
                    return TPLC_ADS_E_PROTO;
                resp.assign(c.rx.begin() + FRAME_HDR,
                            c.rx.begin() + FRAME_HDR + rdlen);
                c.rx.clear();
                c.cstate = 1;
                return TPLC_ADS_OK;
            }
        }
        if (c.rx.size() >= MAX_FRAME) return TPLC_ADS_E_PROTO;
        uint8_t buf[512];
        ssize_t n = ::recv(c.fd, buf, sizeof(buf), 0);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK)
                return TPLC_ADS_E_AGAIN;
            return mapErr(errno);
        }
        if (n == 0) return TPLC_ADS_E_CLOSED;
        c.rx.insert(c.rx.end(), buf, buf + n);
    }
}

int32_t AdsService::adsOpen(void* s, const char* host, uint16_t port,
                            TplcAdsHandle* out) {
    (void)s;
    if (!out || !host || !*host || port == 0) return TPLC_ADS_E_PARAM;
    struct in_addr a4{};
    if (inet_pton(AF_INET, host, &a4) != 1) return TPLC_ADS_E_PARAM;

    int fd = ::socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if (fd < 0) return TPLC_ADS_E_IO;
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));
    struct sockaddr_in sa{};
    sa.sin_family = AF_INET;
    sa.sin_port = htons(port);
    sa.sin_addr = a4;
    if (::connect(fd, (struct sockaddr*)&sa, sizeof(sa)) < 0 &&
        errno != EINPROGRESS) {
        int e = errno;
        ::close(fd);
        return mapErr(e);
    }
    TplcAdsHandle h = next_handle_++;
    if (h == TPLC_ADS_HANDLE_INVALID) h = next_handle_++;
    Conn c;
    c.fd = fd;
    c.tport = port;
    /* Beckhoff convention: a device at IPv4 a.b.c.d usually has AMS
       NetID a.b.c.d.1.1 */
    c.tnetid = std::string(host) + ".1.1";
    conns_[h] = std::move(c);
    *out = h;
    return TPLC_ADS_OK;
}

/* shared op driver: builds payload, exchanges into the caller-owned
   respbuf, parses the ADS result. On success the data/dlen out-params
   point into respbuf (valid until the caller's next exchange). */
int32_t AdsService::opResult(Conn& c, const std::vector<uint8_t>& payload,
                             uint16_t cmd, uint32_t* adserr,
                             const uint8_t** data, uint32_t* dlen,
                             std::vector<uint8_t>& respbuf) {
    int32_t rc = exchange(c, cmd, payload, respbuf);
    if (rc != TPLC_ADS_OK) return rc;
    uint32_t err = 0;
    if (respbuf.size() >= 4) {
        err = get32(respbuf.data());
        if (data) {
            *data = respbuf.data() + 4;
            *dlen = (uint32_t)respbuf.size() - 4;
        }
    }
    if (adserr) *adserr = err;
    return TPLC_ADS_OK;
}

int32_t AdsService::adsRead(void* s, TplcAdsHandle h, uint32_t grp,
                            uint32_t off, uint32_t len, void* buf,
                            uint32_t cap, uint32_t* rlen,
                            uint32_t* adserr) {
    (void)s;
    Conn* c = find(h);
    if (!c || !rlen || !adserr || (len && !buf)) return TPLC_ADS_E_PARAM;
    if (c->cstate == 2 && c->cmd != TPLC_ADS_CMD_READ)
        return TPLC_ADS_E_STATE; /* different op in flight */
    std::vector<uint8_t> p;
    put32(p, grp); put32(p, off); put32(p, len);
    std::vector<uint8_t> resp;
    const uint8_t* d = nullptr;
    uint32_t dl = 0;
    int32_t rc = opResult(*c, p, TPLC_ADS_CMD_READ, adserr, &d, &dl, resp);
    if (rc != TPLC_ADS_OK) return rc;
    /* response payload: result u32, length u32, data[n] — opResult
       already stripped result, so d points at length+data */
    uint32_t n = 0;
    if (dl >= 4) {
        n = get32(d);
        d += 4;
        dl -= 4;
    }
    if (*adserr == 0) {
        uint32_t cp = n < dl ? n : dl;
        if (cp > cap) cp = cap;
        if (cp && buf) memcpy(buf, d, cp);
        *rlen = cp;
    } else {
        *rlen = 0;
    }
    return TPLC_ADS_OK;
}

int32_t AdsService::adsWrite(void* s, TplcAdsHandle h, uint32_t grp,
                             uint32_t off, const void* data, uint32_t len,
                             uint32_t* adserr) {
    (void)s;
    Conn* c = find(h);
    if (!c || !adserr || (len && !data)) return TPLC_ADS_E_PARAM;
    if (c->cstate == 2 && c->cmd != TPLC_ADS_CMD_WRITE)
        return TPLC_ADS_E_STATE;
    std::vector<uint8_t> p;
    put32(p, grp); put32(p, off); put32(p, len);
    if (len) {
        const uint8_t* d = (const uint8_t*)data;
        p.insert(p.end(), d, d + len);
    }
    std::vector<uint8_t> resp;
    return opResult(*c, p, TPLC_ADS_CMD_WRITE, adserr,
                    nullptr, nullptr, resp);
}

int32_t AdsService::adsReadState(void* s, TplcAdsHandle h,
                                 uint32_t* ads_state, uint32_t* dev_state,
                                 uint32_t* adserr) {
    (void)s;
    Conn* c = find(h);
    if (!c || !ads_state || !dev_state || !adserr)
        return TPLC_ADS_E_PARAM;
    if (c->cstate == 2 && c->cmd != TPLC_ADS_CMD_READ_STATE)
        return TPLC_ADS_E_STATE;
    std::vector<uint8_t> resp;
    const uint8_t* d = nullptr;
    uint32_t dl = 0;
    int32_t rc = opResult(*c, {}, TPLC_ADS_CMD_READ_STATE, adserr,
                          &d, &dl, resp);
    if (rc != TPLC_ADS_OK) return rc;
    if (*adserr == 0 && dl >= 4) {
        *ads_state = get16(d);
        *dev_state = get16(d + 2);
    } else {
        *ads_state = 0;
        *dev_state = 0;
    }
    return TPLC_ADS_OK;
}

int32_t AdsService::adsWriteControl(void* s, TplcAdsHandle h,
                                    uint32_t ads_state, uint32_t dev_state,
                                    uint32_t* adserr) {
    (void)s;
    Conn* c = find(h);
    if (!c || !adserr) return TPLC_ADS_E_PARAM;
    if (c->cstate == 2 && c->cmd != TPLC_ADS_CMD_WRITE_CTRL)
        return TPLC_ADS_E_STATE;
    std::vector<uint8_t> p;
    put16(p, (uint16_t)ads_state);
    put16(p, (uint16_t)dev_state);
    put32(p, 0); /* additional data length */
    std::vector<uint8_t> resp;
    return opResult(*c, p, TPLC_ADS_CMD_WRITE_CTRL, adserr,
                    nullptr, nullptr, resp);
}

int32_t AdsService::adsClose(void* s, TplcAdsHandle h) {
    (void)s;
    drop(h);
    return TPLC_ADS_OK;
}

} /* namespace tplc */
