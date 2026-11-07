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
    static const char* states[] = {"stopped", "running", "fault",
                                   "empty"};
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

/* minimal sha256 for deploy hashing */
typedef struct {
    uint32_t h[8]; uint64_t bits; uint8_t buf[64]; size_t buflen;
} Sha;
static uint32_t srr(uint32_t x, int n){return (x>>n)|(x<<(32-n));}
static void sblock(Sha* s, const uint8_t* p) {
    static const uint32_t K[64] = {
        0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,
        0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,
        0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,
        0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,
        0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,
        0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,
        0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,
        0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
        0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,
        0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,
        0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,
        0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,
        0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2};
    uint32_t w[64];
    for (int i = 0; i < 16; i++)
        w[i] = ((uint32_t)p[i*4]<<24)|((uint32_t)p[i*4+1]<<16)|
               ((uint32_t)p[i*4+2]<<8)|p[i*4+3];
    for (int i = 16; i < 64; i++) {
        uint32_t s0 = srr(w[i-15],7)^srr(w[i-15],18)^(w[i-15]>>3);
        uint32_t s1 = srr(w[i-2],17)^srr(w[i-2],19)^(w[i-2]>>10);
        w[i] = w[i-16]+s0+w[i-7]+s1;
    }
    uint32_t a=s->h[0],b=s->h[1],c=s->h[2],d=s->h[3],e=s->h[4],
             f=s->h[5],g=s->h[6],hh=s->h[7];
    for (int i = 0; i < 64; i++) {
        uint32_t S1 = srr(e,6)^srr(e,11)^srr(e,25);
        uint32_t ch = (e&f)^(~e&g);
        uint32_t t1 = hh+S1+ch+K[i]+w[i];
        uint32_t S0 = srr(a,2)^srr(a,13)^srr(a,22);
        uint32_t mj = (a&b)^(a&c)^(b&c);
        hh=g; g=f; f=e; e=d+t1; d=c; c=b; b=a; a=t1+S0+mj;
    }
    s->h[0]+=a;s->h[1]+=b;s->h[2]+=c;s->h[3]+=d;
    s->h[4]+=e;s->h[5]+=f;s->h[6]+=g;s->h[7]+=hh;
}
static void sha_init(Sha* s) {
    static const uint32_t IV[8] = {
        0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,
        0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19};
    memcpy(s->h, IV, sizeof s->h); s->bits = 0; s->buflen = 0;
}
static void sha_upd(Sha* s, const uint8_t* p, size_t n) {
    s->bits += (uint64_t)n * 8;
    while (n) {
        size_t take = 64 - s->buflen;
        if (take > n) take = n;
        memcpy(s->buf + s->buflen, p, take);
        s->buflen += take; p += take; n -= take;
        if (s->buflen == 64) { sblock(s, s->buf); s->buflen = 0; }
    }
}
static void sha_fin(Sha* s, uint8_t out[32]) {
    s->buf[s->buflen++] = 0x80;
    if (s->buflen > 56) {
        while (s->buflen < 64) s->buf[s->buflen++] = 0;
        sblock(s, s->buf); s->buflen = 0;
    }
    while (s->buflen < 56) s->buf[s->buflen++] = 0;
    for (int i = 7; i >= 0; i--)
        s->buf[s->buflen++] = (uint8_t)(s->bits >> (i*8));
    sblock(s, s->buf);
    for (int i = 0; i < 8; i++) {
        out[i*4]=(uint8_t)(s->h[i]>>24);out[i*4+1]=(uint8_t)(s->h[i]>>16);
        out[i*4+2]=(uint8_t)(s->h[i]>>8);out[i*4+3]=(uint8_t)s->h[i];
    }
}
static int sha_file(const char* path, uint8_t out[32],
                    uint64_t* size_out) {
    FILE* f = fopen(path, "rb");
    if (!f) return -1;
    Sha s; sha_init(&s);
    uint8_t buf[8192]; size_t n; uint64_t total = 0;
    while ((n = fread(buf, 1, sizeof buf, f)) > 0) {
        sha_upd(&s, buf, n); total += n;
    }
    fclose(f);
    sha_fin(&s, out);
    *size_out = total;
    return 0;
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

/* deploy <file>: begin → chunks → commit, single round-trips */
static int do_deploy(const char* sock, const char* path) {
    uint8_t sha[32]; uint64_t sz;
    if (sha_file(path, sha, &sz) != 0) {
        fprintf(stderr, "cannot read %s\n", path); return 1;
    }
    int fd = ctl_connect(sock, 3000);
    if (fd < 0) { perror("connect"); return 1; }
    if (ctl_hello(fd, "tplc-ctl") != 0) {
        fprintf(stderr, "hello failed\n"); return 1;
    }
    uint8_t pl[80 * 1024]; uint32_t len; uint16_t fl; uint64_t req;

    uint8_t begin[41];
    memcpy(begin, &sz, 8);
    memcpy(begin + 8, sha, 32);
    begin[40] = 0;                    /* mode: cold */
    len = sizeof pl;
    if (ctl_send(fd, TPLC_OP_DEPLOY_BEGIN, begin, sizeof begin) ||
        ctl_recv(fd, pl, &len, &fl, &req) < 0 ||
        check_err(fl, pl, len)) { close(fd); return 1; }
    uint64_t upid; uint32_t maxc;
    memcpy(&upid, pl, 8); memcpy(&maxc, pl + 8, 4);
    if (maxc == 0 || maxc > 48 * 1024) maxc = 48 * 1024;

    FILE* f = fopen(path, "rb");
    uint8_t* frame = malloc(16 + maxc);
    memcpy(frame, &upid, 8);
    uint64_t off = 0; size_t n;
    while ((n = fread(frame + 16, 1, maxc, f)) > 0) {
        memcpy(frame + 8, &off, 8);
        len = sizeof pl;
        if (ctl_send(fd, TPLC_OP_DEPLOY_CHUNK, frame,
                     (uint32_t)(16 + n)) ||
            ctl_recv(fd, pl, &len, &fl, &req) < 0 ||
            check_err(fl, pl, len)) {
            fclose(f); free(frame); close(fd); return 1;
        }
        off += n;
    }
    fclose(f); free(frame);

    len = sizeof pl;
    if (ctl_send(fd, TPLC_OP_DEPLOY_COMMIT, &upid, 8) ||
        ctl_recv(fd, pl, &len, &fl, &req) < 0 ||
        check_err(fl, pl, len)) { close(fd); return 1; }
    printf("deployed  uuid ");
    for (int i = 0; i < 16 && 1 + i < len; i++)
        printf("%02x", pl[1 + i]);
    printf("  (%lu B)\n", (unsigned long)sz);
    close(fd);
    return 0;
}

int main(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr,
            "usage: %s <sock> ping|status|prog|list|start|stop|image"
            "|shutdown|kill|reset [warm|cold]|unload|prepare X"
            "|pkg|deploy F\n", argv[0]);
        return 2;
    }
    const char* sock = argv[1];
    const char* cmd = argv[2];
    const char* arg = argc > 3 ? argv[3] : NULL;

    uint16_t op = 0;
    uint8_t pre[64 * 1024]; uint32_t pre_len = 0;
    if      (!strcmp(cmd, "ping"))     op = TPLC_OP_PING;
    else if (!strcmp(cmd, "status"))   op = TPLC_OP_RT_STATUS;
    else if (!strcmp(cmd, "prog"))     op = TPLC_OP_PROG_STATUS;
    else if (!strcmp(cmd, "list"))     op = TPLC_OP_PROG_LIST;
    else if (!strcmp(cmd, "start"))    op = TPLC_OP_PROG_START;
    else if (!strcmp(cmd, "stop"))     op = TPLC_OP_PROG_STOP;
    else if (!strcmp(cmd, "image"))    op = TPLC_OP_IMAGE_STATS;
    else if (!strcmp(cmd, "shutdown")) op = TPLC_OP_RT_SHUTDOWN;
    else if (!strcmp(cmd, "kill"))     op = TPLC_OP_PROG_KILL;
    else if (!strcmp(cmd, "unload"))   op = TPLC_OP_PROG_UNLOAD;
    else if (!strcmp(cmd, "pkg"))      op = TPLC_OP_PKG_LIST;
    else if (!strcmp(cmd, "reset")) {
        op = TPLC_OP_PROG_RESET;
        pre[0] = (arg && !strcmp(arg, "cold")) ? 0 : 1;
        pre_len = 1;
    }
    else if (!strcmp(cmd, "prepare")) {
        op = TPLC_OP_PROG_PREPARE;
        if (!arg) { fprintf(stderr, "prepare <path|uuid>\n"); return 2; }
        size_t al = strlen(arg);
        uint8_t u[16];
        if (al == 32) {               /* hex uuid → raw 16B form */
            int ok = 1;
            for (int i = 0; i < 16 && ok; i++) {
                int v; char t[3] = {arg[i*2], arg[i*2+1], 0};
                if (sscanf(t, "%02x", &v) != 1) ok = 0; else u[i]=(uint8_t)v;
            }
            if (!ok) { fprintf(stderr, "bad uuid\n"); return 2; }
            memcpy(pre, u, 16); pre_len = 16;
        } else {
            uint16_t pl = (uint16_t)al;
            memcpy(pre, &pl, 2);
            memcpy(pre + 2, arg, al);
            pre_len = 2 + pl;
        }
    }
    else if (!strcmp(cmd, "deploy")) {
        if (!arg) { fprintf(stderr, "deploy <file.tplc>\n"); return 2; }
        return do_deploy(sock, arg);
    }
    else { fprintf(stderr, "unknown cmd %s\n", cmd); return 2; }

    int fd = ctl_connect(sock, 3000);
    if (fd < 0) { perror("connect"); return 1; }
    if (ctl_hello(fd, "tplc-ctl") != 0) {
        fprintf(stderr, "hello failed\n"); return 1;
    }
    if (ctl_send(fd, op, pre, pre_len) != 0) { perror("send"); return 1; }

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
    else if (op == TPLC_OP_PROG_START || op == TPLC_OP_PROG_STOP ||
             op == TPLC_OP_PROG_KILL || op == TPLC_OP_PROG_RESET ||
             op == TPLC_OP_PROG_UNLOAD || op == TPLC_OP_PROG_PREPARE) {
        const TplcCtlProgStateChange* sc =
            (const TplcCtlProgStateChange*)pl;
        printf("state %u -> %u\n", sc->prev_state, sc->new_state);
    }
    else if (op == TPLC_OP_PKG_LIST) {
        const uint8_t* q = pl;
        uint32_t n; memcpy(&n, q, 4); q += 4;
        printf("packages %u\n", n);
        for (uint32_t i = 0; i < n; i++) {
            printf("  uuid ");
            for (int b = 0; b < 16; b++) printf("%02x", q[b]);
            q += 16;
            uint64_t sz; memcpy(&sz, q, 8); q += 8;
            printf("  %lu B\n", (unsigned long)sz);
        }
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
