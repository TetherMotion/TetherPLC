#include "tplc_io_internal.hpp"

namespace tplcio_int {

static std::mutex g_psMu;
static uint64_t g_psTs = 0;

void invalidateStatusCache() {
    std::lock_guard<std::mutex> lk(g_psMu);
    g_psTs = 0;
}

bool progStatus(TplcCtlProgStatus& ps,
                std::vector<uint8_t>& raw) {
    static TplcCtlProgStatus cached{};
    static std::vector<uint8_t> cached_raw;
    static bool cached_ok = false;
    std::lock_guard<std::mutex> lk(g_psMu);
    uint64_t now = nowUs();
    if (now - g_psTs < 20000 && g_psTs) {
        if (cached_ok) { ps = cached; raw = cached_raw; }
        return cached_ok;
    }
    std::vector<uint8_t> out;
    tplcio::CtlClient::Error e;
    cached_ok = g_ctl.call(TPLC_OP_PROG_STATUS, out, &e) &&
                out.size() >= sizeof(TplcCtlProgStatus);
    g_psTs = now;
    if (!cached_ok) return false;
    memcpy(&cached, out.data(), sizeof cached);
    cached_raw = out;
    ps = cached;
    raw = std::move(out);
    return true;
}

uint64_t g_sigId = 200;

SignalEntry sig(const char* name, const char* desc, ValueType t,
                ReadFn rd) {
    SignalEntry e;
    e.id = g_sigId++;
    e.name = name;
    e.description = desc;
    e.group = "plc";
    e.valueType = t;
    e.readFn = std::move(rd);
    assignSchema(e);
    return e;
}

/* ---- dynamic plc.* field catalog ----------------------------------
 * DBG_FIELDS enumerates the active program's blob fields; each is
 * published under the bridge CatalogExporter naming contract so the
 * WebUI live-value keys resolve:
 *   retain fields → signal plc.<prog>.retain.<name>   (read-only)
 *   const fields  → signal plc.<prog>.<name>          (read-only)
 *   scalar vars   → param  plc.<prog>.<name>          (read+write)
 *   everything else → signal plc.<prog>.<name>        (Binary raw)
 * Image-channel symbols (SMAP kind 10 / located %I/%Q/%M vars) are
 * not LAYD fields, so the plc.image.*.ch.* contract stays unserved —
 * a later image-region reader can add it without disturbing these.
 *
 * Scalar typing needs the package TYPC (field type_ref → tag →
 * ValueType). The runtime does not serve TYPC over ctl, so tplc-io
 * reads the deployed .tplc itself: --packages DIR is scanned for a
 * package whose header uuid matches the running program, --pkg FILE
 * covers boot-time --package loads outside that dir. Without a
 * matching package every field degrades to a Binary signal.        */

std::vector<PlcField> fetchPlcFields() {
    std::vector<PlcField> out;
    std::vector<uint8_t> resp;
    tplcio::CtlClient::Error err;
    if (!g_ctl.call(TPLC_OP_DBG_FIELDS, resp, &err) || resp.size() < 4)
        return out;
    const uint8_t* p = resp.data();
    size_t n = resp.size(), at = 0;
    auto r32 = [&]() { uint32_t v; memcpy(&v, p + at, 4); at += 4; return v; };
    auto r64 = [&]() { uint64_t v; memcpy(&v, p + at, 8); at += 8; return v; };
    uint32_t nf = r32();
    for (uint32_t i = 0; i < nf && at + 25 <= n; i++) {
        PlcField f;
        f.off = r64(); f.size = r64(); f.tref = r32();
        f.kind = p[at]; f.flags = p[at + 1]; f.region = p[at + 2];
        f.access = p[at + 3]; at += 4;
        uint8_t nl = p[at++];
        if (at + nl > n) break;
        f.name.assign((const char*)p + at, nl);
        at += nl;
        out.push_back(std::move(f));
    }
    return out;
}

} // namespace tplcio_int
