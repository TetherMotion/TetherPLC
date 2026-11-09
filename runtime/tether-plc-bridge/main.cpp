/* tether-plc-bridge — dev/sim bridge process (doc 09 §4): owns the
 * image as master, runs the map against the sim transport, and
 * supervises a tether-plc child. On real hardware the Tether app
 * instantiates the same library inside its own cyclic path.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/bridge/ImageServer.hpp>
#include <tplc/bridge/MapRunner.hpp>
#include <tplc/bridge/Supervisor.hpp>
#include <tplc/bridge/CatalogExporter.hpp>
#include <tplc/host/Module.hpp>
#include <tplc/pkg/Package.hpp>
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
        "       %s --catalog <pkg.tplc> [--out FILE]\n"
        "       %s --map-hash <file>\n"
        "  --simdir DIR      sim transport directory (default /tmp/tplc_sim)\n"
        "  --hz N            map cycle rate (default 1000)\n"
        "  --plc ARGS…       spawn tether-plc with ARGS (everything after)\n"
        "  --no-plc          no child process (image+map only)\n",
        a0, a0, a0);
}

/* --map-hash <file>: print the canonical map hash for stamping —
   insert it as a `hash <hex>` line to pin the map's epoch         */
static int do_map_hash(const std::string& map_path) {
    std::ifstream mf(map_path);
    if (!mf) { fprintf(stderr, "map %s unreadable\n", map_path.c_str());
               return 1; }
    std::stringstream ss; ss << mf.rdbuf();
    auto mres = tplc::parse_map(ss.str());
    if (!mres.ok()) {
        fprintf(stderr, "map parse: %s\n", mres.st.msg.c_str());
        return 1;
    }
    printf("%s\n", tplc::map_hash(mres.value).c_str());
    return 0;
}

/* --catalog <pkg>: extract symmap+typc+descriptor → JSON catalog  */
static int do_catalog(const std::string& pkg_path,
                      const std::string& out_path) {
    auto pres = tplc::Package::open(pkg_path);
    if (!pres.ok()) {
        fprintf(stderr, "catalog: %s unreadable: %s\n",
                pkg_path.c_str(), pres.st.msg.c_str());
        return 1;
    }
    tplc::Package pkg = std::move(pres.value);
    tplc::PkgSection smap{}, typc{}, obj{};
    if (!pkg.section(TPLC_SEC_SYMMAP, smap) ||
        !pkg.section(TPLC_SEC_TYPE_CATALOG, typc)) {
        fprintf(stderr, "catalog: package lacks symmap/typecat\n");
        return 1;
    }
    std::string prog = "program";
    std::vector<std::string> tasks;
    if (pkg.section(TPLC_SEC_TARGET_OBJ, obj)) {
        char tp[] = "/tmp/tplc_cat_XXXXXX";
        int tfd = mkstemp(tp);
        if (tfd >= 0 &&
            write(tfd, obj.data, obj.size) == (ssize_t)obj.size) {
            close(tfd);
            auto mres = tplc::Module::open(tp, 0);
            if (mres.ok()) {
                const TplcProgramDescriptor& d =
                    mres.value.descriptor();
                char nm[TPLC_PROG_NAME_MAX + 1]{};
                memcpy(nm, d.name, TPLC_PROG_NAME_MAX);
                if (*nm) prog = nm;
                for (uint32_t i = 0; i < d.task_count; i++) {
                    char tn[TPLC_TASK_NAME_MAX + 1]{};
                    memcpy(tn, d.tasks[i].name, TPLC_TASK_NAME_MAX);
                    tasks.push_back(tn);
                }
            }
        }
        if (tfd >= 0) { close(tfd); unlink(tp); }
    }
    auto cres = tplc::catalog_export(
        (const uint8_t*)smap.data, (uint32_t)smap.size,
        (const uint8_t*)typc.data, (uint32_t)typc.size,
        prog, tasks);
    if (!cres.ok()) {
        fprintf(stderr, "catalog: %s\n", cres.st.msg.c_str());
        return 1;
    }
    std::string j = tplc::catalog_json(cres.value);
    if (out_path.empty()) {
        fputs(j.c_str(), stdout);
    } else {
        FILE* f = fopen(out_path.c_str(), "w");
        if (!f) { perror("out"); return 1; }
        fputs(j.c_str(), f);
        fclose(f);
        fprintf(stderr, "catalog: %zu entries → %s\n",
                cres.value.size(), out_path.c_str());
    }
    return 0;
}

int main(int argc, char** argv) {
    std::string image_path, map_path, sim_dir = "/tmp/tplc_sim";
    std::string catalog_pkg, map_hash_path, out_path;
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
        else if (a == "--catalog")  catalog_pkg = argv[++i];
        else if (a == "--map-hash") map_hash_path = argv[++i];
        else if (a == "--out")      out_path    = argv[++i];
        else if (a == "--no-plc")   { /* marker */ }
        else if (a == "--help" || a == "-h") { usage(argv[0]); return 0; }
        else { usage(argv[0]); return 2; }
    }
    if (!catalog_pkg.empty())
        return do_catalog(catalog_pkg, out_path);
    if (!map_hash_path.empty())
        return do_map_hash(map_hash_path);
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

    /* pdo= channels validate against the live bus catalog the pump
       publishes as <simdir>/.bus; absent file = no bus config, so any
       pdo channel violates MAP_PDO while plain key= maps still pass */
    tplc::BusCatalog bus;
    const tplc::BusCatalog* busp = nullptr;
    {
        std::ifstream bf(sim_dir + "/.bus");
        if (bf) {
            std::stringstream bs; bs << bf.rdbuf();
            auto bres = tplc::parse_bus_catalog(bs.str());
            if (bres.ok()) { bus = std::move(bres.value); busp = &bus; }
            else fprintf(stderr, "bridge: .bus catalog ignored: %s\n",
                         bres.st.msg.c_str());
        }
    }
    auto viol = tplc::validate_map(map, busp);
    if (!viol.empty()) {
        for (auto& x : viol)
            fprintf(stderr, "map: %s (%d): %s\n",
                    x.rule.c_str(), x.code, x.detail.c_str());
        return 1;
    }

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
