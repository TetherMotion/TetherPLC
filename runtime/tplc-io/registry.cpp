#include "tplc_io_internal.hpp"

namespace tplcio_int {

void dbgReadBytes(uint64_t off, uint32_t len, void* dst, size_t cap) {
    W w; w.u64(off); w.u32(len);
    std::vector<uint8_t> out;
    tplcio::CtlClient::Error e;
    if (!g_ctl.call(TPLC_OP_DBG_READ, w.b.data(),
                    (uint32_t)w.b.size(), out, &e))
        return;
    memcpy(dst, out.data(), out.size() < cap ? out.size() : cap);
}

/* Emit one catalog leaf at an absolute blob offset. flags carries the
   LAYD field bits (bit0 retain, bit1 const) — retain/const leaves are
   always read-only signals; scalar leaves are writable params.     */
void emitPlcLeaf(Registry& registry, const std::string& name,
                 const std::string& group, uint64_t off, uint64_t size,
                 uint8_t flags, ValueType vt, const std::string& desc) {
    bool retain = (flags & 1) != 0;
    bool isConst = (flags & 2) != 0;
    uint32_t sb = scalarBytes(vt);
    bool scalar = sb != 0 && sb == (uint32_t)size;
    if (scalar && !retain && !isConst) {
        ParamEntry p;
        p.id = g_sigId++;
        p.name = name;
        p.description = desc;
        p.group = group;
        p.valueType = vt;
        p.readFn = [off, size](void* dst) {
            dbgReadBytes(off, (uint32_t)size, dst, size);
        };
        p.writeFn = [off, size](const void* src) {
            W w; w.u64(off); w.u32((uint32_t)size); w.bytes(src, size);
            std::vector<uint8_t> out;
            tplcio::CtlClient::Error e;
            g_ctl.call(TPLC_OP_DBG_WRITE,
                       w.b.data(), (uint32_t)w.b.size(), out, &e);
        };
        assignSchema(p);
        registry.addParam(std::move(p));
        return;
    }
    if (scalar) {
        SignalEntry s;
        s.id = g_sigId++;
        s.name = name;
        s.description = desc;
        s.group = group;
        s.valueType = vt;
        s.readFn = [off, size](void* dst) {
            dbgReadBytes(off, (uint32_t)size, dst, size);
        };
        assignSchema(s);
        registry.addSignal(std::move(s));
        return;
    }
    /* non-scalar → Binary signal carrying the raw field bytes,
       schema-encoded like the whole-blob signal              */
    SignalEntry b;
    b.id = g_sigId++;
    b.name = name;
    b.description = desc;
    b.group = group;
    b.valueType = ValueType::Binary;
    b.maxValueSize = (uint16_t)(size + 10 > 0xFFFF
                                    ? 0xFFFF : size + 10);
    b.varReadFn = [off, size](void* dst, size_t maxLen) -> size_t {
        uint32_t want = (uint32_t)size;
        if (maxLen > 10 && want > maxLen - 10) want = (uint32_t)(maxLen - 10);
        W w; w.u64(off); w.u32(want);
        std::vector<uint8_t> out;
        tplcio::CtlClient::Error e;
        if (!g_ctl.call(TPLC_OP_DBG_READ, w.b.data(),
                        (uint32_t)w.b.size(), out, &e))
            return 0;
        const SchemaNode* n = nodeFor(ValueType::Binary);
        auto enc = encodeNative(g_graph, *n,
            nv::bytes(std::span<const uint8_t>(out)));
        if (!enc || enc->size() > maxLen) return 0;
        memcpy(dst, enc->data(), enc->size());
        return enc->size();
    };
    assignSchema(b);
    registry.addSignal(std::move(b));
}

/* Catalog entry for one field or member leaf. STRUCT/FUNCTION_BLOCK
   types recurse: the aggregate stays visible as a raw Binary signal
   and each member publishes plc.<prog>.<name>.<member> (matching the
   WebUI pinKeys contract for FB instance pins like TON.ET). Depth is
   capped like tplc-mir migrate.rs leaf expansion.                 */
void emitPlcNode(Registry& registry, const std::string& name,
                 const std::string& group, uint64_t off, uint64_t size,
                 uint32_t tref, uint8_t flags,
                 const std::vector<TypcType>& types, int depth) {
    const TypcType* tt = (tref != kNoIdx && tref < types.size())
                             ? &types[tref] : nullptr;
    if (tt && (tt->tag == kTagStruct || tt->tag == kTagFb) &&
        !tt->members.empty() && depth < 8) {
        emitPlcLeaf(registry, name, group, off, size, flags,
                    ValueType::Binary,
                    tt->tag == kTagFb ? "fb instance" : "struct");
        for (const auto& m : tt->members) {
            if (m.name.empty() || m.type >= types.size()) continue;
            uint64_t moff = off + m.off;
            uint64_t msize = types[m.type].size;
            if (msize == 0 || msize > (1u << 20) ||
                moff + msize > (64ull << 20))
                continue;
            emitPlcNode(registry, name + "." + m.name, group, moff,
                        msize, m.type, flags, types, depth + 1);
        }
        return;
    }
    uint8_t tag = tt ? tt->tag : 0xFF;
    ValueType vt = tag == 0xFF ? ValueType::Binary : vtOfTag(tag);
    bool isConst = (flags & 2) != 0;
    emitPlcLeaf(registry, name, group, off, size, flags, vt,
                isConst ? "const field" : "var");
}

void addPlcFieldEntries(Registry& registry, const std::string& prog,
                        const std::vector<PlcField>& fields,
                        const std::vector<TypcType>& types) {
    std::string base = "plc." + prog;
    std::string pfx = prog + ".";
    for (const auto& f : fields) {
        if (f.name.empty() || f.size == 0 || f.size > (1u << 20))
            continue;
        /* multi-POU layouts qualify field names (POU.var); the catalog
           contract is plc.<prog>.<var> — strip the program prefix  */
        std::string rel = f.name;
        if (rel.rfind(pfx, 0) == 0) rel.erase(0, pfx.size());
        if (rel.empty()) continue;
        bool retain = (f.flags & 1) != 0;
        std::string name = base + (retain ? ".retain." : ".") + rel;
        emitPlcNode(registry, name, base, f.off, f.size, f.tref,
                    f.flags, types, 0);
    }
}


} // namespace tplcio_int
