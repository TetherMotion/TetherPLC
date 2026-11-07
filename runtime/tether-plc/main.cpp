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

#include <cerrno>
#include <atomic>
#include <csignal>
#include <cstdio>
#include <cstring>
#include <deque>
#include <thread>
#include <sys/stat.h>
#include <sys/wait.h>
#include <fcntl.h>
#include <cstdint>
#include <cstring>
#include <ctime>
#include <dirent.h>
#include <optional>
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
        "          [--sim]\n"
        "       %s --supervise [--respawn always|on-fault|never]\n"
        "          [--restart-delay-ms N] -- <tether-plc args>\n",
        a0);
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
    std::string image_path, module_path, package_path, ctl_path;
    std::string retain_dir, runbook_dir, instance_name, packages_dir;
    uint64_t retain_flush_ms = 1000;
    uint64_t wcet_hard_ns = 0;
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

    /* ---------------- package → extracted module -------------- */
    std::string extracted_so;
    if (!package_path.empty()) {
        auto pres = tplc::Package::open(package_path);
        if (!pres.ok()) {
            fprintf(stderr, "tether-plc: package %s unreadable (%d) %s\n",
                    package_path.c_str(), pres.st.code, pres.st.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        tplc::Package pkg = std::move(pres.value);
        auto vst = pkg.validateForLoad(false);
        if (!vst.ok()) {
            fprintf(stderr, "tether-plc: package not loadable (%d) %s\n",
                    vst.code, vst.msg.c_str());
            tplc_image_close(&img);
            return 1;
        }
        tplc::PkgSection obj{};
        /* v1: first target_obj (multi-target selection comes with
           manifest.targets[] parsing) */
        if (!pkg.section(TPLC_SEC_TARGET_OBJ, obj)) {
            fprintf(stderr, "tether-plc: no target_obj section\n");
            tplc_image_close(&img);
            return 1;
        }
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
    std::optional<tplc::Module>      mod;
    std::optional<tplc::Program>     prog;
    std::optional<tplc::RetainStore> retain;
    TplcProgramDescriptor            d{};

    auto unload_program = [&] {
        /* monitor thread skips non-RUNNING states, so clearing state
           first makes destruction race-free */
        prog_state = PS_EMPTY;
        if (prog) { sched.removeProgram(*prog); prog.reset(); }
        mod.reset();
        if (retain) retain->close();
        d = TplcProgramDescriptor{};
        prog_started = false;
    };

    auto load_program = [&](const std::string& mpath) -> tplc::Status {
        unload_program();
        auto mres = tplc::Module::open(mpath, /*host_caps*/ 0);
        if (!mres.ok()) return mres.st;
        mod.emplace(std::move(mres.value));
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

    tplc::Status lst = load_program(module_path);
    if (!lst.ok()) {
        fprintf(stderr, "tether-plc: program load failed (%d) %s\n",
                lst.code, lst.msg.c_str());
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
        tplc_image_set_fault(&img, (uint64_t)-lst.code);
        tplc_image_close(&img);
        return 1;
    }
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

    tplc::TickHooks hooks;
    hooks.latch = [&img](tplc::Program& p, uint32_t) -> int32_t {
        return tplc_image_latch_inputs(&img, p.imageIn(), 0,
                                       p.desc().image_in_bytes);
    };
    hooks.commit = [&img](tplc::Program& p, uint32_t) -> int32_t {
        return tplc_image_commit_outputs(&img, p.imageOut(), 0,
                                         p.desc().image_out_bytes);
    };
    tplc::CtlServer ctl;
    hooks.fault = [&](tplc::Program& p, uint32_t, int32_t frc) {
            /* doc 02 §4.6 / doc 04 §6: task fault → program policy.
           hold = stay faulted; restart = warm reset bounded by
           restart_max/60s then hold; unload = stop+deschedule.      */
        last_fault = frc;
        tplc_image_set_fault(&img, (uint64_t)-frc);
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_FAULT);
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
        w.bytes(&ps, sizeof ps);
        for (uint32_t i = 0; i < d.task_count; i++) {
            const TplcTaskDesc& td = d.tasks[i];
            TplcCtlTaskStat ts{};
            memcpy(ts.name, td.name, TPLC_TASK_NAME_MAX);
            ts.kind = td.kind;
            ts.state = prog_state == PS_RUNNING ? 1 : 0;
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
        /* STOP halts scheduling only — state is kept (doc 04 §4) */
        TplcCtlProgStateChange sc{(uint8_t)prog_state, PS_STOPPED, 0};
        if (prog_state == PS_RUNNING) {
            prog_state = PS_STOPPED;
            tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
        }
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
            tplc_image_set_plc_status(&img, TPLC_IMG_PLC_RUN);
        }
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
        tplc_image_set_plc_status(&img, TPLC_IMG_PLC_STOPPED);
        TplcCtlProgStateChange sc{PS_EMPTY, PS_STOPPED, 0};
        ctl.reply(fd, t, &sc, sizeof sc, req);
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
        /* package_uuid = first 16B of sha256; store atomically */
        char hex[33]; uuid_hex(got, hex);
        std::string dst = packages_dir + "/" + hex + ".tplc";
        ::close(up.fd); up.fd = -1;
        if (::rename(up.tmp_path.c_str(), dst.c_str()) != 0) {
            ::unlink(up.tmp_path.c_str());
            up.drop();
            ctl.errorReply(fd, t, tplc::TPLC_E_HOST_DLOPEN,
                           "store failed", req);
            return;
        }
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
            sched.tick(now, hooks);
            progress.fetch_add(1);
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

        /* wait: min(next task due, poll cap) */
        uint64_t nd = sched.nextDueNs();
        uint64_t wait_ns = poll_ns;
        if (nd != UINT64_MAX && nd > now)
            wait_ns = (nd - now) < poll_ns ? (nd - now) : poll_ns;
        int wait_ms = (int)(wait_ns / 1000000ull) + 1;
        ctl.poll(wait_ms);
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
