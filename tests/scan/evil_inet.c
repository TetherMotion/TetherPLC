/* evil_inet.c — minimal valid module whose tplc_init reaches for the
 * network. Under the tether-plc sandbox (doc 15 §3) the socket()
 * filter only passes AF_UNIX, so module code calling AF_INET must
 * kill the process with SIGSYS — proving generated-code containment.
 * Built as a scan fixture; loaded by sandbox_enforce.sh.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#include <tplc/tplc_abi.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <string.h>

static int32_t main_task(TplcTaskCtx* tc) {
    (void)tc;
    return TPLC_OK;
}

static const TplcTaskFn task_table[] = { main_task };

static const TplcTaskDesc tasks[] = {
    { .name = "MAIN", .kind = TPLC_TASK_CYCLIC,
      .flags = TPLC_TASKF_ENABLED_AT_START,
      .priority = 10, .period_ns = 1000000, .wcet_hint_ns = 0,
      .entry_index = 0, .task_local_size = 0 },
};

static const TplcProgramDescriptor descriptor = {
    .struct_size = sizeof(TplcProgramDescriptor),
    .abi_version = TPLC_ABI_VERSION,
    .abi_ext = 0, .flags = 0,
    .name = "evil_inet",
    .state_blob_size = 16,
    .task_count = 1, .tasks = tasks,
    .task_fns = task_table, .task_fn_count = 1,
};

uint32_t tplc_abi_version(void) { return TPLC_ABI_VERSION; }

const TplcProgramDescriptor* tplc_get_descriptor(void) {
    return &descriptor;
}

int32_t tplc_init(TplcCtx* ctx) {
    memset(ctx->state_blob, 0, 16);
    /* forbidden under the UDS-only profile — the seccomp filter
     * kills the process right here */
    volatile int fd = socket(AF_INET, SOCK_STREAM, 0);
    (void)fd;
    return TPLC_OK;
}

void tplc_shutdown(TplcCtx* ctx) { (void)ctx; }

int32_t tplc_task_run(TplcTaskCtx* tctx) {
    if (!tctx || tctx->task_index != 0) return TPLC_TASK_E_FAULT;
    return main_task(tctx);
}
