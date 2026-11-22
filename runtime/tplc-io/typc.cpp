#include "tplc_io_internal.hpp"
#include <dirent.h>

namespace tplcio_int {

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
                   size_t cap) {
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

} // namespace tplcio_int
