/* mcu_exec_host.c — Cortex-M execution harness for a tplcc
 * --target object. Stands in for the mini-loader: static SRAM
 * buffers, a svc table, io-binding copies, no libc-dependent
 * runtime assumptions beyond newlib for exit/printf.
 *
 * Exit status: number of failed checks (0 = pass). Built with
 * arm-none-eabi-gcc and run under qemu-arm (user mode).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "tplc/tplc_abi.h"

extern uint32_t tplc_abi_version(void);
extern const TplcProgramDescriptor* tplc_get_descriptor(void);
extern int32_t tplc_init(TplcCtx* ctx);
extern void tplc_shutdown(TplcCtx* ctx);
extern int32_t tplc_task_run(TplcTaskCtx* tctx);

static int fails;
static void chk(int cond, const char* what) {
    printf("%s %s\n", cond ? "PASS" : "FAIL", what);
    if (!cond) fails++;
}

/* static memory — the SRAM placement an MCU firmware would use */
static uint8_t blob[16384] __attribute__((aligned(64)));
static uint8_t img_in[256] __attribute__((aligned(64)));
static uint8_t img_out[256] __attribute__((aligned(64)));
static uint8_t markers[256] __attribute__((aligned(64)));

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

/* io copy honoring the descriptor's bindings — byte-granular; the
   test programs bind only whole-byte types */
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

int main(void) {
    chk(tplc_abi_version() == TPLC_ABI_VERSION,
        "abi_version matches header");
    const TplcProgramDescriptor* d = tplc_get_descriptor();
    chk(d && d->task_count >= 1 && d->state_blob_size <= sizeof blob,
        "descriptor sane (tasks/blob fit)");
    if (!d) return 99;

    TplcCtx ctx;
    memset(&ctx, 0, sizeof ctx);
    ctx.struct_size = sizeof(TplcCtx);
    ctx.state_blob = blob;
    ctx.retain_region = (uint8_t*)ctx.state_blob + d->retain_offset;
    ctx.image_in = img_in;
    ctx.image_out = img_out;
    ctx.markers = markers;
    ctx.svc = &services;
    chk(tplc_init(&ctx) == TPLC_OK, "init on static blob");

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
            int32_t rc = tplc_task_run(&t);
            if (rc != TPLC_OK) {
                printf("task_run rc=%d task=%u\n", rc, k);
                chk(0, "task_run returned error");
                return fails > 250 ? 250 : fails;
            }
        }
        sync_io(&ctx, d, 0);
    }

    /* the test program computes out := cnt after cnt := cnt + 1;
       after `cycles` scans the %QD0 binding must read `cycles` */
    int found = 0;
    for (uint32_t i = 0; i < d->io_binding_count; i++) {
        const TplcIoBinding* b = &d->io_bindings[i];
        if (b->region != TPLC_REGION_IMAGE_OUT || b->size < 4)
            continue;
        uint32_t v;
        memcpy(&v, (const uint8_t*)img_out + b->image_offset, 4);
        chk(v == (uint32_t)cycles,
            "image_out counter == cycles after N scans");
        found = 1;
    }
    chk(found, "image_out binding present");

    tplc_shutdown(&ctx);
    printf("== %s (%d fail)\n", fails ? "FAIL" : "PASS", fails);
    return fails > 250 ? 250 : fails;
}
