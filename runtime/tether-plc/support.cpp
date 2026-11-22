/* tether-plc support helpers — implementation (split from main.cpp). */
#include "tether_plc_support.hpp"

#include <tplc/tplc_abi.h>
#include <tplc/tplc_ctl.h>
#include <tplc/host/CtlServer.hpp>

#include <algorithm>
#include <cerrno>
#include <csignal>
#include <cctype>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <deque>
#include <functional>
#include <thread>
#include <sys/prctl.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <fcntl.h>
#include <cstdint>
#include <ctime>
#include <dirent.h>
#include <optional>
#include <filesystem>
#include <fstream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unistd.h>

volatile sig_atomic_t g_stop = 0;

uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

uint64_t realtime_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

/* flat manifest field extractor — manifests are compiler-generated
   flat JSON objects; a bounded substring pull is adequate here      */
std::string manifest_str(const std::string& j, const char* key) {
    std::string k = std::string("\"") + key + "\":\"";
    auto i = j.find(k);
    if (i == std::string::npos) return "";
    auto e = j.find('"', i + k.size());
    return e == std::string::npos ? "" : j.substr(i + k.size(),
                                                 e - i - k.size());
}

void usage(const char* a0) {
    fprintf(stderr,
        "usage: %s --image <shm_path> --module <so>\n"
        "          [--ctl <uds_path>|none] [--wait-ms N] [--poll-ns N]\n"
        "          [--retain DIR] [--retain-flush-ms N]\n"
        "          [--wcet-hard-ns N] [--hang-timeout-ms N]\n"
        "          [--fault-policy hold|restart|unload] [--restart-max N]\n"
        "          [--runbook DIR] [--instance NAME] [--packages DIR]\n"
        "          [--sig-policy open|require|require-trusted]\n"
        "          [--trust-dir DIR] [--no-sandbox]\n"
        "          [--update-auth open|strict]\n"
        "          [--update-auth-map role=policy[@trust-dir]]...\n"
        "          [--quarantine-cycles N] [--shadow-budget-ns N]\n"
        "          [--stopped] [--sim]\n"
        "       %s --supervise [--respawn always|on-fault|never]\n"
        "          [--restart-delay-ms N] -- <tether-plc args>\n",
        a0, a0);
}

enum ProgState : uint8_t { PS_STOPPED = 0, PS_RUNNING = 1, PS_FAULT = 2,
                           PS_EMPTY = 3 };

/* doc 15 §5: minimum ctl role per op. observer = read-only;
   engineer adds debug-state mutation that cannot move the program
   (watch sets); commissioner owns everything that can (deploy,
   lifecycle, suspend/step, writes, forces, breaks, trace arms,
   retain, shutdown). Unknown ops → commissioner (fail-safe).     */
int op_min_role(uint16_t t) {
    switch (t) {
    case TPLC_OP_PING:           case TPLC_OP_SET_NOTIFY:
    case TPLC_OP_PROG_STATUS:    case TPLC_OP_PROG_LIST:
    case TPLC_OP_PROG_LAYOUT:    case TPLC_OP_TASK_STATS:
    case TPLC_OP_DBG_RESOLVE:    case TPLC_OP_DBG_READ:
    case TPLC_OP_DBG_STATE:      case TPLC_OP_DBG_FORCE_LIST:
    case TPLC_OP_DBG_BREAK_LIST: case TPLC_OP_DBG_STATE_DIFF:
    case TPLC_OP_DBG_FIELDS:     case TPLC_OP_TRACE_STATUS:
    case TPLC_OP_TRACE_READ:     case TPLC_OP_DBG_WATCH_GET:
    case TPLC_OP_DBG_WATCH_LIST: case TPLC_OP_RETAIN_STATUS:
    case TPLC_OP_RETAIN_DUMP:    case TPLC_OP_IMAGE_STATS:
    case TPLC_OP_IMAGE_READ:
    case TPLC_OP_RT_STATUS:      case TPLC_OP_PKG_LIST:
    case TPLC_OP_PKG_INFO:       case TPLC_OP_MAP_GET:
    case TPLC_OP_JOURNAL_TAIL:   case TPLC_OP_SHADOW_STATUS:
    case TPLC_OP_SHADOW_COMPARE: case TPLC_OP_SHADOW_IMAGE:
    case TPLC_OP_OD_READ:
    case TPLC_OP_OD_LIST:
    case TPLC_OP_EVTLOG_LIST:    case TPLC_OP_EVTLOG_QUERY:
    case TPLC_OP_EVTLOG_STATS:   case TPLC_OP_EVTLOG_HISTORY:
    case TPLC_OP_HIST_LIST:
    case TPLC_OP_MQTT_STATUS:    case TPLC_OP_MQTT_JOURNAL:
        return tplc::TPLC_ROLE_OBSERVER;
    case TPLC_OP_DBG_WATCH_ADD:
    case TPLC_OP_DBG_WATCH_STREAM:
    case TPLC_OP_DBG_WATCH_REMOVE:
    /* eventlog confirm = HMI alarm acknowledge: engineer floor   */
    case TPLC_OP_EVTLOG_CONFIRM:
    case TPLC_OP_HIST_CONF:
    /* OD_WRITE is a generic write channel: each object declares its
       own floor, enforced in the handler (MGMT_AUTHZ)             */
    case TPLC_OP_OD_WRITE:
        return tplc::TPLC_ROLE_ENGINEER;
    default:
        return tplc::TPLC_ROLE_COMMISSIONER;
    }
}

int role_by_name(const std::string& role) {
    return role == "observer"     ? tplc::TPLC_ROLE_OBSERVER
         : role == "engineer"     ? tplc::TPLC_ROLE_ENGINEER
         : role == "commissioner" ? tplc::TPLC_ROLE_COMMISSIONER
         : tplc::TPLC_ROLE_NONE;
}

/* --update-auth-map key=policy[@trust-dir]: per-channel update
   authentication. key is a ctl role name (observer|engineer|
   commissioner) or a decimal peer uid; policy ∈
   open|require|require-trusted. With --update-auth strict every
   channel is floored at require-trusted (weaker entries are a
   startup error); @dir overrides the trust root for that channel.
   Resolution: uid entry > role entry > global update policy. */

/* --authz file: "<uid> <role>" per line ('#' comments). Entries take
   precedence over the defaults (peer uid == server euid or root →
   commissioner, else observer), so a file can also downgrade.      */
std::unordered_map<uint32_t, int>
load_authz(const std::string& path, bool* ok) {
    std::unordered_map<uint32_t, int> m;
    std::ifstream f(path);
    if (!f) { *ok = false; return m; }
    std::string line;
    while (std::getline(f, line)) {
        size_t h = line.find('#');
        if (h != std::string::npos) line.resize(h);
        std::istringstream is(line);
        uint32_t uid; std::string role;
        if (!(is >> uid >> role)) continue;
        int r = role_by_name(role);
        if (r == tplc::TPLC_ROLE_NONE) { *ok = false; return m; }
        m[uid] = r;
    }
    *ok = true;
    return m;
}

void uuid_hex(const uint8_t u[16], char out[33]) {
    static const char* H = "0123456789abcdef";
    for (int i = 0; i < 16; i++) {
        out[i*2] = H[u[i] >> 4]; out[i*2+1] = H[u[i] & 0xF];
    }
    out[32] = 0;
}
/* uuid-or-path payload: a 14-char path yields a 16-byte
   {u16 plen, path} payload that collides with the raw-uuid form.
   Prefer the path decode when it parses cleanly (plen + 2 == len
   and the tail is printable); a binary uuid essentially never
   does. */
bool uuid_or_path(const uint8_t* p, uint32_t len,
                         const std::string& dir, std::string& out) {
    if (len >= 2) {
        uint16_t plen;
        memcpy(&plen, p, 2);
        if ((uint32_t)plen + 2 == len && plen > 0) {
            bool printable = true;
            for (uint32_t i = 2; i < len; i++)
                if (p[i] < 0x20 || p[i] == 0x7f) {
                    printable = false; break;
                }
            if (printable) {
                out.assign((const char*)p + 2, plen);
                return true;
            }
        }
    }
    if (len == 16) {
        char hex[33]; uuid_hex(p, hex);
        out = dir + "/" + hex + ".tplc";
        return true;
    }
    return false;
}
/* "SHADOW[<i>].<sym>" debug/watch path → 1-based candidate index
   (bare "SHADOW." means the first) and the symbol remainder.
   Indices line up with SHADOW_LIST order.                        */
bool shadow_path(std::string_view p, uint32_t* idx,
                        std::string_view* sym) {
    if (p.size() < 8 || p.substr(0, 6) != "SHADOW") return false;
    size_t i = 6;
    uint32_t n = 1;
    if (i < p.size() && p[i] == '[') {
        i++;
        if (i >= p.size() || p[i] < '1' || p[i] > '9') return false;
        n = 0;
        while (i < p.size() && p[i] >= '0' && p[i] <= '9') {
            if (n > 1000) return false;
            n = n * 10 + (uint32_t)(p[i] - '0');
            i++;
        }
        if (i >= p.size() || p[i] != ']') return false;
        i++;
    }
    if (i >= p.size() || p[i] != '.') return false;
    *idx = n;
    *sym = p.substr(i + 1);
    return !sym->empty();
}

bool hex_uuid(const std::string& hex, uint8_t u[16]) {
    if (hex.size() != 32) return false;
    auto nib = [](char c) -> int {
        if (c >= '0' && c <= '9') return c - '0';
        if (c >= 'a' && c <= 'f') return c - 'a' + 10;
        if (c >= 'A' && c <= 'F') return c - 'A' + 10;
        return -1;
    };
    for (int i = 0; i < 16; i++) {
        int hi = nib(hex[i*2]), lo = nib(hex[i*2+1]);
        if (hi < 0 || lo < 0) return false;
        u[i] = (uint8_t)(hi * 16 + lo);
    }
    return true;
}

/* -------- minimal SHA-256 (FIPS 180-4) — deploy integrity --------- */
struct Sha256 {
    uint32_t h[8]; uint64_t bits = 0;
    uint8_t buf[64]; size_t buflen = 0;

    void init() {
        static const uint32_t IV[8] = {
            0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,
            0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19 };
        memcpy(h, IV, sizeof h); bits = 0; buflen = 0;
    }
    static uint32_t rr(uint32_t x, int n) {
        return (x >> n) | (x << (32 - n));
    }
    void block(const uint8_t* p) {
        static const uint32_t K[64] = {
            0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,
            0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
            0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,
            0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
            0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,
            0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
            0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,
            0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
            0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,
            0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
            0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,
            0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
            0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,
            0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
            0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,
            0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2 };
        uint32_t w[64];
        for (int i = 0; i < 16; i++)
            w[i] = ((uint32_t)p[i*4]<<24)|((uint32_t)p[i*4+1]<<16)|
                   ((uint32_t)p[i*4+2]<<8)|p[i*4+3];
        for (int i = 16; i < 64; i++) {
            uint32_t s0 = rr(w[i-15],7)^rr(w[i-15],18)^(w[i-15]>>3);
            uint32_t s1 = rr(w[i-2],17)^rr(w[i-2],19)^(w[i-2]>>10);
            w[i] = w[i-16]+s0+w[i-7]+s1;
        }
        uint32_t a=h[0],b=h[1],c=h[2],dd=h[3],e=h[4],f=h[5],
                 g=h[6],hh=h[7];
        for (int i = 0; i < 64; i++) {
            uint32_t S1 = rr(e,6)^rr(e,11)^rr(e,25);
            uint32_t ch = (e&f)^(~e&g);
            uint32_t t1 = hh+S1+ch+K[i]+w[i];
            uint32_t S0 = rr(a,2)^rr(a,13)^rr(a,22);
            uint32_t mj = (a&b)^(a&c)^(b&c);
            uint32_t t2 = S0+mj;
            hh=g; g=f; f=e; e=dd+t1; dd=c; c=b; b=a; a=t1+t2;
        }
        h[0]+=a; h[1]+=b; h[2]+=c; h[3]+=dd;
        h[4]+=e; h[5]+=f; h[6]+=g; h[7]+=hh;
    }
    void update(const uint8_t* p, size_t n) {
        bits += (uint64_t)n * 8;
        while (n) {
            size_t take = 64 - buflen;
            if (take > n) take = n;
            memcpy(buf + buflen, p, take);
            buflen += take; p += take; n -= take;
            if (buflen == 64) { block(buf); buflen = 0; }
        }
    }
    void final(uint8_t out[32]) {
        buf[buflen++] = 0x80;
        if (buflen > 56) {
            while (buflen < 64) buf[buflen++] = 0;
            block(buf); buflen = 0;
        }
        while (buflen < 56) buf[buflen++] = 0;
        for (int i = 7; i >= 0; i--) buf[buflen++] = (uint8_t)(bits >> (i*8));
        block(buf);
        for (int i = 0; i < 8; i++) {
            out[i*4]=(uint8_t)(h[i]>>24); out[i*4+1]=(uint8_t)(h[i]>>16);
            out[i*4+2]=(uint8_t)(h[i]>>8); out[i*4+3]=(uint8_t)h[i];
        }
    }
};
void sha256_file(int fd, uint8_t out[32]) {
    Sha256 s; s.init();
    uint8_t chunk[4096];
    lseek(fd, 0, SEEK_SET);
    ssize_t r;
    while ((r = read(fd, chunk, sizeof chunk)) > 0)
        s.update(chunk, (size_t)r);
    s.final(out);
}

/* -------- deploy upload slot (doc 04 §I.1 0x0200 group) -----------
   one outstanding upload; chunks must arrive contiguously           */

/* -------- runbook: per-instance recovery record (doc 04 §7) --------
   <dir>/<instance>.run — key=value lines, atomic tmp+rename          */
void runbook_write(const std::string& dir,
                          const std::string& instance,
                          const char* package, const uint8_t uuid[16],
                          const char* state) {
    if (dir.empty() || instance.empty()) return;
    ::mkdir(dir.c_str(), 0755);
    char hex[33];
    static const char* H = "0123456789abcdef";
    for (int i = 0; i < 16; i++) {
        hex[i*2] = H[uuid[i] >> 4]; hex[i*2+1] = H[uuid[i] & 0xF];
    }
    hex[32] = 0;
    std::string tmp = dir + "/" + instance + ".tmp";
    std::string dst = dir + "/" + instance + ".run";
    int fd = ::open(tmp.c_str(), O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd < 0) return;
    char buf[512];
    int n = snprintf(buf, sizeof buf,
                     "uuid=%s\npackage=%s\nstate=%s\n", hex,
                     package ? package : "", state);
    ssize_t w = ::write(fd, buf, (size_t)n);
    (void)w;
    ::close(fd);
    ::rename(tmp.c_str(), dst.c_str());
}

/* -------- supervisor mode (doc 04 §7 contract) ---------------------
   tether-plc --supervise --respawn P --restart-delay-ms D -- ARGS…
   forks+execs real tether-plc with ARGS, respawns per policy with
   backoff and a 3/60s rate cap. plc_hb staleness is the child's own
   monitor thread's job (P2-02); the supervisor watches liveness.    */
int supervise(char** argv, int sep) {
    const char* policy = "on-fault";
    uint64_t delay_ms = 1000;
    for (int i = 1; i < sep; i++) {
        std::string a = argv[i];
        auto next = [&]() -> const char* {
            return i + 1 < sep ? argv[++i] : "";
        };
        if (a == "--respawn") policy = next();
        else if (a == "--restart-delay-ms")
            delay_ms = strtoull(next(), 0, 0);
        else if (a == "--supervise") { /* self */ }
    }
    std::deque<uint64_t> restarts;
    int child_status = 0;
    for (;;) {
        pid_t pid = fork();
        if (pid == 0) {
            argv[sep] = (char*)"tether-plc";
            execvp(argv[0], &argv[sep]);
            _exit(127);
        }
        if (pid < 0) return 2;
        int st;
        while (waitpid(pid, &st, 0) < 0) { /* EINTR loop */ }
        child_status = WIFEXITED(st) ? WEXITSTATUS(st)
                     : WIFSIGNALED(st) ? 128 + WTERMSIG(st) : 1;
        if (g_stop) break;
        bool fault = child_status != 0;
        bool go = !strcmp(policy, "always") ||
                  (!strcmp(policy, "on-fault") && fault);
        if (!go || !strcmp(policy, "never")) break;
        /* rate cap: max 3 restarts per 60 s (doc 04 §7) */
        uint64_t now = mono_ns();
        while (!restarts.empty() &&
               now - restarts.front() > 60ull * 1000 * 1000 * 1000)
            restarts.pop_front();
        if (restarts.size() >= 3) {
            fprintf(stderr,
                    "tether-plc-supervise: restart rate cap hit\n");
            return 3;
        }
        restarts.push_back(now);
        fprintf(stderr,
                "tether-plc-supervise: child exit %d, respawn in %llums\n",
                child_status, (unsigned long long)delay_ms);
        struct timespec ts {
            (time_t)(delay_ms / 1000),
            (long)((delay_ms % 1000) * 1000000)
        };
        nanosleep(&ts, nullptr);
    }
    return child_status;
}
void DeployUpload::drop() {
    if (fd >= 0) { ::close(fd); ::unlink(tmp_path.c_str()); }
    *this = DeployUpload{};
}
