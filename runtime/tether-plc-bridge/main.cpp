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

#include <algorithm>
#include <atomic>
#include <cctype>
#include <csignal>
#include <cstdio>
#include <cstdlib>
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
        "       %s --validate <map> [--bus <file>]\n"
        "       %s --gen-map <pkg.tplc> [--bus <file>] [-o FILE]\n"
        "  --simdir DIR      sim transport directory (default /tmp/tplc_sim)\n"
        "  --hz N            map cycle rate (default 1000)\n"
        "  --plc ARGS…       spawn tether-plc with ARGS (everything after)\n"
        "  --no-plc          no child process (image+map only)\n",
        a0, a0, a0, a0, a0);
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

/* --validate <map> [--bus <file>]: parse + validate a deploy map
   against an optional bus catalog; prints violations, exit 1 when
   any.  This is the same check MapRunner startup performs.        */
static int do_validate(const std::string& map_path,
                       const std::string& bus_path) {
    std::ifstream mf(map_path);
    if (!mf) { fprintf(stderr, "map %s unreadable\n", map_path.c_str());
               return 1; }
    std::stringstream ss; ss << mf.rdbuf();
    auto mres = tplc::parse_map(ss.str());
    if (!mres.ok()) {
        fprintf(stderr, "map parse: %s\n", mres.st.msg.c_str());
        return 1;
    }
    tplc::IoMap map = std::move(mres.value);
    tplc::BusCatalog bus;
    const tplc::BusCatalog* busp = nullptr;
    if (!bus_path.empty()) {
        std::ifstream bf(bus_path);
        if (!bf) { fprintf(stderr, "bus catalog %s unreadable\n",
                           bus_path.c_str()); return 1; }
        std::stringstream bs; bs << bf.rdbuf();
        auto bres = tplc::parse_bus_catalog(bs.str());
        if (!bres.ok()) {
            fprintf(stderr, "bus catalog: %s\n", bres.st.msg.c_str());
            return 1;
        }
        bus = std::move(bres.value);
        busp = &bus;
    }
    auto viol = tplc::validate_map(map, busp);
    if (!viol.empty()) {
        for (auto& x : viol)
            fprintf(stderr, "map: %s (%d): %s\n",
                    x.rule.c_str(), x.code, x.detail.c_str());
        return 1;
    }
    printf("map ok: %zu channels, %zu binds, hash %s\n",
           map.channels.size(), map.bindings.size(),
           tplc::map_hash(map).c_str());
    return 0;
}

/* ---- deploy-map generator ------------------------------------------
   Reads the package's IO_MAP section (tplcc build emits the AT-bound
   vars as JSON) and drafts a deploy map: image sizes from binding
   coverage, one file-key channel per contiguous byte run per region,
   bind lines for every %I/%Q binding, `bind <at> *` for %M scratch.
   pdo= endpoints and safe policies are left for the user — the bus
   catalog (when given) is echoed as comments so endpoint names and
   payload sizes are visible while editing.                            */

namespace {

/* minimal JSON reader — IO_MAP payloads are flat object arrays with
   string/number/null values only                                  */
struct JVal {
    enum K { NUL, NUM, STR, BOOL, ARR, OBJ } k = NUL;
    double num = 0;
    bool boolean = false;
    std::string str;
    std::vector<JVal> arr;
    std::vector<std::pair<std::string, JVal>> obj;
    const JVal* find(const char* key) const {
        for (auto& [k, v] : obj)
            if (k == key) return &v;
        return nullptr;
    }
};

struct JParser {
    const char* p;
    bool ok = true;
    void ws() {
        while (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r') p++;
    }
    bool lit(const char* w) {
        size_t n = strlen(w);
        if (strncmp(p, w, n) == 0) { p += n; return true; }
        ok = false; return false;
    }
    JVal parse() {
        ws();
        JVal v;
        switch (*p) {
        case '{': {
            v.k = JVal::OBJ; p++;
            ws();
            if (*p == '}') { p++; return v; }
            for (;;) {
                ws();
                if (*p != '"') { ok = false; return v; }
                std::string key = parse_str();
                ws();
                if (*p != ':') { ok = false; return v; }
                p++;
                JVal val = parse();
                if (!ok) return v;
                v.obj.emplace_back(std::move(key), std::move(val));
                ws();
                if (*p == ',') { p++; continue; }
                if (*p == '}') { p++; return v; }
                ok = false; return v;
            }
        }
        case '[': {
            v.k = JVal::ARR; p++;
            ws();
            if (*p == ']') { p++; return v; }
            for (;;) {
                v.arr.push_back(parse());
                if (!ok) return v;
                ws();
                if (*p == ',') { p++; continue; }
                if (*p == ']') { p++; return v; }
                ok = false; return v;
            }
        }
        case '"':
            v.k = JVal::STR; v.str = parse_str(); return v;
        case 't': lit("true");  v.k = JVal::BOOL; v.boolean = true;  return v;
        case 'f': lit("false"); v.k = JVal::BOOL; v.boolean = false; return v;
        case 'n': lit("null");  return v;
        default: {
            char* end;
            double d = strtod(p, &end);
            if (end == p) { ok = false; return v; }
            p = end;
            v.k = JVal::NUM; v.num = d;
            return v;
        }
        }
    }
    std::string parse_str() {
        /* assumes *p == '"' */
        p++;
        std::string out;
        while (*p && *p != '"') {
            if (*p == '\\' && p[1]) {
                p++;
                switch (*p) {
                case 'n': out += '\n'; break;
                case 't': out += '\t'; break;
                case 'r': out += '\r'; break;
                case '"': out += '"';  break;
                case '\\': out += '\\'; break;
                case '/': out += '/';  break;
                default:  out += *p;   break;
                }
                p++;
            } else {
                out += *p++;
            }
        }
        if (*p == '"') p++;
        return out;
    }
};

struct IoMapEntry {
    std::string name, at, region;
    uint64_t byte = 0, size = 0;
    int bit = -1;
    char width = 'X';
};

static std::string jstr(const JVal* v) {
    return v && v->k == JVal::STR ? v->str : "";
}
static uint64_t jnum(const JVal* v) {
    return v && v->k == JVal::NUM ? (uint64_t)v->num : 0;
}

/* safe filesystem-friendly channel key derived from a binding name */
static std::string chan_key(const std::string& name) {
    std::string k;
    for (char c : name) {
        if (isalnum((unsigned char)c) || c == '_' || c == '-' || c == '.')
            k += c;
        else
            k += '_';
    }
    if (k.empty()) k = "ch";
    return k;
}

/* --gen-map <pkg.tplc> [--bus <file>] [-o <file>] */
static int do_gen_map(const std::string& pkg_path,
                      const std::string& bus_path,
                      const std::string& out_path) {
    auto pres = tplc::Package::open(pkg_path);
    if (!pres.ok()) {
        fprintf(stderr, "gen-map: %s unreadable: %s\n",
                pkg_path.c_str(), pres.st.msg.c_str());
        return 1;
    }
    tplc::Package pkg = std::move(pres.value);
    tplc::PkgSection sec;
    if (!pkg.section(TPLC_SEC_IO_MAP, sec)) {
        fprintf(stderr, "gen-map: package has no IO_MAP section "
                "(rebuild with tplcc build)\n");
        return 1;
    }
    std::string json(reinterpret_cast<const char*>(sec.data), sec.size);
    JParser jp{json.c_str()};
    JVal root = jp.parse();
    if (!jp.ok || root.k != JVal::ARR) {
        fprintf(stderr, "gen-map: IO_MAP section is not a JSON array\n");
        return 1;
    }
    std::vector<IoMapEntry> entries;
    for (auto& e : root.arr) {
        if (e.k != JVal::OBJ) continue;
        IoMapEntry x;
        x.name   = jstr(e.find("name"));
        x.at     = jstr(e.find("at"));
        x.region = jstr(e.find("region"));
        x.byte   = jnum(e.find("byte"));
        x.size   = jnum(e.find("size"));
        if (const JVal* b = e.find("bit"))
            if (b->k == JVal::NUM) x.bit = (int)b->num;
        x.width  = jstr(e.find("width")).empty()
                       ? 'X' : jstr(e.find("width"))[0];
        if (x.size == 0) x.size = 1;
        entries.push_back(std::move(x));
    }
    if (entries.empty()) {
        fprintf(stderr, "gen-map: IO_MAP section is empty — "
                "no AT-bound vars in this package\n");
        return 1;
    }

    /* merge each region's bindings into contiguous byte runs — one
       draft channel per run; the user retargets key= → pdo= per bus
       endpoint and fixes sizes to the PDO payload                */
    tplc::IoMap m;
    struct Run { bool in; uint64_t off, end; std::string key; };
    std::vector<Run> runs;
    for (bool want_in : {true, false}) {
        const char want = want_in ? 'I' : 'Q';
        std::vector<const IoMapEntry*> es;
        for (auto& e : entries)
            if (e.region.size() == 1 && e.region[0] == want) es.push_back(&e);
        std::sort(es.begin(), es.end(),
                  [](const IoMapEntry* a, const IoMapEntry* b) {
                      return a->byte < b->byte ||
                             (a->byte == b->byte && a->size < b->size);
                  });
        for (auto* e : es) {
            uint64_t end = e->byte + e->size;
            if (!runs.empty() && runs.back().in == want_in &&
                e->byte <= runs.back().end) {
                runs.back().end = std::max(runs.back().end, end);
            } else {
                Run r;
                r.in = want_in; r.off = e->byte; r.end = end;
                r.key = std::string(want_in ? "in" : "out") + "_" +
                        chan_key(e->name);
                runs.push_back(r);
            }
        }
    }
    for (auto& r : runs) {
        tplc::MapChannel c;
        c.is_input = r.in;
        c.image_off = (uint32_t)r.off;
        c.size = (uint32_t)(r.end - r.off);
        c.key = r.key;
        if (!c.is_input) { c.safe = tplc::SAFE_ZERO; c.has_safe = true; }
        m.channels.push_back(std::move(c));
    }
    for (auto& e : entries) {
        tplc::MapBinding b;
        b.at_name = e.at.empty() ? "%" + e.region + e.width +
                                    std::to_string(e.byte) : e.at;
        if (e.region == "M") {
            b.key = "*";                    /* marker scratch: unmanaged */
        } else {
            bool in = e.region == "I";
            b.key.clear();
            for (auto& r : runs)
                if (r.in == in && e.byte >= r.off && e.byte < r.end)
                    b.key = r.key;
            if (b.key.empty()) b.key = "*";
        }
        m.bindings.push_back(std::move(b));
    }
    for (auto& e : entries)
        if (e.region == "I" || e.region == "Q") {
            uint64_t end = e.byte + e.size;
            if (end > (e.region == "I" ? m.in_size : m.out_size))
                (e.region == "I" ? m.in_size : m.out_size) = end;
        }

    /* serialize */
    std::string text =
        "# deploy map — generated draft from package IO_MAP\n"
        "# TODO: retarget key= channels to pdo=<endpoint> for the live\n"
        "# bus (sizes must equal the PDO payload), review safe= policies,\n"
        "# then stamp: tether-plc-bridge --map-hash <file>\n";
    char line[512];
    snprintf(line, sizeof line, "image in %u out %u\n",
             m.in_size, m.out_size);
    text += line;
    /* bus catalog echo — endpoint names + payload sizes for pdo= */
    if (!bus_path.empty()) {
        std::ifstream bf(bus_path);
        if (bf) {
            std::stringstream bs; bs << bf.rdbuf();
            auto bres = tplc::parse_bus_catalog(bs.str());
            if (bres.ok() && !bres.value.endpoints.empty()) {
                text += "# bus endpoints (pdo= candidates):\n";
                for (auto& ep : bres.value.endpoints) {
                    snprintf(line, sizeof line, "#   %s %s %u\n",
                             ep.is_tx ? "tx" : "rx",
                             ep.key.c_str(), ep.size);
                    text += line;
                }
            }
        }
    }
    auto emit = [&](const tplc::MapChannel& c) {
        snprintf(line, sizeof line, "%s off=%u size=%u %s=%s",
                 c.is_input ? "in " : "out", c.image_off, c.size,
                 c.pdo.empty() ? "key" : "pdo",
                 (c.pdo.empty() ? c.key : c.pdo).c_str());
        text += line;
        if (c.is_input) {
            text += "\n";
        } else {
            text += " safe=zero\n";
        }
    };
    for (auto& c : m.channels) emit(c);
    for (auto& b : m.bindings) {
        snprintf(line, sizeof line, "bind %s %s\n",
                 b.at_name.c_str(), b.key.c_str());
        text += line;
    }
    /* epoch stamp so the draft is deployable as-is */
    snprintf(line, sizeof line, "hash %s\n",
             tplc::map_hash(m).c_str());
    text += line;

    if (!out_path.empty()) {
        FILE* f = fopen(out_path.c_str(), "w");
        if (!f) { perror("out"); return 1; }
        fwrite(text.data(), 1, text.size(), f);
        fclose(f);
        fprintf(stderr, "gen-map: %zu channels, %zu binds → %s\n",
                m.channels.size(), m.bindings.size(), out_path.c_str());
    } else {
        fputs(text.c_str(), stdout);
    }
    return 0;
}

} /* namespace */

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
    std::string validate_map_path, gen_map_pkg, bus_path;
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
        else if (a == "--validate") validate_map_path = argv[++i];
        else if (a == "--gen-map")  gen_map_pkg = argv[++i];
        else if (a == "--bus")      bus_path   = argv[++i];
        else if (a == "--out" || a == "-o") out_path = argv[++i];
        else if (a == "--no-plc")   { /* marker */ }
        else if (a == "--help" || a == "-h") { usage(argv[0]); return 0; }
        else { usage(argv[0]); return 2; }
    }
    if (!catalog_pkg.empty())
        return do_catalog(catalog_pkg, out_path);
    if (!map_hash_path.empty())
        return do_map_hash(map_hash_path);
    if (!validate_map_path.empty())
        return do_validate(validate_map_path, bus_path);
    if (!gen_map_pkg.empty())
        return do_gen_map(gen_map_pkg, bus_path, out_path);
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
