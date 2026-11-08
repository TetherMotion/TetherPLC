/* mcu_shim_host.c — MCU static-link shim conformance host
 * (plan/18-mcu-path.md §7 criterion 3; runtime-abi.md §11.7).
 *
 * Proves the runtime ABI works with NO dynamic loading, NO ELF
 * parsing, NO filesystem, NO mmap and NO threads — the standing
 * guard against the ABI drifting toward Linux-only assumptions.
 *
 * Both module images (v1, v2) are linked into this binary; their ABI
 * symbols are macro-renamed per version at compile time (see
 * CMakeLists.txt) and resolved into a static function table — the
 * mini-loader's svc-table resolution, without a loader. Memory is
 * static: blob, retain, image and marker regions are fixed buffers,
 * exactly the SRAM placement an MCU firmware would use.
 *
 * Subset covered: abi_version, descriptor, init, task dispatch,
 * latched io copy, cooperative abort, blob relocation, migrate
 * (copy/dropped/default-init) and shutdown retain hint.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "tplc/tplc_abi.h"
#include "stub_layout.h"
#include "desc_formats.h"

/* ------------------------------------------------------------------ */
/* Per-version module tables (statically resolved)                     */
/* ------------------------------------------------------------------ */

typedef struct ShimMod {
    uint32_t (*abi_version)(void);
    const TplcProgramDescriptor* (*get_descriptor)(void);
    int32_t (*init)(TplcCtx*);
    void (*shutdown)(TplcCtx*);
    int32_t (*task_run)(TplcTaskCtx*);
    int32_t (*migrate)(TplcMigrateCtx*);
} ShimMod;

#define SHIM_DECL(p) \
    extern uint32_t p##_abi_version(void); \
    extern const TplcProgramDescriptor* p##_get_descriptor(void); \
    extern int32_t p##_init(TplcCtx*); \
    extern void p##_shutdown(TplcCtx*); \
    extern int32_t p##_task_run(TplcTaskCtx*); \
    extern int32_t p##_migrate(TplcMigrateCtx*);
SHIM_DECL(tplc_v1)
SHIM_DECL(tplc_v2)

#define SHIM_TABLE(p) { p##_abi_version, p##_get_descriptor, p##_init, \
                        p##_shutdown, p##_task_run, p##_migrate }
static const ShimMod mod_v1 = SHIM_TABLE(tplc_v1);
static const ShimMod mod_v2 = SHIM_TABLE(tplc_v2);

/* ------------------------------------------------------------------ */
/* Static memory — no malloc, no mmap                                  */
/* ------------------------------------------------------------------ */

static uint8_t blob_a[512] __attribute__((aligned(64)));
static uint8_t blob_b[512] __attribute__((aligned(64)));
static uint8_t img_in[256] __attribute__((aligned(64)));
static uint8_t img_out[256] __attribute__((aligned(64)));
static uint8_t markers[256] __attribute__((aligned(64)));

static uint64_t fake_now;
static uint32_t log_calls, retain_hints, event_calls;

static void svc_log(void* h, uint8_t lvl, uint32_t code,
                    const void* args, uint32_t n) {
    (void)h; (void)lvl; (void)code; (void)args; (void)n;
    log_calls++;
}
static void svc_diag(void* h, uint32_t id, int64_t delta) {
    (void)h; (void)id; (void)delta;
}
static uint64_t svc_now(void* h) { (void)h; return fake_now; }
static void svc_retain(void* h, uint64_t off, uint64_t len) {
    (void)h; (void)off; (void)len;
    retain_hints++;
}
static int32_t svc_event(void* h, uint32_t mask) {
    (void)h; (void)mask;
    event_calls++;
    return 0;
}
static void* svc_get(void* h, const char* name, uint32_t len) {
    (void)h; (void)name; (void)len;
    return 0;                       /* no debug services on bare metal */
}

static const TplcServices services = {
    .struct_size  = sizeof(TplcServices),
    .caps         = 0,
    .log          = svc_log,
    .diag_counter = svc_diag,
    .steady_ns    = svc_now,
    .retain_dirty = svc_retain,
    .raise_event  = svc_event,
    .get_service  = svc_get,
    .host         = 0,
};

/* ------------------------------------------------------------------ */
/* Checks                                                              */
/* ------------------------------------------------------------------ */

static int fails;
static void chk(int cond, const char* what) {
    if (cond) {
        printf("PASS %s\n", what);
    } else {
        printf("FAIL %s\n", what);
        fails++;
    }
}

static uint64_t rd64(const uint8_t* b, uint64_t off) {
    uint64_t v;
    memcpy(&v, b + off, 8);
    return v;
}

/* latched io copy, honoring the descriptor's bindings — the shim's
   stand-in for the image layer (no SHM anywhere) */
static void sync_io(const TplcCtx* c, const TplcProgramDescriptor* d,
                    int to_blob) {
    for (uint32_t i = 0; i < d->io_binding_count; i++) {
        const TplcIoBinding* b = &d->io_bindings[i];
        uint8_t* dst;
        const uint8_t* src;
        if (to_blob) {
            src = (b->region == TPLC_REGION_IMAGE_IN)
                      ? (const uint8_t*)c->image_in + b->image_offset
                      : 0;
            dst = (uint8_t*)c->state_blob + b->blob_offset;
            if (!src) continue;
        } else {
            if (b->region == TPLC_REGION_IMAGE_OUT) {
                dst = (uint8_t*)c->image_out + b->image_offset;
            } else if (b->region == TPLC_REGION_MARKER) {
                dst = (uint8_t*)c->markers + b->image_offset;
            } else {
                continue;
            }
            src = (const uint8_t*)c->state_blob + b->blob_offset;
        }
        memcpy(dst, src, b->size);
    }
}

static int32_t run_task(const ShimMod* m, TplcCtx* c, uint32_t idx) {
    TplcTaskCtx t;
    memset(&t, 0, sizeof t);
    t.prog = c;
    t.task_index = idx;
    t.scheduled_ns = fake_now;
    t.deadline_ns = fake_now + 1000000;
    t.dt_ns = 1000000;
    return m->task_run(&t);
}

/* migrate_desc builder — same wire format abi_conformance uses:
   header(hash32 + vlen16 + "v1" + pad) + rule_count + rules +
   embedded predecessor LAYD (2 regions, 9 fields) */
static size_t build_migd(uint8_t* out, size_t cap) {
    uint8_t* p = out;
    memset(out, 0, cap);
    static const uint8_t hash[32] = { 0xA1,0xA1,0xA1,0xA1 };
    memcpy(p, hash, 32); p += 32;
    uint16_t vlen = 2;
    memcpy(p, &vlen, 2); p += 2;
    memcpy(p, "v1", 2); p += 2;
    while (((size_t)(p - out)) % 4) *p++ = 0;
    MigdRule rules[11];
    memset(rules, 0, sizeof rules);
    uint32_t nr = 0;
    for (uint32_t i = 0; i <= 7; i++) {          /* copy shared */
        rules[nr].kind = 0; rules[nr].old_field = i;
        rules[nr].new_field = i; nr++;
    }
    rules[nr].kind = 3; rules[nr].severity = 1;  /* dropped → degraded */
    rules[nr].old_field = 8; nr++;
    rules[nr].kind = 2; rules[nr].new_field = 8; nr++;  /* default-init */
    rules[nr].kind = 2; rules[nr].new_field = 9; nr++;
    uint32_t rc = nr;
    memcpy(p, &rc, 4); p += 4;
    memcpy(p, rules, rc * sizeof(MigdRule));
    p += rc * sizeof(MigdRule);

    LaydHeader lh;
    memset(&lh, 0, sizeof lh);
    lh.magic = TPLC_LAYD_MAGIC; lh.version = TPLC_LAYD_VERSION;
    lh.region_count = 2; lh.total_size = STUB_V1_BLOB_SIZE;
    memcpy(p, &lh, sizeof lh); p += sizeof lh;
    LaydRegion r0, r1;
    memset(&r0, 0, sizeof r0); memset(&r1, 0, sizeof r1);
    r0.kind = 0; r0.align = 8; r0.offset = 0;
    r0.size = STUB_V1_BLOB_SIZE;
    r1.kind = 3; r1.align = 8; r1.offset = STUB_RETAIN_OFF;
    r1.size = STUB_RETAIN_SIZE;
    memcpy(p, &r0, sizeof r0); p += sizeof r0;
    memcpy(p, &r1, sizeof r1); p += sizeof r1;
    static const struct { uint32_t off, size; uint8_t flags; } flds[] = {
        {STUB_F_IN_BIT,1,0}, {STUB_F_OUT_BIT,1,0},
        {STUB_F_IN_BYTE,1,0}, {STUB_F_OUT_BYTE,1,0},
        {STUB_OFF_FAST,8,0}, {STUB_OFF_SLOW,8,0},
        {STUB_OFF_COUNTER,8,1}, {STUB_OFF_MARKER,16,0},
        {STUB_V1_OFF_OBSOLETE,8,0},
    };
    uint32_t fc = sizeof flds / sizeof flds[0];
    memcpy(p, &fc, 4); p += 4;
    for (uint32_t i = 0; i < fc; i++) {
        LaydField lf;
        memset(&lf, 0, sizeof lf);
        lf.sym_entry = 0xFFFFFFFFu;
        lf.flags = flds[i].flags;
        lf.offset = flds[i].off;
        lf.size = flds[i].size;
        memcpy(p, &lf, sizeof lf); p += sizeof lf;
    }
    return (size_t)(p - out);
}

int main(void) {
    /* 1. ABI surface: version + descriptor, no loader involved */
    chk(mod_v1.abi_version() == TPLC_ABI_VERSION,
        "v1 abi_version matches header");
    const TplcProgramDescriptor* d1 = mod_v1.get_descriptor();
    chk(d1 && strcmp((const char*)d1->name, "stub") == 0 &&
        d1->task_count == 2 && d1->state_blob_size == STUB_V1_BLOB_SIZE &&
        d1->io_binding_count == 5,
        "v1 descriptor sane (name/tasks/blob/bindings)");

    /* 2. init on a static blob */
    TplcCtx ctx;
    memset(&ctx, 0, sizeof ctx);
    ctx.struct_size = sizeof(TplcCtx);
    ctx.state_blob = blob_a;
    ctx.retain_region = blob_a + STUB_RETAIN_OFF;
    ctx.image_in = img_in;
    ctx.image_out = img_out;
    ctx.markers = markers;
    ctx.svc = &services;
    chk(mod_v1.init(&ctx) == TPLC_OK, "v1 init on static blob");

    /* 3. latched io copy + task dispatch for 10 cycles */
    img_in[0] = 1;                    /* %I bit0  */
    img_in[4] = 0x40;                 /* %I byte4 */
    for (int i = 0; i < 10; i++) {
        fake_now += 1000000;
        ctx.now_ns = fake_now;
        ctx.cycle_no = (uint64_t)i;
        sync_io(&ctx, d1, 1);
        if (run_task(&mod_v1, &ctx, 0) != TPLC_OK) break;
        if (run_task(&mod_v1, &ctx, 1) != TPLC_OK) break;
        sync_io(&ctx, d1, 0);
    }
    chk(rd64(blob_a, STUB_OFF_FAST) == 10 &&
        rd64(blob_a, STUB_OFF_SLOW) == 10 &&
        rd64(blob_a, STUB_OFF_COUNTER) == 10,
        "10 cycles dispatch through tplc_task_run");
    chk(img_out[0] == 1 && img_out[4] == 0x41,
        "io bindings copied (out_bit=in_bit, out_byte=in+1)");

    /* 4. cooperative abort checkpoint */
    ctx.abort_flag = 1;
    chk(run_task(&mod_v1, &ctx, 0) == TPLC_TASK_E_ABORTED,
        "abort_flag honored at task checkpoint");
    ctx.abort_flag = 0;

    /* 5. blob relocation: pointer-free state survives a move */
    memcpy(blob_b, blob_a, STUB_V1_BLOB_SIZE);
    ctx.state_blob = blob_b;
    ctx.retain_region = blob_b + STUB_RETAIN_OFF;
    for (int i = 0; i < 5; i++) {
        fake_now += 1000000;
        ctx.now_ns = fake_now;
        run_task(&mod_v1, &ctx, 0);
    }
    chk(rd64(blob_b, STUB_OFF_FAST) == 16,
        "blob relocated mid-life, state continues (pointer-free)");

    /* 6. migrate v1 → v2 without any loader: rules + embedded LAYD */
    static uint8_t migd[2048];
    size_t mlen = build_migd(migd, sizeof migd);
    TplcCtx ctx2;
    memset(&ctx2, 0, sizeof ctx2);
    ctx2.struct_size = sizeof(TplcCtx);
    ctx2.state_blob = blob_a;          /* fresh target buffer */
    memset(blob_a, 0, sizeof blob_a);
    ctx2.retain_region = blob_a + STUB_RETAIN_OFF;
    ctx2.image_in = img_in;
    ctx2.image_out = img_out;
    ctx2.markers = markers;
    ctx2.svc = &services;
    TplcMigrateCtx mctx;
    memset(&mctx, 0, sizeof mctx);
    mctx.new_prog = &ctx2;
    mctx.old_blob = blob_b;
    mctx.old_blob_size = STUB_V1_BLOB_SIZE;
    mctx.old_abi_version = TPLC_ABI_VERSION;
    mctx.migrate_desc = migd;
    mctx.migrate_desc_size = mlen;
    int32_t mrc = mod_v2.migrate(&mctx);
    chk(mrc == TPLC_OK && mctx.out_status == TPLC_MIGRATE_OK_DEGRADED,
        "v2 migrate accepts v1 state (dropped → degraded)");
    chk(rd64(blob_a, STUB_OFF_FAST) == 16 &&
        rd64(blob_a, STUB_OFF_COUNTER) == 10,
        "migrated counters preserved");
    chk(rd64(blob_a, STUB_V2_OFF_TOTALIZER) == 7 &&
        rd64(blob_a, STUB_V2_OFF_PART_COUNT) == 0,
        "new fields default-initialized");

    const TplcProgramDescriptor* d2 = mod_v2.get_descriptor();
    chk(d2 && d2->state_blob_size == STUB_V2_BLOB_SIZE,
        "v2 descriptor after swap");
    chk(run_task(&mod_v2, &ctx2, 1) == TPLC_OK &&
        rd64(blob_a, STUB_OFF_COUNTER) == 11,
        "v2 task continues from migrated state");

    /* 7. shutdown hygiene: retain-dirty hint via the svc table */
    uint32_t before = retain_hints;
    mod_v2.shutdown(&ctx2);
    chk(retain_hints > before, "shutdown raises retain-dirty hint");

    printf("== %s (%d fail)\n", fails ? "FAIL" : "PASS", fails);
    return fails ? 1 : 0;
}
