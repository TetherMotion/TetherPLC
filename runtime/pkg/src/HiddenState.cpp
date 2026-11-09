/* HiddenState.cpp — target_obj hidden-state check.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include "HiddenState.hpp"

#include <tplc/tplc_pkg.h>

#include <cstring>
#include <string>
#include <vector>

namespace tplc {
namespace {

constexpr uint16_t kShNdxCommon = 0xfff2;
constexpr uint16_t kShNdxXindex = 0xffff;

constexpr uint32_t kShtSymtab = 2;
constexpr uint32_t kShtDynsym = 11;
constexpr uint32_t kShtNobits = 8;

constexpr uint64_t kShfWrite = 0x1;
constexpr uint64_t kShfAlloc = 0x2;

uint16_t rd16(const uint8_t* p) {
    return (uint16_t)(p[0] | (p[1] << 8));
}
uint32_t rd32(const uint8_t* p) {
    return (uint32_t)p[0] | ((uint32_t)p[1] << 8) |
           ((uint32_t)p[2] << 16) | ((uint32_t)p[3] << 24);
}
uint64_t rd64(const uint8_t* p) {
    uint64_t lo = rd32(p);
    uint64_t hi = rd32(p + 4);
    return lo | (hi << 32);
}

/* watched = writable + allocated + PROGBITS/NOBITS under a data-family
 * section name. .data.rel.ro and friends carry SHF_WRITE too but are
 * relocated-then-readonly linker output, not program state. */
bool watchedSection(const char* name) {
    static const char* const kPrefix[] = {
        ".data", ".bss", ".sdata", ".sbss", ".tdata", ".tbss",
    };
    for (const char* pre : kPrefix) {
        size_t n = std::strlen(pre);
        if (std::strncmp(name, pre, n) != 0) continue;
        if (std::strstr(name, "rel.ro")) return false;
        char c = name[n];
        return c == '\0' || c == '.';
    }
    return false;
}

/* Generated modules emit no writable globals: every symbol in a
 * watched section must be runtime/crt/linker output. */
bool whitelisted(const char* nm) {
    if (nm[0] == '\0') return true;
    if (std::strncmp(nm, "__tplc_", 7) == 0) return true;
    if (nm[0] == '_' && nm[1] == '_') return true;
    static const char* const kExtra[] = {
        "completed.0", "completed",
        "_DYNAMIC", "_GLOBAL_OFFSET_TABLE_", "_edata", "_end",
        "_etext", "_sdata", "_sdata_start", "_edata", "_gp",
        "_IO_stdin_used", "edata", "end", "data_start",
        "__global_pointer$", "_SDA_BASE_", "_SDA2_BASE_",
        "_bss_start", "_bss_end_", "__bss_end__", "__bss_start__",
        "__executable_start", "__data_start",
        "__init_array_start", "__init_array_end",
        "__fini_array_start", "__fini_array_end",
        "__preinit_array_start", "__preinit_array_end",
        "__gnu_local_gp", "_Jv_RegisterClasses", "__gmon_start__",
        "__libc_csu_init", "__libc_csu_fini", "__libc_start_main",
        "__morecore", "__default_morecore", "__malloc_hook",
        "__free_hook", "__realloc_hook", "__memalign_hook",
        "__after_morecore_hook", "__malloc_initialize_hook",
        "argp_program_version", "argp_program_bug_address",
        "stdin", "stdout", "stderr",
    };
    for (const char* s : kExtra) {
        if (std::strcmp(nm, s) == 0) return true;
    }
    return false;
}

struct Sec {
    uint64_t nameOff, type, flags, offset, size, link;
    uint64_t entsize;
    bool valid = false;
};

struct Sym {
    const char* name;
    uint64_t shndx, size;
};

Status failHidden(const char* nm) {
    std::string msg = "hidden state symbol '";
    msg += nm;
    msg += "' in .data/.bss: program state must live in the state blob";
    return Status::Err(TPLC_E_PKG_HIDDEN_STATE, msg);
}

Status scan(const uint8_t* d, size_t n) {
    if (n < 52 || std::memcmp(d, "\x7f" "ELF", 4) != 0) return Status::Ok();
    if (d[4] != 1 && d[4] != 2) return Status::Ok();  /* not ELF32/64 */
    if (d[5] != 1) return Status::Ok();               /* big-endian */

    bool is64 = d[4] == 2;
    uint64_t shoff, shentsize, shnum, shstrndx;
    if (is64) {
        if (n < 64) return Status::Ok();
        shoff = rd64(d + 40);
        shentsize = rd16(d + 58);
        shnum = rd16(d + 60);
        shstrndx = rd16(d + 62);
    } else {
        shoff = rd32(d + 32);
        shentsize = rd16(d + 46);
        shnum = rd16(d + 48);
        shstrndx = rd16(d + 50);
    }
    if (shentsize < (is64 ? 64u : 40u) || shnum == 0 ||
        shnum > 4096 || shoff >= n)
        return Status::Ok();
    if (shoff + shentsize * shnum > n) return Status::Ok();
    if (shstrndx == kShNdxXindex || shstrndx >= shnum) return Status::Ok();

    auto secAt = [&](uint64_t i) -> Sec {
        const uint8_t* s = d + shoff + i * shentsize;
        Sec r;
        r.nameOff = rd32(s);
        r.type = rd32(s + 4);
        if (is64) {
            r.flags = rd64(s + 8);
            r.offset = rd64(s + 24);
            r.size = rd64(s + 32);
            r.link = rd32(s + 40);
            r.entsize = rd64(s + 56);
        } else {
            r.flags = rd32(s + 8);
            r.offset = rd32(s + 16);
            r.size = rd32(s + 20);
            r.link = rd32(s + 24);
            r.entsize = rd32(s + 36);
        }
        r.valid = true;
        return r;
    };

    /* section name string table */
    Sec shstr = secAt(shstrndx);
    if (shstr.offset >= n || shstr.size > n - shstr.offset)
        return Status::Ok();
    const char* shstrtab = (const char*)d + shstr.offset;

    auto secName = [&](const Sec& s) -> const char* {
        if (s.nameOff >= shstr.size) return "";
        const char* nm = shstrtab + s.nameOff;
        /* ensure NUL within the string table */
        const char* end = shstrtab + shstr.size;
        if (std::memchr(nm, '\0', (size_t)(end - nm)) == nullptr)
            return "";
        return nm;
    };

    /* watched section flags */
    std::vector<uint8_t> watched(shnum, 0);
    for (uint64_t i = 0; i < shnum; ++i) {
        Sec s = secAt(i);
        if (!(s.flags & kShfWrite) || !(s.flags & kShfAlloc)) continue;
        if (s.type != 1 /*PROGBITS*/ && s.type != kShtNobits) continue;
        if (!watchedSection(secName(s))) continue;
        watched[i] = 1;
    }
    bool anyWatched = false;
    for (uint8_t w : watched) anyWatched |= (w != 0);
    if (!anyWatched) return Status::Ok();

    /* iterate symbol tables */
    for (uint64_t i = 0; i < shnum; ++i) {
        Sec tab = secAt(i);
        if (tab.type != kShtSymtab && tab.type != kShtDynsym) continue;
        if (tab.link >= shnum || tab.offset >= n ||
            tab.size > n - tab.offset)
            continue;
        Sec strs = secAt(tab.link);
        if (strs.type != 3 /*STRTAB*/ || strs.offset >= n ||
            strs.size > n - strs.offset)
            continue;
        const uint8_t* symtab = d + tab.offset;
        const char* strtab = (const char*)d + strs.offset;
        const char* strEnd = strtab + strs.size;
        uint64_t esz = tab.entsize ? tab.entsize
                                  : (is64 ? 24 : 16);
        if (esz < (is64 ? 24u : 16u) || esz > 4096) continue;
        uint64_t count = tab.size / esz;
        if (tab.size % esz != 0 || count > (1u << 22)) continue;
        for (uint64_t j = 0; j < count; ++j) {
            const uint8_t* sp = symtab + j * esz;
            uint32_t nameOff = rd32(sp);
            uint64_t shndx, size;
            if (is64) {
                shndx = rd16(sp + 6);
                size = rd64(sp + 16);
            } else {
                shndx = rd16(sp + 14);
                size = rd32(sp + 8);
            }
            if (size == 0 || shndx == kShNdxXindex) continue;
            bool inWatched = shndx == kShNdxCommon ||
                             (shndx < shnum && watched[shndx]);
            if (!inWatched) continue;
            const char* nm = "";
            if (nameOff < strs.size) {
                nm = strtab + nameOff;
                if (std::memchr(nm, '\0',
                                (size_t)(strEnd - nm)) == nullptr)
                    nm = "";
            }
            if (whitelisted(nm)) continue;
            return failHidden(nm);
        }
    }
    return Status::Ok();
}

} /* namespace */

Status pkgCheckHiddenState(const uint8_t* obj, size_t size) {
    return scan(obj, size);
}

} /* namespace tplc */
