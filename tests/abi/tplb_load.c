/* tplb_load.c — TPLB v1 loadable-blob loader (reference
 * implementation of the mini-loader contract documented in
 * docs/plc/mcu-path.md). See tplb_load.h for the API contract.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include "tplb_load.h"

#include <string.h>

static uint16_t rd16(const uint8_t* p) {
    uint16_t v;
    memcpy(&v, p, 2);
    return v;
}
static uint32_t rd32(const uint8_t* p) {
    uint32_t v;
    memcpy(&v, p, 4);
    return v;
}
static void wr16(uint8_t* p, uint16_t v) { memcpy(p, &v, 2); }
static void wr32(uint8_t* p, uint32_t v) { memcpy(p, &v, 4); }

static const char* cstr_at(const uint8_t* strtab, size_t strtab_len,
                           uint32_t off, size_t* out_len) {
    if (off >= strtab_len)
        return 0;
    const uint8_t* s = strtab + off;
    const uint8_t* e = memchr(s, 0, strtab_len - off);
    if (!e)
        return 0;
    *out_len = (size_t)(e - s);
    return (const char*)s;
}

/* blob import index → caller-resolved address; NULL + *err on miss */
static uintptr_t resolve_import(const uint8_t* strtab, size_t strtab_len,
                                const uint8_t* irec,
                                const TplbImport* imports,
                                uint32_t nimports, const char** err) {
    uint32_t name_off = rd32(irec);
    size_t nl;
    const char* name = cstr_at(strtab, strtab_len, name_off, &nl);
    if (!name) {
        *err = "tplb: import name out of bounds";
        return 0;
    }
    for (uint32_t i = 0; i < nimports; i++) {
        if (strlen(imports[i].name) == nl &&
            memcmp(imports[i].name, name, nl) == 0)
            return imports[i].addr;
    }
    *err = "tplb: unresolved import";
    return 0;
}

/* Thumb-2 BL pair encode — inverse of the decode in elf_blob.rs */
static void thm_call_pack(uint8_t* place, int32_t imm) {
    uint32_t u = (uint32_t)imm & 0x1ffffffu;
    uint32_t s = (u >> 24) & 1;
    uint32_t i1 = (u >> 23) & 1;
    uint32_t i2 = (u >> 22) & 1;
    uint32_t j1 = 1 - (i1 ^ s);
    uint32_t j2 = 1 - (i2 ^ s);
    uint16_t hw1 = (uint16_t)(0xf000u | (s << 10) | ((u >> 12) & 0x3ff));
    uint16_t hw2 = (uint16_t)(0xd000u | (j1 << 13) | (j2 << 11) |
                             ((u >> 1) & 0x7ff));
    wr16(place, hw1);
    wr16(place + 2, hw2);
}

const char* tplb_load(const void* blob_, size_t len, uint8_t* base,
                      size_t cap, const TplbImport* imports,
                      uint32_t nimports, TplbImage* img) {
    const uint8_t* b = (const uint8_t*)blob_;
    if (len < TPLB_HEADER_SIZE || rd32(b) != TPLB_MAGIC)
        return "tplb: bad magic/size";
    if (rd32(b + 4) != TPLB_VERSION)
        return "tplb: unsupported version";
    if (rd32(b + 8) != TPLB_HEADER_SIZE)
        return "tplb: bad header size";
    uint32_t nsec = rd32(b + 12), nsym = rd32(b + 16);
    uint32_t nimp = rd32(b + 20), nfix = rd32(b + 24);

    size_t p = TPLB_HEADER_SIZE;
    const uint8_t* srec = b + p;
    p += (size_t)nsec * 24;
    const uint8_t* yrec = b + p;
    p += (size_t)nsym * 16;
    const uint8_t* irec = b + p;
    p += (size_t)nimp * 8;
    const uint8_t* frec = b + p;
    p += (size_t)nfix * 16;
    if (p > len)
        return "tplb: records past end";

    /* strtab runs until the first section's load_off */
    uint32_t first_load = 0xffffffffu, blob_end = 0, max_align = 1;
    for (uint32_t i = 0; i < nsec; i++) {
        const uint8_t* s = srec + i * 24;
        uint32_t lo = rd32(s + 4), sz = rd32(s + 8), al = rd32(s + 12);
        if (lo < first_load)
            first_load = lo;
        if (al > max_align)
            max_align = al;
        if ((uint64_t)lo + sz > len || (uint64_t)lo + sz > cap)
            return "tplb: section past image";
        if (al != 0 && (al & (al - 1)) != 0)
            return "tplb: bad section alignment";
        if (lo + sz > blob_end)
            blob_end = lo + sz;
    }
    if (nsec == 0 || first_load == 0xffffffffu || first_load > len ||
        first_load < p)
        return "tplb: bad section layout";
    if (((uintptr_t)base & (max_align - 1)) != 0)
        return "tplb: base misaligned";

    const uint8_t* strtab = b + p;
    size_t strtab_len = first_load - (uint32_t)p;

    /* place section contents */
    for (uint32_t i = 0; i < nsec; i++) {
        const uint8_t* s = srec + i * 24;
        uint32_t lo = rd32(s + 4), sz = rd32(s + 8);
        memcpy(base + lo, b + lo, sz);
    }

    /* apply fixups */
    for (uint32_t i = 0; i < nfix; i++) {
        const uint8_t* f = frec + i * 16;
        uint32_t place_off = rd32(f);
        uint32_t ftype = rd32(f + 4);
        uint32_t fval = rd32(f + 8);
        int32_t addend = (int32_t)rd32(f + 12);
        if ((uint64_t)place_off + 8 > cap)
            return "tplb: fixup place out of image";
        uint8_t* place = base + place_off;

        uintptr_t s_addr;
        if (fval & TPLB_F_IMPORT) {
            const char* err = 0;
            uint32_t iidx = fval & 0x7fffffffu;
            if (iidx >= nimp)
                return "tplb: import index out of range";
            s_addr = resolve_import(strtab, strtab_len,
                                    irec + iidx * 8, imports, nimports,
                                    &err);
            if (err)
                return err;
        } else {
            s_addr = (uintptr_t)(base + fval);
        }

        switch (ftype) {
        case TPLB_F_ABS32:
            wr32(place, (uint32_t)(s_addr + (uintptr_t)addend));
            break;
        case TPLB_F_ABS64: {
            uint64_t v = (uint64_t)s_addr + (int64_t)addend;
            memcpy(place, &v, 8);
            break;
        }
        case TPLB_F_MOVW:
        case TPLB_F_MOVT: {
            uint32_t v = (uint32_t)(s_addr + (uintptr_t)addend);
            uint16_t imm16 = (uint16_t)(ftype == TPLB_F_MOVW
                                            ? (v & 0xffffu)
                                            : ((v >> 16) & 0xffffu));
            uint16_t hw1 = rd16(place), hw2 = rd16(place + 2);
            hw1 = (uint16_t)((hw1 & ~0xfu) | ((imm16 >> 12) & 0xf));
            hw1 = (uint16_t)((hw1 & ~(1u << 10)) |
                             (((imm16 >> 11) & 1) << 10));
            hw2 = (uint16_t)((hw2 & ~(7u << 12)) |
                             (((imm16 >> 8) & 7) << 12));
            hw2 = (uint16_t)((hw2 & ~0xffu) | (imm16 & 0xff));
            wr16(place, hw1);
            wr16(place + 2, hw2);
            break;
        }
        case TPLB_F_THM_CALL: {
            int64_t d = (int64_t)s_addr + addend -
                        (int64_t)(uintptr_t)place;
            if (d < -0x1000000 || d >= 0x1000000)
                return "tplb: THM_CALL target out of ±16MiB";
            thm_call_pack(place, (int32_t)(d & ~(int64_t)1));
            break;
        }
        case TPLB_F_REL32: {
            int64_t d = (int64_t)s_addr + addend -
                        (int64_t)(uintptr_t)place;
            wr32(place, (uint32_t)d);
            break;
        }
        default:
            return "tplb: unknown fixup type";
        }
    }

    img->base = base;
    img->size = blob_end;
    img->nsyms = nsym;
    img->syms = yrec;
    img->strtab = strtab;
    img->strtab_len = (uint32_t)strtab_len;
    return 0;
}

uintptr_t tplb_lookup(const TplbImage* img, const char* name) {
    size_t nl = strlen(name);
    for (uint32_t i = 0; i < img->nsyms; i++) {
        const uint8_t* y = img->syms + i * 16;
        uint32_t name_off = rd32(y);
        uint32_t value_off = rd32(y + 4);
        size_t sl;
        const char* s = cstr_at(img->strtab, img->strtab_len, name_off, &sl);
        if (s && sl == nl && memcmp(s, name, nl) == 0)
            return (uintptr_t)(img->base + value_off);
    }
    return 0;
}
