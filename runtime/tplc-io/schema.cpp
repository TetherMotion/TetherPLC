#include "tplc_io_internal.hpp"

namespace tplcio_int {

void assignSchema(FunctionParameter& p) {
    const SchemaNode* n = g_graph.find(keyFor(p.type));
    if (!n) return;
    p.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(p.schema)) p.schemaSlot = *s;
}

void assignSchema(FunctionReturn& r) {
    const SchemaNode* n = g_graph.find(keyFor(r.type));
    if (!n) return;
    r.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(r.schema)) r.schemaSlot = *s;
}

void assignSchema(SignalEntry& e) {
    const SchemaNode* n = g_graph.find(keyFor(e.valueType));
    if (!n) return;
    e.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(e.schema)) {
        e.schemaSlot = *s;
        /* V6 clients pick schema-aware decoding off this annotation —
           without it an I64 reads back as a bogus f64              */
        e.metadata["schema.name"] = n->name;
    }
}

void assignSchema(ParamEntry& p) {
    const SchemaNode* n = g_graph.find(keyFor(p.valueType));
    if (!n) return;
    p.schema = SchemaRef{n->key, computeSchemaDigest(*n)};
    if (auto s = g_catalog.slotFor(p.schema)) {
        p.schemaSlot = *s;
        p.metadata["schema.name"] = n->name;
    }
}

std::vector<SchemaManifestEntry> plcManifest(const SchemaGraph& g) {
    std::vector<SchemaManifestEntry> m;
    m.reserve(g.nodes.size());
    for (const auto& n : g.nodes)
        m.push_back({SchemaRef{n.key, computeSchemaDigest(n)}, n.revision});
    return m;
}

void buildSchema() {
    const ValueType types[] = {
        ValueType::U8, ValueType::U16, ValueType::U32, ValueType::U64,
        ValueType::I8, ValueType::I32, ValueType::I64,
        ValueType::F32, ValueType::F64, ValueType::Bool,
        ValueType::String, ValueType::Binary,
    };
    for (ValueType t : types) {
        SchemaNode n;
        n.key = keyFor(t);
        n.kind = t == ValueType::String ? SchemaKind::String
               : t == ValueType::Binary ? SchemaKind::Bytes
                                        : SchemaKind::Scalar;
        n.scalarType = t;
        n.name = "plc.type." + std::to_string((unsigned)t);
        g_graph.nodes.push_back(std::move(n));
    }
    resolveSchemaDigests(g_graph);
    g_catalog.install(g_graph, plcManifest(g_graph));
}

/* ---- function entry builders ------------------------------------- */
uint64_t g_fnId = 100;

FunctionParameter P(const char* name, const char* desc, ValueType t) {
    return FunctionParameter{name, desc, t};
}

FunctionParameter Popt(const char* name, const char* desc, ValueType t,
                       std::vector<uint8_t> dflt) {
    FunctionParameter p{name, desc, t};
    p.optional = true;
    p.hasDefault = true;
    p.defaultValue = std::move(dflt);
    return p;
}

FunctionEntry fn(const char* name, const char* desc,
                 std::vector<FunctionParameter> params,
                 FunctionCallback cb,
                 std::optional<ValueType> ret = std::nullopt) {
    FunctionEntry e;
    e.id = g_fnId++;
    e.name = name;
    e.description = desc;
    e.group = "plc";
    e.parameters = std::move(params);
    for (auto& p : e.parameters) assignSchema(p);
    e.callback = std::move(cb);
    if (ret) {
        e.returnValue.present = true;
        e.returnValue.name = "result";
        e.returnValue.type = *ret;
        assignSchema(e.returnValue);
    }
    return e;
}

/* simple ctl op with no args and an optional JSON decoder */
FunctionEntry jsonOp(const char* name, const char* desc, uint16_t op,
                     std::string (*dec)(const std::vector<uint8_t>&)) {
    return fn(name, desc, {},
              [op, dec](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(op, out, r)) return r;
        return okStr(dec ? dec(out) : std::string(out.begin(), out.end()));
    }, ValueType::String);
}

/* state-changing lifecycle op: no payload → TplcCtlProgStateChange */
FunctionEntry stateOp(const char* name, const char* desc, uint16_t op,
                      std::vector<FunctionParameter> params = {},
                      void (*fill)(W&, const std::vector<FunctionArgument>&)
                          = nullptr) {
    return fn(name, desc, std::move(params),
              [op, fill](const std::vector<FunctionArgument>& a) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        W w;
        if (fill) fill(w, a);
        if (!ctlCall(op, w.b.data(), (uint32_t)w.b.size(), out, r))
            return r;
        return okStr(stateChangeJson(out));
    }, ValueType::String);
}

/* ---- catalog ------------------------------------------------------ */
std::vector<FunctionEntry> buildFunctions() {
    std::vector<FunctionEntry> v;

    /* lifecycle */
    v.push_back(stateOp("prog.start", "start the stopped program",
                        TPLC_OP_PROG_START));
    v.push_back(stateOp("prog.stop", "stop the running program",
                        TPLC_OP_PROG_STOP));
    v.push_back(stateOp("prog.kill",
                        "abort flag + deschedule immediately",
                        TPLC_OP_PROG_KILL));
    v.push_back(stateOp("prog.reset",
                        "re-initialize a stopped program (warm keeps retain)",
                        TPLC_OP_PROG_RESET,
                        {Popt("warm", "keep retain data", ValueType::Bool, {1})},
                        [](W& w, const std::vector<FunctionArgument>& a) {
                            w.u8((uint8_t)argU(a, 0, 1));
                        }));
    v.push_back(stateOp("prog.unload", "unload the stopped program",
                        TPLC_OP_PROG_UNLOAD));
    v.push_back(stateOp("prog.prepare",
                        "load a .tplc/.so into the stopped slot",
                        TPLC_OP_PROG_PREPARE,
                        {P("path", "package path (server-local)",
                           ValueType::String)},
                        [](W& w, const std::vector<FunctionArgument>& a) {
                            w.str(argStr(a, 0));
                        }));
    v.push_back(fn("prog.rollback",
                   "roll back the quarantined program", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_PROG_ROLLBACK, out, r)) return r;
        return okVoid();
    }));

    /* status */
    v.push_back(jsonOp("prog.status",
                       "full program status object (JSON)",
                       TPLC_OP_PROG_STATUS, progStatusJson));
    v.push_back(jsonOp("dbg.state",
                       "blob/retain extents + symbol count",
                       TPLC_OP_DBG_STATE, stateJson));
    v.push_back(fn("dbg.fields",
                   "blob layout: every field with name/off/size/access",
                   {Popt("sel", "0 active, N = shadow N (SHADOW_LIST order)", ValueType::U8,
                         {0})},
                   [](const std::vector<FunctionArgument>& a) {
        uint64_t sel = argU(a, 0, 0);
        W w;
        if (sel) w.u8((uint8_t)sel);
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_FIELDS, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(fieldsJson(out));
    }, ValueType::String));
    v.push_back(jsonOp("dbg.state_diff",
                       "last online-change migrate rules",
                       TPLC_OP_DBG_STATE_DIFF, stateDiffJson));
    v.push_back(jsonOp("image.stats", "process-image counters",
                       TPLC_OP_IMAGE_STATS, imageStatsJson));
    v.push_back(jsonOp("journal.tail",
                       "durable audit journal tail (JSON lines)",
                       TPLC_OP_JOURNAL_TAIL, journalJson));
    v.push_back(jsonOp("map.get", "deploy-map section text",
                       TPLC_OP_MAP_GET,
                       [](const std::vector<uint8_t>& p) {
        J j; j.chr('{'); j.key("map", 0)
            .strv(std::string((const char*)p.data(), p.size()));
        j.chr('}');
        return j.s;
    }));
    v.push_back(jsonOp("retain.status", "retain store state",
                       TPLC_OP_RETAIN_STATUS, retainStatusJson));
    v.push_back(jsonOp("trace.status", "trace recorder state",
                       TPLC_OP_TRACE_STATUS, traceStatusJson));
    v.push_back(jsonOp("dbg.breaks", "armed break/capture list",
                       TPLC_OP_DBG_BREAK_LIST, breakListJson));
    v.push_back(jsonOp("dbg.forces", "active force list",
                       TPLC_OP_DBG_FORCE_LIST, forceListJson));
    v.push_back(jsonOp("dbg.watch_list", "watch handles",
                       TPLC_OP_DBG_WATCH_LIST, watchListJson));

    /* shadow run (doc 19 §3.3) */
    v.push_back(fn("shadow.load",
                   "load a candidate program as shadow (needs running "
                   "active)",
                   {P("path", "package path (server-local)",
                      ValueType::String)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.str(argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_LOAD, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        if (out.size() >= 17 && out[0]) {
            J j; j.chr('{');
            j.key("loaded", 0).num(1);
            j.key("uuid", 1).hex(out.data() + 1, 16);
            j.chr('}');
            return okStr(j.s);
        }
        return okStr(migrateResultJson(out));
    }, ValueType::String));
    v.push_back(fn("shadow.status",
                   "candidate program status (sel: name or uuid)",
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_STATUS, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(shadowStatusJson(out));
    }, ValueType::String));
    v.push_back(jsonOp("shadow.list",
                       "every loaded shadow candidate (JSON array)",
                       TPLC_OP_SHADOW_LIST, shadowListJson));
    v.push_back(fn("shadow.compare",
                   "tail of the %Q diff ring (img-off runs ≤32B)",
                   {Popt("max", "max entries", ValueType::U32,
                         {32, 0, 0, 0}),
                    Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 32));
        wsel(w, argStr(a, 1));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_COMPARE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(shadowCompareJson(out));
    }, ValueType::String));
    v.push_back(fn("shadow.promote",
                   "promote the candidate to active (quarantines "
                   "current; drops other shadows)",
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_PROMOTE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        J j; j.chr('{');
        if (out.size() >= 24) {
            j.key("uuid", 0).hex(out.data(), 16);
            uint64_t ep; memcpy(&ep, out.data() + 16, 8);
            j.key("epoch", 1).num(ep);
        }
        j.chr('}');
        return okStr(j.s);
    }, ValueType::String));
    v.push_back(fn("shadow.drop", "discard a candidate", 
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_DROP, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okVoid();
    }));
    v.push_back(fn("shadow.reset",
                   "re-migrate candidate state from the live blob",
                   {Popt("sel", "shadow name or uuid-hex",
                         ValueType::String, {})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; wsel(w, argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_SHADOW_RESET, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(migrateResultJson(out));
    }, ValueType::String));

    /* debug reads/writes */
    v.push_back(fn("dbg.resolve",
                   "resolve a symbol path to blob offset/size",
                   {P("name", "symbol path", ValueType::String)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.str(argStr(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_RESOLVE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(resolveJson(out));
    }, ValueType::String));
    v.push_back(fn("dbg.read", "read blob bytes",
                   {P("off", "blob offset", ValueType::U64),
                    P("len", "byte count", ValueType::U32),
                    Popt("sel", "0 active, N = shadow N (SHADOW_LIST order)", ValueType::U8,
                         {0})},
                   [](const std::vector<FunctionArgument>& a) {
        uint64_t sel = argU(a, 2, 0);
        W w; w.u64(argU(a, 0)); w.u32((uint32_t)argU(a, 1));
        if (sel) w.u8((uint8_t)sel);
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_READ, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okBin(out);
    }, ValueType::Binary));
    v.push_back(fn("dbg.write", "write blob bytes",
                   {P("off", "blob offset", ValueType::U64),
                    P("data", "bytes", ValueType::Binary)},
                   [](const std::vector<FunctionArgument>& a) {
        auto d = argBin(a, 1);
        W w; w.u64(argU(a, 0));
        w.u32((uint32_t)d.size());
        w.bytes(d.data(), d.size());
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WRITE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okVoid();
    }));
    v.push_back(fn("dbg.force", "force blob bytes (override)",
                   {P("off", "blob offset", ValueType::U64),
                    P("data", "bytes", ValueType::Binary),
                    Popt("mask", "bit force mask", ValueType::Binary, {})},
                   [](const std::vector<FunctionArgument>& a) {
        auto d = argBin(a, 1);
        auto m = argBin(a, 2);
        W w; w.u64(argU(a, 0));
        w.u32((uint32_t)d.size());
        w.bytes(d.data(), d.size());
        if (!m.empty()) w.bytes(m.data(), m.size());
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_FORCE, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("dbg.unforce",
                   "drop a force (0xFFFFFFFF = all)",
                   {P("handle", "force handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 0xFFFFFFFFu));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_FORCE_CLEAR, w.b.data(), 4, out, r))
            return r;
        return okVoid();
    }));

    /* task control */
    for (auto& t : {std::pair{"task.suspend", TPLC_OP_TASK_SUSPEND},
                    {"task.resume",  TPLC_OP_TASK_RESUME}}) {
        v.push_back(fn(t.first, "suspend/resume a task by index",
                       {P("index", "task index", ValueType::U32)},
                       [op = t.second](
                           const std::vector<FunctionArgument>& a) {
            W w; w.u32((uint32_t)argU(a, 0));
            std::vector<uint8_t> out;
            FunctionCallResult r;
            if (!ctlCall(op, w.b.data(), 4, out, r)) return r;
            return okVoid();
        }));
    }
    v.push_back(fn("task.step", "single-step a suspended task",
                   {P("index", "task index", ValueType::U32),
                    Popt("cycles", "cycles to run", ValueType::U32,
                         {1, 0, 0, 0})},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0));
        w.u32((uint32_t)argU(a, 1, 1));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TASK_STEP, w.b.data(), 8, out, r))
            return r;
        return okVoid();
    }));

    /* watch — spec: "off:len,off:len,name" (same as CLI syntax) */
    v.push_back(fn("watch.add",
                   "arm a watch; spec = 'off:len,…' items or symbol paths",
                   {P("spec", "comma-separated items", ValueType::String)},
                   [](const std::vector<FunctionArgument>& a) {
        std::string spec = argStr(a, 0);
        /* split on ',' — item is OFF:LEN digits or a symbol path   */
        std::vector<std::string> items;
        size_t at = 0;
        while (at <= spec.size()) {
            size_t c = spec.find(',', at);
            std::string it = spec.substr(
                at, c == std::string::npos ? c : c - at);
            if (!it.empty()) items.push_back(it);
            if (c == std::string::npos) break;
            at = c + 1;
        }
        if (items.empty() || items.size() > 16) {
            FunctionCallResult r;
            r.errorMessage = "need 1..16 items";
            return r;
        }
        W w;
        w.u32((uint32_t)items.size());
        w.u32(0);
        for (auto& it : items) {
            uint64_t off = 0; uint32_t l = 0; uint16_t nl = 0;
            if (it.find(':') == std::string::npos) {
                nl = (uint16_t)it.size();
            } else if (sscanf(it.c_str(), "%llu:%u",
                              (unsigned long long*)&off, &l) != 2) {
                FunctionCallResult r;
                r.errorMessage = "bad item " + it;
                return r;
            }
            w.u64(off); w.u32(l); w.u16(nl);
            if (nl) w.bytes(it.data(), nl);
        }
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WATCH_ADD, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("watch.get", "read watch values",
                   {P("handle", "watch handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WATCH_GET, w.b.data(), 4, out, r))
            return r;
        return okStr(watchGetJson(out));
    }, ValueType::String));
    v.push_back(fn("watch.rm", "remove a watch (0xFFFFFFFF = all)",
                   {P("handle", "watch handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 0xFFFFFFFFu));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_WATCH_REMOVE, w.b.data(), 4, out, r))
            return r;
        return okVoid();
    }));

    /* break / capture */
    v.push_back(fn("dbg.break_add",
                   "value-compare break (cmp 0..5: == != < <= > >=)",
                   {P("task", "task index", ValueType::U32),
                    P("off", "blob offset", ValueType::U64),
                    P("len", "byte count ≤8", ValueType::U32),
                    P("cmp", "comparator 0..5", ValueType::U8),
                    P("val", "compare value", ValueType::U64),
                    Popt("capture", "freeze trace instead of suspend",
                         ValueType::Bool, {0})},
                   [](const std::vector<FunctionArgument>& a) {
        W w;
        w.u32((uint32_t)argU(a, 0));
        w.u64(argU(a, 1));
        w.u32((uint32_t)argU(a, 2));
        w.u8((uint8_t)argU(a, 3));
        w.u8(0); w.u8(0); w.u8(0);
        w.u64(argU(a, 4));
        uint16_t op = argU(a, 5) ? TPLC_OP_DBG_CAPTURE
                                 : TPLC_OP_DBG_BREAK;
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(op, w.b.data(), (uint32_t)w.b.size(), out, r))
            return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("dbg.break_expr",
                   "expression-triggered break/capture",
                   {P("task", "task index", ValueType::U32),
                    P("expr", "trigger expression", ValueType::String),
                    Popt("capture", "freeze trace instead of suspend",
                         ValueType::Bool, {0})},
                   [](const std::vector<FunctionArgument>& a) {
        std::string ex = argStr(a, 1);
        if (ex.size() > 256) {
            FunctionCallResult r;
            r.errorMessage = "expr > 256 chars";
            return r;
        }
        W w;
        w.u32((uint32_t)argU(a, 0));
        w.u16((uint16_t)ex.size());
        w.bytes(ex.data(), ex.size());
        uint16_t op = argU(a, 2) ? TPLC_OP_DBG_CAPTURE_EXPR
                                 : TPLC_OP_DBG_BREAK_EXPR;
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(op, w.b.data(), (uint32_t)w.b.size(), out, r))
            return r;
        uint32_t h = 0;
        if (out.size() >= 4) memcpy(&h, out.data(), 4);
        return okU32(h);
    }, ValueType::U32));
    v.push_back(fn("dbg.break_clear",
                   "clear a break (0xFFFFFFFF = all)",
                   {P("handle", "break handle", ValueType::U32)},
                   [](const std::vector<FunctionArgument>& a) {
        W w; w.u32((uint32_t)argU(a, 0, 0xFFFFFFFFu));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_DBG_BREAK_CLEAR, w.b.data(), 4, out, r))
            return r;
        return okVoid();
    }));

    /* trace */
    v.push_back(fn("trace.conf",
                   "arm the flight recorder; chans = 'off:len,…' ≤8",
                   {P("chans", "comma-separated off:len channels",
                      ValueType::String),
                    Popt("nofreeze", "don't freeze on fault",
                         ValueType::Bool, {0})},
                   [](const std::vector<FunctionArgument>& a) {
        std::string spec = argStr(a, 0);
        std::vector<std::pair<uint64_t, uint32_t>> ch;
        size_t at = 0;
        while (at <= spec.size() && ch.size() < 8) {
            size_t c = spec.find(',', at);
            std::string it = spec.substr(
                at, c == std::string::npos ? c : c - at);
            uint64_t off; uint32_t l;
            if (!it.empty() &&
                sscanf(it.c_str(), "%llu:%u",
                       (unsigned long long*)&off, &l) == 2 &&
                l >= 1 && l <= 8)
                ch.push_back({off, l});
            else if (!it.empty()) {
                FunctionCallResult r;
                r.errorMessage = "bad channel " + it;
                return r;
            }
            if (c == std::string::npos) break;
            at = c + 1;
        }
        W w;
        w.u32((uint32_t)ch.size());
        w.u32(argU(a, 1) ? 1 : 0);
        for (auto& c : ch) { w.u64(c.first); w.u32(c.second); w.u32(0); }
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TRACE_CONF, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        uint32_t fl = 0;
        if (out.size() >= 4) memcpy(&fl, out.data(), 4);
        return okU32(fl);
    }, ValueType::U32));
    v.push_back(fn("trace.read", "read trace frames",
                   {Popt("from", "first seq", ValueType::U64,
                         {0,0,0,0,0,0,0,0}),
                    Popt("max", "frame cap", ValueType::U32,
                         {0,2,0,0})},
                   [](const std::vector<FunctionArgument>& a) {
        W w;
        w.u64(argU(a, 0));
        w.u32((uint32_t)argU(a, 1, 512));
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TRACE_READ, w.b.data(),
                     (uint32_t)w.b.size(), out, r)) return r;
        return okStr(traceReadJson(out));
    }, ValueType::String));
    v.push_back(fn("trace.clear", "clear the trace ring", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_TRACE_CLEAR, out, r)) return r;
        return okVoid();
    }));

    /* retain */
    v.push_back(fn("retain.flush", "flush retain store now", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_RETAIN_FLUSH, out, r)) return r;
        return okVoid();
    }));
    v.push_back(fn("retain.dump", "retain region bytes", {},
                   [](const std::vector<FunctionArgument>&) {
        std::vector<uint8_t> out;
        FunctionCallResult r;
        if (!ctlCall(TPLC_OP_RETAIN_DUMP, out, r)) return r;
        return okBin(out);
    }, ValueType::Binary));

    return v;
}

/* ---- signals ------------------------------------------------------ */

/* PROG_STATUS → struct, ~20 ms cached so a signal batch costs one
   ctl round-trip (mutex covers the cache; sessions are threaded)  */
} // namespace tplcio_int
