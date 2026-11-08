/* ctl_smoke.cpp — end-to-end smoke: master (this process) + tether-plc
 * (child process) + stub module + control channel. Proves the whole
 * P0 stack works across a real process boundary (plan doc 03/04).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_image.h>
#include <tplc/tplc_ctl.h>
#include "../test_util.h"
#include "../../tools/ctl_client.h"

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

#define IMG_PATH "/tplc-smoke-img"
#define CTL_PATH "/dev/shm/tplc-smoke.ctl"

static int ctl_op(int fd, uint16_t op, const void* pl, uint32_t pl_len,
                  uint8_t* resp, uint32_t* resp_len, uint16_t* flags) {
    if (ctl_send(fd, op, pl, pl_len) != 0) return -1;
    uint64_t req;
    int t = ctl_recv(fd, resp, resp_len, flags, &req);
    return t;
}

int main(void) {
    signal(SIGPIPE, SIG_IGN);
    tplc_image_unlink(IMG_PATH);
    unlink(CTL_PATH);
    TplcImage master;
    CHECK_EQ(tplc_image_create(&master, IMG_PATH, 256, 256), 0);
    tplc_image_set_master_status(&master, TPLC_IMG_MASTER_RUN);
    tplc_image_master_heartbeat(&master, 1);

    pid_t pid = fork();
    CHECK(pid > 0);
    if (pid == 0) {
        execl(TETHER_PLC_PATH, "tether-plc",
              "--image", IMG_PATH,
              "--module", STUB_V1_PATH,
              "--ctl", CTL_PATH,
              "--wait-ms", "5000", nullptr);
        _exit(127);
    }

    /* wait for the ctl socket then hello */
    int fd = ctl_connect(CTL_PATH, 5000);
    CHECK(fd >= 0);
    if (fd < 0) { kill(pid, SIGKILL); TEST_REPORT("ctl_smoke"); }
    CHECK_EQ(ctl_hello(fd, "smoke"), 0);

    uint8_t resp[8192]; uint32_t rlen; uint16_t fl;

    /* ping */
    rlen = sizeof resp;
    CHECK_EQ(ctl_op(fd, TPLC_OP_PING, nullptr, 0, resp, &rlen, &fl),
             TPLC_OP_PING);
    CHECK(!(fl & TPLC_CTL_F_ERROR));
    CHECK_EQ(rlen, 8u);

    /* runtime status: 1 running program, plc_status run */
    rlen = sizeof resp;
    CHECK_EQ(ctl_op(fd, TPLC_OP_RT_STATUS, nullptr, 0, resp, &rlen, &fl),
             TPLC_OP_RT_STATUS);
    {
        const TplcCtlRtStatus* rs = (const TplcCtlRtStatus*)resp;
        CHECK_EQ(rs->prog_count, 1u);
        CHECK_EQ(rs->running_count, 1u);
        CHECK_EQ(rs->plc_status, TPLC_IMG_PLC_RUN);
    }

    /* prog status: running, correct name */
    rlen = sizeof resp;
    CHECK_EQ(ctl_op(fd, TPLC_OP_PROG_STATUS, nullptr, 0,
                    resp, &rlen, &fl), TPLC_OP_PROG_STATUS);
    {
        const TplcCtlProgStatus* ps = (const TplcCtlProgStatus*)resp;
        CHECK_EQ(ps->state, 1u);
        CHECK_EQ(ps->task_count, 2u);
        CHECK(strcmp((const char*)ps->name, "stub") == 0);
    }

    /* data flow: master publishes inputs, plc cycles produce %Q */
    uint8_t in_frame[8] = {0x01, 0, 0, 0, 0x42, 0, 0, 0};
    tplc_image_write_inputs(&master, in_frame, 0, 8);
    usleep(80 * 1000);                       /* several task cycles  */
    uint8_t out_frame[8] = {0};
    CHECK_EQ(tplc_image_read_outputs(&master, out_frame, 0, 8), 0);
    CHECK_EQ(out_frame[0] & 1, 1);           /* out_bit = in_bit     */
    CHECK_EQ(out_frame[4], 0x43);            /* out_byte = in+1      */

    /* task stats show real runs */
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_TASK_STATS, nullptr, 0, resp, &rlen, &fl);
    {
        const TplcCtlTaskStat* t =
            (const TplcCtlTaskStat*)(resp + sizeof(TplcCtlProgStatus));
        CHECK(t[0].runs > 0);
    }

    /* debug plane (doc 12): suspend freezes task 0 while task 1 keeps
       running; step adds exactly N; resume re-arms the schedule    */
    uint64_t runs0 = 0, runs1 = 0;
    {
        uint32_t ti = 0;
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_TASK_SUSPEND, &ti, 4,
                        resp, &rlen, &fl), TPLC_OP_TASK_SUSPEND);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
    }
    usleep(60 * 1000);
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_TASK_STATS, nullptr, 0, resp, &rlen, &fl);
    {
        const TplcCtlTaskStat* t =
            (const TplcCtlTaskStat*)(resp + sizeof(TplcCtlProgStatus));
        CHECK_EQ(t[0].state, 2u);           /* suspended */
        runs0 = t[0].runs;
        runs1 = t[1].runs;
    }
    usleep(60 * 1000);
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_TASK_STATS, nullptr, 0, resp, &rlen, &fl);
    {
        const TplcCtlTaskStat* t =
            (const TplcCtlTaskStat*)(resp + sizeof(TplcCtlProgStatus));
        CHECK_EQ(t[0].runs, runs0);         /* frozen    */
        CHECK(t[1].runs > runs1);           /* unaffected */
        runs1 = t[1].runs;
    }
    {
        uint32_t sr[2] = {0, 2};            /* task 0, 2 steps */
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_TASK_STEP, sr, 8,
                        resp, &rlen, &fl), TPLC_OP_TASK_STEP);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
    }
    usleep(60 * 1000);
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_TASK_STATS, nullptr, 0, resp, &rlen, &fl);
    {
        const TplcCtlTaskStat* t =
            (const TplcCtlTaskStat*)(resp + sizeof(TplcCtlProgStatus));
        CHECK_EQ(t[0].runs, runs0 + 2);     /* exactly 2  */
        CHECK(t[1].runs > runs1);
    }
    /* step on a non-suspended task is a state error */
    {
        uint32_t sr[2] = {1, 1};
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TASK_STEP, sr, 8, resp, &rlen, &fl);
        CHECK(fl & TPLC_CTL_F_ERROR);
    }
    {
        uint32_t ti = 0;
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_TASK_RESUME, &ti, 4,
                        resp, &rlen, &fl), TPLC_OP_TASK_RESUME);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
    }
    usleep(60 * 1000);
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_TASK_STATS, nullptr, 0, resp, &rlen, &fl);
    {
        const TplcCtlTaskStat* t =
            (const TplcCtlTaskStat*)(resp + sizeof(TplcCtlProgStatus));
        CHECK(t[0].runs > runs0 + 2);
        CHECK_EQ(t[0].state, 1u);           /* sched again */
    }
    /* stub module loads without a package: no SMAP → resolve fails
       with DBG_STATE, but blob extents still answer               */
    uint64_t blob_sz = 0;
    uint32_t l = 1;
    {
        uint8_t nm[8]; uint16_t nl = 4;
        memcpy(nm, &nl, 2); memcpy(nm + 2, "stub", 4);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_DBG_RESOLVE, nm, 6, resp, &rlen, &fl);
        CHECK(fl & TPLC_CTL_F_ERROR);
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_DBG_STATE, nullptr, 0,
                        resp, &rlen, &fl), TPLC_OP_DBG_STATE);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        CHECK_EQ(rlen, 44u);
        uint32_t sc;
        memcpy(&sc, resp + 40, 4);
        CHECK_EQ(sc, 0u);                   /* no symbols */
        memcpy(&blob_sz, resp + 16, 8);
    }

    /* flight recorder (doc 12 §7): arm a blob channel, frames accrue
       per scan while work runs, freeze only when every task is
       suspended, reads page by seq, clear disarms                 */
    {
        uint8_t cf[24] = {};
        uint32_t nch = 1; memcpy(cf, &nch, 4);
        l = blob_sz >= 4 ? 4 : 1;
        memcpy(cf + 16, &l, 4);
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_TRACE_CONF, cf, 24,
                        resp, &rlen, &fl), TPLC_OP_TRACE_CONF);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        uint32_t flen = 0; memcpy(&flen, resp, 4);
        CHECK_EQ(flen, l);
    }
    usleep(80 * 1000);
    {
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_TRACE_STATUS, nullptr, 0,
                        resp, &rlen, &fl), TPLC_OP_TRACE_STATUS);
        CHECK_EQ(rlen, 28u);
        uint64_t w, a;
        memcpy(&w, resp + 8, 8); memcpy(&a, resp + 16, 8);
        CHECK(w > 0);
        CHECK_EQ(a, w);
        uint8_t rd[12] = {};
        uint32_t mx = 4; memcpy(rd + 8, &mx, 4);
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_TRACE_READ, rd, 12,
                        resp, &rlen, &fl), TPLC_OP_TRACE_READ);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        uint32_t n, flen;
        memcpy(&n, resp + 8, 4); memcpy(&flen, resp + 12, 4);
        CHECK(n > 0);
        CHECK(n <= 4u);
        CHECK_EQ(flen, l);
        /* frame = {u64 seq, u64 ts, u8 data[flen]}; seq ascending */
        CHECK_EQ(rlen, 16 + n * (16 + flen));
        uint64_t s0, s1;
        memcpy(&s0, resp + 16, 8);
        if (n > 1) {
            memcpy(&s1, resp + 16 + 16 + flen, 8);
            CHECK(s1 > s0);
        }
    }
    /* both tasks suspended → no scan runs work → frames freeze */
    {
        uint32_t t0 = 0, t1 = 1;
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TASK_SUSPEND, &t0, 4, resp, &rlen, &fl);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TASK_SUSPEND, &t1, 4, resp, &rlen, &fl);
        usleep(60 * 1000);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TRACE_STATUS, nullptr, 0, resp, &rlen, &fl);
        uint64_t w1; memcpy(&w1, resp + 8, 8);
        usleep(60 * 1000);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TRACE_STATUS, nullptr, 0, resp, &rlen, &fl);
        uint64_t w2; memcpy(&w2, resp + 8, 8);
        CHECK_EQ(w2, w1);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TASK_RESUME, &t0, 4, resp, &rlen, &fl);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TASK_RESUME, &t1, 4, resp, &rlen, &fl);
    }
    /* clear disarms: subsequent reads are a state error */
    {
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_TRACE_CLEAR, nullptr, 0,
                        resp, &rlen, &fl), TPLC_OP_TRACE_CLEAR);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        uint8_t rd[12] = {};
        uint32_t mx = 4; memcpy(rd + 8, &mx, 4);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_TRACE_READ, rd, 12, resp, &rlen, &fl);
        CHECK(fl & TPLC_CTL_F_ERROR);
    }

    /* watch: add a two-item set, get returns boundary bytes,
       stream arm shows in LIST, remove drops it */
    uint32_t watch_h = 0;
    {
        uint8_t cf[8 + 2 * 14] = {};
        uint32_t ni = 2; memcpy(cf, &ni, 4);
        l = blob_sz >= 4 ? 4 : 1;
        memcpy(cf + 16, &l, 4);              /* item0: off 0 len l */
        memcpy(cf + 30, &l, 4);              /* item1: off 0 len l */
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_DBG_WATCH_ADD, cf, sizeof cf,
                        resp, &rlen, &fl), TPLC_OP_DBG_WATCH_ADD);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        memcpy(&watch_h, resp, 4);
        CHECK(watch_h > 0);
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_DBG_WATCH_GET, &watch_h, 4,
                        resp, &rlen, &fl), TPLC_OP_DBG_WATCH_GET);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        uint32_t n; memcpy(&n, resp, 4);
        CHECK_EQ(n, 2u);
        uint32_t i0; memcpy(&i0, resp + 4, 4);
        CHECK_EQ(i0, l);
        /* arm a 10 Hz stream */
        uint8_t st[8];
        memcpy(st, &watch_h, 4);
        uint32_t hz = 10; memcpy(st + 4, &hz, 4);
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_DBG_WATCH_STREAM, st, 8,
                        resp, &rlen, &fl), TPLC_OP_DBG_WATCH_STREAM);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_DBG_WATCH_LIST, nullptr, 0,
                        resp, &rlen, &fl), TPLC_OP_DBG_WATCH_LIST);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        uint32_t wl_n; memcpy(&wl_n, resp, 4);
        CHECK_EQ(wl_n, 1u);
        uint32_t wl_hz; memcpy(&wl_hz, resp + 12, 4);
        CHECK_EQ(wl_hz, 10u);
        /* remove */
        rlen = sizeof resp;
        CHECK_EQ(ctl_op(fd, TPLC_OP_DBG_WATCH_REMOVE, &watch_h, 4,
                        resp, &rlen, &fl), TPLC_OP_DBG_WATCH_REMOVE);
        CHECK(!(fl & TPLC_CTL_F_ERROR));
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_DBG_WATCH_GET, &watch_h, 4,
               resp, &rlen, &fl);
        CHECK(fl & TPLC_CTL_F_ERROR);
    }

    /* expression triggers: the stub carries no SMAP so every path
       fails resolve; malformed text fails the payload check        */
    {
        uint8_t bx[6 + 32];
        uint32_t ti = 0; memcpy(bx, &ti, 4);
        const char* ex = "nope > 1";
        uint16_t el = (uint16_t)strlen(ex);
        memcpy(bx + 4, &el, 2);
        memcpy(bx + 6, ex, el);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_DBG_BREAK_EXPR, bx, 6 + el,
               resp, &rlen, &fl);
        CHECK(fl & TPLC_CTL_F_ERROR);
        const char* bx2 = "x >>";
        el = (uint16_t)strlen(bx2);
        memcpy(bx + 4, &el, 2);
        memcpy(bx + 6, bx2, el);
        rlen = sizeof resp;
        ctl_op(fd, TPLC_OP_DBG_BREAK_EXPR, bx, 6 + el,
               resp, &rlen, &fl);
        CHECK(fl & TPLC_CTL_F_ERROR);
    }

    /* heartbeats: plc side is ticking */
    CHECK(tplc_image_plc_hb_ns(&master) > 0);

    /* stop: outputs stop changing */
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_PROG_STOP, nullptr, 0, resp, &rlen, &fl);
    {
        const TplcCtlProgStateChange* sc =
            (const TplcCtlProgStateChange*)resp;
        CHECK_EQ(sc->new_state, 0u);
    }
    in_frame[0] = 0x00; in_frame[4] = 0x77;
    tplc_image_write_inputs(&master, in_frame, 0, 8);
    usleep(50 * 1000);
    memset(out_frame, 0, 8);
    tplc_image_read_outputs(&master, out_frame, 0, 8);
    CHECK_EQ(out_frame[4], 0x43);            /* frozen: still old    */

    /* start again */
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_PROG_START, nullptr, 0, resp, &rlen, &fl);
    usleep(50 * 1000);
    memset(out_frame, 0, 8);
    tplc_image_read_outputs(&master, out_frame, 0, 8);
    CHECK_EQ(out_frame[4], 0x78);            /* live again (0x77+1)  */

    /* image stats op */
    rlen = sizeof resp;
    CHECK_EQ(ctl_op(fd, TPLC_OP_IMAGE_STATS, nullptr, 0,
                    resp, &rlen, &fl), TPLC_OP_IMAGE_STATS);

    /* graceful shutdown via control channel */
    rlen = sizeof resp;
    ctl_op(fd, TPLC_OP_RT_SHUTDOWN, nullptr, 0, resp, &rlen, &fl);
    close(fd);
    int wst = 0;
    waitpid(pid, &wst, 0);
    CHECK(WIFEXITED(wst));
    CHECK_EQ(WEXITSTATUS(wst), 0);
    CHECK_EQ(tplc_image_plc_status(&master), TPLC_IMG_PLC_STOPPED);

    tplc_image_close(&master);
    tplc_image_unlink(IMG_PATH);
    unlink(CTL_PATH);
    TEST_REPORT("ctl_smoke");
}
