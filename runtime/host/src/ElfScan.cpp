/* ElfScan.cpp — post-link module scanner (doc 15 §4.3).
 *
 * Static scan of an ELF shared object before dlopen. Checks:
 *   - ELF64 little-endian ET_DYN, e_machine in {x86-64, aarch64}
 *   - no PT_LOAD with both W and X (RWX), no executable GNU_STACK,
 *     no PT_INTERP
 *   - DT_NEEDED restricted to a libc/libm/rt whitelist
 *   - no syscall/sysenter/sysret/sysexit/int/int1/into opcodes at
 *     instruction boundaries inside executable sections
 *
 * The x86-64 check uses a minimal length decoder (LDE) over
 * STT_FUNC symbol ranges from .symtab/.dynsym so that forbidden byte
 * patterns inside immediates (e.g. `mov eax,0x80cd`) do not
 * false-positive. Executable bytes not covered by any symbol (PLT,
 * padding) are scanned literally for the forbidden encodings — a
 * byte-level match there is a reject, since no boundary can be
 * proven. If the LDE desyncs, the remainder is scanned literally.
 * aarch64 instructions are fixed 4-byte, so a direct svc scan is
 * exact. SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/ElfScan.hpp>
#include <tplc/tplc_pkg.h>

#include <cstdio>
#include <cstring>
#include <vector>
#include <algorithm>

namespace tplc {
namespace {

enum : uint16_t { EM_X86_64 = 62, EM_AARCH64 = 183 };

struct Ehdr {
    uint8_t  ident[16];
    uint16_t type, machine;
    uint32_t version;
    uint64_t entry, phoff, shoff;
    uint32_t flags;
    uint16_t ehsize, phentsize, phnum, shentsize, shnum, shstrndx;
};
struct Phdr {
    uint32_t type, flags;
    uint64_t off, vaddr, paddr, filesz, memsz, align;
};
struct Shdr {
    uint32_t name, type;
    uint64_t flags, addr, offset, size;
    uint32_t link, info;
    uint64_t addralign, entsize;
};
struct Dyn { int64_t tag; uint64_t val; };
struct Sym { uint32_t name; uint8_t info, other; uint16_t shndx; uint64_t value, size; };

const uint8_t* rd(const uint8_t* base, size_t size,
                  uint64_t off, uint64_t n) {
    if (off + n < n || off + n > size) return nullptr;
    return base + off;
}

/* ---- x86-64 minimal length decoder ---------------------------------- */

/* Returns instruction length or -1 on undecodable/truncated.
 * `forbidden` is set when the decoded opcode is a gate to the kernel. */
int x86Len(const uint8_t* p, size_t n, bool& forbidden) {
    size_t i = 0;
    bool rexW = false, op16 = false;
    for (;;) {
        if (i >= n) return -1;
        uint8_t b = p[i];
        if (b == 0x67) { i++; continue; }
        if (b == 0x66) { op16 = true; i++; continue; }
        if (b == 0xF0 || b == 0xF2 || b == 0xF3 ||
            b == 0x2E || b == 0x36 || b == 0x3E || b == 0x26 ||
            b == 0x64 || b == 0x65) { i++; continue; }
        if (b >= 0x40 && b <= 0x4F) { rexW = (b & 8) != 0; i++; continue; }
        break;
    }
    uint8_t op = p[i++];
    bool modrm = true;
    int imm = 0, disp = 0;
    /* op16 (0x66 prefix) shrinks imm32→imm16 unless REX.W keeps the
       operand at 64 bits */
    const int imm32 = (op16 && !rexW) ? 2 : 4;

    if (op == 0x0F) {
        if (i >= n) return -1;
        uint8_t o2 = p[i++];
        if (o2 == 0x05 || o2 == 0x07 || o2 == 0x34 || o2 == 0x35) {
            forbidden = true;           /* syscall/sysret/sysenter/sysexit */
            return (int)i;
        }
        if (o2 >= 0x80 && o2 <= 0x8F) {              /* jcc rel32: no modrm */
            modrm = false;
            imm = 4;
        } else if (o2 == 0x38 || o2 == 0x3A) {
            if (i >= n) return -1;
            i++;                        /* map-3 opcode byte */
            imm = (o2 == 0x3A) ? 1 : 0;
        } else {
            static const uint8_t noM[] = {0x06,0x08,0x09,0x0B,0x0E,0x30,0x31,
                0x32,0x33,0x36,0x37,0x77,0xA0,0xA1,0xA8,0xA9};
            static const uint8_t im8[] = {0x70,0x71,0x72,0x73,0xA4,0xAC,0xBA,
                0xC2,0xC4,0xC5,0xC6};
            auto hit = [](const uint8_t* s, size_t m, uint8_t c) {
                return std::memchr(s, (char)c, m) != nullptr;
            };
            if ((o2 >= 0xC8 && o2 <= 0xCF) || hit(noM, sizeof noM, o2))
                modrm = false;
            else if (hit(im8, sizeof im8, o2)) imm = 1;
        }
    } else if (op == 0xC4 || op == 0xC5) {          /* VEX3/VEX2 */
        if (i >= n) return -1;
        uint8_t map = p[i++] & 0x1F;                 /* last payload byte */
        if (i >= n) return -1;
        uint8_t o2 = p[i++];                         /* real opcode */
        static const uint8_t vi8[] = {0x70,0x71,0x72,0x73,0xC2,0xC4,0xC5,0xC6};
        if (map == 3 || std::memchr(vi8, (char)o2, sizeof vi8))
            imm = 1;                                 /* 0F3A map / imm8 ops */
    } else {
        auto in = [](uint8_t a, uint8_t lo, uint8_t hi) { return a >= lo && a <= hi; };
        /* no-modrm opcodes */
        if (op <= 0x3F)
            modrm = (op & 7) < 4;                    /* ALU grp / prefix-class */
        else if (in(op, 0x50, 0x62) || in(op, 0x6C, 0x6F) ||
                 in(op, 0x70, 0x7F) || in(op, 0x90, 0x9F) ||
                 in(op, 0xA0, 0xAF) || in(op, 0xB0, 0xBF) ||
                 in(op, 0xE0, 0xEF) || in(op, 0xF8, 0xFD) ||
                 op == 0xC2 || op == 0xC3 || op == 0xC9 ||
                 op == 0xCA || op == 0xCB || op == 0xCC || op == 0xCD ||
                 op == 0xCE || op == 0xCF ||
                 op == 0xF1 || op == 0xF4 || op == 0xF5 ||
                 op == 0x68 || op == 0x6A ||
                 in(op, 0xD4, 0xD7))
            modrm = false;
        if (op == 0xCD || op == 0xCE || op == 0xF1) forbidden = true; /* int/into/int1 */
        /* immediate widths */
        if (op == 0x6A || op == 0xA8 || in(op, 0xB0, 0xB7) ||
            op == 0xCD || in(op, 0xD4, 0xD5) ||
            in(op, 0x70, 0x7F) || in(op, 0xE0, 0xE7) || op == 0xEB)
            imm = 1;
        else if (op == 0xC2 || op == 0xCA) imm = 2;  /* ret/retf imm16 */
        else if (op == 0xC8) imm = 3;                /* enter */
        else if (op == 0xE8 || op == 0xE9) imm = 4;   /* call/jmp rel32 */
        else if (op == 0x68) imm = op16 ? 2 : 4;     /* push imm */
        else if (op == 0xA9) imm = imm32;            /* test eAX,imm */
        else if (in(op, 0xB8, 0xBF))
            imm = rexW ? 8 : (op16 ? 2 : 4);         /* mov r,imm */
        else if (in(op, 0xA0, 0xA3))
            imm = (op16 && !rexW) ? 4 : 8;           /* mov moffs */
        else if (op == 0x9A || op == 0xEA) imm = 6;        /* far call/jmp */
        else if (op <= 0x3F && (op & 7) == 4) imm = 1;     /* ALU imm8 */
        else if (op <= 0x3F && (op & 7) == 5) imm = imm32; /* ALU eAX,imm */
        /* modrm ops with immediates */
        else if (op == 0x69) imm = imm32;            /* imul imm */
        else if (op == 0x6B || op == 0x80 || op == 0x82 || op == 0x83 ||
                 op == 0xC0 || op == 0xC1 || op == 0xC6) imm = 1;
        else if (op == 0x81 || op == 0xC7) imm = imm32;
        /* 0xF6/0xF7 group-3 imm depends on modrm.reg: handled below */
    }

    if (modrm) {
        if (i >= n) return -1;
        uint8_t m = p[i++];
        int mod = m >> 6, rm = m & 7;
        if (mod != 3 && rm == 4) {                      /* SIB */
            if (i >= n) return -1;
            uint8_t s = p[i++];
            if (mod == 0 && (s & 7) == 5) disp = 4;
        }
        if (mod == 0 && rm == 5) disp = 4;
        else if (mod == 1) disp = 1;
        else if (mod == 2) disp = 4;
        int reg = (m >> 3) & 7;
        if (op == 0xF6 && reg <= 1) imm = 1;
        if (op == 0xF7 && reg <= 1) imm = imm32;
    }
    i += (size_t)disp + (size_t)imm;
    if (i > n) return -1;
    return (int)i;
}

/* literal forbidden byte patterns, for byte-ranges we cannot decode */
bool hasForbiddenBytes(const uint8_t* p, size_t n) {
    for (size_t i = 0; i < n; i++) {
        if (p[i] == 0x0F && i + 1 < n &&
            (p[i + 1] == 0x05 || p[i + 1] == 0x07 ||
             p[i + 1] == 0x34 || p[i + 1] == 0x35))
            return true;
        if (p[i] == 0xCD || p[i] == 0xCE || p[i] == 0xF1) return true;
    }
    return false;
}

/* decode [p,p+n) linearly; on desync fall back to a literal scan of the
 * remainder (found → reject; none → the bytes hold no gate opcodes) */
bool scanExecX86(const uint8_t* p, size_t n, uint64_t va, std::string& why) {
    size_t i = 0;
    while (i < n) {
        bool bad = false;
        int l = x86Len(p + i, n - i, bad);
        if (l <= 0) {
            if (hasForbiddenBytes(p + i, n - i)) {
                why = "kernel-gate bytes past decode desync";
                return false;
            }
            return true;
        }
        if (bad) {
            char buf[80];
            std::snprintf(buf, sizeof buf,
                          "kernel-gate opcode at +0x%zx (va 0x%llx)",
                          i, (unsigned long long)(va + i));
            why = buf;
            return false;
        }
        i += (size_t)l;
    }
    return true;
}

bool scanExecA64(const uint8_t* p, size_t n, std::string& why) {
    for (size_t i = 0; i + 4 <= n; i += 4) {
        uint32_t w;
        std::memcpy(&w, p + i, 4);
        if ((w & 0xFFE0001Fu) == 0xD4000001u) {   /* svc #imm */
            char buf[64];
            std::snprintf(buf, sizeof buf, "svc at +0x%zx", i);
            why = buf;
            return false;
        }
    }
    return true;
}

/* undefined dynamic symbols the module may import — generated code
 * calls abort (panic path) and a small libc/libm surface only */
bool importWhitelisted(const char* s) {
    static const char* ok[] = {
        "abort", "memcpy", "memmove", "memset", "memcmp",
        "strlen", "strnlen", "strcmp", "strncmp",
        "fabs", "floor", "ceil", "trunc", "round", "nearbyint",
        "fmod", "sqrt", "pow", "exp", "log", "log2", "log10",
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
        "sqrtf", "powf", "fabsf", "fmodf", "expf", "logf",
        /* stack-protector + linker-injected weak boilerplate */
        "__stack_chk_fail", "__stack_chk_guard", "__stack_smash_handler",
        "__gmon_start__", "__cxa_finalize", "__tls_get_addr",
        "_ITM_deregisterTMCloneTable", "_ITM_registerTMCloneTable",
    };
    for (const char* w : ok)
        if (std::strcmp(s, w) == 0) return true;
    return false;
}

bool neededWhitelisted(const char* s) {
    static const char* ok[] = {
        "libc.so.6", "libm.so.6", "libdl.so.2", "libpthread.so.0",
        "librt.so.1", "librt.so.2", "libgcc_s.so.1",
        "ld-linux-x86-64.so.2", "ld-linux-aarch64.so.1",
        "linux-vdso.so.1",
    };
    for (const char* w : ok)
        if (std::strcmp(s, w) == 0) return true;
    return false;
}

Status scan(const uint8_t* d, size_t n) {
    if (n < sizeof(Ehdr))
        return Status::Err(TPLC_E_PKG_SYSCALL, "not ELF");
    Ehdr h;
    std::memcpy(&h, d, sizeof h);
    if (std::memcmp(h.ident, "\177ELF", 4) != 0 || h.ident[4] != 2 ||
        h.ident[5] != 1)
        return Status::Err(TPLC_E_PKG_SYSCALL, "not ELF64-LE");
    if (h.type != 3 /*ET_DYN*/)
        return Status::Err(TPLC_E_PKG_SYSCALL, "not a shared object");
    if (h.machine != EM_X86_64 && h.machine != EM_AARCH64)
        return Status::Err(TPLC_E_PKG_SYSCALL, "unsupported e_machine");

    /* program headers */
    for (uint32_t i = 0; i < h.phnum; i++) {
        const uint8_t* pb = rd(d, n, h.phoff + i * (uint64_t)h.phentsize,
                               sizeof(Phdr));
        if (!pb) return Status::Err(TPLC_E_PKG_SYSCALL, "phdr oob");
        Phdr p;
        std::memcpy(&p, pb, sizeof p);
        if (p.type == 1 /*PT_LOAD*/ && (p.flags & 3) == 3)
            return Status::Err(TPLC_E_PKG_SYSCALL, "RWX segment");
        if (p.type == 0x6474E551 /*PT_GNU_STACK*/ && (p.flags & 1))
            return Status::Err(TPLC_E_PKG_SYSCALL, "executable stack");
        if (p.type == 3 /*PT_INTERP*/)
            return Status::Err(TPLC_E_PKG_SYSCALL, "PT_INTERP in module");
    }

    /* section headers required for the audit */
    if (h.shoff == 0 || h.shentsize < sizeof(Shdr) || h.shnum > 512)
        return Status::Err(TPLC_E_PKG_SYSCALL, "no section headers");
    std::vector<Shdr> sh(h.shnum);
    for (uint32_t i = 0; i < h.shnum; i++) {
        const uint8_t* sb = rd(d, n, h.shoff + i * (uint64_t)h.shentsize,
                               sizeof(Shdr));
        if (!sb) return Status::Err(TPLC_E_PKG_SYSCALL, "shdr oob");
        std::memcpy(&sh[i], sb, sizeof(Shdr));
    }

    /* DT_NEEDED whitelist via .dynamic → linked strtab */
    for (const Shdr& s : sh) {
        if (s.type != 6 /*SHT_DYNAMIC*/) continue;
        if (s.link >= sh.size() || sh[s.link].type != 3 /*SHT_STRTAB*/)
            return Status::Err(TPLC_E_PKG_SYSCALL, "dynamic strtab bad");
        const Shdr& st = sh[s.link];
        const uint8_t* dyn = rd(d, n, s.offset, s.size);
        const uint8_t* str = rd(d, n, st.offset, st.size);
        if (!dyn || !str) return Status::Err(TPLC_E_PKG_SYSCALL, "dyn oob");
        for (uint64_t o = 0; o + sizeof(Dyn) <= s.size; o += sizeof(Dyn)) {
            Dyn dy;
            std::memcpy(&dy, dyn + o, sizeof dy);
            if (dy.tag == 0) break;
            if (dy.tag != 1 /*DT_NEEDED*/) continue;
            if (dy.val >= st.size)
                return Status::Err(TPLC_E_PKG_SYSCALL, "needed str oob");
            const char* lib = (const char*)str + dy.val;
            if (!neededWhitelisted(lib))
                return Status::Err(TPLC_E_PKG_SYSCALL,
                                   std::string("DT_NEEDED not whitelisted: ")
                                   + lib);
        }
    }

    /* import whitelist: every undefined dynsym name must be in the
     * libc/libm surface generated code legitimately calls */
    for (const Shdr& s : sh) {
        if (s.type != 11 /*SHT_DYNSYM*/) continue;
        if (s.link >= sh.size() || sh[s.link].type != 3 /*SHT_STRTAB*/)
            return Status::Err(TPLC_E_PKG_SYSCALL, "dynsym strtab bad");
        const Shdr& st = sh[s.link];
        const uint8_t* sp = rd(d, n, s.offset, s.size);
        const uint8_t* str = rd(d, n, st.offset, st.size);
        if (!sp || !str || s.entsize < sizeof(Sym))
            return Status::Err(TPLC_E_PKG_SYSCALL, "dynsym oob");
        for (uint64_t o = 0; o + sizeof(Sym) <= s.size; o += s.entsize) {
            Sym sy;
            std::memcpy(&sy, sp + o, sizeof sy);
            if (sy.shndx != 0 || sy.name == 0) continue;   /* defined/anon */
            if (sy.name >= st.size)
                return Status::Err(TPLC_E_PKG_SYSCALL, "sym name oob");
            const char* nm = (const char*)str + sy.name;
            if (!importWhitelisted(nm))
                return Status::Err(TPLC_E_PKG_SYSCALL,
                                   std::string("import not whitelisted: ")
                                   + nm);
        }
    }

    /* exec sections + STT_FUNC ranges (from .symtab and .dynsym) */
    std::vector<std::vector<std::pair<uint64_t, uint64_t>>> fns(sh.size());
    std::vector<uint16_t> execs;
    for (size_t si = 0; si < sh.size(); si++) {
        const Shdr& s = sh[si];
        if (s.type == 1 /*PROGBITS*/ && (s.flags & 0x4) && s.size)
            execs.push_back((uint16_t)si);
    }
    for (const Shdr& s : sh) {
        if (s.type != 2 && s.type != 11) continue;    /* SYMTAB/DYNSYM */
        if (s.entsize < sizeof(Sym) || s.entsize == 0) continue;
        const uint8_t* sp = rd(d, n, s.offset, s.size);
        if (!sp) continue;
        for (uint64_t o = 0; o + sizeof(Sym) <= s.size; o += s.entsize) {
            Sym sy;
            std::memcpy(&sy, sp + o, sizeof sy);
            if ((sy.info & 0xF) != 2 /*STT_FUNC*/ || sy.size == 0 ||
                sy.shndx == 0 || sy.shndx >= sh.size()) continue;
            fns[sy.shndx].push_back({sy.value, sy.size});
        }
    }

    for (uint16_t si : execs) {
        const Shdr& s = sh[si];
        const uint8_t* body = rd(d, n, s.offset, s.size);
        if (!body) return Status::Err(TPLC_E_PKG_SYSCALL, "section oob");
        std::string why;
        if (h.machine == EM_AARCH64) {
            if (!scanExecA64(body, s.size, why))
                return Status::Err(TPLC_E_PKG_SYSCALL, why);
            continue;
        }
        /* x86-64: decode symbol-covered ranges; literal-scan the rest */
        auto& v = fns[si];
        std::sort(v.begin(), v.end());
        uint64_t cur = s.addr;
        uint64_t end = s.addr + s.size;
        for (auto& f : v) {
            if (f.first < s.addr || f.first >= end) continue;
            uint64_t fb = std::max(f.first, cur);
            uint64_t fe = std::min(f.first + f.second, end);
            if (fb >= fe) continue;
            if (fb > cur && hasForbiddenBytes(body + (cur - s.addr), fb - cur))
                return Status::Err(TPLC_E_PKG_SYSCALL,
                                   "kernel-gate bytes outside symbols");
            if (!scanExecX86(body + (fb - s.addr), fe - fb, fb, why))
                return Status::Err(TPLC_E_PKG_SYSCALL, why);
            cur = fe;
        }
        if (end > cur && hasForbiddenBytes(body + (cur - s.addr), end - cur))
            return Status::Err(TPLC_E_PKG_SYSCALL,
                               "kernel-gate bytes outside symbols");
    }
    return Status();
}

} /* namespace */

Status scanModuleFile(const std::string& path) {
    FILE* f = std::fopen(path.c_str(), "rb");
    if (!f) return Status::Err(TPLC_E_HOST_DLOPEN, "open: " + path);
    std::fseek(f, 0, SEEK_END);
    long sz = std::ftell(f);
    std::fseek(f, 0, SEEK_SET);
    if (sz < 0 || (uint64_t)sz > (64ull << 20)) {
        std::fclose(f);
        return Status::Err(TPLC_E_PKG_SYSCALL, "module size bad");
    }
    std::vector<uint8_t> buf((size_t)sz);
    size_t got = std::fread(buf.data(), 1, buf.size(), f);
    std::fclose(f);
    if (got != buf.size())
        return Status::Err(TPLC_E_PKG_SYSCALL, "short read");
    return scan(buf.data(), buf.size());
}

} /* namespace tplc */
