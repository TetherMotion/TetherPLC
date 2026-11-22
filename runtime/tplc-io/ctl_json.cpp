#include "tplc_io_internal.hpp"

namespace tplcio_int {

std::string progStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() < sizeof(TplcCtlProgStatus)) return "{}";
    const TplcCtlProgStatus* ps = (const TplcCtlProgStatus*)p.data();
    J j; j.chr('{');
    j.key("uuid", 0).hex(ps->program_uuid, 16);
    j.key("name", 1).strv(std::string((const char*)ps->name,
        strnlen((const char*)ps->name, sizeof ps->name)));
    j.key("state", 1).num(ps->state);
    j.key("cycles", 1).num(ps->cycles);
    j.key("last_fault_code", 1).num(ps->last_fault_code);
    j.key("force_count", 1).num(ps->force_count);
    j.key("version", 1).strv(std::string((const char*)ps->version,
        strnlen((const char*)ps->version, sizeof ps->version)));
    j.key("package_hash", 1).hex(ps->package_hash, 32);
    j.key("layout_hash", 1).hex(ps->layout_hash, 32);
    j.key("fault_task", 1).num(ps->fault_task);
    j.key("fault_cycle", 1).num(ps->fault_cycle);
    j.key("retain_restored", 1).num(ps->retain_restored);
    j.key("retain_dirty_bytes", 1).num(ps->retain_dirty_bytes);
    j.key("retain_last_flush_ns", 1).num(ps->retain_last_flush_ns);
    j.key("deployed_at_ns", 1).num(ps->deployed_at_ns);
    j.key("deployed_by_uid", 1).num(ps->deployed_by_uid);
    j.key("predecessor_uuid", 1).hex(ps->predecessor_uuid, 16);
    j.key("swap_stage", 1).num(ps->swap_stage);
    j.key("quarantine_left", 1).num(ps->quarantine_left);
    j.key("epoch", 1).num(ps->epoch);
    j.key("shadow_state", 1).num(ps->shadow_state);
    j.key("shadow_cycles", 1).num(ps->shadow_cycles);
    j.key("shadow_missed", 1).num(ps->shadow_missed);
    j.key("shadow_diff_count", 1).num(ps->shadow_diff_count);
    j.key("shadow_last_diff_cycle", 1).num(ps->shadow_last_diff_cycle);
    j.key("tasks", 1).chr('[');
    const uint8_t* q = p.data() + sizeof(TplcCtlProgStatus);
    size_t left = p.size() - sizeof(TplcCtlProgStatus);
    for (uint32_t i = 0; i < ps->task_count &&
                        left >= sizeof(TplcCtlTaskStat); i++) {
        const TplcCtlTaskStat* t = (const TplcCtlTaskStat*)q;
        if (i) j.chr(',');
        j.chr('{');
        j.key("name", 0).strv(std::string((const char*)t->name,
            strnlen((const char*)t->name, sizeof t->name)));
        j.key("kind", 1).num(t->kind);
        j.key("state", 1).num(t->state);
        j.key("period_ns", 1).num(t->period_ns);
        j.key("runs", 1).num(t->runs);
        j.key("late", 1).num(t->late);
        j.key("faults", 1).num(t->faults);
        j.key("last_rc", 1).num(t->last_rc);
        j.key("overruns", 1).num(t->overruns);
        j.key("exec_max_ns", 1).num(t->exec_max_ns);
        j.key("exec_avg_ns", 1).num(t->exec_avg_ns);
        j.chr('}');
        q += sizeof(TplcCtlTaskStat);
        left -= sizeof(TplcCtlTaskStat);
    }
    j.chr(']').chr('}');
    return j.s;
}

/* SHADOW_STATUS wire: u8 0 (no shadow) or TplcCtlShadowStatus    */
std::string shadowStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() == 1 && p[0] == 0)
        return "{\"loaded\":false}";
    if (p.size() < sizeof(TplcCtlShadowStatus)) return "{}";
    const TplcCtlShadowStatus* ss = (const TplcCtlShadowStatus*)p.data();
    J j; j.chr('{');
    j.key("loaded", 0).num(1);
    j.key("uuid", 1).hex(ss->program_uuid, 16);
    j.key("name", 1).strv(std::string((const char*)ss->name,
        strnlen((const char*)ss->name, sizeof ss->name)));
    j.key("state", 1).num(ss->state);
    j.key("cycles", 1).num(ss->cycles);
    j.key("missed", 1).num(ss->missed);
    j.key("diff_count", 1).num(ss->diff_count);
    j.key("last_diff_cycle", 1).num(ss->last_diff_cycle);
    j.key("runs", 1).num(ss->runs);
    j.chr('}');
    return j.s;
}

/* shadow selector wire tail {u16 slen, sel}: sel is the 16-byte
   program uuid (passed as 32 hex chars) or the program name       */
void wsel(W& w, const std::string& sel) {
    std::vector<uint8_t> b;
    if (sel.size() == 32) {
        auto nib = [](char c) -> int {
            if (c >= '0' && c <= '9') return c - '0';
            if (c >= 'a' && c <= 'f') return c - 'a' + 10;
            if (c >= 'A' && c <= 'F') return c - 'A' + 10;
            return -1;
        };
        bool ok = true;
        for (int i = 0; i < 16 && ok; i++) {
            int hi = nib(sel[i * 2]), lo = nib(sel[i * 2 + 1]);
            if (hi < 0 || lo < 0) { ok = false; break; }
            b.push_back((uint8_t)(hi * 16 + lo));
        }
        if (!ok) b.clear();
    }
    if (b.empty()) b.assign(sel.begin(), sel.end());
    if (b.empty() || b.size() > 0xFFFF) return;
    w.u16((uint16_t)b.size());
    w.bytes(b.data(), (uint32_t)b.size());
}

/* SHADOW_LIST wire: {u32 n, TplcCtlShadowStatus[n]}               */
std::string shadowListJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"shadows\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) { j.raw("]}"); return j.s; }
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n &&
                        left >= sizeof(TplcCtlShadowStatus);
         i++) {
        const TplcCtlShadowStatus* ss =
            (const TplcCtlShadowStatus*)q;
        if (i) j.chr(',');
        j.chr('{');
        j.key("uuid", 0).hex(ss->program_uuid, 16);
        j.key("name", 1).strv(std::string(
            (const char*)ss->name,
            strnlen((const char*)ss->name, sizeof ss->name)));
        j.key("state", 1).num(ss->state);
        j.key("cycles", 1).num(ss->cycles);
        j.key("missed", 1).num(ss->missed);
        j.key("diff_count", 1).num(ss->diff_count);
        j.key("last_diff_cycle", 1).num(ss->last_diff_cycle);
        j.key("runs", 1).num(ss->runs);
        j.chr('}');
        q += sizeof(TplcCtlShadowStatus);
        left -= sizeof(TplcCtlShadowStatus);
    }
    j.raw("]}");
    return j.s;
}

/* SHADOW_COMPARE wire: {u32 n, TplcCtlShadowDiff[n]}             */
std::string shadowCompareJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"diffs\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) { j.raw("]}"); return j.s; }
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= sizeof(TplcCtlShadowDiff);
         i++) {
        const TplcCtlShadowDiff* df = (const TplcCtlShadowDiff*)q;
        if (i) j.chr(',');
        j.chr('{');
        j.key("cycle", 0).num(df->cycle);
        j.key("img_off", 1).num(df->img_off);
        j.key("len", 1).num(df->len);
        j.key("active", 1).hex(df->active_bytes, df->len);
        j.key("shadow", 1).hex(df->shadow_bytes, df->len);
        j.chr('}');
        q += sizeof(TplcCtlShadowDiff);
        left -= sizeof(TplcCtlShadowDiff);
    }
    j.raw("]}");
    return j.s;
}

/* migrate-result replies (load/reset/swap): {u32 status, u32 err} */
std::string migrateResultJson(const std::vector<uint8_t>& p) {
    if (p.size() == 64) {
        J j; j.chr('{');
        j.key("refused", 0).strv("SWAP_RACE");
        j.key("live", 1).hex(p.data(), 32);
        j.key("pkg_pre", 1).hex(p.data() + 32, 32);
        j.chr('}');
        return j.s;
    }
    uint32_t st = 0, fe = 0;
    if (p.size() >= 8) { memcpy(&st, p.data(), 4);
                         memcpy(&fe, p.data() + 4, 4); }
    J j; j.chr('{');
    j.key("out_status", 0).num(st);
    j.key("out_first_error", 1).num(fe);
    j.chr('}');
    return j.s;
}

std::string fieldsJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"fields\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "[]";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 25; i++) {
        uint64_t fo, fs; uint32_t tr;
        memcpy(&fo, q, 8); memcpy(&fs, q + 8, 8); memcpy(&tr, q + 16, 4);
        uint8_t kind = q[20], fl = q[21], reg = q[22], acc = q[23],
                nl = q[24];
        q += 25; left -= 25;
        if (left < nl) break;
        if (i) j.chr(',');
        j.chr('{');
        j.key("off", 0).num(fo);
        j.key("size", 1).num(fs);
        j.key("type", 1).num(tr);
        j.key("kind", 1).num(kind);
        j.key("flags", 1).num(fl);
        j.key("region", 1).num(reg);
        j.key("access", 1).num(acc);
        j.key("name", 1).strv(std::string((const char*)q, nl));
        j.chr('}');
        q += nl; left -= nl;
    }
    j.raw("]}");
    return j.s;
}

std::string resolveJson(const std::vector<uint8_t>& p) {
    if (p.size() < 40) return "{}";
    J j; j.chr('{');
    j.key("uuid", 0).hex(p.data(), 16);
    uint32_t si, tr, sz; uint64_t off;
    memcpy(&si, p.data() + 16, 4);
    memcpy(&tr, p.data() + 24, 4);
    memcpy(&off, p.data() + 28, 8);
    memcpy(&sz, p.data() + 36, 4);
    j.key("sym", 1).num(si);
    j.key("kind", 1).num(p[20]);
    j.key("flags", 1).num(p[21]);
    j.key("region", 1).num(p[22]);
    j.key("access", 1).num(p[23]);
    j.key("type", 1).num(tr);
    j.key("off", 1).num(off);
    j.key("size", 1).num(sz);
    j.chr('}');
    return j.s;
}

std::string stateJson(const std::vector<uint8_t>& p) {
    if (p.size() < 44) return "{}";
    uint64_t bs, ro, rl; uint32_t sc;
    memcpy(&bs, p.data() + 16, 8);
    memcpy(&ro, p.data() + 24, 8);
    memcpy(&rl, p.data() + 32, 8);
    memcpy(&sc, p.data() + 40, 4);
    J j; j.chr('{');
    j.key("uuid", 0).hex(p.data(), 16);
    j.key("blob_size", 1).num(bs);
    j.key("retain_off", 1).num(ro);
    j.key("retain_len", 1).num(rl);
    j.key("symbol_count", 1).num(sc);
    j.chr('}');
    return j.s;
}

std::string forceListJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"forces\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 16; i++) {
        uint32_t h, l; uint64_t off;
        memcpy(&h, q, 4); memcpy(&off, q + 4, 8); memcpy(&l, q + 12, 4);
        q += 16; left -= 16;
        if (left < l) break;
        if (i) j.chr(',');
        j.chr('{');
        j.key("handle", 0).num(h);
        j.key("off", 1).num(off);
        j.key("len", 1).num(l);
        j.key("data", 1).hex(q, l);
        q += l; left -= l;
        j.chr('}');
        /* optional mask row follows when present */
        if (left >= l) { q += l; left -= l; }
    }
    j.raw("]}");
    return j.s;
}

std::string breakListJson(const std::vector<uint8_t>& p) {
    static const char* cmps[] = {"==","!=","<","<=",">",">="};
    J j; j.raw("{\"breaks\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 40; i++) {
        uint32_t h, tix, l; uint64_t off, val, hits;
        memcpy(&h, q, 4); memcpy(&tix, q + 4, 4); memcpy(&off, q + 8, 8);
        memcpy(&l, q + 16, 4);
        uint8_t c = q[20], cap = q[21], stale = q[22];
        memcpy(&val, q + 24, 8); memcpy(&hits, q + 32, 8);
        q += 40; left -= 40;
        if (i) j.chr(',');
        j.chr('{');
        j.key("handle", 0).num(h);
        j.key("task", 1).num(tix);
        j.key("capture", 1).num(cap);
        j.key("stale", 1).num(stale);
        if (c == 0xFF) j.key("expr", 1).num(1);
        else {
            j.key("off", 1).num(off);
            j.key("len", 1).num(l);
            j.key("cmp", 1).strv(cmps[c <= 5 ? c : 0]);
            j.key("val", 1).num(val);
        }
        j.key("hits", 1).num(hits);
        j.chr('}');
    }
    j.raw("]}");
    return j.s;
}

/* {u32 n} × {u32 handle, u32 nchan, u32 rate, u32 tlen, u32 stale} */
std::string watchListJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"watches\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 20; i++) {
        uint32_t h, nc, rt, tl, st;
        memcpy(&h, q, 4); memcpy(&nc, q + 4, 4); memcpy(&rt, q + 8, 4);
        memcpy(&tl, q + 12, 4); memcpy(&st, q + 16, 4);
        q += 20; left -= 20;
        if (i) j.chr(',');
        j.chr('{');
        j.key("handle", 0).num(h);
        j.key("channels", 1).num(nc);
        j.key("rate_hz", 1).num(rt);
        j.key("bytes", 1).num(tl);
        j.key("stale", 1).num(st);
        j.chr('}');
    }
    j.raw("]}");
    return j.s;
}

/* state diff: {u8 same, u8 mig_status, u16 pad, u32 nrules} ×
   {u8 kind,sev, u16 tid, u32 aux, u64 ooff, u32 olen, u32 osym,
    u64 noff, u32 nlen, u32 nsym, u8 ovl, u8 nvl, u16 pad,
    8B prev, 8B next, str oname, str nname}                       */
std::string stateDiffJson(const std::vector<uint8_t>& p) {
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 8) return "{}";
    J j; j.chr('{');
    j.key("same_layout", 0).num(q[0]);
    j.key("mig_status", 1).num(q[1]);
    j.key("rules", 1).chr('[');
    q += 8; left -= 8;
    uint32_t n;
    memcpy(&n, p.data() + 4, 4);
    for (uint32_t i = 0; i < n && left >= 48; i++) {
        uint16_t tid; uint32_t aux, ol, os, nl, ns;
        uint64_t oo, no;
        memcpy(&tid, q + 2, 2); memcpy(&aux, q + 4, 4);
        memcpy(&oo, q + 8, 8); memcpy(&ol, q + 16, 4);
        memcpy(&os, q + 20, 4); memcpy(&no, q + 24, 8);
        memcpy(&nl, q + 32, 4); memcpy(&ns, q + 36, 4);
        uint8_t kind = q[0], sev = q[1], ovl = q[40], nvl = q[41];
        const uint8_t* pv = q + 44;
        const uint8_t* nv2 = q + 52;
        if (left < 60) break;
        q += 60; left -= 60;
        if (i) j.chr(',');
        j.chr('{');
        j.key("kind", 0).num(kind);
        j.key("sev", 1).num(sev);
        j.key("task", 1).num(tid);
        j.key("aux", 1).num(aux);
        j.key("old", 1).chr('{');
        j.key("sym", 0).num(os); j.key("off", 1).num(oo);
        j.key("len", 1).num(ol); j.key("live", 1).num(ovl);
        j.key("prev", 1).hex(pv, 8);
        j.chr('}');
        j.chr(',');
        j.key("new", 1).chr('{');
        j.key("sym", 0).num(ns); j.key("off", 1).num(no);
        j.key("len", 1).num(nl); j.key("live", 1).num(nvl);
        j.key("next", 1).hex(nv2, 8);
        j.chr('}');
        if (left >= 2) {
            uint16_t sl; memcpy(&sl, q, 2);
            if (left >= (size_t)sl + 2) {
                j.key("old_name", 1)
                    .strv(std::string((const char*)q + 2, sl));
                q += 2 + sl; left -= 2 + sl;
            }
        }
        if (left >= 2) {
            uint16_t sl; memcpy(&sl, q, 2);
            if (left >= (size_t)sl + 2) {
                j.key("new_name", 1)
                    .strv(std::string((const char*)q + 2, sl));
                q += 2 + sl; left -= 2 + sl;
            }
        }
        j.chr('}');
    }
    j.raw("]}");
    return j.s;
}

std::string watchGetJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"items\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 4; i++) {
        uint32_t l;
        memcpy(&l, q, 4); q += 4; left -= 4;
        if (left < l) break;
        if (i) j.chr(',');
        j.chr('{');
        j.key("len", 0).num(l);
        j.key("data", 1).hex(q, l);
        j.chr('}');
        q += l; left -= l;
    }
    j.raw("]}");
    return j.s;
}

std::string traceStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() < 24) return "{}";
    uint32_t nch, fl, fl2 = 0; uint64_t wr, av;
    memcpy(&nch, p.data(), 4); memcpy(&fl, p.data() + 4, 4);
    memcpy(&wr, p.data() + 8, 8); memcpy(&av, p.data() + 16, 8);
    if (p.size() >= 28) memcpy(&fl2, p.data() + 24, 4);
    J j; j.chr('{');
    j.key("channels", 0).num(nch);
    j.key("frame_len", 1).num(fl);
    j.key("written", 1).num(wr);
    j.key("available", 1).num(av);
    j.key("frozen", 1).num(fl2 & 1);
    j.chr('}');
    return j.s;
}

std::string traceReadJson(const std::vector<uint8_t>& p) {
    if (p.size() < 16) return "{}";
    const uint8_t* q = p.data();
    uint64_t first; uint32_t n, fl;
    memcpy(&first, q, 8); memcpy(&n, q + 8, 4); memcpy(&fl, q + 12, 4);
    q += 16;
    size_t left = p.size() - 16;
    J j; j.chr('{');
    j.key("first_seq", 0).num(first);
    j.key("frame_len", 1).num(fl);
    j.key("frames", 1).chr('[');
    for (uint32_t i = 0; i < n && left >= 16 + fl; i++) {
        uint64_t s, ts;
        memcpy(&s, q, 8); memcpy(&ts, q + 8, 8);
        if (i) j.chr(',');
        j.chr('{');
        j.key("seq", 0).num(s);
        j.key("ts", 1).num(ts);
        j.key("data", 1).hex(q + 16, fl);
        j.chr('}');
        q += 16 + fl; left -= 16 + fl;
    }
    j.raw("]}");
    return j.s;
}

std::string retainStatusJson(const std::vector<uint8_t>& p) {
    if (p.size() < 44) return "{}";
    uint64_t rlen, rgen, rfl, rrs, rls;
    memcpy(&rlen, p.data() + 4, 8);  memcpy(&rgen, p.data() + 12, 8);
    memcpy(&rfl,  p.data() + 20, 8); memcpy(&rrs,  p.data() + 28, 8);
    memcpy(&rls,  p.data() + 36, 8);
    J j; j.chr('{');
    j.key("attached", 0).num(p[0]);
    j.key("dirty", 1).num(p[1]);
    j.key("len", 1).num(rlen);
    j.key("gen", 1).num(rgen);
    j.key("flushes", 1).num(rfl);
    j.key("restores", 1).num(rrs);
    j.key("losses", 1).num(rls);
    j.chr('}');
    return j.s;
}

std::string imageStatsJson(const std::vector<uint8_t>& p) {
    if (p.size() < sizeof(TplcCtlImageStats)) return "{}";
    const TplcCtlImageStats* is = (const TplcCtlImageStats*)p.data();
    J j; j.chr('{');
    j.key("epoch", 0).num(is->epoch);
    j.key("in_seq", 1).num(is->in_seq);
    j.key("out_seq", 1).num(is->out_seq);
    j.key("in_writes", 1).num(is->in_writes);
    j.key("out_writes", 1).num(is->out_writes);
    j.key("in_retries", 1).num(is->in_retries);
    j.key("out_retries", 1).num(is->out_retries);
    j.key("in_size", 1).num(is->in_size);
    j.key("out_size", 1).num(is->out_size);
    j.chr('}');
    return j.s;
}

std::string journalJson(const std::vector<uint8_t>& p) {
    J j; j.raw("{\"lines\":[");
    const uint8_t* q = p.data();
    size_t left = p.size();
    if (left < 4) return "{}";
    uint32_t n;
    memcpy(&n, q, 4); q += 4; left -= 4;
    for (uint32_t i = 0; i < n && left >= 2; i++) {
        uint16_t l;
        memcpy(&l, q, 2); q += 2; left -= 2;
        if (left < l) break;
        if (i) j.chr(',');
        j.strv(std::string((const char*)q, l));
        q += l; left -= l;
    }
    j.raw("]}");
    return j.s;
}

std::string stateChangeJson(const std::vector<uint8_t>& p) {
    if (p.size() < 4) return "{}";
    J j; j.chr('{');
    j.key("from", 0).num(p[0]);
    j.key("to", 1).num(p[1]);
    j.chr('}');
    return j.s;
}
} // namespace tplcio_int
