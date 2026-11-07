/* tether-plc-bridge — dev/sim bridge process (doc 09 §4): owns the
 * image as master, runs the map against the sim transport, and
 * supervises a tether-plc child. On real hardware the Tether app
 * instantiates the same library inside its own cyclic path.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/bridge/ImageServer.hpp>
#include <tplc/bridge/MapRunner.hpp>
#include <tplc/bridge/Supervisor.hpp>
#include <tplc/tplc_image.h>

#include <atomic>
#include <csignal>
#include <cstdio>
#include <cstring>
#include <fstream>
#include <sstream>
#include <string>
#include <thread>
#include <vector>
#include <chrono>

static volatile sig_atomic_t g_stop = 0;
static void on_sig(int) { g_stop = 1; }

static uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

static void usage(const char* a0) {
    fprintf(stderr,
        "usage: %s --image <shm_path> --map <file> [options]\n"
        "  --simdir DIR      sim transport directory (default /tmp/tplc_sim)\n"
        "  --hz N            map cycle rate (default 1000)\n"
        "  --plc ARGS…       spawn tether-plc with ARGS (everything after)\n"
        "  --no-plc          no child process (image+map only)\n",
        a0);
}

int main(int argc, char** argv) {
    std::string image_path, map_path, sim_dir = "/tmp/tplc_sim";
    uint64_t hz = 1000;
    std::vector<std::string> plc_args;
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        if (a == "--plc") {
            for (int k = i + 1; k < argc; k++)
                plc_args.push_back(argv[k]);
            break;
        } else if (a == "--image")  image_path = argv[++i];
        else if (a == "--map")      map_path   = argv[++i];
        else if (a == "--simdir")   sim_dir    = argv[++i];
        else if (a == "--hz")       hz = strtoull(argv[++i], 0, 0);
        else if (a == "--no-plc")   { /* marker */ }
        else if (a == "--help" || a == "-h") { usage(argv[0]); return 0; }
        else { usage(argv[0]); return 2; }
    }
    if (image_path.empty() || map_path.empty()) {
        usage(argv[0]); return 2;
    }

    std::ifstream mf(map_path);
    if (!mf) { fprintf(stderr, "map %s unreadable\n", map_path.c_str()); return 1; }
    std::stringstream ss; ss << mf.rdbuf();
    auto mres = tplc::parse_map(ss.str());
    if (!mres.ok()) {
        fprintf(stderr, "map parse: %s\n", mres.st.msg.c_str());
        return 1;
    }
    tplc::IoMap map = std::move(mres.value);

    tplc::ImageServer img;
    tplc::Status ist = img.open(image_path, map.in_size, map.out_size);
    if (!ist.ok()) {
        fprintf(stderr, "image: %s\n", ist.msg.c_str());
        return 1;
    }
    fprintf(stderr, "bridge: image %s (%u in / %u out), %zu channels, "
            "simdir %s\n", image_path.c_str(), map.in_size, map.out_size,
            map.channels.size(), sim_dir.c_str());

    tplc::Supervisor sup({tplc::RESPAWN_ON_FAULT, 1000, 3, 3000});
    bool have_plc = !plc_args.empty();
    if (have_plc) {
        if (sup.spawn(plc_args) < 0) {
            fprintf(stderr, "bridge: spawn failed\n");
            img.close();
            return 1;
        }
        fprintf(stderr, "bridge: spawned tether-plc pid %d\n",
                (int)sup.pid());
    }

    tplc::MapRunner runner(map, img, sim_dir);
    img.setStatus(TPLC_IMG_MASTER_RUN);
    signal(SIGTERM, on_sig);
    signal(SIGINT, on_sig);

    uint64_t period_ns = 1000000000ull / (hz ? hz : 1);
    uint64_t next = mono_ns();
    uint64_t last_log = 0;
    while (!g_stop) {
        uint64_t now = mono_ns();
        img.heartbeat(now);
        /* plc_ok: status RUN is only meaningful while the heartbeat
           is fresh — a dead child leaves its status word stale.
           hb may post-date `now` (plc ticked mid-cycle) — the helper
           treats a future hb as fresh                             */
        bool plc_ok = img.plcStatus() == TPLC_IMG_PLC_RUN;
        if (plc_ok && have_plc &&
            tplc_image_hb_stale(img.plcHbNs(), now, 2000000000ull))
            plc_ok = false;
        runner.cycle(plc_ok || !have_plc);
        if (have_plc) {
            int r = sup.tick(&img, now);
            if (r == 2) {           /* gave up */
                fprintf(stderr, "bridge: supervisor gave up — stopping\n");
                break;
            }
        }
        if (now - last_log > 5000000000ull) {
            last_log = now;
            fprintf(stderr, "bridge: %llu map cycles, plc_status %u\n",
                    (unsigned long long)runner.cycles(), img.plcStatus());
        }
        next += period_ns;
        if (next > now) {
            uint64_t d = next - now;
            struct timespec ts {(time_t)(d / 1000000000ull),
                                (long)(d % 1000000000ull)};
            nanosleep(&ts, nullptr);
        } else next = now;          /* overrun → resync, no catch-up */
    }

    fprintf(stderr, "bridge: stopping\n");
    if (have_plc) sup.stop();
    img.close();
    /* master owns the shm lifetime */
    tplc_image_unlink(image_path.c_str());
    return 0;
}
