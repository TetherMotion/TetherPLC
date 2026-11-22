/* tplc-io internal surface — shared declarations for the split TUs.
 * Everything lives in namespace tplcio_int; main.cpp keeps the
 * process entry points (usage/populateRegistry/main). */

#pragma once

#include "CtlClient.hpp"

#include <tether/io/Registry.hpp>
#include <tether/io/SchemaCatalog.hpp>
#include <tether/io/SchemaDigest.hpp>
#include "SchemaValueBridge.hpp"
#include <tether/io/Server.hpp>
#include <tether/io/TcpTransport.hpp>

#include <tplc/tplc_ctl.h>

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <optional>
#include <string>
#include <type_traits>
#include <vector>

namespace tplcio_int {

using namespace tether::io;

uint64_t nowUs();

/* ---- LE payload writer (mirrors ctl W) -------------------------- */
struct W {
    std::vector<uint8_t> b;
    void u8(uint8_t v)  { b.push_back(v); }
    void u16(uint16_t v){ b.push_back(v & 0xFF); b.push_back(v >> 8); }
    void u32(uint32_t v){ for (int i = 0; i < 4; i++) b.push_back(v >> (8 * i)); }
    void u64(uint64_t v){ for (int i = 0; i < 8; i++) b.push_back(v >> (8 * i)); }
    void bytes(const void* p, size_t n) {
        const uint8_t* c = (const uint8_t*)p;
        b.insert(b.end(), c, c + n);
    }
    void str(const std::string& s) { u16((uint16_t)s.size()); bytes(s.data(), s.size()); }
};

/* ---- minimal JSON writer ---------------------------------------- */
struct J {
    std::string s;
    J& raw(const std::string& v) { s += v; return *this; }
    J& chr(char c) { s += c; return *this; }
    J& key(const std::string& k, bool comma) {
        if (comma) s += ',';
        s += '"'; esc(k); s += "\":";
        return *this;
    }
    template <typename T,
              typename = std::enable_if_t<std::is_integral_v<T>>>
    J& num(T v) { s += std::to_string(v); return *this; }
    J& strv(const std::string& v) { s += '"'; esc(v); s += '"'; return *this; }
    J& hex(const uint8_t* p, size_t n) {
        static const char* H = "0123456789abcdef";
        s += '"';
        for (size_t i = 0; i < n; i++) { s += H[p[i] >> 4]; s += H[p[i] & 15]; }
        s += '"';
        return *this;
    }
    void esc(const std::string& v) {
        for (char c : v) {
            switch (c) {
                case '"':  s += "\\\""; break;
                case '\\': s += "\\\\"; break;
                case '\n': s += "\\n";  break;
                case '\r': s += "\\r";  break;
                case '\t': s += "\\t";  break;
                default:
                    if ((unsigned char)c < 0x20) {
                        char b[8];
                        snprintf(b, sizeof b, "\\u%04x", c);
                        s += b;
                    } else s += c;
            }
        }
    }
};

/* ---- V6 schema: one scalar node per used ValueType -----------------
   Clients decode by slot; flat entries without a schema ref can't be
   read by V6 sessions. Keys: 0x50 ('P') in byte 0, tag in byte 1.    */
extern SchemaGraph   g_graph;
extern SchemaCatalog g_catalog;

SchemaKey keyFor(ValueType t);
const SchemaNode* nodeFor(ValueType t);

/* ---- ctl call → FunctionCallResult ------------------------------ */
extern tplcio::CtlClient g_ctl;

FunctionCallResult ferr(const tplcio::CtlClient::Error& e);
FunctionCallResult retOf(ValueType t, const NativeValue& nv);
FunctionCallResult okStr(const std::string& v);
FunctionCallResult okBin(const std::vector<uint8_t>& v);
FunctionCallResult okVoid();
FunctionCallResult okU32(uint32_t v);

/* progStatus keeps a short cache (see below); mutating ops must
   invalidate it or a signal read right after e.g. shadow.load can
   see the pre-op state for the cache window                      */
void invalidateStatusCache();

bool ctlCall(uint16_t op, const void* p, uint32_t n,
             std::vector<uint8_t>& out, FunctionCallResult& r);
bool ctlCall(uint16_t op, std::vector<uint8_t>& out,
             FunctionCallResult& r);

/* ---- argument helpers ------------------------------------------- */
const FunctionArgument* argAt(const std::vector<FunctionArgument>& a,
                              uint32_t pos);
const SchemaNode* argNode(ValueType t);
bool decodeArg(const FunctionArgument& x, ValueType t,
               SchemaValue& out);
std::string argStr(const std::vector<FunctionArgument>& a,
                   uint32_t pos);
uint64_t argU(const std::vector<FunctionArgument>& a, uint32_t pos,
              uint64_t dflt = 0);
std::vector<uint8_t> argBin(const std::vector<FunctionArgument>& a,
                            uint32_t pos);

/* ---- ctl payload → JSON decoders -------------------------------- */
void wsel(W& w, const std::string& sel);
std::string progStatusJson(const std::vector<uint8_t>& p);
std::string shadowStatusJson(const std::vector<uint8_t>& p);
std::string shadowListJson(const std::vector<uint8_t>& p);
std::string shadowCompareJson(const std::vector<uint8_t>& p);
std::string migrateResultJson(const std::vector<uint8_t>& p);
std::string fieldsJson(const std::vector<uint8_t>& p);
std::string resolveJson(const std::vector<uint8_t>& p);
std::string stateJson(const std::vector<uint8_t>& p);
std::string forceListJson(const std::vector<uint8_t>& p);
std::string breakListJson(const std::vector<uint8_t>& p);
std::string watchListJson(const std::vector<uint8_t>& p);
std::string stateDiffJson(const std::vector<uint8_t>& p);
std::string watchGetJson(const std::vector<uint8_t>& p);
std::string traceStatusJson(const std::vector<uint8_t>& p);
std::string traceReadJson(const std::vector<uint8_t>& p);
std::string retainStatusJson(const std::vector<uint8_t>& p);
std::string imageStatsJson(const std::vector<uint8_t>& p);
std::string journalJson(const std::vector<uint8_t>& p);
std::string stateChangeJson(const std::vector<uint8_t>& p);

/* ---- V6 schema construction -------------------------------------- */
void assignSchema(FunctionParameter& p);
void assignSchema(FunctionReturn& r);
void assignSchema(SignalEntry& e);
void assignSchema(ParamEntry& p);
std::vector<SchemaManifestEntry> plcManifest(const SchemaGraph& g);
void buildSchema();
std::vector<FunctionEntry> buildFunctions();

/* function/signal id counters — populateRegistry resets both to
   their base ranges on each rebuild so ids stay stable-ish        */
extern uint64_t g_fnId;
extern uint64_t g_sigId;

SignalEntry sig(const char* name, const char* desc, ValueType t,
                ReadFn rd);

/* ---- program status + PLC field extraction ----------------------- */
struct PlcField {
    std::string name;
    uint8_t kind = 0, flags = 0, region = 0, access = 0;
    uint32_t tref = 0;
    uint64_t off = 0, size = 0;
};

bool progStatus(TplcCtlProgStatus& ps,
                std::vector<uint8_t>& raw);
std::vector<PlcField> fetchPlcFields();

template <typename T, typename F>
ReadFn scalarRd(F proj) {
    return [proj](void* dst) {
        TplcCtlProgStatus ps;
        std::vector<uint8_t> raw;
        T v = proj(ps, progStatus(ps, raw));
        memcpy(dst, &v, sizeof v);
    };
}

/* ---- .tplc TYPC section decode ----------------------------------- */
struct TypcMember {
    std::string name;
    uint32_t type = 0;
    uint32_t off = 0;
};

struct TypcType {
    uint8_t tag = 0;
    uint32_t size = 0;
    std::vector<TypcMember> members;   /* STRUCT/FUNCTION_BLOCK only */
};

constexpr uint8_t kTagStruct = 19;
constexpr uint8_t kTagEnum = 20;
constexpr uint8_t kTagFb = 21;
constexpr uint32_t kNoIdx = 0xFFFFFFFFu;

bool readFileBytes(const std::string& path, std::vector<uint8_t>& out,
                   size_t cap = 64u << 20);
uint32_t scalarBytes(ValueType t);
ValueType vtOfTag(uint8_t t);
std::vector<TypcType> decodeTypc(const std::vector<uint8_t>& sec);
std::string dashedUuid(const uint8_t uuid[16]);
std::vector<TypcType> typcTypesForProgram(
    const TplcCtlProgStatus& ps, const std::string& packagesDir,
    const std::vector<std::string>& pkgFiles);

/* ---- debug-read + catalog emission ------------------------------- */
void dbgReadBytes(uint64_t off, uint32_t len, void* dst, size_t cap);
void emitPlcLeaf(Registry& registry, const std::string& name,
                 const std::string& group, uint64_t off, uint64_t size,
                 uint8_t flags, ValueType vt, const std::string& desc);
void emitPlcNode(Registry& registry, const std::string& name,
                 const std::string& group, uint64_t off, uint64_t size,
                 uint32_t tref, uint8_t flags,
                 const std::vector<TypcType>& types, int depth);
void addPlcFieldEntries(Registry& registry, const std::string& prog,
                        const std::vector<PlcField>& fields,
                        const std::vector<TypcType>& types);

} // namespace tplcio_int
