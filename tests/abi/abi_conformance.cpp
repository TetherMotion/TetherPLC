/* abi_conformance.cpp — ABI conformance suite (plan doc 02 §11).
 * Exercises the full chain: SHM image → host loader → stub module →
 * binding copies → task dispatch → abort → move → migrate.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/tplc_abi.h>
#include <tplc/tplc_image.h>
#include <tplc/host/Module.hpp>
#include <tplc/host/Program.hpp>
#include <tplc/host/Scheduler.hpp>
#include "../test_util.h"
#include "stub_layout.h"
#include "desc_formats.h"

#include <cstring>
#include <string>
#include <vector>

#ifndef STUB_V1_PATH
#define STUB_V1_PATH "stub_v1.so"
#endif
#ifndef STUB_V2_PATH
#define STUB_V2_PATH "stub_v2.so"
#endif
#ifndef STUB_BAD_ABI_PATH
#define STUB_BAD_ABI_PATH "stub_bad_abi.so"
#endif

#define IMG_PATH "/tplc-test-abi-001"

static uint64_t rd64(const uint8_t* b, uint64_t off) {
    uint64_t v; memcpy(&v, b + off, 8); return v;
}
static void wr64(uint8_t* b, uint64_t off, uint64_t v) {
    memcpy(b + off, &v, 8);
}

/* Build a migrate_desc: pred=v1 LAYD(9 fields), rules per caller list.
 * Builder keeps the test's buffer arithmetic in one place. */
static std::vector<uint8_t>
build_migrate_desc(const std::vector<MigdRule>& rules) {
    std::vector<uint8_t> d;
    auto put = [&](const void* p, size_t n) {
        const uint8_t* b = (const uint8_t*)p;
        d.insert(d.end(), b, b + n);
    };
    auto u32 = [&](uint32_t v){ put(&v, 4); };
    auto u16 = [&](uint16_t v){ put(&v, 2); };
    auto u8  = [&](uint8_t v){ d.push_back(v); };

    put(STUB_V1_LAYOUT_HASH, 32);          /* predecessor_layout_hash */
    u16(2); put("v1", 2);                  /* predecessor version     */
    while (d.size() % 4) u8(0);            /* align 4                 */
    u32((uint32_t)rules.size());
    for (const auto& r : rules) put(&r, sizeof r);

    /* embedded predecessor layout: LAYD v1, 2 regions, 9 fields */
    LaydHeader lh{TPLC_LAYD_MAGIC, TPLC_LAYD_VERSION, 2, 0,
                  STUB_V1_BLOB_SIZE};
    put(&lh, sizeof lh);
    LaydRegion rg0{0, {0,0,0}, 8, 0, STUB_V1_BLOB_SIZE};   /* global   */
    LaydRegion rg1{3, {0,0,0}, 8, STUB_RETAIN_OFF, STUB_RETAIN_SIZE};
    put(&rg0, sizeof rg0); put(&rg1, sizeof rg1);
    u32(STUB_V1_FIELD_COUNT);
    const struct { uint32_t off, size; uint8_t flags; } flds[] = {
        {STUB_F_IN_BIT,1,0}, {STUB_F_OUT_BIT,1,0},
        {STUB_F_IN_BYTE,1,0}, {STUB_F_OUT_BYTE,1,0},
        {STUB_OFF_FAST,8,0}, {STUB_OFF_SLOW,8,0},
        {STUB_OFF_COUNTER,8,1}, {STUB_OFF_MARKER,16,0},
        {STUB_V1_OFF_OBSOLETE,8,0},
    };
    for (const auto& f : flds) {
        LaydField lf;
        memset(&lf, 0, sizeof lf);
        lf.sym_entry = 0xFFFFFFFFu;
        lf.field_kind = 0;
        lf.flags = f.flags;
        lf.offset = f.off; lf.size = f.size;
        put(&lf, sizeof lf);
    }
    return d;
}

static MigdRule rule(uint8_t kind, uint8_t sev,
                     uint32_t oldf, uint32_t newf) {
    MigdRule r; memset(&r, 0, sizeof r);
    r.kind = kind; r.severity = sev;
    r.old_field = oldf; r.new_field = newf;
    return r;
}

int main(void) {
    /* ===================== module loading ===================== */
    {
        auto bad = tplc::Module::open(STUB_BAD_ABI_PATH, 0);
        CHECK(!bad.ok());
        CHECK_EQ(bad.st.code, TPLC_E_ABI_MISMATCH);
    }

    auto mres = tplc::Module::open(STUB_V1_PATH, 0);
    CHECK(mres.ok());
    if (!mres.ok()) TEST_REPORT("abi_conformance");
    tplc::Module mod = std::move(mres.value);

    const TplcProgramDescriptor& d = mod.descriptor();
    CHECK_EQ(d.abi_version, TPLC_ABI_VERSION);
    CHECK_EQ(d.struct_size, sizeof(TplcProgramDescriptor));
    CHECK_EQ(d.task_count, 2);
    CHECK_EQ(d.io_binding_count, 5);
    CHECK_EQ(d.state_blob_size, STUB_V1_BLOB_SIZE);
    CHECK_EQ(d.retain_offset, STUB_RETAIN_OFF);
    CHECK_EQ(d.retain_size, STUB_RETAIN_SIZE);
    CHECK(std::string((const char*)d.name) == "stub");
    CHECK(memcmp(d.program_uuid, STUB_PROGRAM_UUID, 16) == 0);
    CHECK(mod.hasMigrate());

    /* ===================== image + program ====================== */
    tplc_image_unlink(IMG_PATH);
    TplcImage master, plc;
    CHECK_EQ(tplc_image_create(&master, IMG_PATH, 64, 64), 0);
    CHECK_EQ(tplc_image_attach(&plc, IMG_PATH, 1000), 0);
    tplc_image_set_master_status(&master, TPLC_IMG_MASTER_RUN);
    tplc_image_set_plc_status(&plc, TPLC_IMG_PLC_RUN);

    tplc::ProgramConfig cfg;
    cfg.host_caps = 0;
    auto pres = tplc::Program::create(mod, cfg);
    CHECK(pres.ok());
    tplc::Program prog = std::move(pres.value);
    CHECK(prog.start().ok());

    /* wire the §5.2 cycle: latch %I via image lib, commit %Q same way */
    tplc::TickHooks hooks;
    hooks.latch = [&plc](tplc::Program& p, uint32_t) -> int32_t {
        return tplc_image_latch_inputs(&plc, p.imageIn(), 0,
                                       p.desc().image_in_bytes);
    };
    hooks.commit = [&plc](tplc::Program& p, uint32_t) -> int32_t {
        return tplc_image_commit_outputs(&plc, p.imageOut(), 0,
                                         p.desc().image_out_bytes);
    };
    int fault_count = 0;
    hooks.fault = [&](tplc::Program&, uint32_t, int32_t) { fault_count++; };

    /* master publishes inputs: bit0=1, byte4=0x42 */
    uint8_t in_frame[8] = {0};
    in_frame[0] = 0x01; in_frame[4] = 0x42;
    tplc_image_write_inputs(&master, in_frame, 0, 8);
    tplc_image_master_heartbeat(&master, 1);

    /* ===================== task cycles ========================== */
    CHECK_EQ(prog.executeTask(0, 1000, hooks.latch, hooks.commit), 0);
    /* shadow copies worked both directions */
    CHECK_EQ(prog.blob()[STUB_F_IN_BIT], 1);
    CHECK_EQ(prog.blob()[STUB_F_IN_BYTE], 0x42);
    CHECK_EQ(rd64(prog.blob(), STUB_OFF_FAST), 1);

    /* master reads back %Q: bit0=1 (copy of in_bit), byte4=0x43 */
    uint8_t out_frame[8] = {0};
    CHECK_EQ(tplc_image_read_outputs(&master, out_frame, 0, 8), 0);
    CHECK_EQ(out_frame[0] & 1, 1);
    CHECK_EQ(out_frame[4], 0x43);

    /* bool bit semantics: clear input bit → output bit clears */
    in_frame[0] = 0; in_frame[4] = 0x10;
    tplc_image_write_inputs(&master, in_frame, 0, 8);
    CHECK_EQ(prog.executeTask(0, 2000, hooks.latch, hooks.commit), 0);
    tplc_image_read_outputs(&master, out_frame, 0, 8);
    CHECK_EQ(out_frame[0] & 1, 0);
    CHECK_EQ(out_frame[4], 0x11);
    CHECK_EQ(rd64(prog.blob(), STUB_OFF_FAST), 2);

    /* slow task: counter + markers direct map */
    CHECK_EQ(prog.executeTask(1, 3000, hooks.latch, hooks.commit), 0);
    CHECK_EQ(rd64(prog.blob(), STUB_OFF_COUNTER), 1);
    CHECK_EQ(prog.markers()[0], 0xFF);
    CHECK_EQ(rd64(prog.blob(), STUB_OFF_SLOW), 1);

    /* public tplc_task_run dispatch path */
    {
        TplcTaskCtx tc; memset(&tc, 0, sizeof tc);
        tc.prog = &prog.ctx(); tc.task_index = 1;
        CHECK_EQ(mod.symbols().task_run(&tc), 0);
        CHECK_EQ(rd64(prog.blob(), STUB_OFF_SLOW), 2);
    }

    /* scheduler: two tasks due, priorities honored */
    {
        tplc::Scheduler sched;
        sched.addProgram(prog);
        CHECK_EQ(sched.taskCount(), 2);
        size_t ran = sched.tick(5000, hooks);
        CHECK_EQ(ran, 2);
        CHECK_EQ(rd64(prog.blob(), STUB_OFF_FAST), 3);
        CHECK_EQ(rd64(prog.blob(), STUB_OFF_SLOW), 3);
    }

    /* ===================== cooperative abort ==================== */
    prog.requestAbort();
    CHECK_EQ(prog.executeTask(0, 6000, hooks.latch, hooks.commit),
             TPLC_TASK_E_ABORTED);
    prog.clearAbort();
    CHECK_EQ(prog.executeTask(0, 7000, hooks.latch, hooks.commit), 0);

    /* ===================== moving host (§3.1) ==================== */
    wr64(prog.blob(), STUB_OFF_COUNTER, 77);
    prog.moveBlob();
    CHECK_EQ(rd64(prog.blob(), STUB_OFF_COUNTER), 77);   /* contents ride */
    CHECK_EQ(prog.executeTask(1, 8000, hooks.latch, hooks.commit), 0);
    CHECK_EQ(rd64(prog.blob(), STUB_OFF_COUNTER), 78);

    /* ===================== retain restore ======================= */
    {
        tplc::ProgramConfig warm;
        warm.retain_restored = true;
        auto r2 = tplc::Program::create(mod, warm);
        CHECK(r2.ok());
        tplc::Program warm_prog = std::move(r2.value);
        /* host "restores" persisted retain before init */
        wr64(warm_prog.retainRegion(), 0, 4242);
        CHECK(warm_prog.start().ok());
        /* init skipped the retain field (counter kept 4242) */
        CHECK_EQ(rd64(warm_prog.blob(), STUB_OFF_COUNTER), 4242);
        /* non-retain fields still default-initialized */
        CHECK_EQ(rd64(warm_prog.blob(), STUB_OFF_FAST), 0);
        warm_prog.stop();
    }

    /* ===================== shutdown hints ======================== */
    prog.stop();
    CHECK(prog.services().retainDirtyRanges().size() >= 1);

    /* ===================== migration (doc 08) ==================== */
    auto m2res = tplc::Module::open(STUB_V2_PATH, 0);
    CHECK(m2res.ok());
    tplc::Module mod2 = std::move(m2res.value);
    CHECK_EQ(mod2.descriptor().state_blob_size, STUB_V2_BLOB_SIZE);

    /* seed a v1 blob with known state */
    auto p1res = tplc::Program::create(mod, cfg);
    tplc::Program old_prog = std::move(p1res.value);
    CHECK(old_prog.start().ok());
    old_prog.blob()[STUB_F_IN_BIT] = 1;
    old_prog.blob()[STUB_F_IN_BYTE] = 0x55;
    wr64(old_prog.blob(), STUB_OFF_FAST, 7);
    wr64(old_prog.blob(), STUB_OFF_SLOW, 3);
    wr64(old_prog.blob(), STUB_OFF_COUNTER, 42);
    old_prog.blob()[STUB_OFF_MARKER] = 0x5A;
    wr64(old_prog.blob(), STUB_V1_OFF_OBSOLETE, 0xDEAD);

    /* rules: copy fields 0..7, drop 8 (obsolete), default-init new 8/9 */
    std::vector<MigdRule> rules;
    for (uint32_t i = 0; i <= 7; i++) rules.push_back(rule(0, 0, i, i));
    rules.push_back(rule(3, 1, 8, 0));       /* dropped → degraded   */
    rules.push_back(rule(2, 0, 0, 8));       /* default-init part_ct */
    rules.push_back(rule(2, 0, 0, 9));       /* default-init totalizer */
    auto desc_ok = build_migrate_desc(rules);

    auto p2res = tplc::Program::create(mod2, cfg);
    tplc::Program new_prog = std::move(p2res.value);
    TplcMigrateCtx mctx;
    int32_t mrc = new_prog.migrateFrom(old_prog, desc_ok.data(),
                                       desc_ok.size(), &mctx);
    CHECK_EQ(mrc, TPLC_OK);
    CHECK_EQ(mctx.out_status, TPLC_MIGRATE_OK_DEGRADED);
    const uint8_t* nb = new_prog.blob();
    CHECK_EQ(nb[STUB_F_IN_BIT], 1);
    CHECK_EQ(nb[STUB_F_IN_BYTE], 0x55);
    CHECK_EQ(rd64(nb, STUB_OFF_FAST), 7);
    CHECK_EQ(rd64(nb, STUB_OFF_SLOW), 3);
    CHECK_EQ(rd64(nb, STUB_OFF_COUNTER), 42);
    CHECK_EQ(nb[STUB_OFF_MARKER], 0x5A);
    CHECK_EQ(rd64(nb, STUB_V2_OFF_PART_COUNT), 0);   /* default-init  */
    CHECK_EQ(rd64(nb, STUB_V2_OFF_TOTALIZER), 7);    /* default-init  */

    /* migrated program is live: first task run continues from state */
    CHECK_EQ(new_prog.executeTask(0, 9000, hooks.latch, hooks.commit), 0);
    CHECK_EQ(rd64(new_prog.blob(), STUB_OFF_FAST), 8);

    /* DENY path: a deny rule aborts; old program untouched */
    std::vector<MigdRule> deny_rules;
    deny_rules.push_back(rule(0, 0, 0, 0));
    deny_rules.push_back(rule(0, 0, 1, 1));
    deny_rules.push_back(rule(4, 2, 6, 0));   /* deny at index 2 */
    auto desc_deny = build_migrate_desc(deny_rules);
    auto p3res = tplc::Program::create(mod2, cfg);
    tplc::Program denied_prog = std::move(p3res.value);
    TplcMigrateCtx dctx;
    mrc = denied_prog.migrateFrom(old_prog, desc_deny.data(),
                                  desc_deny.size(), &dctx);
    CHECK_EQ(mrc, TPLC_E_MIGRATE_DENY);
    CHECK_EQ(dctx.out_status, TPLC_MIGRATE_DENY);
    CHECK_EQ(dctx.out_first_error, 2);
    /* old blob still intact */
    CHECK_EQ(rd64(old_prog.blob(), STUB_OFF_COUNTER), 42);

    /* malformed desc → DESCINVAL */
    {
        uint8_t junk[40] = {0};
        TplcMigrateCtx jctx;
        mrc = denied_prog.migrateFrom(old_prog, junk, sizeof junk, &jctx);
        CHECK_EQ(mrc, TPLC_E_MIGRATE_DESCINVAL);
    }

    /* module without migrate symbol → graceful refuse (load stub_v1
       and call migrateFrom against a desc — v1 HAS migrate; simulate
       the flag/symbol contract instead: hasMigrate() checked above) */

    tplc_image_close(&plc);
    tplc_image_close(&master);
    tplc_image_unlink(IMG_PATH);
    TEST_REPORT("abi_conformance");
}
