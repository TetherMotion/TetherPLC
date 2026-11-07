/* tplc_ctl_proxy — HTTP/1.1 pass-through to the tether-plc control
 * channel (UDS). Browsers cannot reach AF_UNIX sockets; this single-
 * threaded bridge exposes `POST /ctl` so the web IDE can issue ctl ops.
 *
 *   POST /ctl           body = one complete ctl request frame (16B hdr +
 *                       payload); response = reply frame, verbatim
 *   POST /compile?build body = [u16 count][u16 nameLen,name,u32 len,
 *                       content]*N (LE). Runs `tplcc check|build --json`
 *                       in a temp dir; response wraps tplcc's JSON:
 *                       {"compile":{…},"exit":N,"package":"<base64>"}
 *                       (package only in build mode, on success)
 *   GET /health         "ok" when the ctl socket connects, else 503
 *   OPTIONS             CORS preflight (Access-Control-Allow-Origin: *)
 *
 * Limitations: SET_NOTIFY events are not carried (poll PROG_STATUS or
 * use the tether_io catalog); requests are serialized. /compile shells
 * out to tplcc per request — a language service should replace this.
 *
 * Usage: tplc-ctl-proxy [--socket <uds>] [--port N] [--bind <addr>]
 *                       [--tplcc <path>]
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <arpa/inet.h>
#include <cerrno>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <netdb.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/un.h>
#include <sys/wait.h>
#include <unistd.h>

static const char* g_sock = "/tmp/tplc-image.ctl";
static std::string g_tplcc = "tplcc";

static void http_reply(int fd, int code, const char* text,
                       const uint8_t* body, size_t blen,
                       const char* ctype);

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

/* keep file names shell- and fs-safe: basename chars only */
static std::string sanitize_name(const std::string& s) {
    std::string out;
    for (char c : s) {
        if (c == '/' || c == '\\') { out.clear(); continue; }
        if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
            (c >= '0' && c <= '9') || c == '.' || c == '_' || c == '-')
            out += c;
    }
    if (out.empty() || out[0] == '.') out = "src" + out;
    return out;
}

static std::string b64_encode(const uint8_t* p, size_t n) {
    static const char* T =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    std::string o;
    o.reserve((n + 2) / 3 * 4);
    for (size_t i = 0; i < n; i += 3) {
        uint32_t v = (uint32_t)p[i] << 16;
        if (i + 1 < n) v |= (uint32_t)p[i + 1] << 8;
        if (i + 2 < n) v |= p[i + 2];
        o += T[v >> 18];
        o += T[(v >> 12) & 63];
        o += i + 1 < n ? T[(v >> 6) & 63] : '=';
        o += i + 2 < n ? T[v & 63] : '=';
    }
    return o;
}

static bool read_file(const std::string& path, std::vector<uint8_t>& out) {
    FILE* f = fopen(path.c_str(), "rb");
    if (!f) return false;
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    out.resize(n > 0 ? (size_t)n : 0);
    bool ok = n <= 0 || fread(out.data(), 1, (size_t)n, f) == (size_t)n;
    fclose(f);
    return ok;
}

/* parse the [u16 count][u16 nameLen,name,u32 len,content]* frame into
 * per-file (name,content) pairs. LE fields; names sanitized. */
static bool parse_srcs(const std::vector<uint8_t>& body,
                       std::vector<std::pair<std::string, std::vector<uint8_t>>>& files) {
    auto u16 = [&](size_t o) { return (unsigned)body[o] | ((unsigned)body[o + 1] << 8); };
    auto u32 = [&](size_t o) {
        return (uint32_t)body[o] | ((uint32_t)body[o + 1] << 8) |
               ((uint32_t)body[o + 2] << 16) | ((uint32_t)body[o + 3] << 24);
    };
    if (body.size() < 2) return false;
    unsigned count = u16(0);
    size_t o = 2;
    for (unsigned i = 0; i < count; i++) {
        if (o + 2 > body.size()) return false;
        unsigned nl = u16(o); o += 2;
        if (nl == 0 || nl > 255 || o + nl + 4 > body.size()) return false;
        std::string name = sanitize_name(
            std::string((const char*)body.data() + o, nl));
        o += nl;
        uint32_t len = u32(o); o += 4;
        if (len > 4 * 1024 * 1024 || o + len > body.size()) return false;
        files.emplace_back(name,
            std::vector<uint8_t>(body.begin() + (ptrdiff_t)o,
                                 body.begin() + (ptrdiff_t)(o + len)));
        o += len;
    }
    return o == body.size() && !files.empty();
}

/* POST /compile: write the src frame to a temp dir, run tplcc --json,
 * wrap its stdout JSON; build mode appends the .tplc as base64. */
static void handle_compile(int cfd, const std::vector<uint8_t>& body, bool build) {
    std::vector<std::pair<std::string, std::vector<uint8_t>>> files;
    if (!parse_srcs(body, files) || files.size() > 64) {
        const char* m = "bad src frame";
        http_reply(cfd, 400, "Bad Request", (const uint8_t*)m, strlen(m), "text/plain");
        return;
    }
    char dir_t[] = "/tmp/tplc_compile_XXXXXX";
    if (!mkdtemp(dir_t)) {
        const char* m = "mkdtemp failed";
        http_reply(cfd, 500, "Internal Server Error", (const uint8_t*)m, strlen(m), "text/plain");
        return;
    }
    std::string dir = dir_t;
    std::string cmd = std::string("cd ") + dir + " && " + g_tplcc +
                      (build ? " build --json -o out.tplc" : " check --json");
    for (auto& [name, content] : files) {
        std::string p = dir + "/" + name;
        FILE* f = fopen(p.c_str(), "wb");
        if (!f || fwrite(content.data(), 1, content.size(), f) != content.size()) {
            if (f) fclose(f);
            const char* m = "src write failed";
            http_reply(cfd, 500, "Internal Server Error", (const uint8_t*)m, strlen(m), "text/plain");
            return;
        }
        fclose(f);
        cmd += " " + name;
    }
    cmd += " 2>err.log";
    FILE* pipe = popen(cmd.c_str(), "r");
    std::string out;
    if (pipe) {
        char b[4096];
        while (size_t n = fread(b, 1, sizeof b, pipe)) out.append(b, n);
    }
    int rc = pipe ? pclose(pipe) : -1;
    int exit_code = (rc >= 0 && WIFEXITED(rc)) ? WEXITSTATUS(rc) : -1;

    std::string resp;
    if (!out.empty() && out.front() == '{') {
        resp = "{\"compile\":" + std::string(out.c_str()) +
               ",\"exit\":" + std::to_string(exit_code);
    } else {
        std::vector<uint8_t> elog;
        std::string msg = "tplcc invocation failed";
        if (read_file(dir + "/err.log", elog) && !elog.empty())
            msg = std::string((const char*)elog.data(), elog.size());
        resp = "{\"compile\":{\"ok\":false,\"errors\":1,\"diags\":[{"
               "\"code\":\"TPLC_E_DRIVER\",\"severity\":\"error\",\"message\":";
        resp += "\"";
        for (char c : msg) {
            if (c == '"' || c == '\\') resp += '\\';
            if (c == '\n') resp += "\\n"; else if (c != '\r') resp += c;
        }
        resp += "\"}],},\"exit\":-1";
    }
    if (build) {
        std::vector<uint8_t> pkg;
        if (rc == 0 && read_file(dir + "/out.tplc", pkg))
            resp += ",\"package\":\"" + b64_encode(pkg.data(), pkg.size()) + "\"";
    }
    resp += "}";
    http_reply(cfd, 200, "OK", (const uint8_t*)resp.data(), resp.size(), "application/json");
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
    } else if (method == "POST" && path.rfind("/compile", 0) == 0) {
        handle_compile(cfd, body, path.find("build") != std::string::npos);
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
        else if (a == "--tplcc") g_tplcc = next();
        else {
            fprintf(stderr, "usage: %s [--socket uds] [--port N] [--bind addr] [--tplcc path]\n", argv[0]);
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
