/* tplc-io-cli — generic tether_io client for tests and manual poking.
 *
 * Connects to any tether_io TCP/SLIP server (beckhoff_slave_chain,
 * tplc-ec-pump, tplc-io, ...) and lists/reads/writes catalog entries.
 *
 * usage:
 *   tplc-io-cli <port> list params|signals|functions
 *   tplc-io-cli <port> get <name>            (param or signal)
 *   tplc-io-cli <port> set <name> <int>      (param only; 0x.. hex ok)
 *   tplc-io-cli <port> getparam <name> / getsignal <name> / setparam <name> <int>
 *
 * <name> matches the entry name or "group.name".
 * Scalar values print as signed/unsigned decimal per the schema type.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tether/io/TetherIOClient.hpp>
#include <tether/io/TcpTransport.hpp>
#include <tether/io/SlipTransport.hpp>
#include "SchemaValueBridge.hpp"

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>

using namespace tether::io;
namespace nv = tether::io::nv;

static const ClientCatalogEntry* findEntry(
        const std::vector<ClientCatalogEntry>& es, const char* name) {
    for (const auto& e : es) {
        if (e.name == name) return &e;
        if (e.group.size() &&
            e.group + "." + e.name == name) return &e;
    }
    return nullptr;
}

static bool isSigned(ValueType t) {
    return t == ValueType::I8 || t == ValueType::I16 ||
           t == ValueType::I32 || t == ValueType::I64;
}

int main(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr,
            "usage: %s <port> list params|signals|functions\n"
            "       %s <port> get <name>\n"
            "       %s <port> set <name> <int>\n"
            "       %s <port> getparam|getsignal <name>\n"
            "       %s <port> setparam <name> <int>\n", argv[0], argv[0],
            argv[0], argv[0], argv[0]);
        return 2;
    }
    const uint16_t port = (uint16_t)atoi(argv[1]);
    const std::string cmd = argv[2];

    auto tr = tcpConnect("127.0.0.1", port, 5000);
    if (!tr) { fprintf(stderr, "connect :%u failed\n", port); return 1; }
    TetherIOClient client(
        std::make_unique<SlipFramedTransport>(std::move(tr)));
    if (!client.isConnected()) {
        fprintf(stderr, "handshake failed\n");
        return 1;
    }

    if (cmd == "list" && argc >= 4) {
        const std::string what = argv[3];
        if (what == "functions") {
            auto fns = client.listFunctions();
            if (!fns) { fprintf(stderr, "listFunctions: %s\n",
                                fns.error().message.c_str()); return 1; }
            for (const auto& f : *fns)
                printf("%llu %s (%s)\n",
                       (unsigned long long)f.id, f.name.c_str(),
                       f.group.c_str());
            return 0;
        }
        const bool params = what != "signals";
        auto es = params ? client.listParams() : client.listSignals();
        if (!es) { fprintf(stderr, "list: %s\n",
                           es.error().message.c_str()); return 1; }
        for (const auto& e : *es)
            printf("%llu %s%s%s type=%d size=%u\n",
                   (unsigned long long)e.id, e.group.c_str(),
                   e.group.empty() ? "" : ".", e.name.c_str(),
                   (int)e.type, (unsigned)e.valueSize);
        return 0;
    }

    /* resolve <name> across params (set/get) and signals (get) */
    auto resolve = [&](const char* name, bool wantParam,
                       ClientCatalogEntry& out) -> int {
        auto ps = client.listParams();
        if (ps) {
            if (const auto* e = findEntry(*ps, name)) {
                if (!wantParam) { /* signals-only caller still ok */ }
                out = *e;
                return wantParam ? 1 : 2;
            }
        }
        auto ss = client.listSignals();
        if (ss) {
            if (const auto* e = findEntry(*ss, name)) {
                out = *e;
                return 2;
            }
        }
        return 0;
    };

    if ((cmd == "get" || cmd == "getparam" || cmd == "getsignal") &&
        argc >= 4) {
        const bool sigOnly = cmd == "getsignal";
        const bool parOnly = cmd == "getparam";
        ClientCatalogEntry e;
        int kind = resolve(argv[3], !sigOnly, e);
        if (kind == 0 || (sigOnly && kind != 2) || (parOnly && kind != 1)) {
            fprintf(stderr, "no such entry: %s\n", argv[3]);
            return 1;
        }
        auto raw = kind == 1 ? client.getParam(e.id)
                             : client.getSignal(e.id);
        if (!raw) {
            fprintf(stderr, "read failed: %s\n",
                    raw.error().message.c_str());
            return 1;
        }
        auto v = client.decodeValue(e.schemaSlot,
                                    raw->data(), raw->size());
        if (!v) {
            /* fall back to raw hex when the schema can't decode */
            printf("0x");
            for (uint8_t b : *raw) printf("%02x", b);
            printf("\n");
            return 0;
        }
        /* catalog `type` is not authoritative — the schema node's
           scalarType carries the real ValueType for sign handling */
        const SchemaNode* node = client.resolveSchemaSlot(e.schemaSlot);
        const ValueType vt = node ? node->scalarType : e.type;
        auto bits = asU64(*v);
        if (!bits) {
            fprintf(stderr, "non-scalar value\n");
            return 1;
        }
        if (isSigned(vt))
            printf("%lld\n", (long long)(int64_t)*bits);
        else
            printf("%llu\n", (unsigned long long)*bits);
        return 0;
    }

    if ((cmd == "set" || cmd == "setparam") && argc >= 5) {
        ClientCatalogEntry e;
        int kind = resolve(argv[3], true, e);
        if (kind != 1) {
            fprintf(stderr, "no writable param: %s\n", argv[3]);
            return 1;
        }
        char* end = nullptr;
        int64_t val = strtoll(argv[4], &end, 0);
        if (!end || *end) {
            fprintf(stderr, "bad value: %s\n", argv[4]);
            return 2;
        }
        const SchemaNode* node = client.resolveSchemaSlot(e.schemaSlot);
        if (!node) { fprintf(stderr, "bad schema slot\n"); return 1; }
        auto enc = encodeNative(client.schemaGraph(), *node, nv::i(val));
        if (!enc) {
            fprintf(stderr, "encode: %s\n", enc.error().c_str());
            return 1;
        }
        if (enc->size() > 255) { fprintf(stderr, "value too large\n"); return 1; }
        auto r = client.setParam(e.id, enc->data(),
                                 (uint8_t)enc->size());
        if (!r) {
            fprintf(stderr, "set failed: %s\n",
                    r.error().message.c_str());
            return 1;
        }
        printf("ok\n");
        return 0;
    }

    fprintf(stderr, "unknown command: %s\n", cmd.c_str());
    return 2;
}
