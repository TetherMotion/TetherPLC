/* Dcerpc.cpp — DCE/RPC CO PDU codec implementation.
 * Field order verified against MS-RPCE 2.2.1.1 and the impacket
 * MSRPC* structures; header numerics are little-endian per the
 * negotiated data representation (kDcDrep). */
#include <tplc/host/Dcerpc.hpp>

#include <cctype>
#include <cstdio>
#include <cstring>

namespace tplc {
namespace {

uint16_t rd16(const uint8_t* p) { return uint16_t(p[0] | (p[1] << 8)); }
uint32_t rd32(const uint8_t* p) {
    return uint32_t(p[0]) | (uint32_t(p[1]) << 8) |
           (uint32_t(p[2]) << 16) | (uint32_t(p[3]) << 24);
}
void wr16(uint8_t* p, uint16_t v) { p[0] = uint8_t(v); p[1] = uint8_t(v >> 8); }
void wr32(uint8_t* p, uint32_t v) {
    p[0] = uint8_t(v); p[1] = uint8_t(v >> 8);
    p[2] = uint8_t(v >> 16); p[3] = uint8_t(v >> 24);
}

int hexval(char c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    return -1;
}

/* "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX" → 16 wire bytes.
 * DCE UUID wire order: time_low u32 LE, time_mid u16 LE,
 * time_hi_and_version u16 BE, clock_seq+node as-is. */
DcUuid uuidFromHex(const char* s) {
    // collect up to 32 hex digits → 16 bytes in textual order
    uint8_t b[16] = {};
    int nib = -1, idx = 0;
    for (const char* p = s; *p && idx < 16; ++p) {
        int h = hexval(*p);
        if (h < 0) continue;
        if (nib < 0) { b[idx] = uint8_t(h << 4); nib = 0; }
        else { b[idx] |= uint8_t(h); ++idx; nib = -1; }
    }
    if (idx != 16) return DcUuid{};
    // DCE UUID wire order: time_low u32 LE, time_mid u16 LE,
    // time_hi_and_version u16 BE, clock_seq+node as-is.
    DcUuid u{};
    u[0] = b[3]; u[1] = b[2]; u[2] = b[1]; u[3] = b[0];
    u[4] = b[5]; u[5] = b[4];
    u[6] = b[6]; u[7] = b[7];
    for (int i = 8; i < 16; ++i) u[i] = b[i];
    return u;
}

DcUuid uuidPnio(uint32_t first3) {
    // all PNIO activity UUIDs share tail 6c97-11d1-8271-00a02442df7d
    DcUuid u{};
    char s[40];
    std::snprintf(s, sizeof s, "%08x-6c97-11d1-8271-00a02442df7d", first3);
    return uuidFromHex(s);
}

} // namespace

DcUuid dcUuidFromString(const char* str) { return uuidFromHex(str); }

std::string dcUuidToString(const DcUuid& u) {
    // inverse of uuidFromHex
    char s[40];
    std::snprintf(s, sizeof s,
        "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-"
        "%02x%02x%02x%02x%02x%02x",
        u[3], u[2], u[1], u[0], u[5], u[4], u[6], u[7],
        u[8], u[9], u[10], u[11], u[12], u[13], u[14], u[15]);
    return s;
}

DcSyntax dcNdrSyntax() {
    DcSyntax s;
    s.uuid = uuidFromHex("8a885d04-1ceb-11c9-9fe8-08002b104860");
    s.ver = 2; s.minor = 0;
    return s;
}

void dcSyntaxEncode(const DcSyntax& s, uint8_t out[20]) {
    std::memcpy(out, s.uuid.data(), 16);
    wr16(out + 16, s.ver);
    wr16(out + 18, s.minor);
}

bool dcSyntaxDecode(const uint8_t* p, DcSyntax& s) {
    std::memcpy(s.uuid.data(), p, 16);
    s.ver = rd16(p + 16);
    s.minor = rd16(p + 18);
    return true;
}

DcUuid dcPnioDeviceUuid()       { return uuidPnio(0xdea00001); }
DcUuid dcPnioControllerUuid()   { return uuidPnio(0xdea00002); }
DcUuid dcPnioSupervisorUuid()   { return uuidPnio(0xdea00003); }
DcUuid dcPnioParamServerUuid()  { return uuidPnio(0xdea00004); }
DcUuid dcEndpointMapperUuid()   { return uuidFromHex("e1af8308-5d1f-11c9-91a4-08002b14a0fa"); }

// ---------------------------------------------------------------------------

size_t dcHeader(uint8_t* out, DcPtype type, uint8_t flags, uint32_t callId) {
    out[0] = 5;                 // rpc_vers
    out[1] = 0;                 // rpc_vers_minor
    out[2] = uint8_t(type);
    out[3] = flags;
    std::memcpy(out + 4, kDcDrep, 4);
    wr16(out + 8, 0);           // frag_len — patched by finishPdu
    wr16(out + 10, 0);          // auth_len
    wr32(out + 12, callId);
    return 16;
}

namespace {

/* Patch frag_len (bytes 8..9) to the total PDU length. */
void finishPdu(uint8_t* out, size_t totalLen) {
    wr16(out + 8, uint16_t(totalLen));
}

size_t encSecTrailer(uint8_t* out, uint8_t authType, uint8_t authLevel,
                     uint8_t padLen, uint32_t ctxId) {
    out[0] = authType;
    out[1] = authLevel;
    out[2] = padLen;
    out[3] = 0;
    wr32(out + 4, ctxId);
    return 8;
}

} // namespace

size_t dcEncRequest(uint8_t* out, uint32_t callId, uint16_t ctxId,
                    uint16_t opnum, uint32_t allocHint,
                    const DcUuid* object, const uint8_t* stub, size_t stubLen) {
    uint8_t flags = kDcFirstFrag | kDcLastFrag | kDcConcMpx;
    if (object) flags |= kDcObjUuid;
    size_t n = dcHeader(out, DcPtype::Request, flags, callId);
    wr32(out + n, allocHint); n += 4;
    wr16(out + n, ctxId); n += 2;
    wr16(out + n, opnum); n += 2;
    if (object) { std::memcpy(out + n, object->data(), 16); n += 16; }
    if (stubLen) { std::memcpy(out + n, stub, stubLen); n += stubLen; }
    // pad to 4-byte alignment before (empty) sec_trailer region
    while (n & 3) out[n++] = 0;
    finishPdu(out, n);
    return n;
}

size_t dcEncBindLike(uint8_t* out, DcPtype type, uint32_t callId,
                     const std::vector<DcCtxItem>& items,
                     uint16_t maxFrag) {
    size_t n = dcHeader(out, type, kDcFirstFrag | kDcLastFrag | kDcConcMpx, callId);
    wr16(out + n, maxFrag); n += 2;   // max_xmit_frag
    wr16(out + n, maxFrag); n += 2;   // max_recv_frag
    wr32(out + n, 0); n += 4;         // assoc_group
    wr16(out + n, 0); n += 2;         // secondary addr len (empty for UDP CM)
    out[n++] = uint8_t(items.size()); // num_results
    out[n++] = 0;                     // pad
    out[n++] = 0; out[n++] = 0;       // reserved2
    for (const auto& it : items) {
        wr16(out + n, it.ctxId); n += 2;
        out[n++] = 1;                 // num_trans_items
        out[n++] = 0;                 // pad
        uint8_t tmp[20];
        dcSyntaxEncode(it.abstract, tmp);
        std::memcpy(out + n, tmp, 20); n += 20;
        dcSyntaxEncode(it.transfer, tmp);
        std::memcpy(out + n, tmp, 20); n += 20;
    }
    finishPdu(out, n);
    return n;
}

size_t dcEncAuth3(uint8_t* out, uint32_t callId, uint8_t authType,
                  uint8_t authLevel, uint32_t authCtxId,
                  const uint8_t* authData, size_t authLen) {
    size_t n = dcHeader(out, DcPtype::Auth3, kDcFirstFrag | kDcLastFrag, callId);
    // body is empty; pad to 4 then sec_trailer + auth_data
    while (n & 3) out[n++] = 0;
    n += encSecTrailer(out + n, authType, authLevel, 0, authCtxId);
    if (authLen) { std::memcpy(out + n, authData, authLen); n += authLen; }
    wr16(out + 10, uint16_t(authLen));
    finishPdu(out, n);
    return n;
}

size_t dcEncShutdown(uint8_t* out, uint32_t callId) {
    size_t n = dcHeader(out, DcPtype::Shutdown, kDcFirstFrag | kDcLastFrag, callId);
    finishPdu(out, n);
    return n;
}

// ---------------------------------------------------------------------------
// decode
// ---------------------------------------------------------------------------

bool dcParse(const uint8_t* buf, size_t len, DcPdu& p) {
    if (len < 16 || buf[0] != 5) return false;
    p.type = DcPtype(buf[2]);
    p.flags = buf[3];
    p.fragLen = rd16(buf + 8);
    p.authLen = rd16(buf + 10);
    p.callId = rd32(buf + 12);
    if (p.fragLen > len || p.fragLen < 16) return false;

    /* layout: header(16) | pduData | pad(auth_pad_len) |
       sec_trailer(8, iff authLen>0) | auth_data(authLen).
       body = pduData region = [16, fragLen - authLen - 8 - pad). */
    size_t dataEnd = p.fragLen;
    if (p.authLen) {
        if (p.fragLen < 16 + 8 + p.authLen) return false;
        const uint8_t* st = buf + p.fragLen - p.authLen - 8;
        p.authType = st[0]; p.authLevel = st[1]; p.authPadLen = st[2];
        p.authCtxId = rd32(st + 4);
        if (p.authPadLen > p.fragLen) return false;
        dataEnd = size_t(st - buf) - p.authPadLen;
    }
    p.body = buf + 16;
    p.bodyLen = dataEnd > 16 ? dataEnd - 16 : 0;
    return true;
}

bool dcParseRequest(const DcPdu& p, uint32_t& allocHint, uint16_t& ctxId,
                    uint16_t& opnum, DcUuid* object, size_t& stubOff,
                    size_t& stubLen) {
    if (p.type != DcPtype::Request || p.bodyLen < 8) return false;
    const uint8_t* b = p.body;
    allocHint = rd32(b);
    ctxId = rd16(b + 4);
    opnum = rd16(b + 6);
    size_t off = 8;
    if (p.flags & kDcObjUuid) {
        if (object) std::memcpy(object->data(), b + off, 16);
        off += 16;
    }
    stubOff = off;
    stubLen = p.bodyLen - off;
    return true;
}

bool dcParseResponse(const DcPdu& p, uint32_t& allocHint, uint16_t& ctxId,
                     uint8_t& cancelCount) {
    if (p.type != DcPtype::Response || p.bodyLen < 8) return false;
    const uint8_t* b = p.body;
    allocHint = rd32(b);
    ctxId = rd16(b + 4);
    cancelCount = b[6];
    return true;
}

bool dcParseFault(const DcPdu& p, uint32_t& allocHint, uint16_t& cancelCount,
                  uint32_t& status, const uint8_t** stub, size_t& stubLen) {
    if (p.type != DcPtype::Fault || p.bodyLen < 12) return false;
    const uint8_t* b = p.body;
    allocHint = rd32(b);
    cancelCount = b[4];
    status = rd32(b + 8);
    *stub = b + 12;
    stubLen = p.bodyLen - 12;
    return true;
}

bool dcParseBindAck(const DcPdu& p, DcBindAckBody& out) {
    if (p.type != DcPtype::BindAck && p.type != DcPtype::AlterCtxResp)
        return false;
    const uint8_t* b = p.body;
    if (p.bodyLen < 16) return false;
    out.maxXmit = rd16(b);
    out.maxRecv = rd16(b + 2);
    out.assocGroup = rd32(b + 4);
    uint16_t secLen = rd16(b + 8);
    size_t off = 10;
    if (secLen) {
        out.secondaryAddr.assign(reinterpret_cast<const char*>(b + off),
                                 strnlen(reinterpret_cast<const char*>(b + off), secLen));
        off += secLen;
    }
    while (off & 3) ++off;             // pad to 4
    if (off + 4 > p.bodyLen) return false;
    uint8_t n = b[off];
    off += 4;                          // num_results + reserved(3)
    for (uint8_t i = 0; i < n; ++i) {
        if (off + 24 > p.bodyLen) return false;   // result+reason+syntax(20)
        DcBindResult r;
        r.result = rd16(b + off);
        r.reason = rd16(b + off + 2);
        dcSyntaxDecode(b + off + 4, r.transfer);
        out.results.push_back(r);
        off += 24;
    }
    return true;
}

bool dcParseBindNak(const DcPdu& p, uint16_t& reason) {
    if (p.type != DcPtype::BindNak || p.bodyLen < 2) return false;
    reason = rd16(p.body);
    return true;
}

} // namespace tplc

// ---------------------------------------------------------------------------
// DcRpcLink — UDP transport + fragment reassembly
// ---------------------------------------------------------------------------

#include <arpa/inet.h>
#include <fcntl.h>
#include <netdb.h>
#include <poll.h>
#include <sys/socket.h>
#include <unistd.h>

namespace tplc {

DcRpcLink::DcRpcLink() = default;
DcRpcLink::~DcRpcLink() { close(); }

bool DcRpcLink::open(const std::string& host, uint16_t port) {
    close();
    int fd = ::socket(AF_INET, SOCK_DGRAM, 0);
    if (fd < 0) return false;

    addrinfo hints{};
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_DGRAM;
    addrinfo* res = nullptr;
    const std::string ps = std::to_string(port);
    if (getaddrinfo(host.c_str(), ps.c_str(), &hints, &res) != 0 || !res) {
        ::close(fd);
        return false;
    }
    if (::connect(fd, res->ai_addr, res->ai_addrlen) != 0) {
        freeaddrinfo(res);
        ::close(fd);
        return false;
    }
    freeaddrinfo(res);
    fd_ = fd;
    return true;
}

void DcRpcLink::close() {
    if (fd_ >= 0) ::close(fd_);
    fd_ = -1;
    frags_.clear();
}

bool DcRpcLink::isOpen() const { return fd_ >= 0; }

bool DcRpcLink::send(const uint8_t* pdu, size_t len) {
    if (fd_ < 0) return false;
    return ::send(fd_, pdu, len, 0) == ssize_t(len);
}

bool DcRpcLink::recv(std::vector<uint8_t>& out, int timeoutMs) {
    if (fd_ < 0) return false;
    uint8_t buf[65535];
    for (;;) {
        pollfd pfd{fd_, POLLIN, 0};
        int pr = ::poll(&pfd, 1, timeoutMs);
        if (pr <= 0) return false;
        ssize_t n = ::recv(fd_, buf, sizeof buf, 0);
        if (n < 16) continue;

        DcPdu p;
        if (!dcParse(buf, size_t(n), p)) continue;

        if (dcIsFirstFrag(p) && dcIsLastFrag(p)) {
            out.assign(buf, buf + p.fragLen);
            return true;
        }
        /* multi-fragment PDU: keep frag1's header, append each frag's
         * pduData region (bytes [16, fragLen)) — later fragments do not
         * repeat the type-specific header. Reassembled frag_len is
         * patched on output. No-auth PDUs only (AR path). */
        auto& slot = frags_;
        size_t idx = 0;
        for (; idx < slot.size(); ++idx)
            if (slot[idx].first == p.callId) break;
        if (idx == slot.size()) slot.push_back({p.callId, {}});
        Frag& f = slot[idx].second;
        if (f.data.empty()) {
            f.data.assign(buf, buf + 16);       // common header (frag1)
        }
        // append this fragment's body bytes [16, fragLen)
        f.data.insert(f.data.end(), buf + 16, buf + p.fragLen);
        if (dcIsLastFrag(p)) {
            // patch frag_len = 16 + accumulated body
            uint16_t total = uint16_t(f.data.size());
            f.data[8] = uint8_t(total); f.data[9] = uint8_t(total >> 8);
            // strip any trailing pad: bodyLen accounting not needed for
            // AR-sized PDUs; parsers bound-check against frag_len.
            out = std::move(f.data);
            slot.erase(slot.begin() + long(idx));
            return true;
        }
    }
}

bool DcRpcLink::call(const uint8_t* req, size_t reqLen,
                     std::vector<uint8_t>& resp, int timeoutMs) {
    if (fd_ < 0 || reqLen < 16) return false;
    uint32_t callId = rd32(req + 12);   // caller encoded allocCallId()
    if (!send(req, reqLen)) return false;

    /* collect until a Response/Fault with our call_id arrives complete */
    for (;;) {
        if (!recv(resp, timeoutMs)) return false;
        DcPdu p;
        if (!dcParse(resp.data(), resp.size(), p)) continue;
        if (p.callId != callId) continue;
        if (p.type == DcPtype::Response || p.type == DcPtype::Fault ||
            p.type == DcPtype::BindAck || p.type == DcPtype::BindNak ||
            p.type == DcPtype::AlterCtxResp || p.type == DcPtype::Shutdown)
            return true;
    }
}

} // namespace tplc
