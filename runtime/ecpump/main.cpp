/* tplc-ec-pump — EtherCAT master pump for the emulated Beckhoff fleet.
 *
 * Connects to beckhoff_slave_chain over a virtual link as an
 * EtherCAT master (Tether examples/common FleetMaster bring-up) — UDP
 * by default, or raw L2 frames on a kernel interface via --iface —
 * then bridges the fleet's PDO entries to files so the
 * tether-plc-bridge simdir transport can carry them into the PLC
 * process image:
 *
 *   <key>.tx   TxPDO image (slave inputs; pump writes, bridge reads)
 *   <key>.rx   RxPDO image (slave outputs; bridge writes, pump reads)
 *
 * The master-side PDO views are also exposed over Tether IO so tests
 * can observe what the master received/sent without touching files:
 *   signals master.<key>.di<g> / .ai<c>   (TxPDO, slave -> master)
 *   signals master.<key>.do<g> / .ao<c>   (RxPDO, master -> slave)
 *   signals master.link / .cycles / .errors
 *
 * usage: tplc-ec-pump (--peer HOST --port PORT | --iface NAME)
 *                    --simdir DIR
 *                    [--io-port N] [--io-bind ADDR] [--hz N] [--time S]
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include "common/BeckhoffFleet.hpp"
#include "common/EthercatFleetMaster.hpp"

#include "tether/io/Registry.hpp"
#include "tether/io/SchemaCatalog.hpp"
#include "tether/io/SchemaDigest.hpp"
#include "tether/io/Server.hpp"
#include "tether/io/TcpTransport.hpp"

#include <arpa/inet.h>

#include <atomic>
#include <chrono>
#include <csignal>
#include <cstdarg>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <memory>
#include <string>
#include <thread>
#include <vector>

using namespace tether::examples;
using namespace tether::io;

namespace {

std::atomic<bool> g_stop{false};
std::atomic<bool> g_link{false};
std::atomic<uint64_t> g_cycles{0};
std::atomic<uint64_t> g_errors{0};
void onSigint(int) { g_stop = true; }

uint64_t nowUs() {
    return static_cast<uint64_t>(std::chrono::duration_cast<
        std::chrono::microseconds>(
            std::chrono::steady_clock::now().time_since_epoch()).count());
}

void pumpLog(const char* level, const char* msg, ...) {
    va_list ap;
    va_start(ap, msg);
    std::fprintf(stderr, "[ec-pump:%s] ", level);
    std::vfprintf(stderr, msg, ap);
    std::fprintf(stderr, "\n");
    va_end(ap);
}

/* ---- PDO<->file pump ---------------------------------------------------- */

struct PdoLink {
    std::string key;        /* fleet key, e.g. "el1004.c1"             */
    bool        txpdo;      /* true: TxPDO (.tx file), false: RxPDO    */
    int         entry;      /* PDOManager entry index                  */
    uint16_t    size;       /* PDO payload bytes                       */
};

bool writeFileAtomic(const std::filesystem::path& path,
                     const uint8_t* data, size_t len) {
    const auto tmp = path.string() + ".tmp";
    FILE* f = std::fopen(tmp.c_str(), "wb");
    if (!f) return false;
    const bool ok = std::fwrite(data, 1, len, f) == len;
    std::fclose(f);
    if (!ok) { std::remove(tmp.c_str()); return false; }
    return std::rename(tmp.c_str(), path.c_str()) == 0;
}

bool readFile(const std::filesystem::path& path,
              uint8_t* dst, size_t len) {
    FILE* f = std::fopen(path.c_str(), "rb");
    if (!f) return false;
    const size_t n = std::fread(dst, 1, len, f);
    std::fclose(f);
    return n == len;
}

/* ---- Tether IO surface (master-side PDO views) --------------------------- */

struct IoSchemas {
    SchemaGraph graph;
    std::vector<SchemaManifestEntry> manifest;

    SchemaRef addScalar(uint8_t nodeIdx, uint8_t kind, uint8_t channel,
                        ValueType type, const std::string& name) {
        SchemaNode n;
        n.key = {};
        n.key[0] = 0xB3;                    // tplc ec-pump namespace
        n.key[1] = nodeIdx;
        n.key[2] = kind;
        n.key[3] = channel;
        n.kind = SchemaKind::Scalar;
        n.scalarType = type;
        n.name = name;
        SchemaRef ref{n.key, computeSchemaDigest(n)};
        manifest.push_back({ref, 1});
        graph.nodes.push_back(std::move(n));
        return ref;
    }
};

struct PendingSignal {
    SignalEntry signal;
    SchemaRef    ref;
};

} // namespace

int main(int argc, char** argv) {
    std::string peer = "127.0.0.1", simdir, ioBind = "127.0.0.1", iface;
    uint16_t port = 47820, ioPort = 0;
    uint32_t hz = 500;
    double runSecs = 0;
    for (int i = 1; i < argc; ++i) {
        const std::string a = argv[i];
        auto need = [&](const char* name) -> const char* {
            if (i + 1 >= argc) {
                std::fprintf(stderr, "%s needs a value\n", name);
                std::exit(2);
            }
            return argv[++i];
        };
        if      (a == "--peer")    peer    = need("--peer");
        else if (a == "--port")    port    = (uint16_t)std::stoul(need("--port"));
        else if (a == "--iface")   iface   = need("--iface");
        else if (a == "--simdir")  simdir  = need("--simdir");
        else if (a == "--io-port") ioPort  = (uint16_t)std::stoul(need("--io-port"));
        else if (a == "--io-bind") ioBind  = need("--io-bind");
        else if (a == "--hz")      hz      = (uint32_t)std::stoul(need("--hz"));
        else if (a == "--time")    runSecs = std::stod(need("--time"));
        else if (a == "--help" || a == "-h") {
            std::fprintf(stderr,
                "usage: %s (--peer HOST --port PORT | --iface NAME) "
                "--simdir DIR "
                "[--io-port N] [--io-bind ADDR] [--hz N] [--time S]\n",
                argv[0]);
            return 0;
        } else {
            std::fprintf(stderr, "unknown arg %s\n", a.c_str());
            return 2;
        }
    }
    if (simdir.empty() || hz == 0) {
        std::fprintf(stderr, "--simdir required, --hz must be > 0\n");
        return 2;
    }
    /* the pump owns the directory it publishes into — the bridge only
       mkdirs lazily at MapRunner construction, which is too late for
       the one-shot .bus catalog write below                      */
    std::error_code ec;
    std::filesystem::create_directories(simdir, ec);
    if (ec) {
        std::fprintf(stderr, "tplc-ec-pump: simdir %s: %s\n",
                     simdir.c_str(), ec.message().c_str());
        return 1;
    }

    signal(SIGINT, onSigint);
    signal(SIGTERM, onSigint);

    const auto infos = fleetInfos();

    FleetMaster fm;
    const bool linkOk = iface.empty() ? fm.open(peer, port)
                                      : fm.openIface(iface);
    if (!linkOk) {
        std::fprintf(stderr, "tplc-ec-pump: link open failed\n");
        return 1;
    }

    std::vector<EtherCAT::DiscoveredSlave> discovered;
    const int n = fm.bringUp(infos, &discovered);
    if (n < 0 || static_cast<size_t>(n) != infos.size()) {
        std::fprintf(stderr,
            "tplc-ec-pump: bring-up failed (%d of %zu slaves)\n",
            n, infos.size());
        return 1;
    }
    int identityFails = 0;
    for (size_t i = 0; i < discovered.size() && i < infos.size(); ++i) {
        const auto& d = discovered[i];
        const auto& inf = infos[i];
        auto eq = [](const std::optional<uint32_t>& got, uint32_t want,
                     const char* what, size_t idx) {
            if (!got || *got != want) {
                std::fprintf(stderr,
                    "tplc-ec-pump: slave %zu %s mismatch\n",
                    idx, what);
                return false;
            }
            return true;
        };
        if (!eq(d.vendor_id, inf.vendorId, "vendor", i) ||
            !eq(d.product_code, inf.productCode, "product", i) ||
            !eq(d.revision_number, beckhoff_ids::kRevision, "revision", i))
            ++identityFails;
    }
    if (identityFails) {
        std::fprintf(stderr,
            "tplc-ec-pump: %d identity check(s) failed\n", identityFails);
        return 1;
    }
    std::fprintf(stderr, "tplc-ec-pump: %d slaves in OP\n", n);

    // PDO entry map for every I/O module (couplers carry no process data).
    std::vector<PdoLink> links;
    for (const auto& inf : infos) {
        if (inf.isCoupler) continue;
        for (bool tx : {true, false}) {
            const uint16_t size = tx ? inf.txSize : inf.rxSize;
            if (!size) continue;
            const int e = fm.findEntry(
                static_cast<uint16_t>(inf.chainIndex), tx);
            if (e < 0) {
                std::fprintf(stderr,
                    "tplc-ec-pump: no %s PDO entry for %s\n",
                    tx ? "Tx" : "Rx", inf.key.c_str());
                return 1;
            }
            links.push_back({inf.key, tx, e, size});
        }
    }

    // Bus catalog for map validation (TPLC_E_MAP_PDO): the bridge's
    // pdo= channels are checked against <simdir>/.bus.  Static for the
    // life of the pump — the fleet topology does not change.
    {
        std::string cat = "# tplc-ec-pump bus catalog v1\n";
        for (const auto& l : links) {
            cat += l.txpdo ? "tx " : "rx ";
            cat += l.key + " " + std::to_string(l.size) + "\n";
        }
        const auto busPath = std::filesystem::path(simdir) / ".bus";
        const auto tmp = busPath.string() + ".tmp";
        if (FILE* f = std::fopen(tmp.c_str(), "wb")) {
            std::fwrite(cat.data(), 1, cat.size(), f);
            std::fclose(f);
            std::rename(tmp.c_str(), busPath.c_str());
        }
    }

    // --- Tether IO server: master-side PDO views ---------------------------
    IoSchemas ioSchemas;
    std::vector<PendingSignal> pending;
    uint64_t nextId = 1;
    auto addSig = [&](uint8_t nodeIdx, uint8_t kind, uint8_t channel,
                      ValueType t, const std::string& name,
                      ReadFn r) {
        PendingSignal e;
        e.ref = ioSchemas.addScalar(nodeIdx, kind, channel, t, name);
        e.signal.id = nextId++;
        e.signal.name = name;
        e.signal.group = "master";
        e.signal.description = name;
        e.signal.valueType = t;
        e.signal.readFn = std::move(r);
        pending.push_back(std::move(e));
    };

    // Entry lookup is resolved lazily inside each readFn so a stale
    // index can never dereference a reallocated mapping.
    auto entryBytes = [&](uint16_t chainIndex, bool txpdo,
                          uint16_t off, void* dst, size_t len) {
        const int e = fm.findEntry(chainIndex, txpdo);
        if (e < 0) return;
        const auto* ent = fm.pdo().mapping().get_entry(e);
        if (!ent || off + len > ent->data_size) return;
        std::memcpy(dst, ent->storage + off, len);
    };

    uint8_t nodeIdx = 0;
    for (const auto& inf : infos) {
        ++nodeIdx;
        if (inf.isCoupler) continue;
        const uint16_t ci = static_cast<uint16_t>(inf.chainIndex);
        for (uint8_t g = 0; g < inf.diSize; ++g) {
            const std::string nm = inf.key + ".di" + std::to_string(g);
            addSig(nodeIdx, 1, g, ValueType::U8, nm,
                   [&, ci, g](void* dst) {
                       entryBytes(ci, true, inf.diOff + g, dst, 1);
                   });
        }
        for (uint8_t c = 0; c < inf.aiSize / 2; ++c) {
            const std::string nm = inf.key + ".ai" + std::to_string(c);
            addSig(nodeIdx, 2, c, ValueType::I16, nm,
                   [&, ci, c](void* dst) {
                       entryBytes(ci, true, inf.aiOff + c * 2, dst, 2);
                   });
        }
        for (uint8_t g = 0; g < inf.doSize; ++g) {
            const std::string nm = inf.key + ".do" + std::to_string(g);
            addSig(nodeIdx, 3, g, ValueType::U8, nm,
                   [&, ci, g](void* dst) {
                       entryBytes(ci, false, inf.doOff + g, dst, 1);
                   });
        }
        for (uint8_t c = 0; c < inf.aoSize / 2; ++c) {
            const std::string nm = inf.key + ".ao" + std::to_string(c);
            addSig(nodeIdx, 4, c, ValueType::I16, nm,
                   [&, ci, c](void* dst) {
                       entryBytes(ci, false, inf.aoOff + c * 2, dst, 2);
                   });
        }
    }
    {
        addSig(0xFF, 0, 0, ValueType::U64, "link",
               [](void* dst) {
                   const uint64_t v = g_link.load() ? 1 : 0;
                   std::memcpy(dst, &v, 8);
               });
        addSig(0xFF, 0, 1, ValueType::U64, "cycles",
               [](void* dst) {
                   const uint64_t v = g_cycles.load();
                   std::memcpy(dst, &v, 8);
               });
        addSig(0xFF, 0, 2, ValueType::U64, "errors",
               [](void* dst) {
                   const uint64_t v = g_errors.load();
                   std::memcpy(dst, &v, 8);
               });
    }

    // catalog/registry/ioSchemas must outlive the server — declare at
    // function scope like beckhoff_slave_chain does.
    SchemaCatalog catalog;
    Registry registry;
    std::unique_ptr<Server> ioServer;
    if (ioPort != 0) {
        if (!catalog.install(ioSchemas.graph, ioSchemas.manifest)) {
            std::fprintf(stderr, "tplc-ec-pump: IO catalog install failed\n");
            return 1;
        }
        for (auto& e : pending) {
            const auto slot = catalog.slotFor(e.ref);
            if (!slot) {
                std::fprintf(stderr, "tplc-ec-pump: IO slot resolve failed\n");
                return 1;
            }
            e.signal.schema = e.ref;
            e.signal.schemaSlot = *slot;
            if (!registry.addSignal(e.signal)) {
                std::fprintf(stderr, "tplc-ec-pump: IO register failed\n");
                return 1;
            }
        }
        ServerConfig ioCfg;
        ioCfg.framing = Framing::Slip;
        ioCfg.timestampFn = nowUs;
        ioCfg.logFn = pumpLog;
        ioCfg.schemaCatalog = &catalog;
        ioCfg.maxClients = 4;
        ioServer = std::make_unique<Server>(
            registry, std::make_unique<TcpTransportServer>(ioPort, ioBind),
            ioCfg);
        if (!ioServer->start()) {
            std::fprintf(stderr,
                "tplc-ec-pump: IO listen %s:%u failed\n",
                ioBind.c_str(), ioPort);
            return 1;
        }
        std::fprintf(stderr, "tplc-ec-pump: tether_io on %s:%u\n",
                     ioBind.c_str(), ioPort);
    }

    std::filesystem::create_directories(simdir);

    // Drive exchanges synchronously so file->PDO and PDO->file stay
    // ordered within each pump cycle.
    fm.stopExchangeThread();

    const auto period =
        std::chrono::nanoseconds(1'000'000'000ULL / hz);
    const auto deadline = runSecs > 0
        ? std::chrono::steady_clock::now() +
              std::chrono::duration<double>(runSecs)
        : std::chrono::steady_clock::time_point::max();

    g_link.store(true);
    uint64_t cyclesDone = 0;
    auto next = std::chrono::steady_clock::now();
    while (!g_stop && std::chrono::steady_clock::now() < deadline) {
        // RxPDO (master->slave): file -> entry storage.
        for (const auto& l : links) {
            if (l.txpdo) continue;
            auto* e = fm.pdo().mapping().get_entry_mut(l.entry);
            if (!e || e->data_size != l.size) continue;
            readFile(std::filesystem::path(simdir) /
                         (l.key + ".rx"),
                     e->storage, e->data_size);
        }
        if (!fm.exchange()) {
            ++g_errors;
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
            continue;
        }
        // TxPDO (slave->master): entry storage -> file.
        for (const auto& l : links) {
            if (!l.txpdo) continue;
            const auto* e = fm.pdo().mapping().get_entry(l.entry);
            if (!e || e->data_size != l.size) continue;
            writeFileAtomic(std::filesystem::path(simdir) /
                                (l.key + ".tx"),
                            e->storage, e->data_size);
        }
        g_cycles.store(cyclesDone + 1);
        ++cyclesDone;
        next += period;
        const auto now = std::chrono::steady_clock::now();
        if (next > now) std::this_thread::sleep_until(next);
        else next = now + period;
    }

    g_link.store(false);
    if (ioServer) ioServer->stop();
    std::fprintf(stderr, "tplc-ec-pump: pump ran %llu cycles\n",
                 static_cast<unsigned long long>(cyclesDone));
    return 0;
}
