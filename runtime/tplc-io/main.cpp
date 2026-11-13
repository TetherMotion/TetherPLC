/* tplc-io — tether_io server exposing the tether-plc control channel.
 * Live attach: any tether_io client (TCP) can invoke the full debug /
 * lifecycle surface and stream live values, with the runtime reached
 * through ctl.sock — the sidecar's stable boundary.
 *
 *   tplc-io --ctl /path/plc.ctl [--port 43770] [--bind 127.0.0.1]
 *
 * Function surface returns raw bytes for flat-typed values (no V6
 * schema required); complex ctl replies decode to JSON strings.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include "CtlClient.hpp"

#include <tether/io/Registry.hpp>
#include <tether/io/SchemaCatalog.hpp>
#include <tether/io/SchemaDigest.hpp>
#include "SchemaValueBridge.hpp"
#include <tether/io/Server.hpp>
#include <tether/io/TcpTransport.hpp>

#include <tplc/tplc_ctl.h>

#include <csignal>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <dirent.h>
#include <optional>
#include <string>
#include <type_traits>
#include <vector>

using namespace tether::io;

namespace {

volatile sig_atomic_t g_stop = 0;
void onSig(int) { g_stop = 1; }

uint64_t nowUs() {
    timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000000ull + (uint64_t)ts.tv_nsec / 1000;
}

/* ---- LE payload writer (mirrors ctl W) -------------------------- */
struct W {
    std::vector<uint8_t> b;
    void u8(uint8_t v)  { b.push_back(v); }
    void u16(uint16_t v){ b.push_back(v & 0xFF); b.push_back(v >> 8); }
    void u32(uint32_t v){ for (int i = 0; i < 4; i++) b.push_back(v >> (8 * i)); }
    void u64(uint64_t v){ for (int i = 0; i < 8; i++) b.push_back(v >> (8 * i)); }
    void bytes(const void* p, size_t n) {
        const uint8_t* c = (const uint8_t*)p;
        b.insert(b.end(), c, c + n);
    }
    void str(const std::string& s) { u16((uint16_t)s.size()); bytes(s.data(), s.size()); }
};

/* ---- minimal JSON writer ---------------------------------------- */
struct J {
    std::string s;
    J& raw(const std::string& v) { s += v; return *this; }
    J& chr(char c) { s += c; return *this; }
    J& key(const std::string& k, bool comma) {
        if (comma) s += ',';
        s += '"'; esc(k); s += "\":";
        return *this;
    }
    template <typename T,
              typename = std::enable_if_t<std::is_integral_v<T>>>
    J& num(T v) { s += std::to_string(v); return *this; }
    J& strv(const std::string& v) { s += '"'; esc(v); s += '"'; return *this; }
    J& hex(const uint8_t* p, size_t n) {
        static const char* H = "0123456789abcdef";
        s += '"';
        for (size_t i = 0; i < n; i++) { s += H[p[i] >> 4]; s += H[p[i] & 15]; }
        s += '"';
        return *this;
    }
    void esc(const std::string& v) {
        for (char c : v) {
            switch (c) {
                case '"':  s += "\\\""; break;
                case '\\': s += "\\\\"; break;
                case '\n': s += "\\n";  break;
                case '\r': s += "\\r";  break;
                case '\t': s += "\\t";  break;
                default:
                    if ((unsigned char)c < 0x20) {
                        char b[8];
                        snprintf(b, sizeof b, "\\u%04x", c);
                        s += b;
                    } else s += c;
            }
        }
    }
};

/* ---- V6 schema: one scalar node per used ValueType -----------------
   Clients decode by slot; flat entries without a schema ref can't be
   read by V6 sessions. Keys: 0x50 ('P') in byte 0, tag in byte 1.    */
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
              uint64_t dflt = 0) {
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
std::string progStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() < sizeof(TplcCtlProgStatus)) return "{}";
    const TplcCtlProgStatus* ps = (const TplcCtlProgStatus*)p.data();
    J j; j.chr('{');
    j.key("uuid", 0).hex(ps->program_uuid, 16);
    j.key("name", 1).strv(std::string((const char*)ps->name,
        strnlen((const char*)ps->name, sizeof ps->name)));
    j.key("state", 1).num(ps->state);
    j.key("cycles", 1).num(ps->cycles);
    j.key("last_fault_code", 1).num(ps->last_fault_code);
    j.key("force_count", 1).num(ps->force_count);
    j.key("version", 1).strv(std::string((const char*)ps->version,
        strnlen((const char*)ps->version, sizeof ps->version)));
    j.key("package_hash", 1).hex(ps->package_hash, 32);
    j.key("layout_hash", 1).hex(ps->layout_hash, 32);
    j.key("fault_task", 1).num(ps->fault_task);
    j.key("fault_cycle", 1).num(ps->fault_cycle);
    j.key("retain_restored", 1).num(ps->retain_restored);
    j.key("retain_dirty_bytes", 1).num(ps->retain_dirty_bytes);
    j.key("retain_last_flush_ns", 1).num(ps->retain_last_flush_ns);
    j.key("deployed_at_ns", 1).num(ps->deployed_at_ns);
    j.key("deployed_by_uid", 1).num(ps->deployed_by_uid);
    j.key("predecessor_uuid", 1).hex(ps->predecessor_uuid, 16);
    j.key("swap_stage", 1).num(ps->swap_stage);
    j.key("quarantine_left", 1).num(ps->quarantine_left);
    j.key("epoch", 1).num(ps->epoch);
    j.key("shadow_state", 1).num(ps->shadow_state);
    j.key("shadow_cycles", 1).num(ps->shadow_cycles);
    j.key("shadow_missed", 1).num(ps->shadow_missed);
    j.key("shadow_diff_count", 1).num(ps->shadow_diff_count);
    j.key("shadow_last_diff_cycle", 1).num(ps->shadow_last_diff_cycle);
    j.key("tasks", 1).chr('[');
    const uint8_t* q = p.data() + sizeof(TplcCtlProgStatus);
    size_t left = p.size() - sizeof(TplcCtlProgStatus);
    for (uint32_t i = 0; i < ps->task_count &&
                        left >= sizeof(TplcCtlTaskStat); i++) {
        const TplcCtlTaskStat* t = (const TplcCtlTaskStat*)q;
        if (i) j.chr(',');
        j.chr('{');
        j.key("name", 0).strv(std::string((const char*)t->name,
            strnlen((const char*)t->name, sizeof t->name)));
        j.key("kind", 1).num(t->kind);
        j.key("state", 1).num(t->state);
        j.key("period_ns", 1).num(t->period_ns);
        j.key("runs", 1).num(t->runs);
        j.key("late", 1).num(t->late);
        j.key("faults", 1).num(t->faults);
        j.key("last_rc", 1).num(t->last_rc);
        j.key("overruns", 1).num(t->overruns);
        j.key("exec_max_ns", 1).num(t->exec_max_ns);
        j.key("exec_avg_ns", 1).num(t->exec_avg_ns);
        j.chr('}');
        q += sizeof(TplcCtlTaskStat);
        left -= sizeof(TplcCtlTaskStat);
    }
    j.chr(']').chr('}');
    return j.s;
}

/* SHADOW_STATUS wire: u8 0 (no shadow) or TplcCtlShadowStatus    */
std::string shadowStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() == 1 && p[0] == 0)
        return "{\"loaded\":false}";
    if (p.size() < sizeof(TplcCtlShadowStatus)) return "{}";
    const TplcCtlShadowStatus* ss = (const TplcCtlShadowStatus*)p.data();
    J j; j.chr('{');
    j.key("loaded", 0).num(1);
    j.key("uuid", 1).hex(ss->program_uuid, 16);
    j.key("name", 1).strv(std::string((const char*)ss->name,
        strnlen((const char*)ss->name, sizeof ss->name)));
    j.key("state", 1).num(ss->state);
    j.key("cycles", 1).num(ss->cycles);
    j.key("missed", 1).num(ss->missed);
    j.key("diff_count", 1).num(ss->diff_count);
    j.key("last_diff_cycle", 1).num(ss->last_diff_cycle);
    j.key("runs", 1).num(ss->runs);
    j.chr('}');
    return j.s;
}

/* shadow selector wire tail {u16 slen, sel}: sel is the 16-byte
   program uuid (passed as 32 hex chars) or the program name       */
void wsel(W& w, const std::string& sel) {
    std::vector<uint8_t> b;
    if (sel.size() == 32) {
        auto nib = [](char c) -> int {
            if (c >= '0' && c <= '9') return c - '0';
            if (c >= 'a' && c <= 'f') return c - 'a' + 10;
            if (c >= 'A' && c <= 'F') return c - 'A' + 10;
            return -1;
        };
        bool ok = true;
        for (int i = 0; i < 16 && ok; i++) {
            int hi = nib(sel[i * 2]), lo = nib(sel[i * 2 + 1]);
            if (hi < 0 || lo < 0) { ok = false; break; }
            b.push_back((uint8_t)(hi * 16 + lo));
        }
        if (!ok) b.clear();
    }
    if (b.empty()) b.assign(sel.begin(), sel.end());
    if (b.empty() || b.size() > 0xFFFF) return;
    w.u16((uint16_t)b.size());
    w.bytes(b.data(), (uint32_t)b.size());
}

/* SHADOW_LIST wire: {u32 n, TplcCtlShadowStatus[n]}               */
std::string shadowListJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"shadows\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) { j.raw("]}"); return j.s; }
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n &&
                        left >= sizeof(TplcCtlShadowStatus);
         i++) {
        const TplcCtlShadowStatus* ss =
            (const TplcCtlShadowStatus*)q;
        if (i) j.chr(',');
        j.chr('{');
        j.key("uuid", 0).hex(ss->program_uuid, 16);
        j.key("name", 1).strv(std::string(
            (const char*)ss->name,
            strnlen((const char*)ss->name, sizeof ss->name)));
        j.key("state", 1).num(ss->state);
        j.key("cycles", 1).num(ss->cycles);
        j.key("missed", 1).num(ss->missed);
        j.key("diff_count", 1).num(ss->diff_count);
        j.key("last_diff_cycle", 1).num(ss->last_diff_cycle);
        j.key("runs", 1).num(ss->runs);
        j.chr('}');
        q += sizeof(TplcCtlShadowStatus);
        left -= sizeof(TplcCtlShadowStatus);
    }
    j.raw("]}");
    return j.s;
}

/* SHADOW_COMPARE wire: {u32 n, TplcCtlShadowDiff[n]}             */
std::string shadowCompareJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"diffs\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) { j.raw("]}"); return j.s; }
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= sizeof(TplcCtlShadowDiff);
         i++) {
        const TplcCtlShadowDiff* df = (const TplcCtlShadowDiff*)q;
        if (i) j.chr(',');
        j.chr('{');
        j.key("cycle", 0).num(df->cycle);
        j.key("img_off", 1).num(df->img_off);
        j.key("len", 1).num(df->len);
        j.key("active", 1).hex(df->active_bytes, df->len);
        j.key("shadow", 1).hex(df->shadow_bytes, df->len);
        j.chr('}');
        q += sizeof(TplcCtlShadowDiff);
        left -= sizeof(TplcCtlShadowDiff);
    }
    j.raw("]}");
    return j.s;
}

/* migrate-result replies (load/reset/swap): {u32 status, u32 err} */
std::string migrateResultJson(const std::vector<uint8_t>& p) {
    if (p.size() == 64) {
        J j; j.chr('{');
        j.key("refused", 0).strv("SWAP_RACE");
        j.key("live", 1).hex(p.data(), 32);
        j.key("pkg_pre", 1).hex(p.data() + 32, 32);
        j.chr('}');
        return j.s;
    }
    uint32_t st = 0, fe = 0;
    if (p.size() >= 8) { memcpy(&st, p.data(), 4);
                         memcpy(&fe, p.data() + 4, 4); }
    J j; j.chr('{');
    j.key("out_status", 0).num(st);
    j.key("out_first_error", 1).num(fe);
    j.chr('}');
    return j.s;
}

std::string fieldsJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"fields\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "[]";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 25; i++) {
        uint64_t fo, fs; uint32_t tr;
        memcpy(&fo, q, 8); memcpy(&fs, q + 8, 8); memcpy(&tr, q + 16, 4);
        uint8_t kind = q[20], fl = q[21], reg = q[22], acc = q[23],
                nl = q[24];
        q += 25; left -= 25;
        if (left < nl) break;
        if (i) j.chr(',');
        j.chr('{');
        j.key("off", 0).num(fo);
        j.key("size", 1).num(fs);
        j.key("type", 1).num(tr);
        j.key("kind", 1).num(kind);
        j.key("flags", 1).num(fl);
        j.key("region", 1).num(reg);
        j.key("access", 1).num(acc);
        j.key("name", 1).strv(std::string((const char*)q, nl));
        j.chr('}');
        q += nl; left -= nl;
    }
    j.raw("]}");
    return j.s;
}

std::string resolveJson(const std::vector<uint8_t>& p) {
    if (p.size() < 40) return "{}";
    J j; j.chr('{');
    j.key("uuid", 0).hex(p.data(), 16);
    uint32_t si, tr, sz; uint64_t off;
    memcpy(&si, p.data() + 16, 4);
    memcpy(&tr, p.data() + 24, 4);
    memcpy(&off, p.data() + 28, 8);
    memcpy(&sz, p.data() + 36, 4);
    j.key("sym", 1).num(si);
    j.key("kind", 1).num(p[20]);
    j.key("flags", 1).num(p[21]);
    j.key("region", 1).num(p[22]);
    j.key("access", 1).num(p[23]);
    j.key("type", 1).num(tr);
    j.key("off", 1).num(off);
    j.key("size", 1).num(sz);
    j.chr('}');
    return j.s;
}

std::string stateJson(const std::vector<uint8_t>& p) {
    if (p.size() < 44) return "{}";
    uint64_t bs, ro, rl; uint32_t sc;
    memcpy(&bs, p.data() + 16, 8);
    memcpy(&ro, p.data() + 24, 8);
    memcpy(&rl, p.data() + 32, 8);
    memcpy(&sc, p.data() + 40, 4);
    J j; j.chr('{');
    j.key("uuid", 0).hex(p.data(), 16);
    j.key("blob_size", 1).num(bs);
    j.key("retain_off", 1).num(ro);
    j.key("retain_len", 1).num(rl);
    j.key("symbol_count", 1).num(sc);
    j.chr('}');
    return j.s;
}

std::string forceListJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"forces\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 16; i++) {
        uint32_t h, l; uint64_t off;
        memcpy(&h, q, 4); memcpy(&off, q + 4, 8); memcpy(&l, q + 12, 4);
        q += 16; left -= 16;
        if (left < l) break;
        if (i) j.chr(',');
        j.chr('{');
        j.key("handle", 0).num(h);
        j.key("off", 1).num(off);
        j.key("len", 1).num(l);
        j.key("data", 1).hex(q, l);
        q += l; left -= l;
        j.chr('}');
        /* optional mask row follows when present */
        if (left >= l) { q += l; left -= l; }
    }
    j.raw("]}");
    return j.s;
}

std::string breakListJson(const std::vector<uint8_t>& p) {
    static const char* cmps[] = {"==","!=","<","<=",">",">="};
    J j; j.raw("{\"breaks\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 40; i++) {
        uint32_t h, tix, l; uint64_t off, val, hits;
        memcpy(&h, q, 4); memcpy(&tix, q + 4, 4); memcpy(&off, q + 8, 8);
        memcpy(&l, q + 16, 4);
        uint8_t c = q[20], cap = q[21], stale = q[22];
        memcpy(&val, q + 24, 8); memcpy(&hits, q + 32, 8);
        q += 40; left -= 40;
        if (i) j.chr(',');
        j.chr('{');
        j.key("handle", 0).num(h);
        j.key("task", 1).num(tix);
        j.key("capture", 1).num(cap);
        j.key("stale", 1).num(stale);
        if (c == 0xFF) j.key("expr", 1).num(1);
        else {
            j.key("off", 1).num(off);
            j.key("len", 1).num(l);
            j.key("cmp", 1).strv(cmps[c <= 5 ? c : 0]);
            j.key("val", 1).num(val);
        }
        j.key("hits", 1).num(hits);
        j.chr('}');
    }
    j.raw("]}");
    return j.s;
}

/* {u32 n} × {u32 handle, u32 nchan, u32 rate, u32 tlen, u32 stale} */
std::string watchListJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"watches\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 20; i++) {
        uint32_t h, nc, rt, tl, st;
        memcpy(&h, q, 4); memcpy(&nc, q + 4, 4); memcpy(&rt, q + 8, 4);
        memcpy(&tl, q + 12, 4); memcpy(&st, q + 16, 4);
        q += 20; left -= 20;
        if (i) j.chr(',');
        j.chr('{');
        j.key("handle", 0).num(h);
        j.key("channels", 1).num(nc);
        j.key("rate_hz", 1).num(rt);
        j.key("bytes", 1).num(tl);
        j.key("stale", 1).num(st);
        j.chr('}');
    }
    j.raw("]}");
    return j.s;
}

/* state diff: {u8 same, u8 mig_status, u16 pad, u32 nrules} ×
   {u8 kind,sev, u16 tid, u32 aux, u64 ooff, u32 olen, u32 osym,
    u64 noff, u32 nlen, u32 nsym, u8 ovl, u8 nvl, u16 pad,
    8B prev, 8B next, str oname, str nname}                       */
std::string stateDiffJson(const std::vector<uint8_t>& p) {
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 8) return "{}";
    J j; j.chr('{');
    j.key("same_layout", 0).num(q[0]);
    j.key("mig_status", 1).num(q[1]);
    j.key("rules", 1).chr('[');
    q += 8; left -= 8;
    uint32_t n;
    memcpy(&n, p.data() + 4, 4);
    for (uint32_t i = 0; i < n && left >= 48; i++) {
        uint16_t tid; uint32_t aux, ol, os, nl, ns;
        uint64_t oo, no;
        memcpy(&tid, q + 2, 2); memcpy(&aux, q + 4, 4);
        memcpy(&oo, q + 8, 8); memcpy(&ol, q + 16, 4);
        memcpy(&os, q + 20, 4); memcpy(&no, q + 24, 8);
        memcpy(&nl, q + 32, 4); memcpy(&ns, q + 36, 4);
        uint8_t kind = q[0], sev = q[1], ovl = q[40], nvl = q[41];
        const uint8_t* pv = q + 44;
        const uint8_t* nv2 = q + 52;
        if (left < 60) break;
        q += 60; left -= 60;
        if (i) j.chr(',');
        j.chr('{');
        j.key("kind", 0).num(kind);
        j.key("sev", 1).num(sev);
        j.key("task", 1).num(tid);
        j.key("aux", 1).num(aux);
        j.key("old", 1).chr('{');
        j.key("sym", 0).num(os); j.key("off", 1).num(oo);
        j.key("len", 1).num(ol); j.key("live", 1).num(ovl);
        j.key("prev", 1).hex(pv, 8);
        j.chr('}');
        j.chr(',');
        j.key("new", 1).chr('{');
        j.key("sym", 0).num(ns); j.key("off", 1).num(no);
        j.key("len", 1).num(nl); j.key("live", 1).num(nvl);
        j.key("next", 1).hex(nv2, 8);
        j.chr('}');
        if (left >= 2) {
            uint16_t sl; memcpy(&sl, q, 2);
            if (left >= (size_t)sl + 2) {
                j.key("old_name", 1)
                    .strv(std::string((const char*)q + 2, sl));
                q += 2 + sl; left -= 2 + sl;
            }
        }
        if (left >= 2) {
            uint16_t sl; memcpy(&sl, q, 2);
            if (left >= (size_t)sl + 2) {
                j.key("new_name", 1)
                    .strv(std::string((const char*)q + 2, sl));
                q += 2 + sl; left -= 2 + sl;
            }
        }
        j.chr('}');
    }
    j.raw("]}");
    return j.s;
}

std::string watchGetJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"items\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 4; i++) {
        uint32_t l;
        memcpy(&l, q, 4); q += 4; left -= 4;
        if (left < l) break;
        if (i) j.chr(',');
        j.chr('{');
        j.key("len", 0).num(l);
        j.key("data", 1).hex(q, l);
        j.chr('}');
        q += l; left -= l;
    }
    j.raw("]}");
    return j.s;
}

std::string traceStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() < 24) return "{}";
    uint32_t nch, fl, fl2 = 0; uint64_t wr, av;
    memcpy(&nch, p.data(), 4); memcpy(&fl, p.data() + 4, 4);
    memcpy(&wr, p.data() + 8, 8); memcpy(&av, p.data() + 16, 8);
    if (p.size() >= 28) memcpy(&fl2, p.data() + 24, 4);
    J j; j.chr('{');
    j.key("channels", 0).num(nch);
    j.key("frame_len", 1).num(fl);
    j.key("written", 1).num(wr);
    j.key("available", 1).num(av);
    j.key("frozen", 1).num(fl2 & 1);
    j.chr('}');
    return j.s;
}

std::string traceReadJson(const std::vector<uint8_t>& p) {
    if (p.size() < 16) return "{}";
    const uint8_t* q = p.data();
    uint64_t first; uint32_t n, fl;
    memcpy(&first, q, 8); memcpy(&n, q + 8, 4); memcpy(&fl, q + 12, 4);
    q += 16;
    size_t left = p.size() - 16;
    J j; j.chr('{');
    j.key("first_seq", 0).num(first);
    j.key("frame_len", 1).num(fl);
    j.key("frames", 1).chr('[');
    for (uint32_t i = 0; i < n && left >= 16 + fl; i++) {
        uint64_t s, ts;
        memcpy(&s, q, 8); memcpy(&ts, q + 8, 8);
        if (i) j.chr(',');
        j.chr('{');
        j.key("seq", 0).num(s);
        j.key("ts", 1).num(ts);
        j.key("data", 1).hex(q + 16, fl);
        j.chr('}');
        q += 16 + fl; left -= 16 + fl;
    }
    j.raw("]}");
    return j.s;
}

std::string retainStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() < 44) return "{}";
    uint64_t rlen, rgen, rfl, rrs, rls;
    memcpy(&rlen, p.data() + 4, 8);  memcpy(&rgen, p.data() + 12, 8);
    memcpy(&rfl,  p.data() + 20, 8); memcpy(&rrs,  p.data() + 28, 8);
    memcpy(&rls,  p.data() + 36, 8);
    J j; j.chr('{');
    j.key("attached", 0).num(p[0]);
    j.key("dirty", 1).num(p[1]);
    j.key("len", 1).num(rlen);
    j.key("gen", 1).num(rgen);
    j.key("flushes", 1).num(rfl);
    j.key("restores", 1).num(rrs);
    j.key("losses", 1).num(rls);
    j.chr('}');
    return j.s;
}

std::string imageStatsJson(const std::vector<uint8_t>& p) {
    if (p.size() < sizeof(TplcCtlImageStats)) return "{}";
    const TplcCtlImageStats* is = (const TplcCtlImageStats*)p.data();
    J j; j.chr('{');
    j.key("epoch", 0).num(is->epoch);
    j.key("in_seq", 1).num(is->in_seq);
    j.key("out_seq", 1).num(is->out_seq);
    j.key("in_writes", 1).num(is->in_writes);
    j.key("out_writes", 1).num(is->out_writes);
    j.key("in_retries", 1).num(is->in_retries);
    j.key("out_retries", 1).num(is->out_retries);
    j.key("in_size", 1).num(is->in_size);
    j.key("out_size", 1).num(is->out_size);
    j.chr('}');
    return j.s;
}

std::string journalJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"lines\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 2; i++) {
        uint16_t l;
        memcpy(&l, q, 2); q += 2; left -= 2;
        if (left < l) break;
        if (i) j.chr(',');
        j.strv(std::string((const char*)q, l));
        q += l; left -= l;
    }
    j.raw("]}");
    return j.s;
}

std::string stateChangeJson(const std::vector<uint8_t>& p) {
    if (p.size() < 4) return "{}";
    J j; j.chr('{');
    j.key("from", 0).num(p[0]);
    j.key("to", 1).num(p[1]);
    j.chr('}');
    return j.s;
}

void assignSchema(FunctionParameter& p) {
    const SchemaNode* n = g_graph.find(keyFor(p.type));
    if (!n) return;
    p.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(p.schema)) p.schemaSlot = *s;
}

void assignSchema(FunctionReturn& r) {
    const SchemaNode* n = g_graph.find(keyFor(r.type));
    if (!n) return;
    r.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(r.schema)) r.schemaSlot = *s;
}

void assignSchema(SignalEntry& e) {
    const SchemaNode* n = g_graph.find(keyFor(e.valueType));
    if (!n) return;
    e.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(e.schema)) {
        e.schemaSlot = *s;
        /* V6 clients pick schema-aware decoding off this annotation —
           without it an I64 reads back as a bogus f64              */
        e.metadata["schema.name"] = n->name;
    }
}

void assignSchema(ParamEntry& p) {
    const SchemaNode* n = g_graph.find(keyFor(p.valueType));
    if (!n) return;
    p.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(p.schema)) {
        p.schemaSlot = *s;
        p.metadata["schema.name"] = n->name;
    }
}

std::vector<SchemaManifestEntry> plcManifest(const SchemaGraph& g) {
    std::vector<SchemaManifestEntry> m;
    m.reserve(g.nodes.size());
    for (const auto& n : g.nodes)
        m.push_back({SchemaRef{n.key, computeSchemaDigest(n)}, n.revision});
    return m;
}

void buildSchema() {
    const ValueType types[] = {
        ValueType::U8, ValueType::U16, ValueType::U32, ValueType::U64,
        ValueType::I8, ValueType::I32, ValueType::I64,
        ValueType::F32, ValueType::F64, ValueType::Bool,
        ValueType::String, ValueType::Binary,
    };
    for (ValueType t : types) {
        SchemaNode n;
        n.key = keyFor(t);
        n.kind = t == ValueType::String ? SchemaKind::String
               : t == ValueType::Binary ? SchemaKind::Bytes
                                        : SchemaKind::Scalar;
        n.scalarType = t;
        n.name = "plc.type." + std::to_string((unsigned)t);
        g_graph.nodes.push_back(std::move(n));
    }
    resolveSchemaDigests(g_graph);
    g_catalog.install(g_graph, plcManifest(g_graph));
}

/* ---- function entry builders ------------------------------------- */
uint64_t g_fnId = 100;

FunctionParameter P(const char* name, const char* desc, ValueType t) {
    return FunctionParameter{name, desc, t};
}

FunctionParameter Popt(const char* name, const char* desc, ValueType t,
                       std::vector<uint8_t> dflt) {
    FunctionParameter p{name, desc, t};
    p.optional = true;
    p.hasDefault = true;
    p.defaultValue = std::move(dflt);
    return p;
}

FunctionEntry fn(const char* name, const char* desc,
                 std::vector<FunctionParameter> params,
                 FunctionCallback cb,
                 std::optional<ValueType> ret = std::nullopt) {
    FunctionEntry e;
    e.id = g_fnId++;
    e.name = name;
    e.description = desc;
    e.group = "plc";
    e.parameters = std::move(params);
    for (auto& p : e.parameters) assignSchema(p);
    e.callback = std::move(cb);
    if (ret) {
        e.returnValue.present = true;
        e.returnValue.name = "result";
        e.returnValue.type = *ret;
        assignSchema(e.returnValue);
    }
    return e;
}

/* simple ctl op with no args and an optional JSON decoder */
FunctionEntry jsonOp(const char* name, const char* desc, uint16_t op,
                     std::string (*dec)(const std::vector<uint8_t>&)) {
    return fn(name, desc, {},
              [op, dec](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(op, out, r)) return r;
        return okStr(dec ? dec(out) : std::string(out.begin(), out.end()));
    }, ValueType::String);
}

/* state-changing lifecycle op: no payload → TplcCtlProgStateChange */
FunctionEntry stateOp(const char* name, const char* desc, uint16_t op,
                      std::vector<FunctionParameter> params = {},
                      void (*fill)(W&, const std::vector<FunctionArgument>&)
                          = nullptr) {
    return fn(name, desc, std::move(params),
              [op, fill](const std::vector<FunctionArgument>& a) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        W w;
        if (fill) fill(w, a);
        if (!ctlCall(op, w.b.data(), (uint32_t)w.b.size(), out, r))
            return r;
        return okStr(stateChangeJson(out));
    }, ValueType::String);
}

/* ---- catalog ------------------------------------------------------ */
std::vector<FunctionEntry> buildFunctions() {
    std::vector<FunctionEntry> v;

    /* lifecycle */
    v.push_back(stateOp("prog.start", "start the stopped program",
                        TPLC_OP_PROG_START));
    v.push_back(stateOp("prog.stop", "stop the running program",
                        TPLC_OP_PROG_STOP));
    v.push_back(stateOp("prog.kill",
                        "abort flag + deschedule immediately",
                        TPLC_OP_PROG_KILL));
    v.push_back(stateOp("prog.reset",
                        "re-initialize a stopped program (warm keeps retain)",
                        TPLC_OP_PROG_RESET,
                        {Popt("warm", "keep retain data", ValueType::Bool, {1})},
                        [](W& w, const std::vector<FunctionArgument>& a) {
                            w.u8((uint8_t)argU(a, 0, 1));
                        }));
    v.push_back(stateOp("prog.unload", "unload the stopped program",
                        TPLC_OP_PROG_UNLOAD));
    v.push_back(stateOp("prog.prepare",
                        "load a .tplc/.so into the stopped slot",
                        TPLC_OP_PROG_PREPARE,
                        {P("path", "package path (server-local)",
                           ValueType::String)},
                        [](W& w, const std::vector<FunctionArgument>& a) {
                            w.str(argStr(a, 0));
                        }));
    v.push_back(fn("prog.rollback",
                   "roll back the quarantined program", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_PROG_ROLLBACK, out, r)) return r;
        return okVoid();
    }));

    /* status */
    v.push_back(jsonOp("prog.status",
                       "full program status object (JSON)",
                       TPLC_OP_PROG_STATUS, progStatusJson));
    v.push_back(jsonOp("dbg.state",
                       "blob/retain extents + symbol count",
                       TPLC_OP_DBG_STATE, stateJson));
    v.push_back(fn("dbg.fields",
                   "blob layout: every field with name/off/size/access",
                   {Popt("sel", "0 active, N = shadow N (SHADOW_LIST order)", ValueType::U8,
                         {0})},
                   [](const std::vector<FunctionArgument>& a) {
        uint64_t sel = argU(a, 0, 0);
        W w;
        if (sel) w.u8((uint8_t)sel);
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_FIELDS, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(fieldsJson(out));
    }, ValueType::String));
    v.push_back(jsonOp("dbg.state_diff",
                       "last online-change migrate rules",
                       TPLC_OP_DBG_STATE_DIFF, stateDiffJson));
    v.push_back(jsonOp("image.stats", "process-image counters",
                       TPLC_OP_IMAGE_STATS, imageStatsJson));
    v.push_back(jsonOp("journal.tail",
                       "durable audit journal tail (JSON lines)",
                       TPLC_OP_JOURNAL_TAIL, journalJson));
    v.push_back(jsonOp("map.get", "deploy-map section text",
                       TPLC_OP_MAP_GET,
                       [](const std::vector<uint8_t>& p) {
        J j; j.chr('{'); j.key("map", 0)
            .strv(std::string((const char*)p.data(), p.size()));
        j.chr('}');
        return j.s;
    }));
    v.push_back(jsonOp("retain.status", "retain store state",
                       TPLC_OP_RETAIN_STATUS, retainStatusJson));
    v.push_back(jsonOp("trace.status", "trace recorder state",
                       TPLC_OP_TRACE_STATUS, traceStatusJson));
    v.push_back(jsonOp("dbg.breaks", "armed break/capture list",
                       TPLC_OP_DBG_BREAK_LIST, breakListJson));
    v.push_back(jsonOp("dbg.forces", "active force list",
                       TPLC_OP_DBG_FORCE_LIST, forceListJson));
    v.push_back(jsonOp("dbg.watch_list", "watch handles",
                       TPLC_OP_DBG_WATCH_LIST, watchListJson));

    /* shadow run (doc 19 §3.3) */
    v.push_back(fn("shadow.load",
                   "load a candidate program as shadow (needs running "
                   "active)",
                   {P("path", "package path (server-local)",
                      ValueType::String)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.str(argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_LOAD, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        if (out.size() >= 17 && out[0]) {
            J j; j.chr('{');
            j.key("loaded", 0).num(1);
            j.key("uuid", 1).hex(out.data() + 1, 16);
            j.chr('}');
            return okStr(j.s);
        }
        return okStr(migrateResultJson(out));
    }, ValueType::String));
    v.push_back(fn("shadow.status",
                   "candidate program status (sel: name or uuid)",
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_STATUS, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(shadowStatusJson(out));
    }, ValueType::String));
    v.push_back(jsonOp("shadow.list",
                       "every loaded shadow candidate (JSON array)",
                       TPLC_OP_SHADOW_LIST, shadowListJson));
    v.push_back(fn("shadow.compare",
                   "tail of the %Q diff ring (img-off runs ≤32B)",
                   {Popt("max", "max entries", ValueType::U32,
                         {32, 0, 0, 0}),
                    Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 32));
        wsel(w, argStr(a, 1));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_COMPARE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(shadowCompareJson(out));
    }, ValueType::String));
    v.push_back(fn("shadow.promote",
                   "promote the candidate to active (quarantines "
                   "current; drops other shadows)",
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_PROMOTE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        J j; j.chr('{');
        if (out.size() >= 24) {
            j.key("uuid", 0).hex(out.data(), 16);
            uint64_t ep; memcpy(&ep, out.data() + 16, 8);
            j.key("epoch", 1).num(ep);
        }
        j.chr('}');
        return okStr(j.s);
    }, ValueType::String));
    v.push_back(fn("shadow.drop", "discard a candidate", 
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_DROP, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okVoid();
    }));
    v.push_back(fn("shadow.reset",
                   "re-migrate candidate state from the live blob",
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_RESET, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(migrateResultJson(out));
    }, ValueType::String));

    /* debug reads/writes */
    v.push_back(fn("dbg.resolve",
                   "resolve a symbol path to blob offset/size",
                   {P("name", "symbol path", ValueType::String)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.str(argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_RESOLVE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(resolveJson(out));
    }, ValueType::String));
    v.push_back(fn("dbg.read", "read blob bytes",
                   {P("off", "blob offset", ValueType::U64),
                    P("len", "byte count", ValueType::U32),
                    Popt("sel", "0 active, N = shadow N (SHADOW_LIST order)", ValueType::U8,
                         {0})},
                   [](const std::vector<FunctionArgument>& a) {
        uint64_t sel = argU(a, 2, 0);
        W w; w.u64(argU(a, 0)); w.u32((uint32_t)argU(a, 1));
        if (sel) w.u8((uint8_t)sel);
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_READ, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okBin(out);
    }, ValueType::Binary));
    v.push_back(fn("dbg.write", "write blob bytes",
                   {P("off", "blob offset", ValueType::U64),
                    P("data", "bytes", ValueType::Binary)},
                   [](const std::vector<FunctionArgument>& a) {
        auto d = argBin(a, 1);
        W w; w.u64(argU(a, 0));
        w.u32((uint32_t)d.size());
        w.bytes(d.data(), d.size());
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WRITE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okVoid();
    }));
    v.push_back(fn("dbg.force", "force blob bytes (override)",
                   {P("off", "blob offset", ValueType::U64),
                    P("data", "bytes", ValueType::Binary),
                    Popt("mask", "bit force mask", ValueType::Binary, {})},
                   [](const std::vector<FunctionArgument>& a) {
        auto d = argBin(a, 1);
        auto m = argBin(a, 2);
        W w; w.u64(argU(a, 0));
        w.u32((uint32_t)d.size());
        w.bytes(d.data(), d.size());
        if (!m.empty()) w.bytes(m.data(), m.size());
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_FORCE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("dbg.unforce",
                   "drop a force (0xFFFFFFFF = all)",
                   {P("handle", "force handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 0xFFFFFFFFu));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_FORCE_CLEAR, w.b.data(), 4, out, r))
            return r;
        return okVoid();
    }));

    /* task control */
    for (auto& t : {std::pair{"task.suspend", TPLC_OP_TASK_SUSPEND},
                    {"task.resume",  TPLC_OP_TASK_RESUME}}) {
        v.push_back(fn(t.first, "suspend/resume a task by index",
                       {P("index", "task index", ValueType::U32)},
                       [op = t.second](
                           const std::vector<FunctionArgument>& a) {
            W w; w.u32((uint32_t)argU(a, 0));
            std::vector<uint8_t> out;
            FunctionCallResult r;
            if (!ctlCall(op, w.b.data(), 4, out, r)) return r;
            return okVoid();
        }));
    }
    v.push_back(fn("task.step", "single-step a suspended task",
                   {P("index", "task index", ValueType::U32),
                    Popt("cycles", "cycles to run", ValueType::U32,
                         {1, 0, 0, 0})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0));
        w.u32((uint32_t)argU(a, 1, 1));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TASK_STEP, w.b.data(), 8, out, r))
            return r;
        return okVoid();
    }));

    /* watch — spec: "off:len,off:len,name" (same as CLI syntax) */
    v.push_back(fn("watch.add",
                   "arm a watch; spec = 'off:len,…' items or symbol paths",
                   {P("spec", "comma-separated items", ValueType::String)},
                   [](const std::vector<FunctionArgument>& a) {
        std::string spec = argStr(a, 0);
        /* split on ',' — item is OFF:LEN digits or a symbol path   */
        std::vector<std::string> items;
        size_t at = 0;
        while (at <= spec.size()) {
            size_t c = spec.find(',', at);
            std::string it = spec.substr(
                at, c == std::string::npos ? c : c - at);
            if (!it.empty()) items.push_back(it);
            if (c == std::string::npos) break;
            at = c + 1;
        }
        if (items.empty() || items.size() > 16) {
            FunctionCallResult r;
            r.errorMessage = "need 1..16 items";
            return r;
        }
        W w;
        w.u32((uint32_t)items.size());
        w.u32(0);
        for (auto& it : items) {
            uint64_t off = 0; uint32_t l = 0; uint16_t nl = 0;
            if (it.find(':') == std::string::npos) {
                nl = (uint16_t)it.size();
            } else if (sscanf(it.c_str(), "%llu:%u",
                              (unsigned long long*)&off, &l) != 2) {
                FunctionCallResult r;
                r.errorMessage = "bad item " + it;
                return r;
            }
            w.u64(off); w.u32(l); w.u16(nl);
            if (nl) w.bytes(it.data(), nl);
        }
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WATCH_ADD, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("watch.get", "read watch values",
                   {P("handle", "watch handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WATCH_GET, w.b.data(), 4, out, r))
            return r;
        return okStr(watchGetJson(out));
    }, ValueType::String));
    v.push_back(fn("watch.rm", "remove a watch (0xFFFFFFFF = all)",
                   {P("handle", "watch handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 0xFFFFFFFFu));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WATCH_REMOVE, w.b.data(), 4, out, r))
            return r;
        return okVoid();
    }));

    /* break / capture */
    v.push_back(fn("dbg.break_add",
                   "value-compare break (cmp 0..5: == != < <= > >=)",
                   {P("task", "task index", ValueType::U32),
                    P("off", "blob offset", ValueType::U64),
                    P("len", "byte count ≤8", ValueType::U32),
                    P("cmp", "comparator 0..5", ValueType::U8),
                    P("val", "compare value", ValueType::U64),
                    Popt("capture", "freeze trace instead of suspend",
                         ValueType::Bool, {0})},
                   [](const std::vector<FunctionArgument>& a) {
        W w;
        w.u32((uint32_t)argU(a, 0));
        w.u64(argU(a, 1));
        w.u32((uint32_t)argU(a, 2));
        w.u8((uint8_t)argU(a, 3));
        w.u8(0); w.u8(0); w.u8(0);
        w.u64(argU(a, 4));
        uint16_t op = argU(a, 5) ? TPLC_OP_DBG_CAPTURE
                                 : TPLC_OP_DBG_BREAK;
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(op, w.b.data(), (uint32_t)w.b.size(), out, r))
            return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("dbg.break_expr",
                   "expression-triggered break/capture",
                   {P("task", "task index", ValueType::U32),
                    P("expr", "trigger expression", ValueType::String),
                    Popt("capture", "freeze trace instead of suspend",
                         ValueType::Bool, {0})},
                   [](const std::vector<FunctionArgument>& a) {
        std::string ex = argStr(a, 1);
        if (ex.size() > 256) {
            FunctionCallResult r;
            r.errorMessage = "expr > 256 chars";
            return r;
        }
        W w;
        w.u32((uint32_t)argU(a, 0));
        w.u16((uint16_t)ex.size());
        w.bytes(ex.data(), ex.size());
        uint16_t op = argU(a, 2) ? TPLC_OP_DBG_CAPTURE_EXPR
                                 : TPLC_OP_DBG_BREAK_EXPR;
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(op, w.b.data(), (uint32_t)w.b.size(), out, r))
            return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("dbg.break_clear",
                   "clear a break (0xFFFFFFFF = all)",
                   {P("handle", "break handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 0xFFFFFFFFu));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_BREAK_CLEAR, w.b.data(), 4, out, r))
            return r;
        return okVoid();
    }));

    /* trace */
    v.push_back(fn("trace.conf",
                   "arm the flight recorder; chans = 'off:len,…' ≤8",
                   {P("chans", "comma-separated off:len channels",
                      ValueType::String),
                    Popt("nofreeze", "don't freeze on fault",
                         ValueType::Bool, {0})},
                   [](const std::vector<FunctionArgument>& a) {
        std::string spec = argStr(a, 0);
        std::vector<std::pair<uint64_t, uint32_t>> ch;
        size_t at = 0;
        while (at <= spec.size() && ch.size() < 8) {
            size_t c = spec.find(',', at);
            std::string it = spec.substr(
                at, c == std::string::npos ? c : c - at);
            uint64_t off; uint32_t l;
            if (!it.empty() &&
                sscanf(it.c_str(), "%llu:%u",
                       (unsigned long long*)&off, &l) == 2 &&
                l >= 1 && l <= 8)
                ch.push_back({off, l});
            else if (!it.empty()) {
                FunctionCallResult r;
                r.errorMessage = "bad channel " + it;
                return r;
            }
            if (c == std::string::npos) break;
            at = c + 1;
        }
        W w;
        w.u32((uint32_t)ch.size());
        w.u32(argU(a, 1) ? 1 : 0);
        for (auto& c : ch) { w.u64(c.first); w.u32(c.second); w.u32(0); }
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TRACE_CONF, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        uint32_t fl = 0;
        if (out.size() >= 4) memcpy(&fl, out.data(), 4);
        return okU32(fl);
    }, ValueType::U32));
    v.push_back(fn("trace.read", "read trace frames",
                   {Popt("from", "first seq", ValueType::U64,
                         {0,0,0,0,0,0,0,0}),
                    Popt("max", "frame cap", ValueType::U32,
                         {0,2,0,0})},
                   [](const std::vector<FunctionArgument>& a) {
        W w;
        w.u64(argU(a, 0));
        w.u32((uint32_t)argU(a, 1, 512));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TRACE_READ, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(traceReadJson(out));
    }, ValueType::String));
    v.push_back(fn("trace.clear", "clear the trace ring", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TRACE_CLEAR, out, r)) return r;
        return okVoid();
    }));

    /* retain */
    v.push_back(fn("retain.flush", "flush retain store now", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_RETAIN_FLUSH, out, r)) return r;
        return okVoid();
    }));
    v.push_back(fn("retain.dump", "retain region bytes", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_RETAIN_DUMP, out, r)) return r;
        return okBin(out);
    }, ValueType::Binary));

    return v;
}

/* ---- signals ------------------------------------------------------ */

/* PROG_STATUS → struct, ~20 ms cached so a signal batch costs one
   ctl round-trip (mutex covers the cache; sessions are threaded)  */
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

template <typename T, typename F>
ReadFn scalarRd(F proj) {
    return [proj](void* dst) {
        TplcCtlProgStatus ps;
        std::vector<uint8_t> raw;
        T v = proj(ps, progStatus(ps, raw));
        memcpy(dst, &v, sizeof v);
    };
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

struct PlcField {
    std::string name;
    uint8_t kind = 0, flags = 0, region = 0, access = 0;
    uint32_t tref = 0;
    uint64_t off = 0, size = 0;
};

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

static uint32_t le32(const uint8_t* p) {
    uint32_t v; memcpy(&v, p, 4); return v;
}
static uint64_t le64(const uint8_t* p) {
    uint64_t v; memcpy(&v, p, 8); return v;
}

/* TYPC tag → tether_io ValueType (mirrors bridge CatalogExporter) */
ValueType vtOfTag(uint8_t t) {
    switch (t) {
        case 0:  return ValueType::Bool;
        case 1:  return ValueType::I8;
        case 2:  return ValueType::I16;
        case 3:  return ValueType::I32;
        case 4:  return ValueType::I64;
        case 5:  return ValueType::U8;
        case 6:  return ValueType::U16;
        case 7:  return ValueType::U32;
        case 8:  return ValueType::U64;
        case 9:  return ValueType::F32;
        case 10: return ValueType::F64;
        case 11: return ValueType::I64;   /* TIME → i64 ns */
        case 15: return ValueType::String;
        case 18: return ValueType::Array;
        case 19: return ValueType::Struct;
        case 20: return ValueType::Enum;
        case 21: return ValueType::Struct; /* FB instance */
        default: return ValueType::Binary;
    }
}

uint32_t scalarBytes(ValueType t) {
    switch (t) {
        case ValueType::Bool: case ValueType::U8: case ValueType::I8:
            return 1;
        case ValueType::U16: case ValueType::I16: return 2;
        case ValueType::U32: case ValueType::I32: case ValueType::F32:
            return 4;
        case ValueType::U64: case ValueType::I64: case ValueType::F64:
            return 8;
        default: return 0;
    }
}

bool readFileBytes(const std::string& path, std::vector<uint8_t>& out,
                   size_t cap = 64u << 20) {
    FILE* f = fopen(path.c_str(), "rb");
    if (!f) return false;
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (sz < 0 || (uint64_t)sz > cap) { fclose(f); return false; }
    out.resize((size_t)sz);
    bool ok = sz == 0 || fread(out.data(), 1, (size_t)sz, f) == (size_t)sz;
    fclose(f);
    return ok;
}

/* TYPC member row: (name strtab off, member type idx, offset, flags)
   — 16 B each, indexed via the type entry's member_start/count.   */
struct TypcMember {
    std::string name;
    uint32_t type = 0;
    uint32_t off = 0;
};

struct TypcType {
    uint8_t tag = 0;
    uint32_t size = 0;
    std::vector<TypcMember> members;   /* STRUCT/FUNCTION_BLOCK only */
};

constexpr uint8_t kTagStruct = 19;
constexpr uint8_t kTagEnum = 20;
constexpr uint8_t kTagFb = 21;
constexpr uint32_t kNoIdx = 0xFFFFFFFFu;

/* .tplc container: 160B header (uuid ASCII @32, toc_count @16),
   32B TOC entries from offset 160 {type,flags,off,size,ver,aux}.
   TYPC layout (tplc-mir desc.rs / migrate.rs decode_typc):
     hdr 24B: magic, ver, count, member_table_off, strtab_off, rsv
     entries: count × 56B {tag@0, name@4, elem@8, mstart@12, mcount@16,
                           lo@20, hi@24, size@28, align@32}
             ENUM entries carry an inline tail {u32 n + n×(name i32? no:
             name_off u32 + ordinal i64)} — skipped by sequential walk
     member rows: 16B {name_off u32, type u32, off u32, flags u32}
     strtab: NUL-terminated strings                                  */
std::vector<TypcType> decodeTypc(const std::vector<uint8_t>& sec) {
    std::vector<TypcType> out;
    if (sec.size() < 24 || memcmp(sec.data(), "TYPC", 4) != 0) return out;
    uint32_t count = le32(sec.data() + 8);
    uint32_t mtab = le32(sec.data() + 12);
    uint32_t stab = le32(sec.data() + 16);
    if (24ull + (uint64_t)count * 56 > sec.size() ||
        mtab >= sec.size() || stab > sec.size())
        return out;
    auto str = [&](uint32_t o) -> std::string {
        if (o >= sec.size() - stab) return {};
        const char* p = (const char*)sec.data() + stab + o;
        size_t mx = sec.size() - stab - o;
        const void* z = memchr(p, 0, mx);
        return std::string(p, z ? (const char*)z - p : mx);
    };
    size_t pos = 24;
    for (uint32_t i = 0; i < count && pos + 56 <= sec.size(); i++) {
        const uint8_t* e = sec.data() + pos;
        TypcType t;
        t.tag = e[0];
        uint32_t mstart = le32(e + 12), mcount = le32(e + 16);
        t.size = le32(e + 28);
        pos += 56;
        if (t.tag == kTagEnum && pos + 4 <= sec.size()) {
            uint32_t nv = le32(sec.data() + pos);
            uint64_t tail = 4ull + (uint64_t)nv * 12;
            pos += tail <= sec.size() - pos ? (size_t)tail : 0;
        }
        if ((t.tag == kTagStruct || t.tag == kTagFb) &&
            mstart != kNoIdx && mcount) {
            for (uint32_t k = 0; k < mcount; k++) {
                uint64_t ro = (uint64_t)mtab +
                              ((uint64_t)mstart + k) * 16;
                if (ro + 16 > sec.size()) break;
                const uint8_t* r = sec.data() + ro;
                TypcMember m;
                m.name = str(le32(r));
                m.type = le32(r + 4);
                m.off = le32(r + 8);
                t.members.push_back(std::move(m));
            }
        }
        out.push_back(std::move(t));
    }
    return out;
}

std::string dashedUuid(const uint8_t uuid[16]) {
    char b[37];
    snprintf(b, sizeof b,
             "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-"
             "%02x%02x%02x%02x%02x%02x",
             uuid[0], uuid[1], uuid[2], uuid[3], uuid[4], uuid[5],
             uuid[6], uuid[7], uuid[8], uuid[9], uuid[10], uuid[11],
             uuid[12], uuid[13], uuid[14], uuid[15]);
    return b;
}

/* TYPC section bytes for the package matching the running program's
   uuid; empty when no --packages/--pkg source matches. */
std::vector<uint8_t> typcSectionForProgram(
    const TplcCtlProgStatus& ps, const std::string& packagesDir,
    const std::vector<std::string>& pkgFiles) {
    std::string want = dashedUuid(ps.program_uuid);
    auto tryPath = [&](const std::string& path) -> std::vector<uint8_t> {
        std::vector<uint8_t> pkg;
        if (!readFileBytes(path, pkg) ||
            pkg.size() < 192 || memcmp(pkg.data(), "TPLCPKG\0", 8) != 0)
            return {};
        char puuid[37]{};
        memcpy(puuid, pkg.data() + 32, 36);
        if (want != puuid) return {};
        uint32_t ntoc = le32(pkg.data() + 16);
        if (ntoc > 64 || 160ull + (uint64_t)ntoc * 32 > pkg.size())
            return {};
        const uint8_t* toc = pkg.data() + 160;
        for (uint32_t i = 0; i < ntoc; i++) {
            const uint8_t* e = toc + (size_t)i * 32;
            if (le32(e) != 0x0021u) continue;          /* TYPE_CATALOG */
            uint64_t off = le64(e + 8), sz = le64(e + 16);
            if (off + sz > pkg.size() || sz < 24)
                return std::vector<uint8_t>{};
            const uint8_t* t = pkg.data() + off;
            if (memcmp(t, "TYPC", 4) != 0)
                return std::vector<uint8_t>{};
            return std::vector<uint8_t>(t, t + sz);
        }
        return {};
    };
    for (const auto& f : pkgFiles) {
        auto t = tryPath(f);
        if (!t.empty()) return t;
    }
    if (!packagesDir.empty()) {
        DIR* d = opendir(packagesDir.c_str());
        if (d) {
            struct dirent* de;
            uint32_t seen = 0;
            while ((de = readdir(d)) && seen++ < 512) {
                std::string n = de->d_name;
                if (n.size() < 5 ||
                    n.compare(n.size() - 5, 5, ".tplc") != 0)
                    continue;
                auto t = tryPath(packagesDir + "/" + n);
                if (!t.empty()) { closedir(d); return t; }
            }
            closedir(d);
        }
    }
    return {};
}

std::vector<TypcType> typcTypesForProgram(
    const TplcCtlProgStatus& ps, const std::string& packagesDir,
    const std::vector<std::string>& pkgFiles) {
    return decodeTypc(typcSectionForProgram(ps, packagesDir, pkgFiles));
}

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

} /* namespace */

static void usage(const char* a0) {
    fprintf(stderr,
        "usage: %s --ctl SOCK [--port 43770] [--bind 127.0.0.1]\n"
        "       [--packages DIR] [--pkg FILE]...\n"
        "       [--no-blob-signal]  (drop the whole-blob Binary\n"
        "         signal from the catalog)\n"
        "  --packages DIR  scan DIR/*.tplc for the running program's\n"
        "                  package (TYPC types for plc.* field entries)\n"
        "  --pkg FILE      extra .tplc to try (boot --package loads)\n", a0);
}

/* Populate the whole registry: status signals, epoch marker, blob
 * signal, the ctl function surface, and the dynamic plc.* field
 * entries for the active program. Called before the server starts
 * and again after each program-set epoch change — registry mutation
 * is only safe with no sessions attached, so the watchdog stops the
 * server first (joins in-flight callbacks), clears, repopulates, and
 * rebinds; clients see a brief disconnect then re-list. */
/* returns true when the catalog may be incomplete — no program name
   yet or the field table unreadable — so the caller keeps polling
   DBG_FIELDS and rebuilds once the layout reappears (deploy
   stop→prepare→start races the epoch bump)                     */
static bool populateRegistry(Registry& registry, uint64_t epoch,
                             bool want_blob_sig,
                             const std::string& packagesDir,
                             const std::vector<std::string>& pkgFiles) {
    /* ids live in the registry's shared idMap_ (params, signals and
       functions compete for the same id space) and the map is cleared
       on every rebuild — so the counters restart with each populate.
       Letting them run process-wide eventually pushes function ids
       (g_fnId) into the signal range and addParam/addSignal start
       failing on phantom collisions.                            */
    g_fnId = 100;
    g_sigId = 200;
    bool fieldsMissing = false;
    /* live signals: status scalars every session can stream/poll   */
    registry.addSignal(sig("status.state", "0 stop 1 run 2 fault 3 empty",
        ValueType::U8,
        scalarRd<uint8_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.state : (uint8_t)0xFF; })));
    registry.addSignal(sig("status.cycles", "program cycle counter",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.cycles : 0; })));
    registry.addSignal(sig("status.epoch",
        "management epoch (bumps on program-set change)",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.epoch : 0; })));
    registry.addSignal(sig("status.fault_code", "last fault rc",
        ValueType::I32,
        scalarRd<int32_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.last_fault_code : 0; })));
    registry.addSignal(sig("status.force_count", "active forces",
        ValueType::U32,
        scalarRd<uint32_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.force_count : 0; })));
    registry.addSignal(sig("status.swap_stage",
        "0 none 1 quarantine",
        ValueType::U8,
        scalarRd<uint8_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.swap_stage : (uint8_t)0; })));
    registry.addSignal(sig("shadow.state", "0 none 1 running",
        ValueType::U8,
        scalarRd<uint8_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_state : (uint8_t)0; })));
    registry.addSignal(sig("shadow.cycles", "candidate task runs",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_cycles : 0; })));
    registry.addSignal(sig("shadow.missed",
        "budget-skipped candidate cycles",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_missed : 0; })));
    registry.addSignal(sig("shadow.diff_count",
        "total %Q diff entries",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_diff_count : 0; })));

    if (epoch) {
        SignalEntry m;
        m.id = g_sigId++;
        m.name = "epoch." + std::to_string(epoch);
        m.description =
            "program-set generation marker (epoch bump)";
        m.group = "plc";
        m.valueType = ValueType::U64;
        m.readFn = [ep = epoch](void* dst) {
            uint64_t v = ep;
            memcpy(dst, &v, 8);
        };
        assignSchema(m);
        registry.addSignal(std::move(m));
    }

    /* the whole program blob as one streamable Binary signal —
       clients slice offsets locally via plc.dbg.fields            */
    if (want_blob_sig) {
        std::vector<uint8_t> st;
        tplcio::CtlClient::Error e;
        uint64_t blob = 0;
        if (g_ctl.call(TPLC_OP_DBG_STATE, st, &e) && st.size() >= 24)
            memcpy(&blob, st.data() + 16, 8);
        if (blob && blob <= (1u << 20)) {
            SignalEntry b;
            b.id = g_sigId++;
            b.name = "blob";
            b.description =
                "whole program blob (post-scan boundary image)";
            b.group = "plc";
            b.valueType = ValueType::Binary;
            /* schema-wire Bytes = varint len + payload; advertise the
               encoded size                                              */
            b.maxValueSize = (uint16_t)(blob + 10 > 0xFFFF
                                            ? 0xFFFF : blob + 10);
            b.varReadFn = [](void* dst, size_t maxLen) -> size_t {
                W w; w.u64(0);
                w.u32((uint32_t)(maxLen > 10 ? maxLen - 10 : 0));
                std::vector<uint8_t> out;
                tplcio::CtlClient::Error e;
                if (!g_ctl.call(TPLC_OP_DBG_READ, w.b.data(), 12,
                                out, &e))
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
    }

    for (auto& f : buildFunctions())
        registry.addFunction(std::move(f));

    /* dynamic plc.<prog>.<var> entries for the active program      */
    TplcCtlProgStatus ps{};
    std::vector<uint8_t> raw;
    bool haveProg = progStatus(ps, raw) && ps.name[0];
    if (haveProg) {
        size_t nl = strnlen((const char*)ps.name, TPLC_PROG_NAME_MAX);
        std::string prog((const char*)ps.name, nl);
        auto fields = fetchPlcFields();
        /* a deploy swaps stop→prepare→start inside one watchdog
           tick; if the epoch bump landed mid-swap the layout is
           briefly absent and DBG_FIELDS errors — retry briefly so
           the rebuilt catalog doesn't lose every plc.* field      */
        for (int i = 0; fields.empty() && i < 5; i++) {
            struct timespec ts{0, 200 * 1000 * 1000};
            nanosleep(&ts, nullptr);
            fields = fetchPlcFields();
        }
        fieldsMissing = fields.empty();
        if (!fields.empty()) {
            auto types = typcTypesForProgram(ps, packagesDir, pkgFiles);
            addPlcFieldEntries(registry, prog, fields, types);
        }
    }
    return !haveProg || fieldsMissing;
}

int main(int argc, char** argv) {
    std::string ctl_path, bind = "127.0.0.1", packages_dir;
    std::vector<std::string> pkg_files;
    uint16_t port = 43770;
    bool want_blob_sig = true;
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        if (a == "--ctl")       ctl_path = argv[++i];
        else if (a == "--port") port = (uint16_t)atoi(argv[++i]);
        else if (a == "--bind") bind = argv[++i];
        else if (a == "--packages") packages_dir = argv[++i];
        else if (a == "--pkg")   pkg_files.push_back(argv[++i]);
        else if (a == "--no-blob-signal") want_blob_sig = false;
        else { usage(argv[0]); return 2; }
    }
    if (ctl_path.empty()) { usage(argv[0]); return 2; }

    signal(SIGINT, onSig);
    signal(SIGTERM, onSig);

    if (!g_ctl.connectSock(ctl_path, 10000)) {
        fprintf(stderr, "tplc-io: ctl connect %s failed\n",
                ctl_path.c_str());
        return 1;
    }

    buildSchema();
    Registry registry;

    TplcCtlProgStatus ps0{};
    std::vector<uint8_t> raw0;
    uint64_t last_epoch = 0;
    if (progStatus(ps0, raw0)) last_epoch = ps0.epoch;
    /* field_retry_deadline (nowUs): while set, the watchdog polls
       DBG_FIELDS cheaply and rebuilds once the layout reappears —
       covers deploy swaps that outlast the in-populate retry      */
    uint64_t field_retry_deadline =
        populateRegistry(registry, last_epoch, want_blob_sig,
                         packages_dir, pkg_files)
            ? nowUs() + 15000000
            : 0;

    ServerConfig cfg;
    cfg.timestampFn = nowUs;
    cfg.maxClients = 8;
    cfg.schemaCatalog = &g_catalog;
    auto mkServer = [&]() {
        auto tsrv = std::make_unique<TcpTransportServer>(port, bind);
        return std::make_unique<Server>(registry, std::move(tsrv), cfg);
    };
    auto server = mkServer();
    if (!server->start()) {
        fprintf(stderr, "tplc-io: listen %s:%u failed\n",
                bind.c_str(), port);
        return 1;
    }
    fprintf(stderr,
            "tplc-io: tether_io on %s:%u → %s "
            "(%u functions, %u signals)\n",
            bind.c_str(), port, ctl_path.c_str(),
            (unsigned)(g_fnId - 100), (unsigned)(g_sigId - 200));

    /* management-epoch → catalog rebuild: when the runtime's
       program-set epoch changes, re-install the schema catalog
       (epoch bump → stale client epochs are rejected), rebuild the
       registry (dynamic plc.* fields track the new program), and
       rebind the listener so no session ever observes a half-built
       catalog. Clients see a brief disconnect, then CatalogChanged. */
    while (!g_stop) {
        /* ctl link watchdog: reconnect keeps the surface live
           across runtime restarts                                   */
        if (!g_ctl.connected()) {
            if (!g_ctl.connectSock(ctl_path, 2000))
                fprintf(stderr, "tplc-io: ctl reconnect failed\n");
        } else {
            TplcCtlProgStatus ps{};
            std::vector<uint8_t> raw;
            auto rebuild = [&](uint64_t epoch, const char* why) {
                g_catalog.install(g_graph, plcManifest(g_graph));
                server->stop();
                registry.clear();
                field_retry_deadline =
                    populateRegistry(registry, epoch, want_blob_sig,
                                     packages_dir, pkg_files)
                        ? nowUs() + 15000000
                        : 0;
                if (field_retry_deadline)
                    fprintf(stderr,
                            "tplc-io: catalog incomplete, "
                            "retrying fields for 15s\n");
                server = mkServer();
                if (server->start()) {
                    fprintf(stderr, "tplc-io: %s, catalog rebuilt\n",
                            why);
                } else {
                    fprintf(stderr,
                            "tplc-io: rebind %s:%u failed\n",
                            bind.c_str(), port);
                }
            };
            if (progStatus(ps, raw) && ps.epoch != last_epoch) {
                last_epoch = ps.epoch;
                char why[96];
                snprintf(why, sizeof why, "mgmt epoch → %llu",
                         (unsigned long long)ps.epoch);
                rebuild(ps.epoch, why);
            } else if (field_retry_deadline) {
                /* last rebuild caught the program mid-swap; poll the
                   field table cheaply and rebuild once it is back   */
                if (nowUs() >= field_retry_deadline) {
                    field_retry_deadline = 0;
                } else if (!fetchPlcFields().empty()) {
                    field_retry_deadline = 0;
                    rebuild(last_epoch, "program fields restored");
                }
            }
        }
        struct timespec ts{0, 500 * 1000 * 1000};
        nanosleep(&ts, nullptr);
    }
    server->stop();
    return 0;
}
