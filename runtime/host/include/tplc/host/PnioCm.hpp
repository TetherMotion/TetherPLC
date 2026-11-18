/* PnioCm.hpp — PROFINET IO Context Manager (PNIO-CM) client codec.
 *
 * Implements the controller side of PNIO-CM: connectionless DCE/RPC v4
 * over UDP/34964 (NOT connection-oriented — no bind handshake), the
 * NDR array marshaling that carries PNIO blocks, and the block-level
 * codec for the AR establishment exchange:
 *
 *   Connect (opnum 0)  → ARBlockReq + IOCRBlockReq×N +
 *                        ExpectedSubmoduleBlockReq×N + AlarmCRBlockReq
 *   response           → ARBlockRes + IOCRBlockRes×N + AlarmCRBlockRes +
 *                        ARServerBlockRes
 *   Write   (opnum 3)  → IODWriteReqHeader records (FParametersBlock, …),
 *                        optionally nested in a MultipleWrite (0xe040) record
 *   Control (opnum 4)  → IODControlReq PrmEnd → IODControlRes Done
 *   device→controller  → IODControlReq ApplicationReady → we answer Done
 *   Release (opnum 1)  → IODControlReq Release → IODControlRes Done
 *
 * Wire format verified byte-exact against p-net golden captures
 * (pnio crate testdata/cm/*.hex, real CPU↔device exchange).
 *
 * Endianness: controllers speak little-endian DREP (0x10) on requests and
 * read big-endian (0x00) device responses; the device-initiated
 * ApplicationReady request arrives big-endian and our answer is
 * little-endian (flags1 0x0a, matching the capture).
 */
#pragma once

#include <array>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include <tplc/host/Dcerpc.hpp>

namespace tplc::host {

/* ---- constants ---- */

inline constexpr uint16_t kPnioUdpPort = 34964;
inline constexpr size_t kPnioRpcHdrLen = 80;
inline constexpr size_t kPnioNdrLen = 20;
inline constexpr uint32_t kPnioArgsMax = 0xFFFF; /* args_max seen: 557/344/32/1340 — use negotiated */

/* DCE/RPC connectionless packet types (subset used by PNIO-CM). */
enum class PnioPtype : uint8_t {
    Request = 0,
    Response = 2,
    Fault = 3,
};

/* flags1 bits. */
inline constexpr uint8_t kPnioFlagLastFrag = 0x02;
inline constexpr uint8_t kPnioFlagFrag = 0x04;
inline constexpr uint8_t kPnioFlagNoFack = 0x08;
inline constexpr uint8_t kPnioFlagIdempotent = 0x20;

/* PNIO-CM opnums (device interface). */
enum class PnioOpnum : uint16_t {
    Connect = 0,
    Release = 1,
    Read = 2,
    Write = 3,
    Control = 4,
    ReadImplicit = 5,
};

/* Interface UUIDs (canonical byte order, as printed). */
inline const std::array<uint8_t, 16> kPnioDeviceIfaceUuid = {
    0xde, 0xa0, 0x00, 0x01, 0x6c, 0x97, 0x11, 0xd1,
    0x82, 0x71, 0x00, 0xa0, 0x24, 0x42, 0xdf, 0x7d};
inline const std::array<uint8_t, 16> kPnioControllerIfaceUuid = {
    0xde, 0xa0, 0x00, 0x02, 0x6c, 0x97, 0x11, 0xd1,
    0x82, 0x71, 0x00, 0xa0, 0x24, 0x42, 0xdf, 0x7d};

/* PNIO object UUID prefix: dea00000-6c97-11d1-8271-{instance}{device}{vendor}.
 * instance u16 BE at [8..10), device_id u16 BE at [10..12), vendor u32 BE [12..16). */
inline std::array<uint8_t, 16> pnioObjectUuid(uint16_t instance,
                                               uint16_t deviceId,
                                               uint32_t vendorId) {
    std::array<uint8_t, 16> u = {
        0xde, 0xa0, 0x00, 0x00, 0x6c, 0x97, 0x11, 0xd1,
        0x82, 0x71, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
    u[8] = uint8_t(instance >> 8); u[9] = uint8_t(instance);
    u[10] = uint8_t(deviceId >> 8); u[11] = uint8_t(deviceId);
    u[12] = uint8_t(vendorId >> 24); u[13] = uint8_t(vendorId >> 16);
    u[14] = uint8_t(vendorId >> 8); u[15] = uint8_t(vendorId);
    return u;
}

/* ---- endianness helpers ---- */

struct PnioDrep {
    bool little = true; /* controllers: LE requests, BE device responses */
    static PnioDrep fromByte(uint8_t b) { return { (b & 0x10) != 0 }; }
    uint8_t byte0() const { return little ? 0x10 : 0x00; }
};

/* ---- CL RPC PDU ---- */

struct PnioRpcHeader {
    PnioPtype ptype = PnioPtype::Request;
    uint8_t flags1 = kPnioFlagIdempotent;
    uint8_t flags2 = 0;
    PnioDrep drep{true};
    uint8_t serialHi = 0, serialLo = 0;
    std::array<uint8_t, 16> object{};     /* target object UUID */
    std::array<uint8_t, 16> interface{};  /* service interface UUID */
    std::array<uint8_t, 16> activity{};   /* session activity UUID */
    uint32_t serverBoot = 0;
    uint32_t ifVersion = 1;
    uint32_t seqNum = 0;
    uint16_t opnum = 0;
    uint16_t ihint = 0xffff, ahint = 0xffff;
    uint16_t fragLen = 0;   /* bytes after the 80-byte header (NDR+blocks) */
    uint16_t fragNum = 0;
    uint8_t authProto = 0;
};

struct PnioRpcPdu {
    PnioRpcHeader hdr;
    std::vector<uint8_t> ndr;    /* 20-byte NDR array header */
    std::vector<uint8_t> blocks; /* PNIO blocks payload */
};

/* Encode an 80-byte CL header. out must have ≥80 bytes free. */
void pnioEncRpcHeader(uint8_t* out, const PnioRpcHeader& h);

/* Parse an 80-byte CL header; false on malformed input. */
bool pnioDecRpcHeader(const uint8_t* in, size_t len, PnioRpcHeader& h);

/* NDR request header: [argsMax,argsLen,maxCount,offset,actualCount] in drep.
 * Controller convention: all five = blocks length, offset 0. */
void pnioEncNdrRequest(uint8_t out[20], uint32_t blocksLen, PnioDrep drep);
/* NDR response header: [status,argsLen,maxCount=echo argsMax,offset,actual]. */
void pnioEncNdrResponse(uint8_t out[20], uint32_t status, uint32_t blocksLen,
                        uint32_t reqArgsMax, PnioDrep drep);

struct PnioNdrResponse {
    uint32_t status = 0;
    uint32_t argsLen = 0, maxCount = 0, offset = 0, actualCount = 0;
    const uint8_t* blocks = nullptr; /* points into caller buffer */
    size_t blocksLen = 0;
};
/* Parse the 20-byte NDR response header + locate block payload. */
bool pnioDecNdrResponse(const uint8_t* in, size_t len, PnioDrep drep,
                        PnioNdrResponse& r);

/* Build a complete request PDU (header+NDR+blocks) for `blocks`. */
std::vector<uint8_t> pnioBuildRequest(const PnioRpcHeader& base,
                                      PnioOpnum opnum,
                                      const std::vector<uint8_t>& blocks,
                                      uint32_t argsMax = 0xFFFF);

/* ---- PNIO block codec ---- */

/* Block types. */
enum PnioBlockType : uint16_t {
    kBlkArBlockReq = 0x0101,
    kBlkIocrBlockReq = 0x0102,
    kBlkAlarmCrBlockReq = 0x0103,
    kBlkExpectedSubmoduleBlockReq = 0x0104,
    kBlkIodWriteReqHeader = 0x0008,
    kBlkIodReadReqHeader = 0x0009,
    kBlkIodControlReqPrmEnd = 0x0110,
    kBlkIoxBlockReqAppReady = 0x0112,
    kBlkReleaseBlockReq = 0x0114,
    kBlkArBlockRes = 0x8101,
    kBlkIocrBlockRes = 0x8102,
    kBlkAlarmCrBlockRes = 0x8103,
    kBlkArServerBlockRes = 0x8106,
    kBlkIodWriteResHeader = 0x8008,
    kBlkIodReadResHeader = 0x8009,
    kBlkIodControlResPrmEnd = 0x8110,
    kBlkIoxBlockResAppReady = 0x8112,
    kBlkReleaseBlockRes = 0x8114,
};

/* Append a 6-byte block header (type, len=body+2 incl. version, ver 1.0). */
void pnioBlockHeader(std::vector<uint8_t>& out, uint16_t type, uint16_t bodyLen);

/* Big-endian cursor over a block body. */
struct PnioCursor {
    const uint8_t* p;
    size_t n, pos = 0;
    bool u8(uint8_t& v);
    bool u16(uint16_t& v);
    bool i16(int16_t& v);
    bool u32(uint32_t& v);
    bool uuid(std::array<uint8_t, 16>& v); /* canonical order */
    bool mac(uint8_t v[6]);
    bool bytes(size_t n, const uint8_t*& v);
    bool skip(size_t n);
    size_t remaining() const { return n - pos; }
};

/* ---- Connect request blocks ---- */

struct PnioIocrObject {
    uint16_t slot = 0, subslot = 0, frameOffset = 0;
};
struct PnioIocrApi {
    uint32_t api = 0;
    std::vector<PnioIocrObject> ioData, iocs;
};
struct PnioIocrBlockReq {
    uint16_t iocrType = 0; /* 1=Input, 2=Output */
    uint16_t reference = 0;
    uint16_t lt = 0x8892;
    uint32_t properties = 0;
    uint16_t dataLength = 0;
    uint16_t frameId = 0xffff; /* 0xffff = device-selected (Output CR only) */
    uint16_t sendClockFactor = 0, reductionRatio = 0;
    uint16_t phase = 0, sequence = 0;
    uint32_t frameSendOffset = 0;
    uint16_t watchdogFactor = 0, dataHoldFactor = 0;
    uint16_t tagHeader = 0;
    uint8_t multicastMac[6] = {};
    std::vector<PnioIocrApi> apis;
};
struct PnioDataDescription {
    uint16_t dataLength = 0;
    uint8_t lengthIocs = 0, lengthIops = 0;
};
struct PnioExpectedSubmodule {
    uint16_t subslot = 0;
    uint32_t submoduleIdent = 0;
    uint16_t properties = 0; /* bits0-1: 0/1 in, 2 out, 3 both */
    std::optional<PnioDataDescription> input, output;
};
struct PnioExpectedApi {
    uint32_t api = 0;
    uint16_t slot = 0;
    uint32_t moduleIdent = 0;
    uint16_t moduleProperties = 0;
    std::vector<PnioExpectedSubmodule> submodules;
};
struct PnioExpectedSubmoduleBlockReq {
    std::vector<PnioExpectedApi> apis;
};
struct PnioAlarmCrBlockReq {
    uint16_t alarmCrType = 1;
    uint16_t lt = 0x8892;
    uint32_t properties = 0;
    /* standard controller defaults (scapy/real CPU captures) */
    uint16_t rtaTimeoutFactor = 0x0001, rtaRetries = 0x0003;
    uint16_t localAlarmReference = 0x0003;
    uint16_t maxAlarmDataLength = 0x00C8;
    uint16_t tagHeaderHigh = 0xC000, tagHeaderLow = 0xA000;
};
struct PnioArBlockReq {
    uint16_t arType = 1; /* IOCARSingle */
    std::array<uint8_t, 16> arUuid{};
    uint16_t sessionKey = 0;
    uint8_t initiatorMac[6] = {};
    std::array<uint8_t, 16> initiatorObjectUuid{};
    uint32_t arProperties = 0;
    uint16_t activityTimeoutFactor = 0;
    uint16_t initiatorUdpRtPort = 0x8892;
    std::string stationName;
};

/* Serialize Connect request blocks in the order given. */
void pnioEncArBlockReq(std::vector<uint8_t>& out, const PnioArBlockReq& b);
void pnioEncIocrBlockReq(std::vector<uint8_t>& out, const PnioIocrBlockReq& b);
void pnioEncExpectedSubmoduleBlockReq(std::vector<uint8_t>& out,
                                      const PnioExpectedSubmoduleBlockReq& b);
void pnioEncAlarmCrBlockReq(std::vector<uint8_t>& out,
                            const PnioAlarmCrBlockReq& b);

/* ---- Connect response blocks ---- */

struct PnioConnectRes {
    std::array<uint8_t, 16> arUuid{};
    uint16_t sessionKey = 0;
    uint8_t responderMac[6] = {};
    uint16_t responderUdpRtPort = 0;
    /* per-IOCRBlockRes, in response order */
    struct IocrRes { uint16_t iocrType = 0, reference = 0, frameId = 0; };
    std::vector<IocrRes> iocrs;
    uint16_t alarmLocalRef = 0, alarmMaxDataLen = 0;
    std::string stationName; /* ARServerBlockRes */
};
/* Parse the concatenated response blocks. false on malformed. */
bool pnioDecConnectRes(const uint8_t* blocks, size_t len, PnioConnectRes& r);

/* ---- Write records ---- */

inline constexpr uint16_t kPnioIndexMultipleWrite = 0xe040;
inline constexpr uint16_t kPnioIndexFParameters = 0x0100;

struct PnioWriteRecord {
    uint16_t seq = 0;
    std::array<uint8_t, 16> arUuid{};
    uint32_t api = 0xffffffff;
    uint16_t slot = 0xffff, subslot = 0xffff;
    uint16_t index = 0;
    std::vector<uint8_t> data;
};
void pnioEncIodWriteReq(std::vector<uint8_t>& out, const PnioWriteRecord& r);
/* Build a Write-request blocks payload: either a single record, or a
 * MultipleWrite container nesting `inner` records. */
void pnioEncWriteBlocks(std::vector<uint8_t>& out,
                        const std::vector<PnioWriteRecord>& inner);

struct PnioWriteResRecord {
    uint16_t seq = 0;
    std::array<uint8_t, 16> arUuid{};
    uint32_t api = 0;
    uint16_t slot = 0, subslot = 0, index = 0;
    uint32_t status = 0;
};
bool pnioDecWriteRes(const uint8_t* blocks, size_t len,
                     std::vector<PnioWriteResRecord>& out);

/* ---- Control blocks (PrmEnd / AppReady / Release) ---- */

enum PnioControlCmd : uint16_t {
    kCmdPrmEnd = 0x0001,
    kCmdAppReady = 0x0002,
    kCmdRelease = 0x0004,
    kCmdDone = 0x0008,
};
struct PnioControlBlock {
    uint16_t blockType = 0;
    std::array<uint8_t, 16> arUuid{};
    uint16_t sessionKey = 0;
    uint16_t command = 0;
    uint16_t properties = 0;
};
void pnioEncControlBlock(std::vector<uint8_t>& out, const PnioControlBlock& b);
bool pnioDecControlBlock(const uint8_t* blocks, size_t len, PnioControlBlock& b);
/* convenience builders */
PnioControlBlock pnioPrmEndReq(const std::array<uint8_t,16>& ar, uint16_t key);
PnioControlBlock pnioPrmEndRes(const PnioControlBlock& req);
PnioControlBlock pnioAppReadyReq(const std::array<uint8_t,16>& ar, uint16_t key);
PnioControlBlock pnioAppReadyRes(const PnioControlBlock& req);
PnioControlBlock pnioReleaseReq(const std::array<uint8_t,16>& ar, uint16_t key);
PnioControlBlock pnioReleaseRes(const PnioControlBlock& req);

/* ---- F-Parameters block (IODWrite index 0x0100) ---- */

struct PnioFParameters {
    uint8_t flag1 = 0;      /* see scapy FParametersBlock bit layout */
    uint8_t flag2 = 0;
    uint16_t sourceAdd = 0, destAdd = 0;
    int16_t watchdogTime = 0;
    std::optional<int16_t> watchdogTime2; /* when blockId has WD_Time2 */
    std::optional<uint32_t> iParCrc;      /* when blockId has iPar_CRC */
    uint16_t parCrc = 0;
};
/* flag2 bits: [7:6] F_Par_Version(2), [5:3] F_Block_ID(3), [2:1] rsv, [0] passivation */
uint8_t pnioFBlockId(const PnioFParameters& f);
void pnioEncFParameters(std::vector<uint8_t>& out, const PnioFParameters& f);
bool pnioDecFParameters(const uint8_t* data, size_t len, PnioFParameters& f);

/* ---- AR client session ---- */

struct PnioArConfig {
    /* device identity (from DCP/I&M) */
    std::array<uint8_t, 16> deviceObjectUuid{};
    std::array<uint8_t, 16> arUuid{};          /* controller-chosen AR UUID */
    std::array<uint8_t, 16> activityUuid{};   /* session activity UUID */
    uint8_t controllerMac[6] = {};
    std::array<uint8_t, 16> controllerObjectUuid{};
    std::string stationName;
    /* negotiated content */
    PnioArBlockReq ar;
    std::vector<PnioIocrBlockReq> iocrs;       /* input then output */
    std::vector<PnioExpectedSubmoduleBlockReq> expected;
    PnioAlarmCrBlockReq alarmCr;
    /* our args_max announced in NDR requests (capture used up to 1340) */
    uint32_t argsMax = 0x1000;
};

enum class PnioArState {
    Idle,
    Connected,     /* Connect accepted, params negotiated */
    PrmEndSent,    /* waiting device AppReady */
    DataExchange,  /* AppReady done — cyclic IO may proceed */
    Released,
};

class PnioArClient {
public:
    /* transport: reuse DcRpcLink's UDP socket */
    bool open(DcRpcLink& link, const std::string& host, uint16_t port);
    void close();
    bool isOpen() const;

    PnioArState state() const { return state_; }
    const PnioConnectRes& connectRes() const { return res_; }
    uint32_t lastStatus() const { return lastStatus_; }

    /* Blocking-with-timeout RPC call: send request PDU, wait matching
     * response (same seq_num/opnum/interface). Returns response blocks. */
    bool rpcCall(PnioOpnum opnum, const std::vector<uint8_t>& blocks,
                 std::vector<uint8_t>& resBlocks, uint32_t& status,
                 int timeoutMs = 3000);

    /* AR establishment sequence. */
    bool connect(const PnioArConfig& cfg, int timeoutMs = 3000);
    /* Connect with caller-encoded request blocks (service/C-API path):
     * the three UUIDs identify the RPC target, activity and AR. Fills
     * connectRes() on success. */
    bool connectRaw(const std::array<uint8_t,16>& devObj,
                    const std::array<uint8_t,16>& actUuid,
                    const std::array<uint8_t,16>& arUuid,
                    const uint8_t* blocks, size_t len,
                    int timeoutMs = 3000);
    /* raw response blocks of the most recent successful rpcCall */
    const std::vector<uint8_t>& lastResBlocks() const { return lastRes_; }
    bool writeRecords(const std::vector<PnioWriteRecord>& records,
                      int timeoutMs = 3000);
    bool prmEnd(int timeoutMs = 3000);
    bool release(int timeoutMs = 3000);

    /* Pump one datagram (non-blocking-ish): handles device-initiated
     * ApplicationReady and stray responses. Returns true if it consumed
     * an AppReady request (caller may then proceed to data exchange). */
    bool service(int timeoutMs = 0);

    /* seq_num management — monotonic per activity like the capture */
    uint32_t nextSeq() { return seq_++; }

private:
    DcRpcLink* link_ = nullptr;
    PnioArConfig cfg_{};
    PnioConnectRes res_{};
    PnioArState state_ = PnioArState::Idle;
    uint32_t seq_ = 0;
    uint32_t lastStatus_ = 0;
    bool haveCfg_ = false;
    std::vector<uint8_t> lastRes_;
};

} // namespace tplc::host
