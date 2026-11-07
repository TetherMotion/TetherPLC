#include <tplc/bridge/CatalogExporter.hpp>
#include <cstring>
#include <cstdio>

namespace tplc {

/* ---- wire formats (compiler/tplc-mir/src/desc.rs) -----------------
   SMAP: 32B hdr {magic,ver,count,strtab_off,strtab_len,tdbase,flags}
         + count × 64B {name_off,kind,flags,region,access,type_ref,
           offset:u64,size,align,parent,first_child,next_sib,prov,pad}
   TYPC: 24B hdr {magic,ver,count,mtab_off,strtab_off,pad}
         + count × 56B {tag,pad,dims,name,elem,ms,mc,lo,hi,size,align}
         + enum tails + member table 16B rows {name,ty,off,flags}    */

static uint32_t le32(const uint8_t* p) {
    uint32_t v; memcpy(&v, p, 4); return v;
}
static uint64_t le64(const uint8_t* p) {
    uint64_t v; memcpy(&v, p, 8); return v;
}

static uint64_t fnv1a(const std::string& s) {
    uint64_t h = 1469598103934665603ull;
    for (unsigned char c : s) { h ^= c; h *= 1099511628211ull; }
    return h;
}

static const char* vt_name(uint8_t t) {
    static const char* N[] = {
        "none","u8","u16","u32","u64","i8","i16","i32","i64",
        "f32","f64","bool","string","binary","ipv4","ipv6","mac",
        "enum","uvarint","ivarint","struct","array","stream",
        "fixedstr" };
    return t < sizeof(N)/sizeof(N[0]) ? N[t] : "?";
}

/* TYPC tag → tether_io ValueType */
static uint8_t vt_of(uint8_t tag) {
    switch (tag) {
        case 0:  return 11; /* Bool   */
        case 1:  return 5;  /* I8     */
        case 2:  return 6;  /* I16    */
        case 3:  return 7;  /* I32    */
        case 4:  return 8;  /* I64    */
        case 5:  return 1;  /* U8     */
        case 6:  return 2;  /* U16    */
        case 7:  return 3;  /* U32    */
        case 8:  return 4;  /* U64    */
        case 9:  return 9;  /* F32    */
        case 10: return 10; /* F64    */
        case 11: return 8;  /* TIME → i64 ns */
        case 15: return 12; /* String */
        case 18: return 21; /* Array  */
        case 19: return 20; /* Struct */
        case 20: return 17; /* Enum   */
        case 21: return 20; /* FB → struct */
        default: return 13; /* Binary */
    }
}

struct Typc {
    const uint8_t* base;
    uint32_t count;
    const uint8_t* entries;   /* count × 56B                      */
    const uint8_t* strs;
    uint32_t strs_end;

    const char* str(uint32_t off) const {
        const uint8_t* p = strs + off;
        const uint8_t* e = strs + strs_end;
        return (p < e) ? (const char*)p : "";
    }
    uint8_t tag_of_idx(uint32_t i) const {
        return i < count ? entries[i * 56] : 0xFF;
    }
    uint32_t elem_of_idx(uint32_t i) const {
        return i < count ? le32(entries + i * 56 + 8) : 0xFFFFFFFFu;
    }
    const char* name_of_idx(uint32_t i) const {
        if (i >= count) return "";
        uint32_t no = le32(entries + i * 56 + 4);
        return no ? str(no) : "";
    }
};

static bool parse_typc(const uint8_t* t, uint32_t len, Typc& out) {
    if (len < 24 || memcmp(t, "TYPC", 4) != 0) return false;
    out.count   = le32(t + 8);
    uint32_t mto = le32(t + 12);         /* member table off        */
    uint32_t sto = le32(t + 16);         /* strtab off              */
    if (sto >= len || mto >= len) return false;
    out.base     = t;
    out.entries  = t + 24;
    out.strs     = t + sto;
    out.strs_end = len - sto;
    return true;
}

Result<std::vector<CatalogEntry>> catalog_export(
    const uint8_t* smap, uint32_t smap_len,
    const uint8_t* typc, uint32_t typc_len,
    const std::string& prog_name,
    const std::vector<std::string>& task_names) {
    if (smap_len < 32 || memcmp(smap, "SMAP", 4) != 0)
        return Result<std::vector<CatalogEntry>>::Err(
            Status::Err(-1, "bad SMAP"));
    uint32_t count = le32(smap + 8);
    uint32_t strtab_off = le32(smap + 12);
    uint32_t strtab_len = le32(smap + 16);
    if (strtab_off + strtab_len > smap_len ||
        32 + count * 64 > smap_len)
        return Result<std::vector<CatalogEntry>>::Err(
            Status::Err(-1, "SMAP out of range"));
    const uint8_t* ents = smap + 32;
    const uint8_t* strs = smap + strtab_off;
    auto str = [&](uint32_t off) -> const char* {
        return off < strtab_len ? (const char*)(strs + off) : "";
    };
    Typc tc{};
    bool have_tc = typc && parse_typc(typc, typc_len, tc);

    std::vector<CatalogEntry> out;
    std::string base = "plc." + prog_name;

    for (uint32_t i = 0; i < count; i++) {
        const uint8_t* e = ents + i * 64;
        uint8_t kind   = e[4];
        uint8_t flags  = e[5];
        uint8_t region = e[6];
        const char* qn = str(le32(e));
        if (!*qn) continue;

        uint8_t vt = 13, et = 0;
        if (have_tc) {
            uint32_t ti = le32(e + 8);
            vt = vt_of(tc.tag_of_idx(ti));
            if (vt == 21) {           /* array → element type        */
                uint32_t ei = tc.elem_of_idx(ti);
                et = ei != 0xFFFFFFFFu ? vt_of(tc.tag_of_idx(ei)) : 0;
            }
        }
        bool retain = (flags & 1) || region == 3;

        if (kind == 10) {             /* io channel                  */
            const char* sp = region == 1 ? "I" : region == 2 ? "Q" : "M";
            CatalogEntry ce;
            ce.name = "plc.image." + std::string(sp) + ".ch." + qn;
            ce.group = "plc.image";
            ce.kind = "signal";
            ce.value_type = vt;
            ce.element_type = et;
            ce.description = "io channel";
            ce.id = fnv1a(ce.name);
            out.push_back(std::move(ce));
            continue;
        }
        if (kind != 1 && kind != 7) continue;

        CatalogEntry ce;
        if (retain) {
            ce.name = base + ".retain." + qn;
            ce.writable = false;
        } else {
            ce.name = base + "." + qn;
            ce.writable = true;
        }
        ce.group = base;
        ce.kind = "param";
        ce.value_type = vt;
        ce.element_type = et;
        ce.description = (kind == 7) ? "fb instance" : "var";
        ce.id = fnv1a(ce.name);
        out.push_back(std::move(ce));
    }

    /* plc.<prog>.task.<name>.stats — counter group per task        */
    static const char* stat_fields[] = {
        "runs", "late", "faults", "overruns", "exec_max_ns" };
    for (auto& tn : task_names) {
        for (auto f : stat_fields) {
            CatalogEntry ce;
            ce.name = base + ".task." + tn + ".stats." + f;
            ce.group = base + ".task." + tn;
            ce.kind = "signal";
            ce.value_type = 4;        /* u64 */
            ce.description = "task stat";
            ce.id = fnv1a(ce.name);
            out.push_back(std::move(ce));
        }
    }

    /* plc.runtime.* — fixed runtime counters (doc 09 §6)           */
    static const char* rt[] = {
        "uptime_s", "prog_count", "running_count", "cycles",
        "retain_flushes", "retain_losses" };
    for (auto f : rt) {
        CatalogEntry ce;
        ce.name = "plc.runtime." + std::string(f);
        ce.group = "plc.runtime";
        ce.kind = "signal";
        ce.value_type = 4;
        ce.description = "runtime counter";
        ce.id = fnv1a(ce.name);
        out.push_back(std::move(ce));
    }
    return Result<std::vector<CatalogEntry>>::Ok(std::move(out));
}

std::string catalog_json(const std::vector<CatalogEntry>& entries) {
    std::string j = "[\n";
    char buf[1024];
    for (size_t i = 0; i < entries.size(); i++) {
        auto& e = entries[i];
        snprintf(buf, sizeof buf,
            "  {\"id\":%llu,\"name\":\"%s\",\"group\":\"%s\","
            "\"kind\":\"%s\",\"type\":\"%s\",\"elem\":\"%s\","
            "\"rw\":%s,\"desc\":\"%s\"}%s\n",
            (unsigned long long)e.id, e.name.c_str(), e.group.c_str(),
            e.kind.c_str(), vt_name(e.value_type),
            vt_name(e.element_type),
            e.writable ? "true" : "false", e.description.c_str(),
            i + 1 < entries.size() ? "," : "");
        j += buf;
    }
    return j + "]\n";
}

} /* namespace tplc */
