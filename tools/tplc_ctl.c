/* tplc-ctl — control-channel client CLI.
 *   tplc-ctl <sock> ping|status|prog|list|start|stop|image|shutdown
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_ctl.h>
#include "ctl_client.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void print_prog_status(const uint8_t* p, uint32_t len) {
    if (len < sizeof(TplcCtlProgStatus)) { puts("(short payload)"); return; }
    const TplcCtlProgStatus* ps = (const TplcCtlProgStatus*)p;
    static const char* states[] = {"stopped", "running", "fault"};
    printf("program    %s  state %s  uuid %02x%02x%02x%02x…\n",
           ps->name, states[ps->state <= 2 ? ps->state : 3],
           ps->program_uuid[0], ps->program_uuid[1],
           ps->program_uuid[2], ps->program_uuid[3]);
    printf("cycles     %lu   last_fault %d   tasks %u\n",
           (unsigned long)ps->cycles, ps->last_fault_code,
           ps->task_count);
    const TplcCtlTaskStat* t =
        (const TplcCtlTaskStat*)(p + sizeof(TplcCtlProgStatus));
    uint32_t n = ps->task_count;
    if (sizeof(TplcCtlProgStatus) + n * sizeof(TplcCtlTaskStat) > len)
        n = (len - sizeof(TplcCtlProgStatus)) / sizeof(TplcCtlTaskStat);
    for (uint32_t i = 0; i < n; i++) {
        printf("  task %-8s kind %u period %lu ns  runs %lu  "
               "late %lu (max %d ns)  faults %lu\n",
               t[i].name, t[i].kind,
               (unsigned long)t[i].period_ns,
               (unsigned long)t[i].runs,
               (unsigned long)t[i].late,
               t[i].last_late_ns,
               (unsigned long)t[i].faults);
    }
}

static int check_err(uint16_t fl, const uint8_t* p, uint32_t len) {
    if (!(fl & TPLC_CTL_F_ERROR)) return 0;
    if (len >= sizeof(TplcCtlError)) {
        const TplcCtlError* e = (const TplcCtlError*)p;
        uint16_t ml = 0;
        if (len >= sizeof(TplcCtlError) + 2)
            memcpy(&ml, p + sizeof(TplcCtlError), 2);
        fprintf(stderr, "error %u: %.*s\n", e->code,
                (int)(ml < 200 ? ml : 200),
                len > sizeof(TplcCtlError) + 2
                    ? (const char*)(p + sizeof(TplcCtlError) + 2) : "");
    }
    return 1;
}

int main(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr,
            "usage: %s <sock> ping|status|prog|list|start|stop|image"
            "|shutdown\n", argv[0]);
        return 2;
    }
    const char* sock = argv[1];
    const char* cmd = argv[2];

    uint16_t op;
    if      (!strcmp(cmd, "ping"))     op = TPLC_OP_PING;
    else if (!strcmp(cmd, "status"))   op = TPLC_OP_RT_STATUS;
    else if (!strcmp(cmd, "prog"))     op = TPLC_OP_PROG_STATUS;
    else if (!strcmp(cmd, "list"))     op = TPLC_OP_PROG_LIST;
    else if (!strcmp(cmd, "start"))    op = TPLC_OP_PROG_START;
    else if (!strcmp(cmd, "stop"))     op = TPLC_OP_PROG_STOP;
    else if (!strcmp(cmd, "image"))    op = TPLC_OP_IMAGE_STATS;
    else if (!strcmp(cmd, "shutdown")) op = TPLC_OP_RT_SHUTDOWN;
    else { fprintf(stderr, "unknown cmd %s\n", cmd); return 2; }

    int fd = ctl_connect(sock, 3000);
    if (fd < 0) { perror("connect"); return 1; }
    if (ctl_hello(fd, "tplc-ctl") != 0) {
        fprintf(stderr, "hello failed\n"); return 1;
    }
    if (ctl_send(fd, op, NULL, 0) != 0) { perror("send"); return 1; }

    uint8_t pl[64 * 1024]; uint32_t len = sizeof pl;
    uint16_t fl; uint64_t req;
    int t = ctl_recv(fd, pl, &len, &fl, &req);
    if (t < 0) { fprintf(stderr, "recv failed\n"); return 1; }
    if (check_err(fl, pl, len)) { close(fd); return 1; }

    if (op == TPLC_OP_PROG_STATUS || op == TPLC_OP_TASK_STATS)
        print_prog_status(pl, len);
    else if (op == TPLC_OP_RT_STATUS) {
        const TplcCtlRtStatus* rs = (const TplcCtlRtStatus*)pl;
        printf("runtime    %s  proto %u  uptime %u s\n",
               rs->version, rs->proto_ver, rs->uptime_s);
        printf("programs   %u (%u running)\n",
               rs->prog_count, rs->running_count);
        printf("master_st  %u  hb %lu\n", rs->master_status,
               (unsigned long)rs->master_hb_ns);
        printf("plc_st     %u  hb %lu\n", rs->plc_status,
               (unsigned long)rs->plc_hb_ns);
    }
    else if (op == TPLC_OP_IMAGE_STATS) {
        const TplcCtlImageStats* is = (const TplcCtlImageStats*)pl;
        printf("epoch %lu  in_seq %lu  out_seq %lu\n",
               (unsigned long)is->epoch, (unsigned long)is->in_seq,
               (unsigned long)is->out_seq);
        printf("in  size %lu writes %lu retries %lu\n",
               (unsigned long)is->in_size,
               (unsigned long)is->in_writes,
               (unsigned long)is->in_retries);
        printf("out size %lu writes %lu retries %lu\n",
               (unsigned long)is->out_size,
               (unsigned long)is->out_writes,
               (unsigned long)is->out_retries);
    }
    else if (op == TPLC_OP_PROG_START || op == TPLC_OP_PROG_STOP) {
        const TplcCtlProgStateChange* sc =
            (const TplcCtlProgStateChange*)pl;
        printf("state %u -> %u\n", sc->prev_state, sc->new_state);
    }
    else if (op == TPLC_OP_PROG_LIST) {
        const uint8_t* q = pl;
        uint32_t n; memcpy(&n, q, 4); q += 4;
        for (uint32_t i = 0; i < n; i++) {
            printf("uuid %02x… ", q[0]); q += 16;
            uint16_t nl; memcpy(&nl, q, 2); q += 2;
            printf("%.*s state %u\n", nl, (const char*)q, q[nl + 3]);
            q += nl + 4;
        }
    }
    else if (op == TPLC_OP_PING && len >= 8) {
        uint64_t ns; memcpy(&ns, pl, 8);
        printf("pong %lu ns\n", (unsigned long)ns);
    }
    close(fd);
    return 0;
}
