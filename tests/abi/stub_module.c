/* stub_module.c — hand-written module exercising every ABI rule
 * (plan doc 02 §11 fixture 1). Compiled twice: TPLC_STUB_V=1 → v1
 * (with OBSOLETE field), TPLC_STUB_V=2 → v2 (with PART_COUNT/
 * TOTALIZER); plus -DTPLC_STUB_BAD_ABI for a version-mismatch build.
 * Plays the role generated code will later play: knows its own blob
 * layout, implements init/task/migrate directly.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_abi.h>
#include "stub_layout.h"
#include "desc_formats.h"

#include <string.h>

#ifndef TPLC_STUB_V
#define TPLC_STUB_V 1
#endif

/* ------------------------------------------------------------------ */
/* Own layout (the module's layout_desc field-table equivalent)        */
/* ------------------------------------------------------------------ */

typedef struct OwnField { uint32_t off; uint32_t size; } OwnField;

static const OwnField own_fields[] = {
    [STUB_FI_IN_BIT]   = {STUB_F_IN_BIT, 1},
    [STUB_FI_OUT_BIT]  = {STUB_F_OUT_BIT, 1},
    [STUB_FI_IN_BYTE]  = {STUB_F_IN_BYTE, 1},
    [STUB_FI_OUT_BYTE] = {STUB_F_OUT_BYTE, 1},
    [STUB_FI_FAST]     = {STUB_OFF_FAST, 8},
    [STUB_FI_SLOW]     = {STUB_OFF_SLOW, 8},
    [STUB_FI_COUNTER]  = {STUB_OFF_COUNTER, 8},
    [STUB_FI_MARKER]   = {STUB_OFF_MARKER, 16},
#if TPLC_STUB_V == 1
    [STUB_V1_FI_OBSOLETE] = {STUB_V1_OFF_OBSOLETE, 8},
#elif TPLC_STUB_V == 2
    [STUB_V2_FI_PART_COUNT] = {STUB_V2_OFF_PART_COUNT, 8},
    [STUB_V2_FI_TOTALIZER]  = {STUB_V2_OFF_TOTALIZER, 8},
#endif
};
#define OWN_FIELD_COUNT (sizeof(own_fields) / sizeof(own_fields[0]))

#if TPLC_STUB_V == 1
#define STUB_BLOB_SIZE STUB_V1_BLOB_SIZE
#elif TPLC_STUB_V == 2
#define STUB_BLOB_SIZE STUB_V2_BLOB_SIZE
#endif

/* ------------------------------------------------------------------ */
/* Default init — shared by tplc_init and tplc_migrate                 */
/* (doc 02 §4.4: migrate must produce an init-consistent blob)         */
/* ------------------------------------------------------------------ */

static void default_init(uint8_t* blob, int skip_retain) {
    memset(blob + STUB_F_IN_BIT, 0, 4);            /* io shadows      */
    *(uint64_t*)(blob + STUB_OFF_FAST) = 0;
    *(uint64_t*)(blob + STUB_OFF_SLOW) = 0;
    if (!skip_retain)
        *(uint64_t*)(blob + STUB_OFF_COUNTER) = 0;
    memset(blob + STUB_OFF_MARKER, 0, 16);
#if TPLC_STUB_V == 1
    *(uint64_t*)(blob + STUB_V1_OFF_OBSOLETE) = 0x1111;
#elif TPLC_STUB_V == 2
    *(uint64_t*)(blob + STUB_V2_OFF_PART_COUNT) = 0;
    *(uint64_t*)(blob + STUB_V2_OFF_TOTALIZER) = 7;
#endif
}

/* ------------------------------------------------------------------ */
/* Tasks                                                               */
/* ------------------------------------------------------------------ */

static int32_t fast_task(TplcTaskCtx* tc) {
    TplcCtx* c = tc->prog;
    uint8_t* b = (uint8_t*)c->state_blob;

    /* out_bit = in_bit ; out_byte = in_byte + 1 */
    b[STUB_F_OUT_BIT] = b[STUB_F_IN_BIT];
    b[STUB_F_OUT_BYTE] = (uint8_t)(b[STUB_F_IN_BYTE] + 1);
    (*(uint64_t*)(b + STUB_OFF_FAST))++;

    /* cooperative checkpoint pattern: bounded spin honoring abort_flag
       — what generated code emits inside loop back-edges (doc 04 §5) */
    for (volatile int i = 0; i < 10000; i++)
        if (c->abort_flag) return TPLC_TASK_E_ABORTED;
    return TPLC_OK;
}

static int32_t slow_task(TplcTaskCtx* tc) {
    TplcCtx* c = tc->prog;
    uint8_t* b = (uint8_t*)c->state_blob;
    (*(uint64_t*)(b + STUB_OFF_SLOW))++;
    (*(uint64_t*)(b + STUB_OFF_COUNTER))++;
    /* %M direct map: markers buffer is program-internal scratch */
    if (c->markers) ((uint8_t*)c->markers)[0] ^= 0xFF;
    b[STUB_OFF_MARKER] ^= 0xFF;
    return TPLC_OK;
}

static const TplcTaskFn task_table[] = { fast_task, slow_task };

static const TplcTaskDesc tasks[] = {
    { .name = "FAST", .kind = TPLC_TASK_CYCLIC,
      .flags = TPLC_TASKF_ENABLED_AT_START | TPLC_TASKF_WATCHDOGGED,
      .priority = 10, .period_ns = 1000000, .wcet_hint_ns = 50000,
      .entry_index = 0, .task_local_size = 0 },
    { .name = "SLOW", .kind = TPLC_TASK_CYCLIC,
      .flags = TPLC_TASKF_ENABLED_AT_START,
      .priority = 50, .period_ns = 10000000, .wcet_hint_ns = 50000,
      .entry_index = 1, .task_local_size = 0 },
};

static const TplcIoBinding io_bindings[] = {
    /* %I bit0 → in_bit shadow */
    { .blob_offset = STUB_F_IN_BIT, .region = TPLC_REGION_IMAGE_IN,
      .type_code = TPLC_IOT_BOOL, .flags = TPLC_IOB_FORCEABLE,
      .unit = 0, .image_offset = 0, .size = 1 },
    /* %I byte4 → in_byte shadow */
    { .blob_offset = STUB_F_IN_BYTE, .region = TPLC_REGION_IMAGE_IN,
      .type_code = TPLC_IOT_U8, .unit = 0, .image_offset = 4,
      .size = 1 },
    /* out_bit shadow → %Q bit0 */
    { .blob_offset = STUB_F_OUT_BIT, .region = TPLC_REGION_IMAGE_OUT,
      .type_code = TPLC_IOT_BOOL, .flags = TPLC_IOB_FORCEABLE,
      .unit = 0, .image_offset = 0, .size = 1 },
    /* out_byte shadow → %Q byte4 */
    { .blob_offset = STUB_F_OUT_BYTE, .region = TPLC_REGION_IMAGE_OUT,
      .type_code = TPLC_IOT_U8, .unit = 0, .image_offset = 4,
      .size = 1 },
    /* marker_shadow ↔ %M region (direct map, no copy) */
    { .blob_offset = STUB_OFF_MARKER, .region = TPLC_REGION_MARKER,
      .type_code = TPLC_IOT_RAW, .unit = 0, .image_offset = 0,
      .size = 16 },
};

static const TplcProgramDescriptor descriptor = {
    .struct_size  = sizeof(TplcProgramDescriptor),
    .abi_version  = TPLC_ABI_VERSION,
    .abi_ext      = 0,
    .flags        = 1u,  /* has_migrate */
    .program_uuid = {0x51,0x50,0x4c,0x43,0,0,0,0,0,0,0,0,0,0,0,1},
#if TPLC_STUB_V == 1
    .layout_hash  = {0xA1,0xA1,0xA1,0xA1},
#else
    .layout_hash  = {0xB2,0xB2,0xB2,0xB2},
#endif
    .name         = "stub",
    .state_blob_size = STUB_BLOB_SIZE,
    .retain_size     = STUB_RETAIN_SIZE,
    .retain_offset   = STUB_RETAIN_OFF,
    .task_scratch_size = 0,
    .image_in_bytes  = STUB_IMAGE_BYTES,
    .image_out_bytes = STUB_IMAGE_BYTES,
    .marker_bytes    = STUB_MARKER_BYTES,
    .task_count = 2, .tasks = tasks,
    .task_fns = task_table, .task_fn_count = 2,
    .io_bindings = io_bindings,
    .io_binding_count = sizeof(io_bindings) / sizeof(io_bindings[0]),
};

/* ------------------------------------------------------------------ */
/* Exported ABI surface                                                */
/* ------------------------------------------------------------------ */

uint32_t tplc_abi_version(void) {
#ifdef TPLC_STUB_BAD_ABI
    return 999u;
#else
    return TPLC_ABI_VERSION;
#endif
}

const TplcProgramDescriptor* tplc_get_descriptor(void) {
    return &descriptor;
}

int32_t tplc_init(TplcCtx* ctx) {
    if (!ctx || ctx->struct_size != sizeof(TplcCtx) || !ctx->state_blob)
        return TPLC_E_INIT_INVALID;
    if (STUB_BLOB_SIZE != descriptor.state_blob_size)
        return TPLC_E_INIT_LAYOUT;
    default_init((uint8_t*)ctx->state_blob,
                 (ctx->ctx_flags & TPLC_CTXF_RETAIN_RESTORED) != 0);
    return TPLC_OK;
}

void tplc_shutdown(TplcCtx* ctx) {
    /* retain-dirty hint on shutdown (doc 02 §4.5 hygiene) */
    if (ctx && ctx->svc && ctx->svc->retain_dirty)
        ctx->svc->retain_dirty(ctx->svc->host, STUB_RETAIN_OFF,
                               STUB_RETAIN_SIZE);
}

int32_t tplc_task_run(TplcTaskCtx* tctx) {
    /* single-entry dispatch path for loaders that prefer it */
    if (!tctx || tctx->task_index >= descriptor.task_count)
        return TPLC_TASK_E_FAULT;
    return task_table[descriptor.tasks[tctx->task_index].entry_index](tctx);
}

/* ------------------------------------------------------------------ */
/* tplc_migrate — walks migrate_desc rules (doc 01 §8 formats).        */
/* The embedded predecessor LAYD supplies old offsets; own_fields[]    */
/* supplies new offsets — exactly the generated-code pattern.          */
/* ------------------------------------------------------------------ */

static const uint8_t* desc_bytes(const void* p) { return (const uint8_t*)p; }

int32_t tplc_migrate(TplcMigrateCtx* mctx) {
    mctx->out_status = TPLC_MIGRATE_ERR_DESC;
    mctx->out_first_error = 0;
    if (!mctx->new_prog || !mctx->new_prog->state_blob ||
        !mctx->old_blob || !mctx->migrate_desc)
        return TPLC_E_MIGRATE_DESCINVAL;

    const uint8_t* d = desc_bytes(mctx->migrate_desc);
    const uint8_t* end = d + mctx->migrate_desc_size;
    if (mctx->migrate_desc_size < sizeof(MigdHeader) + 4)
        return TPLC_E_MIGRATE_DESCINVAL;

    uint16_t vlen;
    memcpy(&vlen, d + 32, 2);
    const uint8_t* p = d + 32 + 2 + vlen;
    p += (4 - ((uintptr_t)(p - d) & 3u)) & 3u;   /* align 4 */
    if (p + 4 > end) return TPLC_E_MIGRATE_DESCINVAL;

    uint32_t rule_count;
    memcpy(&rule_count, p, 4);
    const MigdRule* rules = (const MigdRule*)(p + 4);
    const uint8_t* layd = p + 4 + (uint64_t)rule_count * sizeof(MigdRule);
    if (layd + sizeof(LaydHeader) > end) return TPLC_E_MIGRATE_DESCINVAL;

    /* embedded predecessor LAYD */
    LaydHeader lh;
    memcpy(&lh, layd, sizeof lh);
    if (lh.magic != TPLC_LAYD_MAGIC) return TPLC_E_MIGRATE_DESCINVAL;
    const uint8_t* ftab = layd + sizeof(LaydHeader) +
                          lh.region_count * sizeof(LaydRegion);
    if (ftab + 4 > end) return TPLC_E_MIGRATE_DESCINVAL;
    uint32_t pred_field_count;
    memcpy(&pred_field_count, ftab, 4);
    const LaydField* pred_fields = (const LaydField*)(ftab + 4);
    if ((const uint8_t*)(pred_fields + pred_field_count) > end)
        return TPLC_E_MIGRATE_DESCINVAL;

    uint8_t* new_blob = (uint8_t*)mctx->new_prog->state_blob;
    const uint8_t* old_blob = (const uint8_t*)mctx->old_blob;
    int degraded = 0;

    /* init-consistent base, then overlay migrated values
       (doc 02 §4.4 post-migrate consistency) */
    default_init(new_blob, /*skip_retain=*/1);

    for (uint32_t i = 0; i < rule_count; i++) {
        const MigdRule* r = &rules[i];
        if (r->kind == 4 /* deny */ ||
            (r->severity == 2 && r->kind != 3)) {
            mctx->out_status = TPLC_MIGRATE_DENY;
            mctx->out_first_error = i;
            return TPLC_E_MIGRATE_DENY;
        }
        if (r->kind == 3 /* dropped */) { degraded = 1; continue; }
        if (r->new_field >= OWN_FIELD_COUNT)
            return TPLC_E_MIGRATE_DESCINVAL;
        if (r->kind == 2 /* default-init */) continue; /* already done */
        /* copy / transform / alias-rename: field-granular copy */
        if (r->old_field >= pred_field_count)
            return TPLC_E_MIGRATE_DESCINVAL;
        const LaydField* of = &pred_fields[r->old_field];
        const OwnField* nf = &own_fields[r->new_field];
        if (of->offset + of->size > mctx->old_blob_size)
            return TPLC_E_MIGRATE_BLOBRANGE;
        uint64_t n = of->size < nf->size ? of->size : nf->size;
        memcpy(new_blob + nf->off, old_blob + of->offset, n);
        /* retain fields copied across are "restored" values */
    }

    mctx->out_status = degraded ? TPLC_MIGRATE_OK_DEGRADED
                                : TPLC_MIGRATE_OK;
    return TPLC_OK;
}
