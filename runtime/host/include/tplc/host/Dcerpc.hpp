/* Dcerpc.hpp — DCE/RPC connection-oriented PDU codec (MS-RPCE / C706)
 * for PROFINET CM (PNIO-CM) acyclic RPC over UDP 34964.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_DCERPC_HPP
#define TPLC_HOST_DCERPC_HPP

#include <array>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

namespace tplc {

/* PDU types (MS-RPCE 2.2.1.1.3) */
enum class DcPtype : uint8_t {
    Request = 0x00, Ping = 0x01, Response = 0x02, Fault = 0x03,
    Working = 0x04, NoCall = 0x05, Reject = 0x06, Ack = 0x07,
    ClCancel = 0x08, FAck = 0x09, CancelAck = 0x0A,
    Bind = 0x0B, BindAck = 0x0C, BindNak = 0x0D,
    AlterCtx = 0x0E, AlterCtxResp = 0x0F, Auth3 = 0x10,
    Shutdown = 0x11, CoCancel = 0x12, Orphaned = 0x13,
};

/* pfc_flags bits */
inline constexpr uint8_t kDcFirstFrag = 0x01;
inline constexpr uint8_t kDcLastFrag  = 0x02;
inline constexpr uint8_t kDcConcMpx   = 0x10;
inline constexpr uint8_t kDcMaybe     = 0x40;
inline constexpr uint8_t kDcObjUuid   = 0x80;

/* data-representation bytes: ASCII char rep + little-endian int/float
 * (the form PROFINET CM peers and MS-RPCE endpoints negotiate) */
inline constexpr uint8_t kDcDrep[4] = {0x10, 0x00, 0x00, 0x00};

/* 16-byte activity/interface UUID in wire (DCE mixed-endian) form */
using DcUuid = std::array<uint8_t, 16>;

/* 20-byte abstract/transfer syntax blob: uuid + ver u16 + minor u16 */
struct DcSyntax {
    DcUuid uuid{};
    uint16_t ver = 1;
    uint16_t minor = 0;
};

DcUuid dcUuidFromString(const char* str);          /* "dea00001-6c97-..." */
DcSyntax dcNdrSyntax();                            /* NDR v2.0 transfer syntax */
std::string dcUuidToString(const DcUuid& u);
void dcSyntaxEncode(const DcSyntax& s, uint8_t out[20]);
bool dcSyntaxDecode(const uint8_t* p, DcSyntax& s);

/* ---- PDU encode (caller owns the buffer; returns bytes written) ---- */

/* Common header: ver 5.0, ptype, flags, drep, frag_len (filled by caller
 * via finish), auth_len=0, call_id. */
size_t dcHeader(uint8_t* out, DcPtype type, uint8_t flags, uint32_t callId);

/* request: alloc_hint, ctx_id, opnum [, object uuid], stub bytes */
size_t dcEncRequest(uint8_t* out, uint32_t callId, uint16_t ctxId,
                    uint16_t opnum, uint32_t allocHint,
                    const DcUuid* object, const uint8_t* stub, size_t stubLen);

/* bind / alter_context: ctx items {ctx_id, abstract, transfer} */
struct DcCtxItem { uint16_t ctxId; DcSyntax abstract; DcSyntax transfer; };
size_t dcEncBindLike(uint8_t* out, DcPtype type, uint32_t callId,
                     const std::vector<DcCtxItem>& items,
                     uint16_t maxFrag = 4280);

/* auth3 (sec_trailer-only body) and shutdown */
size_t dcEncAuth3(uint8_t* out, uint32_t callId, uint8_t authType,
                  uint8_t authLevel, uint32_t authCtxId,
                  const uint8_t* authData, size_t authLen);
size_t dcEncShutdown(uint8_t* out, uint32_t callId);

/* ---- PDU decode ---- */

struct DcPdu {
    DcPtype type = DcPtype::Request;
    uint8_t flags = 0;
    uint16_t fragLen = 0;
    uint16_t authLen = 0;
    uint32_t callId = 0;
    const uint8_t* body = nullptr;   /* points into the source buffer */
    size_t bodyLen = 0;
    /* sec_trailer (present iff authLen > 0, 8 bytes before auth_data) */
    uint8_t authType = 0, authLevel = 0, authPadLen = 0;
    uint32_t authCtxId = 0;
};

/* Parse one complete PDU from buf[0..len). Returns false on malformed
 * header or truncated body. `body`/`bodyLen` cover the pduData region
 * (excludes pad, sec_trailer, auth_data). */
bool dcParse(const uint8_t* buf, size_t len, DcPdu& pdu);

/* request/response/fault fixed fields */
bool dcParseRequest(const DcPdu& p, uint32_t& allocHint, uint16_t& ctxId,
                    uint16_t& opnum, DcUuid* object, size_t& stubOff,
                    size_t& stubLen);
bool dcParseResponse(const DcPdu& p, uint32_t& allocHint, uint16_t& ctxId,
                     uint8_t& cancelCount);
bool dcParseFault(const DcPdu& p, uint32_t& allocHint, uint16_t& cancelCount,
                  uint32_t& status, const uint8_t** stub, size_t& stubLen);

/* bind_ack body: max frags, assoc group, secondary addr, ctx results */
struct DcBindResult { uint16_t result; uint16_t reason; DcSyntax transfer; };
struct DcBindAckBody {
    uint16_t maxXmit = 0, maxRecv = 0;
    uint32_t assocGroup = 0;
    std::string secondaryAddr;
    std::vector<DcBindResult> results;
};
bool dcParseBindAck(const DcPdu& p, DcBindAckBody& out);
bool dcParseBindNak(const DcPdu& p, uint16_t& reason);

/* fragmentation helpers for the link layer */
inline bool dcIsFirstFrag(const DcPdu& p) { return (p.flags & kDcFirstFrag) != 0; }
inline bool dcIsLastFrag(const DcPdu& p)  { return (p.flags & kDcLastFrag) != 0; }

/* Well-known PROFINET interface UUIDs (DCE wire form) */
DcUuid dcPnioDeviceUuid();       /* dea00001-6c97-11d1-8271-00a02442df7d v1 */
DcUuid dcPnioControllerUuid();   /* dea00002-... */
DcUuid dcPnioSupervisorUuid();   /* dea00003-... */
DcUuid dcPnioParamServerUuid();  /* dea00004-... */
DcUuid dcEndpointMapperUuid();   /* e1af8308-5d1f-11c9-91a4-08002b14a0fa */

// ---------------------------------------------------------------------------
// DcRpcLink — minimal UDP transport for PNIO-CM (port 34964).
// DCE/RPC over UDP carries the same CO PDUs; each datagram holds one or
// more fragments. This link reassembles fragments per call_id.
// ---------------------------------------------------------------------------

class DcRpcLink {
public:
    DcRpcLink();
    ~DcRpcLink();
    DcRpcLink(const DcRpcLink&) = delete;
    DcRpcLink& operator=(const DcRpcLink&) = delete;

    /* open a UDP socket bound locally, connected to host:port */
    bool open(const std::string& host, uint16_t port);
    void close();
    bool isOpen() const;

    /* send one encoded PDU buffer (already frag_len'd) */
    bool send(const uint8_t* pdu, size_t len);

    /* receive + reassemble the next complete PDU; timeoutMs <= 0 = block.
     * Returns the full PDU bytes (single buffer, frag_len'd). */
    bool recv(std::vector<uint8_t>& out, int timeoutMs);

    /* receive one raw datagram without CO-PDU parsing/reassembly —
     * for connectionless protocols (PNIO-CM) that share the transport
     * but not the 16-byte CO header layout. */
    bool recvDatagram(std::vector<uint8_t>& out, int timeoutMs);

    /* allocate the call_id the next request must carry; call() matches
     * responses by it. Encode requests with this id. */
    uint32_t allocCallId() { return nextCall_++; }

    /* one-shot call: send request (already encoded with allocCallId()),
     * wait for the matching response/fault/bind_ack. */
    bool call(const uint8_t* req, size_t reqLen,
              std::vector<uint8_t>& resp, int timeoutMs);

private:
    int fd_ = -1;
    uint32_t nextCall_ = 1;
    /* reassembly: call_id → accumulated bytes */
    struct Frag { std::vector<uint8_t> data; bool last = false; };
    std::vector<std::pair<uint32_t, Frag>> frags_;
};

} // namespace tplc
#endif
