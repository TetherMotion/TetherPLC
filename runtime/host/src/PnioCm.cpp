/* PnioCm.cpp — PNIO-CM connectionless DCE/RPC + block codec.
 * Wire format verified against p-net golden captures (testdata/cm/*.hex).
 */
#include <tplc/host/PnioCm.hpp>

#include <cstring>

namespace tplc::host {
namespace {

void put16(std::vector<uint8_t>& o, uint16_t v) {
    o.push_back(uint8_t(v >> 8)); o.push_back(uint8_t(v));
}
void put32(std::vector<uint8_t>& o, uint32_t v) {
    o.push_back(uint8_t(v >> 24)); o.push_back(uint8_t(v >> 16));
    o.push_back(uint8_t(v >> 8)); o.push_back(uint8_t(v));
}
void putBytes(std::vector<uint8_t>& o, const void* p, size_t n) {
    const uint8_t* b = static_cast<const uint8_t*>(p);
    o.insert(o.end(), b, b + n);
}

/* drep-ordered scalar write into a fixed 80-byte header region */
void hdrPut16(uint8_t* o, size_t off, uint16_t v, bool le) {
    if (le) { o[off] = uint8_t(v); o[off+1] = uint8_t(v >> 8); }
    else    { o[off] = uint8_t(v >> 8); o[off+1] = uint8_t(v); }
}
void hdrPut32(uint8_t* o, size_t off, uint32_t v, bool le) {
    if (le) { o[off]=uint8_t(v); o[off+1]=uint8_t(v>>8);
              o[off+2]=uint8_t(v>>16); o[off+3]=uint8_t(v>>24); }
    else    { o[off]=uint8_t(v>>24); o[off+1]=uint8_t(v>>16);
              o[off+2]=uint8_t(v>>8); o[off+3]=uint8_t(v); }
}
uint16_t hdrGet16(const uint8_t* o, size_t off, bool le) {
    if (le) return uint16_t(o[off]) | (uint16_t(o[off+1]) << 8);
    return (uint16_t(o[off]) << 8) | uint16_t(o[off+1]);
}
uint32_t hdrGet32(const uint8_t* o, size_t off, bool le) {
    if (le) return uint32_t(o[off]) | (uint32_t(o[off+1]) << 8) |
                  (uint32_t(o[off+2]) << 16) | (uint32_t(o[off+3]) << 24);
    return (uint32_t(o[off]) << 24) | (uint32_t(o[off+1]) << 16) |
           (uint32_t(o[off+2]) << 8) | uint32_t(o[off+3]);
}

/* UUID wire form: time_low u32, time_mid u16, time_hi u16 in drep order,
 * remaining 8 bytes verbatim. Canonical storage is big-endian text order. */
void uuidPut(uint8_t* o, const std::array<uint8_t,16>& u, bool le) {
    if (le) {
        /* canonical u[0..4] is BE u32 → write LE */
        o[0]=u[3]; o[1]=u[2]; o[2]=u[1]; o[3]=u[0];
        o[4]=u[5]; o[5]=u[4];
        o[6]=u[7]; o[7]=u[6];
    } else {
        std::memcpy(o, u.data(), 8);
    }
    std::memcpy(o + 8, u.data() + 8, 8);
}
bool uuidGet(const uint8_t* i, std::array<uint8_t,16>& u, bool le) {
    if (le) {
        u[0]=i[3]; u[1]=i[2]; u[2]=i[1]; u[3]=i[0];
        u[4]=i[5]; u[5]=i[4];
        u[6]=i[7]; u[7]=i[6];
    } else {
        std::memcpy(u.data(), i, 8);
    }
    std::memcpy(u.data() + 8, i + 8, 8);
    return true;
}

} // namespace

/* ================= CL RPC codec ================= */

void pnioEncRpcHeader(uint8_t* out, const PnioRpcHeader& h) {
    std::memset(out, 0, kPnioRpcHdrLen);
    out[0] = 4;                    /* version */
    out[1] = uint8_t(h.ptype);
    out[2] = h.flags1;
    out[3] = h.flags2;
    out[4] = h.drep.byte0();       /* drep: 3 bytes, only byte0 meaningful */
    out[5] = 0; out[6] = 0;
    out[7] = h.serialHi;
    uuidPut(out + 8, h.object, h.drep.little);
    uuidPut(out + 24, h.interface, h.drep.little);
    uuidPut(out + 40, h.activity, h.drep.little);
    const bool le = h.drep.little;
    hdrPut32(out, 56, h.serverBoot, le);
    hdrPut32(out, 60, h.ifVersion, le);
    hdrPut32(out, 64, h.seqNum, le);
    hdrPut16(out, 68, h.opnum, le);
    hdrPut16(out, 70, h.ihint, le);
    hdrPut16(out, 72, h.ahint, le);
    hdrPut16(out, 74, h.fragLen, le);
    hdrPut16(out, 76, h.fragNum, le);
    out[78] = h.authProto;
    out[79] = h.serialLo;
}

bool pnioDecRpcHeader(const uint8_t* in, size_t len, PnioRpcHeader& h) {
    if (len < kPnioRpcHdrLen || in[0] != 4) return false;
    h.ptype = PnioPtype(in[1]);
    h.flags1 = in[2]; h.flags2 = in[3];
    h.drep = PnioDrep::fromByte(in[4]);
    h.serialHi = in[7];
    uuidGet(in + 8, h.object, h.drep.little);
    uuidGet(in + 24, h.interface, h.drep.little);
    uuidGet(in + 40, h.activity, h.drep.little);
    const bool le = h.drep.little;
    h.serverBoot = hdrGet32(in, 56, le);
    h.ifVersion = hdrGet32(in, 60, le);
    h.seqNum = hdrGet32(in, 64, le);
    h.opnum = hdrGet16(in, 68, le);
    h.ihint = hdrGet16(in, 70, le);
    h.ahint = hdrGet16(in, 72, le);
    h.fragLen = hdrGet16(in, 74, le);
    h.fragNum = hdrGet16(in, 76, le);
    h.authProto = in[78];
    h.serialLo = in[79];
    if (h.fragNum != 0 || (h.flags1 & kPnioFlagFrag)) return false; /* no frag support */
    return true;
}

void pnioEncNdrRequest(uint8_t out[20], uint32_t blocksLen, PnioDrep drep) {
    const bool le = drep.little;
    hdrPut32(out, 0, blocksLen, le);   /* args_max */
    hdrPut32(out, 4, blocksLen, le);   /* args_len */
    hdrPut32(out, 8, blocksLen, le);   /* max_count echoes args_max */
    hdrPut32(out, 12, 0, le);          /* offset */
    hdrPut32(out, 16, blocksLen, le);  /* actual_count */
}

void pnioEncNdrResponse(uint8_t out[20], uint32_t status, uint32_t blocksLen,
                        uint32_t reqArgsMax, PnioDrep drep) {
    const bool le = drep.little;
    hdrPut32(out, 0, status, le);
    hdrPut32(out, 4, blocksLen, le);      /* args_len */
    hdrPut32(out, 8, reqArgsMax, le);     /* max_count echoes request args_max */
    hdrPut32(out, 12, 0, le);             /* offset */
    hdrPut32(out, 16, blocksLen, le);     /* actual_count */
}

bool pnioDecNdrResponse(const uint8_t* in, size_t len, PnioDrep drep,
                        PnioNdrResponse& r) {
    if (len < kPnioNdrLen) return false;
    const bool le = drep.little;
    r.status = hdrGet32(in, 0, le);
    r.argsLen = hdrGet32(in, 4, le);
    r.maxCount = hdrGet32(in, 8, le);
    r.offset = hdrGet32(in, 12, le);
    r.actualCount = hdrGet32(in, 16, le);
    if (r.argsLen != r.actualCount || r.offset != 0) return false;
    if (kPnioNdrLen + r.actualCount > len) return false;
    r.blocks = in + kPnioNdrLen;
    r.blocksLen = r.actualCount;
    return true;
}

std::vector<uint8_t> pnioBuildRequest(const PnioRpcHeader& base,
                                      PnioOpnum opnum,
                                      const std::vector<uint8_t>& blocks,
                                      uint32_t argsMax) {
    PnioRpcHeader h = base;
    h.ptype = PnioPtype::Request;
    h.flags1 = kPnioFlagIdempotent;
    h.drep = PnioDrep{true};          /* controller requests are LE */
    h.opnum = uint16_t(opnum);
    h.fragLen = uint16_t(kPnioNdrLen + blocks.size());
    h.fragNum = 0;
    h.authProto = 0;

    std::vector<uint8_t> out;
    out.reserve(kPnioRpcHdrLen + kPnioNdrLen + blocks.size());
    uint8_t hdr[kPnioRpcHdrLen];
    pnioEncRpcHeader(hdr, h);
    out.insert(out.end(), hdr, hdr + kPnioRpcHdrLen);
    uint8_t ndr[kPnioNdrLen];
    /* args_max carries the negotiated capacity, not just this payload */
    pnioEncNdrRequest(ndr, uint32_t(blocks.size()), h.drep);
    /* overwrite args_max/max_count with configured capacity */
    hdrPut32(ndr, 0, argsMax, true);
    hdrPut32(ndr, 8, argsMax, true);
    out.insert(out.end(), ndr, ndr + kPnioNdrLen);
    out.insert(out.end(), blocks.begin(), blocks.end());
    return out;
}

/* ================= block codec ================= */

void pnioBlockHeader(std::vector<uint8_t>& out, uint16_t type, uint16_t bodyLen) {
    put16(out, type);
    put16(out, uint16_t(bodyLen + 2)); /* length counts the 2 version bytes */
    out.push_back(1); out.push_back(0); /* version 1.0 */
}

/* --- cursor --- */
bool PnioCursor::u8(uint8_t& v) {
    if (pos + 1 > n) return false;
    v = p[pos++]; return true;
}
bool PnioCursor::u16(uint16_t& v) {
    if (pos + 2 > n) return false;
    v = uint16_t(p[pos] << 8) | p[pos + 1]; pos += 2; return true;
}
bool PnioCursor::i16(int16_t& v) {
    uint16_t u; if (!u16(u)) return false;
    v = int16_t(u); return true;
}
bool PnioCursor::u32(uint32_t& v) {
    if (pos + 4 > n) return false;
    v = (uint32_t(p[pos]) << 24) | (uint32_t(p[pos+1]) << 16) |
        (uint32_t(p[pos+2]) << 8) | p[pos+3];
    pos += 4; return true;
}
bool PnioCursor::uuid(std::array<uint8_t,16>& v) {
    if (pos + 16 > n) return false;
    std::memcpy(v.data(), p + pos, 16); pos += 16; return true;
}
bool PnioCursor::mac(uint8_t v[6]) {
    if (pos + 6 > n) return false;
    std::memcpy(v, p + pos, 6); pos += 6; return true;
}
bool PnioCursor::bytes(size_t cnt, const uint8_t*& v) {
    if (pos + cnt > n) return false;
    v = p + pos; pos += cnt; return true;
}
bool PnioCursor::skip(size_t cnt) {
    if (pos + cnt > n) return false;
    pos += cnt; return true;
}

/* parse one block header; returns body pointer+len, advances past block.
 * bodyLen excludes the 2 version bytes already counted in block_length. */
static bool blockHead(const uint8_t*& p, size_t& left, uint16_t& type,
                      const uint8_t*& body, size_t& bodyLen) {
    if (left < 6) return false;
    type = uint16_t(p[0] << 8) | p[1];
    uint16_t blen = uint16_t(p[2] << 8) | p[3];
    if (p[4] != 1 || p[5] != 0) return false;
    if (blen < 2 || size_t(blen - 2) + 6 > left) return false;
    bodyLen = blen - 2;
    body = p + 6;
    size_t total = 6 + bodyLen;
    p += total; left -= total;
    return true;
}

/* --- ARBlockReq --- */
void pnioEncArBlockReq(std::vector<uint8_t>& out, const PnioArBlockReq& b) {
    std::vector<uint8_t> body;
    put16(body, b.arType);
    putBytes(body, b.arUuid.data(), 16);
    put16(body, b.sessionKey);
    putBytes(body, b.initiatorMac, 6);
    putBytes(body, b.initiatorObjectUuid.data(), 16);
    put32(body, b.arProperties);
    put16(body, b.activityTimeoutFactor);
    put16(body, b.initiatorUdpRtPort);
    put16(body, uint16_t(b.stationName.size()));
    putBytes(body, b.stationName.data(), b.stationName.size());
    pnioBlockHeader(out, kBlkArBlockReq, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

/* --- IOCRBlockReq --- */
void pnioEncIocrBlockReq(std::vector<uint8_t>& out, const PnioIocrBlockReq& b) {
    std::vector<uint8_t> body;
    put16(body, b.iocrType);
    put16(body, b.reference);
    put16(body, b.lt);
    put32(body, b.properties);
    put16(body, b.dataLength);
    put16(body, b.frameId);
    put16(body, b.sendClockFactor);
    put16(body, b.reductionRatio);
    put16(body, b.phase);
    put16(body, b.sequence);
    put32(body, b.frameSendOffset);
    put16(body, b.watchdogFactor);
    put16(body, b.dataHoldFactor);
    put16(body, b.tagHeader);
    putBytes(body, b.multicastMac, 6);
    put16(body, uint16_t(b.apis.size()));
    for (const auto& api : b.apis) {
        put32(body, api.api);
        put16(body, uint16_t(api.ioData.size()));
        for (const auto& o : api.ioData) {
            put16(body, o.slot); put16(body, o.subslot); put16(body, o.frameOffset);
        }
        put16(body, uint16_t(api.iocs.size()));
        for (const auto& o : api.iocs) {
            put16(body, o.slot); put16(body, o.subslot); put16(body, o.frameOffset);
        }
    }
    pnioBlockHeader(out, kBlkIocrBlockReq, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

/* --- ExpectedSubmoduleBlockReq --- */
static void encDataDesc(std::vector<uint8_t>& body, uint16_t tag,
                        const PnioDataDescription& d) {
    put16(body, tag);
    put16(body, d.dataLength);
    body.push_back(d.lengthIocs);
    body.push_back(d.lengthIops);
}
void pnioEncExpectedSubmoduleBlockReq(std::vector<uint8_t>& out,
                                      const PnioExpectedSubmoduleBlockReq& b) {
    std::vector<uint8_t> body;
    put16(body, uint16_t(b.apis.size()));
    for (const auto& api : b.apis) {
        put32(body, api.api);
        put16(body, api.slot);
        put32(body, api.moduleIdent);
        put16(body, api.moduleProperties);
        put16(body, uint16_t(api.submodules.size()));
        for (const auto& sm : api.submodules) {
            put16(body, sm.subslot);
            put32(body, sm.submoduleIdent);
            put16(body, sm.properties);
            const uint16_t t = sm.properties & 0x3;
            /* type 0/1 → Input desc; 2 → Output; 3 → both (matches capture) */
            if (t == 0 || t == 1 || t == 3) {
                encDataDesc(body, 1, sm.input.value_or(PnioDataDescription{}));
            }
            if (t == 2 || t == 3) {
                encDataDesc(body, 2, sm.output.value_or(PnioDataDescription{}));
            }
        }
    }
    pnioBlockHeader(out, kBlkExpectedSubmoduleBlockReq, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

/* --- AlarmCRBlockReq --- */
void pnioEncAlarmCrBlockReq(std::vector<uint8_t>& out,
                            const PnioAlarmCrBlockReq& b) {
    std::vector<uint8_t> body;
    put16(body, b.alarmCrType);
    put16(body, b.lt);
    put32(body, b.properties);
    put16(body, b.rtaTimeoutFactor);
    put16(body, b.rtaRetries);
    put16(body, b.localAlarmReference);
    put16(body, b.maxAlarmDataLength);
    put16(body, b.tagHeaderHigh);
    put16(body, b.tagHeaderLow);
    pnioBlockHeader(out, kBlkAlarmCrBlockReq, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

/* --- Connect response decode --- */
bool pnioDecConnectRes(const uint8_t* blocks, size_t len, PnioConnectRes& r) {
    const uint8_t* p = blocks;
    size_t left = len;
    uint16_t type; const uint8_t* body; size_t blen;
    bool gotAr = false, gotAlarm = false;
    while (blockHead(p, left, type, body, blen)) {
        PnioCursor c{body, blen, 0};
        switch (type) {
        case kBlkArBlockRes: {
            uint16_t arType, port;
            if (!c.u16(arType) || !c.uuid(r.arUuid) || !c.u16(r.sessionKey) ||
                !c.mac(r.responderMac) || !c.u16(port)) return false;
            r.responderUdpRtPort = port;
            gotAr = true;
            break;
        }
        case kBlkIocrBlockRes: {
            PnioConnectRes::IocrRes ir;
            if (!c.u16(ir.iocrType) || !c.u16(ir.reference) || !c.u16(ir.frameId))
                return false;
            r.iocrs.push_back(ir);
            break;
        }
        case kBlkAlarmCrBlockRes: {
            uint16_t t, ref;
            if (!c.u16(t) || !c.u16(ref) || !c.u16(r.alarmMaxDataLen)) return false;
            r.alarmLocalRef = ref;
            gotAlarm = true;
            break;
        }
        case kBlkArServerBlockRes: {
            uint16_t nl;
            const uint8_t* nm;
            if (!c.u16(nl) || !c.bytes(nl, nm)) return false;
            r.stationName.assign(reinterpret_cast<const char*>(nm), nl);
            break;
        }
        default:
            /* ModuleDiffBlock or unknown — skip (kept raw-parse tolerant) */
            break;
        }
    }
    return gotAr && gotAlarm;
}

/* --- IODWriteReqHeader record --- */
void pnioEncIodWriteReq(std::vector<uint8_t>& out, const PnioWriteRecord& r) {
    std::vector<uint8_t> body;
    put16(body, r.seq);
    putBytes(body, r.arUuid.data(), 16);
    put32(body, r.api);
    put16(body, r.slot);
    put16(body, r.subslot);
    put16(body, 0);                    /* pad */
    put16(body, r.index);
    put32(body, uint32_t(r.data.size()));
    putBytes(body, nullptr, 0);
    /* 24-byte reserved field */
    for (int i = 0; i < 24; ++i) body.push_back(0);
    pnioBlockHeader(out, kBlkIodWriteReqHeader, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
    out.insert(out.end(), r.data.begin(), r.data.end());
    /* records are 4-byte aligned: pad data to next multiple of 4 within
     * the stream (the block length does not count data). */
    while (out.size() % 4) out.push_back(0);
}

void pnioEncWriteBlocks(std::vector<uint8_t>& out,
                        const std::vector<PnioWriteRecord>& inner) {
    if (inner.size() == 1) {
        pnioEncIodWriteReq(out, inner[0]);
        return;
    }
    /* MultipleWrite container: outer record index 0xe040, api/slot/subslot
     * all 0xffffffff/0xffff, data = nested 4-aligned records. */
    std::vector<uint8_t> nested;
    for (const auto& r : inner) {
        /* encode each into a temp then strip trailing alignment? No —
         * pnioEncIodWriteReq already aligns; nested records keep their
         * own alignment padding. */
        std::vector<uint8_t> tmp;
        pnioEncIodWriteReq(tmp, r);
        nested.insert(nested.end(), tmp.begin(), tmp.end());
    }
    PnioWriteRecord outer;
    outer.seq = inner.empty() ? 0 : inner[0].seq;
    outer.arUuid = inner.empty() ? std::array<uint8_t,16>{} : inner[0].arUuid;
    outer.api = 0xffffffff;
    outer.slot = 0xffff; outer.subslot = 0xffff;
    outer.index = kPnioIndexMultipleWrite;
    outer.data = nested;
    /* write outer manually WITHOUT trailing alignment pad (container size
     * is exact; alignment padding inside is already in nested data) */
    std::vector<uint8_t> body;
    put16(body, outer.seq);
    putBytes(body, outer.arUuid.data(), 16);
    put32(body, outer.api);
    put16(body, outer.slot); put16(body, outer.subslot);
    put16(body, 0);
    put16(body, outer.index);
    put32(body, uint32_t(outer.data.size()));
    for (int i = 0; i < 24; ++i) body.push_back(0);
    pnioBlockHeader(out, kBlkIodWriteReqHeader, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
    out.insert(out.end(), outer.data.begin(), outer.data.end());
}

bool pnioDecWriteRes(const uint8_t* blocks, size_t len,
                     std::vector<PnioWriteResRecord>& out) {
    const uint8_t* p = blocks;
    size_t left = len;
    uint16_t type; const uint8_t* body; size_t blen;
    while (blockHead(p, left, type, body, blen)) {
        if (type != kBlkIodWriteResHeader) return false;
        PnioCursor c{body, blen, 0};
        PnioWriteResRecord r;
        uint16_t pad;
        uint32_t rdl, av1, av2;
        if (!c.u16(r.seq) || !c.uuid(r.arUuid) || !c.u32(r.api) ||
            !c.u16(r.slot) || !c.u16(r.subslot) || !c.u16(pad) ||
            !c.u16(r.index) || !c.u32(rdl) || !c.u32(av1) || !c.u32(av2) ||
            !c.u32(r.status)) return false;
        out.push_back(r);
    }
    return !out.empty();
}

/* --- Control blocks --- */
void pnioEncControlBlock(std::vector<uint8_t>& out, const PnioControlBlock& b) {
    std::vector<uint8_t> body;
    put16(body, 0);                    /* reserved */
    putBytes(body, b.arUuid.data(), 16);
    put16(body, b.sessionKey);
    put16(body, 0);                    /* reserved */
    put16(body, b.command);
    put16(body, b.properties);
    pnioBlockHeader(out, b.blockType, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

bool pnioDecControlBlock(const uint8_t* blocks, size_t len, PnioControlBlock& b) {
    const uint8_t* p = blocks;
    size_t left = len;
    uint16_t type; const uint8_t* body; size_t blen;
    if (!blockHead(p, left, type, body, blen)) return false;
    switch (type) {
    case kBlkIodControlResPrmEnd: case kBlkIoxBlockResAppReady:
    case kBlkReleaseBlockRes: case kBlkIodControlReqPrmEnd:
    case kBlkIoxBlockReqAppReady: case kBlkReleaseBlockReq:
        break;
    default: return false;
    }
    PnioCursor c{body, blen, 0};
    uint16_t r1, r2;
    if (!c.u16(r1) || !c.uuid(b.arUuid) || !c.u16(b.sessionKey) ||
        !c.u16(r2) || !c.u16(b.command) || !c.u16(b.properties)) return false;
    b.blockType = type;
    return true;
}

PnioControlBlock pnioPrmEndReq(const std::array<uint8_t,16>& ar, uint16_t key) {
    PnioControlBlock b; b.blockType = kBlkIodControlReqPrmEnd;
    b.arUuid = ar; b.sessionKey = key; b.command = kCmdPrmEnd; return b;
}
PnioControlBlock pnioPrmEndRes(const PnioControlBlock& req) {
    PnioControlBlock b; b.blockType = kBlkIodControlResPrmEnd;
    b.arUuid = req.arUuid; b.sessionKey = req.sessionKey;
    b.command = kCmdDone; return b;
}
PnioControlBlock pnioAppReadyReq(const std::array<uint8_t,16>& ar, uint16_t key) {
    PnioControlBlock b; b.blockType = kBlkIoxBlockReqAppReady;
    b.arUuid = ar; b.sessionKey = key; b.command = kCmdAppReady; return b;
}
PnioControlBlock pnioAppReadyRes(const PnioControlBlock& req) {
    PnioControlBlock b; b.blockType = kBlkIoxBlockResAppReady;
    b.arUuid = req.arUuid; b.sessionKey = req.sessionKey;
    b.command = kCmdDone; return b;
}
PnioControlBlock pnioReleaseReq(const std::array<uint8_t,16>& ar, uint16_t key) {
    PnioControlBlock b; b.blockType = kBlkReleaseBlockReq;
    b.arUuid = ar; b.sessionKey = key; b.command = kCmdRelease; return b;
}
PnioControlBlock pnioReleaseRes(const PnioControlBlock& req) {
    PnioControlBlock b; b.blockType = kBlkReleaseBlockRes;
    b.arUuid = req.arUuid; b.sessionKey = req.sessionKey;
    b.command = kCmdDone; return b;
}

/* --- F-Parameters block --- */
uint8_t pnioFBlockId(const PnioFParameters& f) {
    /* derive from which optional fields are present */
    bool wd2 = f.watchdogTime2.has_value();
    bool ipar = f.iParCrc.has_value();
    return uint8_t((wd2 ? 2 : 0) | (ipar ? 1 : 0));
}
void pnioEncFParameters(std::vector<uint8_t>& out, const PnioFParameters& f) {
    uint8_t flag2 = uint8_t(((f.flag2 >> 3) & 0x38) | (pnioFBlockId(f) & 0x7));
    /* preserve version/passivation bits from f.flag2, set block id */
    flag2 = uint8_t((f.flag2 & 0xC7) | ((pnioFBlockId(f) & 0x7) << 3));
    out.push_back(f.flag1);
    out.push_back(flag2);
    put16(out, f.sourceAdd);
    put16(out, f.destAdd);
    put16(out, uint16_t(f.watchdogTime));
    if (f.watchdogTime2) put16(out, uint16_t(*f.watchdogTime2));
    if (f.iParCrc) put32(out, *f.iParCrc);
    put16(out, f.parCrc);
}
bool pnioDecFParameters(const uint8_t* data, size_t len, PnioFParameters& f) {
    if (len < 10) return false;
    PnioCursor c{data, len, 0};
    uint8_t f1, f2; uint16_t sa, da, wd, crc;
    int16_t w;
    if (!c.u8(f1) || !c.u8(f2) || !c.u16(sa) || !c.u16(da) || !c.i16(w)) return false;
    f.flag1 = f1; f.flag2 = f2;
    f.sourceAdd = sa; f.destAdd = da; f.watchdogTime = w;
    const uint8_t blkId = (f2 >> 3) & 0x7;
    if (blkId & 0x2) { /* WD_Time2 present */
        int16_t w2; if (!c.i16(w2)) return false;
        f.watchdogTime2 = w2;
    }
    if (blkId & 0x1) { /* iPar_CRC present */
        uint32_t ic; if (!c.u32(ic)) return false;
        f.iParCrc = ic;
    }
    if (!c.u16(crc)) return false;
    f.parCrc = crc;
    return true;
}

/* ================= IODRead records ================= */

void pnioEncIodReadReq(std::vector<uint8_t>& out, const PnioReadRecord& r) {
    std::vector<uint8_t> body;
    put16(body, r.seq);
    putBytes(body, r.arUuid.data(), 16);
    put32(body, r.api);
    put16(body, r.slot);
    put16(body, r.subslot);
    put16(body, 0);                    /* pad */
    put16(body, r.index);
    put32(body, r.recordDataLength);
    putBytes(body, r.targetArUuid.data(), 16);
    for (int i = 0; i < 8; ++i) body.push_back(0); /* pad */
    pnioBlockHeader(out, kBlkIodReadReqHeader, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

bool pnioDecReadRes(const uint8_t* blocks, size_t len,
                    std::vector<PnioReadResRecord>& out) {
    const uint8_t* p = blocks;
    size_t left = len;
    uint16_t type; const uint8_t* body; size_t blen;
    while (blockHead(p, left, type, body, blen)) {
        if (type != kBlkIodReadResHeader) break; /* record data trails as
            its own block (e.g. I&M0 type 0x0020) — not a res header */
        PnioCursor c{body, blen, 0};
        PnioReadResRecord r;
        uint16_t pad, av1, av2;
        uint32_t rdl;
        if (!c.u16(r.req.seq) || !c.uuid(r.req.arUuid) || !c.u32(r.req.api) ||
            !c.u16(r.req.slot) || !c.u16(r.req.subslot) || !c.u16(pad) ||
            !c.u16(r.req.index) || !c.u32(rdl) ||
            !c.u16(av1) || !c.u16(av2))
            return false;
        r.additionalValue1 = av1; r.additionalValue2 = av2;
        /* record data trails the 58-byte fixed body in the block stream
         * (its own block header included), length = record_data_length */
        if (rdl && left >= rdl) {
            r.data.assign(p, p + rdl);
            p += rdl; left -= rdl;
        } else if (left) {
            r.data.assign(p, p + left);
            left = 0;
        }
        out.push_back(std::move(r));
    }
    return !out.empty();
}

/* ================= I&M records ================= */

static void pushPadded(std::vector<uint8_t>& o, const std::string& s,
                       size_t len) {
    const size_t n = std::min(s.size(), len);
    o.insert(o.end(), s.begin(), s.begin() + n);
    for (size_t i = n; i < len; ++i) o.push_back(' ');
}
static std::string trimPadded(const uint8_t* p, size_t n) {
    while (n && (p[n-1] == ' ' || p[n-1] == '\0')) --n;
    return std::string(reinterpret_cast<const char*>(p), n);
}

void pnioEncIm0(std::vector<uint8_t>& out, const PnioIm0& im) {
    std::vector<uint8_t> body;
    put16(body, im.vendorId);
    pushPadded(body, im.orderId, 20);
    pushPadded(body, im.serialNumber, 16);
    put16(body, im.hardwareRevision);
    body.push_back(uint8_t(im.swPrefix));
    body.push_back(im.swFunctional);
    body.push_back(im.swBugFix);
    body.push_back(im.swInternal);
    put16(body, im.revisionCounter);
    put16(body, im.profileId);
    put16(body, im.profileSpecificType);
    body.push_back(im.imVersionMajor);
    body.push_back(im.imVersionMinor);
    put16(body, im.supported);
    pnioBlockHeader(out, 0x0020, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

bool pnioDecIm0(const uint8_t* rec, size_t len, PnioIm0View& v) {
    /* rec = full record incl. 6-byte block header */
    if (len < 60) return false;
    const uint8_t* b = rec + 6;
    PnioCursor c{b, 54, 0};
    uint8_t maj, minr;
    if (!c.u16(v.vendorId)) return false;
    const uint8_t* t;
    if (!c.bytes(20, t)) return false;
    v.orderId = trimPadded(t, 20);
    if (!c.bytes(16, t)) return false;
    v.serialNumber = trimPadded(t, 16);
    if (!c.u16(v.hardwareRevision)) return false;
    uint8_t pf, f, bf, inf;
    if (!c.u8(pf) || !c.u8(f) || !c.u8(bf) || !c.u8(inf)) return false;
    v.swPrefix = char(pf); v.swFunctional = f; v.swBugFix = bf; v.swInternal = inf;
    if (!c.u16(v.revisionCounter) || !c.u16(v.profileId) ||
        !c.u16(v.profileSpecificType)) return false;
    if (!c.u8(maj) || !c.u8(minr) || !c.u16(v.supported)) return false;
    return true;
}

void pnioEncImRecord(std::vector<uint8_t>& out, uint16_t index,
                     const PnioImText& t) {
    if (index != kPnioIndexIm1 && index != kPnioIndexIm2 &&
        index != kPnioIndexIm3) return;
    const uint16_t btype = index == kPnioIndexIm1 ? 0x0021
                         : index == kPnioIndexIm2 ? 0x0022 : 0x0023;
    const size_t bodyLen = index == kPnioIndexIm2 ? kIm2Len : kIm1Len;
    std::vector<uint8_t> body(bodyLen, ' ');
    auto put = [&](size_t off, size_t cap, const std::string& s) {
        const size_t n = std::min(s.size(), cap);
        if (off + n <= bodyLen) std::memcpy(body.data() + off, s.data(), n);
    };
    if (index == kPnioIndexIm1) { put(0, 32, t.a); put(32, 22, t.b); }
    else put(0, bodyLen, t.a);
    pnioBlockHeader(out, btype, uint16_t(body.size()));
    out.insert(out.end(), body.begin(), body.end());
}

bool pnioDecImRecord(const uint8_t* rec, size_t len, uint16_t expectedIndex,
                     PnioImText& t) {
    if (len < 6) return false;
    const uint16_t type = uint16_t(rec[0] << 8) | rec[1];
    const uint16_t blen = uint16_t(rec[2] << 8) | rec[3];
    const uint16_t want = expectedIndex == kPnioIndexIm1 ? 0x0021
                           : expectedIndex == kPnioIndexIm2 ? 0x0022
                           : expectedIndex == kPnioIndexIm3 ? 0x0023 : 0;
    if (!want || type != want || len < 6 + blen - 2) return false;
    const uint8_t* b = rec + 6;
    const size_t bl = blen - 2;
    if (expectedIndex == kPnioIndexIm2) {
        t.a = trimPadded(b, bl); t.b.clear();
        return true;
    }
    if (expectedIndex == kPnioIndexIm3) {
        t.a = trimPadded(b, bl); t.b.clear();
        return true;
    }
    if (bl >= 32) {
        t.a = trimPadded(b, 32);
        t.b = trimPadded(b + 32, bl - 32 > 22 ? 22 : bl - 32);
        return true;
    }
    t.a = trimPadded(b, bl); t.b.clear();
    return true;
}

/* ================= AR client session ================= */

bool PnioArClient::open(DcRpcLink& link, const std::string& host, uint16_t port) {
    if (!link.open(host, port)) return false;
    link_ = &link;
    return true;
}
void PnioArClient::close() {
    if (link_) link_->close();
    link_ = nullptr;
    state_ = PnioArState::Idle;
}
bool PnioArClient::isOpen() const { return link_ && link_->isOpen(); }

bool PnioArClient::rpcCall(PnioOpnum opnum, const std::vector<uint8_t>& blocks,
                           std::vector<uint8_t>& resBlocks, uint32_t& status,
                           int timeoutMs) {
    if (!link_) return false;
    PnioRpcHeader base;
    base.object = cfg_.deviceObjectUuid;
    base.interface = kPnioDeviceIfaceUuid;
    base.activity = cfg_.activityUuid;
    base.seqNum = nextSeq();
    base.serverBoot = 0;
    base.ifVersion = 1;
    base.ihint = 0xffff; base.ahint = 0xffff;
    base.serialHi = 0; base.serialLo = 0;

    /* golden captures show controller requests carry args_max ==
     * payload length; cfg_.argsMax overrides when nonzero */
    const uint32_t amax = cfg_.argsMax ? cfg_.argsMax
                                     : uint32_t(blocks.size());
    auto pdu = pnioBuildRequest(base, opnum, blocks, amax);
    /* patch frag_len already set inside; send raw */
    if (!link_->send(pdu.data(), pdu.size())) return false;

    /* receive until a Response with matching seq_num+opnum arrives */
    const uint32_t wantSeq = base.seqNum;
    const uint16_t wantOp = uint16_t(opnum);
    int waited = 0;
    while (waited < timeoutMs) {
        std::vector<uint8_t> buf;
        int slice = std::min(200, timeoutMs - waited);
        if (!link_->recvDatagram(buf, slice)) { waited += slice; continue; }
        waited = 0;
        PnioRpcHeader rh;
        if (!pnioDecRpcHeader(buf.data(), buf.size(), rh)) continue;
        if (rh.ptype != PnioPtype::Response) continue;
        if (rh.seqNum != wantSeq || rh.opnum != wantOp) continue;
        if (rh.interface != kPnioDeviceIfaceUuid) continue;
        PnioNdrResponse nr;
        if (!pnioDecNdrResponse(buf.data() + kPnioRpcHdrLen,
                                buf.size() - kPnioRpcHdrLen, rh.drep, nr))
            continue;
        status = nr.status;
        if (nr.blocksLen)
            resBlocks.assign(nr.blocks, nr.blocks + nr.blocksLen);
        else
            resBlocks.clear();
        lastStatus_ = status;
        lastRes_ = resBlocks;
        return status == 0;
    }
    return false;
}

bool PnioArClient::connect(const PnioArConfig& cfg, int timeoutMs) {
    cfg_ = cfg;
    haveCfg_ = true;
    state_ = PnioArState::Idle;

    std::vector<uint8_t> blocks;
    pnioEncArBlockReq(blocks, cfg.ar);
    for (const auto& io : cfg.iocrs) pnioEncIocrBlockReq(blocks, io);
    for (const auto& ex : cfg.expected) pnioEncExpectedSubmoduleBlockReq(blocks, ex);
    pnioEncAlarmCrBlockReq(blocks, cfg.alarmCr);

    std::vector<uint8_t> resBlocks;
    uint32_t status = 0;
    if (!rpcCall(PnioOpnum::Connect, blocks, resBlocks, status, timeoutMs))
        return false;
    if (status != 0) { lastStatus_ = status; return false; }
    if (!pnioDecConnectRes(resBlocks.data(), resBlocks.size(), res_))
        return false;
    /* sanity: response AR UUID must match what we sent */
    if (res_.arUuid != cfg.ar.arUuid) return false;
    state_ = PnioArState::Connected;
    return true;
}

bool PnioArClient::connectRaw(const std::array<uint8_t,16>& devObj,
                              const std::array<uint8_t,16>& actUuid,
                              const std::array<uint8_t,16>& arUuid,
                              const uint8_t* blocks, size_t len,
                              int timeoutMs) {
    cfg_ = PnioArConfig{};
    cfg_.deviceObjectUuid = devObj;
    cfg_.activityUuid = actUuid;
    cfg_.ar.arUuid = arUuid;
    haveCfg_ = true;
    state_ = PnioArState::Idle;

    std::vector<uint8_t> req(blocks, blocks + len);
    std::vector<uint8_t> resBlocks;
    uint32_t status = 0;
    if (!rpcCall(PnioOpnum::Connect, req, resBlocks, status, timeoutMs))
        return false;
    if (status != 0) { lastStatus_ = status; return false; }
    if (!pnioDecConnectRes(resBlocks.data(), resBlocks.size(), res_))
        return false;
    if (res_.arUuid != arUuid) return false;
    state_ = PnioArState::Connected;
    return true;
}

bool PnioArClient::writeRecords(const std::vector<PnioWriteRecord>& records,
                                int timeoutMs) {
    if (state_ != PnioArState::Connected &&
        state_ != PnioArState::PrmEndSent)
        return false;
    std::vector<uint8_t> blocks;
    pnioEncWriteBlocks(blocks, records);
    std::vector<uint8_t> resBlocks;
    uint32_t status = 0;
    if (!rpcCall(PnioOpnum::Write, blocks, resBlocks, status, timeoutMs))
        return false;
    if (status != 0) return false;
    std::vector<PnioWriteResRecord> wr;
    return pnioDecWriteRes(resBlocks.data(), resBlocks.size(), wr);
}

bool PnioArClient::readRecord(uint16_t index, uint16_t slot, uint16_t subslot,
                              bool implicit, std::vector<uint8_t>& record,
                              int timeoutMs) {
    if (state_ == PnioArState::Idle) return false;
    PnioReadRecord rr;
    rr.seq = 0;
    rr.arUuid = cfg_.ar.arUuid;
    rr.api = 0;
    rr.slot = slot; rr.subslot = subslot;
    rr.index = index;
    rr.recordDataLength = 0x1000;
    rr.targetArUuid = cfg_.ar.arUuid;
    std::vector<uint8_t> blocks;
    pnioEncIodReadReq(blocks, rr);
    std::vector<uint8_t> resBlocks;
    uint32_t status = 0;
    const auto op = implicit ? PnioOpnum::ReadImplicit : PnioOpnum::Read;
    if (!rpcCall(op, blocks, resBlocks, status, timeoutMs)) return false;
    if (status != 0) return false;
    std::vector<PnioReadResRecord> rr2;
    if (!pnioDecReadRes(resBlocks.data(), resBlocks.size(), rr2) ||
        rr2.empty()) return false;
    record = rr2[0].data;
    return !record.empty();
}

bool PnioArClient::prmEnd(int timeoutMs) {
    if (state_ != PnioArState::Connected) return false;
    std::vector<uint8_t> blocks;
    pnioEncControlBlock(blocks, pnioPrmEndReq(cfg_.ar.arUuid, res_.sessionKey));
    std::vector<uint8_t> resBlocks;
    uint32_t status = 0;
    if (!rpcCall(PnioOpnum::Control, blocks, resBlocks, status, timeoutMs))
        return false;
    if (status != 0) return false;
    PnioControlBlock rb;
    if (!pnioDecControlBlock(resBlocks.data(), resBlocks.size(), rb)) return false;
    if (rb.blockType != kBlkIodControlResPrmEnd ||
        (rb.command & kCmdDone) == 0) return false;
    state_ = PnioArState::PrmEndSent;
    return true;
}

bool PnioArClient::release(int timeoutMs) {
    if (state_ == PnioArState::Idle || state_ == PnioArState::Released)
        return true;
    std::vector<uint8_t> blocks;
    pnioEncControlBlock(blocks,
                        pnioReleaseReq(cfg_.ar.arUuid, res_.sessionKey));
    std::vector<uint8_t> resBlocks;
    uint32_t status = 0;
    if (!rpcCall(PnioOpnum::Release, blocks, resBlocks, status, timeoutMs))
        return false;
    if (status != 0) return false;
    PnioControlBlock rb;
    if (!pnioDecControlBlock(resBlocks.data(), resBlocks.size(), rb)) return false;
    if (rb.blockType != kBlkReleaseBlockRes || (rb.command & kCmdDone) == 0)
        return false;
    state_ = PnioArState::Released;
    return true;
}

bool PnioArClient::service(int timeoutMs) {
    if (!link_) return false;
    std::vector<uint8_t> buf;
    if (!link_->recvDatagram(buf, timeoutMs)) return false;
    PnioRpcHeader h;
    if (!pnioDecRpcHeader(buf.data(), buf.size(), h)) return false;
    /* device→controller requests use the CONTROLLER interface, BE drep.
     * The NDR body is a *request* array header (no status word):
     * [argsMax,argsLen,maxCount,offset,actualCount]. */
    if (h.ptype != PnioPtype::Request ||
        h.interface != kPnioControllerIfaceUuid) return false;
    const uint8_t* nb = buf.data() + kPnioRpcHdrLen;
    const size_t nlen = buf.size() - kPnioRpcHdrLen;
    if (nlen < kPnioNdrLen) return false;
    const bool le = h.drep.little;
    auto g32 = [&](size_t o) { return le ? uint32_t(nb[o]) |
               (uint32_t(nb[o+1])<<8) | (uint32_t(nb[o+2])<<16) |
               (uint32_t(nb[o+3])<<24)
             : (uint32_t(nb[o])<<24) | (uint32_t(nb[o+1])<<16) |
               (uint32_t(nb[o+2])<<8) | uint32_t(nb[o+3]); };
    const uint32_t reqArgsMax = g32(0);
    const uint32_t argsLen = g32(4), offset = g32(12), actual = g32(16);
    if (argsLen != actual || offset != 0 ||
        kPnioNdrLen + actual > nlen || actual == 0) return false;
    const uint8_t* blocks = nb + kPnioNdrLen;
    const size_t blocksLen = actual;
    PnioControlBlock cb;
    if (!pnioDecControlBlock(blocks, blocksLen, cb)) return false;
    if (cb.blockType != kBlkIoxBlockReqAppReady ||
        (cb.command & kCmdAppReady) == 0) return false;
    if (cb.arUuid != cfg_.ar.arUuid) return false;

    /* answer ApplicationReady Done: response ptype, flags 0x0a
     * (LAST_FRAG|NO_FACK — matches controller capture), LE drep,
     * interface = device iface, object = controller object uuid */
    std::vector<uint8_t> resBlocks;
    pnioEncControlBlock(resBlocks, pnioAppReadyRes(cb));

    PnioRpcHeader rh;
    rh.ptype = PnioPtype::Response;
    rh.flags1 = kPnioFlagLastFrag | kPnioFlagNoFack; /* 0x0a */
    rh.drep = PnioDrep{true};
    rh.object = cfg_.deviceObjectUuid;
    rh.interface = kPnioDeviceIfaceUuid;
    rh.activity = cfg_.activityUuid;
    rh.seqNum = h.seqNum;
    rh.opnum = h.opnum;
    rh.serverBoot = 0; rh.ifVersion = 1;
    rh.ihint = 0xffff; rh.ahint = 0xffff;
    rh.fragLen = uint16_t(kPnioNdrLen + resBlocks.size());

    std::vector<uint8_t> out;
    out.reserve(kPnioRpcHdrLen + kPnioNdrLen + resBlocks.size());
    uint8_t hdr[kPnioRpcHdrLen];
    pnioEncRpcHeader(hdr, rh);
    out.insert(out.end(), hdr, hdr + kPnioRpcHdrLen);
    uint8_t ndr[kPnioNdrLen];
    pnioEncNdrResponse(ndr, 0, uint32_t(resBlocks.size()), reqArgsMax,
                       rh.drep);
    out.insert(out.end(), ndr, ndr + kPnioNdrLen);
    out.insert(out.end(), resBlocks.begin(), resBlocks.end());
    if (!link_->send(out.data(), out.size())) return false;
    state_ = PnioArState::DataExchange;
    return true;
}

} // namespace tplc::host
