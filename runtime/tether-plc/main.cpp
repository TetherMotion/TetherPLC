/* tether-plc — the PLC runtime process (P0-07 slice of plan doc 04).
 * Attaches to the SHM process image (plc role), loads a program
 * module, schedules its tasks, publishes heartbeats/status, and
 * serves the control channel. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_abi.h>
#include <tplc/tplc_image.h>
#include <tplc/tplc_ctl.h>
#include <tplc/host/Module.hpp>
#include <tplc/pkg/Package.hpp>
#include <tplc/host/Program.hpp>
#include <tplc/host/Scheduler.hpp>
#include <tplc/host/CtlServer.hpp>
#include <tplc/host/RetainStore.hpp>
#include <tplc/host/Sandbox.hpp>
#include <tplc/host/AuditLog.hpp>

#include <algorithm>
#include <cerrno>
#include <atomic>
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
#include <cstring>
#include <ctime>
#include <dirent.h>
#include <optional>
#include <filesystem>
#include <fstream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unistd.h>

static volatile sig_atomic_t g_stop = 0;
static void on_sig(int) { g_stop = 1; }

static uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

static uint64_t realtime_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

/* flat manifest field extractor — manifests are compiler-generated
   flat JSON objects; a bounded substring pull is adequate here      */
static std::string manifest_str(const std::string& j, const char* key) {
    std::string k = std::string("\"") + key + "\":\"";
    auto i = j.find(k);
    if (i == std::string::npos) return "";
    auto e = j.find('"', i + k.size());
    return e == std::string::npos ? "" : j.substr(i + k.size(),
                                                 e - i - k.size());
}

static void usage(const char* a0) {
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
static int op_min_role(uint16_t t) {
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
    case TPLC_OP_RT_STATUS:      case TPLC_OP_PKG_LIST:
    case TPLC_OP_PKG_INFO:       case TPLC_OP_MAP_GET:
    case TPLC_OP_JOURNAL_TAIL:   case TPLC_OP_SHADOW_STATUS:
    case TPLC_OP_SHADOW_COMPARE: case TPLC_OP_OD_READ:
    case TPLC_OP_OD_LIST:
        return tplc::TPLC_ROLE_OBSERVER;
    case TPLC_OP_DBG_WATCH_ADD:
    case TPLC_OP_DBG_WATCH_STREAM:
    case TPLC_OP_DBG_WATCH_REMOVE:
    /* OD_WRITE is a generic write channel: each object declares its
       own floor, enforced in the handler (MGMT_AUTHZ)             */
    case TPLC_OP_OD_WRITE:
        return tplc::TPLC_ROLE_ENGINEER;
    default:
        return tplc::TPLC_ROLE_COMMISSIONER;
    }
}

static int role_by_name(const std::string& role) {
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
struct UpdateAuthRule {
    std::string policy;
    std::string trust_dir;
};

/* --authz file: "<uid> <role>" per line ('#' comments). Entries take
   precedence over the defaults (peer uid == server euid or root →
   commissioner, else observer), so a file can also downgrade.      */
static std::unordered_map<uint32_t, int>
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

static void uuid_hex(const uint8_t u[16], char out[33]) {
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
static bool uuid_or_path(const uint8_t* p, uint32_t len,
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
static bool hex_uuid(const std::string& hex, uint8_t u[16]) {
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
static void sha256_file(int fd, uint8_t out[32]) {
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
struct DeployUpload {
    bool        active = false;
    uint64_t    id = 0, size = 0, received = 0;
    uint8_t     sha[32]{};
    uint8_t     mode = 0;
    int         fd = -1;
    std::string tmp_path;
    void drop() {
        if (fd >= 0) { ::close(fd); ::unlink(tmp_path.c_str()); }
        *this = DeployUpload{};
    }
};

/* -------- runbook: per-instance recovery record (doc 04 §7) --------
   <dir>/<instance>.run — key=value lines, atomic tmp+rename          */
static void runbook_write(const std::string& dir,
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
static int supervise(char** argv, int sep) {
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

int main(int argc, char** argv) {
    /* default timer slack coalesces nanosleep/epoll wakes by up to
       ~1 ms; release timing needs precise wakes */
    prctl(PR_SET_TIMERSLACK, 1UL);
    std::string image_path, module_path, package_path, ctl_path;
    std::string retain_dir, runbook_dir, instance_name, packages_dir;
    std::string sig_policy = "open";               /* doc 15 §4.1 */
    std::string update_auth = "open";              /* plan 20: strict = updates need trusted sigs */
    std::vector<std::string> update_auth_specs;    /* --update-auth-map entries */
    std::string trust_dir  = "/etc/tether/trust";
    std::string authz_path, audit_path;            /* doc 15 §5   */
    uint64_t retain_flush_ms = 1000;
    uint64_t wcet_hard_ns = 0;
    int      quarantine_cycles = 10;
    bool     auto_rollback = false;
    bool     no_sandbox = false;
    uint64_t hang_timeout_ms = 2000;
    uint8_t  fault_policy = 0;
    uint32_t restart_max = 3;
    int wait_ms = 10000;
    uint64_t poll_ns = 1000 * 1000;          /* 1 ms max idle         */
    uint64_t shadow_budget_ns = 200000;      /* slack needed to tick  */
    bool     stopped_boot = false;
    uint32_t ctx_flags = 0;

    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "--"))
            return supervise(argv, i);
    }
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        auto next = [&]() -> std::string {
            return i + 1 < argc ? argv[++i] : "";
        };
        if (a == "--image") image_path = next();
        else if (a == "--module") module_path = next();
        else if (a == "--package") package_path = next();
        else if (a == "--ctl") ctl_path = next();
        else if (a == "--wait-ms") wait_ms = atoi(next().c_str());
        else if (a == "--poll-ns") poll_ns = strtoull(next().c_str(), 0, 0);
        else if (a == "--retain") retain_dir = next();
        else if (a == "--retain-flush-ms")
            retain_flush_ms = strtoull(next().c_str(), 0, 0);
        else if (a == "--wcet-hard-ns")
            wcet_hard_ns = strtoull(next().c_str(), 0, 0);
        else if (a == "--hang-timeout-ms")
            hang_timeout_ms = strtoull(next().c_str(), 0, 0);
        else if (a == "--fault-policy") {
            std::string p = next();
            fault_policy = p == "restart" ? 1 : p == "unload" ? 2 : 0;
        }
        else if (a == "--restart-max")
            restart_max = (uint32_t)atoi(next().c_str());
        else if (a == "--runbook") runbook_dir = next();
        else if (a == "--instance") instance_name = next();
        else if (a == "--packages") packages_dir = next();
        else if (a == "--quarantine-cycles")
            quarantine_cycles = atoi(next().c_str());
        else if (a == "--shadow-budget-ns")
            shadow_budget_ns = strtoull(next().c_str(), 0, 0);
        else if (a == "--auto-rollback") auto_rollback = true;
        else if (a == "--stopped") stopped_boot = true;
        else if (a == "--sig-policy") sig_policy = next();
        else if (a == "--trust-dir") trust_dir = next();
        else if (a == "--update-auth") update_auth = next();
        else if (a == "--update-auth-map")
            update_auth_specs.push_back(next());
        else if (a == "--no-sandbox") no_sandbox = true;
        else if (a == "--authz") authz_path = next();
        else if (a == "--audit-log") audit_path = next();
        else if (a == "--sim") ctx_flags |= TPLC_CTXF_SIM;
        else if (a == "--help" || a == "-h") { usage(argv[0]); return 0; }
        else { usage(argv[0]); return 2; }
    }
    if (image_path.empty() || (module_path.empty() && package_path.empty())) {
        usage(argv[0]);
        return 2;
    }
    if (packages_dir.empty()) packages_dir = "packages";
    if (ctl_path.empty()) ctl_path = image_path + ".ctl";
    const bool ctl_enabled = (ctl_path != "none");

    /* per-channel update authentication: key=policy[@trust-dir];
       strict floors every channel at require-trusted so a weaker
       entry is a startup error, never a silent bypass. */
    std::unordered_map<uint8_t, UpdateAuthRule> update_auth_map;
    std::unordered_map<uint32_t, UpdateAuthRule> update_auth_uid;
    for (const auto& spec : update_auth_specs) {
        auto bad = [&]() {
            fprintf(stderr,
                    "tether-plc: bad --update-auth-map '%s' "
                    "(want role-or-uid=policy[@trust-dir])\n",
                    spec.c_str());
        };
        auto eq = spec.find('=');
        if (eq == std::string::npos) { bad(); return 2; }
        std::string key = spec.substr(0, eq);
        std::string val = spec.substr(eq + 1);
        auto at = val.find('@');
        std::string pol = at == std::string::npos ? val : val.substr(0, at);
        std::string tdir = at == std::string::npos ? "" : val.substr(at + 1);
        if ((pol != "open" && pol != "require" &&
             pol != "require-trusted") ||
            (!tdir.empty() && tdir.front() != '/')) {
            bad(); return 2;
        }
        if (update_auth == "strict" && pol != "require-trusted") {
            fprintf(stderr,
                    "tether-plc: --update-auth strict cannot be "
                    "weakened per channel ('%s')\n", spec.c_str());
            return 2;
        }
        bool numeric = !key.empty() &&
            std::all_of(key.begin(), key.end(),
                        [](char c) { return std::isdigit((unsigned char)c); });
        if (numeric) {
            update_auth_uid[(uint32_t)strtoul(key.c_str(), nullptr, 10)] =
                {pol, tdir};
        } else {
            int role = role_by_name(key);
            if (role == tplc::TPLC_ROLE_NONE) { bad(); return 2; }
            update_auth_map[(uint8_t)role] = {pol, tdir};
        }
    }

    /* ---------------- attach image (plc role) ---------------- */
    TplcImage img;
    int rc = tplc_image_attach(&img, image_path.c_str(), wait_ms);
    if (rc != 0) {
        fprintf(stderr, "tether-plc: attach %s failed (%d)\n",
                image_path.c_str(), rc);
        return 1;
    }
    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_OPEN);

    /* process sandbox (doc 15 §3): default-on, --no-sandbox is the
       journaled escape. Install before any module bytes are touched;
       the whitelist keeps dlopen/ctl/retain paths working. */
    if (!no_sandbox) {
        tplc::Status sbst = tplc::installSandbox();
        if (!sbst.ok()) {
            fprintf(stderr, "tether-plc: sandbox failed (%d) %s\n",
                    sbst.code, sbst.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
    } else {
        fprintf(stderr, "tether-plc: WARNING --no-sandbox: process "
                        "containment disabled\n");
    }

    /* durable audit journal (doc 15 §5): append-only, fsync'd per
       record, survives restart. Default path: runbook dir → retain
       dir → cwd. An explicit --audit-log that fails is fatal — a
       silently absent audit trail is worse than no start.        */
    tplc::AuditLog audit_log;
    std::string al_path = audit_path;
    if (al_path.empty()) {
        const std::string& base = !runbook_dir.empty() ? runbook_dir
                                : !retain_dir.empty() ? retain_dir
                                : std::string(".");
        al_path = base + "/audit.log";
    }
    if (tplc::Status ast = audit_log.open(al_path); !ast.ok()) {
        if (!audit_path.empty()) {
            fprintf(stderr, "tether-plc: %s\n", ast.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        fprintf(stderr, "tether-plc: audit journal unavailable "
                        "(%s) — stderr only\n", ast.msg.c_str());
    }
    if (no_sandbox)
        audit_log.record((uint32_t)getpid(), (uint32_t)getuid(),
                         (uint32_t)getgid(), 0, "runtime",
                         "runtime.no_sandbox",
                         "containment disabled at boot", 0);

    /* ---------------- package → extracted module -------------- */
    std::string extracted_so;
    std::vector<uint8_t> boot_layd, boot_smap,
                        boot_iomap;             /* sections from --package */
    uint8_t  boot_hash[32] = {};
    char     boot_ver[32] = {};
    if (!package_path.empty()) {
        auto pres = tplc::Package::open(package_path);
        if (!pres.ok()) {
            fprintf(stderr, "tether-plc: package %s unreadable (%d) %s\n",
                    package_path.c_str(), pres.st.code, pres.st.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        tplc::Package pkg = std::move(pres.value);
        auto sigst = pkg.verifySignature(sig_policy, trust_dir);
        if (!sigst.ok()) {
            fprintf(stderr, "tether-plc: signature policy (%d) %s\n",
                    sigst.code, sigst.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        {
            tplc::PkgSection sig{};
            if (!pkg.section(TPLC_SEC_SIGNATURE, sig))
                fprintf(stderr,
                        "tether-plc: WARNING package is unsigned"
                        " (signature.policy=open)\n");
        }
        auto vst = pkg.validateForLoad(false);
        if (!vst.ok()) {
            fprintf(stderr, "tether-plc: package not loadable (%d) %s\n",
                    vst.code, vst.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        {
            auto rst = pkg.checkRequires(tplc_image_in_size(&img),
                                         tplc_image_out_size(&img));
            if (!rst.ok()) {
                fprintf(stderr, "tether-plc: requires unmet (%d) %s\n",
                        rst.code, rst.msg.c_str());
                tplc_image_close(&img);
                return 1;
            }
        }
        tplc::PkgSection obj{}, b_layd{}, b_smap{}, b_iomap{};
        /* v1: first target_obj (multi-target selection comes with
           manifest.targets[] parsing) */
        if (!pkg.section(TPLC_SEC_TARGET_OBJ, obj)) {
            fprintf(stderr, "tether-plc: no target_obj section\n");
            tplc_image_close(&img);
            return 1;
        }
        /* copy section bytes: PkgSection points into `pkg`, which dies
           at the end of this block */
        if (pkg.section(TPLC_SEC_LAYOUT_DESC, b_layd) && b_layd.data)
            boot_layd.assign(b_layd.data, b_layd.data + b_layd.size);
        if (pkg.section(TPLC_SEC_SYMMAP, b_smap) && b_smap.data)
            boot_smap.assign(b_smap.data, b_smap.data + b_smap.size);
        if (pkg.section(TPLC_SEC_DEPLOY_MAP, b_iomap) && b_iomap.data)
            boot_iomap.assign(b_iomap.data,
                              b_iomap.data + b_iomap.size);
        memcpy(boot_hash, pkg.header().content_hash, 32);
        std::string bv = manifest_str(pkg.manifestJson(), "version");
        snprintf(boot_ver, sizeof boot_ver, "%s", bv.c_str());
        char tpath[] = "/tmp/tplc_pkg_XXXXXX";
        int fd = mkstemp(tpath);
        if (fd < 0 || write(fd, obj.data, obj.size) != (ssize_t)obj.size) {
            fprintf(stderr, "tether-plc: extract target_obj failed\n");
            if (fd >= 0) close(fd);
            tplc_image_close(&img);
            return 1;
        }
        close(fd);
        extracted_so = tpath;
        module_path = extracted_so;
        fprintf(stderr, "tether-plc: package %s → %s\n",
                package_path.c_str(), extracted_so.c_str());
    }

    /* ---------------- scheduler + swappable program --------------
       ctl prepare/unload swap the loaded program: sched unregisters
       the old Program before it is destroyed (module dlclose order
       matters — Program holds Module*), then the new one loads
       stopped (READY) and the start op runs tplc_init.            */
    tplc::Scheduler sched;
    ProgState prog_state = PS_EMPTY;
    int32_t last_fault = 0;
    /* management lineage (doc 16 §3.3): package identity + who put it
       here; epoch bumps on every program-set change (prepare, swap,
       unload, rollback) so bridge catalogs can invalidate stale views */
    uint8_t  pkg_hash[32] = {};
    char     pkg_version[32] = {};
    uint64_t lin_deployed_at_ns = 0;
    uint32_t lin_deployed_by = 0;
    uint8_t  lin_predecessor[16] = {};
    uint32_t fault_task = 0;
    uint64_t fault_cycle = 0;
    uint64_t mgmt_epoch = 0;
    uint8_t  retain_restored = 0;
    uint64_t retain_last_flush_ns = 0;
    bool     prog_started = false;

    tplc::ProgramConfig pcfg;
    pcfg.ctx_flags = ctx_flags;
    pcfg.wcet_hard_ns = wcet_hard_ns;
    pcfg.fault_policy = fault_policy;
    pcfg.restart_max = restart_max;

    std::deque<uint64_t> fault_restarts;
    /* heap Module: Program::mod_ binds its address — unique_ptr keeps
       the object stable across quar/mod ownership moves (doc 08)   */
    std::unique_ptr<tplc::Module>    mod;
    std::optional<tplc::Program>     prog;
    std::optional<tplc::RetainStore> retain;
    TplcProgramDescriptor            d{};
    std::vector<uint8_t>             pkg_layd;   /* for OP_PROG_LAYOUT */
    std::vector<uint8_t>             pkg_smap;   /* for OP_DBG_RESOLVE   */
    std::vector<uint8_t>             pkg_iomap;  /* for safety-boundary  */
    /* what the last committed swap's migrate rules did: MIGD bytes
       (rule table + embedded predecessor LAYD), the frozen pre-swap
       blob snapshot, and the old symmap for name lookups           */
    struct StateDiff {
        bool                 ready = false;
        bool                 same_layout = false;
        uint32_t             mig_status = 0;
        std::vector<uint8_t> migd;
        std::vector<uint8_t> old_blob;
        std::vector<uint8_t> old_smap;
    } sdiff;
    /* force table (doc 12 §6): blob-offset overrides re-stamped at
       every task boundary so program writes cannot undo them.
       mask[i]&bit set → val bit wins; a full-0xFF mask is the
       plain overwrite case                                         */
    struct Force {
        uint32_t              handle;
        uint64_t              off;
        std::vector<uint8_t>  val;
        std::vector<uint8_t>  mask;
    };
    std::vector<Force>               forces;
    uint32_t                         next_force_h = 1;
    static const size_t              MAX_FORCES = 64;
    /* trigger expressions (doc 12 §M): a bounded stack machine over
       resolved symmap paths; every path is a slot sampled once per
       scan so changed()/rising()/falling() see a stable previous   */
    enum ExprOp : uint8_t {
        XO_LIT = 0x01,   /* u16 lit idx → push lit                  */
        XO_SLOT = 0x02,  /* u8 slot idx → push sampled field value  */
        XO_NEG = 0x03, XO_NOT = 0x04,
        XO_ADD = 0x05, XO_SUB = 0x06, XO_MUL = 0x07,
        XO_DIV = 0x08, XO_MOD = 0x09,
        XO_EQ = 0x0A, XO_NE = 0x0B, XO_LT = 0x0C,
        XO_LE = 0x0D, XO_GT = 0x0E, XO_GE = 0x0F,
        XO_AND = 0x10, XO_OR = 0x11,
        XO_CHANGED = 0x12, XO_RISING = 0x13, XO_FALLING = 0x14,
    };
    struct Expr {
        struct Slot { uint64_t off; uint32_t len; };
        std::vector<Slot>    slots;          /* blob ranges sampled */
        std::vector<int64_t> prev;           /* last scan's values  */
        std::vector<int64_t> lits;
        std::vector<uint8_t> code;
    };
    /* break_when (doc 12 §5): conditional suspend, evaluated at the
       post-tick boundary — fires only on a fresh suspension        */
    struct Break {
        uint32_t handle;
        uint32_t task_idx;
        uint64_t off;
        uint32_t len;      /* 1/2/4/8  */
        uint8_t  cmp;      /* 0== 1!= 2< 3<= 4> 5>= */
        uint64_t val;
        uint64_t hits = 0; /* triggers fired      */
        bool     capture = false; /* freeze recorder, not suspend */
        bool     stale = false;   /* unresolved after a swap       */
        std::optional<Expr> x;    /* set → expression trigger        */
        std::string src;          /* expr text for re-resolution   */
    };
    std::vector<Break>               breaks;
    uint32_t                         next_break_h = 1;
    static const size_t              MAX_BREAKS = 32;
    /* flight recorder (doc 12 §7): fixed-capacity ring of per-scan
       samples — one frame per scan that ran ≥1 task, seq-numbered
       so ring wrap shows as a gap rather than silent loss         */
    struct TraceCh { uint64_t off; uint32_t len; };
    struct TraceFrame { uint64_t seq; uint64_t ts; uint8_t data[64]; };
    std::vector<TraceCh>             trace_ch;
    std::vector<TraceFrame>          trace_ring;
    uint64_t                         trace_wr = 0;  /* frames written */
    uint32_t                         trace_flags = 0;
    bool                             trace_frozen = false;
    static const size_t              TRACE_CAP = 4096;
    static const size_t              MAX_TRACE_CH = 8;
    static const uint32_t            TRACE_NO_FAULT_FREEZE = 1;
    /* flight recorder is ON BY DEFAULT (doc 12 §6): at each program
       start, if nothing is armed, record the program's I/O-bound
       blob fields so a fault always leaves pre-fault history. A
       user TRACE_CONF replaces the defaults; TRACE_CLEAR disarms
       until the next start transition.                          */
    auto arm_default_trace = [&] {
        if (!trace_ch.empty() || !prog) return;
        const TplcProgramDescriptor& dd = prog->desc();
        size_t frame = 0;
        for (uint32_t i = 0; i < dd.io_binding_count &&
                trace_ch.size() < MAX_TRACE_CH; i++) {
            const TplcIoBinding& b = dd.io_bindings[i];
            if (b.region == TPLC_REGION_MARKER) continue;
            uint32_t bl = b.size ? b.size : 1;
            if (bl > 8) bl = 8;
            if (frame + bl > sizeof(TraceFrame::data)) break;
            if (b.blob_offset + bl > prog->blobSize()) continue;
            trace_ch.push_back({(uint64_t)b.blob_offset, bl});
            frame += bl;
        }
    };
    /* watch (doc 12 §3): named blob-range read sets; stream arms a
       per-watch push of EV_DBG_WATCH at ≤ rate_hz, sampled at task
       boundaries like the recorder                                 */
    /* a watch item may carry the symbol path it was armed with so a
       swap can re-resolve it through the new symmap; offset-only
       items and vanished paths go stale (len=0 rows, listed)      */
    struct WatchItem {
        uint64_t    off;
        uint32_t    len;
        std::string path;
        bool        stale = false;
        bool        shadow = false;   /* path resolved via shadow->smap */
    };
    struct Watch {
        uint32_t              handle;
        std::vector<WatchItem> chs;
        uint32_t              rate_hz = 0;
        uint64_t              next_emit = 0;
        uint64_t              seq = 0;
    };
    std::vector<Watch>               watches;
    /* assigned once the expr machinery exists; re-resolves named
       watch items and expression triggers through the live symmap,
       marking unresolved entries stale. Called on swap commit and
       rollback.                                                   */
    std::function<void()>            reresolve_subs;
    uint32_t                         next_watch_h = 1;
    static const size_t              MAX_WATCHES = 256;
    static const size_t              MAX_WATCH_ITEMS = 16;
    static const size_t              MAX_WATCH_LEN = 512;
    static const size_t              MAX_WATCH_STREAMS = 8;
    /* online-change quarantine (doc 08 §7): the swapped-out program
       + module stay live for quarantine_cycles scans so rollback is
       a pointer swap back to untouched pre-swap state              */
    struct Quarantine {
        std::unique_ptr<tplc::Module> mod;
        std::optional<tplc::Program> prog;
        std::vector<uint8_t>         layd;
        std::vector<uint8_t>         smap;
        uint8_t                      hash[32] = {};
        char                         ver[32] = {};
        int                          cycles_left = 0;
    } quar;

    /* shadow run (doc 19): a candidate program on the same latched
       %I stream. Its %Q stays inside its own image_out buffer —
       structurally unreachable by the image. Runs after the active
       commit on a slack budget; promotion is a pointer swap.      */
    struct ShadowDiff {
        uint64_t cycle;
        uint64_t img_off;
        uint32_t len;
        uint8_t  active_bytes[32];
        uint8_t  shadow_bytes[32];
    };
    struct Shadow {
        std::unique_ptr<tplc::Module> mod;
        std::optional<tplc::Program>  prog;
        tplc::Scheduler               sched;
        std::vector<uint8_t>          layd;
        std::vector<uint8_t>          smap;
        std::vector<uint8_t>          imap;
        uint8_t  pkg_hash[32] = {};
        char     pkg_version[32] = {};
        /* migrate outcome from SHADOW_LOAD (for STATE_DIFF after
           promotion and SHADOW_RESET reporting)                   */
        bool     mig_ready = false;
        bool     same_layout = false;
        uint32_t mig_status = 0;
        std::vector<uint8_t> migd;
        uint64_t cycles = 0, missed = 0;
        uint64_t diff_count = 0, last_diff_cycle = 0;
        std::vector<ShadowDiff> diffs;
        uint64_t diff_wr = 0;
    };
    static const size_t SHADOW_DIFF_CAP = 256;
    std::optional<Shadow> shadow;
    bool     shadow_faulted = false;      /* set by shooks.fault  */
    uint64_t shadow_last_now = UINT64_MAX; /* one shadow cycle per
                                             scan instant — the loop
                                             spins between releases */

    auto drop_shadow = [&](const char* why) {
        if (!shadow) return;
        shadow.reset();
        shadow_faulted = false;
        audit_log.record((uint32_t)getpid(), (uint32_t)geteuid(), 0,
                         0, "-", "shadow.drop", why, 0);
    };

    /* task-control preservation across program adoption: fresh
       addProgram entries come up running — a suspended task must
       not silently resume across swap/promote/rollback          */
    std::vector<uint32_t> adopt_suspended;
    auto adopt_begin = [&] {
        adopt_suspended.clear();
        if (prog)
            for (uint32_t i = 0; i < d.task_count; i++)
                if (sched.suspended(*prog, i))
                    adopt_suspended.push_back(i);
    };
    auto adopt_end = [&] {
        for (uint32_t i : adopt_suspended) sched.suspend(*prog, i);
        adopt_suspended.clear();
    };

    auto unload_program = [&] {
        /* monitor thread skips non-RUNNING states, so clearing state
           first makes destruction race-free */
        prog_state = PS_EMPTY;
        if (prog) { sched.removeProgram(*prog); prog.reset(); }
        mod.reset();
        drop_shadow("unload");
        if (retain) retain->close();
        quar = Quarantine{};
        pkg_layd.clear();
        pkg_smap.clear();
        pkg_iomap.clear();
        sdiff = StateDiff{};
        forces.clear();
        next_force_h = 1;
        breaks.clear();
        next_break_h = 1;
        trace_ch.clear();
        trace_ring.clear();
        trace_wr = 0;
        trace_flags = 0;
        trace_frozen = false;
        watches.clear();
        next_watch_h = 1;
        d = TplcProgramDescriptor{};
        prog_started = false;
        memset(pkg_hash, 0, 32); pkg_version[0] = 0;
        memset(lin_predecessor, 0, 16);
        retain_restored = 0;
    };

    auto load_program = [&](const std::string& mpath) -> tplc::Status {
        unload_program();
        auto mres = tplc::Module::open(mpath, /*host_caps*/ 0);
        if (!mres.ok()) return mres.st;
        mod = std::make_unique<tplc::Module>(std::move(mres.value));
        const TplcProgramDescriptor& nd = mod->descriptor();
        if (nd.image_in_bytes > tplc_image_in_size(&img) ||
            nd.image_out_bytes > tplc_image_out_size(&img))
            return tplc::Status::Err(TPLC_E_IMG_TOO_SMALL,
                                     "image too small");
        auto pres = tplc::Program::create(*mod, pcfg);
        if (!pres.ok()) { unload_program(); return pres.st; }
        prog.emplace(std::move(pres.value));
        d = mod->descriptor();
        /* retain store (doc 04 §5): mirror retain_region to
           <dir>/<uuid>.bin; uuid+layout-hash match restores bytes */
        if (!retain_dir.empty()) {
            if (!retain) retain.emplace();
            uint64_t lh;
            memcpy(&lh, d.layout_hash, 8);
            tplc::Status rst = retain->open(retain_dir, d.program_uuid,
                                            lh, prog->retainSize());
            if (!rst.ok()) { unload_program(); return rst; }
            if (retain->load(prog->retainRegion())) {
                prog->setRetainRestored();
                retain_restored = 1;
                fprintf(stderr, "tether-plc: retain restored (%s)\n",
                        retain->path().c_str());
            }
        }
        sched.addProgram(*prog);
        prog_state = PS_STOPPED;
        last_fault = 0;
        fault_restarts.clear();
        return tplc::Status::Ok();
    };

    /* rollback (doc 08 §7): quarantined pre-swap program+module swap
       back in — its blob is untouched since the snapshot, so the
       restore is a pure pointer swap (retain file is re-opened under
       the old identity but not re-loaded: the blob already holds it) */
    /* a swap/promote/rollback rebuilds scheduler entries, so the
       debug-plane hold state must travel with the program: a task
       that was suspended (or mid single-step) must not start
       free-running under its replacement                      */
    auto snapshot_suspension = [&](const tplc::Program& p) {
        std::vector<bool> v(p.desc().task_count);
        for (uint32_t i = 0; i < v.size(); i++)
            v[i] = sched.suspended(p, i);
        return v;
    };
    auto apply_suspension = [&](tplc::Program& p,
                                const std::vector<bool>& v) {
        for (uint32_t i = 0; i < p.desc().task_count && i < v.size(); i++)
            if (v[i]) sched.suspend(p, i);
    };

    auto do_rollback = [&]() -> bool {
        if (!quar.prog) return false;
        /* same storage-address rule as commit: the scheduler keeps
           Program* — move objects between homes explicitly, then
           register the final address (heap Modules never move)     */
        adopt_begin();
        std::vector<bool> susp = snapshot_suspension(*prog);
        sched.removeProgram(*prog);
        tplc::Program oldp = std::move(*quar.prog);
        quar.prog.reset();
        tplc::Program newp = std::move(*prog);
        prog.reset();
        quar.prog.emplace(std::move(newp));
        prog.emplace(std::move(oldp));
        std::swap(mod, quar.mod);
        std::swap(pkg_layd, quar.layd);
        std::swap(pkg_smap, quar.smap);
        drop_shadow("rollback");  /* candidate is stale post-rollback */
        forces.clear();   /* blob identity changed — forces stale */
        trace_ch.clear();
        trace_ring.clear();
        trace_wr = 0;
        trace_flags = 0;
        trace_frozen = false;
        if (reresolve_subs) reresolve_subs();
        sched.addProgram(*prog);
        apply_suspension(*prog, susp);
        adopt_end();
        d = mod->descriptor();
        /* lineage restore (doc 16 §3.3): the rolled-back program
           reclaims its package identity; the failed program becomes
           the predecessor                                            */
        std::swap(pkg_hash, quar.hash);
        std::swap(pkg_version, quar.ver);
        memcpy(lin_predecessor, quar.prog->desc().program_uuid, 16);
        mgmt_epoch++;
        if (retain) {
            retain->close();
            if (!retain_dir.empty()) {
                uint64_t lh;
                memcpy(&lh, d.layout_hash, 8);
                retain->open(retain_dir, d.program_uuid, lh,
                             prog->retainSize());
            }
        }
        quar = Quarantine{};
        prog_state = PS_RUNNING;
        prog_started = true;
        arm_default_trace();
        return true;
    };

    /* force stamping (doc 12 §6): applied at every task boundary.
       where: 0 → the blob itself; 1 → translate into img_in_
       (post-latch, pre-copyInputsIn); 2 → img_out_ (post-copy,
       pre-commit). I/O-bound ranges translate through io_bindings —
       BOOL forces become bit ops on the image byte.              */
    auto stamp_forces = [&](int where) {
        if (!prog || forces.empty()) return;
        const TplcProgramDescriptor& dd = prog->desc();
        if (where == 0) {
            for (auto& f : forces) {
                uint8_t* dst = prog->blob() + f.off;
                for (size_t i = 0; i < f.val.size(); i++)
                    dst[i] = (uint8_t)((dst[i] & ~f.mask[i]) |
                                       (f.val[i] & f.mask[i]));
            }
            return;
        }
        uint8_t  reg   = where == 1 ? TPLC_REGION_IMAGE_IN
                                    : TPLC_REGION_IMAGE_OUT;
        uint8_t* img_p = where == 1 ? prog->imageIn() : prog->imageOut();
        for (auto& f : forces)
            for (uint32_t i = 0; i < dd.io_binding_count; i++) {
                const TplcIoBinding& b = dd.io_bindings[i];
                if (b.region != reg) continue;
                uint32_t bs = b.size ? b.size : 1;
                if (f.off < b.blob_offset ||
                    f.off + f.val.size() >
                        (uint64_t)b.blob_offset + bs)
                    continue;
                if (b.type_code == TPLC_IOT_BOOL) {
                    if (f.val.size() != 1) break;
                    if (!(f.mask[0] & 1)) break;
                    uint8_t bit = (uint8_t)(1u << (b.unit & 7u));
                    if (f.val[0] & 1) img_p[b.image_offset] |= bit;
                    else              img_p[b.image_offset] &=
                                          (uint8_t)~bit;
                } else {
                    uint8_t* dst = img_p + b.image_offset +
                                   (f.off - b.blob_offset);
                    for (size_t i = 0; i < f.val.size(); i++)
                        dst[i] = (uint8_t)((dst[i] & ~f.mask[i]) |
                                           (f.val[i] & f.mask[i]));
                }
                break;
            }
    };

    tplc::Status lst = load_program(module_path);
    if (!lst.ok()) {
        fprintf(stderr, "tether-plc: program load failed (%d) %s\n",
                lst.code, lst.msg.c_str());
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
        tplc_image_set_fault(&img, (uint64_t)-lst.code);
        tplc_image_close(&img);
        return 1;
    }
    /* descriptors captured at --package extraction outlive the load's
       internal unload (which clears pkg_* for swap-time use) */
    pkg_layd = std::move(boot_layd);
    pkg_smap = std::move(boot_smap);
    pkg_iomap = std::move(boot_iomap);
    memcpy(pkg_hash, boot_hash, 32);
    snprintf(pkg_version, sizeof pkg_version, "%s", boot_ver);
    lin_deployed_at_ns = realtime_ns();
    lin_deployed_by = (uint32_t)getuid();   /* boot deploy = self */
    memset(lin_predecessor, 0, 16);
    mgmt_epoch++;
    /* --stopped: program stays loaded + registered but unstarted until
       a ctl 'start'. Lets a stepper (sim --scan, commissioning UI)
       suspend tasks before the first free-run can fire — without it
       the first_run race makes startup-state vectors nondeterministic */
    tplc::Status st = tplc::Status::Ok();
    if (!stopped_boot) {
        st = prog->start();
        if (!st.ok()) {
            fprintf(stderr, "tether-plc: init failed (%d) %s\n",
                    st.code, st.msg.c_str());
            tplc_image_set_fault(&img, (uint64_t)-st.code);
            tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
            tplc_image_close(&img);
            return 1;
        }
        prog_started = true;
        prog_state = PS_RUNNING;
        arm_default_trace();
    }

    tplc::TickHooks hooks;
    hooks.latch = [&](tplc::Program& p, uint32_t) -> int32_t {
        int32_t rc = tplc_image_latch_inputs(&img, p.imageIn(), 0,
                                             p.desc().image_in_bytes);
        if (rc == 0) stamp_forces(1);  /* forced inputs reach task */
        return rc;
    };
    hooks.commit = [&](tplc::Program& p, uint32_t) -> int32_t {
        stamp_forces(2);               /* forced outs reach field  */
        return tplc_image_commit_outputs(&img, p.imageOut(), 0,
                                         p.desc().image_out_bytes);
    };
    /* shadow hooks (doc 19): identical inputs by construction — the
       latch copies the active program's already-latched %I buffer,
       never re-reads the image. The commit is a no-op: img_out_ is
       a private per-program buffer, i.e. the shadow output region —
       nothing in this path can steer the real image.             */
    tplc::TickHooks shooks;
    shooks.latch = [&](tplc::Program& p, uint32_t) -> int32_t {
        if (!prog) return tplc::TPLC_E_HOST_STATE;
        uint64_t n = std::min((uint64_t)p.desc().image_in_bytes,
                              (uint64_t)prog->desc().image_in_bytes);
        memcpy(p.imageIn(), prog->imageIn(), n);
        return 0;
    };
    shooks.commit = [](tplc::Program&, uint32_t) -> int32_t {
        return 0;
    };
    shooks.fault = [&](tplc::Program&, uint32_t ti, int32_t rc) {
        /* cannot destroy the Shadow inside sched.tick — flag it and
           let the scan loop drop it after the tick returns        */
        shadow_faulted = true;
        fprintf(stderr,
                "tether-plc: shadow task %u fault rc=%d "
                "(active unaffected)\n", ti, rc);
    };
    tplc::CtlServer ctl;
    /* doc 16 §4: every program-set transition emits a state event a
       subscriber can drive a §2 state machine from — uuid + from/to +
       reason + epoch, no polling needed                             */
    auto emit_transition = [&](uint8_t from, uint8_t to,
                               uint8_t reason) {
        if (!ctl_enabled) return;
        TplcCtlEvTransition ev{};
        memcpy(ev.program_uuid, d.program_uuid, 16);
        ev.from_state = from; ev.to_state = to; ev.reason = reason;
        ev.ts_ns = realtime_ns(); ev.epoch = mgmt_epoch;
        ctl.event(TPLC_EV_PROG_TRANSITION,
                  1u << (TPLC_EV_PROG_TRANSITION & 0xFF),
                  &ev, sizeof ev);
    };
    hooks.fault = [&](tplc::Program& p, uint32_t ftask, int32_t frc) {
            /* doc 02 §4.6 / doc 04 §6: task fault → program policy.
           hold = stay faulted; restart = warm reset bounded by
           restart_max/60s then hold; unload = stop+deschedule.      */
        last_fault = frc;
        fault_task = ftask;
        fault_cycle = p.ctx().cycle_no;
        /* flight recorder (doc 12 §6): faults freeze the ring by
           default — the last pre-fault scans are the post-mortem  */
        if (!trace_ch.empty() &&
            !(trace_flags & TRACE_NO_FAULT_FREEZE))
            trace_frozen = true;
        tplc_image_set_fault(&img, (uint64_t)-frc);
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
        /* doc 08 §7: a fault inside the quarantine window restores
           the prior program — alarm loud, never silently healthy   */
        if (auto_rollback && quar.prog) {
            fprintf(stderr,
                    "tether-plc: new program faulted rc=%d within "
                    "quarantine — auto-rollback\n", frc);
            do_rollback();
            emit_transition((uint8_t)PS_RUNNING, (uint8_t)PS_RUNNING,
                            2);
            tplc_image_set_plc_status(&img, TPLC_IMG_PLC_RUN);
            runbook_write(runbook_dir, instance_name,
                          package_path.empty() ? module_path.c_str()
                                               : package_path.c_str(),
                          d.program_uuid, "error");
            return;
        }
        if (ctl_enabled)
            ctl.event(TPLC_EV_FAULT, 1u << (TPLC_EV_FAULT & 0xFF),
                  &frc, sizeof frc);
        if (fault_policy == 1) {
            uint64_t now = mono_ns();
            while (!fault_restarts.empty() &&
                   now - fault_restarts.front() >
                       pcfg.restart_window_ns)
                fault_restarts.pop_front();
            if (fault_restarts.size() < pcfg.restart_max) {
                fault_restarts.push_back(now);
                tplc::Status rs = p.reset(/*warm=*/true);
                if (rs.ok()) {
                    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_RUN);
                    fprintf(stderr,
                            "tether-plc: fault rc=%d — warm restart\n",
                            frc);
                    return;
                }
                fprintf(stderr,
                        "tether-plc: restart failed %s\n",
                        rs.msg.c_str());
            }
            fprintf(stderr,
                    "tether-plc: fault rc=%d — restart budget out\n",
                    frc);
        } else if (fault_policy == 2) {
            fprintf(stderr,
                    "tether-plc: fault rc=%d — safe unload\n", frc);
            p.stop();
        } else {
            fprintf(stderr, "tether-plc: program fault rc=%d\n", frc);
        }
        prog_state = PS_FAULT;
        emit_transition((uint8_t)PS_RUNNING, (uint8_t)PS_FAULT, 1);
        runbook_write(runbook_dir, instance_name,
                      package_path.empty() ? module_path.c_str()
                                           : package_path.c_str(),
                      d.program_uuid, "error");
    };

    /* audit trail (doc 12 §8 / doc 15 §5): every mutating op logs
       who(session+name+kernel uid/gid/pid), what, the operand hash
       and the result — stderr (service journal) + the durable
       fsync'd AuditLog that survives restart                      */
    auto audit = [&](int fd, const char* op,
                     const std::string& detail, int32_t rc = 0) {
        uint64_t s = 0; std::string nm;
        uint32_t pid = 0, uid = 0, gid = 0;
        ctl.clientInfo(fd, &s, &nm);
        ctl.peerCred(fd, &pid, &uid, &gid);
        fprintf(stderr, "audit s%llu %s: %s %s\n",
                (unsigned long long)s,
                nm.empty() ? "?" : nm.c_str(), op,
                detail.c_str());
        audit_log.record(pid, uid, gid, s,
                         nm.empty() ? "-" : nm.c_str(), op, detail,
                         rc);
    };

    /* authZ + rate limiting at the dispatch gate */
    if (!authz_path.empty()) {
        bool azok = false;
        auto azm = load_authz(authz_path, &azok);
        if (!azok) {
            fprintf(stderr, "tether-plc: --authz %s unreadable or "
                            "bad role\n", authz_path.c_str());
            tplc_image_close(&img);
            return 1;
        }
        ctl.setRoleResolver([azm](uint32_t uid) -> int {
            auto it = azm.find(uid);
            if (it != azm.end()) return it->second;
            return (uid == 0 || uid == geteuid())
                 ? tplc::TPLC_ROLE_COMMISSIONER
                 : tplc::TPLC_ROLE_OBSERVER;
        });
    }
    ctl.setOpMinRole([](uint16_t t) { return op_min_role(t); });
    ctl.setMutRate(20.0, 40.0);              /* 20 ops/s, burst 40  */
    ctl.setDenyHook([&](int fd, uint16_t type, int32_t code) {
        char op[24];
        snprintf(op, sizeof op, "op.0x%04x.denied", type);
        audit(fd, op, "gate", code);
    });

    /* ---- trigger expressions (doc 12 §M) ------------------------
       Compiler + evaluator for the bounded stack machine; paths
       resolve through the symmap at arm time                      */
    auto smap_find_in = [&](const std::vector<uint8_t>& smap,
                            std::string_view want,
                            uint64_t* off, uint32_t* len) -> bool {
        const uint8_t* sm = smap.data();
        if (smap.size() < 32 || memcmp(sm, "SMAP", 4) != 0)
            return false;
        uint32_t count, stroff, strln;
        memcpy(&count, sm + 8, 4);
        memcpy(&stroff, sm + 12, 4);
        memcpy(&strln, sm + 16, 4);
        if ((uint64_t)stroff + strln > smap.size() ||
            32ull + (uint64_t)count * 64 > smap.size())
            return false;
        const uint8_t* strs = sm + stroff;
        for (uint32_t i = 0; i < count; i++) {
            const uint8_t* e = sm + 32 + (size_t)i * 64;
            uint32_t noff; memcpy(&noff, e, 4);
            if (noff >= strln) continue;
            const char* nm = (const char*)strs + noff;
            if (!memchr(nm, 0, strln - noff)) continue;
            if (want != nm) continue;
            memcpy(off, e + 12, 8);
            memcpy(len, e + 20, 4);
            return true;
        }
        return false;
    };
    auto smap_find = [&](std::string_view want,
                         uint64_t* off, uint32_t* len) -> bool {
        return smap_find_in(pkg_smap, want, off, len);
    };
    /* compile expr text into X; returns "" or an error phrase */
    auto expr_compile = [&](std::string_view src,
                            Expr& X) -> std::string {
        struct C {
            Expr* X;
            std::string_view s;
            C(Expr* x, std::string_view sv) : X(x), s(sv) {}
            size_t i = 0;
            std::string err;
            void ws() { while (i < s.size() &&
                         (s[i] == ' ' || s[i] == '\t')) i++; }
            bool tok(std::string_view t) {
                ws();
                if (s.substr(i, t.size()) == t) { i += t.size();
                    return true; }
                return false;
            }
            bool ident(std::string_view& out) {
                ws();
                size_t b = i;
                while (i < s.size() &&
                       (isalnum((uint8_t)s[i]) || s[i] == '_' ||
                        s[i] == '.' || s[i] == '[' || s[i] == ']'))
                    i++;
                out = s.substr(b, i - b);
                return !out.empty() &&
                       !(isdigit((uint8_t)out[0]));
            }
            bool number(int64_t* v) {
                ws();
                size_t b = i;
                if (i + 1 < s.size() && s[i] == '0' &&
                    (s[i + 1] == 'x' || s[i + 1] == 'X')) {
                    i += 2;
                    while (i < s.size() && isxdigit((uint8_t)s[i])) i++;
                    if (i - b <= 2) return false;
                    *v = (int64_t)strtoll(
                        std::string(s.substr(b, i - b)).c_str(),
                        nullptr, 16);
                    return true;
                }
                while (i < s.size() && isdigit((uint8_t)s[i])) i++;
                if (i == b) return false;
                *v = strtoll(
                    std::string(s.substr(b, i - b)).c_str(),
                    nullptr, 10);
                return true;
            }
            bool emit(uint8_t op) {
                if (ops >= 64) { err = "expr too long";
                    return false; }
                ops++;
                X->code.push_back(op);
                return true;
            }
            int ops = 0;
            bool lit(int64_t v) {
                if (X->lits.size() >= 32) { err = "too many literals";
                    return false; }
                if (!emit(XO_LIT)) return false;
                uint16_t k = (uint16_t)(X->lits.size());
                X->lits.push_back(v);
                X->code.push_back((uint8_t)k);
                X->code.push_back((uint8_t)(k >> 8));
                return true;
            }
            int slot(std::string_view path) {
                for (size_t i = 0; i < slots.size(); i++)
                    if (slotnames[i] == path) return (int)i;
                if (slots.size() >= 16) { err = "too many paths";
                    return -1; }
                uint64_t off; uint32_t len;
                if (!find(path, &off, &len)) {
                    err = "unknown path " + std::string(path);
                    return -1;
                }
                if (len == 0 || len > 8) {
                    err = "path not scalar " + std::string(path);
                    return -1;
                }
                X->slots.push_back({off, len});
                X->prev.push_back(0);
                slots.push_back(1); slotnames.emplace_back(path);
                return (int)(X->slots.size() - 1);
            }
            std::vector<int> slots;
            std::vector<std::string> slotnames;
            std::function<bool(std::string_view, uint64_t*, uint32_t*)>
                find;
            bool e_or() {
                if (!e_and()) return false;
                while (tok("||")) {
                    if (!e_and() || !emit(XO_OR)) return false;
                }
                return true;
            }
            bool e_and() {
                if (!e_cmp()) return false;
                while (tok("&&")) {
                    if (!e_cmp() || !emit(XO_AND)) return false;
                }
                return true;
            }
            bool e_cmp() {
                if (!e_add()) return false;
                ws();
                uint8_t op = 0;
                if (tok("==")) op = XO_EQ;
                else if (tok("!=")) op = XO_NE;
                else if (tok("<=")) op = XO_LE;
                else if (tok(">=")) op = XO_GE;
                else if (tok("<"))  op = XO_LT;
                else if (tok(">"))  op = XO_GT;
                if (!op) return true;
                if (!e_add() || !emit(op)) return false;
                return true;
            }
            bool e_add() {
                if (!e_mul()) return false;
                for (;;) {
                    if (tok("+")) {
                        if (!e_mul() || !emit(XO_ADD)) return false;
                    } else if (tok("-")) {
                        if (!e_mul() || !emit(XO_SUB)) return false;
                    } else return true;
                }
            }
            bool e_mul() {
                if (!e_unary()) return false;
                for (;;) {
                    if (tok("*")) {
                        if (!e_unary() || !emit(XO_MUL)) return false;
                    } else if (tok("/")) {
                        if (!e_unary() || !emit(XO_DIV)) return false;
                    } else if (tok("%")) {
                        if (!e_unary() || !emit(XO_MOD)) return false;
                    } else return true;
                }
            }
            bool e_unary() {
                if (tok("!")) {
                    if (!e_unary() || !emit(XO_NOT)) return false;
                    return true;
                }
                if (tok("-")) {
                    if (!e_unary() || !emit(XO_NEG)) return false;
                    return true;
                }
                return e_prim();
            }
            bool e_prim() {
                int64_t v;
                if (number(&v)) return lit(v);
                if (tok("(")) {
                    if (!e_or() || !tok(")")) {
                        if (err.empty()) err = "missing )";
                        return false;
                    }
                    return true;
                }
                std::string_view id;
                if (!ident(id)) { err = "operand expected";
                    return false; }
                if (id == "true")  return lit(1);
                if (id == "false") return lit(0);
                if (id == "changed" || id == "rising" ||
                    id == "falling") {
                    uint8_t op = id == "changed" ? XO_CHANGED
                                 : id == "rising" ? XO_RISING
                                                  : XO_FALLING;
                    if (!tok("(")) { err = "missing ( after " +
                        std::string(id); return false; }
                    std::string_view path;
                    if (!ident(path) || !tok(")")) {
                        if (err.empty()) err = "bad " +
                            std::string(id) + " arg";
                        return false;
                    }
                    int si = slot(path);
                    if (si < 0) return false;
                    if (!emit(op)) return false;
                    X->code.push_back((uint8_t)si);
                    return true;
                }
                int si = slot(id);
                if (si < 0) return false;
                if (!emit(XO_SLOT)) return false;
                X->code.push_back((uint8_t)si);
                return true;
            }
        } c(&X, src);
        c.find = [&](std::string_view p, uint64_t* o, uint32_t* l) {
            return smap_find(p, o, l);
        };
        if (!c.e_or()) return c.err.empty() ? "syntax" : c.err;
        c.ws();
        if (c.i != c.s.size()) return "trailing input";
        return "";
    };
    /* evaluate X against the current blob; updates slot prev values */
    auto expr_eval = [&](Expr& X) -> bool {
        int64_t v[16];
        for (size_t i = 0; i < X.slots.size(); i++) {
            uint64_t u = 0;
            memcpy(&u, prog->blob() + X.slots[i].off, X.slots[i].len);
            switch (X.slots[i].len) {
                case 1: v[i] = (int8_t)u; break;
                case 2: v[i] = (int16_t)u; break;
                case 4: v[i] = (int32_t)u; break;
                default: v[i] = (int64_t)u; break;
            }
        }
        int64_t st[64]; int sp = 0;
        auto bin = [&](uint8_t op) {
            int64_t b = st[--sp], a = st[--sp];
            switch (op) {
                case XO_ADD: st[sp++] = a + b; break;
                case XO_SUB: st[sp++] = a - b; break;
                case XO_MUL: st[sp++] = a * b; break;
                case XO_DIV: st[sp++] = b ? a / b : 0; break;
                case XO_MOD: st[sp++] = b ? a % b : 0; break;
                case XO_EQ:  st[sp++] = a == b; break;
                case XO_NE:  st[sp++] = a != b; break;
                case XO_LT:  st[sp++] = a <  b; break;
                case XO_LE:  st[sp++] = a <= b; break;
                case XO_GT:  st[sp++] = a >  b; break;
                default:     st[sp++] = a >= b; break;
                case XO_AND: st[sp++] = a && b; break;
                case XO_OR:  st[sp++] = a || b; break;
            }
        };
        for (size_t pc = 0; pc < X.code.size();) {
            uint8_t op = X.code[pc++];
            if (op == XO_LIT) {
                uint16_t k = X.code[pc] | (X.code[pc + 1] << 8);
                pc += 2;
                st[sp++] = X.lits[k];
            } else if (op == XO_SLOT) {
                st[sp++] = v[X.code[pc++]];
            } else if (op == XO_NEG) {
                st[sp - 1] = -st[sp - 1];
            } else if (op == XO_NOT) {
                st[sp - 1] = !st[sp - 1];
            } else if (op >= XO_CHANGED) {
                uint8_t si = X.code[pc++];
                int64_t cur = v[si], pr = X.prev[si];
                st[sp++] = op == XO_CHANGED ? cur != pr
                          : op == XO_RISING  ? !pr && cur
                                             : pr && !cur;
            } else {
                bin(op);
            }
        }
        for (size_t i = 0; i < X.slots.size(); i++)
            X.prev[i] = v[i];
        return sp > 0 && st[sp - 1] != 0;
    };
    /* seed slot history from the current blob; used at arm time and
       after a swap re-resolves an expression against the new map   */
    auto expr_seed = [&](Expr& X) {
        for (size_t i = 0; i < X.slots.size(); i++) {
            uint64_t u = 0;
            memcpy(&u, prog->blob() + X.slots[i].off,
                   X.slots[i].len);
            switch (X.slots[i].len) {
                case 1: X.prev[i] = (int8_t)u; break;
                case 2: X.prev[i] = (int16_t)u; break;
                case 4: X.prev[i] = (int32_t)u; break;
                default: X.prev[i] = (int64_t)u; break;
            }
        }
    };

    /* subscriptions re-resolve against the live symmap: named
       watch items re-bind to their new range, expression triggers
       recompile and re-seed; vanished paths and offset-bound
       entries go stale instead of silently dropping so the client
       sees what broke                                             */
    reresolve_subs = [&] {
        for (auto& wt : watches)
            for (auto& it : wt.chs) {
                uint64_t o2 = 0; uint32_t l2 = 0;
                it.stale = it.path.empty() ||
                    it.shadow ||
                    !smap_find(it.path, &o2, &l2) || l2 > 64;
                if (!it.stale) { it.off = o2; it.len = l2; }
            }
        for (auto& b : breaks) {
            if (b.src.empty()) { b.stale = true; continue; }
            Expr X;
            if (expr_compile(b.src, X).empty()) {
                expr_seed(X);
                b.x = std::move(X);
                b.stale = false;
            } else b.stale = true;
        }
    };

    /* ---------------- control channel -------------------------- */
    if (ctl_enabled) {
        tplc::Status cst = ctl.open(ctl_path, 0660);
        if (!cst.ok())
            fprintf(stderr, "tether-plc: ctl %s failed: %s\n",
                    ctl_path.c_str(), cst.msg.c_str());
        /* else: keep running headless — ctl is optional per spec   */
    }

    uint64_t started_ns = mono_ns();

    ctl.on(TPLC_OP_PING, [&](int fd, uint16_t t,
                           const uint8_t*, uint32_t, uint64_t req) {
        tplc::CtlServer::W w;
        w.u64(mono_ns());
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    auto write_prog_status = [&](tplc::CtlServer::W& w) {
        TplcCtlProgStatus ps{};
        memcpy(ps.program_uuid, d.program_uuid, 16);
        memcpy(ps.name, d.name, TPLC_PROG_NAME_MAX);
        ps.state = prog_state;
        ps.task_count = d.task_count;
        ps.cycles = prog ? prog->ctx().cycle_no : 0;
        ps.last_fault_code = last_fault;
        ps.force_count = (uint32_t)forces.size();
        /* v2 (doc 16 §3.3): provenance + lineage + retain + swap */
        memcpy(ps.package_hash, pkg_hash, 32);
        memcpy(ps.layout_hash, d.layout_hash, 32);
        memcpy(ps.version, pkg_version, sizeof ps.version);
        ps.fault_task = fault_task;
        ps.fault_cycle = fault_cycle;
        ps.retain_restored = retain_restored;
        ps.retain_dirty_bytes =
            (prog && retain && retain->attached() && retain->dirty())
                ? (uint64_t)prog->retainSize() : 0;
        ps.retain_last_flush_ns = retain_last_flush_ns;
        ps.deployed_at_ns = lin_deployed_at_ns;
        ps.deployed_by_uid = lin_deployed_by;
        memcpy(ps.predecessor_uuid, lin_predecessor, 16);
        ps.swap_stage = quar.prog ? 1 : 0;
        ps.quarantine_left = quar.prog ? (uint32_t)quar.cycles_left : 0;
        ps.epoch = mgmt_epoch;
        ps.shadow_state = (shadow && shadow->prog) ? 1 : 0;
        ps.shadow_cycles = shadow ? shadow->cycles : 0;
        ps.shadow_missed = shadow ? shadow->missed : 0;
        ps.shadow_diff_count = shadow ? shadow->diff_count : 0;
        ps.shadow_last_diff_cycle =
            shadow ? shadow->last_diff_cycle : 0;
        w.bytes(&ps, sizeof ps);
        for (uint32_t i = 0; i < d.task_count; i++) {
            const TplcTaskDesc& td = d.tasks[i];
            TplcCtlTaskStat ts{};
            memcpy(ts.name, td.name, TPLC_TASK_NAME_MAX);
            ts.kind = td.kind;
            ts.state = prog_state == PS_RUNNING ? 1 : 0;
            if (prog && sched.suspended(*prog, i)) ts.state = 2;
            ts.period_ns = td.period_ns;
            if (prog) if (const auto* s = sched.stats(*prog, i)) {
                ts.runs = s->runs; ts.late = s->late;
                ts.faults = s->faults;
                ts.overruns = s->overruns;
                ts.exec_max_ns = s->exec_max_ns;
                ts.exec_avg_ns = s->execAvgNs();
                ts.last_late_ns = (int32_t)s->max_late_ns;
            }
            w.bytes(&ts, sizeof ts);
        }
    };

    ctl.on(TPLC_OP_PROG_STATUS, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        tplc::CtlServer::W w;
        write_prog_status(w);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_TASK_STATS, [&](int fd, uint16_t t,
                                   const uint8_t*, uint32_t,
                                   uint64_t req) {
        tplc::CtlServer::W w;
        write_prog_status(w);   /* same payload for now */
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_PROG_LIST, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        tplc::CtlServer::W w;
        w.u32(prog ? 1 : 0);            /* program count */
        if (prog) {
            w.bytes(d.program_uuid, 16);
            w.strn((const char*)d.name, TPLC_PROG_NAME_MAX);
            w.u8(prog_state); w.u8(0); w.u16(0);
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_PROG_STOP, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        /* STOP halts scheduling only — state is kept (doc 04 §4);
           forces drop on stop (doc 12 §8 force_on_stop default:
           a hidden force surviving a stop is a safety hazard)      */
        /* doc 16 §2: illegal transitions error with current+requested
           — no silent no-ops                                        */
        if (prog_state != PS_RUNNING) {
            char m[64];
            snprintf(m, sizeof m, "current=%u requested=stop",
                     prog_state);
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, m, req);
            return;
        }
        prog_state = PS_STOPPED;
        drop_shadow("stop");
        forces.clear();
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
        audit(fd, "prog.stop", "");
        emit_transition(PS_RUNNING, PS_STOPPED, 0);
        TplcCtlProgStateChange sc{PS_RUNNING, PS_STOPPED, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    ctl.on(TPLC_OP_PROG_START, [&](int fd, uint16_t t,
                                   const uint8_t*, uint32_t,
                                   uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (prog_state != PS_STOPPED) {
            char m[64];
            snprintf(m, sizeof m, "current=%u requested=start",
                     prog_state);
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, m, req);
            return;
        }
        prog->clearAbort();
        if (!prog_started) {
            tplc::Status sst = prog->start();
            if (!sst.ok()) {
                ctl.errorReply(fd, t, sst.code, sst.msg.c_str(),
                               req);
                return;
            }
            prog_started = true;
        }
        prog_state = PS_RUNNING;
        arm_default_trace();
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_RUN);
        audit(fd, "prog.start", "");
        emit_transition(PS_STOPPED, PS_RUNNING, 0);
        TplcCtlProgStateChange sc{PS_STOPPED, PS_RUNNING, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    /* kill: set the abort flag and deschedule immediately — ctl ops
       run between ticks, so no task is in-flight (doc 04 §4 kill)  */
    ctl.on(TPLC_OP_PROG_KILL, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        if (!prog || prog_state != PS_RUNNING) {
            char m[64];
            snprintf(m, sizeof m, "current=%u requested=kill",
                     prog_state);
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, m, req);
            return;
        }
        /* ctl ops run between ticks, so no task is in-flight: set the
           abort flag (harmless pending), deschedule immediately, and
           let start clear the flag — immediate kill per doc 04 §4   */
        prog->requestAbort();
        prog_state = PS_STOPPED;
        drop_shadow("kill");
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
        emit_transition(PS_RUNNING, PS_STOPPED, 0);
        TplcCtlProgStateChange sc{PS_RUNNING, PS_STOPPED, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    /* reset {u8 warm}: warm keeps retain bytes + file, cold zeroes
       both (doc 04 §4). Requires STOPPED.                         */
    ctl.on(TPLC_OP_PROG_RESET, [&](int fd, uint16_t t,
                                   const uint8_t* payload, uint32_t len,
                                   uint64_t req) {
        if (!prog || prog_state != PS_STOPPED) {
            char m[64];
            snprintf(m, sizeof m, "current=%u requested=reset",
                     prog_state);
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, m, req);
            return;
        }
        bool warm = len >= 1 ? payload[0] != 0 : true;
        if (!warm && retain && retain->attached()) retain->clear();
        tplc::Status rst = prog->reset(warm);
        if (!rst.ok()) {
            ctl.errorReply(fd, t, rst.code, rst.msg.c_str(), req);
            return;
        }
        prog_started = true;
        TplcCtlProgStateChange sc{PS_STOPPED, PS_STOPPED, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    ctl.on(TPLC_OP_PROG_UNLOAD, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (prog_state == PS_RUNNING) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE,
                           "current=running requested=unload", req);
            return;
        }
        unload_program();
        mgmt_epoch++;
        emit_transition(PS_STOPPED, PS_EMPTY, 0);
        TplcCtlProgStateChange sc{PS_STOPPED, PS_EMPTY, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    /* prepare {u16 len + path} or {16B package_uuid in --packages}:
       load (or swap to) a module — lands STOPPED; start runs init */
    /* plan 20: update ops authenticate against the update policy —
       strict upgrades every update to require-trusted regardless of
       the boot --sig-policy. --update-auth-map overrides per channel
       (peer uid, then role) and may name a different trust root;
       channels without an entry use the global policy. */
    const std::string update_policy =
        update_auth == "strict" ? "require-trusted" : sig_policy;
    auto update_auth_for = [&](int fd)
            -> std::pair<std::string, std::string> {
        uint32_t pid, uid, gid;
        if (ctl.peerCred(fd, &pid, &uid, &gid)) {
            auto u = update_auth_uid.find(uid);
            if (u != update_auth_uid.end())
                return {u->second.policy,
                        u->second.trust_dir.empty()
                            ? trust_dir : u->second.trust_dir};
        }
        auto it = update_auth_map.find(ctl.roleOf(fd));
        if (it == update_auth_map.end())
            return {update_policy, trust_dir};
        return {it->second.policy,
                it->second.trust_dir.empty() ? trust_dir
                                             : it->second.trust_dir};
    };
    ctl.on(TPLC_OP_PROG_PREPARE, [&](int fd, uint16_t t,
                                     const uint8_t* payload,
                                     uint32_t len, uint64_t req) {
        if (prog_state == PS_RUNNING) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE,
                           "current=running requested=prepare", req);
            return;
        }
        std::string path;
        if (!uuid_or_path(payload, len, packages_dir, path)) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "uuid or path", req);
            return;
        }
        std::string mpath = path;
        std::string tmp_so;
        std::vector<uint8_t> layd_b, smap_b, iomap_b; /* sections borrow pk */
        uint8_t new_hash[32] = {};
        std::string new_ver;
        /* .tplc → extract target_obj; .so → load directly */
        if (path.size() >= 5 &&
            path.compare(path.size() - 5, 5, ".tplc") == 0) {
            auto pres = tplc::Package::open(path);
            if (!pres.ok()) {
                ctl.errorReply(fd, t, pres.st.code,
                               pres.st.msg.c_str(), req);
                return;
            }
            tplc::Package pk = std::move(pres.value);
            auto ua = update_auth_for(fd);
            tplc::Status sgst = pk.verifySignature(ua.first, ua.second);
            if (!sgst.ok()) {
                ctl.errorReply(fd, t, sgst.code, sgst.msg.c_str(), req);
                return;
            }
            auto vst = pk.validateForLoad(false);
            if (!vst.ok()) {
                ctl.errorReply(fd, t, vst.code, vst.msg.c_str(), req);
                return;
            }
            auto rst = pk.checkRequires(tplc_image_in_size(&img),
                                        tplc_image_out_size(&img));
            if (!rst.ok()) {
                ctl.errorReply(fd, t, rst.code, rst.msg.c_str(), req);
                return;
            }
            tplc::PkgSection sec{};
            if (pk.section(TPLC_SEC_LAYOUT_DESC, sec))
                layd_b.assign(sec.data, sec.data + sec.size);
            if (pk.section(TPLC_SEC_SYMMAP, sec))
                smap_b.assign(sec.data, sec.data + sec.size);
            if (pk.section(TPLC_SEC_DEPLOY_MAP, sec))
                iomap_b.assign(sec.data, sec.data + sec.size);
            memcpy(new_hash, pk.header().content_hash, 32);
            new_ver = manifest_str(pk.manifestJson(), "version");
            tplc::PkgSection obj{};
            if (!pk.section(TPLC_SEC_TARGET_OBJ, obj)) {
                ctl.errorReply(fd, t, TPLC_E_PKG_TOC,
                               "no target_obj", req);
                return;
            }
            char tp[] = "/tmp/tplc_prep_XXXXXX";
            int tfd = mkstemp(tp);
            if (tfd < 0 ||
                write(tfd, obj.data, obj.size) != (ssize_t)obj.size) {
                if (tfd >= 0) close(tfd);
                ctl.errorReply(fd, t, tplc::TPLC_E_HOST_DLOPEN,
                               "extract failed", req);
                return;
            }
            close(tfd);
            tmp_so = tp; mpath = tmp_so;
        } else {
            /* a raw module path carries no signature section — only
               the open policy may load it; require* needs a .tplc
               package so the image is authenticated */
            auto ua = update_auth_for(fd);
            if (ua.first != "open") {
                ctl.errorReply(fd, t, TPLC_E_SIG_REQUIRED,
                               "unsigned module: .tplc package required",
                               req);
                return;
            }
        }
        /* the program being replaced (if any) is the predecessor —
           load_program destroys it, so capture its uuid first       */
        uint8_t old_uuid[16] = {};
        if (prog) memcpy(old_uuid, prog->desc().program_uuid, 16);
        tplc::Status lst = load_program(mpath);
        if (!lst.ok()) {
            ctl.errorReply(fd, t, lst.code, lst.msg.c_str(), req);
            return;
        }
        /* install the package's debug descriptors so resolve/watch/
           expr ops see symbols for prepare-loaded programs too */
        pkg_layd = std::move(layd_b);
        pkg_smap = std::move(smap_b);
        pkg_iomap = std::move(iomap_b);
        /* management lineage (doc 16 §3.3): deployer is the
           kernel-attested uid of the requesting client              */
        memcpy(lin_predecessor, old_uuid, 16);
        memcpy(pkg_hash, new_hash, 32);
        snprintf(pkg_version, sizeof pkg_version, "%s", new_ver.c_str());
        lin_deployed_at_ns = realtime_ns();
        uint32_t _p, _g;
        ctl.peerCred(fd, &_p, &lin_deployed_by, &_g);
        mgmt_epoch++;
        emit_transition(PS_EMPTY, PS_STOPPED, 0);
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
        TplcCtlProgStateChange sc{PS_EMPTY, PS_STOPPED, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });

    /* ---------------- online change (doc 08) ----------------------
       PROG_LAYOUT: live layout query for `tplcc --change-against`.
       PROG_SWAP: migrate-and-swap of the running program against a
       staged package — snapshot → tplc_migrate → pointer commit →
       quarantine. PROG_ROLLBACK: restore inside the window.       */
    ctl.on(TPLC_OP_PROG_LAYOUT, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        std::vector<uint8_t> out(48 + pkg_layd.size());
        memcpy(out.data(), d.program_uuid, 16);
        memcpy(out.data() + 16, d.layout_hash, 32);
        if (!pkg_layd.empty())
            memcpy(out.data() + 48, pkg_layd.data(), pkg_layd.size());
        ctl.reply(fd, t, out.data(), (uint32_t)out.size(), req);
    });

    ctl.on(TPLC_OP_PROG_ROLLBACK, [&](int fd, uint16_t t,
                                      const uint8_t*, uint32_t,
                                      uint64_t req) {
        if (!quar.prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no quarantined program", req);
            return;
        }
        do_rollback();
        emit_transition((uint8_t)PS_RUNNING, (uint8_t)PS_RUNNING, 2);
        fprintf(stderr, "tether-plc: rollback restored prior program\n");
        ctl.replyEmpty(fd, t, req);
    });

    ctl.on(TPLC_OP_PROG_SWAP, [&](int fd, uint16_t t,
                                  const uint8_t* payload,
                                  uint32_t len, uint64_t req) {
        if (prog_state != PS_RUNNING || !prog) {
            char m[64];
            snprintf(m, sizeof m, "current=%u requested=swap",
                     prog_state);
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, m, req);
            return;
        }
        /* resolve the staged package — 16B uuid or {len,path} */
        std::string path;
        if (!uuid_or_path(payload, len, packages_dir, path)) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "uuid or path", req);
            return;
        }
        auto pres = tplc::Package::open(path);
        if (!pres.ok()) {
            ctl.errorReply(fd, t, pres.st.code, pres.st.msg.c_str(),
                           req);
            return;
        }
        tplc::Package pk = std::move(pres.value);
        auto ua = update_auth_for(fd);
        tplc::Status sgst = pk.verifySignature(ua.first, ua.second);
        if (!sgst.ok()) {
            ctl.errorReply(fd, t, sgst.code, sgst.msg.c_str(), req);
            return;
        }
        auto vst = pk.validateForLoad(false);
        if (!vst.ok()) {
            ctl.errorReply(fd, t, vst.code, vst.msg.c_str(), req);
            return;
        }
        auto rst = pk.checkRequires(tplc_image_in_size(&img),
                                    tplc_image_out_size(&img));
        if (!rst.ok()) {
            ctl.errorReply(fd, t, rst.code, rst.msg.c_str(), req);
            return;
        }
        tplc::PkgSection obj{}, migd{}, layd{}, smap{}, imap{};
        if (!pk.section(TPLC_SEC_TARGET_OBJ, obj)) {
            ctl.errorReply(fd, t, TPLC_E_PKG_TOC, "no target_obj",
                           req);
            return;
        }
        bool has_migd = pk.section(TPLC_SEC_MIGRATE_DESC, migd);
        /* references are layout-relative: the migrate plan resets them
           (kind=3 dropped, aux bit1) and the swap says so loudly    */
        if (has_migd && migd.size >= 38) {
            const uint8_t* m = migd.data;
            uint16_t vlen; memcpy(&vlen, m + 32, 2);
            size_t at = 34 + vlen;
            at += (4 - (at % 4)) % 4;
            if (at + 4 <= migd.size) {
                uint32_t nr; memcpy(&nr, m + at, 4); at += 4;
                uint32_t refs = 0, kept = 0;
                for (uint32_t i = 0; i < nr && at + 16 <= migd.size;
                     i++, at += 16) {
                    uint8_t kind = m[at];
                    uint32_t aux; memcpy(&aux, m + at + 12, 4);
                    if (kind == 3 && (aux & 2)) refs++;
                    /* bit2: a reference copy the planner kept because
                       the referenced layout did not move */
                    if (kind == 0 && (aux & 4)) kept++;
                }
                if (refs || kept)
                    fprintf(stderr,
                            "tether-plc: online change: %u reference "
                            "field(s) preserved (layout unchanged), "
                            "%u reset (layout moved)\n", kept, refs);
            }
        }
        pk.section(TPLC_SEC_LAYOUT_DESC, layd);
        pk.section(TPLC_SEC_SYMMAP, smap);
        pk.section(TPLC_SEC_DEPLOY_MAP, imap);
        char tp[] = "/tmp/tplc_swap_XXXXXX";
        int tfd = mkstemp(tp);
        if (tfd < 0 ||
            write(tfd, obj.data, obj.size) != (ssize_t)obj.size) {
            if (tfd >= 0) close(tfd);
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_DLOPEN,
                           "extract failed", req);
            return;
        }
        close(tfd);
        auto mres = tplc::Module::open(tp, /*host_caps*/ 0);
        if (!mres.ok()) {
            ctl.errorReply(fd, t, mres.st.code, mres.st.msg.c_str(),
                           req);
            return;
        }
        tplc::Module nmod = std::move(mres.value);
        const TplcProgramDescriptor& nd = nmod.descriptor();
        if (nd.image_in_bytes > tplc_image_in_size(&img) ||
            nd.image_out_bytes > tplc_image_out_size(&img)) {
            ctl.errorReply(fd, t, TPLC_E_IMG_TOO_SMALL,
                           "image too small", req);
            return;
        }
        bool same_layout =
            memcmp(nd.layout_hash, d.layout_hash, 32) == 0;
        if (!same_layout) {
            if (!nmod.hasMigrate() || !has_migd) {
                ctl.errorReply(fd, t, TPLC_E_MIGRATE_LAYOUT,
                               "no migrate path in package", req);
                return;
            }
            /* §8.2 race gate: migd predecessor hash vs live layout */
            if (migd.size < 32 ||
                memcmp(migd.data, d.layout_hash, 32) != 0) {
                uint8_t rep[64] = {};
                memcpy(rep, d.layout_hash, 32);
                if (migd.size >= 32)
                    memcpy(rep + 32, migd.data, 32);
                ctl.reply(fd, t, rep, sizeof rep, req); /* both hashes */
                fprintf(stderr,
                        "tether-plc: swap refused — SWAP_RACE "
                        "(compiled against stale layout)\n");
                return;
            }
        }
        auto nres = tplc::Program::create(nmod, pcfg);
        if (!nres.ok()) {
            ctl.errorReply(fd, t, nres.st.code, nres.st.msg.c_str(),
                           req);
            return;
        }
        tplc::Program nprog = std::move(nres.value);
        tplc::Status ist = nprog.start();      /* tplc_init: defaults */
        if (!ist.ok()) {
            ctl.errorReply(fd, t, ist.code, ist.msg.c_str(), req);
            return;
        }
        /* the pause: blob snapshot — everything else runs off-cycle
           relative to the old program which simply is not ticked
           while ctl is being serviced                              */
        std::vector<uint8_t> snap = prog->snapshotBlob();
        TplcMigrateCtx mo{};
        memset(&mo, 0, sizeof mo);
        if (same_layout) {
            if (nprog.blobSize() == snap.size())
                memcpy(nprog.blob(), snap.data(), snap.size());
        } else {
            int32_t mrc = nprog.migrateFrom(*prog, migd.data,
                                            migd.size, &mo);
            if (mrc != 0) {
                /* migrate refused/failed — old program untouched */
                uint8_t rep[8];
                memcpy(rep, &mo.out_status, 4);
                memcpy(rep + 4, &mo.out_first_error, 4);
                ctl.reply(fd, t, rep, sizeof rep, req);
                fprintf(stderr,
                        "tether-plc: migrate rc=%d status=%u — "
                        "swap aborted, old program running\n",
                        mrc, mo.out_status);
                return;
            }
        }
        /* commit: quarantine old, adopt new — the scheduler keeps
           Program* pointers, so registration happens only after
           every object rests at its final storage address (heap
           Modules move owners freely — their address never does)   */
        adopt_begin();
        std::vector<bool> susp = snapshot_suspension(*prog);
        sched.removeProgram(*prog);
        tplc::Program oldp = std::move(*prog);
        prog.reset();
        quar.prog.emplace(std::move(oldp));
        quar.mod = std::move(mod);
        mod.reset();
        quar.layd = std::move(pkg_layd);
        quar.smap = std::move(pkg_smap);
        sdiff.ready = true;
        sdiff.same_layout = same_layout;
        sdiff.mig_status = same_layout ? 0 : mo.out_status;
        sdiff.migd.clear();
        /* only a real migrate ran the MIGD rules; same-layout swaps
           byte-copy the blob and the embedded table is a stale
           description of the packaged predecessor                  */
        if (!same_layout && migd.data && migd.size)
            sdiff.migd.assign(migd.data, migd.data + migd.size);
        sdiff.old_blob = std::move(snap);
        sdiff.old_smap = quar.smap;
        drop_shadow("swap");   /* candidate is stale post-swap    */
        forces.clear();   /* new blob layout — overrides are stale */
        trace_ch.clear();
        trace_ring.clear();
        trace_wr = 0;
        trace_flags = 0;
        trace_frozen = false;
        quar.cycles_left = quarantine_cycles;
        mod = std::make_unique<tplc::Module>(std::move(nmod));
        /* rebind: Program::create captured &nmod — retarget the new
           program's mod_ to the heap Module before it ticks        */
        prog.emplace(std::move(nprog));
        prog->rebindModule(*mod);
        sched.addProgram(*prog);
        apply_suspension(*prog, susp);
        adopt_end();
        d = mod->descriptor();
        if (layd.data && layd.size)
            pkg_layd.assign(layd.data, layd.data + layd.size);
        else
            pkg_layd.clear();
        if (smap.data && smap.size)
            pkg_smap.assign(smap.data, smap.data + smap.size);
        else
            pkg_smap.clear();
        if (imap.data && imap.size)
            pkg_iomap.assign(imap.data, imap.data + imap.size);
        else
            pkg_iomap.clear();
        if (!retain_dir.empty()) {
            if (!retain) retain.emplace();
            retain->close();
            uint64_t lh;
            memcpy(&lh, d.layout_hash, 8);
            /* no load(): the migrated blob already carries retain */
            retain->open(retain_dir, d.program_uuid, lh,
                         prog->retainSize());
        }
        reresolve_subs();
        prog_state = PS_RUNNING;
        prog_started = true;
        arm_default_trace();
        last_fault = 0;
        fprintf(stderr,
                "tether-plc: online change committed (%s, "
                "quarantine %d scans)\n",
                same_layout ? "module-swap" : "migrated",
                quarantine_cycles);
        /* lineage (doc 16 §3.3): quarantined program is the
           predecessor; deployer is the requesting client's uid      */
        memcpy(lin_predecessor, quar.prog->desc().program_uuid, 16);
        /* quarantined side keeps the prior package identity so
           rollback restores lineage, not just code                  */
        memcpy(quar.hash, pkg_hash, 32);
        snprintf(quar.ver, sizeof quar.ver, "%s", pkg_version);
        memcpy(pkg_hash, pk.header().content_hash, 32);
        snprintf(pkg_version, sizeof pkg_version, "%s",
                 manifest_str(pk.manifestJson(), "version").c_str());
        lin_deployed_at_ns = realtime_ns();
        {
            uint32_t _p, _g;
            ctl.peerCred(fd, &_p, &lin_deployed_by, &_g);
        }
        mgmt_epoch++;
        emit_transition((uint8_t)PS_RUNNING, (uint8_t)PS_RUNNING, 0);
        uint8_t rep[8];
        memcpy(rep, &mo.out_status, 4);
        memcpy(rep + 4, &mo.out_first_error, 4);
        ctl.reply(fd, t, rep, sizeof rep, req);
    });

    /* ---- shadow run ops (doc 19 §3) -----------------------------

       SHADOW_LOAD stages a candidate program: sig-checked like a
       swap, blob seeded by migrate-from-active (or same-layout byte
       copy), tasks registered on the shadow scheduler.            */
    ctl.on(TPLC_OP_SHADOW_LOAD, [&](int fd, uint16_t t,
                                    const uint8_t* payload,
                                    uint32_t len, uint64_t req) {
        if (prog_state != PS_RUNNING || !prog) {
            char m[64];
            snprintf(m, sizeof m, "current=%u requested=shadow.load",
                     prog_state);
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, m, req);
            return;
        }
        if (shadow) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE,
                           "shadow already loaded", req);
            return;
        }
        std::string path;
        if (!uuid_or_path(payload, len, packages_dir, path)) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "uuid or path", req);
            return;
        }
        auto pres = tplc::Package::open(path);
        if (!pres.ok()) {
            ctl.errorReply(fd, t, pres.st.code, pres.st.msg.c_str(),
                           req);
            return;
        }
        tplc::Package pk = std::move(pres.value);
        auto ua = update_auth_for(fd);
        tplc::Status sgst = pk.verifySignature(ua.first, ua.second);
        if (!sgst.ok()) {
            ctl.errorReply(fd, t, sgst.code, sgst.msg.c_str(), req);
            return;
        }
        auto vst = pk.validateForLoad(false);
        if (!vst.ok()) {
            ctl.errorReply(fd, t, vst.code, vst.msg.c_str(), req);
            return;
        }
        auto rst = pk.checkRequires(tplc_image_in_size(&img),
                                    tplc_image_out_size(&img));
        if (!rst.ok()) {
            ctl.errorReply(fd, t, rst.code, rst.msg.c_str(), req);
            return;
        }
        tplc::PkgSection obj{}, migd{}, layd{}, smap{}, imap{};
        if (!pk.section(TPLC_SEC_TARGET_OBJ, obj)) {
            ctl.errorReply(fd, t, TPLC_E_PKG_TOC, "no target_obj",
                           req);
            return;
        }
        bool has_migd = pk.section(TPLC_SEC_MIGRATE_DESC, migd);
        /* references are layout-relative: the migrate plan resets them
           (kind=3 dropped, aux bit1) and the swap says so loudly    */
        if (has_migd && migd.size >= 38) {
            const uint8_t* m = migd.data;
            uint16_t vlen; memcpy(&vlen, m + 32, 2);
            size_t at = 34 + vlen;
            at += (4 - (at % 4)) % 4;
            if (at + 4 <= migd.size) {
                uint32_t nr; memcpy(&nr, m + at, 4); at += 4;
                uint32_t refs = 0, kept = 0;
                for (uint32_t i = 0; i < nr && at + 16 <= migd.size;
                     i++, at += 16) {
                    uint8_t kind = m[at];
                    uint32_t aux; memcpy(&aux, m + at + 12, 4);
                    if (kind == 3 && (aux & 2)) refs++;
                    /* bit2: a reference copy the planner kept because
                       the referenced layout did not move */
                    if (kind == 0 && (aux & 4)) kept++;
                }
                if (refs || kept)
                    fprintf(stderr,
                            "tether-plc: online change: %u reference "
                            "field(s) preserved (layout unchanged), "
                            "%u reset (layout moved)\n", kept, refs);
            }
        }
        pk.section(TPLC_SEC_LAYOUT_DESC, layd);
        pk.section(TPLC_SEC_SYMMAP, smap);
        pk.section(TPLC_SEC_DEPLOY_MAP, imap);
        char tp[] = "/tmp/tplc_shadow_XXXXXX";
        int tfd = mkstemp(tp);
        if (tfd < 0 ||
            write(tfd, obj.data, obj.size) != (ssize_t)obj.size) {
            if (tfd >= 0) close(tfd);
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_DLOPEN,
                           "extract failed", req);
            return;
        }
        close(tfd);
        auto mres = tplc::Module::open(tp, /*host_caps*/ 0);
        if (!mres.ok()) {
            ctl.errorReply(fd, t, mres.st.code, mres.st.msg.c_str(),
                           req);
            return;
        }
        tplc::Module nmod = std::move(mres.value);
        const TplcProgramDescriptor& nd = nmod.descriptor();
        if (nd.image_in_bytes > tplc_image_in_size(&img) ||
            nd.image_out_bytes > tplc_image_out_size(&img)) {
            ctl.errorReply(fd, t, TPLC_E_IMG_TOO_SMALL,
                           "image too small", req);
            return;
        }
        bool same_layout =
            memcmp(nd.layout_hash, d.layout_hash, 32) == 0;
        if (!same_layout) {
            if (!nmod.hasMigrate() || !has_migd) {
                ctl.errorReply(fd, t, TPLC_E_MIGRATE_LAYOUT,
                               "no migrate path in package", req);
                return;
            }
            if (migd.size < 32 ||
                memcmp(migd.data, d.layout_hash, 32) != 0) {
                uint8_t rep[64] = {};
                memcpy(rep, d.layout_hash, 32);
                if (migd.size >= 32)
                    memcpy(rep + 32, migd.data, 32);
                ctl.reply(fd, t, rep, sizeof rep, req);
                fprintf(stderr,
                        "tether-plc: shadow load refused — SWAP_RACE "
                        "(compiled against stale layout)\n");
                return;
            }
        }
        auto nres = tplc::Program::create(nmod, pcfg);
        if (!nres.ok()) {
            ctl.errorReply(fd, t, nres.st.code, nres.st.msg.c_str(),
                           req);
            return;
        }
        tplc::Program nprog = std::move(nres.value);
        tplc::Status ist = nprog.start();
        if (!ist.ok()) {
            ctl.errorReply(fd, t, ist.code, ist.msg.c_str(), req);
            return;
        }
        /* seed the shadow blob from the live program: same-layout
           byte copy, or tplc_migrate with the package MIGD          */
        TplcMigrateCtx mo{};
        memset(&mo, 0, sizeof mo);
        std::vector<uint8_t> snap = prog->snapshotBlob();
        if (same_layout) {
            if (nprog.blobSize() == snap.size())
                memcpy(nprog.blob(), snap.data(), snap.size());
        } else {
            int32_t mrc = nprog.migrateFrom(*prog, migd.data,
                                            migd.size, &mo);
            if (mrc != 0) {
                uint8_t rep[8];
                memcpy(rep, &mo.out_status, 4);
                memcpy(rep + 4, &mo.out_first_error, 4);
                ctl.reply(fd, t, rep, sizeof rep, req);
                fprintf(stderr,
                        "tether-plc: shadow migrate rc=%d "
                        "status=%u — not loaded\n", mrc, mo.out_status);
                return;
            }
        }
        Shadow sh;
        sh.mig_ready = true;
        sh.same_layout = same_layout;
        sh.mig_status = same_layout ? 0 : mo.out_status;
        if (!same_layout && migd.data && migd.size)
            sh.migd.assign(migd.data, migd.data + migd.size);
        if (layd.data && layd.size)
            sh.layd.assign(layd.data, layd.data + layd.size);
        if (smap.data && smap.size)
            sh.smap.assign(smap.data, smap.data + smap.size);
        if (imap.data && imap.size)
            sh.imap.assign(imap.data, imap.data + imap.size);
        memcpy(sh.pkg_hash, pk.header().content_hash, 32);
        snprintf(sh.pkg_version, sizeof sh.pkg_version, "%s",
                 manifest_str(pk.manifestJson(), "version").c_str());
        sh.mod = std::make_unique<tplc::Module>(std::move(nmod));
        sh.prog.emplace(std::move(nprog));
        sh.prog->rebindModule(*sh.mod);
        /* the scheduler keeps Program* — register only after the
           Program rests at its final address inside `shadow`      */
        shadow.emplace(std::move(sh));
        shadow->sched.addProgram(*shadow->prog);
        audit(fd, "shadow.load", path);
        fprintf(stderr, "tether-plc: shadow loaded (%.*s)\n",
                TPLC_PROG_NAME_MAX, (const char*)nd.name);
        tplc::CtlServer::W w;
        w.u8(1);                    /* loaded */
        w.bytes(shadow->prog->desc().program_uuid, 16);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    auto write_shadow_status = [&](tplc::CtlServer::W& w) {
        if (!shadow || !shadow->prog) { w.u8(0); return; }
        const TplcProgramDescriptor& sd = shadow->prog->desc();
        TplcCtlShadowStatus ss{};
        memcpy(ss.program_uuid, sd.program_uuid, 16);
        memcpy(ss.name, sd.name, TPLC_PROG_NAME_MAX);
        ss.state = 1;
        ss.cycles = shadow->cycles;
        ss.missed = shadow->missed;
        ss.diff_count = shadow->diff_count;
        ss.last_diff_cycle = shadow->last_diff_cycle;
        for (uint32_t i = 0; i < sd.task_count; i++)
            if (const auto* s = shadow->sched.stats(*shadow->prog, i))
                ss.runs += s->runs;
        w.bytes(&ss, sizeof ss);
    };

    ctl.on(TPLC_OP_SHADOW_STATUS, [&](int fd, uint16_t t,
                                      const uint8_t*, uint32_t,
                                      uint64_t req) {
        tplc::CtlServer::W w;
        write_shadow_status(w);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    ctl.on(TPLC_OP_SHADOW_COMPARE, [&](int fd, uint16_t t,
                                       const uint8_t* p, uint32_t n,
                                       uint64_t req) {
        if (n != 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 max}", req);
            return;
        }
        uint32_t maxn; memcpy(&maxn, p, 4);
        tplc::CtlServer::W w;
        if (!shadow) { w.u32(0); ctl.reply(fd, t, w.data(), w.size(), req); return; }
        size_t have = shadow->diffs.size();
        size_t take = std::min<size_t>(maxn, have);
        /* ring holds chronological entries; reply the newest `take`
           oldest-first                                              */
        w.u32((uint32_t)take);
        for (size_t i = have - take; i < have; i++) {
            const ShadowDiff& df = shadow->diffs[i];
            w.u64(df.cycle); w.u64(df.img_off); w.u32(df.len);
            w.bytes(df.active_bytes, 32);
            w.bytes(df.shadow_bytes, 32);
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    ctl.on(TPLC_OP_SHADOW_DROP, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        if (!shadow) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, "no shadow",
                           req);
            return;
        }
        drop_shadow("op");
        audit(fd, "shadow.drop", "op");
        ctl.replyEmpty(fd, t, req);
    });

    ctl.on(TPLC_OP_SHADOW_RESET, [&](int fd, uint16_t t,
                                     const uint8_t*, uint32_t,
                                     uint64_t req) {
        if (!shadow || !shadow->prog) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, "no shadow",
                           req);
            return;
        }
        TplcMigrateCtx mo{};
        memset(&mo, 0, sizeof mo);
        std::vector<uint8_t> snap = prog->snapshotBlob();
        if (shadow->same_layout) {
            if (shadow->prog->blobSize() == snap.size())
                memcpy(shadow->prog->blob(), snap.data(), snap.size());
            mo.out_status = 0;
        } else {
            if (shadow->migd.empty()) {
                ctl.errorReply(fd, t, TPLC_E_MIGRATE_LAYOUT,
                               "no migrate descriptor", req);
                return;
            }
            int32_t mrc = shadow->prog->migrateFrom(
                *prog, shadow->migd.data(), shadow->migd.size(), &mo);
            if (mrc != 0) {
                uint8_t rep[8];
                memcpy(rep, &mo.out_status, 4);
                memcpy(rep + 4, &mo.out_first_error, 4);
                ctl.reply(fd, t, rep, sizeof rep, req);
                return;
            }
        }
        shadow->diffs.clear(); shadow->diff_wr = 0;
        shadow->diff_count = 0; shadow->last_diff_cycle = 0;
        audit(fd, "shadow.reset", "resync");
        uint8_t rep[8];
        memcpy(rep, &mo.out_status, 4);
        memcpy(rep + 4, &mo.out_first_error, 4);
        ctl.reply(fd, t, rep, sizeof rep, req);
    });

    /* SHADOW_PROMOTE: quarantine active, adopt shadow — same commit
       dance as swap, except the blob is already live and no migrate
       runs here (it ran at SHADOW_LOAD)                            */
    ctl.on(TPLC_OP_SHADOW_PROMOTE, [&](int fd, uint16_t t,
                                       const uint8_t*, uint32_t,
                                       uint64_t req) {
        if (prog_state != PS_RUNNING || !prog || !shadow ||
            !shadow->prog) {
            char m[64];
            snprintf(m, sizeof m, "current=%u requested=promote",
                     prog_state);
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, m, req);
            return;
        }
        sdiff.old_blob = prog->snapshotBlob();
        adopt_begin();
        std::vector<bool> susp = snapshot_suspension(*prog);
        sched.removeProgram(*prog);
        quar.prog.emplace(std::move(*prog));
        prog.reset();
        quar.mod = std::move(mod);
        mod.reset();
        quar.layd = std::move(pkg_layd);
        quar.smap = std::move(pkg_smap);
        memcpy(quar.hash, pkg_hash, 32);
        snprintf(quar.ver, sizeof quar.ver, "%s", pkg_version);
        quar.cycles_left = quarantine_cycles;
        sdiff.ready = shadow->mig_ready;
        sdiff.same_layout = shadow->same_layout;
        sdiff.mig_status = shadow->mig_status;
        sdiff.migd = std::move(shadow->migd);
        sdiff.old_smap = quar.smap;
        mod = std::move(shadow->mod);
        prog.emplace(std::move(*shadow->prog));
        prog->rebindModule(*mod);
        sched.addProgram(*prog);
        apply_suspension(*prog, susp);
        adopt_end();
        d = mod->descriptor();
        pkg_layd = std::move(shadow->layd);
        pkg_smap = std::move(shadow->smap);
        pkg_iomap = std::move(shadow->imap);
        memcpy(pkg_hash, shadow->pkg_hash, 32);
        snprintf(pkg_version, sizeof pkg_version, "%s",
                 shadow->pkg_version);
        shadow.reset();
        if (!retain_dir.empty()) {
            if (!retain) retain.emplace();
            retain->close();
            uint64_t lh;
            memcpy(&lh, d.layout_hash, 8);
            retain->open(retain_dir, d.program_uuid, lh,
                         prog->retainSize());
        }
        forces.clear();   /* new program state — overrides stale */
        trace_ch.clear();
        trace_ring.clear();
        trace_wr = 0;
        trace_flags = 0;
        trace_frozen = false;
        memcpy(lin_predecessor, quar.prog->desc().program_uuid, 16);
        lin_deployed_at_ns = realtime_ns();
        {
            uint32_t _p, _g;
            ctl.peerCred(fd, &_p, &lin_deployed_by, &_g);
        }
        reresolve_subs();
        arm_default_trace();
        last_fault = 0;
        mgmt_epoch++;
        emit_transition((uint8_t)PS_RUNNING, (uint8_t)PS_RUNNING, 3);
        audit(fd, "shadow.promote", "adopted");
        fprintf(stderr,
                "tether-plc: shadow promoted (quarantine %d scans)\n",
                quarantine_cycles);
        tplc::CtlServer::W w;
        w.bytes(d.program_uuid, 16);
        w.u64(mgmt_epoch);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    /* map.get / journal.tail (doc 16 §3.2): the deploy map and the
       durable audit tail, read-only at observer role               */
    ctl.on(TPLC_OP_MAP_GET, [&](int fd, uint16_t t,
                                const uint8_t*, uint32_t,
                                uint64_t req) {
        ctl.reply(fd, t, pkg_iomap.empty() ? nullptr : pkg_iomap.data(),
                  (uint32_t)pkg_iomap.size(), req);
    });
    ctl.on(TPLC_OP_JOURNAL_TAIL, [&](int fd, uint16_t t,
                                     const uint8_t* payload,
                                     uint32_t len, uint64_t req) {
        uint32_t n = 20;
        if (len >= 4) memcpy(&n, payload, 4);
        if (n > 256) n = 256;
        tplc::CtlServer::W w;
        auto lines = audit_log.tail(n);
        w.u32((uint32_t)lines.size());
        for (auto& l : lines) w.str(l.c_str(), (uint32_t)l.size());
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    ctl.on(TPLC_OP_IMAGE_STATS, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        TplcCtlImageStats is{};
        is.epoch       = tplc_image_epoch(&img);
        is.in_seq      = tplc_img_load_u64(&img.hdr->in_seq);
        is.out_seq     = tplc_img_load_u64(&img.hdr->out_seq);
        is.in_writes   = tplc_image_in_writes(&img);
        is.out_writes  = tplc_image_out_writes(&img);
        is.in_retries  = tplc_image_in_retries(&img);
        is.out_retries = tplc_image_out_retries(&img);
        is.in_size     = tplc_image_in_size(&img);
        is.out_size    = tplc_image_out_size(&img);
        ctl.reply(fd, t, &is, sizeof is, req);
    });
    ctl.on(TPLC_OP_RETAIN_STATUS, [&](int fd, uint16_t t,
                                      const uint8_t*, uint32_t,
                                      uint64_t req) {
        tplc::CtlServer::W w;
        bool att = retain && retain->attached();
        w.u8(att ? 1 : 0);
        w.u8(att && retain->dirty() ? 1 : 0);
        w.u16(0);
        w.u64(att ? retain->len() : 0);
        w.u64(att ? retain->generation() : 0);
        w.u64(att ? retain->flushes() : 0);
        w.u64(att ? retain->restores() : 0);
        w.u64(att ? retain->losses() : 0);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    /* ---------------- object dictionary (doc 16 §7) ----------------
       CiA-405-style index/subindex view over the same state the
       status ops expose; entry-level authz below the op gate.     */
    enum : uint8_t { OD_T_U8 = 0, OD_T_U16, OD_T_U32, OD_T_U64,
                     OD_T_I32, OD_T_STR, OD_T_BYTES };
    enum : uint8_t { OD_RO = 0, OD_RW, OD_WO };

    struct OdRow {
        uint16_t index;
        uint8_t  sub;
        uint8_t  type;
        uint8_t  access;
        uint8_t  min_role;
        const char* name;
        std::vector<uint8_t> bytes;
    };
    auto od_put = [](std::vector<uint8_t>& v, const void* p, size_t n) {
        const uint8_t* b = (const uint8_t*)p;
        v.insert(v.end(), b, b + n);
    };
    /* enumerate the dictionary — `sub == 0xFF` means "any sub" */
    auto od_rows = [&](uint16_t index, uint8_t sub) {
        std::vector<OdRow> rows;
        auto add = [&](uint16_t i, uint8_t sb, uint8_t ty, uint8_t acc,
                       uint8_t role, const char* nm,
                       std::vector<uint8_t> b) {
            rows.push_back(OdRow{i, sb, ty, acc, role, nm, std::move(b)});
        };
        auto want = [&](uint16_t i, uint8_t sb) {
            return (index == 0xFFFF || index == i) &&
                   (sub == 0xFF || sub == sb);
        };
        if (want(0x2000, 0)) {
            add(0x2000, 0, OD_T_U8, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "state", {(uint8_t)prog_state});
        }
        if (want(0x2001, 0)) {
            std::vector<uint8_t> b(d.program_uuid,
                                   d.program_uuid + 16);
            add(0x2001, 0, OD_T_BYTES, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "program_uuid", std::move(b));
        }
        if (want(0x2002, 0)) {
            uint64_t cyc = prog ? prog->ctx().cycle_no : 0;
            std::vector<uint8_t> b(8);
            memcpy(b.data(), &cyc, 8);
            add(0x2002, 0, OD_T_U64, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "cycles", std::move(b));
        }
        if (want(0x2003, 0)) {
            int32_t lf = last_fault;
            std::vector<uint8_t> b(4);
            memcpy(b.data(), &lf, 4);
            add(0x2003, 0, OD_T_I32, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "last_fault", std::move(b));
        }
        if (want(0x2004, 0)) {
            uint64_t e = mgmt_epoch;
            std::vector<uint8_t> b(8);
            memcpy(b.data(), &e, 8);
            add(0x2004, 0, OD_T_U64, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "epoch", std::move(b));
        }
        if (want(0x2100, 0)) {
            uint32_t n = d.task_count;
            std::vector<uint8_t> b(4);
            memcpy(b.data(), &n, 4);
            add(0x2100, 0, OD_T_U32, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "task_count", std::move(b));
        }
        /* per-task stats rows: runs, late, faults, overruns, max_late,
           exec_max, exec_avg (7 × u64) */
        if (prog) {
            for (uint32_t i = 0; i < d.task_count; i++) {
                if (!want(0x2100, (uint8_t)(i + 1))) continue;
                const tplc::Scheduler::TaskStats* st =
                    sched.stats(*prog, i);
                if (!st) continue;
                uint64_t v[7] = { st->runs, st->late, st->faults,
                                  st->overruns,
                                  (uint64_t)st->max_late_ns,
                                  st->exec_max_ns, st->execAvgNs() };
                std::vector<uint8_t> b(sizeof v);
                memcpy(b.data(), v, sizeof v);
                add(0x2100, (uint8_t)(i + 1), OD_T_BYTES, OD_RO,
                    tplc::TPLC_ROLE_OBSERVER, "task_stats", std::move(b));
            }
        }
        if (want(0x2200, 0)) {
            uint32_t n = tplc_image_in_size(&img);
            std::vector<uint8_t> b(4);
            memcpy(b.data(), &n, 4);
            add(0x2200, 0, OD_T_U32, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "image_in_size", std::move(b));
        }
        if (want(0x2201, 0)) {
            uint32_t n = tplc_image_out_size(&img);
            std::vector<uint8_t> b(4);
            memcpy(b.data(), &n, 4);
            add(0x2201, 0, OD_T_U32, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "image_out_size", std::move(b));
        }
        if (want(0x2300, 0)) {
            std::vector<uint8_t> b(pkg_version,
                                   pkg_version + strlen(pkg_version));
            add(0x2300, 0, OD_T_STR, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "version", std::move(b));
        }
        if (want(0x2301, 0)) {
            std::vector<uint8_t> b(d.layout_hash, d.layout_hash + 32);
            add(0x2301, 0, OD_T_BYTES, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "layout_hash", std::move(b));
        }
        if (want(0x2400, 0)) {
            uint64_t n = (prog && retain && retain->attached() &&
                          retain->dirty()) ? (uint64_t)prog->retainSize()
                                           : 0;
            std::vector<uint8_t> b(8);
            memcpy(b.data(), &n, 8);
            add(0x2400, 0, OD_T_U64, OD_RO, tplc::TPLC_ROLE_OBSERVER,
                "retain_dirty_bytes", std::move(b));
        }
        if (want(0x2400, 1)) {
            add(0x2400, 1, OD_T_U8, OD_WO,
                tplc::TPLC_ROLE_COMMISSIONER, "retain_flush", {});
        }
        return rows;
    };

    ctl.on(TPLC_OP_OD_READ, [&](int fd, uint16_t t, const uint8_t* p,
                                uint32_t n, uint64_t req) {
        if (n < 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u16 index, u8 sub}", req);
            return;
        }
        uint16_t idx; uint8_t sub;
        memcpy(&idx, p, 2);
        sub = p[2];
        auto rows = od_rows(idx, sub);
        if (rows.empty()) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_OP,
                           "no such object", req);
            return;
        }
        const OdRow& r = rows[0];
        if (r.access == OD_WO) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_SCHEMA,
                           "write-only object", req);
            return;
        }
        tplc::CtlServer::W w;
        w.u8(r.type);
        w.u8(r.access);
        w.u16((uint16_t)r.bytes.size());
        w.bytes(r.bytes.data(), r.bytes.size());
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    ctl.on(TPLC_OP_OD_LIST, [&](int fd, uint16_t t, const uint8_t*,
                                uint32_t, uint64_t req) {
        auto rows = od_rows(0xFFFF, 0xFF);
        tplc::CtlServer::W w;
        w.u32((uint32_t)rows.size());
        for (const auto& r : rows) {
            w.u16(r.index);
            w.u8(r.sub);
            w.u8(r.access);
            w.u16((uint16_t)r.bytes.size());
            w.str(r.name, (uint32_t)strlen(r.name));
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    ctl.on(TPLC_OP_OD_WRITE, [&](int fd, uint16_t t, const uint8_t* p,
                                 uint32_t n, uint64_t req) {
        if (n < 8) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u16 index, u8 sub, u16 len, data}", req);
            return;
        }
        uint16_t idx, dlen; uint8_t sub;
        memcpy(&idx, p, 2);
        sub = p[2];
        memcpy(&dlen, p + 4, 2);
        if (n < 8 + (uint32_t)dlen) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "payload shorter than len", req);
            return;
        }
        auto rows = od_rows(idx, sub);
        if (rows.empty()) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_OP,
                           "no such object", req);
            return;
        }
        const OdRow& r = rows[0];
        if (r.access == OD_RO) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_SCHEMA,
                           "read-only object", req);
            return;
        }
        if (ctl.roleOf(fd) < r.min_role) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_AUTHZ,
                           "object needs a higher role", req);
            return;
        }
        if (idx == 0x2400 && sub == 1) {
            if (dlen != 1) {
                ctl.errorReply(fd, t, TPLC_E_MGMT_SCHEMA,
                               "retain_flush takes u8", req);
                return;
            }
            if (!retain || !retain->attached() || !prog) {
                ctl.errorReply(fd, t, TPLC_E_CTL_STATE,
                               "no retain store", req);
                return;
            }
            retain->stage(prog->retainRegion());
            tplc::Status fs = retain->flush();
            if (!fs.ok()) {
                ctl.errorReply(fd, t, fs.code, fs.msg.c_str(), req);
                return;
            }
            audit(fd, "od.write", "0x2400/1 retain_flush");
            uint8_t ok = 1;
            ctl.reply(fd, t, &ok, 1, req);
            return;
        }
        ctl.errorReply(fd, t, TPLC_E_MGMT_OP, "no writer", req);
    });

    ctl.on(TPLC_OP_RETAIN_FLUSH, [&](int fd, uint16_t t,
                                     const uint8_t*, uint32_t,
                                     uint64_t req) {
        if (!retain || !retain->attached() || !prog) {
            ctl.errorReply(fd, t, TPLC_E_CTL_STATE,
                           "no retain store", req);
            return;
        }
        retain->stage(prog->retainRegion());
        tplc::Status fs = retain->flush();
        if (fs.ok()) ctl.replyEmpty(fd, t, req);
        else ctl.errorReply(fd, t, fs.code, fs.msg.c_str(), req);
    });
    ctl.on(TPLC_OP_RETAIN_CLEAR, [&](int fd, uint16_t t,
                                     const uint8_t*, uint32_t,
                                     uint64_t req) {
        if (!retain || !retain->attached()) {
            ctl.errorReply(fd, t, TPLC_E_CTL_STATE,
                           "no retain store", req);
            return;
        }
        tplc::Status cs = retain->clear();
        if (cs.ok()) ctl.replyEmpty(fd, t, req);
        else ctl.errorReply(fd, t, cs.code, cs.msg.c_str(), req);
    });
    ctl.on(TPLC_OP_RETAIN_DUMP, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        ctl.reply(fd, t, prog->retainRegion(),
                  (uint32_t)prog->retainSize(), req);
    });

    /* ---------------- debug plane (doc 12) ---------------------- */
    /* every op lands at the ctl drain point between scheduler
       ticks, so reads/writes see a task-boundary-consistent blob   */
    ctl.on(TPLC_OP_TASK_SUSPEND, [&](int fd, uint16_t t,
                                     const uint8_t* p, uint32_t n,
                                     uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n != 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 task_idx}", req);
            return;
        }
        uint32_t idx; memcpy(&idx, p, 4);
        if (!sched.suspend(*prog, idx)) {
            ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                           "no such task", req);
            return;
        }
        audit(fd, "task.suspend", std::to_string(idx));
        ctl.replyEmpty(fd, t, req);
    });
    ctl.on(TPLC_OP_TASK_RESUME, [&](int fd, uint16_t t,
                                    const uint8_t* p, uint32_t n,
                                    uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n != 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 task_idx}", req);
            return;
        }
        uint32_t idx; memcpy(&idx, p, 4);
        if (!sched.resume(*prog, idx)) {
            ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                           "no such task", req);
            return;
        }
        audit(fd, "task.resume", std::to_string(idx));
        ctl.replyEmpty(fd, t, req);
    });
    ctl.on(TPLC_OP_TASK_STEP, [&](int fd, uint16_t t,
                                  const uint8_t* p, uint32_t n,
                                  uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n != 8) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 task_idx, u32 n}", req);
            return;
        }
        uint32_t idx, count;
        memcpy(&idx, p, 4); memcpy(&count, p + 4, 4);
        if (count == 0) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD, "n=0", req);
            return;
        }
        if (idx >= d.task_count) {
            ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                           "no such task", req);
            return;
        }
        if (!sched.step(*prog, idx, count)) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "task not suspended", req);
            return;
        }
        audit(fd, "task.step",
              std::to_string(idx) + " n=" + std::to_string(count));
        ctl.replyEmpty(fd, t, req);
    });

    /* resolve: qualified symbol name → identity + placement
       reply {u8 uuid[16], u32 sym_idx, u8 kind, u8 flags, u8 region,
              u8 access, u32 type_ref, u64 offset, u32 size}         */
    ctl.on(TPLC_OP_DBG_RESOLVE, [&](int fd, uint16_t t,
                                    const uint8_t* p, uint32_t n,
                                    uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n < 2) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u16 nlen, name}", req);
            return;
        }
        uint16_t nl; memcpy(&nl, p, 2);
        if ((uint32_t)nl + 2 > n) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "name overruns payload", req);
            return;
        }
        /* doc 19 §3.2: SHADOW.<sym> resolves through the candidate
           program's symmap; the reply uuid identifies the program  */
        std::string_view want((const char*)p + 2, nl);
        bool to_shadow = want.size() > 7 &&
                         want.substr(0, 7) == "SHADOW.";
        if (to_shadow && (!shadow || !shadow->prog)) {
            ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, "no shadow",
                           req);
            return;
        }
        const std::vector<uint8_t>& rsmap =
            to_shadow ? shadow->smap : pkg_smap;
        const uint8_t* rsm = rsmap.data();
        if (to_shadow) want.remove_prefix(7);
        if (rsmap.size() < 32 || memcmp(rsm, "SMAP", 4) != 0) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "no symbol map", req);
            return;
        }
        uint32_t count, stroff, strln;
        memcpy(&count, rsm + 8, 4);
        memcpy(&stroff, rsm + 12, 4);
        memcpy(&strln, rsm + 16, 4);
        if ((uint64_t)stroff + strln > rsmap.size() ||
            32ull + (uint64_t)count * 64 > rsmap.size()) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "corrupt symbol map", req);
            return;
        }
        const uint8_t* strs = rsm + stroff;
        for (uint32_t i = 0; i < count; i++) {
            const uint8_t* e = rsm + 32 + (size_t)i * 64;
            uint32_t noff; memcpy(&noff, e, 4);
            if (noff >= strln) continue;
            const char* nm = (const char*)strs + noff;
            if (!memchr(nm, 0, strln - noff)) continue;
            if (want != nm) continue;
            tplc::CtlServer::W w;
            if (to_shadow)
                w.bytes(shadow->prog->desc().program_uuid, 16);
            else
                w.bytes(d.program_uuid, 16);
            w.u32(i);
            w.bytes(e + 4, 4);   /* kind flags region access */
            w.bytes(e + 8, 4);   /* type_ref  */
            w.bytes(e + 12, 8);  /* offset    */
            w.bytes(e + 20, 4);  /* size      */
            ctl.reply(fd, t, w.data(), w.size(), req);
            return;
        }
        /* a miss that the last swap's predecessor symmap still holds
           is stale (-411), not merely unknown (-410)               */
        if (!to_shadow && sdiff.ready) {
            uint64_t oo; uint32_t ol;
            if (smap_find_in(sdiff.old_smap, want, &oo, &ol)) {
                ctl.errorReply(fd, t, TPLC_E_DBG_STALE,
                               "symbol removed by online change", req);
                return;
            }
        }
        ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                       "symbol not found", req);
    });

    ctl.on(TPLC_OP_DBG_READ, [&](int fd, uint16_t t,
                                 const uint8_t* p, uint32_t n,
                                 uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        /* doc 19 §3.2: 13th byte selects the program — 0/default
           active, 1 = shadow blob (read-only through this path)   */
        if (n != 12 && n != 13) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u64 off, u32 len[, u8 sel]}", req);
            return;
        }
        uint64_t off; uint32_t rl;
        memcpy(&off, p, 8); memcpy(&rl, p + 8, 4);
        tplc::Program* tgt = prog ? &*prog : nullptr;
        if (n == 13 && p[12] == 1) {
            if (!shadow || !shadow->prog) {
                ctl.errorReply(fd, t, TPLC_E_MGMT_STATE,
                               "no shadow", req);
                return;
            }
            tgt = &*shadow->prog;
        } else if (n == 13 && p[12] != 0) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "sel must be 0 or 1", req);
            return;
        }
        if (off > tgt->blobSize() || rl > tgt->blobSize() - off) {
            ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                           "range outside blob", req);
            return;
        }
        ctl.reply(fd, t, tgt->blob() + off, rl, req);
    });

    /* read-only ranges: LAYD field flags bit1 marks VAR_CONSTANT
       fields; writes and forces intersecting one are denied. A
       program without layout descriptors has no annotation, so
       nothing is ro by default                                    */
    auto layd_ro_range = [&](uint64_t off, uint64_t len) {
        const uint8_t* b = pkg_layd.data();
        size_t n = pkg_layd.size();
        auto r32 = [](const uint8_t* p) {
            uint32_t v; memcpy(&v, p, 4); return v; };
        auto r64 = [](const uint8_t* p) {
            uint64_t v; memcpy(&v, p, 8); return v; };
        if (!b || n < 24 || memcmp(b, "LAYD", 4)) return false;
        uint32_t nreg = r32(b + 8);
        size_t at = 24 + (size_t)nreg * 24;
        if (at + 4 > n) return false;
        uint32_t nf = r32(b + at); at += 4;
        if (at + (uint64_t)nf * 32 > n) return false;
        for (uint32_t i = 0; i < nf; i++) {
            const uint8_t* f = b + at + (size_t)i * 32;
            uint64_t fo = r64(f + 8), fs = r64(f + 16);
            if ((f[5] & 2) && off < fo + fs && off + len > fo)
                return true;
        }
        return false;
    };

    /* safety boundary (doc 15 §2): channels whose endpoint key is
       listed in a `safety` line of the package's DEPLOY_MAP section
       (bridge-format deploy map) are read-only for user logic. A
       force/write whose blob range backs a binding intersecting a
       safety channel's image range is denied at the resolver.
       Parsed lazily — the section swaps with the program.        */
    auto safety_denied = [&](uint64_t off, uint64_t len) {
        if (pkg_iomap.empty() || !prog) return false;
        std::unordered_map<std::string, bool> skeys;
        struct Ch { bool in; std::string key; uint64_t off, size; };
        std::vector<Ch> chs;
        std::istringstream is(
            std::string(pkg_iomap.begin(), pkg_iomap.end()));
        std::string line;
        while (std::getline(is, line)) {
            std::istringstream ls(line);
            std::string t0;
            if (!(ls >> t0) || t0[0] == '#') continue;
            if (t0 == "safety") {
                std::string k;
                if (ls >> k) skeys[k] = true;
                continue;
            }
            if (t0 != "in" && t0 != "out") continue;
            Ch c{t0 == "in", "", 0, 0};
            std::string tk;
            while (ls >> tk) {
                auto eq = tk.find('=');
                if (eq == std::string::npos) continue;
                std::string k = tk.substr(0, eq),
                            v = tk.substr(eq + 1);
                if      (k == "key")  c.key = v;
                else if (k == "off")  c.off = strtoull(v.c_str(), 0, 0);
                else if (k == "size") c.size = strtoull(v.c_str(), 0, 0);
            }
            chs.push_back(c);
        }
        const TplcProgramDescriptor& dd = prog->desc();
        for (uint32_t i = 0; i < dd.io_binding_count; i++) {
            const TplcIoBinding& b = dd.io_bindings[i];
            bool in = b.region == TPLC_REGION_IMAGE_IN;
            if (!in && b.region != TPLC_REGION_IMAGE_OUT) continue;
            for (auto& c : chs) {
                if (c.in != in || !c.size || !skeys.count(c.key))
                    continue;
                uint32_t bs = b.size ? b.size : 1;
                if ((uint64_t)b.image_offset < c.off + c.size &&
                    (uint64_t)b.image_offset + bs > c.off &&
                    off < (uint64_t)b.blob_offset + bs &&
                    off + len > b.blob_offset)
                    return true;
            }
        }
        return false;
    };

    ctl.on(TPLC_OP_DBG_WRITE, [&](int fd, uint16_t t,
                                  const uint8_t* p, uint32_t n,
                                  uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n < 12) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u64 off, u32 len, bytes}", req);
            return;
        }
        uint64_t off; uint32_t wl;
        memcpy(&off, p, 8); memcpy(&wl, p + 8, 4);
        if ((uint64_t)wl + 12 != n) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "len/byte count mismatch", req);
            return;
        }
        if (off > prog->blobSize() || wl > prog->blobSize() - off) {
            ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                           "range outside blob", req);
            return;
        }
        if (layd_ro_range(off, wl)) {
            ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                           "range intersects a read-only field", req);
            return;
        }
        if (safety_denied(off, wl)) {
            ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                           "safety channel is read-only", req);
            return;
        }
        memcpy(prog->blob() + off, p + 12, wl);
        /* writes into the retain region must reach the store */
        if (retain && retain->attached() && prog->retainSize() > 0) {
            uint64_t roff =
                (uint64_t)(prog->retainRegion() - prog->blob());
            uint64_t rlen = prog->retainSize();
            if (off < roff + rlen && off + wl > roff)
                retain->noteDirty();
        }
        audit(fd, "dbg.write", "off=" + std::to_string(off) +
              " len=" + std::to_string(wl));
        ctl.replyEmpty(fd, t, req);
    });

    /* reply {u8 uuid[16], u64 blob_size, u64 retain_off,
              u64 retain_len, u32 sym_count}                        */
    ctl.on(TPLC_OP_DBG_STATE, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        uint64_t roff = 0;
        if (prog->retainSize() > 0)
            roff = (uint64_t)(prog->retainRegion() - prog->blob());
        uint32_t scount = 0;
        if (pkg_smap.size() >= 32 &&
            memcmp(pkg_smap.data(), "SMAP", 4) == 0)
            memcpy(&scount, pkg_smap.data() + 8, 4);
        tplc::CtlServer::W w;
        w.bytes(d.program_uuid, 16);
        w.u64(prog->blobSize());
        w.u64(roff);
        w.u64(prog->retainSize());
        w.u32(scount);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    /* force (doc 12 §6): {u64 off, u32 len, bytes} → {u32 handle}.
       the override re-stamps every boundary until cleared — a
       one-shot DBG_WRITE can be overwritten by the program, a
       force cannot. overlaps are denied, not merged               */
    ctl.on(TPLC_OP_DBG_FORCE, [&](int fd, uint16_t t,
                                  const uint8_t* p, uint32_t n,
                                  uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n < 12) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u64 off, u32 len, bytes}", req);
            return;
        }
        uint64_t off; uint32_t wl;
        memcpy(&off, p, 8); memcpy(&wl, p + 8, 4);
        if (wl == 0 ||
            (n != 12 + (uint64_t)wl && n != 12 + 2 * (uint64_t)wl)) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "len/byte count mismatch (mask optional)",
                           req);
            return;
        }
        if (off > prog->blobSize() || wl > prog->blobSize() - off) {
            ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                           "range outside blob", req);
            return;
        }
        if (forces.size() >= MAX_FORCES) {
            ctl.errorReply(fd, t, TPLC_E_DBG_OVERLOAD,
                           "force table full", req);
            return;
        }
        for (auto& f : forces)
            if (off < f.off + f.val.size() && off + wl > f.off) {
                ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                               "overlaps existing force", req);
                return;
            }
        /* a range intersecting a non-forceable io binding is denied;
           MARKER bindings have no image copy to translate into      */
        const TplcProgramDescriptor& dd = prog->desc();
        for (uint32_t i = 0; i < dd.io_binding_count; i++) {
            const TplcIoBinding& b = dd.io_bindings[i];
            uint32_t bs = b.size ? b.size : 1;
            if (b.region == TPLC_REGION_MARKER &&
                off < (uint64_t)b.blob_offset + bs &&
                off + wl > b.blob_offset) {
                ctl.errorReply(fd, t, TPLC_E_FORCE_DENIED,
                               "marker bindings are not forceable",
                               req);
                return;
            }
            if (b.region != TPLC_REGION_MARKER &&
                off < (uint64_t)b.blob_offset + bs &&
                off + wl > b.blob_offset &&
                !(b.flags & TPLC_IOB_FORCEABLE)) {
                ctl.errorReply(fd, t, TPLC_E_FORCE_DENIED,
                               "binding not forceable", req);
                return;
            }
        }
        if (layd_ro_range(off, wl)) {
            ctl.errorReply(fd, t, TPLC_E_FORCE_DENIED,
                           "constant fields are not forceable", req);
            return;
        }
        if (safety_denied(off, wl)) {
            ctl.errorReply(fd, t, TPLC_E_FORCE_DENIED,
                           "safety channel is read-only", req);
            return;
        }
        bool masked = n == 12 + 2 * (uint64_t)wl;
        forces.push_back({next_force_h++, off,
                          std::vector<uint8_t>(p + 12, p + 12 + wl),
                          masked
                              ? std::vector<uint8_t>(p + 12 + wl,
                                                     p + 12 + 2 * wl)
                              : std::vector<uint8_t>(wl, 0xFF)});
        stamp_forces(0);   /* effective even while stopped */
        audit(fd, "dbg.force", "h=" +
              std::to_string(forces.back().handle) +
              " off=" + std::to_string(off) +
              " len=" + std::to_string(wl) +
              (masked ? " masked" : ""));
        tplc::CtlServer::W w;
        w.u32(forces.back().handle);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    /* {u32 n, {u32 handle, u64 off, u32 len, u8 val[len],
                u8 mask[len]}*}                                    */
    ctl.on(TPLC_OP_DBG_FORCE_LIST, [&](int fd, uint16_t t,
                                       const uint8_t*, uint32_t,
                                       uint64_t req) {
        tplc::CtlServer::W w;
        w.u32((uint32_t)forces.size());
        for (auto& f : forces) {
            w.u32(f.handle);
            w.u64(f.off);
            w.u32((uint32_t)f.val.size());
            w.bytes(f.val.data(), f.val.size());
            w.bytes(f.mask.data(), f.mask.size());
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_DBG_FORCE_CLEAR, [&](int fd, uint16_t t,
                                        const uint8_t* p, uint32_t n,
                                        uint64_t req) {
        if (n != 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 handle}", req);
            return;
        }
        uint32_t h; memcpy(&h, p, 4);
        if (h == 0xFFFFFFFFu) {
            forces.clear();
            audit(fd, "dbg.force.clear", "all");
            ctl.replyEmpty(fd, t, req);
            return;
        }
        for (auto it = forces.begin(); it != forces.end(); ++it)
            if (it->handle == h) {
                forces.erase(it);
                audit(fd, "dbg.force.clear", std::to_string(h));
                ctl.replyEmpty(fd, t, req);
                return;
            }
        ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                       "no such force", req);
    });

    /* break_when (doc 12 §5): arm a conditional suspend; capture_on
       shares the payload but freezes the flight recorder instead.
       {u32 task_idx, u64 off, u32 len, u8 cmp, u8 pad[3], u64 val}
       → {u32 handle}                                              */
    auto arm_break = [&](int fd, uint16_t t,
                         const uint8_t* p, uint32_t n,
                         uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n != 28) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 task, u64 off, u32 len, u8 cmp, "
                           "u64 val}", req);
            return;
        }
        uint32_t task_idx, len; uint64_t off, val; uint8_t cmp;
        memcpy(&task_idx, p, 4);
        memcpy(&off, p + 4, 8);
        memcpy(&len, p + 12, 4);
        cmp = p[16];
        memcpy(&val, p + 20, 8);
        if (task_idx >= d.task_count) {
            ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                           "no such task", req);
            return;
        }
        if (len != 1 && len != 2 && len != 4 && len != 8) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "len must be 1,2,4,8", req);
            return;
        }
        if (cmp > 5) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "cmp out of range", req);
            return;
        }
        if (off > prog->blobSize() || len > prog->blobSize() - off) {
            ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                           "range outside blob", req);
            return;
        }
        if (breaks.size() >= MAX_BREAKS) {
            ctl.errorReply(fd, t, TPLC_E_DBG_OVERLOAD,
                           "break table full", req);
            return;
        }
        breaks.push_back({next_break_h++, task_idx, off, len,
                          cmp, val, 0,
                          t == TPLC_OP_DBG_CAPTURE, false,
                          std::nullopt, {}});
        audit(fd, t == TPLC_OP_DBG_CAPTURE ? "dbg.capture"
                                           : "dbg.break",
              "h=" + std::to_string(breaks.back().handle) +
              " task=" + std::to_string(task_idx) +
              " off=" + std::to_string(off) +
              " len=" + std::to_string(len) +
              " cmp=" + std::to_string(cmp) +
              " val=" + std::to_string(val));
        tplc::CtlServer::W w;
        w.u32(breaks.back().handle);
        ctl.reply(fd, t, w.data(), w.size(), req);
    };
    ctl.on(TPLC_OP_DBG_BREAK, arm_break);
    ctl.on(TPLC_OP_DBG_CAPTURE, arm_break);
    /* expression triggers (doc 12 §M): {u32 task_idx, u16 elen,
       char expr[]} → {u32 handle}. Compile failures carry the
       offending term in the error detail                          */
    auto arm_break_expr = [&](int fd, uint16_t t,
                              const uint8_t* p, uint32_t n,
                              uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n < 6) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 task_idx, u16 elen, char expr[]}",
                           req);
            return;
        }
        uint32_t task_idx; uint16_t el;
        memcpy(&task_idx, p, 4);
        memcpy(&el, p + 4, 2);
        if ((uint32_t)el + 6 != n || el == 0 || el > 256) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "expr len 1..256", req);
            return;
        }
        if (task_idx >= d.task_count) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "no such task", req);
            return;
        }
        if (breaks.size() >= MAX_BREAKS) {
            ctl.errorReply(fd, t, TPLC_E_DBG_OVERLOAD,
                           "break table full", req);
            return;
        }
        Expr X;
        std::string err = expr_compile(
            std::string_view((const char*)p + 6, el), X);
        if (!err.empty()) {
            ctl.errorReply(fd, t,
                           err.rfind("unknown", 0) == 0
                               ? TPLC_E_DBG_RESOLVE
                               : TPLC_E_CTL_PAYLOAD,
                           err.c_str(), req);
            return;
        }
        /* seed slot history at arm time so changed/rising/falling
           start from the current value, not zero                  */
        expr_seed(X);
        Break b{next_break_h++, task_idx, 0, 0, 0xFF, 0, 0,
                t == TPLC_OP_DBG_CAPTURE_EXPR, false, std::move(X),
                std::string((const char*)p + 6, el)};
        breaks.push_back(std::move(b));
        audit(fd, t == TPLC_OP_DBG_CAPTURE_EXPR ? "dbg.capture.expr"
                                                : "dbg.break.expr",
              "h=" + std::to_string(breaks.back().handle) +
              " task=" + std::to_string(task_idx) + " \"" +
              std::string((const char*)p + 6, el) + "\"");
        tplc::CtlServer::W w;
        w.u32(breaks.back().handle);
        ctl.reply(fd, t, w.data(), w.size(), req);
    };
    ctl.on(TPLC_OP_DBG_BREAK_EXPR, arm_break_expr);
    ctl.on(TPLC_OP_DBG_CAPTURE_EXPR, arm_break_expr);
    /* state.diff: report the last committed swap's migrate rules —
       MIGD rows resolved to names/offsets via the embedded old LAYD
       and the live LAYD, values sampled from the frozen snapshot
       and the running blob                                       */
    ctl.on(TPLC_OP_DBG_STATE_DIFF,
           [&](int fd, uint16_t t, const uint8_t*, uint32_t,
               uint64_t req) {
        if (!sdiff.ready || !prog) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "no committed swap", req);
            return;
        }
        auto r16 = [](const uint8_t* p) { uint16_t v;
            memcpy(&v, p, 2); return v; };
        auto r32 = [](const uint8_t* p) { uint32_t v;
            memcpy(&v, p, 4); return v; };
        auto r64 = [](const uint8_t* p) { uint64_t v;
            memcpy(&v, p, 8); return v; };
        /* LAYD field i → {sym_entry, off, len} */
        auto layd_field = [&](const uint8_t* b, size_t n, uint32_t i,
                              uint32_t* sym, uint64_t* off,
                              uint64_t* len) -> bool {
            if (!b || n < 24 || memcmp(b, "LAYD", 4)) return false;
            uint32_t nreg = r32(b + 8);
            size_t at = 24 + (size_t)nreg * 24;
            if (at + 4 > n) return false;
            uint32_t nf = r32(b + at); at += 4;
            if (i >= nf || at + (uint64_t)i * 32 + 32 > n) return false;
            const uint8_t* f = b + at + (size_t)i * 32;
            *sym = r32(f); *off = r64(f + 8); *len = r64(f + 16);
            return true;
        };
        auto smap_name = [&](const uint8_t* sm, size_t n,
                             uint32_t idx) -> std::string {
            if (!sm || n < 32 || memcmp(sm, "SMAP", 4)) return "";
            uint32_t cnt = r32(sm + 8), so = r32(sm + 12),
                     sl = r32(sm + 16);
            if ((uint64_t)so + sl > n ||
                32ull + (uint64_t)cnt * 64 > n || idx >= cnt)
                return "";
            uint32_t noff = r32(sm + 32 + (size_t)idx * 64);
            if (noff >= sl) return "";
            const char* nm = (const char*)sm + so + noff;
            size_t mx = sl - noff;
            const void* z = memchr(nm, 0, mx);
            return std::string(nm, z ? (const char*)z - nm : mx);
        };
        /* walk the MIGD: [32 hash][u16 vlen][ver][pad4]
           [u32 nrules][16B rules][predecessor LAYD]              */
        const uint8_t* m = sdiff.migd.data();
        size_t mn = sdiff.migd.size();
        struct R { uint8_t kind, sev; uint16_t tid;
                   uint32_t of, nf, aux; };
        std::vector<R> rules;
        const uint8_t* olayd = nullptr;
        size_t oln = 0;
        if (mn >= 34) {
            size_t at = 34 + r16(m + 32);
            at += (4 - (at % 4)) % 4;
            if (at + 4 <= mn) {
                uint32_t nr = r32(m + at); at += 4;
                for (uint32_t i = 0; i < nr && at + 16 <= mn; i++) {
                    rules.push_back({m[at], m[at + 1],
                                     r16(m + at + 2), r32(m + at + 4),
                                     r32(m + at + 8), r32(m + at + 12)});
                    at += 16;
                }
                if (at < mn) { olayd = m + at; oln = mn - at; }
            }
        }
        tplc::CtlServer::W w;
        w.u8(sdiff.same_layout ? 1 : 0);
        w.u8((uint8_t)sdiff.mig_status);
        w.u16(0);
        w.u32((uint32_t)rules.size());
        for (const R& r : rules) {
            uint32_t os = 0xFFFFFFFFu, ns = 0xFFFFFFFFu;
            uint64_t oo = 0, no = 0, ol = 0, nl = 0;
            if (r.of != 0xFFFFFFFFu)
                layd_field(olayd, oln, r.of, &os, &oo, &ol);
            if (r.nf != 0xFFFFFFFFu)
                layd_field(pkg_layd.data(), pkg_layd.size(),
                           r.nf, &ns, &no, &nl);
            w.u8(r.kind); w.u8(r.sev); w.u16(r.tid); w.u32(r.aux);
            w.u64(oo); w.u32((uint32_t)ol); w.u32(os);
            w.u64(no); w.u32((uint32_t)nl); w.u32(ns);
            uint8_t ovl = 0, nvl = 0;
            if (oo + ol <= sdiff.old_blob.size())
                ovl = (uint8_t)(ol < 8 ? ol : 8);
            if (no + nl <= prog->blobSize())
                nvl = (uint8_t)(nl < 8 ? nl : 8);
            w.u8(ovl); w.u8(nvl); w.u16(0);
            uint8_t pv[8] = {0};
            if (ovl) memcpy(pv, sdiff.old_blob.data() + oo, ovl);
            w.bytes(pv, 8);
            memset(pv, 0, 8);
            if (nvl) memcpy(pv, prog->blob() + no, nvl);
            w.bytes(pv, 8);
            std::string on = os != 0xFFFFFFFFu
                ? smap_name(sdiff.old_smap.data(),
                            sdiff.old_smap.size(), os) : "";
            std::string nn = ns != 0xFFFFFFFFu
                ? smap_name(pkg_smap.data(), pkg_smap.size(), ns) : "";
            w.str(on.data(), (uint32_t)on.size());
            w.str(nn.data(), (uint32_t)nn.size());
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    /* blob layout browser (doc 12 §7): per-field rows from LAYD +
       SMAP so a client can enumerate the loaded program's fields.
       Optional {u8 sel} payload (doc 19 §3.2): sel=1 enumerates the
       shadow candidate's fields                                   */
    ctl.on(TPLC_OP_DBG_FIELDS, [&](int fd, uint16_t t,
                                   const uint8_t* p, uint32_t pn,
                                   uint64_t req) {
        const std::vector<uint8_t>* layd_p = &pkg_layd;
        const std::vector<uint8_t>* smap_p = &pkg_smap;
        if (pn == 1 && p[0] == 1) {
            if (!shadow || !shadow->prog) {
                ctl.errorReply(fd, t, TPLC_E_MGMT_STATE, "no shadow",
                               req);
                return;
            }
            layd_p = &shadow->layd;
            smap_p = &shadow->smap;
        } else if (pn != 0) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u8 sel}", req);
            return;
        }
        const uint8_t* b = layd_p->data();
        size_t n = layd_p->size();
        auto r32 = [](const uint8_t* p) {
            uint32_t v; memcpy(&v, p, 4); return v; };
        auto r64 = [](const uint8_t* p) {
            uint64_t v; memcpy(&v, p, 8); return v; };
        if (!b || n < 24 || memcmp(b, "LAYD", 4)) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "no layout descriptors", req);
            return;
        }
        uint32_t nreg = r32(b + 8);
        size_t at = 24 + (size_t)nreg * 24;
        if (at + 4 > n) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "corrupt layout", req);
            return;
        }
        uint32_t nf = r32(b + at); at += 4;
        if (at + (uint64_t)nf * 32 > n) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "corrupt layout", req);
            return;
        }
        /* SMAP rows carry name + region + access per sym_entry   */
        const uint8_t* sm = smap_p->data();
        bool have_sm = sm && smap_p->size() >= 32 &&
                       !memcmp(sm, "SMAP", 4);
        uint32_t scnt = have_sm ? r32(sm + 8) : 0;
        uint32_t so = have_sm ? r32(sm + 12) : 0;
        uint32_t sl = have_sm ? r32(sm + 16) : 0;
        if (have_sm && ((uint64_t)so + sl > smap_p->size() ||
                        32ull + (uint64_t)scnt * 64 >
                            smap_p->size()))
            have_sm = false;
        tplc::CtlServer::W w;
        w.u32(nf);
        for (uint32_t i = 0; i < nf; i++) {
            const uint8_t* f = b + at + (size_t)i * 32;
            uint32_t sym = r32(f);
            uint64_t off = r64(f + 8), sz = r64(f + 16);
            w.u64(off); w.u64(sz); w.u32(r32(f + 24));
            uint8_t region = 0, access = 0;
            std::string nm;
            if (have_sm && sym < scnt) {
                const uint8_t* e = sm + 32 + (size_t)sym * 64;
                region = e[6]; access = e[7];
                uint32_t noff = r32(e);
                if (noff < sl) {
                    const char* p = (const char*)sm + so + noff;
                    size_t mx = sl - noff;
                    const void* z = memchr(p, 0, mx);
                    nm.assign(p, z ? (const char*)z - p : mx);
                }
            }
            w.u8(f[4]); w.u8(f[5]); w.u8(region); w.u8(access);
            w.u8((uint8_t)(nm.size() > 255 ? 255 : nm.size()));
            w.bytes(nm.data(), nm.size() > 255 ? 255 : nm.size());
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    /* {u32 n, {u32 handle, u32 task, u64 off, u32 len, u8 cmp,
                u8 pad[3], u64 val, u64 hits}*}                    */
    ctl.on(TPLC_OP_DBG_BREAK_LIST, [&](int fd, uint16_t t,
                                       const uint8_t*, uint32_t,
                                       uint64_t req) {
        tplc::CtlServer::W w;
        w.u32((uint32_t)breaks.size());
        for (auto& b : breaks) {
            w.u32(b.handle); w.u32(b.task_idx); w.u64(b.off);
            w.u32(b.len); w.u8(b.cmp); w.u8(b.capture ? 1 : 0);
            w.u8(b.stale ? 1 : 0); w.u8(0);
            w.u64(b.val); w.u64(b.hits);
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_DBG_BREAK_CLEAR, [&](int fd, uint16_t t,
                                        const uint8_t* p, uint32_t n,
                                        uint64_t req) {
        if (n != 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 handle}", req);
            return;
        }
        uint32_t h; memcpy(&h, p, 4);
        if (h == 0xFFFFFFFFu) {
            breaks.clear();
            audit(fd, "dbg.break.clear", "all");
            ctl.replyEmpty(fd, t, req);
            return;
        }
        for (auto it = breaks.begin(); it != breaks.end(); ++it)
            if (it->handle == h) {
                breaks.erase(it);
                audit(fd, "dbg.break.clear", std::to_string(h));
                ctl.replyEmpty(fd, t, req);
                return;
            }
        ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                       "no such break", req);
    });

    /* flight recorder (doc 12 §7) — see enum block for the wire
       shapes. CONF validates all channels before arming; a bad set
       leaves the previous config untouched.                        */
    ctl.on(TPLC_OP_TRACE_CONF, [&](int fd, uint16_t t,
                                   const uint8_t* p, uint32_t n,
                                   uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n < 8 || (n - 8) % 16 != 0) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 n_ch, u32 pad, {u64 off, u32 len, "
                           "u32 pad}*}", req);
            return;
        }
        uint32_t nch, flags;
        memcpy(&nch, p, 4);
        memcpy(&flags, p + 4, 4);
        if (nch > MAX_TRACE_CH || n != 8 + nch * 16) {
            ctl.errorReply(fd, t, TPLC_E_TRACE_CONFIG,
                           "channel count/frame shape", req);
            return;
        }
        std::vector<TraceCh> chs;
        uint32_t total = 0;
        for (uint32_t i = 0; i < nch; i++) {
            uint64_t off; uint32_t len;
            memcpy(&off, p + 8 + i * 16, 8);
            memcpy(&len, p + 8 + i * 16 + 8, 4);
            if (len == 0 || len > 8 ||
                off > prog->blobSize() ||
                len > prog->blobSize() - off) {
                ctl.errorReply(fd, t, TPLC_E_TRACE_CONFIG,
                               "channel range outside blob", req);
                return;
            }
            total += len;
            if (total > 64) {
                ctl.errorReply(fd, t, TPLC_E_TRACE_CONFIG,
                               "frame > 64B", req);
                return;
            }
            chs.push_back({off, len});
        }
        trace_ch = std::move(chs);
        trace_ring.clear();
        trace_wr = 0;
        trace_flags = flags;
        trace_frozen = false;
        audit(fd, "trace.conf", "ch=" + std::to_string(nch) +
              " flags=" + std::to_string(flags));
        tplc::CtlServer::W w;
        w.u32(total);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_TRACE_STATUS, [&](int fd, uint16_t t,
                                     const uint8_t*, uint32_t,
                                     uint64_t req) {
        uint32_t fl = 0;
        for (auto& c : trace_ch) fl += c.len;
        tplc::CtlServer::W w;
        w.u32((uint32_t)trace_ch.size());
        w.u32(fl);
        w.u64(trace_wr);
        w.u64((uint64_t)trace_ring.size());
        w.u32(trace_frozen ? 1u : 0u);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_TRACE_READ, [&](int fd, uint16_t t,
                                   const uint8_t* p, uint32_t n,
                                   uint64_t req) {
        if (n != 12) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u64 from_seq, u32 max}", req);
            return;
        }
        if (trace_ring.empty() && trace_ch.empty()) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "no trace armed", req);
            return;
        }
        uint64_t from; uint32_t mx;
        memcpy(&from, p, 8);
        memcpy(&mx, p + 8, 4);
        uint64_t oldest = trace_wr - trace_ring.size();
        uint64_t first = from > oldest ? from : oldest;
        if (first > trace_wr) first = trace_wr;
        uint64_t avail = trace_wr - first;
        uint32_t n_out = (uint32_t)(avail < (uint64_t)mx ? avail : mx);
        if (n_out > 512) n_out = 512;
        uint32_t fl = 0;
        for (auto& c : trace_ch) fl += c.len;
        tplc::CtlServer::W w;
        w.u64(first);
        w.u32(n_out);
        w.u32(fl);
        for (uint32_t i = 0; i < n_out; i++) {
            const TraceFrame& f =
                trace_ring[(first + i) % TRACE_CAP];
            w.u64(f.seq);
            w.u64(f.ts);
            w.bytes(f.data, fl);
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_TRACE_CLEAR, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t n,
                                    uint64_t req) {
        if (n != 0) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD, "{}", req);
            return;
        }
        trace_ch.clear();
        trace_ring.clear();
        trace_wr = 0;
        trace_flags = 0;
        trace_frozen = false;
        audit(fd, "trace.clear", "");
        ctl.replyEmpty(fd, t, req);
    });

    /* watch (doc 12 §3): ADD shares the TRACE_CONF channel list;
       GET returns the boundary-consistent bytes right now; STREAM
       arms a ≤50 Hz EV_DBG_WATCH push sampled at task boundaries  */
    ctl.on(TPLC_OP_DBG_WATCH_ADD, [&](int fd, uint16_t t,
                                      const uint8_t* p, uint32_t n,
                                      uint64_t req) {
        if (!prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "no program", req);
            return;
        }
        if (n < 8) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 n, {u64 off, u32 len, u16 nlen,"
                           " char name[nlen]}*}",
                           req);
            return;
        }
        uint32_t ni; memcpy(&ni, p, 4);
        if (ni == 0 || ni > MAX_WATCH_ITEMS) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "item count/shape", req);
            return;
        }
        /* items {u64 off, u32 len, u16 nlen, char name[nlen]}: a
           named item resolves its range through the symmap and the
           path is kept for post-swap re-resolution                */
        std::vector<WatchItem> chs;
        uint32_t total = 0;
        size_t at = 8;
        for (uint32_t i = 0; i < ni; i++) {
            if (at + 14 > n) {
                ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                               "item truncated", req);
                return;
            }
            uint64_t off; uint32_t l; uint16_t nl2;
            memcpy(&off, p + at, 8);
            memcpy(&l, p + at + 8, 4);
            memcpy(&nl2, p + at + 12, 2);
            if (nl2 > 127 || at + 14 + nl2 > n) {
                ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                               "item name truncated", req);
                return;
            }
            std::string path((const char*)p + at + 14, nl2);
            at += 14 + nl2;
            /* doc 19 §3.2: SHADOW.<sym> items resolve through the
               candidate's symmap and sample its blob               */
            bool item_shadow = path.size() > 7 &&
                               path.compare(0, 7, "SHADOW.") == 0;
            if (nl2) {
                bool found;
                if (item_shadow) {
                    if (!shadow || !shadow->prog) {
                        ctl.errorReply(fd, t, TPLC_E_MGMT_STATE,
                                       "no shadow", req);
                        return;
                    }
                    found = smap_find_in(shadow->smap,
                                         std::string_view(path)
                                             .substr(7),
                                         &off, &l);
                } else {
                    found = smap_find(path, &off, &l);
                }
                if (!found) {
                    if (!item_shadow && sdiff.ready &&
                        smap_find_in(sdiff.old_smap, path, &off, &l)) {
                        ctl.errorReply(fd, t, TPLC_E_DBG_STALE,
                                       ("path removed by online "
                                        "change " + path).c_str(),
                                       req);
                    } else {
                        ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                                       ("unknown path " + path)
                                           .c_str(), req);
                    }
                    return;
                }
                if (l > 64) {
                    ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                                   "symbol too large", req);
                    return;
                }
            } else if (l == 0 || l > 64 ||
                       off > prog->blobSize() ||
                       l > prog->blobSize() - off) {
                ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                               "item outside blob", req);
                return;
            }
            total += l;
            if (total > MAX_WATCH_LEN) {
                ctl.errorReply(fd, t, TPLC_E_DBG_OVERLOAD,
                               "watch payload > 512 B", req);
                return;
            }
            chs.push_back({off, l, std::move(path), false,
                           item_shadow});
        }
        if (at != n) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "trailing bytes", req);
            return;
        }
        if (watches.size() >= MAX_WATCHES) {
            ctl.errorReply(fd, t, TPLC_E_DBG_OVERLOAD,
                           "watch table full", req);
            return;
        }
        watches.push_back({next_watch_h++, std::move(chs), 0, 0, 0});
        audit(fd, "dbg.watch.add",
              "h=" + std::to_string(watches.back().handle) +
              " items=" + std::to_string(ni));
        tplc::CtlServer::W w;
        w.u32(watches.back().handle);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    /* {u32 handle} → {u32 n, {u32 len, u8 data[len]}*} */
    ctl.on(TPLC_OP_DBG_WATCH_GET, [&](int fd, uint16_t t,
                                      const uint8_t* p, uint32_t n,
                                      uint64_t req) {
        if (n != 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 handle}", req);
            return;
        }
        uint32_t h; memcpy(&h, p, 4);
        for (auto& wt : watches)
            if (wt.handle == h) {
                tplc::CtlServer::W w;
                w.u32((uint32_t)wt.chs.size());
                for (auto& c : wt.chs) {
                    tplc::Program* src =
                        c.shadow ? (shadow && shadow->prog
                                    ? &*shadow->prog : nullptr)
                                 : (prog ? &*prog : nullptr);
                    bool stale = c.stale || !src ||
                        c.off > src->blobSize() ||
                        c.len > src->blobSize() - c.off;
                    w.u32(stale ? 0 : c.len);
                    if (!stale)
                        w.bytes(src->blob() + c.off, c.len);
                }
                ctl.reply(fd, t, w.data(), w.size(), req);
                return;
            }
        ctl.errorReply(fd, t, TPLC_E_DBG_STATE, "no such watch", req);
    });
    /* {u32 handle, u32 rate_hz} — rate 1..50 arm, 0 disarm */
    ctl.on(TPLC_OP_DBG_WATCH_STREAM, [&](int fd, uint16_t t,
                                         const uint8_t* p, uint32_t n,
                                         uint64_t req) {
        if (n != 8) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 handle, u32 rate_hz}", req);
            return;
        }
        uint32_t h, hz;
        memcpy(&h, p, 4); memcpy(&hz, p + 4, 4);
        if (hz > 50) {
            ctl.errorReply(fd, t, TPLC_E_DBG_RATE,
                           "rate > 50 Hz", req);
            return;
        }
        uint32_t armed = 0;
        for (auto& wt : watches)
            if (wt.rate_hz) armed++;
        for (auto& wt : watches)
            if (wt.handle == h) {
                if (hz && !wt.rate_hz && armed >= MAX_WATCH_STREAMS) {
                    ctl.errorReply(fd, t, TPLC_E_DBG_OVERLOAD,
                                   "stream slots full", req);
                    return;
                }
                wt.rate_hz = hz;
                /* emit first frame at the next boundary */
                wt.next_emit = 0;
                audit(fd, "dbg.watch.stream",
                      "h=" + std::to_string(h) +
                      " hz=" + std::to_string(hz));
                ctl.replyEmpty(fd, t, req);
                return;
            }
        ctl.errorReply(fd, t, TPLC_E_DBG_STATE, "no such watch", req);
    });
    /* {u32 handle} 0xFFFFFFFF = all */
    ctl.on(TPLC_OP_DBG_WATCH_REMOVE, [&](int fd, uint16_t t,
                                         const uint8_t* p, uint32_t n,
                                         uint64_t req) {
        if (n != 4) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u32 handle}", req);
            return;
        }
        uint32_t h; memcpy(&h, p, 4);
        if (h == 0xFFFFFFFFu) {
            watches.clear();
            audit(fd, "dbg.watch.remove", "all");
            ctl.replyEmpty(fd, t, req);
            return;
        }
        for (auto it = watches.begin(); it != watches.end(); ++it)
            if (it->handle == h) {
                watches.erase(it);
                audit(fd, "dbg.watch.remove", std::to_string(h));
                ctl.replyEmpty(fd, t, req);
                return;
            }
        ctl.errorReply(fd, t, TPLC_E_DBG_STATE, "no such watch", req);
    });
    /* → {u32 n, {u32 handle, u32 n_items, u32 rate_hz,
                  u32 total_len}*}                                 */
    ctl.on(TPLC_OP_DBG_WATCH_LIST, [&](int fd, uint16_t t,
                                       const uint8_t*, uint32_t,
                                       uint64_t req) {
        tplc::CtlServer::W w;
        w.u32((uint32_t)watches.size());
        for (auto& wt : watches) {
            uint32_t tl = 0, nst = 0;
            for (auto& c : wt.chs)
                if (c.stale) nst++; else tl += c.len;
            w.u32(wt.handle); w.u32((uint32_t)wt.chs.size());
            w.u32(wt.rate_hz); w.u32(tl); w.u32(nst);
        }
        ctl.reply(fd, t, w.data(), w.size(), req);
    });

    /* ---------------- deploy group (doc 04 §I.1 0x0200) ---------- */
    DeployUpload up;
    ctl.on(TPLC_OP_DEPLOY_BEGIN, [&](int fd, uint16_t t,
                                     const uint8_t* payload,
                                     uint32_t len, uint64_t req) {
        if (len < 41) { /* u64 size + 32 sha + u8 mode */
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD, "short", req);
            return;
        }
        if (up.active) {
            ctl.errorReply(fd, t, TPLC_E_CTL_STATE,
                           "upload in progress", req);
            return;
        }
        uint64_t sz;
        memcpy(&sz, payload, 8);
        if (sz == 0 || sz > 64ull * 1024 * 1024) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD, "bad size", req);
            return;
        }
        ::mkdir(packages_dir.c_str(), 0755);
        char tp[] = "/tmp/tplc_dep_XXXXXX";
        int ufd = mkstemp(tp);
        if (ufd < 0) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_DLOPEN,
                           "tmp open failed", req);
            return;
        }
        up.size = sz; up.mode = payload[40];
        memcpy(up.sha, payload + 8, 32);
        up.fd = ufd; up.tmp_path = tp;
        static uint64_t up_ctr = 0;
        up.id = ++up_ctr; up.active = true; up.received = 0;
        tplc::CtlServer::W w;
        w.u64(up.id); w.u32(64 * 1024);       /* max_chunk */
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_DEPLOY_CHUNK, [&](int fd, uint16_t t,
                                     const uint8_t* payload,
                                     uint32_t len, uint64_t req) {
        if (len < 16) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD, "short", req);
            return;
        }
        uint64_t id, off;
        memcpy(&id, payload, 8); memcpy(&off, payload + 8, 8);
        if (!up.active || id != up.id) {
            ctl.errorReply(fd, t, TPLC_E_CTL_STATE,
                           "no upload", req);
            return;
        }
        if (off != up.received) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "out of order", req);
            return;
        }
        uint32_t dlen = len - 16;
        ssize_t wn = write(up.fd, payload + 16, dlen);
        if (wn != (ssize_t)dlen) {
            up.drop();
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_DLOPEN,
                           "write failed", req);
            return;
        }
        up.received += dlen;
        ctl.replyEmpty(fd, t, req);
    });
    ctl.on(TPLC_OP_DEPLOY_COMMIT, [&](int fd, uint16_t t,
                                      const uint8_t* payload,
                                      uint32_t len, uint64_t req) {
        if (len < 8) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD, "short", req);
            return;
        }
        uint64_t id;
        memcpy(&id, payload, 8);
        if (!up.active || id != up.id) {
            ctl.errorReply(fd, t, TPLC_E_CTL_STATE,
                           "no upload", req);
            return;
        }
        if (up.received != up.size) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "incomplete", req);
            return;
        }
        uint8_t got[32];
        sha256_file(up.fd, got);
        if (memcmp(got, up.sha, 32) != 0) {
            up.drop();
            ctl.errorReply(fd, t, TPLC_E_PKG_HASH_CONTENT,
                           "sha256 mismatch", req);
            return;
        }
        /* package_uuid = first 16B of sha256; store atomically. The
           tmp file may live on a different fs than packages_dir, so
           copy into dst.tmp then rename inside the same dir. */
        char hex[33]; uuid_hex(got, hex);
        std::string dst = packages_dir + "/" + hex + ".tplc";
        std::string staging = dst + ".tmp";
        ::close(up.fd); up.fd = -1;
        std::error_code ec;
        std::filesystem::copy_file(up.tmp_path, staging,
                                   std::filesystem::copy_options::overwrite_existing,
                                   ec);
        if (ec || ::rename(staging.c_str(), dst.c_str()) != 0) {
            ::unlink(up.tmp_path.c_str());
            ::unlink(staging.c_str());
            up.drop();
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_DLOPEN,
                           "store failed", req);
            return;
        }
        ::unlink(up.tmp_path.c_str());
        auto pres = tplc::Package::open(dst);
        if (!pres.ok()) {
            ::unlink(dst.c_str());
            up.drop();
            ctl.errorReply(fd, t, pres.st.code,
                           pres.st.msg.c_str(), req);
            return;
        }
        up.drop();
        tplc::CtlServer::W w;
        w.u8(0);                            /* result: ok */
        w.bytes(got, 16);                   /* package_uuid */
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_DEPLOY_ABORT, [&](int fd, uint16_t t,
                                     const uint8_t*, uint32_t,
                                     uint64_t req) {
        up.drop();
        ctl.replyEmpty(fd, t, req);
    });
    ctl.on(TPLC_OP_PKG_LIST, [&](int fd, uint16_t t,
                                 const uint8_t*, uint32_t,
                                 uint64_t req) {
        tplc::CtlServer::W w;
        DIR* dp = opendir(packages_dir.c_str());
        uint32_t count = 0;
        if (dp) {
            struct dirent* e;
            while ((e = readdir(dp))) {
                std::string n = e->d_name;
                if (n.size() != 37 ||
                    n.compare(32, 5, ".tplc") != 0) continue;
                uint8_t u[16];
                if (!hex_uuid(n.substr(0, 32), u)) continue;
                std::string p = packages_dir + "/" + n;
                struct stat stf;
                if (stat(p.c_str(), &stf) != 0) continue;
                w.bytes(u, 16);
                w.u64((uint64_t)stf.st_size);
                count++;
            }
            closedir(dp);
        }
        /* count leads the entry list */
        std::vector<uint8_t> buf;
        buf.resize(4);
        memcpy(buf.data(), &count, 4);
        buf.insert(buf.end(), w.data(), w.data() + w.size());
        ctl.reply(fd, t, buf.data(), (uint32_t)buf.size(), req);
    });
    ctl.on(TPLC_OP_PKG_INFO, [&](int fd, uint16_t t,
                                 const uint8_t* payload,
                                 uint32_t len, uint64_t req) {
        if (len < 16) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD, "short", req);
            return;
        }
        char hex[33]; uuid_hex(payload, hex);
        std::string p = packages_dir + "/" + hex + ".tplc";
        struct stat stf;
        if (stat(p.c_str(), &stf) != 0) {
            ctl.errorReply(fd, t, TPLC_E_CTL_STATE,
                           "not found", req);
            return;
        }
        tplc::CtlServer::W w;
        w.bytes(payload, 16);
        w.u64((uint64_t)stf.st_size);
        ctl.reply(fd, t, w.data(), w.size(), req);
    });
    ctl.on(TPLC_OP_RT_STATUS, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        TplcCtlRtStatus rs{};
        rs.proto_ver = TPLC_CTL_PROTO_VER;
        rs.uptime_s = (uint32_t)((mono_ns() - started_ns) / 1000000000ull);
        rs.prog_count = 1;
        rs.running_count = prog_state == PS_RUNNING ? 1 : 0;
        rs.master_status = tplc_image_master_status(&img);
        rs.plc_status = tplc_image_plc_status(&img);
        rs.master_hb_ns = tplc_image_master_hb_ns(&img);
        rs.plc_hb_ns = tplc_image_plc_hb_ns(&img);
        memcpy(rs.version, "tether-plc 0.1", 14);
        ctl.reply(fd, t, &rs, sizeof rs, req);
    });
    ctl.on(TPLC_OP_RT_SHUTDOWN, [&](int fd, uint16_t t,
                                    const uint8_t*, uint32_t,
                                    uint64_t req) {
        ctl.replyEmpty(fd, t, req);
        g_stop = 1;
    });

    /* ---------------- signals + main loop ----------------------- */
    signal(SIGTERM, on_sig);
    signal(SIGINT, on_sig);
    signal(SIGPIPE, SIG_IGN);

    uint64_t last_flush_ns = mono_ns();
    /* watchdog monitor (doc 04 §6): the scan path is this thread, so
       a hang inside a task stalls progress counting — the monitor is
       the backstop that aborts cooperatively, then self-terminates
       for supervisor respawn */
    std::atomic<uint64_t> progress{0};
    std::atomic<bool>     mon_running{true};
    /* ctl.event isn't thread-safe from the monitor — it flags the
       fault transition and the scan loop emits it (doc 16 §4)      */
    std::atomic<bool>     stall_fault_ev{false};
    std::thread monitor([&] {
        uint64_t last_seen = 0;
        uint64_t stale_since = mono_ns();
        uint64_t fault_since = 0;
        while (mon_running.load()) {
            std::this_thread::sleep_for(std::chrono::milliseconds(50));
            uint64_t p = progress.load();
            uint64_t now = mono_ns();
            if (p != last_seen) {
                last_seen = p; stale_since = now; fault_since = 0;
            }
            if (prog_state != PS_RUNNING) {
                /* escalation: only the stall WE flagged counts — the
                   wedged task never returns to service ctl, so self-
                   termination is the only escape. A cooperative fault
                   (task aborted, operator can still ctl) keeps the
                   process alive for diagnosis. */
                if (fault_since &&
                    (now - fault_since) / 1000000ull > hang_timeout_ms) {
                    fprintf(stderr,
                            "tether-plc: unrecovered stall — exiting\n");
                    _exit(2);
                }
                if (!fault_since) stale_since = now;
                continue;
            }
            uint64_t stale_ms = (now - stale_since) / 1000000ull;
            if (stale_ms > hang_timeout_ms &&
                prog_state == PS_RUNNING && prog) {
                prog->requestAbort();
                prog_state = PS_FAULT;
                stall_fault_ev = true;
                tplc_image_set_fault(&img, (uint64_t)-TPLC_TASK_E_FAULT);
                tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
                fault_since = now;
                fprintf(stderr,
                        "tether-plc: scan stall %llums — fault + abort\n",
                        (unsigned long long)stale_ms);
            }
        }
    });

    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_RUN);
    runbook_write(runbook_dir, instance_name,
                  package_path.empty() ? module_path.c_str()
                                       : package_path.c_str(),
                  d.program_uuid, "running");
    fprintf(stderr, "tether-plc: running (module=%s image=%s ctl=%s)\n",
            module_path.c_str(), image_path.c_str(), ctl_path.c_str());

    const bool sim_mode = (ctx_flags & TPLC_CTXF_SIM) != 0;
    /* break_when evaluation (doc 12 §5): each armed condition is a
       ≤8B blob read at the post-tick boundary — bounded work. A
       break fires only when it newly suspends its task (resuming
       while the condition holds re-arms it).                      */
    auto eval_breaks = [&]() {
        if (!prog || breaks.empty()) return;
        for (auto& b : breaks) {
            if (b.stale) continue;
            /* suspend-breaks skip while their task is held; capture
               breaks skip while the recorder is already frozen    */
            if (b.capture ? trace_frozen
                          : sched.suspended(*prog, b.task_idx))
                continue;
            uint64_t cur = 0;
            bool hit;
            if (b.x) {
                hit = expr_eval(*b.x);
                cur = (uint64_t)hit;
            } else {
                memcpy(&cur, prog->blob() + b.off, b.len);
                switch (b.cmp) {
                    case 0: hit = cur == b.val; break;
                    case 1: hit = cur != b.val; break;
                    case 2: hit = cur <  b.val; break;
                    case 3: hit = cur <= b.val; break;
                    case 4: hit = cur >  b.val; break;
                    default: hit = cur >= b.val; break;
                }
            }
            if (!hit) continue;
            bool fired = b.capture
                ? (trace_frozen = true)
                : sched.suspend(*prog, b.task_idx);
            if (fired) {
                b.hits++;
                tplc::CtlServer::W w;
                w.u32(b.handle); w.u32(b.task_idx); w.u64(cur);
                ctl.event(TPLC_EV_DBG_BREAK,
                          1u << (TPLC_EV_DBG_BREAK & 0xFF),
                          w.data(), w.size());
            }
        }
    };

    /* watch streams (doc 12 §3): armed watches push one
       EV_DBG_WATCH frame when their interval elapses — the sample
       is the boundary-consistent blob bytes of that scan          */
    auto eval_watches = [&](uint64_t now) {
        if (!prog || watches.empty()) return;
        for (auto& wt : watches) {
            if (!wt.rate_hz) continue;
            if (wt.next_emit && now < wt.next_emit) continue;
            wt.next_emit = now + 1000000000ull / wt.rate_hz;
            tplc::CtlServer::W w;
            w.u32(wt.handle); w.u64(wt.seq++);
            w.u32((uint32_t)wt.chs.size());
            for (auto& c : wt.chs) {
                tplc::Program* src =
                    c.shadow ? (shadow && shadow->prog
                                ? &*shadow->prog : nullptr)
                             : prog ? &*prog : nullptr;
                bool stale = c.stale || !src ||
                    c.off > src->blobSize() ||
                    c.len > src->blobSize() - c.off;
                w.u32(stale ? 0 : c.len);
                if (!stale)
                    w.bytes(src->blob() + c.off, c.len);
            }
            ctl.event(TPLC_EV_DBG_WATCH,
                      1u << (TPLC_EV_DBG_WATCH & 0xFF),
                      w.data(), w.size());
        }
    };

    while (!g_stop) {
        uint64_t now = mono_ns();
        if (sim_mode && prog) {
            uint64_t sn = tplc_image_sim_now(&img);
            prog->services().setSimNow(sn);
            now = sn;
        }

        /* master-loss policy surface (doc 03 §6.3): we keep cycling on
           held inputs — the latch returns last-good data; status goes
           degraded-announced via plc heartbeat continuing to tick     */
        uint32_t mst = tplc_image_master_status(&img);
        if (prog_state == PS_RUNNING &&
            (mst == TPLC_IMG_MASTER_RUN ||
             mst == TPLC_IMG_MASTER_DEGRADED ||
             mst == TPLC_IMG_MASTER_INIT)) {
            size_t ran = sched.tick(now, hooks);
            /* re-assert overrides at the boundary: internals and
               shadows hold the forced value for observers too      */
            if (!forces.empty()) stamp_forces(0);
            /* flight recorder: one frame per scan that ran work —
               post-force state; bounded 64B copy; sampled before
               break/capture eval so the triggering scan is kept   */
            if (ran > 0 && !trace_ch.empty() && !trace_frozen &&
                prog) {
                TraceFrame f;
                f.seq = trace_wr;
                f.ts = now;
                uint8_t* dp = f.data;
                for (auto& c : trace_ch) {
                    memcpy(dp, prog->blob() + c.off, c.len);
                    dp += c.len;
                }
                if (trace_ring.size() < TRACE_CAP)
                    trace_ring.push_back(f);
                else
                    trace_ring[trace_wr % TRACE_CAP] = f;
                trace_wr++;
            }
            /* watch streams sample the same boundary image        */
            if (ran > 0) eval_watches(now);
            /* breaks see post-force state — the effective value    */
            if (!breaks.empty()) eval_breaks();
            /* shadow run (doc 19 §2.4): after the active commit,
               on the slack left until the next active release —
               the shadow can never delay the scan. A faster-period
               shadow ticks between active releases too (it re-reads
               the same latched %I, consistent though stale)       */
            if (shadow && shadow->prog) {
                uint64_t sdue = shadow->sched.nextDueNs();
                bool sh_due = sdue != UINT64_MAX && sdue <= now;
                uint64_t due = sched.nextDueNs();
                int64_t slack =
                    due == UINT64_MAX ? INT64_MAX
                  : due <= now        ? 0
                  : (int64_t)std::min<uint64_t>(due - now,
                                                (uint64_t)INT64_MAX);
                if (!sh_due) {
                    /* no shadow work due this scan */
                } else if (shadow->cycles == 0 && ran == 0) {
                    /* join boundary: the first shadow cycle waits
                       for an active scan so its latch copies this
                       scan's %I, never stale pre-load inputs     */
                } else if (slack >= (int64_t)shadow_budget_ns &&
                           now != shadow_last_now) {
                    /* one shadow cycle per scan instant: the main loop
                       spins while sim time is frozen, and a catch-up
                       tick would consume the same latched %I twice
                       and desynchronise the candidate from the active */
                    shadow_last_now = now;
                    size_t sran = shadow->sched.tick(now, shooks);
                    if (shadow_faulted) {
                        shadow_faulted = false;
                        drop_shadow("task fault");
                    } else if (sran > 0) {
                        shadow->cycles += sran;
                        /* compare shadow %Q vs active %Q in image-
                           offset space — run-length diffs ≤32B     */
                        uint64_t nb = std::min(
                            (uint64_t)prog->desc().image_out_bytes,
                            (uint64_t)shadow->prog->desc()
                                .image_out_bytes);
                        const uint8_t* a = prog->imageOut();
                        const uint8_t* s = shadow->prog->imageOut();
                        for (uint64_t off = 0; off < nb;) {
                            if (a[off] == s[off]) { off++; continue; }
                            uint64_t e = off + 1;
                            while (e < nb && a[e] != s[e] &&
                                   e - off < 32) e++;
                            ShadowDiff df{};
                            df.cycle = shadow->cycles;
                            df.img_off = off;
                            df.len = (uint32_t)(e - off);
                            memcpy(df.active_bytes, a + off, e - off);
                            memcpy(df.shadow_bytes, s + off, e - off);
                            if (shadow->diffs.size() < SHADOW_DIFF_CAP)
                                shadow->diffs.push_back(df);
                            else
                                shadow->diffs[shadow->diff_wr %
                                              SHADOW_DIFF_CAP] = df;
                            shadow->diff_wr++;
                            shadow->diff_count++;
                            shadow->last_diff_cycle = shadow->cycles;
                            off = e;
                        }
                    }
                } else {
                    shadow->missed++;
                }
            }
            progress.fetch_add(1);
            /* quarantine window: N scans after commit the old module
               is released — doc 08 §7 (drop order: Program first)  */
            if (quar.cycles_left > 0 && --quar.cycles_left == 0) {
                quar.prog.reset();
                quar.mod.reset();
                quar.layd.clear();
                fprintf(stderr, "tether-plc: quarantine released\n");
            }
        }

        /* retain: module dirty hints mark the store; staging happens
           here — between ticks, i.e. at task boundaries (doc 04 §5) */
        if (retain && retain->attached() && prog) {
            if (!prog->services().retainDirtyRanges().empty()) {
                prog->services().clearRetainDirty();
                retain->noteDirty();
            }
            if (retain->dirty() ||
                now - last_flush_ns >= retain_flush_ms * 1000000ull) {
                retain->stage(prog->retainRegion());
                retain->flush();
                last_flush_ns = now;
                retain_last_flush_ns = realtime_ns();
            }
        }

        /* deferred monitor-thread fault event (doc 16 §4)          */
        if (stall_fault_ev.exchange(false))
            emit_transition((uint8_t)PS_RUNNING, (uint8_t)PS_FAULT, 1);

        tplc_image_plc_heartbeat(&img, now);

        /* wait: min(next task due, poll cap); a task already due
           must not sleep past its release */
        uint64_t nd = sched.nextDueNs();
        uint64_t wait_ns = poll_ns;
        if (nd != UINT64_MAX)
            wait_ns = (nd <= now) ? 0
                      : ((nd - now) < poll_ns ? (nd - now) : poll_ns);
        /* an epoll ms timeout can overshoot the due instant by a
           scheduler tick, landing every release ~1 ms late; inside
           IMMINENT the poll only peeks and the tail below carries
           the wait with ns precision instead */
        const uint64_t IMMINENT = 3000000ull;   /* 3 ms */
        static uint64_t spin_margin = 600000;   /* ns, adapts below */
        if (nd != UINT64_MAX && nd > now && nd - now < IMMINENT)
            wait_ns = 0;
        ctl.poll((int)(wait_ns / 1000000ull));
        if (nd != UINT64_MAX && (now = mono_ns()) < nd &&
            nd - now < IMMINENT) {
            uint64_t rem = nd - now;
            /* coarse-sleep to spin_margin out then spin the rest;
               the margin adapts to measured sleep overshoot so a
               loaded box widens it and an idle one narrows it,
               bounded [200 us, 2 ms]                           */
            if (rem > spin_margin) {
                struct timespec ts{0, (long)(rem - spin_margin)};
                nanosleep(&ts, nullptr);
                now = mono_ns();
                int64_t over = (int64_t)now - (int64_t)(nd - spin_margin);
                if (over > 0 && spin_margin < 2000000ull)
                    spin_margin += (uint64_t)over / 4 + 1;
                else if (over < -(int64_t)(spin_margin / 2) &&
                         spin_margin > 200000ull)
                    spin_margin -= spin_margin / 8;
            }
            while (nd > now) now = mono_ns();
        }
    }

    fprintf(stderr, "tether-plc: stopping\n");
    mon_running = false;
    if (monitor.joinable()) monitor.join();
    /* final runbook state: a program that faulted stays "error" —
       auto-resume restores ERROR not RUNNING (doc 04 §7)           */
    const char* final_state = prog_state == PS_FAULT ? "error"
                          : prog_state == PS_EMPTY  ? "unloaded"
                          : "stopped";
    prog_state = PS_STOPPED;
    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
    tplc_image_plc_heartbeat(&img, mono_ns());
    runbook_write(runbook_dir, instance_name,
                  package_path.empty() ? module_path.c_str()
                                       : package_path.c_str(),
                  d.program_uuid, final_state);
    if (retain && retain->attached() && prog) {
        retain->stage(prog->retainRegion());
        retain->flush();              /* final persist on shutdown */
    }
    if (prog) prog->stop();
    ctl.close();
    tplc_image_close(&img);
    return 0;
}
