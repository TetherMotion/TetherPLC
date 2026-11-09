/* tplc-io-probe — tether_io client acceptance probe for tplc-io.
   Drives a live provider through the debug surface and prints
   PASS/FAIL lines; exit 0 only when every check passes.

   usage: tplc-io-probe <port>                                        */

#include <tether/io/TetherIOClient.hpp>
#include <tether/io/TcpTransport.hpp>
#include <tether/io/SlipTransport.hpp>
#include "SchemaValueBridge.hpp"
#include <cstdio>
#include <cstring>
#include <string>

using namespace tether::io;
namespace nv = tether::io::nv;

static int g_fail = 0;
static void check(bool ok, const char* name) {
    printf("%s %s\n", ok ? "PASS" : "FAIL", name);
    if (!ok) g_fail++;
}

/* schema-wire encode a native value against a function param slot   */
static std::vector<uint8_t> encArg(const TetherIOClient& c,
                                   const ClientFunctionEntry& fn,
                                   const char* pname,
                                   uint32_t* pos,
                                   const NativeValue& nv) {
    for (const auto& p : fn.parameters) {
        if (p.name != pname) continue;
        *pos = &p - fn.parameters.data();
        const SchemaNode* n = c.resolveSchemaSlot(p.schemaSlot);
        if (!n) return {};
        auto e = encodeNative(c.schemaGraph(), *n, nv);
        return e ? *e : std::vector<uint8_t>{};
    }
    return {};
}

static TetherIOClient::FunctionArg mkArg(const TetherIOClient& c,
                                         const ClientFunctionEntry& fn,
                                         const char* pname,
                                         const NativeValue& nv) {
    TetherIOClient::FunctionArg a;
    a.value = encArg(c, fn, pname, &a.position, nv);
    for (const auto& p : fn.parameters)
        if (p.name == pname) { a.type = p.type; a.key = p.key; }
    return a;
}

static const ClientFunctionEntry* findFn(
        const std::vector<ClientFunctionEntry>& fns,
        const char* name) {
    for (const auto& f : fns)
        if (f.name == name) return &f;
    return nullptr;
}

static const ClientCatalogEntry* findSig(
        const std::vector<ClientCatalogEntry>& es,
        const char* name) {
    for (const auto& e : es)
        if (e.name == name) return &e;
    return nullptr;
}

static std::expected<ClientCallResult, ClientError>
call(TetherIOClient& c, const ClientFunctionEntry& fn,
     std::initializer_list<TetherIOClient::FunctionArg> args = {}) {
    return c.callFunction(fn.id,
        std::vector<TetherIOClient::FunctionArg>(args));
}

static std::string retStr(TetherIOClient& c,
                          const ClientFunctionEntry& fn,
                          const ClientCallResult& r) {
    if (!r.hasReturnValue) return {};
    auto v = c.decodeValue(fn.returnSchemaSlot,
                           r.returnValue.data(), r.returnValue.size());
    if (!v) return {};
    if (auto s = asString(*v)) return *s;
    return {};
}

int main(int argc, char** argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s <port> [shadow_pkg]\n", argv[0]);
        return 2;
    }
    const char* shadow_pkg = argc > 2 ? argv[2] : nullptr;
    auto tr = tcpConnect("127.0.0.1", (uint16_t)atoi(argv[1]), 5000);
    if (!tr) { printf("FAIL connect\n"); return 1; }
    TetherIOClient c(
        std::make_unique<SlipFramedTransport>(std::move(tr)));

    auto p = c.ping(0xCAFE);
    check(p && *p == 0xCAFE, "ping");
    printf("info schema_epoch=%llu\n",
           (unsigned long long)c.schemaEpoch());

    auto sigs = c.listSignals();
    check(sigs && sigs->size() >= 6, "listSignals >= 6");
    auto fns = c.listFunctions();
    check(fns && fns->size() >= 30, "listFunctions >= 30");

    /* ---- live signal reads ------------------------------------- */
    const ClientCatalogEntry* st = sigs ? findSig(*sigs, "status.state") : nullptr;
    const ClientCatalogEntry* cy = sigs ? findSig(*sigs, "status.cycles") : nullptr;
    const ClientCatalogEntry* bl = sigs ? findSig(*sigs, "blob") : nullptr;
    check(st && cy && bl, "status+cycle+blob signals present");
    if (st) {
        auto v = c.getSignal(st->id);
        check(v && c.decodeValue(st->schemaSlot, v->data(), v->size()),
              "status.state decodes");
    }
    if (cy) {
        auto v = c.getSignal(cy->id);
        check(v && c.decodeValue(cy->schemaSlot, v->data(), v->size()),
              "status.cycles decodes");
    }
    if (bl) {
        auto v = c.getSignal(bl->id);
        auto dv = v ? c.decodeValue(bl->schemaSlot, v->data(), v->size())
                    : std::unexpected(ClientError{});
        check(dv && std::get_if<SchemaValue::Bytes>(&dv->payload),
              "blob decodes to Bytes");
    }

    /* ---- debug surface ------------------------------------------ */
    auto* fStatus = fns ? findFn(*fns, "prog.status") : nullptr;
    auto* fFields = fns ? findFn(*fns, "dbg.fields") : nullptr;
    auto* fResolve = fns ? findFn(*fns, "dbg.resolve") : nullptr;
    auto* fRead = fns ? findFn(*fns, "dbg.read") : nullptr;
    auto* fWrite = fns ? findFn(*fns, "dbg.write") : nullptr;
    auto* fForce = fns ? findFn(*fns, "dbg.force") : nullptr;
    auto* fUnforce = fns ? findFn(*fns, "dbg.unforce") : nullptr;
    auto* fForces = fns ? findFn(*fns, "dbg.forces") : nullptr;
    auto* fSuspend = fns ? findFn(*fns, "task.suspend") : nullptr;
    auto* fResume = fns ? findFn(*fns, "task.resume") : nullptr;
    auto* fStep = fns ? findFn(*fns, "task.step") : nullptr;
    auto* fWAdd = fns ? findFn(*fns, "watch.add") : nullptr;
    auto* fWGet = fns ? findFn(*fns, "watch.get") : nullptr;
    auto* fWRm = fns ? findFn(*fns, "watch.rm") : nullptr;
    auto* fStop = fns ? findFn(*fns, "prog.stop") : nullptr;
    auto* fStart = fns ? findFn(*fns, "prog.start") : nullptr;
    check(fStatus && fFields && fResolve && fRead && fWrite &&
          fForce && fUnforce && fSuspend && fResume && fStep &&
          fWAdd && fWGet && fWRm && fStop && fStart,
          "debug function surface present");

    if (fStatus) {
        auto r = call(c, *fStatus);
        std::string j = (r && r->success) ? retStr(c, *fStatus, *r) : "";
        check(j.find("\"state\"") != std::string::npos &&
              j.find("\"uuid\"") != std::string::npos,
              "prog.status returns state JSON");
    }
    if (fFields) {
        auto r = call(c, *fFields);
        std::string j = (r && r->success) ? retStr(c, *fFields, *r) : "";
        check(j.find("\"fields\":[") != std::string::npos,
              "dbg.fields returns field list");
    }
    if (fResolve) {
        auto a = mkArg(c, *fResolve, "name", nv::str("n"));
        auto r = call(c, *fResolve, {a});
        std::string j = (r && r->success) ? retStr(c, *fResolve, *r) : "";
        check(j.find("\"sym\"") != std::string::npos,
              "dbg.resolve 'n' resolves");
        auto bad = call(c, *fResolve,
                        {mkArg(c, *fResolve, "name", nv::str("zzz_no"))});
        check(bad && !bad->success &&
              bad->errorMessage.find("not found") != std::string::npos,
              "dbg.resolve missing symbol errors");
    }
    if (fRead) {
        auto r = call(c, *fRead,
                      {mkArg(c, *fRead, "off", nv::u(0)),
                       mkArg(c, *fRead, "len", nv::u(4))});
        bool binOk = false;
        if (r && r->success && r->hasReturnValue) {
            auto v = c.decodeValue(fRead->returnSchemaSlot,
                                   r->returnValue.data(),
                                   r->returnValue.size());
            if (v) {
                if (auto* b = std::get_if<SchemaValue::Bytes>(&v->payload))
                    binOk = b->value.size() == 4;
            }
        }
        check(binOk, "dbg.read returns 4 blob bytes");
    }
    if (fWrite && fRead) {
        const uint8_t patch[] = {0xAB};
        auto wr = call(c, *fWrite,
                       {mkArg(c, *fWrite, "off", nv::u(2)),
                        mkArg(c, *fWrite, "data",
                              nv::bytes(std::span<const uint8_t>(patch)))});
        check(wr && wr->success, "dbg.write succeeds");
    }
    if (fForce && fUnforce && fForces) {
        const uint8_t one[] = {0xFF};
        auto fr = call(c, *fForce,
                       {mkArg(c, *fForce, "off", nv::u(3)),
                        mkArg(c, *fForce, "data",
                              nv::bytes(std::span<const uint8_t>(one))),
                        mkArg(c, *fForce, "mask",
                              nv::bytes(std::span<const uint8_t>(one)))});
        uint32_t h = 0;
        if (fr && fr->success && fr->hasReturnValue) {
            auto v = c.decodeValue(fForce->returnSchemaSlot,
                                   fr->returnValue.data(),
                                   fr->returnValue.size());
            if (v) if (auto u = asU64(*v))
                h = (uint32_t)*u;
        }
        check(h != 0, "dbg.force returns a handle");
        auto lst = call(c, *fForces);
        std::string j = (lst && lst->success) ? retStr(c, *fForces, *lst) : "";
        check(j.find("\"handle\":") != std::string::npos,
              "dbg.forces lists the force");
        auto un = call(c, *fUnforce,
                       {mkArg(c, *fUnforce, "handle", nv::u(h))});
        check(un && un->success, "dbg.unforce by handle");
    }
    if (fSuspend && fStep && fResume) {
        auto s = call(c, *fSuspend,
                      {mkArg(c, *fSuspend, "index", nv::u(0))});
        check(s && s->success, "task.suspend");
        auto st2 = call(c, *fStep,
                        {mkArg(c, *fStep, "index", nv::u(0)),
                         mkArg(c, *fStep, "cycles", nv::u(2))});
        check(st2 && st2->success, "task.step 2 cycles");
        auto rs = call(c, *fResume,
                       {mkArg(c, *fResume, "index", nv::u(0))});
        check(rs && rs->success, "task.resume");
    }
    if (fWAdd && fWGet && fWRm) {
        auto wa = call(c, *fWAdd,
                       {mkArg(c, *fWAdd, "spec", nv::str("n"))});
        uint32_t h = 0;
        if (wa && wa->success && wa->hasReturnValue) {
            auto v = c.decodeValue(fWAdd->returnSchemaSlot,
                                   wa->returnValue.data(),
                                   wa->returnValue.size());
            if (v) if (auto u = asU64(*v))
                h = (uint32_t)*u;
        }
        check(h != 0, "watch.add 'n' returns handle");
        auto wg = call(c, *fWGet,
                       {mkArg(c, *fWGet, "handle", nv::u(h))});
        std::string j = (wg && wg->success) ? retStr(c, *fWGet, *wg) : "";
        check(j.find("\"data\":") != std::string::npos,
              "watch.get returns item bytes");
        auto wr = call(c, *fWRm,
                       {mkArg(c, *fWRm, "handle", nv::u(h))});
        check(wr && wr->success, "watch.rm");
    }

    /* ---- shadow run (doc 19 §3.3) — only when a candidate package
       path is supplied; promotes last (it replaces the active)   */
    if (shadow_pkg) {
        auto* fShLoad = fns ? findFn(*fns, "shadow.load") : nullptr;
        auto* fShStatus = fns ? findFn(*fns, "shadow.status") : nullptr;
        auto* fShCompare = fns ? findFn(*fns, "shadow.compare") : nullptr;
        auto* fShPromote = fns ? findFn(*fns, "shadow.promote") : nullptr;
        auto* fShDrop = fns ? findFn(*fns, "shadow.drop") : nullptr;
        auto* fShReset = fns ? findFn(*fns, "shadow.reset") : nullptr;
        check(fShLoad && fShStatus && fShCompare && fShPromote &&
              fShDrop && fShReset, "shadow function surface present");
        auto* sShState = sigs ? findSig(*sigs, "shadow.state") : nullptr;
        auto* sShDiff = sigs ? findSig(*sigs, "shadow.diff_count") : nullptr;
        check(sShState && sShDiff, "shadow signals present");
        if (fShStatus) {
            auto r = call(c, *fShStatus);
            std::string j = (r && r->success)
                ? retStr(c, *fShStatus, *r) : "";
            check(j.find("false") != std::string::npos,
                  "shadow.status empty before load");
        }
        bool loaded = false;
        if (fShLoad) {
            auto r = call(c, *fShLoad,
                          {mkArg(c, *fShLoad, "path",
                                 nv::str(shadow_pkg))});
            std::string j = (r && r->success)
                ? retStr(c, *fShLoad, *r) : "";
            loaded = j.find("loaded") != std::string::npos &&
                     j.find("1") != std::string::npos;
            check(loaded, "shadow.load");
        }
        if (loaded) {
            if (sShState) {
                auto v = c.getSignal(sShState->id);
                auto dv = v ? c.decodeValue(sShState->schemaSlot,
                                            v->data(), v->size())
                            : std::unexpected(ClientError{});
                bool one = false;
                if (dv) if (auto u = asU64(*dv)) one = *u == 1;
                check(one, "shadow.state signal reads 1");
            }
            auto r = call(c, *fShStatus);
            std::string j = (r && r->success)
                ? retStr(c, *fShStatus, *r) : "";
            check(j.find("\"state\":1") != std::string::npos,
                  "shadow.status running");
            auto rc = call(c, *fShCompare);
            std::string jc = (rc && rc->success)
                ? retStr(c, *fShCompare, *rc) : "";
            check(jc.find("diffs") != std::string::npos,
                  "shadow.compare returns diff JSON");
            auto rr = call(c, *fShReset);
            check(rr && rr->success, "shadow.reset");
            auto rp = call(c, *fShPromote);
            std::string jp = (rp && rp->success)
                ? retStr(c, *fShPromote, *rp) : "";
            check(jp.find("epoch") != std::string::npos,
                  "shadow.promote commits");
            auto r2 = call(c, *fShStatus);
            std::string j2 = (r2 && r2->success)
                ? retStr(c, *fShStatus, *r2) : "";
            check(j2.find("false") != std::string::npos,
                  "shadow slot freed post-promote");
        }
    }

    /* ---- lifecycle: strict transitions -------------------------- */
    if (fStop && fStart) {
        auto s1 = call(c, *fStop);
        std::string j = (s1 && s1->success) ? retStr(c, *fStop, *s1) : "";
        check(j.find("\"to\":0") != std::string::npos,
              "prog.stop → state 0");
        auto s2 = call(c, *fStop);
        check(s2 && !s2->success, "second stop rejected");
        auto s3 = call(c, *fStart);
        check(s3 && s3->success &&
              retStr(c, *fStart, *s3).find("\"to\":1") != std::string::npos,
              "prog.start → state 1");
    }

    printf("%s\n", g_fail ? "PROBE FAILED" : "PROBE PASSED");
    return g_fail ? 1 : 0;
}
