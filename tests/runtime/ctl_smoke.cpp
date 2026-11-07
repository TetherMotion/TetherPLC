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
