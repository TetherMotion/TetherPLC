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
#include <string>
#include <unistd.h>

static volatile sig_atomic_t g_stop = 0;
static void on_sig(int) { g_stop = 1; }

static uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
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
        "          [--sim]\n"
        "       %s --supervise [--respawn always|on-fault|never]\n"
        "          [--restart-delay-ms N] -- <tether-plc args>\n",
        a0, a0);
}

enum ProgState : uint8_t { PS_STOPPED = 0, PS_RUNNING = 1, PS_FAULT = 2,
                           PS_EMPTY = 3 };

static void uuid_hex(const uint8_t u[16], char out[33]) {
    static const char* H = "0123456789abcdef";
    for (int i = 0; i < 16; i++) {
        out[i*2] = H[u[i] >> 4]; out[i*2+1] = H[u[i] & 0xF];
    }
    out[32] = 0;
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
    std::string trust_dir  = "/etc/tether/trust";
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
        else if (a == "--auto-rollback") auto_rollback = true;
        else if (a == "--sig-policy") sig_policy = next();
        else if (a == "--trust-dir") trust_dir = next();
        else if (a == "--no-sandbox") no_sandbox = true;
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

    /* ---------------- package → extracted module -------------- */
    std::string extracted_so;
    std::vector<uint8_t> boot_layd, boot_smap;   /* sections from --package */
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
        tplc::PkgSection obj{}, b_layd{}, b_smap{};
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
        int                          cycles_left = 0;
    } quar;

    auto unload_program = [&] {
        /* monitor thread skips non-RUNNING states, so clearing state
           first makes destruction race-free */
        prog_state = PS_EMPTY;
        if (prog) { sched.removeProgram(*prog); prog.reset(); }
        mod.reset();
        if (retain) retain->close();
        quar = Quarantine{};
        pkg_layd.clear();
        pkg_smap.clear();
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
    auto do_rollback = [&]() -> bool {
        if (!quar.prog) return false;
        /* same storage-address rule as commit: the scheduler keeps
           Program* — move objects between homes explicitly, then
           register the final address (heap Modules never move)     */
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
        forces.clear();   /* blob identity changed — forces stale */
        trace_ch.clear();
        trace_ring.clear();
        trace_wr = 0;
        trace_flags = 0;
        trace_frozen = false;
        if (reresolve_subs) reresolve_subs();
        sched.addProgram(*prog);
        d = mod->descriptor();
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
    tplc::Status st = prog->start();
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
    tplc::CtlServer ctl;
    hooks.fault = [&](tplc::Program& p, uint32_t, int32_t frc) {
            /* doc 02 §4.6 / doc 04 §6: task fault → program policy.
           hold = stay faulted; restart = warm reset bounded by
           restart_max/60s then hold; unload = stop+deschedule.      */
        last_fault = frc;
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
        runbook_write(runbook_dir, instance_name,
                      package_path.empty() ? module_path.c_str()
                                           : package_path.c_str(),
                      d.program_uuid, "error");
    };

    /* audit trail (doc 12 §8): every mutating debug/lifecycle op
       logs who(session+name), what, and the operand — stderr so it
       lands in the service journal                                */
    auto audit = [&](int fd, const char* op,
                     const std::string& detail) {
        uint64_t s = 0; std::string nm;
        ctl.clientInfo(fd, &s, &nm);
        fprintf(stderr, "audit s%llu %s: %s %s\n",
                (unsigned long long)s,
                nm.empty() ? "?" : nm.c_str(), op,
                detail.c_str());
    };

    /* ---- trigger expressions (doc 12 §M) ------------------------
       Compiler + evaluator for the bounded stack machine; paths
       resolve through the symmap at arm time                      */
    auto smap_find = [&](std::string_view want,
                         uint64_t* off, uint32_t* len) -> bool {
        const uint8_t* sm = pkg_smap.data();
        if (pkg_smap.size() < 32 || memcmp(sm, "SMAP", 4) != 0)
            return false;
        uint32_t count, stroff, strln;
        memcpy(&count, sm + 8, 4);
        memcpy(&stroff, sm + 12, 4);
        memcpy(&strln, sm + 16, 4);
        if ((uint64_t)stroff + strln > pkg_smap.size() ||
            32ull + (uint64_t)count * 64 > pkg_smap.size())
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
        TplcCtlProgStateChange sc{(uint8_t)prog_state, PS_STOPPED, 0};
        if (prog_state == PS_RUNNING) {
            prog_state = PS_STOPPED;
            forces.clear();
            tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
        }
        audit(fd, "prog.stop", "");
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
        uint8_t prev = prog_state;
        if (prog_state == PS_STOPPED) {
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
        }
        audit(fd, "prog.start", "");
        TplcCtlProgStateChange sc{prev, (uint8_t)prog_state, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    /* kill: set the abort flag and deschedule immediately — ctl ops
       run between ticks, so no task is in-flight (doc 04 §4 kill)  */
    ctl.on(TPLC_OP_PROG_KILL, [&](int fd, uint16_t t,
                                  const uint8_t*, uint32_t,
                                  uint64_t req) {
        if (!prog || prog_state != PS_RUNNING) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "not running", req);
            return;
        }
        /* ctl ops run between ticks, so no task is in-flight: set the
           abort flag (harmless pending), deschedule immediately, and
           let start clear the flag — immediate kill per doc 04 §4   */
        prog->requestAbort();
        prog_state = PS_STOPPED;
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
        TplcCtlProgStateChange sc{PS_RUNNING, PS_STOPPED, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    /* reset {u8 warm}: warm keeps retain bytes + file, cold zeroes
       both (doc 04 §4). Requires STOPPED.                         */
    ctl.on(TPLC_OP_PROG_RESET, [&](int fd, uint16_t t,
                                   const uint8_t* payload, uint32_t len,
                                   uint64_t req) {
        if (!prog || prog_state != PS_STOPPED) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "requires stopped", req);
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
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "stop first", req);
            return;
        }
        unload_program();
        TplcCtlProgStateChange sc{PS_STOPPED, PS_EMPTY, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
    });
    /* prepare {u16 len + path} or {16B package_uuid in --packages}:
       load (or swap to) a module — lands STOPPED; start runs init */
    ctl.on(TPLC_OP_PROG_PREPARE, [&](int fd, uint16_t t,
                                     const uint8_t* payload,
                                     uint32_t len, uint64_t req) {
        if (prog_state == PS_RUNNING) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "stop first", req);
            return;
        }
        std::string path;
        if (len == 16) {
            char hex[33]; uuid_hex(payload, hex);
            path = packages_dir + "/" + hex + ".tplc";
        } else if (len >= 2) {
            uint16_t plen;
            memcpy(&plen, payload, 2);
            if ((uint32_t)plen + 2 > len) {
                ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                               "bad path", req);
                return;
            }
            path.assign((const char*)payload + 2, plen);
        } else {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "uuid or path", req);
            return;
        }
        std::string mpath = path;
        std::string tmp_so;
        std::vector<uint8_t> layd_b, smap_b;   /* sections borrow pk */
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
            tplc::Status sgst = pk.verifySignature(sig_policy, trust_dir);
            if (!sgst.ok()) {
                ctl.errorReply(fd, t, sgst.code, sgst.msg.c_str(), req);
                return;
            }
            tplc::PkgSection sec{};
            if (pk.section(TPLC_SEC_LAYOUT_DESC, sec))
                layd_b.assign(sec.data, sec.data + sec.size);
            if (pk.section(TPLC_SEC_SYMMAP, sec))
                smap_b.assign(sec.data, sec.data + sec.size);
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
        }
        tplc::Status lst = load_program(mpath);
        if (!lst.ok()) {
            ctl.errorReply(fd, t, lst.code, lst.msg.c_str(), req);
            return;
        }
        /* install the package's debug descriptors so resolve/watch/
           expr ops see symbols for prepare-loaded programs too */
        pkg_layd = std::move(layd_b);
        pkg_smap = std::move(smap_b);
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
        fprintf(stderr, "tether-plc: rollback restored prior program\n");
        ctl.replyEmpty(fd, t, req);
    });

    ctl.on(TPLC_OP_PROG_SWAP, [&](int fd, uint16_t t,
                                  const uint8_t* payload,
                                  uint32_t len, uint64_t req) {
        if (prog_state != PS_RUNNING || !prog) {
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_STATE,
                           "program not running", req);
            return;
        }
        /* resolve the staged package — 16B uuid or {len,path} */
        std::string path;
        if (len == 16) {
            char hex[33]; uuid_hex(payload, hex);
            path = packages_dir + "/" + hex + ".tplc";
        } else if (len >= 2) {
            uint16_t plen;
            memcpy(&plen, payload, 2);
            if ((uint32_t)plen + 2 > len) {
                ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                               "bad path", req);
                return;
            }
            path.assign((const char*)payload + 2, plen);
        } else {
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
        tplc::Status sgst = pk.verifySignature(sig_policy, trust_dir);
        if (!sgst.ok()) {
            ctl.errorReply(fd, t, sgst.code, sgst.msg.c_str(), req);
            return;
        }
        tplc::PkgSection obj{}, migd{}, layd{}, smap{};
        if (!pk.section(TPLC_SEC_TARGET_OBJ, obj)) {
            ctl.errorReply(fd, t, TPLC_E_PKG_TOC, "no target_obj",
                           req);
            return;
        }
        bool has_migd = pk.section(TPLC_SEC_MIGRATE_DESC, migd);
        pk.section(TPLC_SEC_LAYOUT_DESC, layd);
        pk.section(TPLC_SEC_SYMMAP, smap);
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
        d = mod->descriptor();
        if (layd.data && layd.size)
            pkg_layd.assign(layd.data, layd.data + layd.size);
        else
            pkg_layd.clear();
        if (smap.data && smap.size)
            pkg_smap.assign(smap.data, smap.data + smap.size);
        else
            pkg_smap.clear();
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
        if (ctl_enabled)
            ctl.event(TPLC_EV_PROG_TRANSITION, 2, nullptr, 0);
        uint8_t rep[8];
        memcpy(rep, &mo.out_status, 4);
        memcpy(rep + 4, &mo.out_first_error, 4);
        ctl.reply(fd, t, rep, sizeof rep, req);
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
        const uint8_t* sm = pkg_smap.data();
        if (pkg_smap.size() < 32 || memcmp(sm, "SMAP", 4) != 0) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "no symbol map", req);
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
        std::string_view want((const char*)p + 2, nl);
        uint32_t count, stroff, strln;
        memcpy(&count, sm + 8, 4);
        memcpy(&stroff, sm + 12, 4);
        memcpy(&strln, sm + 16, 4);
        if ((uint64_t)stroff + strln > pkg_smap.size() ||
            32ull + (uint64_t)count * 64 > pkg_smap.size()) {
            ctl.errorReply(fd, t, TPLC_E_DBG_STATE,
                           "corrupt symbol map", req);
            return;
        }
        const uint8_t* strs = sm + stroff;
        for (uint32_t i = 0; i < count; i++) {
            const uint8_t* e = sm + 32 + (size_t)i * 64;
            uint32_t noff; memcpy(&noff, e, 4);
            if (noff >= strln) continue;
            const char* nm = (const char*)strs + noff;
            if (!memchr(nm, 0, strln - noff)) continue;
            if (want != nm) continue;
            tplc::CtlServer::W w;
            w.bytes(d.program_uuid, 16);
            w.u32(i);
            w.bytes(e + 4, 4);   /* kind flags region access */
            w.bytes(e + 8, 4);   /* type_ref  */
            w.bytes(e + 12, 8);  /* offset    */
            w.bytes(e + 20, 4);  /* size      */
            ctl.reply(fd, t, w.data(), w.size(), req);
            return;
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
        if (n != 12) {
            ctl.errorReply(fd, t, TPLC_E_CTL_PAYLOAD,
                           "{u64 off, u32 len}", req);
            return;
        }
        uint64_t off; uint32_t rl;
        memcpy(&off, p, 8); memcpy(&rl, p + 8, 4);
        if (off > prog->blobSize() || rl > prog->blobSize() - off) {
            ctl.errorReply(fd, t, TPLC_E_DBG_ACCESS,
                           "range outside blob", req);
            return;
        }
        ctl.reply(fd, t, prog->blob() + off, rl, req);
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
       SMAP so a client can enumerate the loaded program's fields  */
    ctl.on(TPLC_OP_DBG_FIELDS, [&](int fd, uint16_t t,
                                   const uint8_t*, uint32_t,
                                   uint64_t req) {
        const uint8_t* b = pkg_layd.data();
        size_t n = pkg_layd.size();
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
        const uint8_t* sm = pkg_smap.data();
        bool have_sm = sm && pkg_smap.size() >= 32 &&
                       !memcmp(sm, "SMAP", 4);
        uint32_t scnt = have_sm ? r32(sm + 8) : 0;
        uint32_t so = have_sm ? r32(sm + 12) : 0;
        uint32_t sl = have_sm ? r32(sm + 16) : 0;
        if (have_sm && ((uint64_t)so + sl > pkg_smap.size() ||
                        32ull + (uint64_t)scnt * 64 >
                            pkg_smap.size()))
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
            if (nl2) {
                if (!smap_find(path, &off, &l)) {
                    ctl.errorReply(fd, t, TPLC_E_DBG_RESOLVE,
                                   ("unknown path " + path).c_str(),
                                   req);
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
            chs.push_back({off, l, std::move(path), false});
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
                    w.u32(c.stale ? 0 : c.len);
                    if (!c.stale && prog)
                        w.bytes(prog->blob() + c.off, c.len);
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
    std::thread monitor([&] {
        uint64_t last_seen = 0;
        uint64_t stale_since = mono_ns();
        while (mon_running.load()) {
            std::this_thread::sleep_for(std::chrono::milliseconds(50));
            uint64_t p = progress.load();
            uint64_t now = mono_ns();
            if (p != last_seen) { last_seen = p; stale_since = now; }
            if (prog_state != PS_RUNNING) { stale_since = now; continue; }
            uint64_t stale_ms = (now - stale_since) / 1000000ull;
            if (stale_ms > hang_timeout_ms) {
                if (prog_state == PS_RUNNING && prog) {
                    prog->requestAbort();
                    prog_state = PS_FAULT;
                    tplc_image_set_fault(&img, (uint64_t)-TPLC_TASK_E_FAULT);
                    tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
                    fprintf(stderr,
                            "tether-plc: scan stall %llums — fault + abort\n",
                            (unsigned long long)stale_ms);
                }
                if (stale_ms > 2 * hang_timeout_ms) {
                    fprintf(stderr,
                            "tether-plc: unrecovered stall — exiting\n");
                    _exit(2);
                }
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
                w.u32(c.stale ? 0 : c.len);
                if (!c.stale)
                    w.bytes(prog->blob() + c.off, c.len);
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
            }
        }

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
