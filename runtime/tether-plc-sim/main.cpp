/* tether-plc-sim — L2 process-sim runner (doc 14 §1): publishes the
 * image as master, spawns the real tether-plc with --sim, then steps
 * a deterministic scenario — sim clock + %I writes in, %Q captures
 * out — and diffs expectations. Divergence is reported per step and
 * byte offset, exit 1 on any mismatch (TPLC_E_SIM_DIVERGENCE class).
 *
 * step-file line format:
 *   step <idx> time=<ns> [in <off>=<hex>[,<hex>..]] [expect <off>=<hex>..]
 *              [expectf <blob_off>=<hex>..]
 *   `expect` compares process-image %Q bytes; `expectf` reads back
 *   program-blob bytes through the ctl channel (DBG_READ), so field
 *   expectations like timer ET/CV values can be checked. expectf
 *   requires --ctl to point at a real socket path.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_image.h>
#include "../../tools/ctl_client.h"

#include <csignal>
#include <cstdio>
#include <cstring>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <unistd.h>
#include <sys/wait.h>

static volatile sig_atomic_t g_stop = 0;
static void on_sig(int) { g_stop = 1; }

static uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

/* one ctl round-trip: send `type` with `plen` bytes, skip pushed
   event frames, return reply length or -1 on error. rbuf/rcap hold
   the reply payload.                                                    */
static int ctl_call(int fd, uint16_t type, const void* pl, uint32_t plen,
                    uint8_t* rbuf, uint32_t rcap) {
    if (ctl_send(fd, type, pl, plen) != 0) return -1;
    for (int tr = 0; tr < 8; tr++) {
        uint32_t len = rcap;
        uint16_t fl = 0;
        uint64_t req = 0;
        int t = ctl_recv(fd, rbuf, &len, &fl, &req);
        if (t < 0) return -1;
        if (t == type && !(fl & TPLC_CTL_F_ERROR))
            return (int)len;
        if (fl & TPLC_CTL_F_ERROR) return -2;
    }
    return -1;
}

struct Step {
    uint32_t idx = 0;
    uint64_t time_ns = 0;
    std::vector<std::pair<uint32_t,std::vector<uint8_t>>> in, expect,
                                                          expectf;
};

static bool hex_bytes(const std::string& s, std::vector<uint8_t>& out) {
    for (size_t i = 0; i + 1 < s.size(); i += 3) {
        unsigned v;
        if (sscanf(s.c_str() + i, "%02x", &v) != 1) return false;
        out.push_back((uint8_t)v);
    }
    return true;
}

static bool parse_steps(const std::string& path,
                        std::vector<Step>& steps) {
    std::ifstream f(path);
    if (!f) return false;
    std::string line;
    Step cur;
    bool have = false;
    auto flush = [&] { if (have) steps.push_back(cur); cur = Step{}; };
    uint32_t ln = 0;
    while (std::getline(f, line)) {
        ln++;
        size_t h = line.find('#');
        if (h != std::string::npos) line.resize(h);
        std::istringstream is(line);
        std::string w;
        std::vector<std::string> t;
        while (is >> w) t.push_back(w);
        if (t.empty()) continue;
        if (t[0] != "step") { fprintf(stderr, "line %u: want step\n", ln); return false; }
        flush();
        have = true;
        cur.idx = (uint32_t)strtoul(t.size() > 1 ? t[1].c_str() : "0", 0, 0);
        for (size_t i = 2; i < t.size(); i++) {
            std::string& w2 = t[i];
            if (w2.compare(0, 5, "time=") == 0) {
                cur.time_ns = strtoull(w2.c_str() + 5, 0, 0);
            } else if (w2 == "in" || w2 == "expect" || w2 == "expectf") {
                if (i + 1 >= t.size()) return false;
                std::string spec = t[++i];
                size_t eq = spec.find('=');
                if (eq == std::string::npos) return false;
                uint32_t off = (uint32_t)strtoul(spec.c_str(), 0, 0);
                std::vector<uint8_t> bytes;
                if (!hex_bytes(spec.substr(eq + 1), bytes)) {
                    fprintf(stderr, "line %u: bad hex\n", ln);
                    return false;
                }
                auto& dst = w2 == "in" ? cur.in
                          : w2 == "expect" ? cur.expect : cur.expectf;
                dst.push_back({off, bytes});
            }
        }
    }
    flush();
    return true;
}

int main(int argc, char** argv) {
    std::string image_path, pkg_path, steps_path, plc_bin;
    std::string ctl_path = "none";
    uint32_t in_size = 64, out_size = 64;
    int wait_running_ms = 5000;
    int hold_ms = 0;
    int settle_ms = 0;
    int scan_mode = 0;   /* --scan: one ctl TASK_STEP per step      */
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        if      (a == "--image")  image_path = argv[++i];
        else if (a == "--package") pkg_path   = argv[++i];
        else if (a == "--steps")  steps_path  = argv[++i];
        else if (a == "--plc")    plc_bin     = argv[++i];
        else if (a == "--ctl")    ctl_path    = argv[++i];
        else if (a == "--hold-ms") hold_ms    = atoi(argv[++i]);
        else if (a == "--settle-ms") settle_ms = atoi(argv[++i]);
        else if (a == "--scan")   scan_mode  = 1;
        else if (a == "--in")     in_size     = (uint32_t)strtoul(argv[++i],0,0);
        else if (a == "--out")    out_size    = (uint32_t)strtoul(argv[++i],0,0);
        else if (a == "--help" || a == "-h") {
            fprintf(stderr,
                "usage: %s --image X --package P --steps F "
                "[--plc PATH] [--ctl PATH] [--in N] [--out N] "
                "[--scan] [--settle-ms N] [--hold-ms N]\n",
                argv[0]);
            return 0;
        }
    }
    if (image_path.empty() || pkg_path.empty() || steps_path.empty()) {
        fprintf(stderr, "--image, --package, --steps required\n");
        return 2;
    }
    if (plc_bin.empty()) plc_bin = "./build/tether-plc";

    std::vector<Step> steps;
    if (!parse_steps(steps_path, steps)) {
        fprintf(stderr, "sim: cannot parse %s\n", steps_path.c_str());
        return 1;
    }

    TplcImage img;
    if (tplc_image_create(&img, image_path.c_str(), in_size, out_size)) {
        fprintf(stderr, "sim: image create failed\n");
        return 1;
    }
    tplc_image_set_master_status(&img, TPLC_IMG_MASTER_RUN);

    pid_t pid = fork();
    if (pid == 0) {
        execlp(plc_bin.c_str(), plc_bin.c_str(),
               "--image", image_path.c_str(),
               "--package", pkg_path.c_str(),
               "--ctl", ctl_path.c_str(), "--sim", (char*)nullptr);
        _exit(127);
    }
    if (pid < 0) { fprintf(stderr, "sim: fork failed\n"); return 1; }

    signal(SIGTERM, on_sig);
    signal(SIGINT, on_sig);

    /* wait for plc RUN */
    int rc = 1;
    uint64_t t0 = mono_ns();
    while (mono_ns() - t0 < (uint64_t)wait_running_ms * 1000000ull) {
        if (tplc_image_plc_status(&img) == TPLC_IMG_PLC_RUN) break;
        int st;
        if (waitpid(pid, &st, WNOHANG) == pid) {
            fprintf(stderr, "sim: plc exited during startup\n");
            goto out;
        }
        usleep(5000);
    }
    if (tplc_image_plc_status(&img) != TPLC_IMG_PLC_RUN) {
        fprintf(stderr, "sim: plc never reached RUN\n");
        goto out;
    }
    fprintf(stderr, "sim: plc running, %zu steps\n", steps.size());

    /* settle window: the plc is up but the sim clock is still 0, so
       ctl clients can arm watches/traces before the scenario runs */
    if (settle_ms > 0) usleep((useconds_t)settle_ms * 1000);

    {
        uint32_t divergent = 0;
        int ctl_fd = -1;   /* lazy ctl connection for expectf/scan */
        auto ctl_ensure = [&]() -> int {
            if (ctl_fd < 0 && ctl_path != "none") {
                ctl_fd = ctl_connect(ctl_path.c_str(), 3000);
                if (ctl_fd >= 0 &&
                    ctl_hello(ctl_fd, "tether-plc-sim") != 0) {
                    close(ctl_fd);
                    ctl_fd = -1;
                }
            }
            return ctl_fd;
        };
        /* scan mode: hold task 0, then each step runs exactly one
           scan via TASK_STEP — cycle-per-step semantics matching
           the oracle vectors (L1 interp) on the real runtime     */
        if (scan_mode) {
            uint8_t rb[64];
            uint32_t idx = 0;
            if (ctl_ensure() < 0 ||
                ctl_call(ctl_fd, TPLC_OP_TASK_SUSPEND, &idx, 4,
                         rb, sizeof rb) < 0) {
                fprintf(stderr, "sim: --scan needs a live ctl "
                        "socket and a suspendable task\n");
                goto out;
            }
        }
        std::vector<uint8_t> obuf(out_size);
        for (auto& s : steps) {
            if (g_stop) break;
            uint64_t seq0 = tplc_img_load_u64(&img.hdr->out_seq);
            tplc_image_sim_set_now(&img, s.time_ns);
            tplc_image_master_heartbeat(&img, mono_ns());
            for (auto& [off, bytes] : s.in)
                tplc_image_write_inputs(&img, bytes.data(), off,
                                        (uint32_t)bytes.size());
            if (scan_mode) {
                uint8_t rq[8], rb[64];
                uint32_t zero = 0, one = 1;
                memcpy(rq, &zero, 4);
                memcpy(rq + 4, &one, 4);
                if (ctl_call(ctl_fd, TPLC_OP_TASK_STEP, rq, sizeof rq,
                             rb, sizeof rb) < 0) {
                    fprintf(stderr, "step %u: TASK_STEP failed\n",
                            s.idx);
                    divergent++;
                    continue;
                }
            }
            /* wait for the plc to commit at least one new scan */
            uint64_t wt = mono_ns();
            while (tplc_img_load_u64(&img.hdr->out_seq) == seq0 &&
                   mono_ns() - wt < 2000000000ull)
                usleep(500);
            int rc2 = tplc_image_read_outputs(&img, obuf.data(), 0,
                                              out_size);
            if (rc2 != 0) {
                fprintf(stderr, "step %u: read_outputs rc=%d\n",
                        s.idx, rc2);
                divergent++;
                continue;
            }
            for (auto& [off, exp] : s.expect) {
                if (off + exp.size() > out_size) continue;
                if (memcmp(obuf.data() + off, exp.data(),
                           exp.size()) != 0) {
                    divergent++;
                    fprintf(stderr,
                            "step %u: DIVERGENT out[%u] expected ",
                            s.idx, off);
                    for (auto b : exp) fprintf(stderr, "%02x", b);
                    fprintf(stderr, " got ");
                    for (size_t k = 0; k < exp.size(); k++)
                        fprintf(stderr, "%02x", obuf[off + k]);
                    fprintf(stderr, "\n");
                }
            }
            /* blob-field expectations via ctl DBG_READ */
            if (!s.expectf.empty()) {
                if (ctl_ensure() < 0) {
                    fprintf(stderr, "step %u: expectf needs a live "
                                    "ctl socket (--ctl)\n", s.idx);
                    divergent += (uint32_t)s.expectf.size();
                } else {
                    for (auto& [off, exp] : s.expectf) {
                        uint8_t rq[12], rb[4096];
                        uint64_t off64 = off;
                        uint32_t rl = (uint32_t)exp.size();
                        memcpy(rq, &off64, 8);
                        memcpy(rq + 8, &rl, 4);
                        int blen = ctl_call(ctl_fd, TPLC_OP_DBG_READ,
                                            rq, sizeof rq, rb,
                                            sizeof rb);
                        if (blen == (int)exp.size() &&
                            memcmp(rb, exp.data(), exp.size()) == 0)
                            continue;
                        divergent++;
                        fprintf(stderr, "step %u: DIVERGENT field[%u] "
                                        "expected ", s.idx, off);
                        for (auto b : exp) fprintf(stderr, "%02x", b);
                        fprintf(stderr, " got ");
                        if (blen > 0)
                            for (int k = 0;
                                 k < blen && k < (int)exp.size(); k++)
                                fprintf(stderr, "%02x", rb[k]);
                        else
                            fprintf(stderr, "<dbg-read rc=%d>", blen);
                        fprintf(stderr, "\n");
                    }
                }
            }
            int st;
            if (waitpid(pid, &st, WNOHANG) == pid) {
                fprintf(stderr, "step %u: plc died\n", s.idx);
                divergent++;
                break;
            }
        }
        if (divergent == 0) {
            fprintf(stderr, "sim: %zu steps all matched\n",
                    steps.size());
            rc = 0;
        } else {
            fprintf(stderr, "sim: %u divergent check(s)\n", divergent);
        }
        /* keep the plc alive so external ctl clients can inspect
           the frozen post-scenario state (debug-plane parity) */
        if (hold_ms > 0) usleep((useconds_t)hold_ms * 1000);
        if (ctl_fd >= 0) close(ctl_fd);
    }

out:
    kill(pid, SIGTERM);
    int st;
    for (int i = 0; i < 100 && waitpid(pid, &st, WNOHANG) == 0; i++)
        usleep(20000);
    if (waitpid(pid, &st, WNOHANG) == 0) { kill(pid, SIGKILL); waitpid(pid, &st, 0); }
    tplc_image_close(&img);
    tplc_image_unlink(image_path.c_str());
    return rc;
}
