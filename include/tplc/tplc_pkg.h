/* tplc_pkg.h — .tplc package container constants and error codes.
 * Normative spec: plan doc 01 §2/§13. Shared by the Rust writer
 * (compiler/tplc-pkg) and the C++ reader (runtime/pkg).
 * All fields little-endian. SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_PKG_H
#define TPLC_PKG_H

#include <stdint.h>

#define TPLC_PKG_MAGIC_0 'T' /* "TPLCPKG\0" at bytes 0..8 */
#define TPLC_PKG_FORMAT_VERSION 1u
#define TPLC_PKG_HEADER_FIXED 160u /* header bytes before TOC */
#define TPLC_PKG_TOC_ENTRY_SIZE 32u
#define TPLC_PKG_SECTION_ALIGN 16u
#define TPLC_PKG_UUID_FIELD_LEN 64u /* ASCII uuid text, NUL-padded */
#define TPLC_PKG_HASH_LEN 32u

/* header flags */
#define TPLC_PKGF_SIGNED     (1u << 0)
#define TPLC_PKGF_COMPRESSED (1u << 1)

/* section flags */
#define TPLC_PKG_SECF_REQUIRED   (1u << 0) /* unknown+required → hard reject */
#define TPLC_PKG_SECF_COMPRESSED (1u << 1) /* payload is zstd-compressed */

/* section type ids (doc 01 §2.1) */
#define TPLC_SEC_MANIFEST        0x0001u
#define TPLC_SEC_IR_BITCODE      0x0002u
#define TPLC_SEC_TARGET_OBJ      0x0010u
#define TPLC_SEC_SYMMAP          0x0020u
#define TPLC_SEC_TYPE_CATALOG    0x0021u
#define TPLC_SEC_LAYOUT_DESC     0x0022u
#define TPLC_SEC_MIGRATE_DESC    0x0023u
#define TPLC_SEC_IO_MAP          0x0030u
#define TPLC_SEC_SOURCE_MANIFEST 0x0040u
#define TPLC_SEC_PLCOPEN_XML     0x0041u
#define TPLC_SEC_ST_SOURCES      0x0042u
#define TPLC_SEC_DEBUG_LINE      0x0050u
#define TPLC_SEC_SIGNATURE       0x0060u
#define TPLC_SEC_TOOLCHAIN       0x0070u

/* default limits (doc 01 §2.2) */
#define TPLC_PKG_MAX_FILE        (64ull << 20)
#define TPLC_PKG_MAX_TOC         64u
#define TPLC_PKG_MAX_IR          (32ull << 20)
#define TPLC_PKG_MAX_TARGET_OBJ  (32ull << 20)
#define TPLC_PKG_MAX_SYMMAP_CAT  (8ull << 20)
#define TPLC_PKG_MAX_MANIFEST    (256ull << 10)

/* package reader error codes — owned by doc 01 §13 (-200.. block) */
#define TPLC_E_PKG_MAGIC           (-200)
#define TPLC_E_PKG_VERSION         (-201)
#define TPLC_E_PKG_TOC             (-202)
#define TPLC_E_PKG_HASH_HEADER     (-203)
#define TPLC_E_PKG_HASH_CONTENT    (-204)
#define TPLC_E_PKG_UNKNOWN_SECTION (-205)
#define TPLC_E_PKG_LIMIT           (-206)
#define TPLC_E_SIG_REQUIRED        (-207)
#define TPLC_E_SIG_INVALID         (-208)
#define TPLC_E_SIG_UNTRUSTED       (-209)
#define TPLC_E_PKG_HIDDEN_STATE    (-210)
#define TPLC_E_SYMMAP_RANGE        (-211)
#define TPLC_E_TARGET_MISMATCH     (-212)
#define TPLC_E_PKG_ABI_MISMATCH    (-213) /* doc's ABI_MISMATCH; abi.h owns -64 */
#define TPLC_E_REQUIRES_UNMET      (-214)
#define TPLC_E_PKG_SECTION_FORMAT  (-215) /* impl addition: malformed payload */
#define TPLC_E_PKG_SYSCALL         (-216) /* doc 15 §9: post-link scanner reject */

/* on-wire structures for readers that want them (packed, LE) */
#pragma pack(push, 1)
typedef struct TplcPkgHeader {
    uint8_t  magic[8];          /* "TPLCPKG\0" */
    uint32_t format_version;
    uint32_t header_size;
    uint32_t toc_count;
    uint32_t flags;
    uint64_t total_size;
    uint8_t  package_uuid[TPLC_PKG_UUID_FIELD_LEN];
    uint8_t  content_hash[TPLC_PKG_HASH_LEN];
    uint8_t  header_hash[TPLC_PKG_HASH_LEN];
} TplcPkgHeader;                /* 160 B */

typedef struct TplcPkgTocEntry {
    uint32_t section_type;
    uint32_t section_flags;
    uint64_t offset;
    uint64_t size;
    uint32_t section_version;
    uint32_t aux;
} TplcPkgTocEntry;              /* 32 B */
#pragma pack(pop)

#endif /* TPLC_PKG_H */
