/* BacnetService.cpp — BACnet/IP (ASHRAE 135) client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/BacnetService.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <netinet/in.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

namespace tplc {

namespace {
constexpr uint8_t BVLL_TYPE = 0x81;
constexpr uint8_t BVLL_UCAST = 0x0A;
constexpr uint8_t BVLL_BCAST = 0x0B;
constexpr uint8_t NPDU_VER = 1;
constexpr uint8_t NPDU_CTRL_REPLY = 0x04; /* expecting reply, prio 0 */
constexpr uint8_t APDU_CONF_REQ = 0x00;
constexpr uint8_t APDU_UNCONF_REQ = 0x10; /* type 1 << 4 */
constexpr uint8_t SVC_I_AM = 0;
constexpr uint8_t SVC_WHO_IS = 8;
constexpr uint8_t SVC_READ_PROP = 12;
constexpr uint8_t SVC_WRITE_PROP = 15;
constexpr uint32_t OBJ_DEV_MAX = 4194303u; /* 0x3FFFFF */
constexpr uint64_t NS_PER_MS = 1000000ull;
constexpr size_t RX_CAP = 4096;

/* app tag numbers (payload type selectors) */
constexpr uint32_t TAG_NULL = 0, TAG_BOOL = 1, TAG_UNSIGNED = 2,
                   TAG_INTEGER = 3, TAG_REAL = 4, TAG_DOUBLE = 5,
                   TAG_OCTET = 6, TAG_CHAR = 7, TAG_ENUM = 9,
                   TAG_OBJID = 12;

uint16_t rd16(const uint8_t* p) { return uint16_t(p[0]) << 8 | p[1]; }
uint32_t rd32(const uint8_t* p) {
    return (uint32_t(p[0]) << 24) | (uint32_t(p[1]) << 16) |
           (uint32_t(p[2]) << 8) | uint32_t(p[3]);
}

struct Cur {
    const uint8_t* p;
    size_t n, i;
    bool tag(bool* ctx, uint32_t* num, const uint8_t** d, uint32_t* len) {
        if (i >= n) return false;
        uint8_t o = p[i++];
        *ctx = (o & 0x08) != 0;
        *num = o >> 4;
        if (*num == 0x0F) {
            if (i >= n) return false;
            *num = p[i++];
        }
        uint32_t l = o & 0x07;
        if (l == 6 || l == 7) { /* opening/closing: no payload */
            *d = p + i;
            *len = 0;
            return true;
        }
        if (l == 5) {
            if (i >= n) return false;
            l = p[i++];
            if (l == 254) {
                if (i + 2 > n) return false;
                l = rd16(p + i);
                i += 2;
            } else if (l == 255) {
                if (i + 4 > n) return false;
                l = rd32(p + i);
                i += 4;
            }
        }
        if (i + l > n) return false;
        *d = p + i;
        *len = l;
        i += l;
        return true;
    }
    bool done() const { return i >= n; }
};

/* encode one tag: ctx selects context (0x08) vs application (0x00)
   class; num is the tag number; d/len the payload. */
size_t put_tag(uint8_t* b, bool ctx, uint32_t num, const uint8_t* d,
               size_t len) {
    size_t n = 0;
    uint8_t o = (ctx ? 0x08 : 0x00);
    if (num < 15)
        o |= uint8_t(num << 4);
    else
        o |= 0xF0;
    /* encode length field: <5 inline, <=253 one ext byte,
       else 254 + u16 (payloads are bounded to 256) */
    uint8_t hdr[8];
    size_t hn = 0;
    if (len < 5) {
        o |= uint8_t(len);
    } else if (len <= 253) {
        o |= 5;
        hdr[hn++] = uint8_t(len);
    } else {
        o |= 5;
        hdr[hn++] = 254;
        hdr[hn++] = uint8_t(len >> 8);
        hdr[hn++] = uint8_t(len);
    }
    b[n++] = o;
    if (num >= 15) b[n++] = uint8_t(num);
    memcpy(b + n, hdr, hn);
    n += hn;
    memcpy(b + n, d, len);
    n += len;
    return n;
}

size_t put_ctag_u32(uint8_t* b, uint32_t num, uint32_t v) {
    /* unsigned context param: minimal big-endian bytes */
    uint8_t t[4];
    int nb = 0;
    for (int s = 24; s >= 0; s -= 8) {
        uint8_t c = uint8_t(v >> s);
        if (nb || c || s == 0) t[nb++] = c;
    }
    return put_tag(b, true, num, t, nb);
}

uint32_t u32_from_min_be(const uint8_t* d, uint32_t len) {
    uint32_t v = 0;
    for (uint32_t i = 0; i < len && i < 4; i++) v = (v << 8) | d[i];
    return v;
}

uint32_t objid_bytes(uint32_t type, uint32_t inst) {
    return ((type & 0x3FF) << 22) | (inst & OBJ_DEV_MAX);
}

/* dotted-quad -> in_addr; "" and "broadcast"/"255.255.255.255"
   all map to the limited-broadcast address */
static bool parse_v4(const char* host, struct in_addr* out) {
    if (!host || !*host || strcmp(host, "broadcast") == 0 ||
        strcmp(host, "255.255.255.255") == 0) {
        out->s_addr = htonl(INADDR_BROADCAST);
        return true;
    }
    unsigned a, b, c, d;
    char tail;
    if (sscanf(host, "%u.%u.%u.%u%c", &a, &b, &c, &d, &tail) != 4 ||
        a > 255 || b > 255 || c > 255 || d > 255)
        return false;
    uint32_t v = (a << 24) | (b << 16) | (c << 8) | d;
    out->s_addr = htonl(v);
    return true;
}
} /* namespace */

BacnetService* BacnetService::self(void* p) {
    return static_cast<BacnetService*>(p);
}

BacnetService::BacnetService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcBacnetService);
    svc_.abi_version = TPLC_BACNET_ABI_VERSION;
    svc_.host = this;
    svc_.bac_begin = [](void* s, const TplcBacnetReq* r,
                        TplcBacnetHandle* o) {
        return self(s)->begin(s, r, o);
    };
    svc_.bac_poll = [](void* s, TplcBacnetHandle h,
                       TplcBacnetRespInfo* i, uint8_t* v,
                       uint32_t vc, uint32_t* ids, uint8_t* ips,
                       uint32_t dc) {
        return self(s)->poll(s, h, i, v, vc, ids, ips, dc);
    };
    svc_.bac_abort = [](void* s, TplcBacnetHandle h) {
        return self(s)->abort(s, h);
    };
}

/* ---- request encoding ---- */

bool BacnetService::encode(Txn& t) {
    const TplcBacnetReq& r = t.req;
    uint8_t* b = t.pkt;
    size_t cap = sizeof(t.pkt);
    size_t n = 0;

    /* BVLC header (patched length at the end) */
    if (n + 4 > cap) return false;
    b[n++] = BVLL_TYPE;
    struct in_addr da{};
    if (!parse_v4(r.peer.host, &da)) return false;
    const bool bcast = da.s_addr == htonl(INADDR_BROADCAST);
    b[n++] = bcast ? BVLL_BCAST : BVLL_UCAST;
    size_t len_off = n;
    n += 2;

    /* NPDU: version 1, control = expecting-reply for confirmed */
    const bool confirmed = r.func != TPLC_BACNET_F_WHO_IS;
    if (n + 2 > cap) return false;
    b[n++] = NPDU_VER;
    b[n++] = confirmed ? NPDU_CTRL_REPLY : 0x00;

    /* APDU */
    if (r.func == TPLC_BACNET_F_WHO_IS) {
        /* unconfirmed request: type 1, service 8, optional range */
        if (n + 2 > cap) return false;
        b[n++] = APDU_UNCONF_REQ;
        b[n++] = SVC_WHO_IS;
        if (r.flags & TPLC_BACNET_F_HAS_RANGE) {
            n += put_ctag_u32(b + n, 0, r.aux0);
            n += put_ctag_u32(b + n, 1, r.aux1);
        }
    } else {
        uint8_t svc =
            r.func == TPLC_BACNET_F_READ_PROP ? SVC_READ_PROP
                                              : SVC_WRITE_PROP;
        /* confirmed request: type 0, maxSeg=0/maxResp=4 (1476),
           invoke id, service, context-tagged params */
        if (n + 4 > cap) return false;
        b[n++] = APDU_CONF_REQ;
        b[n++] = 0x04; /* unsegmented, 1476-octet responses */
        b[n++] = uint8_t(t.invoke_id & 0xFF);
        b[n++] = svc;
        /* ctx0 objectIdentifier: 4-byte (type<<22)|instance */
        uint32_t oid = objid_bytes(r.obj_type, r.obj_inst);
        uint8_t ob[4] = {uint8_t(oid >> 24), uint8_t(oid >> 16),
                         uint8_t(oid >> 8), uint8_t(oid)};
        n += put_tag(b + n, true, 0, ob, 4);
        /* ctx1 propertyIdentifier: minimal unsigned */
        n += put_ctag_u32(b + n, 1, r.prop_id);
        /* ctx2 propertyArrayIndex (optional) */
        if (r.flags & TPLC_BACNET_F_HAS_IDX)
            n += put_ctag_u32(b + n, 2, r.prop_idx);
        if (r.func == TPLC_BACNET_F_WRITE_PROP) {
            /* ctx3 Any: application-tagged value, payload verbatim */
            if (r.val_type > TAG_OBJID || r.val_type == 8 ||
                r.val_type == 10 || r.val_type == 11 ||
                r.val_type == 13 || r.val_type == 14 ||
                r.val_type == 15)
                return false;
            n += put_tag(b + n, false, r.val_type, r.val, r.val_len);
            /* ctx4 priority (optional integer, minimal BE) */
            if (r.flags & TPLC_BACNET_F_HAS_PRIO)
                n += put_ctag_u32(b + n, 4, r.aux0);
        }
    }

    if (n > cap) return false;
    t.pkt_len = uint32_t(n);
    /* patch BVLC length (includes the 4-byte header) */
    uint16_t L = uint16_t(n);
    t.pkt[len_off] = uint8_t(L >> 8);
    t.pkt[len_off + 1] = uint8_t(L);
    return true;
}

int32_t BacnetService::begin(void*, const TplcBacnetReq* req,
                             TplcBacnetHandle* out) {
    if (!req || !out) return TPLC_BACNET_E_PARAM;
    if (req->func < TPLC_BACNET_F_WHO_IS ||
        req->func > TPLC_BACNET_F_WRITE_PROP)
        return TPLC_BACNET_E_PARAM;
    if (req->func != TPLC_BACNET_F_WHO_IS) {
        if (req->obj_inst > OBJ_DEV_MAX || req->obj_type > 1023)
            return TPLC_BACNET_E_RANGE;
    }
    if (req->func == TPLC_BACNET_F_WRITE_PROP) {
        if (req->val_len > TPLC_BACNET_MAX_VAL ||
            req->val_type > TAG_OBJID)
            return TPLC_BACNET_E_RANGE;
        if (req->val_len == 0 && req->val_type != TAG_NULL)
            return TPLC_BACNET_E_PARAM;
    }
    if (req->func == TPLC_BACNET_F_WHO_IS &&
        !(req->flags & TPLC_BACNET_F_HAS_RANGE)) {
        /* bare Who-Is: no object fields needed */
    }

    int fd = socket(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK, 0);
    if (fd < 0) return TPLC_BACNET_E_IO;
    int one = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));
    setsockopt(fd, SOL_SOCKET, SO_BROADCAST, &one, sizeof(one));
    sockaddr_in la{};
    la.sin_family = AF_INET;
    la.sin_addr.s_addr = htonl(INADDR_ANY);
    la.sin_port = 0;
    if (bind(fd, reinterpret_cast<sockaddr*>(&la), sizeof(la)) < 0) {
        close(fd);
        return TPLC_BACNET_E_IO;
    }

    uint32_t h = next_handle_++;
    if (h == 0) h = next_handle_++;
    Txn t;
    t.fd = fd;
    t.req = *req;
    t.invoke_id = next_invoke_ = (next_invoke_ + 1) & 0xFF;
    if (t.invoke_id == 0) t.invoke_id = next_invoke_++;
    uint32_t to = req->timeout_ms ? req->timeout_ms : 1000;
    if (to > 60000) to = 60000;
    t.deadline_ns = clock_(clock_ctx_) + uint64_t(to) * NS_PER_MS;
    t.info.status = TPLC_BACNET_ST_PENDING;
    if (!encode(t)) {
        close(fd);
        txns_.erase(h);
        return TPLC_BACNET_E_PARAM;
    }
    txns_.emplace(h, std::move(t));
    Txn& tt = txns_[h];
    /* send immediately; nonblocking UDP rarely fails */
    struct in_addr da{};
    if (!parse_v4(req->peer.host, &da)) {
        txns_.erase(h);
        close(fd);
        return TPLC_BACNET_E_PARAM;
    }
    sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_addr = da;
    dst.sin_port = htons(req->peer.port ? req->peer.port
                                        : TPLC_BACNET_PORT);
    ssize_t w = sendto(tt.fd, tt.pkt, tt.pkt_len, 0,
                       reinterpret_cast<sockaddr*>(&dst),
                       sizeof(dst));
    tt.sent = w == ssize_t(tt.pkt_len);
    /* a nonblocking sendto can still hit EAGAIN; poll retries once */
    *out = h;
    return TPLC_BACNET_OK;
}

/* ---- response decoding ---- */

void BacnetService::handleDatagram(Txn& t, const uint8_t* d,
                                   size_t n,
                                   const struct sockaddr_in& from) {
    TplcBacnetRespInfo& r = t.info;
    if (n < 4 || d[0] != BVLL_TYPE) return;
    uint8_t func = d[1];
    if (func != BVLL_UCAST && func != BVLL_BCAST) return;
    uint32_t blen = rd16(d + 2);
    if (blen < 4 || blen > n) return;

    /* NPDU at offset 4 */
    size_t i = 4;
    if (i + 2 > n) return;
    if (d[i] != NPDU_VER) return;
    uint8_t ctrl = d[i + 1];
    i += 2;
    if (ctrl & 0x20) { /* DNET/DLEN/DADR + hop */
        if (i + 2 > n) return;
        uint16_t dnet = rd16(d + i);
        (void)dnet;
        i += 2;
        if (i >= n) return;
        uint8_t dlen = d[i++];
        i += dlen;
        if (i >= n) return;
        i++; /* hop count */
    }
    if (ctrl & 0x08) { /* SNET/SLEN/SADR */
        if (i + 2 > n) return;
        i += 2;
        if (i >= n) return;
        uint8_t slen = d[i++];
        i += slen;
    }
    if (i >= n) return;

    /* APDU */
    uint8_t b0 = d[i++];
    uint8_t atype = b0 >> 4;
    r.apdu_type = atype;
    Cur c{d + i, n - i, 0};

    if (t.req.func == TPLC_BACNET_F_WHO_IS) {
        /* collect I-Am unconfirmed responses until deadline */
        if (atype != 1 || c.i >= c.n) return;
        if (c.p[c.i] != SVC_I_AM) return;
        c.i++;
        /* app-tagged params: objid, maxapdu, seg, vendor */
        uint32_t oid = 0, apdu = 0, seg = 0, vendor = 0;
        bool ctx;
        uint32_t num;
        const uint8_t* pd;
        uint32_t pl;
        int seen = 0;
        while (c.tag(&ctx, &num, &pd, &pl) && !ctx) {
            if (num == TAG_OBJID && pl == 4) {
                oid = rd32(pd);
            } else if (num == TAG_UNSIGNED) {
                uint32_t v = u32_from_min_be(pd, pl);
                if (seen == 1) apdu = v;
                else if (seen == 2) seg = v;
                else if (seen == 3) vendor = v;
            }
            seen++;
        }
        if (oid == 0 && seen == 0) return;
        if (t.dev_total < TPLC_BACNET_MAX_RESPONDERS) {
            uint32_t k = t.dev_total++;
            t.dev_id[k] = oid;
            memcpy(t.dev_ip[k], &from.sin_addr.s_addr, 4);
        }
        r.dev_count = t.dev_total;
        r.dev_vendor = vendor;
        r.dev_max_apdu = apdu;
        r.dev_seg = seg;
        r.service = SVC_I_AM;
        return;
    }

    /* confirmed services: match invoke id */
    /* APDU type is the high nibble of the first octet (0..15) */
    if (atype == 2) { /* SimpleACK */
        if (c.i + 2 > c.n) return;
        uint8_t iid = c.p[c.i++];
        if (iid != t.invoke_id) return;
        r.service = c.p[c.i];
        r.val_type = 0;
        r.val_len = 0;
        finish(t, TPLC_BACNET_ST_OK);
        return;
    }
    if (atype == 3) { /* ComplexACK */
        if (c.i + 2 > c.n) return;
        uint8_t iid = c.p[c.i++];
        if (iid != t.invoke_id) return;
        r.service = c.p[c.i++];
        /* parse context params: 0=oid,1=propid,2=idx?,3=Any */
        bool ctx;
        uint32_t num;
        const uint8_t* pd;
        uint32_t pl;
        bool got_val = false;
        while (c.tag(&ctx, &num, &pd, &pl)) {
            if (!ctx) {
                /* Any value: first application tag is the value */
                if (!got_val) {
                    r.val_type = num;
                    t.val_total = pl;
                    r.val_len = pl;
                    uint32_t cp = pl > TPLC_BACNET_MAX_VAL
                                      ? TPLC_BACNET_MAX_VAL
                                      : pl;
                    memcpy(t.val, pd, cp);
                    got_val = true;
                }
                continue;
            }
            switch (num) {
            case 0:
                if (pl == 4) {
                    uint32_t oid = rd32(pd);
                    r.obj_type = oid >> 22;
                    r.obj_inst = oid & OBJ_DEV_MAX;
                }
                break;
            case 1:
                r.prop_id = u32_from_min_be(pd, pl);
                break;
            case 2:
                break; /* array index echo */
            default:
                break;
            }
        }
        if (got_val || t.req.func == TPLC_BACNET_F_READ_PROP) {
            /* ReadProperty always carries a value; WriteProperty
               acks are Simple-ACK, so a complex ack here is a
               read-style response — accept it */
            finish(t, TPLC_BACNET_ST_OK);
        }
        return;
    }
    if (atype == 5 || atype == 6 || atype == 7) { /* Error/Reject/Abort */
        if (c.i >= c.n) return;
        uint8_t iid = c.p[c.i++];
        if (iid != t.invoke_id) return;
        r.service = c.i < c.n ? c.p[c.i] : 0;
        finish(t, TPLC_BACNET_ST_ERROR);
        return;
    }
    /* other APDU types (segment ack etc.) — we never segment */
    r.status = TPLC_BACNET_ST_BADRESP;
}

void BacnetService::drain(Txn& t) {
    uint8_t buf[RX_CAP];
    for (int k = 0; k < 32 && !t.finished; k++) {
        sockaddr_in from{};
        socklen_t fl = sizeof(from);
        ssize_t n = recvfrom(t.fd, buf, sizeof(buf), MSG_DONTWAIT,
                             reinterpret_cast<sockaddr*>(&from), &fl);
        if (n <= 0) break;
        handleDatagram(t, buf, size_t(n), from);
    }
}

void BacnetService::finish(Txn& t, int32_t status) {
    t.info.status = status;
    t.finished = true;
}

int32_t BacnetService::poll(void*, TplcBacnetHandle h,
                            TplcBacnetRespInfo* info, uint8_t* val,
                            uint32_t val_cap, uint32_t* dev_ids,
                            uint8_t* dev_ips, uint32_t dev_cap) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_BACNET_E_STATE;
    Txn& t = it->second;
    if (!t.finished) {
        /* retry the send once if the first attempt hit EAGAIN */
        if (!t.sent) {
            struct in_addr da{};
            if (parse_v4(t.req.peer.host, &da)) {
                sockaddr_in dst{};
                dst.sin_family = AF_INET;
                dst.sin_addr = da;
                dst.sin_port = htons(t.req.peer.port
                                         ? t.req.peer.port
                                         : TPLC_BACNET_PORT);
                ssize_t w = sendto(t.fd, t.pkt, t.pkt_len,
                                   MSG_DONTWAIT,
                                   reinterpret_cast<sockaddr*>(&dst),
                                   sizeof(dst));
                if (w == ssize_t(t.pkt_len)) t.sent = true;
            }
        }
        drain(t);
        if (!t.finished) {
            uint64_t now = clock_(clock_ctx_);
            if (now >= t.deadline_ns) {
                if (t.req.func == TPLC_BACNET_F_WHO_IS) {
                    /* discovery window elapsed: report what we saw */
                    t.info.status = t.dev_total
                                        ? TPLC_BACNET_ST_OK
                                        : TPLC_BACNET_ST_TIMEOUT;
                } else {
                    t.info.status = TPLC_BACNET_ST_TIMEOUT;
                }
                t.finished = true;
            } else {
                t.info.status = TPLC_BACNET_ST_PENDING;
            }
        }
    }
    if (info) *info = t.info;
    if (val && val_cap) {
        uint32_t cp = t.val_total < val_cap ? t.val_total : val_cap;
        memcpy(val, t.val, cp);
    }
    if (dev_ids && dev_ips && dev_cap) {
        uint32_t cp = t.dev_total < dev_cap ? t.dev_total : dev_cap;
        for (uint32_t i = 0; i < cp; i++) {
            dev_ids[i] = t.dev_id[i];
            memcpy(dev_ips + i * 4, t.dev_ip[i], 4);
        }
    }
    if (t.finished) {
        /* terminal result delivered: release the transaction and
           its socket, matching the modbus poll-drop model */
        if (t.fd >= 0) close(t.fd);
        txns_.erase(it);
    }
    return TPLC_BACNET_OK;
}

int32_t BacnetService::abort(void*, TplcBacnetHandle h) {
    auto it = txns_.find(h);
    if (it == txns_.end()) return TPLC_BACNET_E_STATE;
    if (it->second.fd >= 0) close(it->second.fd);
    txns_.erase(it);
    return TPLC_BACNET_OK;
}

} /* namespace tplc */
