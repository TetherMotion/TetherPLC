#include "tplc_io_internal.hpp"

namespace tplcio_int {

uint64_t nowUs() {
    timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000000ull + (uint64_t)ts.tv_nsec / 1000;
}
SchemaGraph   g_graph;
SchemaCatalog g_catalog;

SchemaKey keyFor(ValueType t) {
    SchemaKey k{};
    k[0] = 0x50;                       /* 'P' — plc namespace        */
    k[1] = (uint8_t)t;
    return k;
}

const SchemaNode* nodeFor(ValueType t) {
    return g_graph.find(keyFor(t));
}

/* ---- ctl call → FunctionCallResult ------------------------------ */
tplcio::CtlClient g_ctl;

FunctionCallResult ferr(const tplcio::CtlClient::Error& e) {
    FunctionCallResult r;
    r.errorMessage = "ctl " + std::to_string(e.code) +
                     (e.msg.empty() ? "" : ": " + e.msg);
    return r;
}

/* returns are schema-wire encoded against the declared node        */
FunctionCallResult retOf(ValueType t, const NativeValue& nv) {
    FunctionCallResult r;
    const SchemaNode* n = nodeFor(t);
    if (!n) { r.errorMessage = "no schema for return"; return r; }
    auto enc = encodeNative(g_graph, *n, nv);
    if (!enc) { r.errorMessage = "encode: " + enc.error(); return r; }
    r.success = true;
    r.returnValue = std::move(*enc);
    return r;
}

FunctionCallResult okStr(const std::string& v) {
    return retOf(ValueType::String, nv::str(v));
}

FunctionCallResult okBin(const std::vector<uint8_t>& v) {
    return retOf(ValueType::Binary,
                 nv::bytes(std::span<const uint8_t>(v)));
}

/* void functions (no declared return) must return truly empty bytes —
   a schema-encoded empty value fails invokeFunctionChecked          */
FunctionCallResult okVoid() {
    FunctionCallResult r;
    r.success = true;
    return r;
}

FunctionCallResult okU32(uint32_t v) {
    return retOf(ValueType::U32, nv::u(v));
}

/* progStatus keeps a short cache (see below); mutating ops must
   invalidate it or a signal read right after e.g. shadow.load can
   see the pre-op state for the cache window                      */
void invalidateStatusCache();

bool ctlCall(uint16_t op, const void* p, uint32_t n,
             std::vector<uint8_t>& out, FunctionCallResult& r) {
    tplcio::CtlClient::Error e;
    if (!g_ctl.call(op, p, n, out, &e)) { r = ferr(e); return false; }
    if (op != TPLC_OP_PROG_STATUS) invalidateStatusCache();
    return true;
}

bool ctlCall(uint16_t op, std::vector<uint8_t>& out,
             FunctionCallResult& r) {
    return ctlCall(op, nullptr, 0, out, r);
}

/* ---- argument helpers ------------------------------------------- */
const FunctionArgument* argAt(const std::vector<FunctionArgument>& a,
                              uint32_t pos) {
    for (const auto& x : a)
        if (x.position == pos && x.provided) return &x;
    return nullptr;
}

/* args arrive schema-wire encoded against the declared node; flat
   clients may still send raw LE — decode via schema first, fall
   back to raw                                                     */
const SchemaNode* argNode(ValueType t) { return nodeFor(t); }

bool decodeArg(const FunctionArgument& x, ValueType t,
               SchemaValue& out) {
    const SchemaNode* n = argNode(t);
    if (!n || x.value.empty()) return false;
    auto v = decodeSchemaValue(g_graph, *n, x.value.data(),
                               x.value.size());
    if (!v) return false;
    out = std::move(*v);
    return true;
}

uint64_t argU(const std::vector<FunctionArgument>& a, uint32_t pos,
              uint64_t dflt) {
    const auto* x = argAt(a, pos);
    if (!x || x->value.empty()) return dflt;
    SchemaValue v;
    if (decodeArg(*x, ValueType::U64, v))
        if (auto u = asU64(v)) return *u;
    uint64_t r = 0;
    memcpy(&r, x->value.data(), x->value.size() < 8 ? x->value.size()
                                                  : 8);
    return r;
}

std::string argStr(const std::vector<FunctionArgument>& a,
                   uint32_t pos) {
    const auto* x = argAt(a, pos);
    if (!x) return {};
    SchemaValue v;
    if (decodeArg(*x, ValueType::String, v))
        if (auto s = asString(v)) return *s;
    return std::string((const char*)x->value.data(), x->value.size());
}

std::vector<uint8_t> argBin(const std::vector<FunctionArgument>& a,
                            uint32_t pos) {
    const auto* x = argAt(a, pos);
    if (!x) return {};
    SchemaValue v;
    if (decodeArg(*x, ValueType::Binary, v))
        if (const auto* b = std::get_if<SchemaValue::Bytes>(&v.payload))
            return b->value;
    return x->value;
}

/* ---- reply decoders (ctl wire → JSON) ---------------------------- */
} // namespace tplcio_int
