/* tplc-io — tether_io server exposing the tether-plc control channel.
 * Live attach: any tether_io client (TCP) can invoke the full debug /
 * lifecycle surface and stream live values, with the runtime reached
 * through ctl.sock — the sidecar's stable boundary.
 *
 *   tplc-io --ctl /path/plc.ctl [--port 43770] [--bind 127.0.0.1]
 *
 * Function surface returns raw bytes for flat-typed values (no V6
 * schema required); complex ctl replies decode to JSON strings.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include "tplc_io_internal.hpp"
#include <csignal>

using namespace tplcio_int;
using namespace tether::io;

namespace {

volatile sig_atomic_t g_stop = 0;
void onSig(int) { g_stop = 1; }
} // namespace

static void usage(const char* a0) {
    fprintf(stderr,
        "usage: %s --ctl SOCK [--port 43770] [--bind 127.0.0.1]\n"
        "       [--packages DIR] [--pkg FILE]...\n"
        "       [--no-blob-signal]  (drop the whole-blob Binary\n"
        "         signal from the catalog)\n"
        "  --packages DIR  scan DIR/*.tplc for the running program's\n"
        "                  package (TYPC types for plc.* field entries)\n"
        "  --pkg FILE      extra .tplc to try (boot --package loads)\n", a0);
}

/* Populate the whole registry: status signals, epoch marker, blob
 * signal, the ctl function surface, and the dynamic plc.* field
 * entries for the active program. Called before the server starts
 * and again after each program-set epoch change — registry mutation
 * is only safe with no sessions attached, so the watchdog stops the
 * server first (joins in-flight callbacks), clears, repopulates, and
 * rebinds; clients see a brief disconnect then re-list. */
/* returns true when the catalog may be incomplete — no program name
   yet or the field table unreadable — so the caller keeps polling
   DBG_FIELDS and rebuilds once the layout reappears (deploy
   stop→prepare→start races the epoch bump)                     */
static bool populateRegistry(Registry& registry, uint64_t epoch,
                             bool want_blob_sig,
                             const std::string& packagesDir,
                             const std::vector<std::string>& pkgFiles) {
    /* ids live in the registry's shared idMap_ (params, signals and
       functions compete for the same id space) and the map is cleared
       on every rebuild — so the counters restart with each populate.
       Letting them run process-wide eventually pushes function ids
       (g_fnId) into the signal range and addParam/addSignal start
       failing on phantom collisions.                            */
    g_fnId = 100;
    g_sigId = 200;
    bool fieldsMissing = false;
    /* live signals: status scalars every session can stream/poll   */
    registry.addSignal(sig("status.state", "0 stop 1 run 2 fault 3 empty",
        ValueType::U8,
        scalarRd<uint8_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.state : (uint8_t)0xFF; })));
    registry.addSignal(sig("status.cycles", "program cycle counter",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.cycles : 0; })));
    registry.addSignal(sig("status.epoch",
        "management epoch (bumps on program-set change)",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.epoch : 0; })));
    registry.addSignal(sig("status.fault_code", "last fault rc",
        ValueType::I32,
        scalarRd<int32_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.last_fault_code : 0; })));
    registry.addSignal(sig("status.force_count", "active forces",
        ValueType::U32,
        scalarRd<uint32_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.force_count : 0; })));
    registry.addSignal(sig("status.swap_stage",
        "0 none 1 quarantine",
        ValueType::U8,
        scalarRd<uint8_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.swap_stage : (uint8_t)0; })));
    registry.addSignal(sig("shadow.state", "0 none 1 running",
        ValueType::U8,
        scalarRd<uint8_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_state : (uint8_t)0; })));
    registry.addSignal(sig("shadow.cycles", "candidate task runs",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_cycles : 0; })));
    registry.addSignal(sig("shadow.missed",
        "budget-skipped candidate cycles",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_missed : 0; })));
    registry.addSignal(sig("shadow.diff_count",
        "total %Q diff entries",
        ValueType::U64,
        scalarRd<uint64_t>([](TplcCtlProgStatus& ps, bool ok) {
            return ok ? ps.shadow_diff_count : 0; })));

    if (epoch) {
        SignalEntry m;
        m.id = g_sigId++;
        m.name = "epoch." + std::to_string(epoch);
        m.description =
            "program-set generation marker (epoch bump)";
        m.group = "plc";
        m.valueType = ValueType::U64;
        m.readFn = [ep = epoch](void* dst) {
            uint64_t v = ep;
            memcpy(dst, &v, 8);
        };
        assignSchema(m);
        registry.addSignal(std::move(m));
    }

    /* the whole program blob as one streamable Binary signal —
       clients slice offsets locally via plc.dbg.fields            */
    if (want_blob_sig) {
        std::vector<uint8_t> st;
        tplcio::CtlClient::Error e;
        uint64_t blob = 0;
        if (g_ctl.call(TPLC_OP_DBG_STATE, st, &e) && st.size() >= 24)
            memcpy(&blob, st.data() + 16, 8);
        if (blob && blob <= (1u << 20)) {
            SignalEntry b;
            b.id = g_sigId++;
            b.name = "blob";
            b.description =
                "whole program blob (post-scan boundary image)";
            b.group = "plc";
            b.valueType = ValueType::Binary;
            /* schema-wire Bytes = varint len + payload; advertise the
               encoded size                                              */
            b.maxValueSize = (uint16_t)(blob + 10 > 0xFFFF
                                            ? 0xFFFF : blob + 10);
            b.varReadFn = [](void* dst, size_t maxLen) -> size_t {
                W w; w.u64(0);
                w.u32((uint32_t)(maxLen > 10 ? maxLen - 10 : 0));
                std::vector<uint8_t> out;
                tplcio::CtlClient::Error e;
                if (!g_ctl.call(TPLC_OP_DBG_READ, w.b.data(), 12,
                                out, &e))
                    return 0;
                const SchemaNode* n = nodeFor(ValueType::Binary);
                auto enc = encodeNative(g_graph, *n,
                    nv::bytes(std::span<const uint8_t>(out)));
                if (!enc || enc->size() > maxLen) return 0;
                memcpy(dst, enc->data(), enc->size());
                return enc->size();
            };
            assignSchema(b);
            registry.addSignal(std::move(b));
        }
    }

    for (auto& f : buildFunctions())
        registry.addFunction(std::move(f));

    /* dynamic plc.<prog>.<var> entries for the active program      */
    TplcCtlProgStatus ps{};
    std::vector<uint8_t> raw;
    bool haveProg = progStatus(ps, raw) && ps.name[0];
    if (haveProg) {
        size_t nl = strnlen((const char*)ps.name, TPLC_PROG_NAME_MAX);
        std::string prog((const char*)ps.name, nl);
        auto fields = fetchPlcFields();
        /* a deploy swaps stop→prepare→start inside one watchdog
           tick; if the epoch bump landed mid-swap the layout is
           briefly absent and DBG_FIELDS errors — retry briefly so
           the rebuilt catalog doesn't lose every plc.* field      */
        for (int i = 0; fields.empty() && i < 5; i++) {
            struct timespec ts{0, 200 * 1000 * 1000};
            nanosleep(&ts, nullptr);
            fields = fetchPlcFields();
        }
        fieldsMissing = fields.empty();
        if (!fields.empty()) {
            auto types = typcTypesForProgram(ps, packagesDir, pkgFiles);
            addPlcFieldEntries(registry, prog, fields, types);
        }
    }
    return !haveProg || fieldsMissing;
}

int main(int argc, char** argv) {
    std::string ctl_path, bind = "127.0.0.1", packages_dir;
    std::vector<std::string> pkg_files;
    uint16_t port = 43770;
    bool want_blob_sig = true;
    for (int i = 1; i < argc; i++) {
        std::string a = argv[i];
        if (a == "--ctl")       ctl_path = argv[++i];
        else if (a == "--port") port = (uint16_t)atoi(argv[++i]);
        else if (a == "--bind") bind = argv[++i];
        else if (a == "--packages") packages_dir = argv[++i];
        else if (a == "--pkg")   pkg_files.push_back(argv[++i]);
        else if (a == "--no-blob-signal") want_blob_sig = false;
        else { usage(argv[0]); return 2; }
    }
    if (ctl_path.empty()) { usage(argv[0]); return 2; }

    signal(SIGINT, onSig);
    signal(SIGTERM, onSig);

    if (!g_ctl.connectSock(ctl_path, 10000)) {
        fprintf(stderr, "tplc-io: ctl connect %s failed\n",
                ctl_path.c_str());
        return 1;
    }

    buildSchema();
    Registry registry;

    TplcCtlProgStatus ps0{};
    std::vector<uint8_t> raw0;
    uint64_t last_epoch = 0;
    if (progStatus(ps0, raw0)) last_epoch = ps0.epoch;
    /* field_retry_deadline (nowUs): while set, the watchdog polls
       DBG_FIELDS cheaply and rebuilds once the layout reappears —
       covers deploy swaps that outlast the in-populate retry      */
    uint64_t field_retry_deadline =
        populateRegistry(registry, last_epoch, want_blob_sig,
                         packages_dir, pkg_files)
            ? nowUs() + 15000000
            : 0;

    ServerConfig cfg;
    cfg.timestampFn = nowUs;
    cfg.maxClients = 8;
    cfg.schemaCatalog = &g_catalog;
    auto mkServer = [&]() {
        auto tsrv = std::make_unique<TcpTransportServer>(port, bind);
        return std::make_unique<Server>(registry, std::move(tsrv), cfg);
    };
    auto server = mkServer();
    if (!server->start()) {
        fprintf(stderr, "tplc-io: listen %s:%u failed\n",
                bind.c_str(), port);
        return 1;
    }
    fprintf(stderr,
            "tplc-io: tether_io on %s:%u → %s "
            "(%u functions, %u signals)\n",
            bind.c_str(), port, ctl_path.c_str(),
            (unsigned)(g_fnId - 100), (unsigned)(g_sigId - 200));

    /* management-epoch → catalog rebuild: when the runtime's
       program-set epoch changes, re-install the schema catalog
       (epoch bump → stale client epochs are rejected), rebuild the
       registry (dynamic plc.* fields track the new program), and
       rebind the listener so no session ever observes a half-built
       catalog. Clients see a brief disconnect, then CatalogChanged. */
    while (!g_stop) {
        /* ctl link watchdog: reconnect keeps the surface live
           across runtime restarts                                   */
        if (!g_ctl.connected()) {
            if (!g_ctl.connectSock(ctl_path, 2000))
                fprintf(stderr, "tplc-io: ctl reconnect failed\n");
        } else {
            TplcCtlProgStatus ps{};
            std::vector<uint8_t> raw;
            auto rebuild = [&](uint64_t epoch, const char* why) {
                g_catalog.install(g_graph, plcManifest(g_graph));
                server->stop();
                registry.clear();
                field_retry_deadline =
                    populateRegistry(registry, epoch, want_blob_sig,
                                     packages_dir, pkg_files)
                        ? nowUs() + 15000000
                        : 0;
                if (field_retry_deadline)
                    fprintf(stderr,
                            "tplc-io: catalog incomplete, "
                            "retrying fields for 15s\n");
                server = mkServer();
                if (server->start()) {
                    fprintf(stderr, "tplc-io: %s, catalog rebuilt\n",
                            why);
                } else {
                    fprintf(stderr,
                            "tplc-io: rebind %s:%u failed\n",
                            bind.c_str(), port);
                }
            };
            if (progStatus(ps, raw) && ps.epoch != last_epoch) {
                last_epoch = ps.epoch;
                char why[96];
                snprintf(why, sizeof why, "mgmt epoch → %llu",
                         (unsigned long long)ps.epoch);
                rebuild(ps.epoch, why);
            } else if (field_retry_deadline) {
                /* last rebuild caught the program mid-swap; poll the
                   field table cheaply and rebuild once it is back   */
                if (nowUs() >= field_retry_deadline) {
                    field_retry_deadline = 0;
                } else if (!fetchPlcFields().empty()) {
                    field_retry_deadline = 0;
                    rebuild(last_epoch, "program fields restored");
                }
            }
        }
        struct timespec ts{0, 500 * 1000 * 1000};
        nanosleep(&ts, nullptr);
    }
    server->stop();
    return 0;
}
