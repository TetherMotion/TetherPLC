/* SacnService.cpp — sACN (ANSI E1.31) source/receiver service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/SacnService.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

namespace tplc {

namespace {
constexpr size_t FLEN_ROOT = 16;  /* root flags+length offset      */
constexpr size_t FLEN_FRAME = 38; /* framing flags+length offset   */
constexpr size_t DMP_OFF = 115;    /* data packet DMP layer         */
constexpr size_t UDL_OFF = 112;    /* discovery universe list       */
constexpr uint32_t VEC_DATA = 0x00000004;
constexpr uint32_t VEC_EXT  = 0x00000008;
constexpr uint32_t VEC_FRAME_DATA = 0x00000002;
constexpr uint32_t VEC_FRAME_SYNC = 0x00000001;
constexpr uint32_t VEC_FRAME_DISC = 0x00000002;
constexpr uint8_t  VEC_DMP_SET = 0x02;   /* single-byte DMP vector  */
constexpr uint32_t VEC_UDL_LIST = 0x00000001;
constexpr size_t MIN_PACKET = 38; /* root layer end                   */
constexpr size_t NAME_OFF = 44;   /* framing source name             */
constexpr uint8_t  OPT_FORCE_SYNC = 0x20;
constexpr uint8_t  OPT_STREAM_TERM = 0x40;
constexpr uint8_t  OPT_PREVIEW = 0x80;
constexpr uint8_t  FLAG_SYNC = 0x01;
constexpr uint8_t  FLAG_DISC = 0x02;
} /* namespace */

static SacnService* self(void* p) { return static_cast<SacnService*>(p); }

SacnService::SacnService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcSacnService);
    svc_.abi_version = TPLC_SACN_ABI_VERSION;
    svc_.sacn_src_begin = [](void* s, const TplcSacnSrcReq* r,
                             TplcSacnHandle* o) {
        return self(s)->srcBegin(s, r, o);
    };
    svc_.sacn_src_poll = [](void* s, TplcSacnHandle h,
                            const TplcSacnFrame* f, TplcSacnSrcInfo* i) {
        return self(s)->srcPoll(s, h, f, i);
    };
    svc_.sacn_src_close = [](void* s, TplcSacnHandle h) {
        return self(s)->srcClose(s, h);
    };
    svc_.sacn_rcv_begin = [](void* s, const TplcSacnRcvReq* r,
                             TplcSacnHandle* o) {
        return self(s)->rcvBegin(s, r, o);
    };
    svc_.sacn_rcv_poll = [](void* s, TplcSacnHandle h,
                            TplcSacnRcvInfo* i, uint8_t* dmx,
                            uint16_t dmx_cap, uint16_t* unis,
                            uint16_t unis_cap, char* name,
                            uint32_t name_cap) {
        return self(s)->rcvPoll(s, h, i, dmx, dmx_cap, unis,
                                unis_cap, name, name_cap);
    };
    svc_.sacn_rcv_close = [](void* s, TplcSacnHandle h) {
        return self(s)->rcvClose(s, h);
    };
}

uint64_t SacnService::nowNs() const {
    if (clock_) return clock_(clock_ctx_);
    struct timespec ts{};
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

void SacnService::put16(std::vector<uint8_t>& v, uint16_t x) {
    v.push_back((x >> 8) & 0xff);
    v.push_back(x & 0xff);
}
void SacnService::put32(std::vector<uint8_t>& v, uint32_t x) {
    v.push_back((x >> 24) & 0xff);
    v.push_back((x >> 16) & 0xff);
    v.push_back((x >> 8) & 0xff);
    v.push_back(x & 0xff);
}
uint16_t SacnService::get16be(const uint8_t* p) {
    return (uint16_t)((p[0] << 8) | p[1]);
}
uint32_t SacnService::get32be(const uint8_t* p) {
    return ((uint32_t)p[0] << 24) | ((uint32_t)p[1] << 16) |
           ((uint32_t)p[2] << 8) | p[3];
}

/* flags+length field: upper nibble 0x7, lower 12 bits = bytes from
   this field's offset to the end of the packet */
void SacnService::flagsLen(std::vector<uint8_t>& v, size_t off,
                           size_t total) {
    size_t len = total - off;
    v.push_back((uint8_t)(0x70 | ((len >> 8) & 0x0f)));
    v.push_back((uint8_t)(len & 0xff));
}

/* 38-byte ACN root layer: 16B preamble, flags+len (total-off),
   vector, 16-byte CID */
void SacnService::rootHdr(std::vector<uint8_t>& v, uint32_t vector,
                          const uint8_t cid[16], size_t total) {
    static const uint8_t pre[16] = {
        0x00, 0x10, 0x00, 0x00,
        'A', 'S', 'C', '-', 'E', '1', '.', '1', '7', 0, 0, 0};
    v.insert(v.end(), pre, pre + 16);
    flagsLen(v, FLEN_ROOT, total);
    put32(v, vector);
    v.insert(v.end(), cid, cid + 16);
}

void SacnService::mcastAddr(uint32_t universe, uint8_t ip[4]) {
    ip[0] = 239;
    ip[1] = 255;
    ip[2] = (uint8_t)(universe >> 8);
    ip[3] = (uint8_t)(universe & 0xff);
}

void SacnService::fillSrcInfo(const Src& t, TplcSacnSrcInfo* info) const {
    memset(info, 0, sizeof(*info));
    info->sent = t.sent;
    info->aux_sent = t.aux_sent;
    info->seq = t.seq;
    info->last_err = t.last_err;
}

/* ------------------------- TX side ------------------------- */

std::vector<uint8_t> SacnService::buildData(const Src& s) {
    size_t total = 126 + s.dmx_len;
    std::vector<uint8_t> v;
    v.reserve(total);
    rootHdr(v, VEC_DATA, s.req.cid, total);
    flagsLen(v, FLEN_FRAME, total);
    put32(v, VEC_FRAME_DATA);
    /* source name: UTF-8 bytes up to 63, NUL-padded to 64 */
    size_t nl = strnlen(s.req.name, TPLC_SACN_NAME_LEN - 1);
    v.insert(v.end(), s.req.name, s.req.name + nl);
    v.resize(v.size() + (TPLC_SACN_NAME_LEN - nl));
    v.push_back((uint8_t)s.req.priority);
    put16(v, (uint16_t)s.req.sync_addr);
    v.push_back((uint8_t)s.seq);
    v.push_back(s.req.options);
    put16(v, (uint16_t)s.req.universe);
    flagsLen(v, DMP_OFF, total);
    v.push_back(VEC_DMP_SET);
    v.push_back(0xa1);          /* address type + data type        */
    v.push_back(0x00);
    v.push_back(0x00);
    v.push_back(0x00);          /* first property address          */
    v.push_back(0x01);          /* address increment               */
    put16(v, (uint16_t)(s.dmx_len + 1)); /* value count incl. start code */
    v.push_back(s.start_code);
    v.insert(v.end(), s.dmx, s.dmx + s.dmx_len);
    return v;
}

std::vector<uint8_t> SacnService::buildSync(const Src& s) {
    std::vector<uint8_t> v;
    v.reserve(49);
    rootHdr(v, VEC_EXT, s.req.cid, 49);
    flagsLen(v, FLEN_FRAME, 49);
    put32(v, VEC_FRAME_SYNC);
    v.push_back((uint8_t)s.sync_seq);
    put16(v, (uint16_t)s.req.sync_addr);
    v.push_back(0);
    v.push_back(0);
    return v;
}

std::vector<uint8_t> SacnService::buildDiscovery(const Src& s) {
    /* single-universe discovery advertisement */
    std::vector<uint8_t> v;
    size_t total = 122;
    v.reserve(total);
    rootHdr(v, VEC_EXT, s.req.cid, total);
    flagsLen(v, FLEN_FRAME, total);
    put32(v, VEC_FRAME_DISC);
    size_t nl = strnlen(s.req.name, TPLC_SACN_NAME_LEN - 1);
    v.insert(v.end(), s.req.name, s.req.name + nl);
    v.resize(v.size() + (TPLC_SACN_NAME_LEN - nl));
    v.push_back(0);
    v.push_back(0);
    v.push_back(0);
    v.push_back(0);             /* reserved                        */
    flagsLen(v, UDL_OFF, total);
    put32(v, VEC_UDL_LIST);
    v.push_back(0);             /* page                            */
    v.push_back(0);             /* last page                       */
    put16(v, (uint16_t)s.req.universe);
    return v;
}

int32_t SacnService::srcBegin(void* s, const TplcSacnSrcReq* req,
                              TplcSacnHandle* out) {
    (void)s;
    if (!out || !req) return TPLC_SACN_E_PARAM;
    if (req->universe < 1 || req->universe > 63999 ||
        req->priority > 200 ||
        (req->sync_addr &&
         (req->sync_addr < 1 || req->sync_addr > 63999)) ||
        strnlen(req->name, TPLC_SACN_NAME_LEN) >= TPLC_SACN_NAME_LEN)
        return TPLC_SACN_E_RANGE;

    int fd = ::socket(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK, 0);
    if (fd < 0) return TPLC_SACN_E_IO;
    int one = 1;
    ::setsockopt(fd, SOL_SOCKET, SO_BROADCAST, &one, sizeof(one));
    int ttl = req->ttl ? (int)req->ttl : 8;
    if (ttl > 255) ttl = 255;
    ::setsockopt(fd, IPPROTO_IP, IP_MULTICAST_TTL, &ttl, sizeof(ttl));
    one = 1;
    ::setsockopt(fd, IPPROTO_IP, IP_MULTICAST_LOOP, &one, sizeof(one));

    Src src;
    src.fd = fd;
    src.req = *req;
    /* all-zero CID -> derive a per-handle pseudo CID */
    bool zero = true;
    for (uint32_t i = 0; i < TPLC_SACN_CID_LEN; i++)
        if (req->cid[i]) { zero = false; break; }
    if (zero) {
        uint64_t h = (uint64_t)next_src_ * 0x9E3779B97F4A7C15ull ^
                     nowNs();
        for (uint32_t i = 0; i < TPLC_SACN_CID_LEN; i++) {
            src.req.cid[i] = (uint8_t)(h >> ((i % 8) * 8));
            h = h * 0x100000001B3ull + i;
        }
    }

    TplcSacnHandle h = next_src_++;
    if (h == TPLC_SACN_HANDLE_INVALID) h = next_src_++;
    srcs_[h] = src;
    *out = h;
    return TPLC_SACN_OK;
}

int32_t SacnService::srcPoll(void* s, TplcSacnHandle h,
                              const TplcSacnFrame* frame,
                              TplcSacnSrcInfo* info) {
    (void)s;
    Src* t = findSrc(h);
    if (!t || !info) return TPLC_SACN_E_STATE;
    if (frame) {
        if (!frame->dmx && frame->dmx_len)
            return TPLC_SACN_E_PARAM;
        size_t n = frame->dmx_len > TPLC_SACN_MAX_DMX
                       ? TPLC_SACN_MAX_DMX
                       : frame->dmx_len;
        if (n && frame->dmx)
            memcpy(t->dmx, frame->dmx, n);
        t->dmx_len = (uint16_t)n;
        t->start_code = frame->start_code;
        t->have_frame = true;
    }
    if (!t->have_frame) {
        fillSrcInfo(*t, info);
        return TPLC_SACN_OK;
    }

    uint64_t now = nowNs();
    uint64_t iv = (uint64_t)t->req.interval_ms * 1000000ull;
    if (t->last_send_ns && iv && now - t->last_send_ns < iv) {
        fillSrcInfo(*t, info);
        return TPLC_SACN_OK;
    }

    /* build and send the data packet */
    std::vector<uint8_t> pkt = buildData(*t);
    uint8_t ip[4];
    mcastAddr(t->req.universe, ip);
    struct sockaddr_in dst{};
    dst.sin_family = AF_INET;
    dst.sin_port = htons(TPLC_SACN_PORT);
    dst.sin_addr.s_addr = htonl((ip[0] << 24) | (ip[1] << 16) |
                                (ip[2] << 8) | ip[3]);
    ssize_t n = ::sendto(t->fd, pkt.data(), pkt.size(), MSG_NOSIGNAL,
                         (struct sockaddr*)&dst, sizeof(dst));
    if (n < 0) {
        t->last_err = errno;
        fillSrcInfo(*t, info);
        return TPLC_SACN_OK; /* retry next poll */
    }
    t->sent++;
    t->seq = (t->seq + 1) & 0xff;
    t->last_send_ns = now;
    t->last_err = 0;

    /* optional aux packets ride the same interval */
    if (t->req.flags & FLAG_SYNC) {
        if (t->req.sync_addr) {
            std::vector<uint8_t> sp = buildSync(*t);
            uint8_t sip[4];
            mcastAddr(t->req.sync_addr, sip);
            dst.sin_addr.s_addr = htonl((sip[0] << 24) |
                                        (sip[1] << 16) |
                                        (sip[2] << 8) | sip[3]);
            if (::sendto(t->fd, sp.data(), sp.size(), MSG_NOSIGNAL,
                         (struct sockaddr*)&dst, sizeof(dst)) > 0) {
                t->sync_seq = (t->sync_seq + 1) & 0xff;
                t->aux_sent++;
            }
        }
    }
    if (t->req.flags & FLAG_DISC) {
        std::vector<uint8_t> dp = buildDiscovery(*t);
        struct sockaddr_in bcast{};
        bcast.sin_family = AF_INET;
        bcast.sin_port = htons(TPLC_SACN_PORT);
        bcast.sin_addr.s_addr = htonl(INADDR_BROADCAST);
        if (::sendto(t->fd, dp.data(), dp.size(), MSG_NOSIGNAL,
                     (struct sockaddr*)&bcast, sizeof(bcast)) > 0)
            t->aux_sent++;
    }
    fillSrcInfo(*t, info);
    return TPLC_SACN_OK;
}

int32_t SacnService::srcClose(void* s, TplcSacnHandle h) {
    (void)s;
    auto it = srcs_.find(h);
    if (it == srcs_.end()) return TPLC_SACN_E_STATE;
    if (it->second.fd >= 0) ::close(it->second.fd);
    srcs_.erase(it);
    return TPLC_SACN_OK;
}

/* ------------------------- RX side ------------------------- */

int32_t SacnService::rcvBegin(void* s, const TplcSacnRcvReq* req,
                              TplcSacnHandle* out) {
    (void)s;
    if (!out || !req) return TPLC_SACN_E_PARAM;
    if (req->universe < 1 || req->universe > 63999 ||
        (req->sync_universe &&
         (req->sync_universe < 1 || req->sync_universe > 63999)))
        return TPLC_SACN_E_RANGE;
    uint16_t port = req->port ? (uint16_t)req->port : TPLC_SACN_PORT;

    int fd = ::socket(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK, 0);
    if (fd < 0) return TPLC_SACN_E_IO;
    int one = 1;
    ::setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));
    struct sockaddr_in sa{};
    sa.sin_family = AF_INET;
    sa.sin_port = htons(port);
    sa.sin_addr.s_addr = htonl(INADDR_ANY);
    if (::bind(fd, (struct sockaddr*)&sa, sizeof(sa)) < 0) {
        ::close(fd);
        return TPLC_SACN_E_IO;
    }
    auto join = [&](uint32_t universe) {
        struct ip_mreq m{};
        uint8_t ip[4];
        mcastAddr(universe, ip);
        m.imr_multiaddr.s_addr = htonl((ip[0] << 24) |
                                       (ip[1] << 16) |
                                       (ip[2] << 8) | ip[3]);
        m.imr_interface.s_addr = htonl(INADDR_ANY);
        return ::setsockopt(fd, IPPROTO_IP, IP_ADD_MEMBERSHIP,
                            &m, sizeof(m)) == 0;
    };
    if (!join(req->universe) ||
        (req->sync_universe && !join(req->sync_universe))) {
        ::close(fd);
        return TPLC_SACN_E_IO;
    }

    Rcv r;
    r.fd = fd;
    r.req = *req;
    TplcSacnHandle h = next_rcv_++;
    if (h == TPLC_SACN_HANDLE_INVALID) h = next_rcv_++;
    rcvs_[h] = r;
    *out = h;
    return TPLC_SACN_OK;
}

/* validate the shared root layer; on success returns the root
   vector and requires the datagram to cover the declared length */
static bool sacnRoot(const uint8_t* p, size_t n, uint32_t& vector) {
    if (n < MIN_PACKET) return false;
    static const uint8_t pre[16] = {
        0x00, 0x10, 0x00, 0x00,
        'A', 'S', 'C', '-', 'E', '1', '.', '1', '7', 0, 0, 0};
    if (memcmp(p, pre, 16) != 0) return false;
    size_t len = ((size_t)(p[16] & 0x0f) << 8) | p[17];
    if (n < 16 + len || n < MIN_PACKET) return false;
    vector = ((uint32_t)p[18] << 24) | ((uint32_t)p[19] << 16) |
             ((uint32_t)p[20] << 8) | p[21];
    return true;
}

bool SacnService::parseData(const uint8_t* p, size_t n, Rcv& r) {
    if (n < 126) return false;
    uint32_t fv = get32be(p + 40);
    if (fv != VEC_FRAME_DATA || p[117] != VEC_DMP_SET || p[118] != 0xa1)
        return false;
    size_t dmp_len = ((size_t)(p[115] & 0x0f) << 8) | p[116];
    if (115 + dmp_len > n) return false;
    uint16_t count = get16be(p + 123);
    if (count < 1 || 125 + (size_t)count > n) return false;
    uint16_t universe = get16be(p + 113);
    if (universe != r.req.universe) {
        r.data_dropped++;
        return false;
    }
    /* priority 0 is most important: a packet less important than
       the accepted one is dropped; equal priority takes the newer
       arrival (packets are processed in order) */
    uint8_t prio = p[108];
    if (r.have_data && prio > r.priority) {
        r.data_dropped++;
        return false;
    }
    size_t dmx = count - 1; /* property values minus start code */
    if (dmx > TPLC_SACN_MAX_DMX) dmx = TPLC_SACN_MAX_DMX;
    r.have_data = true;
    r.preview = (p[112] & OPT_PREVIEW) ? 1 : 0;
    r.stream_term = (p[112] & OPT_STREAM_TERM) ? 1 : 0;
    r.force_sync = (p[112] & OPT_FORCE_SYNC) ? 1 : 0;
    r.start_code = p[125];
    r.seq = p[111];
    r.priority = prio;
    r.sync_addr = get16be(p + 109);
    r.universe = universe;
    r.dmx_len = (uint16_t)dmx;
    memcpy(r.dmx, p + 126, dmx);
    size_t nl = strnlen((const char*)(p + NAME_OFF),
                        TPLC_SACN_NAME_LEN);
    memcpy(r.source_name, p + NAME_OFF, nl);
    r.source_name[nl] = 0;
    r.data_rx++;
    return true;
}

bool SacnService::parseSync(const uint8_t* p, size_t n, Rcv& r) {
    if (n < 49 || get32be(p + 40) != VEC_FRAME_SYNC) return false;
    r.have_sync = true;
    r.sync_seq = p[44];
    r.sync_addr_seen = get16be(p + 45);
    r.sync_rx++;
    return true;
}

bool SacnService::parseDiscovery(const uint8_t* p, size_t n, Rcv& r) {
    if (n < 120 || get32be(p + 40) != VEC_FRAME_DISC ||
        get32be(p + 114) != VEC_UDL_LIST)
        return false;
    size_t udl_len = ((size_t)(p[112] & 0x0f) << 8) | p[113];
    if (UDL_OFF + udl_len > n) return false;
    size_t uni_bytes = udl_len > 8 ? udl_len - 8 : 0;
    size_t avail = n > 120 ? n - 120 : 0;
    if (uni_bytes > avail) uni_bytes = avail;
    size_t count = uni_bytes / 2;
    r.have_discovery = true;
    r.uni_count = (uint16_t)count;
    r.uni_page = p[118];
    r.uni_last_page = p[119];
    size_t w = count < TPLC_SACN_UNI_WINDOW ? count
                                          : TPLC_SACN_UNI_WINDOW;
    for (size_t i = 0; i < w; i++)
        r.universes[i] = get16be(p + 120 + i * 2);
    r.disc_rx++;
    return true;
}

int32_t SacnService::rcvPoll(void* s, TplcSacnHandle h,
                              TplcSacnRcvInfo* info, uint8_t* dmx,
                              uint16_t dmx_cap, uint16_t* unis,
                              uint16_t unis_cap, char* name,
                              uint32_t name_cap) {
    (void)s;
    Rcv* r = findRcv(h);
    if (!r || !info) return TPLC_SACN_E_STATE;
    for (;;) {
        uint8_t buf[1200];
        struct sockaddr_in from{};
        socklen_t fl = sizeof(from);
        ssize_t n = ::recvfrom(r->fd, buf, sizeof(buf), MSG_DONTWAIT,
                               (struct sockaddr*)&from, &fl);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) break;
            break; /* transient errors: keep last state */
        }
        uint32_t vector = 0;
        if (!sacnRoot(buf, (size_t)n, vector)) continue;
        if (vector == VEC_DATA) {
            parseData(buf, (size_t)n, *r);
        } else if (vector == VEC_EXT) {
            uint32_t fv = get32be(buf + 40);
            if (fv == VEC_FRAME_SYNC) parseSync(buf, (size_t)n, *r);
            else if (fv == VEC_FRAME_DISC)
                parseDiscovery(buf, (size_t)n, *r);
        }
        r->peer = from;
        r->have_peer = true;
    }
    memset(info, 0, sizeof(*info));
    info->have_data = r->have_data ? 1 : 0;
    info->preview = r->preview;
    info->stream_terminated = r->stream_term;
    info->force_sync = r->force_sync;
    info->start_code = r->start_code;
    info->seq = r->seq;
    info->priority = r->priority;
    info->sync_addr = r->sync_addr;
    info->universe = r->universe;
    info->dmx_len = r->dmx_len;
    info->data_rx = r->data_rx;
    info->data_dropped = r->data_dropped;
    info->have_sync = r->have_sync ? 1 : 0;
    info->sync_seq = r->sync_seq;
    info->sync_addr_seen = r->sync_addr_seen;
    info->sync_rx = r->sync_rx;
    info->have_discovery = r->have_discovery ? 1 : 0;
    info->uni_count = r->uni_count;
    info->uni_page = r->uni_page;
    info->uni_last_page = r->uni_last_page;
    info->disc_rx = r->disc_rx;
    if (r->have_peer) {
        uint32_t a = ntohl(r->peer.sin_addr.s_addr);
        info->remote_ip[0] = (a >> 24) & 0xff;
        info->remote_ip[1] = (a >> 16) & 0xff;
        info->remote_ip[2] = (a >> 8) & 0xff;
        info->remote_ip[3] = a & 0xff;
    }
    if (dmx && dmx_cap)
        memcpy(dmx, r->dmx, r->dmx_len < dmx_cap ? r->dmx_len
                                                  : dmx_cap);
    if (unis && unis_cap) {
        uint16_t w = r->uni_count < unis_cap ? r->uni_count
                                             : unis_cap;
        memcpy(unis, r->universes, w * sizeof(uint16_t));
    }
    if (name && name_cap) {
        size_t nl = strnlen(r->source_name, TPLC_SACN_NAME_LEN);
        if (nl >= name_cap) nl = name_cap - 1;
        memcpy(name, r->source_name, nl);
        name[nl] = 0;
    }
    return TPLC_SACN_OK;
}

int32_t SacnService::rcvClose(void* s, TplcSacnHandle h) {
    (void)s;
    auto it = rcvs_.find(h);
    if (it == rcvs_.end()) return TPLC_SACN_E_STATE;
    if (it->second.fd >= 0) ::close(it->second.fd);
    rcvs_.erase(it);
    return TPLC_SACN_OK;
}

SacnService::Src* SacnService::findSrc(uint32_t h) {
    auto it = srcs_.find(h);
    return it == srcs_.end() ? nullptr : &it->second;
}
SacnService::Rcv* SacnService::findRcv(uint32_t h) {
    auto it = rcvs_.find(h);
    return it == rcvs_.end() ? nullptr : &it->second;
}

} /* namespace tplc */
