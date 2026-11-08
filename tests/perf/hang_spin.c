/* hang_spin.c — perf-evidence fixture: a module whose task spins
   uncooperatively (never checks abort_flag), used to measure the
   watchdog's stall→fault→image-flag latency against the
   --hang-timeout-ms budget. Loads via --module (no package needed).
   SPDX-License-Identifier: Apache-2.0 OR MPL-2.0                  */

#include <tplc/tplc_abi.h>
#include <string.h>

#define SPIN_BLOB_SIZE 16

static int32_t spin_task(TplcTaskCtx* tc) {
    (void)tc;
    for (;;) { }                     /* uncooperative: no checkpoints */
    return TPLC_OK;
}

static const TplcTaskFn task_table[] = { spin_task };

static const TplcTaskDesc tasks[] = {
    { .name = "SPIN", .kind = TPLC_TASK_CYCLIC,
      .flags = TPLC_TASKF_ENABLED_AT_START | TPLC_TASKF_WATCHDOGGED,
      .priority = 10, .period_ns = 1000000, .wcet_hint_ns = 50000,
      .entry_index = 0, .task_local_size = 0 },
};

static const TplcProgramDescriptor descriptor = {
    .struct_size  = sizeof(TplcProgramDescriptor),
    .abi_version  = TPLC_ABI_VERSION,
    .abi_ext      = 0,
    .flags        = 0,
    .program_uuid = {0x51,0x50,0x4c,0x43,0,0,0,0,0,0,0,0,0,0,0,0x5A},
    .layout_hash  = {0xC3,0xC3,0xC3,0xC3},
    .name         = "hang_spin",
    .state_blob_size = SPIN_BLOB_SIZE,
    .retain_size     = 0,
    .retain_offset   = 0,
    .task_scratch_size = 0,
    .image_in_bytes  = 0,
    .image_out_bytes = 0,
    .marker_bytes    = 0,
    .task_count = 1, .tasks = tasks,
    .task_fns = task_table, .task_fn_count = 1,
    .io_bindings = 0, .io_binding_count = 0,
};

uint32_t tplc_abi_version(void) { return TPLC_ABI_VERSION; }

const TplcProgramDescriptor* tplc_get_descriptor(void) {
    return &descriptor;
}

int32_t tplc_init(TplcCtx* ctx) {
    if (!ctx || ctx->struct_size != sizeof(TplcCtx) || !ctx->state_blob)
        return TPLC_E_INIT_INVALID;
    if (SPIN_BLOB_SIZE != descriptor.state_blob_size)
        return TPLC_E_INIT_LAYOUT;
    memset(ctx->state_blob, 0, SPIN_BLOB_SIZE);
    return TPLC_OK;
}

void tplc_shutdown(TplcCtx* ctx) { (void)ctx; }

int32_t tplc_task_run(TplcTaskCtx* tctx) {
    if (!tctx || tctx->task_index >= descriptor.task_count)
        return TPLC_TASK_E_FAULT;
    return task_table[descriptor.tasks[tctx->task_index].entry_index](tctx);
}
