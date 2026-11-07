/* desc_formats.h — binary layouts of layout_desc and migrate_desc
 * (plan doc 01 §7/§8 + Appendix H). Test-local until the package
 * reader owns them. All fields little-endian, packed.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef DESC_FORMATS_H
#define DESC_FORMATS_H

#include <stdint.h>

#define TPLC_LAYD_MAGIC 0x4459414Cu /* 'LAYD' LE */
#define TPLC_LAYD_VERSION 1u

/* layout_desc header (doc 01 §7) */
#pragma pack(push, 1)
typedef struct LaydHeader {
    uint32_t magic;        /* TPLC_LAYD_MAGIC             */
    uint32_t version;
    uint32_t region_count;
    uint32_t _pad0;
    uint64_t total_size;
} LaydHeader;              /* 24 B */

typedef struct LaydRegion {
    uint8_t  kind;         /* 0 global 1 fb 2 task_local 3 retain 4 internal */
    uint8_t  _pad[3];
    uint32_t align;
    uint64_t offset;
    uint64_t size;
} LaydRegion;              /* 24 B */

/* layout_desc field-table entry (doc 01 App H, 32 B) */
typedef struct LaydField {
    uint32_t sym_entry;    /* symmap idx or 0xFFFFFFFF      */
    uint8_t  field_kind;   /* 0 var 1 fb_inst 2 task_local 3 internal
                              4 retain_wrapper             */
    uint8_t  flags;        /* bit0 retain bit4 fb_instance  */
    uint16_t gen;
    uint64_t offset;
    uint64_t size;
    uint32_t type_ref;
    uint32_t _pad;
} LaydField;               /* 32 B */

/* migrate_desc header (doc 01 §8) */
typedef struct MigdHeader {
    uint8_t  predecessor_layout_hash[32];
    uint16_t predecessor_version_len;
    /* predecessor_version bytes follow (not part of this struct,
       then pad to 4, then rule_count + rules + embedded LAYD) */
} MigdHeader;

/* migrate_desc rule entry (doc 01 App H, 16 B) */
typedef struct MigdRule {
    uint8_t  kind;         /* 0 copy 1 transform 2 default-init
                              3 dropped 4 deny 5 alias-rename */
    uint8_t  severity;     /* 0 info 1 degraded 2 deny        */
    uint16_t transform_id; /* for kind=1                      */
    uint32_t old_field;    /* idx into embedded pred table    */
    uint32_t new_field;    /* idx into own layout_desc table  */
    uint32_t aux;          /* deny reason | alias str off     */
} MigdRule;                /* 16 B */
#pragma pack(pop)

#endif /* DESC_FORMATS_H */
