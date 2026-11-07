/* tplc_ctl_proxy — HTTP/1.1 pass-through to the tether-plc control
 * channel (UDS). Browsers cannot reach AF_UNIX sockets; this single-
 * threaded bridge exposes `POST /ctl` so the web IDE can issue ctl ops.
 *
 *   POST /ctl   body = one complete ctl request frame (16B hdr + payload)
 *               response = one complete reply frame, verbatim
 *   GET /health "ok" when the ctl socket connects, else 503
 *   OPTIONS     CORS preflight (Access-Control-Allow-Origin: *)
 *
 * Limitations: SET_NOTIFY events are not carried (poll PROG_STATUS or
 * use the tether_io catalog); requests are serialized.
 *
 * Usage: tplc-ctl-proxy [--socket <uds>] [--port N] [--bind <addr>]
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <arpa/inet.h>
#include <cerrno>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <string>
#include <vector>
#include <netdb.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <sys/un.h>
#include <unistd.h>

static const char* g_sock = "/tmp/tplc-image.ctl";

static int uds_connect() {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -1;
    sockaddr_un a{};
    a.sun_family = AF_UNIX;
    strncpy(a.sun_path, g_sock, sizeof a.sun_path - 1);
    timeval tv{5, 0};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof tv);
    if (connect(fd, (sockaddr*)&a, sizeof a) != 0) { close(fd); return -1; }
    return fd;
}

/* read exactly n bytes; false on eof/timeout */
static bool read_full(int fd, uint8_t* p, size_t n) {
    size_t got = 0;
    while (got < n) {
        ssize_t r = read(fd, p + got, n - got);
        if (r <= 0) return false;
        got += (size_t)r;
    }
    return true;
}

/* issue the session HELLO the ctl server gates on. Per-request UDS
 * connections mean each HTTP call gets a fresh session — stateless ops
 * and the deploy upload-id flow tolerate that. */
static bool ctl_hello(int fd) {
    /* TplcCtlHello {u16 proto_ver, u16 pad, u32 caps} + u16 name_len */
    uint8_t buf[24] = {1, 0, 0, 0, 0, 0, 0, 0, 14, 0};
    const char* name = "tplc-ctl-proxy";
    memcpy(buf + 10, name, 14);
    uint8_t hdr[16] = {0x01, 0x01, 0, 0, 24, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0};
    uint8_t frame[16 + sizeof buf];
    memcpy(frame, hdr, 16);
    memcpy(frame + 16, buf, sizeof buf);
    if (send(fd, frame, sizeof frame, MSG_NOSIGNAL) != (ssize_t)sizeof frame)
        return false;
    if (!read_full(fd, hdr, 16)) return false;
    uint32_t rlen = (uint32_t)hdr[4] | ((uint32_t)hdr[5] << 8) |
                    ((uint32_t)hdr[6] << 16) | ((uint32_t)hdr[7] << 24);
    std::vector<uint8_t> pl(rlen);
    if (rlen && !read_full(fd, pl.data(), rlen)) return false;
    return !(hdr[3] & 0x80);          /* F_ERROR bit15 → flags byte1 */
}

/* forward a request frame to the UDS, reply stored verbatim */
static bool ctl_roundtrip(const std::vector<uint8_t>& req,
                          std::vector<uint8_t>& rep) {
    if (req.size() < 16) return false;
    uint32_t plen = (uint32_t)req[4] | ((uint32_t)req[5] << 8) |
                    ((uint32_t)req[6] << 16) | ((uint32_t)req[7] << 24);
    if (req.size() != 16 + plen) return false;
    uint16_t type = (uint16_t)(req[0] | ((uint16_t)req[1] << 8));
    int fd = uds_connect();
    if (fd < 0) return false;
    bool ok = false;
    if ((type == 0x0101 /* HELLO */ || ctl_hello(fd)) &&
        send(fd, req.data(), req.size(), MSG_NOSIGNAL) ==
            (ssize_t)req.size()) {
        uint8_t hdr[16];
        if (read_full(fd, hdr, sizeof hdr)) {
            uint32_t rlen = (uint32_t)hdr[4] | ((uint32_t)hdr[5] << 8) |
                            ((uint32_t)hdr[6] << 16) | ((uint32_t)hdr[7] << 24);
            rep.assign(hdr, hdr + 16);
            rep.resize(16 + rlen);
            ok = rlen == 0 || read_full(fd, rep.data() + 16, rlen);
        }
    }
    close(fd);
    return ok;
}

static void http_reply(int fd, int code, const char* text,
                       const uint8_t* body, size_t blen,
                       const char* ctype) {
    char hdr[512];
    int n = snprintf(hdr, sizeof hdr,
        "HTTP/1.1 %d %s\r\nContent-Length: %zu\r\nContent-Type: %s\r\n"
        "Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: content-type\r\n"
        "Access-Control-Allow-Methods: GET,POST,OPTIONS\r\nConnection: close\r\n\r\n",
        code, text, blen, ctype);
    send(fd, hdr, (size_t)n, MSG_NOSIGNAL);
    if (blen) send(fd, body, blen, MSG_NOSIGNAL);
}

static void serve(int cfd) {
    timeval tv{10, 0};
    setsockopt(cfd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof tv);
    std::vector<uint8_t> buf(4096);
    size_t used = 0;
    size_t hdr_end = std::string::npos;
    while (hdr_end == std::string::npos && used < buf.size()) {
        ssize_t r = read(cfd, buf.data() + used, buf.size() - used);
        if (r <= 0) { close(cfd); return; }
        used += (size_t)r;
        std::string v((const char*)buf.data(), used);
        hdr_end = v.find("\r\n\r\n");
    }
    if (hdr_end == std::string::npos) { close(cfd); return; }

    std::string head((const char*)buf.data(), hdr_end);
    std::string method, path;
    {
        char m[16] = {0}, p[1024] = {0};
        sscanf(head.c_str(), "%15s %1023s", m, p);
        method = m; path = p;
    }
    size_t clen = 0;
    if (auto p = head.find("Content-Length:"); p != std::string::npos)
        clen = strtoul(head.c_str() + p + 15, nullptr, 10);
    if (clen > 16 * 1024 * 1024) {
        http_reply(cfd, 413, "Payload Too Large", nullptr, 0, "text/plain");
        close(cfd); return;
    }
    std::vector<uint8_t> body(clen);
    size_t have = used - (hdr_end + 4);
    size_t copy = have < clen ? have : clen;
    memcpy(body.data(), buf.data() + hdr_end + 4, copy);
    if (!read_full(cfd, body.data() + copy, clen - copy)) {
        close(cfd); return;
    }

    if (method == "OPTIONS") {
        http_reply(cfd, 204, "No Content", nullptr, 0, "text/plain");
    } else if (method == "GET" && path == "/health") {
        int u = uds_connect();
        if (u >= 0) { close(u); http_reply(cfd, 200, "OK", (const uint8_t*)"ok", 2, "text/plain"); }
        else http_reply(cfd, 503, "Service Unavailable", (const uint8_t*)"down", 4, "text/plain");
    } else if (method == "POST" && path == "/ctl") {
        std::vector<uint8_t> rep;
        if (ctl_roundtrip(body, rep))
            http_reply(cfd, 200, "OK", rep.data(), rep.size(), "application/octet-stream");
        else {
            const char* m = "ctl roundtrip failed";
            http_reply(cfd, 502, "Bad Gateway", (const uint8_t*)m, strlen(m), "text/plain");
        }
    } else {
        const char* m = "not found";
        http_reply(cfd, 404, "Not Found", (const uint8_t*)m, strlen(m), "text/plain");
    }
    close(cfd);
}

int main(int argc, char** argv) {
    int port = 8089;
    const char* bind_addr = "127.0.0.1";
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        auto next = [&]() -> const char* { return i + 1 < argc ? argv[++i] : ""; };
        if (a == "--socket") g_sock = next();
        else if (a == "--port") port = atoi(next());
        else if (a == "--bind") bind_addr = next();
        else {
            fprintf(stderr, "usage: %s [--socket uds] [--port N] [--bind addr]\n", argv[0]);
            return 2;
        }
    }
    int lfd = socket(AF_INET, SOCK_STREAM, 0);
    if (lfd < 0) { perror("socket"); return 1; }
    int one = 1;
    setsockopt(lfd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof one);
    sockaddr_in a{};
    a.sin_family = AF_INET;
    a.sin_port = htons((uint16_t)port);
    inet_pton(AF_INET, bind_addr, &a.sin_addr);
    if (bind(lfd, (sockaddr*)&a, sizeof a) != 0 || listen(lfd, 16) != 0) {
        perror("bind/listen"); return 1;
    }
    fprintf(stderr, "tplc-ctl-proxy %s:%d → %s\n", bind_addr, port, g_sock);
    for (;;) {
        int cfd = accept(lfd, nullptr, nullptr);
        if (cfd < 0) { if (errno == EINTR) continue; break; }
        serve(cfd);
    }
    close(lfd);
    return 0;
}
