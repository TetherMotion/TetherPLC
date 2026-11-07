/* ctl_client.h — minimal control-channel client helper (header-only).
 * Used by tplc-ctl and tests. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_CTL_CLIENT_H
#define TPLC_CTL_CLIENT_H

#include <tplc/tplc_ctl.h>

#include <errno.h>
#include <stdint.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

static int ctl_connect(const char* path, int timeout_ms) {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -1;
    struct sockaddr_un addr;
    memset(&addr, 0, sizeof addr);
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, path, sizeof addr.sun_path - 1);
    long waited = 0;
    for (;;) {
        if (connect(fd, (struct sockaddr*)&addr, sizeof addr) == 0)
            break;
        if (errno != ENOENT && errno != ECONNREFUSED) {
            close(fd); return -1;
        }
        if (waited >= timeout_ms) { close(fd); return -1; }
        usleep(20 * 1000); waited += 20;
    }
    return fd;
}

static uint64_t ctl_req_id = 1;

static int ctl_send(int fd, uint16_t type, const void* payload,
                    uint32_t len) {
    uint8_t hdr[TPLC_CTL_HDR_SIZE];
    uint64_t req = ctl_req_id++;
    hdr[0] = type & 0xFF; hdr[1] = type >> 8;
    hdr[2] = hdr[3] = 0;
    for (int i = 0; i < 4; i++) hdr[4 + i] = len >> (8 * i);
    for (int i = 0; i < 8; i++) hdr[8 + i] = req >> (8 * i);
    if (send(fd, hdr, sizeof hdr, MSG_NOSIGNAL) != (ssize_t)sizeof hdr)
        return -1;
    if (len && send(fd, payload, len, MSG_NOSIGNAL) != (ssize_t)len)
        return -1;
    return 0;
}

/* recv one frame; returns msg_type or -1/-2. flags_io/req_io filled.
 * len_io is in=capacity, out=received. */
static int ctl_recv(int fd, uint8_t* payload, uint32_t* len_io,
                    uint16_t* flags_io, uint64_t* req_io) {
    uint8_t hdr[TPLC_CTL_HDR_SIZE];
    uint32_t got = 0;
    while (got < sizeof hdr) {
        ssize_t n = read(fd, hdr + got, sizeof hdr - got);
        if (n <= 0) return -1;
        got += (uint32_t)n;
    }
    uint16_t type  = hdr[0] | (hdr[1] << 8);
    uint16_t flags = hdr[2] | (hdr[3] << 8);
    uint32_t len   = hdr[4] | (hdr[5] << 8) | (hdr[6] << 16) |
                     (hdr[7] << 24);
    uint64_t req   = 0;
    for (int i = 0; i < 8; i++) req |= (uint64_t)hdr[8 + i] << (8 * i);
    if (len > *len_io) return -2;         /* caller's buffer too small */
    got = 0;
    while (got < len) {
        ssize_t n = read(fd, payload + got, len - got);
        if (n <= 0) return -1;
        got += (uint32_t)n;
    }
    *len_io = len;
    if (flags_io) *flags_io = flags;
    if (req_io) *req_io = req;
    return type;
}

/* session bootstrap: send hello, expect ack */
static int ctl_hello(int fd, const char* name) {
    uint8_t buf[256];
    TplcCtlHello h;
    memset(&h, 0, sizeof h);
    h.proto_ver = TPLC_CTL_PROTO_VER;
    memcpy(buf, &h, sizeof h);
    uint16_t nl = (uint16_t)strlen(name);
    memcpy(buf + sizeof h, &nl, 2);
    memcpy(buf + sizeof h + 2, name, nl);
    if (ctl_send(fd, TPLC_OP_HELLO, buf, (uint32_t)(sizeof h + 2 + nl)))
        return -1;
    uint8_t pl[1024]; uint32_t len = sizeof pl;
    uint16_t fl; uint64_t req;
    int t = ctl_recv(fd, pl, &len, &fl, &req);
    if (t != TPLC_OP_HELLO_ACK) return -1;
    return 0;
}

#endif /* TPLC_CTL_CLIENT_H */
