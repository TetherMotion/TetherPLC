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
 *   GET /events         Server-Sent Events stream of ctl F_EVENT frames:
 *                         data: {"type":N,"flags":F,"req":R,"payload":"<b64>"}
 *                       Each connection forks a child holding its own UDS
 *                       session with SET_NOTIFY(all categories).
 *   GET /health         "ok" when the ctl socket connects, else 503
 *   OPTIONS             CORS preflight (Access-Control-Allow-Origin: *)
 *
 * Limitations: requests are serialized. /compile shells out to tplcc per
 * request — a language service should replace this.
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
#include <dirent.h>
#include <string>
#include <vector>
#include <netdb.h>
#include <signal.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/un.h>
#include <sys/wait.h>
#include <unistd.h>

static const char* g_sock = "/tmp/tplc-image.ctl";
static std::string g_tplcc = "tplcc";
static std::string g_tplccd = "/tmp/tplccd.sock";
static std::vector<std::string> g_libdirs;
static std::string g_blocks_cache;
static bool g_blocks_dirty = true;

static void http_reply(int fd, int code, const char* text,
                       const uint8_t* body, size_t blen,
                       const char* ctype);

static int uds_connect_to(const char* path) {
    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) return -1;
    sockaddr_un a{};
    a.sun_family = AF_UNIX;
    strncpy(a.sun_path, path, sizeof a.sun_path - 1);
    timeval tv{5, 0};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof tv);
    if (connect(fd, (sockaddr*)&a, sizeof a) != 0) { close(fd); return -1; }
    return fd;
}

static int uds_connect() {
    return uds_connect_to(g_sock);
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

/* POSIX single-quote escaping for embedding a path in a shell command */
static std::string shq(const std::string& s) {
    std::string o = "'";
    for (char c : s) {
        if (c == '\'') o += "'\\''";
        else o += c;
    }
    return o + "'";
}

/* collect *.st / *.tplc under dir (non-recursive) — no shell: the dir
 * comes from --libdir but filenames are untrusted directory content */
static void collect_lib_files(const std::string& dir,
                              std::vector<std::string>& out) {
    DIR* d = opendir(dir.c_str());
    if (!d) return;
    while (dirent* e = readdir(d)) {
        std::string n = e->d_name;
        if (n.size() > 3 &&
            (n.rfind(".st") == n.size() - 3 || n.rfind(".tplc") == n.size() - 5))
            out.push_back(dir + "/" + n);
    }
    closedir(d);
}

static std::string json_escape(const std::string& s) {
    std::string o;
    for (char c : s) {
        if (c == '"' || c == '\\') o += '\\';
        if (c == '\n') o += "\\n"; else if (c != '\r') o += c;
    }
    return o;
}

/* GET /blocks: run `tplcc blocks --json` over every --libdir's sources,
 * tag each block with its library (dir basename) and file, merge into
 * one catalog. Cached until ?refresh=1 — libraries are static at
 * runtime. */
static void handle_blocks(int cfd, bool refresh) {
    if (g_blocks_dirty || refresh || g_blocks_cache.empty()) {
        std::string merged = "{\"libs\":[";
        bool first_lib = true;
        for (auto& dir : g_libdirs) {
            std::vector<std::string> files;
            collect_lib_files(dir, files);
            if (files.empty()) continue;
            std::string cmd = shq(g_tplcc) + " blocks --json";
            for (auto& f : files) cmd += " " + shq(f);
            cmd += " 2>/dev/null";
            FILE* p = popen(cmd.c_str(), "r");
            std::string out;
            if (p) {
                char b[65536];
                while (size_t n = fread(b, 1, sizeof b, p)) out.append(b, n);
                pclose(p);
            }
            if (out.empty() || out.front() != '{') continue;
            /* basename of dir = library label; each lib's catalog is
             * emitted verbatim inside its wrapper */
            std::string lib = dir;
            while (lib.size() > 1 && lib.back() == '/') lib.pop_back();
            lib = lib.substr(lib.find_last_of('/') + 1);
            if (!first_lib) merged += ",";
            first_lib = false;
            merged += "{\"name\":\"" + json_escape(lib) + "\",\"dir\":\"" +
                      json_escape(dir) + "\",\"catalog\":" + out + "}";
        }
        merged += "]}";
        g_blocks_cache = merged;
        g_blocks_dirty = false;
    }
    http_reply(cfd, 200, "OK", (const uint8_t*)g_blocks_cache.data(),
               g_blocks_cache.size(), "application/json");
}

/* GET /libsrc?lib=<dir-index-or-name>&file=<basename>: serve one
 * library source file so the IDE can attach it to a project.
 * .st files are read directly; .tplc packages go through `tplcc src`. */
static void handle_libsrc(int cfd, const std::string& query) {
    auto qval = [&](const char* key) -> std::string {
        std::string k = std::string(key) + "=";
        size_t p = query.find(k);
        if (p == std::string::npos) return "";
        p += k.size();
        size_t e = query.find('&', p);
        std::string v = query.substr(p, e == std::string::npos ? e : e - p);
        /* url-decode the few chars we emit */
        std::string o;
        for (size_t i = 0; i < v.size(); i++) {
            if (v[i] == '%' && i + 2 < v.size()) {
                o += (char)strtoul(v.substr(i + 1, 2).c_str(), nullptr, 16);
                i += 2;
            } else if (v[i] == '+') o += ' ';
            else o += v[i];
        }
        return o;
    };
    std::string lib = qval("lib"), file = qval("file");
    /* file is interpolated into a shell command — restrict it to the
     * same basename charset /compile accepts (no quotes, no $(), no
     * whitespace), on top of the .. and / rejection above */
    bool safe_name = !file.empty();
    for (char c : file) {
        if (!((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
              (c >= '0' && c <= '9') || c == '.' || c == '_' || c == '-'))
            safe_name = false;
    }
    if (lib.empty() || file.empty() || file.find("..") != std::string::npos ||
        file.find('/') != std::string::npos || !safe_name) {
        const char* m = "bad libsrc query";
        http_reply(cfd, 400, "Bad Request", (const uint8_t*)m, strlen(m), "text/plain");
        return;
    }
    /* resolve lib → dir: index into g_libdirs or basename match */
    std::string dir;
    char* end = nullptr;
    long idx = strtol(lib.c_str(), &end, 10);
    if (end && *end == 0 && idx >= 0 && (size_t)idx < g_libdirs.size()) {
        dir = g_libdirs[idx];
    } else {
        for (auto& d : g_libdirs) {
            std::string b = d;
            while (b.size() > 1 && b.back() == '/') b.pop_back();
            if (b.substr(b.find_last_of('/') + 1) == lib) { dir = d; break; }
        }
    }
    if (dir.empty()) {
        const char* m = "unknown lib";
        http_reply(cfd, 404, "Not Found", (const uint8_t*)m, strlen(m), "text/plain");
        return;
    }
    /* file names a source: try <dir>/<file> directly (.st libraries),
     * then each .tplc package in the dir via `tplcc src <pkg> <name>`
     * (packages embed their ST sources). */
    std::string path = dir + "/" + file;
    std::vector<uint8_t> body;
    bool ok = read_file(path, body);
    if (!ok) {
        std::vector<std::string> pkgs;
        collect_lib_files(dir, pkgs);
        for (auto& p : pkgs) {
            if (p.size() < 5 || p.rfind(".tplc") != p.size() - 5) continue;
            std::string cmd = shq(g_tplcc) + " src " + shq(p) + " " + shq(file);
            FILE* fp = popen(cmd.c_str(), "r");
            if (!fp) continue;
            body.clear();
            char b[65536]; size_t n;
            while ((n = fread(b, 1, sizeof b, fp))) body.insert(body.end(), b, b + n);
            ok = pclose(fp) == 0 && !body.empty();
            if (ok) break;
        }
    }
    if (!ok || body.empty()) {
        const char* m = "not found";
        http_reply(cfd, 404, "Not Found", (const uint8_t*)m, strlen(m), "text/plain");
        return;
    }
    http_reply(cfd, 200, "OK", body.data(), body.size(), "text/plain; charset=utf-8");
}

/* POST /compile: write the src frame to a temp dir, run tplcc --json,
 * wrap its stdout JSON; build mode appends the .tplc as base64. */
static void handle_compile(int cfd, const std::vector<uint8_t>& body, bool build) {
    /* check requests prefer the persistent `tplcc serve` daemon — the
     * src frame forwards verbatim and its reply is already the right
     * envelope shape; on connect failure fall through to popen. */
    if (!build && !g_tplccd.empty()) {
        int u = uds_connect_to(g_tplccd.c_str());
        if (u >= 0) {
            send(u, body.data(), body.size(), MSG_NOSIGNAL);
            shutdown(u, SHUT_WR);
            std::string out;
            uint8_t b[8192];
            ssize_t n;
            while ((n = read(u, b, sizeof b)) > 0) out.append((const char*)b, n);
            close(u);
            if (!out.empty()) {
                http_reply(cfd, 200, "OK", (const uint8_t*)out.data(), out.size(),
                           "application/json");
                return;
            }
        }
    }
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
    std::string cmd = "cd " + shq(dir) + " && " + shq(g_tplcc) +
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
        cmd += " " + shq(name);
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
    /* SIGCHLD is SIG_IGN (auto-reap for the /events children), so
     * pclose() can report ECHILD (-1) even when the compiler ran fine.
     * Fall back to the compiler's own verdict, which is what the
     * client reads anyway. */
    bool ok = exit_code == 0;
    if (exit_code < 0 && !out.empty() &&
        out.find("\"ok\":true") != std::string::npos) {
        ok = true;
        exit_code = 0;
    }

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
               "\"code\":\"TPLC_E_DRIVER\",\"severity\":\"error\",\"message\":\"" +
               json_escape(msg) + "\"}]},\"exit\":-1";
    }
    if (build) {
        std::vector<uint8_t> pkg;
        if (ok && read_file(dir + "/out.tplc", pkg))
            resp += ",\"package\":\"" + b64_encode(pkg.data(), pkg.size()) + "\"";
    }
    resp += "}";
    http_reply(cfd, 200, "OK", (const uint8_t*)resp.data(), resp.size(), "application/json");
    for (auto& [name, _] : files) unlink((dir + "/" + name).c_str());
    unlink((dir + "/err.log").c_str());
    unlink((dir + "/out.tplc").c_str());
    rmdir(dir.c_str());
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

/* GET /events: fork a child that holds its own UDS session subscribed to
 * every notify category and pumps each ctl frame as an SSE `data:` line.
 * The parent returns immediately so other requests stay serialized. */
static void handle_events(int cfd) {
    pid_t pid = fork();
    if (pid != 0) { close(cfd); return; }
    /* the runtime may still be starting: retry the ctl socket briefly
     * instead of dying before the first event */
    int u = -1;
    for (int i = 0; i < 50 && u < 0; i++) {
        u = uds_connect();
        if (u < 0) usleep(100000);
    }
    if (u < 0 || !ctl_hello(u)) _exit(1);
    timeval ntv{0, 0};
    setsockopt(u, SOL_SOCKET, SO_RCVTIMEO, &ntv, sizeof ntv);
    /* SET_NOTIFY 0x0104 {u32 mask} — subscribe all categories */
    uint8_t sn[20] = {0x04, 0x01, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                      0xff, 0xff, 0xff, 0xff};
    if (send(u, sn, sizeof sn, MSG_NOSIGNAL) != (ssize_t)sizeof sn)
        _exit(1);
    static const char* shdr =
        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\n"
        "Cache-Control: no-cache\r\nAccess-Control-Allow-Origin: *\r\n"
        "Connection: close\r\n\r\n";
    send(cfd, shdr, strlen(shdr), MSG_NOSIGNAL);
    for (;;) {
        uint8_t fh[16];
        if (!read_full(u, fh, sizeof fh)) break;
        uint32_t plen = (uint32_t)fh[4] | ((uint32_t)fh[5] << 8) |
                        ((uint32_t)fh[6] << 16) | ((uint32_t)fh[7] << 24);
        if (plen > (1u << 20)) break;
        std::vector<uint8_t> pl(plen);
        if (plen && !read_full(u, pl.data(), plen)) break;
        uint16_t type = (uint16_t)(fh[0] | ((uint16_t)fh[1] << 8));
        uint16_t flags = (uint16_t)(fh[2] | ((uint16_t)fh[3] << 8));
        uint64_t req;
        memcpy(&req, fh + 8, 8);
        std::string line = "data: {\"type\":" + std::to_string(type) +
            ",\"flags\":" + std::to_string(flags) +
            ",\"req\":" + std::to_string((unsigned long long)req) +
            ",\"payload\":\"" + b64_encode(pl.data(), pl.size()) +
            "\"}\n\n";
        if (send(cfd, line.data(), line.size(), MSG_NOSIGNAL) !=
            (ssize_t)line.size()) break;
    }
    _exit(0);
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
    } else if (method == "GET" && path.rfind("/blocks", 0) == 0) {
        handle_blocks(cfd, path.find("refresh") != std::string::npos);
    } else if (method == "GET" && path.rfind("/libsrc", 0) == 0) {
        size_t q = path.find('?');
        handle_libsrc(cfd, q == std::string::npos ? "" : path.substr(q + 1));
    } else if (method == "GET" && path == "/events") {
        handle_events(cfd);             /* parent closes; child owns */
        return;
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
        else if (a == "--tplccd") g_tplccd = next();
        else if (a == "--libdir") g_libdirs.push_back(next());
        else {
            fprintf(stderr, "usage: %s [--socket uds] [--port N] [--bind addr] [--tplcc path] [--tplccd uds|off] [--libdir dir]...\n", argv[0]);
            return 2;
        }
    }
    signal(SIGCHLD, SIG_IGN);           /* auto-reap /events children */
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
