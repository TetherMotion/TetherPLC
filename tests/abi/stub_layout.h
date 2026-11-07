/* stub_layout.h — the hand-computed blob layouts of the stub modules.
 * Shared between stub_module.c and the conformance test so both agree
 * on field offsets/indices. The stub plays the role generated code
 * will later play: it knows its own layout at compile time.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef STUB_LAYOUT_H
#define STUB_LAYOUT_H

#include <stdint.h>

/* Common prefix (v1 and v2 identical up to offset 48) */
#define STUB_F_IN_BIT    0   /* u8 shadow of %I bit0            */
#define STUB_F_OUT_BIT   1   /* u8 shadow of %Q bit0            */
#define STUB_F_IN_BYTE   2   /* u8 shadow of %I byte4           */
#define STUB_F_OUT_BYTE  3   /* u8 shadow of %Q byte4           */
#define STUB_OFF_FAST    8   /* u64 fast task cycle counter     */
#define STUB_OFF_SLOW    16  /* u64 slow task cycle counter     */
#define STUB_OFF_COUNTER 24  /* u64 retained counter            */
#define STUB_OFF_MARKER  32  /* u8[16] marker shadow            */
#define STUB_RETAIN_OFF  24u
#define STUB_RETAIN_SIZE 8u

/* v1-only tail */
#define STUB_V1_OFF_OBSOLETE 48  /* u64 — dropped in v2 */
#define STUB_V1_BLOB_SIZE    56u

/* v2 tail */
#define STUB_V2_OFF_PART_COUNT 48  /* u64 new field, default 0 */
#define STUB_V2_OFF_TOTALIZER  56  /* u64 new field, default 7 */
#define STUB_V2_BLOB_SIZE      64u

/* Field-table indices (the module's own layout_desc equivalent) */
enum {
    STUB_FI_IN_BIT = 0, STUB_FI_OUT_BIT, STUB_FI_IN_BYTE, STUB_FI_OUT_BYTE,
    STUB_FI_FAST, STUB_FI_SLOW, STUB_FI_COUNTER, STUB_FI_MARKER,
    STUB_V1_FI_OBSOLETE = 8,
    STUB_V1_FIELD_COUNT  = 9,
    STUB_V2_FI_PART_COUNT = 8, STUB_V2_FI_TOTALIZER = 9,
    STUB_V2_FIELD_COUNT   = 10
};

#define STUB_IMAGE_BYTES 8u
#define STUB_MARKER_BYTES 16u

/* Fixed test identities */
static const uint8_t STUB_PROGRAM_UUID[16] =
    {0x51,0x50,0x4c,0x43,0,0,0,0,0,0,0,0,0,0,0,1};
static const uint8_t STUB_V1_LAYOUT_HASH[32] =
    {0xA1,0xA1,0xA1,0xA1,0,0,0,0,0,0,0,0,0,0,0,0,
     0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
static const uint8_t STUB_V2_LAYOUT_HASH[32] =
    {0xB2,0xB2,0xB2,0xB2,0,0,0,0,0,0,0,0,0,0,0,0,
     0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};

#endif /* STUB_LAYOUT_H */
