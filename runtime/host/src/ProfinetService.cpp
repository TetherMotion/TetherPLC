/* ProfinetService.cpp — tplc-profinet: PROFINET DCP discovery/
 * configuration + cyclic IO record exchange over per-port UDP
 * transports.
 *
 * DCP wire format (PROFInet spec / scapy pnio_dcp):
 *   header (12B): FrameID u16 BE | ServiceID u8 | ServiceType u8 |
 *                 XID u32 BE | ResponseDelay u16 | DCPDataLength u16
 *   block:        Option u8 | Suboption u8 | DcpBlockLength u16 BE |
 *                 BlockInfo/Status u16 BE | data[blocklen-2] | pad
 *   FrameID: 0xFEFD get/set, 0xFEFE identify request, 0xFEFF identify
 *   response.  ServiceID: 3=Get 4=Set 5=Identify.  ServiceType:
 *   0=request, 1=response success, 5=not supported.
 *
 * Transport: one UDP socket per port from TPLC_PROFINET_IO
 * (comma-separated host:port or udp://host:port). Real PROFINET
 * carries DCP in Ethertype-0x8892 Ethernet frames — a Layer-2
 * transport is the hardware path; UDP is the tunnel/test transport.
 * Synchronous per call.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/ProfinetService.hpp>
#include <tplc/host/PnioCm.hpp>
#include <tplc/host/Dcerpc.hpp>

#include <algorithm>
#include <arpa/inet.h>
#include <array>
#include <cerrno>
#include <cstring>
#include <cstdlib>
#include <fcntl.h>
#include <netdb.h>
#include <poll.h>
#include <string>
#include <sys/socket.h>
#include <unistd.h>

namespace tplc {

struct ProfinetService::Port {
    int fd = -1;
    std::string endpoint;
    uint32_t pst = TPLC_PN_ST_CLOSED;
    uint32_t errors = 0;
    uint32_t frames = 0;
    /* cached identification */
    char name[32] = {};
    uint32_t vendor = 0, device = 0, role = 0, instance = 0, oem = 0;
    bool haveIdent = false;
    /* IOCR cyclic state */
    uint16_t frameId = 0x8000;
    uint16_t cycle = 0;
    /* PNIO-CM AR session (UDP/34964, connectionless DCE/RPC) */
    std::unique_ptr<DcRpcLink> cmLink;
    host::PnioArClient ar;
    bool arLinked = false;
};

namespace {

constexpr uint8_t DCP_SVC_GET = 0x03;
constexpr uint8_t DCP_SVC_SET = 0x04;
constexpr uint8_t DCP_SVC_IDENT = 0x05;
constexpr uint8_t DCP_TYPE_REQ = 0x00;
constexpr uint8_t DCP_TYPE_OK = 0x01;
constexpr uint16_t FID_GETSET = 0xFEFD;
constexpr uint16_t FID_IDENT_REQ = 0xFEFE;
constexpr uint16_t FID_IDENT_RESP = 0xFEFF;
constexpr int IO_TIMEOUT_MS = 200;

struct PnDcpBlock {
    uint8_t option;
    uint8_t sub;
    const uint8_t* data;   /* payload after the 2-byte info/status */
    uint32_t len;          /* payload length (blocklen - 2) */
    uint16_t info;         /* block info/status word */
};

uint16_t rd16(const uint8_t* p) { return uint16_t((p[0] << 8) | p[1]); }
void wr16(uint8_t* p, uint16_t v) { p[0] = uint8_t(v >> 8); p[1] = uint8_t(v); }
void wr32(uint8_t* p, uint32_t v) {
    p[0] = uint8_t(v >> 24); p[1] = uint8_t(v >> 16);
    p[2] = uint8_t(v >> 8); p[3] = uint8_t(v);
}

/* build a DCP request: header + blocks already encoded */
size_t dcpHeader(uint8_t* out, uint16_t fid, uint8_t svc,
                 uint8_t type, uint32_t xid, uint16_t dataLen) {
    wr16(out, fid);
    out[2] = svc;
    out[3] = type;
    wr32(out + 4, xid);
    wr16(out + 8, 0);            /* response delay */
    wr16(out + 10, dataLen);
    return 12;
}

/* encode one request block: option,sub,len=2+dlen,info=0,data */
size_t encBlock(uint8_t* out, uint8_t opt, uint8_t sub,
                const uint8_t* data, uint32_t dlen) {
    out[0] = opt;
    out[1] = sub;
    wr16(out + 2, uint16_t(2 + dlen));
    wr16(out + 4, 0);            /* request info = 0 */
    if (dlen) memcpy(out + 6, data, dlen);
    return 6 + dlen;
}

/* encode a bare selector block (get/identify): option,sub,len=0 */
size_t encSelector(uint8_t* out, uint8_t opt, uint8_t sub) {
    out[0] = opt;
    out[1] = sub;
    wr16(out + 2, 0);
    return 4;
}

/* walk response blocks; cb returns true to stop */
template <typename F>
bool walkBlocks(const uint8_t* p, size_t n, F&& cb) {
    size_t off = 0;
    while (off + 4 <= n) {
        PnDcpBlock b;
        b.option = p[off];
        b.sub = p[off + 1];
        const uint16_t blen = rd16(p + off + 2);
        if (blen < 2 || off + 4 + blen > n) return false;
        b.info = rd16(p + off + 4);
        b.data = p + off + 6;
        b.len = blen - 2;
        if (cb(b)) return true;
        off += 4 + blen;
        if (blen & 1) ++off;      /* blocks pad to even */
    }
    return true;
}

int udpConnect(const std::string& ep) {
    std::string s = ep;
    for (const char* pre : {"udp://", "tcp://"}) {
        const std::string p = pre;
        if (s.rfind(p, 0) == 0) { s = s.substr(p.size()); break; }
    }
    const size_t colon = s.rfind(':');
    if (colon == std::string::npos) return -1;
    const std::string host = s.substr(0, colon);
    const int port = std::stoi(s.substr(colon + 1));
    addrinfo hints{}, *res = nullptr;
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_DGRAM;
    if (getaddrinfo(host.c_str(), std::to_string(port).c_str(), &hints,
                    &res) != 0 || !res)
        return -1;
    int fd = socket(res->ai_family, res->ai_socktype, res->ai_protocol);
    if (fd >= 0) {
        if (connect(fd, res->ai_addr, res->ai_addrlen) != 0) {
            close(fd); fd = -1;
        } else {
            struct timeval tv{0, IO_TIMEOUT_MS * 1000};
            setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
            setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv));
        }
    }
    freeaddrinfo(res);
    return fd;
}

ProfinetService* self(void* s) { return static_cast<ProfinetService*>(s); }

} // namespace

ProfinetService::ProfinetService() {
    const char* env = std::getenv("TPLC_PROFINET_IO");
    std::vector<std::string> eps;
    if (env && *env) {
        std::string all(env), cur;
        for (char c : all) {
            if (c == ',') { if (!cur.empty()) eps.push_back(cur); cur.clear(); }
            else cur += c;
        }
        if (!cur.empty()) eps.push_back(cur);
    }
    if (eps.empty()) {
        const char* fb = std::getenv("TPLC_IOLINK_IO");
        if (!fb || !*fb) fb = std::getenv("TPLC_CANOPEN_IO");
        if (!fb || !*fb) fb = std::getenv("TPLC_EC_IO");
        if (fb && *fb) eps.push_back(fb);
    }
    for (size_t i = 0; i < eps.size() && i < TPLC_PN_MAX_PORTS; ++i) {
        auto p = std::make_unique<Port>();
        p->endpoint = eps[i];
        ports_.push_back(std::move(p));
    }
    active_ = !ports_.empty();

    svc_.struct_size = sizeof(TplcProfinetService);
    svc_.abi_version = TPLC_PN_ABI_VERSION;
    svc_.host = this;
    svc_.pn_state = [](void* s) { return self(s)->state(s); };
    svc_.pn_open = [](void* s, uint32_t p) { return self(s)->open(s, p); };
    svc_.pn_close = [](void* s, uint32_t p) { return self(s)->close(s, p); };
    svc_.pn_identify = [](void* s, uint32_t p, uint32_t x, char* n,
                          uint32_t nc, uint32_t* v, uint32_t* d,
                          uint32_t* r, uint32_t* i, uint32_t* o,
                          uint8_t* rw, uint32_t rc, uint32_t* rl) {
        return self(s)->identify(s, p, x, n, nc, v, d, r, i, o,
                                 rw, rc, rl);
    };
    svc_.pn_dcp_get = [](void* s, uint32_t p, uint32_t x, uint8_t o,
                         uint8_t sb, uint8_t* d, uint32_t c,
                         uint32_t* l) {
        return self(s)->dcpGet(s, p, x, o, sb, d, c, l);
    };
    svc_.pn_dcp_set = [](void* s, uint32_t p, uint32_t x, uint8_t o,
                         uint8_t sb, const uint8_t* d, uint32_t l) {
        return self(s)->dcpSet(s, p, x, o, sb, d, l);
    };
    svc_.pn_io_exchange = [](void* s, uint32_t p, const uint8_t* orr,
                             uint32_t ol, uint8_t* ir, uint32_t ic,
                             uint32_t* il) {
        return self(s)->ioExchange(s, p, orr, ol, ir, ic, il);
    };
    svc_.pn_io_cr = [](void* s, uint32_t p, uint32_t fid,
                       const uint8_t* orr, uint32_t ol, uint8_t* ir,
                       uint32_t ic, uint32_t* il, uint32_t* co,
                       uint32_t* so) {
        return self(s)->ioCr(s, p, fid, orr, ol, ir, ic, il, co, so);
    };
    svc_.pn_port_info = [](void* s, uint32_t p, uint32_t* st,
                           uint32_t* e, uint32_t* f) {
        return self(s)->portInfo(s, p, st, e, f);
    };
    svc_.pn_ar_connect = [](void* s, uint32_t p, const uint8_t* d,
                            const uint8_t* a, const uint8_t* ar,
                            const uint8_t* b, uint32_t bl,
                            uint8_t* r, uint32_t rc, uint32_t* rl,
                            uint32_t* ps) {
        return self(s)->arConnect(s, p, d, a, ar, b, bl, r, rc, rl, ps);
    };
    svc_.pn_ar_write = [](void* s, uint32_t p, const uint8_t* b,
                          uint32_t bl, uint8_t* r, uint32_t rc,
                          uint32_t* rl, uint32_t* ps) {
        return self(s)->arExchange(s, p, 3, b, bl, r, rc, rl, ps);
    };
    svc_.pn_ar_control = [](void* s, uint32_t p, const uint8_t* b,
                            uint32_t bl, uint8_t* r, uint32_t rc,
                            uint32_t* rl, uint32_t* ps) {
        return self(s)->arExchange(s, p, 4, b, bl, r, rc, rl, ps);
    };
    svc_.pn_ar_read = [](void* s, uint32_t p, const uint8_t* b,
                         uint32_t bl, uint8_t* r, uint32_t rc,
                         uint32_t* rl, uint32_t* ps) {
        return self(s)->arExchange(s, p, 2, b, bl, r, rc, rl, ps);
    };
    svc_.pn_ar_service = [](void* s, uint32_t p, uint32_t t,
                            uint32_t* st) {
        return self(s)->arService(s, p, t, st);
    };
    svc_.pn_ar_state = [](void* s, uint32_t p, uint32_t* st,
                          uint16_t* sk, uint8_t* au) {
        return self(s)->arState(s, p, st, sk, au);
    };
}

ProfinetService::~ProfinetService() = default;

ProfinetService::Port* ProfinetService::portAt(uint32_t i) {
    if (i >= ports_.size()) return nullptr;
    return ports_[i].get();
}

int32_t ProfinetService::state(void*) { return active_ ? 1 : 0; }

int32_t ProfinetService::open(void*, uint32_t port) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (p->fd >= 0) return TPLC_PN_OK;
    p->fd = udpConnect(p->endpoint);
    if (p->fd < 0) { p->pst = TPLC_PN_ST_CLOSED; ++p->errors; return TPLC_PN_E_IO; }
    p->pst = TPLC_PN_ST_OPEN;
    p->haveIdent = false;
    return TPLC_PN_OK;
}

int32_t ProfinetService::close(void*, uint32_t port) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (p->fd >= 0) { ::close(p->fd); p->fd = -1; }
    p->pst = TPLC_PN_ST_CLOSED;
    return TPLC_PN_OK;
}

/* send one DCP request, receive + validate one response.
   Returns TPLC_PN_OK and fills resp/respLen (header+blocks). */
int32_t ProfinetService::exchangeDcp(Port* p, const uint8_t* req,
                                     size_t reqLen, uint8_t* resp,
                                     size_t respCap, uint32_t* respLen) {
    if (p->fd < 0) return TPLC_PN_E_NOEP;
    ssize_t w = send(p->fd, req, reqLen, 0);
    if (w != (ssize_t)reqLen) { ++p->errors; return TPLC_PN_E_IO; }
    uint8_t buf[512];
    ssize_t r = recv(p->fd, buf, sizeof buf, 0);
    if (r < 12) { ++p->errors; return TPLC_PN_E_IO; }
    /* validate: FrameID response-ish, ServiceType != request,
       XID echo */
    const uint16_t fid = rd16(buf);
    const uint8_t svc = buf[2];
    const uint8_t type = buf[3];
    const uint32_t xid = (uint32_t(buf[4]) << 24) | (uint32_t(buf[5]) << 16) |
                         (uint32_t(buf[6]) << 8) | uint32_t(buf[7]);
    const uint32_t reqXid = (uint32_t(req[4]) << 24) | (uint32_t(req[5]) << 16) |
                            (uint32_t(req[6]) << 8) | uint32_t(req[7]);
    (void)svc;
    if (xid != reqXid || type == DCP_TYPE_REQ ||
        (fid != FID_IDENT_RESP && fid != FID_GETSET &&
         fid != FID_IDENT_REQ)) {
        ++p->errors;
        return TPLC_PN_E_PROTO;
    }
    const uint16_t dlen = rd16(buf + 10);
    const size_t total = 12 + dlen;
    if (total > (size_t)r || total > respCap) {
        ++p->errors;
        return TPLC_PN_E_PROTO;
    }
    memcpy(resp, buf, total);
    if (respLen) *respLen = uint32_t(total);
    ++p->frames;
    return TPLC_PN_OK;
}

int32_t ProfinetService::identify(void*, uint32_t port, uint32_t xid,
                                  char* name, uint32_t name_cap,
                                  uint32_t* vendor, uint32_t* device,
                                  uint32_t* role, uint32_t* instance,
                                  uint32_t* oem,
                                  uint8_t* raw, uint32_t raw_cap,
                                  uint32_t* raw_len) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (p->fd < 0) return TPLC_PN_E_STATE;
    /* Identify request: FrameID FEFE, svc 5, all-selector block */
    uint8_t req[64];
    size_t n = dcpHeader(req, FID_IDENT_REQ, DCP_SVC_IDENT,
                         DCP_TYPE_REQ, xid, 4);
    n += encSelector(req + n, TPLC_PN_DCP_OPT_ALL, 0xFF);
    /* patch data length now that we know block size */
    wr16(req + 10, uint16_t(n - 12));

    uint8_t resp[512];
    uint32_t rlen = 0;
    int32_t rc = exchangeDcp(p, req, n, resp, sizeof resp, &rlen);
    if (rc != TPLC_PN_OK) return rc;

    /* service type must be response-success */
    if (resp[3] != DCP_TYPE_OK) { ++p->errors; return TPLC_PN_E_PROTO; }

    /* stash raw blocks payload (after 12-byte header) */
    const uint32_t payLen = rlen > 12 ? rlen - 12 : 0;
    if (raw_len) *raw_len = payLen < raw_cap ? payLen : raw_cap;
    if (raw && raw_cap) memcpy(raw, resp + 12, *raw_len);

    /* walk blocks for known properties */
    walkBlocks(resp + 12, payLen, [&](const PnDcpBlock& b) {
        if (b.option == TPLC_PN_DCP_OPT_PROP) {
            switch (b.sub) {
            case TPLC_PN_DCP_SUB_NAME:
                if (name && name_cap) {
                    uint32_t l = b.len < name_cap - 1 ? b.len : name_cap - 1;
                    memcpy(name, b.data, l);
                    name[l] = 0;
                }
                break;
            case TPLC_PN_DCP_SUB_DEVID:
                if (b.len >= 4) {
                    if (vendor) *vendor = rd16(b.data);
                    if (device) *device = rd16(b.data + 2);
                }
                break;
            case TPLC_PN_DCP_SUB_ROLE:
                if (b.len >= 1 && role) *role = b.data[0];
                break;
            case TPLC_PN_DCP_SUB_INSTANCE:
                if (b.len >= 2 && instance) *instance = rd16(b.data);
                else if (b.len >= 4 && instance)
                    *instance = (uint32_t(b.data[0]) << 24) |
                                (uint32_t(b.data[1]) << 16) |
                                (uint32_t(b.data[2]) << 8) | b.data[3];
                break;
            case TPLC_PN_DCP_SUB_OEM:
                if (b.len >= 4 && oem)
                    *oem = (uint32_t(b.data[0]) << 24) |
                           (uint32_t(b.data[1]) << 16) |
                           (uint32_t(b.data[2]) << 8) | b.data[3];
                break;
            default: break;
            }
        }
        return false;
    });

    /* cache into port record */
    if (name && name_cap) {
        strncpy(p->name, name, sizeof p->name - 1);
        p->name[sizeof p->name - 1] = 0;
    }
    if (vendor) p->vendor = *vendor;
    if (device) p->device = *device;
    if (role) p->role = *role;
    if (instance) p->instance = *instance;
    if (oem) p->oem = *oem;
    p->haveIdent = true;
    p->pst = TPLC_PN_IDENTIFIED;
    return TPLC_PN_OK;
}

int32_t ProfinetService::dcpGet(void*, uint32_t port, uint32_t xid,
                                uint8_t option, uint8_t suboption,
                                uint8_t* data, uint32_t cap,
                                uint32_t* out_len) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (p->fd < 0) return TPLC_PN_E_STATE;
    uint8_t req[64];
    size_t n = dcpHeader(req, FID_GETSET, DCP_SVC_GET, DCP_TYPE_REQ,
                         xid, 4);
    n += encSelector(req + n, option, suboption);
    wr16(req + 10, uint16_t(n - 12));

    uint8_t resp[512];
    uint32_t rlen = 0;
    int32_t rc = exchangeDcp(p, req, n, resp, sizeof resp, &rlen);
    if (rc != TPLC_PN_OK) return rc;
    if (resp[3] != DCP_TYPE_OK) { ++p->errors; return TPLC_PN_E_PROTO; }

    /* find matching block, return its payload (after info word) */
    const uint16_t payLen = rd16(resp + 10);
    bool found = false;
    walkBlocks(resp + 12, payLen, [&](const PnDcpBlock& b) {
        if (b.option == option && b.sub == suboption) {
            found = true;
            if (b.info != 0 && b.info != 1) return true; /* error */
            uint32_t l = b.len < cap ? b.len : cap;
            if (data && l) memcpy(data, b.data, l);
            if (out_len) *out_len = l;
            return true;
        }
        return false;
    });
    if (!found) { ++p->errors; return TPLC_PN_E_PROTO; }
    return TPLC_PN_OK;
}

int32_t ProfinetService::dcpSet(void*, uint32_t port, uint32_t xid,
                                uint8_t option, uint8_t suboption,
                                const uint8_t* data, uint32_t len) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (p->fd < 0) return TPLC_PN_E_STATE;
    if (!data && len) return TPLC_PN_E_PARAM;
    uint8_t req[300];
    size_t n = dcpHeader(req, FID_GETSET, DCP_SVC_SET, DCP_TYPE_REQ,
                         xid, 0);
    const size_t blk = encBlock(req + n, option, suboption, data, len);
    n += blk;
    wr16(req + 10, uint16_t(n - 12));

    uint8_t resp[512];
    uint32_t rlen = 0;
    int32_t rc = exchangeDcp(p, req, n, resp, sizeof resp, &rlen);
    if (rc != TPLC_PN_OK) return rc;
    if (resp[3] != DCP_TYPE_OK) { ++p->errors; return TPLC_PN_E_PROTO; }

    /* response block info: 1=success, 2..6=block errors */
    const uint16_t payLen = rd16(resp + 10);
    int32_t result = TPLC_PN_OK;
    walkBlocks(resp + 12, payLen, [&](const PnDcpBlock& b) {
        if (b.option == option && b.sub == suboption) {
            if (b.info != 1) result = TPLC_PN_E_DEV;
            return true;
        }
        return false;
    });
    if (result != TPLC_PN_OK) ++p->errors;
    return result;
}

int32_t ProfinetService::ioExchange(void*, uint32_t port,
                                    const uint8_t* out_rec,
                                    uint32_t out_len,
                                    uint8_t* in_rec, uint32_t in_cap,
                                    uint32_t* in_len) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (p->fd < 0 || p->pst < TPLC_PN_ST_OPEN)
        return TPLC_PN_E_STATE;
    if (out_len > TPLC_PN_MAX_IO || in_cap > TPLC_PN_MAX_IO + 16)
        return TPLC_PN_E_PARAM;
    /* cyclic record primitive: 2-byte record tag + payload.  AR/IOCR
       cyclic framing (Ethertype 0x8892 RT frames) is staged — this
       exchanges provider/consumer records over the port transport. */
    uint8_t tx[80];
    tx[0] = 0x50; tx[1] = 0x4E;            /* 'PN' record tag */
    tx[2] = uint8_t(out_len & 0xFF);
    if (out_len) memcpy(tx + 3, out_rec, out_len);
    if (send(p->fd, tx, 3 + out_len, 0) != ssize_t(3 + out_len)) {
        ++p->errors; return TPLC_PN_E_IO;
    }
    uint8_t buf[80];
    ssize_t r = recv(p->fd, buf, sizeof buf, 0);
    if (r < 3 || buf[0] != 0x50 || buf[1] != 0x4E) {
        ++p->errors; return TPLC_PN_E_IO;
    }
    const uint32_t ilen = uint8_t(buf[2]);
    if (ilen > in_cap) { ++p->errors; return TPLC_PN_E_PROTO; }
    if (in_rec && ilen) memcpy(in_rec, buf + 3, ilen);
    if (in_len) *in_len = ilen;
    ++p->frames;
    return TPLC_PN_OK;
}

/* IOCR cyclic exchange: IEC 61158-6-10 APDU over the port transport.
   DataRequest/DataResponse PDU =
     [FrameID u16 BE][C-SDU >=40B][cycle u16 BE][dataStatus u8]
     [transferStatus u8]
   The C-SDU carries one provider/consumer record pair in the 'PN'
   tag format used by io_exchange: request C-SDU = 'PN'+len+out_rec,
   reply C-SDU = 'PN'+len+in_rec.  Full multi-object C-SDU layout
   (per-submodule IOPS/IOCS) is staged with AR negotiation. */
int32_t ProfinetService::ioCr(void*, uint32_t port, uint32_t frame_id,
                              const uint8_t* out_rec, uint32_t out_len,
                              uint8_t* in_rec, uint32_t in_cap,
                              uint32_t* in_len, uint32_t* cyc_out,
                              uint32_t* stat_out) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (p->fd < 0 || p->pst < TPLC_PN_ST_OPEN)
        return TPLC_PN_E_STATE;
    if (out_len > TPLC_PN_MAX_IO || in_cap > TPLC_PN_MAX_IO)
        return TPLC_PN_E_PARAM;
    if (frame_id) p->frameId = uint16_t(frame_id & 0xFFFF);

    /* C-SDU: 'PN' record + zero pad to the 40-octet minimum */
    uint8_t csdu[48];
    memset(csdu, 0, sizeof csdu);
    csdu[0] = 0x50; csdu[1] = 0x4E;
    csdu[2] = uint8_t(out_len & 0xFF);
    if (out_len) memcpy(csdu + 3, out_rec, out_len);
    const uint32_t csduLen = (3 + out_len + 39) / 40 * 40;
    if (csduLen > sizeof csdu) return TPLC_PN_E_PARAM;

    /* PDU: FrameID + C-SDU + APDU trailer (cycle, dataStatus,
       transferStatus); request status = RUN/primary/valid/OK */
    uint8_t tx[80];
    wr16(tx, p->frameId);
    memcpy(tx + 2, csdu, csduLen);
    const size_t apdu = 2 + csduLen;
    wr16(tx + apdu, p->cycle);
    tx[apdu + 2] = 0x35;   /* DataStatus: primary|RUN|valid|OK */
    tx[apdu + 3] = 0x00;   /* TransferStatus: usable */
    if (send(p->fd, tx, apdu + 4, 0) != ssize_t(apdu + 4)) {
        ++p->errors; return TPLC_PN_E_IO;
    }
    p->cycle = uint16_t(p->cycle + 1);

    uint8_t buf[128];
    ssize_t r = recv(p->fd, buf, sizeof buf, 0);
    if (r < 2 + 40 + 4 || buf[0] != uint8_t(p->frameId >> 8) ||
        buf[1] != uint8_t(p->frameId & 0xFF)) {
        ++p->errors; return TPLC_PN_E_IO;
    }
    /* trailer sits at the tail: cycle(2) + dataStatus + transfer */
    const uint8_t* tr = buf + r - 4;
    if (cyc_out) *cyc_out = rd16(tr);
    if (stat_out) *stat_out = tr[2];
    if (tr[3] != 0) { ++p->errors; return TPLC_PN_E_DEV; }
    /* C-SDU record at offset 2, minimum 40 octets */
    const size_t csduOff = 2;
    const size_t csduAvail = size_t(r) - csduOff - 4;
    if (csduAvail < 40 || buf[csduOff] != 0x50 ||
        buf[csduOff + 1] != 0x4E) {
        ++p->errors; return TPLC_PN_E_PROTO;
    }
    const uint32_t ilen = buf[csduOff + 2];
    if (ilen > in_cap || csduOff + 3 + ilen > size_t(r) - 4) {
        ++p->errors; return TPLC_PN_E_PROTO;
    }
    if (in_rec && ilen) memcpy(in_rec, buf + csduOff + 3, ilen);
    if (in_len) *in_len = ilen;
    ++p->frames;
    return TPLC_PN_OK;
}

int32_t ProfinetService::portInfo(void*, uint32_t port, uint32_t* st,
                                  uint32_t* errors, uint32_t* frames) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (st) *st = p->pst;
    if (errors) *errors = p->errors;
    if (frames) *frames = p->frames;
    return TPLC_PN_OK;
}

/* ---- PNIO-CM AR session ----
 * CM rides connectionless DCE/RPC on UDP/34964 to the same device host
 * as the port endpoint. The AR client owns a dedicated DcRpcLink. */

int32_t ProfinetService::arEnsure(Port* p) {
    if (p->arLinked) return p->ar.isOpen() ? TPLC_PN_OK : TPLC_PN_E_IO;
    /* CM host = endpoint host, port 34964 */
    std::string ep = p->endpoint;
    for (const char* pre : {"udp://", "tcp://"}) {
        const std::string pr = pre;
        if (ep.rfind(pr, 0) == 0) { ep = ep.substr(pr.size()); break; }
    }
    const size_t colon = ep.rfind(':');
    if (colon == std::string::npos) return TPLC_PN_E_PARAM;
    const std::string host = ep.substr(0, colon);
    p->cmLink = std::make_unique<DcRpcLink>();
    if (!p->ar.open(*p->cmLink, host, host::kPnioUdpPort)) {
        p->cmLink.reset();
        ++p->errors;
        return TPLC_PN_E_IO;
    }
    p->arLinked = true;
    return TPLC_PN_OK;
}

int32_t ProfinetService::arConnect(void*, uint32_t port,
                                   const uint8_t dev[16], const uint8_t act[16],
                                   const uint8_t aru[16],
                                   const uint8_t* blocks, uint32_t blen,
                                   uint8_t* res, uint32_t resCap,
                                   uint32_t* resLen, uint32_t* pnioStatus) {
    Port* p = portAt(port);
    if (!p || !blocks || !blen) return TPLC_PN_E_PARAM;
    const int32_t rc = arEnsure(p);
    if (rc != TPLC_PN_OK) return rc;
    std::array<uint8_t,16> d{}, a{}, ar{};
    memcpy(d.data(), dev, 16); memcpy(a.data(), act, 16);
    memcpy(ar.data(), aru, 16);
    if (!p->ar.connectRaw(d, a, ar, blocks, blen, 3000)) {
        ++p->errors;
        if (pnioStatus) *pnioStatus = p->ar.lastStatus();
        return TPLC_PN_E_DEV;
    }
    if (pnioStatus) *pnioStatus = 0;
    const auto& rb = p->ar.lastResBlocks();
    if (resLen) *resLen = uint32_t(std::min<size_t>(rb.size(), resCap));
    if (res && resCap) memcpy(res, rb.data(), std::min<size_t>(rb.size(), resCap));
    ++p->frames;
    return TPLC_PN_OK;
}

int32_t ProfinetService::arExchange(void*, uint32_t port, uint16_t opnum,
                                    const uint8_t* blocks, uint32_t blen,
                                    uint8_t* res, uint32_t resCap,
                                    uint32_t* resLen, uint32_t* pnioStatus) {
    Port* p = portAt(port);
    if (!p || !blocks || !blen) return TPLC_PN_E_PARAM;
    if (!p->arLinked || !p->ar.isOpen()) return TPLC_PN_E_STATE;
    std::vector<uint8_t> req(blocks, blocks + blen), rb;
    uint32_t status = 0;
    const auto op = host::PnioOpnum(opnum);
    if (!p->ar.rpcCall(op, req, rb, status, 3000)) {
        ++p->errors;
        if (pnioStatus) *pnioStatus = status;
        return TPLC_PN_E_IO;
    }
    if (pnioStatus) *pnioStatus = status;
    if (resLen) *resLen = uint32_t(std::min<size_t>(rb.size(), resCap));
    if (res && resCap) memcpy(res, rb.data(), std::min<size_t>(rb.size(), resCap));
    ++p->frames;
    return status == 0 ? TPLC_PN_OK : TPLC_PN_E_DEV;
}

int32_t ProfinetService::arService(void*, uint32_t port, uint32_t timeoutMs,
                                   uint32_t* arState) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (arState) {
        *arState = uint32_t(p->ar.state());
        if (!p->arLinked || !p->ar.isOpen()) *arState = 0;
    }
    if (!p->arLinked || !p->ar.isOpen()) return TPLC_PN_E_STATE;
    /* service() consumes one datagram; AppReady → DataExchange */
    if (p->ar.service(int(timeoutMs))) ++p->frames;
    if (arState) *arState = uint32_t(p->ar.state());
    return TPLC_PN_OK;
}

int32_t ProfinetService::arState(void*, uint32_t port, uint32_t* arState,
                                 uint16_t* sessionKey, uint8_t arUuid[16]) {
    Port* p = portAt(port);
    if (!p) return TPLC_PN_E_PARAM;
    if (arState) {
        *arState = p->arLinked ? uint32_t(p->ar.state()) : 0;
    }
    if (sessionKey) *sessionKey = p->ar.connectRes().sessionKey;
    if (arUuid) {
        const auto& u = p->ar.connectRes().arUuid;
        memcpy(arUuid, u.data(), 16);
    }
    return TPLC_PN_OK;
}

} // namespace tplc
