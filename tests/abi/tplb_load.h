/* tplb_load.h — TPLB v1 loadable-blob loader (reference
 * implementation of the mini-loader contract documented in
 * docs/plc/mcu-path.md).
 *
 * A TPLB blob is produced by `tplcc build --target <arm-triple>
 * --pic-blob`: LLVM's ET_REL object is relocated at emit time
 * (PC-relative fields) and shipped with a fixup table covering
 * absolute references and undefined-symbol imports. The loader
 * memcpy's section contents to a caller-chosen RAM base, resolves
 * imports by name, and applies the fixups — no target linker.
 *
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLB_LOAD_H
#define TPLB_LOAD_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLB_MAGIC   0x424c5054u /* 'TPLB' little-endian */
#define TPLB_VERSION 1u
#define TPLB_HEADER_SIZE 32u

/* section record flags */
#define TPLB_SEC_WRITE 1u
#define TPLB_SEC_EXEC  2u

/* fixup types (wire contract, must match elf_blob.rs) */
#define TPLB_F_ABS32    1u
#define TPLB_F_ABS64    2u
#define TPLB_F_MOVW     3u
#define TPLB_F_MOVT     4u
#define TPLB_F_THM_CALL 5u
#define TPLB_F_REL32    6u
/* fixup.value flag: low 31 bits index the import table */
#define TPLB_F_IMPORT   0x80000000u

typedef struct {
    const char* name; /* import symbol name (e.g. "__adddf3") */
    uintptr_t   addr; /* resolved address the fixups use */
} TplbImport;

typedef struct {
    uint8_t*  base;      /* load base the blob was placed at */
    uint32_t  size;      /* end offset of last section (blob span) */
    uint32_t  nsyms;
    const uint8_t* syms;     /* nsyms × 16-byte symbol records */
    const uint8_t* strtab;   /* NUL-separated name table */
    uint32_t  strtab_len;
} TplbImage;

/* Parse + validate the header, copy section contents to `base`
 * (which must satisfy every section's alignment and have `cap`
 * bytes available), resolve imports by name against `imports`,
 * and apply all fixups.
 *
 * `imports`/`nimports`: caller-supplied name→addr table covering
 * every import the blob references; entries are matched by name
 * and their `addr` is used verbatim (function pointers keep the
 * Thumb bit on Cortex-M). Returns NULL on success, else a static
 * error string. */
const char* tplb_load(const void* blob, size_t len, uint8_t* base,
                      size_t cap, const TplbImport* imports,
                      uint32_t nimports, TplbImage* img);

/* Look up an exported symbol by name; returns its runtime
 * address (state bit preserved) or 0 when absent. */
uintptr_t tplb_lookup(const TplbImage* img, const char* name);

#ifdef __cplusplus
}
#endif
#endif /* TPLB_LOAD_H */
