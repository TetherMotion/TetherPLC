/* mcu_blob_host.c — Cortex-M harness that LOADS a TPLB blob
 * produced by `tplcc build --target armv7em-none-eabihf
 * --pic-blob` through the reference loader (tplb_load.c) and runs
 * the module ABI surface on it — no linker ever sees the blob.
 *
 * The blob is read from argv[1]; libgcc soft-float helpers the
 * module imports (__adddf3/__muldf3/…) are resolved by name from
 * this harness's own link against libgcc.
 *
 * Exit status: number of failed checks (0 = pass). Built with
 * arm-none-eabi-gcc -mcpu=cortex-m4 -mthumb and run under qemu-arm.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tplc/tplc_abi.h"
#include "tplb_load.h"

static int fails;
static void chk(int cond, const char* what) {
    printf("%s %s\n", cond ? "PASS" : "FAIL", what);
    if (!cond) fails++;
}

/* libgcc soft-float imports — addresses only, declared as arrays
 * so no prototype is needed; -mthumb pulls the Thumb multilib */
extern char __adddf3[], __subdf3[], __muldf3[], __divdf3[];
extern char __truncdfsf2[], __floatsidf[], __floatunsidf[];
extern char __aeabi_d2f[], __aeabi_f2d[];

static const TplbImport imports[] = {
    { "__adddf3",      (uintptr_t)__adddf3      },
    { "__subdf3",      (uintptr_t)__subdf3      },
    { "__muldf3",      (uintptr_t)__muldf3      },
    { "__divdf3",      (uintptr_t)__divdf3      },
    { "__truncdfsf2",  (uintptr_t)__truncdfsf2  },
    { "__floatsidf",   (uintptr_t)__floatsidf   },
    { "__floatunsidf", (uintptr_t)__floatunsidf },
    { "__aeabi_d2f",   (uintptr_t)__aeabi_d2f   },
    { "__aeabi_f2d",   (uintptr_t)__aeabi_f2d   },
};

/* static memory — the SRAM placement an MCU firmware would use */
static uint8_t ram[65536] __attribute__((aligned(4096)));
static uint8_t state[16384] __attribute__((aligned(64)));
static uint8_t img_in[256] __attribute__((aligned(64)));
static uint8_t img_out[256] __attribute__((aligned(64)));
static uint8_t markers[256] __attribute__((aligned(64)));
static uint8_t blobfile[262144];

static uint64_t fake_now;
static uint32_t svc_calls;

static void svc_log(void* h, uint8_t lvl, uint32_t code,
                    const void* args, uint32_t n) {
    (void)h; (void)lvl; (void)code; (void)args; (void)n;
    svc_calls++;
}
static void svc_diag(void* h, uint32_t id, int64_t delta) {
    (void)h; (void)id; (void)delta;
}
static uint64_t svc_now(void* h) { (void)h; return fake_now; }
static void svc_retain(void* h, uint64_t off, uint64_t len) {
    (void)h; (void)off; (void)len;
}
static int32_t svc_event(void* h, uint32_t mask) {
    (void)h; (void)mask; return 0;
}
static void* svc_get(void* h, const char* name, uint32_t len) {
    (void)h; (void)name; (void)len; return 0;
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

/* io copy honoring the descriptor's bindings — byte-granular */
static void sync_io(const TplcCtx* c, const TplcProgramDescriptor* d,
                    int to_blob) {
    for (uint32_t i = 0; i < d->io_binding_count; i++) {
        const TplcIoBinding* b = &d->io_bindings[i];
        uint8_t* dst;
        const uint8_t* src;
        if (to_blob) {
            if (b->region != TPLC_REGION_IMAGE_IN) continue;
            src = (const uint8_t*)c->image_in + b->image_offset;
            dst = (uint8_t*)c->state_blob + b->blob_offset;
        } else {
            if (b->region == TPLC_REGION_IMAGE_OUT)
                dst = (uint8_t*)c->image_out + b->image_offset;
            else if (b->region == TPLC_REGION_MARKER)
                dst = (uint8_t*)c->markers + b->image_offset;
            else
                continue;
            src = (const uint8_t*)c->state_blob + b->blob_offset;
        }
        memcpy(dst, src, b->size);
    }
}

typedef uint32_t            (*FnU32V)(void);
typedef const TplcProgramDescriptor* (*FnDescV)(void);
typedef int32_t             (*FnInit)(TplcCtx*);
typedef void                (*FnVoidCtx)(TplcCtx*);
typedef int32_t             (*FnTask)(TplcTaskCtx*);

int main(int argc, char** argv) {
    if (argc < 2) {
        printf("usage: %s <blob.tplb>\n", argv[0]);
        return 2;
    }
    FILE* f = fopen(argv[1], "rb");
    if (!f) { printf("FAIL cannot open %s\n", argv[1]); return 2; }
    size_t blen = fread(blobfile, 1, sizeof blobfile, f);
    fclose(f);

    TplbImage img;
    const char* err = tplb_load(blobfile, blen, ram, sizeof ram,
                                imports, sizeof imports / sizeof imports[0],
                                &img);
    chk(err == 0, "tplb_load places + fixes up blob");
    if (err) { printf("loader: %s\n", err); return fails ? fails : 1; }

    FnU32V      abi_version = (FnU32V)tplb_lookup(&img, "tplc_abi_version");
    FnDescV     get_desc    = (FnDescV)tplb_lookup(&img, "tplc_get_descriptor");
    FnInit      init        = (FnInit)tplb_lookup(&img, "tplc_init");
    FnVoidCtx   shutdown    = (FnVoidCtx)tplb_lookup(&img, "tplc_shutdown");
    FnTask      task_run    = (FnTask)tplb_lookup(&img, "tplc_task_run");
    chk(abi_version && get_desc && init && shutdown && task_run,
        "ABI symbols resolved from blob symtab");
    if (!(abi_version && get_desc && init && shutdown && task_run))
        return fails ? fails : 1;

    chk(abi_version() == TPLC_ABI_VERSION,
        "abi_version matches header");
    const TplcProgramDescriptor* d = get_desc();
    chk(d && d->task_count >= 1 && d->state_blob_size <= sizeof state,
        "descriptor sane (tasks/blob fit)");
    if (!d) return 99;

    TplcCtx ctx;
    memset(&ctx, 0, sizeof ctx);
    ctx.struct_size = sizeof(TplcCtx);
    ctx.state_blob = state;
    ctx.retain_region = (uint8_t*)ctx.state_blob + d->retain_offset;
    ctx.image_in = img_in;
    ctx.image_out = img_out;
    ctx.markers = markers;
    ctx.svc = &services;
    chk(init(&ctx) == TPLC_OK, "init on loaded blob");

    /* xin = 1.0 at %IL0; program computes acc += xin*1.5 + 2.0,
       cnt += 1, y = acc — exercises __muldf3/__adddf3 imports */
    double one = 1.0;
    memcpy(img_in, &one, 8);

    const int cycles = 7;
    for (int i = 0; i < cycles; i++) {
        fake_now += 1000000;
        ctx.now_ns = fake_now;
        ctx.cycle_no = (uint64_t)i;
        sync_io(&ctx, d, 1);
        for (uint32_t k = 0; k < d->task_count; k++) {
            TplcTaskCtx t;
            memset(&t, 0, sizeof t);
            t.prog = &ctx;
            t.task_index = k;
            t.scheduled_ns = fake_now;
            t.deadline_ns = fake_now + 1000000;
            t.dt_ns = 1000000;
            int32_t rc = task_run(&t);
            if (rc != TPLC_OK) {
                printf("task_run rc=%d task=%u\n", rc, k);
                chk(0, "task_run returned error");
                return fails > 250 ? 250 : fails;
            }
        }
        sync_io(&ctx, d, 0);
    }

    /* cnt (%QD8) must equal cycles; y (%QL16) must equal
       cycles * (1.0*1.5 + 2.0) = cycles * 3.5 */
    int found_cnt = 0, found_y = 0;
    for (uint32_t i = 0; i < d->io_binding_count; i++) {
        const TplcIoBinding* b = &d->io_bindings[i];
        if (b->region != TPLC_REGION_IMAGE_OUT)
            continue;
        if (b->size == 4) {
            uint32_t v;
            memcpy(&v, (const uint8_t*)img_out + b->image_offset, 4);
            chk(v == (uint32_t)cycles,
                "image_out counter == cycles after N scans");
            found_cnt = 1;
        } else if (b->size == 8) {
            double v;
            memcpy(&v, (const uint8_t*)img_out + b->image_offset, 8);
            chk(v == (double)cycles * 3.5,
                "image_out LREAL == cycles*3.5 (soft-float imports)");
            found_y = 1;
        }
    }
    chk(found_cnt, "DINT image_out binding present");
    chk(found_y, "LREAL image_out binding present");

    shutdown(&ctx);
    printf("== %s (%d fail)\n", fails ? "FAIL" : "PASS", fails);
    return fails > 250 ? 250 : fails;
}
