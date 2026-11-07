/* fault_injection.cpp — P0-09 harness: cooperative-abort (hang) path,
 * kill -9 of the real runtime process, and a cross-process seq-odd
 * adversary (crashed writer). Proves doc-03/04 promises: bounded reads,
 * no torn data, liveness detection, no crash contagion, restart works.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_image.h>
#include <tplc/host/Module.hpp>
#include <tplc/host/Program.hpp>
#include "../test_util.h"

#include <csignal>
#include <cstring>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#ifndef TETHER_PLC_PATH
#define TETHER_PLC_PATH "./tether-plc"
#endif
#ifndef STUB_V1_PATH
#define STUB_V1_PATH "./stub_v1.so"
#endif

#define IMG_PATH  "/tplc-fault-img"
#define IMG_PATH2 "/tplc-fault-img2"

static uint64_t now_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

/* ================================================================== */
/* Scenario A — cooperative abort: the hang path (doc 04 §5).          */
/* The stub's fast task polls abort_flag inside its spin loop — the    */
/* same checkpoint pattern codegen will emit at back-edges. Abort must */
/* surface TPLC_TASK_E_ABORTED and skip the %Q commit for that cycle.  */
/* ================================================================== */
static int latch_ok(tplc::Program&, uint32_t) { return 0; }

static void scenario_abort(void) {
    auto mres = tplc::Module::open(STUB_V1_PATH, 0);
    CHECK(mres.ok());
    if (!mres.ok()) return;
    tplc::Module mod = std::move(mres.value);

    tplc::ProgramConfig cfg;
    auto pres = tplc::Program::create(mod, cfg);
    CHECK(pres.ok());
    if (!pres.ok()) return;
    tplc::Program prog = std::move(pres.value);
    CHECK(prog.start().ok());

    /* drive %I so the task writes a recognizable %Q */
    prog.imageIn()[0] = 0x01;
    prog.imageIn()[4] = 0x10;
    bool committed = false;
    auto commit = [&](tplc::Program&, uint32_t) { committed = true; return 0; };

    CHECK_EQ(prog.executeTask(0, 1000, latch_ok, commit), TPLC_OK);
    CHECK(committed);
    CHECK_EQ(prog.imageOut()[4], 0x11);         /* in_byte + 1 */

    /* raise abort → next cycle aborts at the checkpoint, no commit */
    committed = false;
    prog.requestAbort();
    prog.imageIn()[4] = 0x20;
    CHECK_EQ(prog.executeTask(0, 2000, latch_ok, commit),
             TPLC_TASK_E_ABORTED);
    CHECK(!committed);                          /* %Q untouched  */
    CHECK_EQ(prog.imageOut()[4], 0x11);         /* old value kept */

    /* the abort_flag is latched by the module read, not auto-cleared —
       host clears it (doc 02 §2.5 host-owned ctx semantics) */
    prog.clearAbort();
    committed = false;
    CHECK_EQ(prog.executeTask(0, 3000, latch_ok, commit), TPLC_OK);
    CHECK(committed);
    CHECK_EQ(prog.imageOut()[4], 0x21);         /* live again */

    prog.stop();
}

/* ================================================================== */
/* Scenario B — kill -9 of tether-plc (doc 03 §6.1 liveness).          */
/* Master must observe heartbeat stall; the image itself must stay     */
/* consistent; a fresh runtime must be able to reattach and run.       */
/* ================================================================== */
static void scenario_kill(void) {
    tplc_image_unlink(IMG_PATH);
    TplcImage master;
    CHECK_EQ(tplc_image_create(&master, IMG_PATH, 256, 256), 0);
    tplc_image_set_master_status(&master, TPLC_IMG_MASTER_RUN);
    tplc_image_master_heartbeat(&master, 1);

    pid_t pid = fork();
    if (pid == 0) {
        execl(TETHER_PLC_PATH, "tether-plc",
              "--image", IMG_PATH,
              "--module", STUB_V1_PATH,
              "--ctl", "none",             /* headless */
              "--wait-ms", "5000", nullptr);
        _exit(127);
    }
    CHECK(pid > 0);

    /* wait for the plc heartbeat to start ticking (attached + running) */
    uint64_t hb1 = 0;
    for (int i = 0; i < 200; i++) {
        usleep(10 * 1000);
        hb1 = tplc_image_plc_hb_ns(&master);
        if (hb1) break;
    }
    CHECK(hb1 > 0);
    uint64_t hb2 = hb1;
    for (int i = 0; i < 200 && hb2 == hb1; i++) {
        usleep(10 * 1000);
        hb2 = tplc_image_plc_hb_ns(&master);
    }
    CHECK(hb2 > hb1);                          /* was ticking */

    /* SIGKILL — the crash case; heartbeat must stall */
    kill(pid, SIGKILL);
    int wst = 0;
    waitpid(pid, &wst, 0);
    CHECK(WIFSIGNALED(wst) && WTERMSIG(wst) == SIGKILL);

    uint64_t dead_hb = tplc_image_plc_hb_ns(&master);
    usleep(60 * 1000);
    CHECK_EQ(tplc_image_plc_hb_ns(&master), dead_hb);  /* frozen */

    /* staleness rule detects loss within a bounded window */
    CHECK(tplc_image_hb_stale(dead_hb, now_ns(), 5000000ull /*5ms*/));

    /* image is NOT corrupted by the crash: master writes/reads fine */
    uint8_t f[8] = {1,0,0,0,0x55,0,0,0};
    CHECK_EQ(tplc_image_write_inputs(&master, f, 0, 8), 0);
    uint8_t o[8] = {0};
    CHECK_EQ(tplc_image_read_outputs(&master, o, 0, 8), 0); /* last good */

    /* restart: a fresh tether-plc reattaches to the same image */
    pid_t pid2 = fork();
    if (pid2 == 0) {
        execl(TETHER_PLC_PATH, "tether-plc",
              "--image", IMG_PATH,
              "--module", STUB_V1_PATH,
              "--ctl", "none",
              "--wait-ms", "5000", nullptr);
        _exit(127);
    }
    CHECK(pid2 > 0);
    uint64_t hb3 = dead_hb;
    for (int i = 0; i < 300 && hb3 <= dead_hb; i++) {
        usleep(10 * 1000);
        hb3 = tplc_image_plc_hb_ns(&master);
    }
    CHECK(hb3 > dead_hb);                       /* new runtime ticks  */

    /* new instance re-inits and runs: %Q data flow live again */
    usleep(60 * 1000);
    uint8_t o2[8] = {0};
    CHECK_EQ(tplc_image_read_outputs(&master, o2, 0, 8), 0);
    CHECK_EQ(o2[4], 0x56);                      /* 0x55 + 1          */

    kill(pid2, SIGKILL);                        /* teardown          */
    waitpid(pid2, &wst, 0);

    tplc_image_close(&master);
    tplc_image_unlink(IMG_PATH);
}

/* ================================================================== */
/* Scenario C — seq-odd adversary (crashed writer, cross-process).     */
/* A child holds out_seq odd (dies inside the write bracket); the      */
/* master's read_outputs must exhaust retries bounded, return no torn  */
/* data, and recover once the sequence is repaired.                    */
/* ================================================================== */
static void scenario_seqodd(void) {
    tplc_image_unlink(IMG_PATH2);
    TplcImage master;
    CHECK_EQ(tplc_image_create(&master, IMG_PATH2, 128, 128), 0);

    /* baseline good data committed by a healthy writer */
    {
        TplcImage plc;
        CHECK_EQ(tplc_image_attach(&plc, IMG_PATH2, 1000), 0);
        uint8_t good[8] = {0xAA,1,2,3,4,5,6,7};
        CHECK_EQ(tplc_image_commit_outputs(&plc, good, 0, 8), 0);
        tplc_image_close(&plc);
    }
    uint8_t rd[8] = {0};
    CHECK_EQ(tplc_image_read_outputs(&master, rd, 0, 8), 0);
    CHECK_EQ(rd[0], 0xAA);

    /* child: attach as plc, force out_seq odd, then freeze forever —
       a writer that died mid-commit */
    pid_t pid = fork();
    if (pid == 0) {
        TplcImage plc;
        if (tplc_image_attach(&plc, IMG_PATH2, 5000) != 0) _exit(2);
        /* seq bracket open, never closed: odd value = write in flight */
        uint64_t s = tplc_img_load_u64(&plc.hdr->out_seq);
        tplc_img_store_u64(&plc.hdr->out_seq, s | 1u);
        /* also scribble a half-written payload — torn on purpose */
        memset(plc.out_area, 0xEE, 8);
        pause();
        _exit(0);
    }
    CHECK(pid > 0);

    /* give the child a moment to open the bracket */
    usleep(30 * 1000);

    /* bounded read: exhausted retries, dst keeps old (not torn) data */
    memset(rd, 0, sizeof rd);
    int rc = tplc_image_read_outputs(&master, rd, 0, 8);
    CHECK_EQ(rc, TPLC_E_IMG_SEQ_EXHAUSTED);
    bool torn = false;
    for (int i = 0; i < 8; i++)
        if (rd[i] == 0xEE) torn = true;
    CHECK(!torn);                                /* no partial copy   */
    CHECK(tplc_image_out_retries(&master) > 0);  /* retries counted   */

    /* kill -9 the stuck writer — seq stays odd; still bounded */
    kill(pid, SIGKILL);
    int wst = 0;
    waitpid(pid, &wst, 0);
    CHECK(WIFSIGNALED(wst));
    CHECK_EQ(tplc_image_read_outputs(&master, rd, 0, 8),
             TPLC_E_IMG_SEQ_EXHAUSTED);

    /* recovery path: repairing the seq (a fresh writer would own it)
       makes reads succeed again — with the torn bytes now visible as
       committed data, which is the documented crash-window semantic
       (writer died mid-write: content may be the partial write). */
    tplc_img_store_u64(&master.hdr->out_seq,
                       tplc_img_load_u64(&master.hdr->out_seq) + 1);
    memset(rd, 0, sizeof rd);
    CHECK_EQ(tplc_image_read_outputs(&master, rd, 0, 8), 0);

    tplc_image_close(&master);
    tplc_image_unlink(IMG_PATH2);
}

int main(void) {
    signal(SIGPIPE, SIG_IGN);
    scenario_abort();
    scenario_kill();
    scenario_seqodd();
    TEST_REPORT("fault_injection");
}
