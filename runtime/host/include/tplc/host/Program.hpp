/* Program.hpp — a loaded program instance (plan doc 02 §3, §5).
 * Owns the state blob + image buffers + markers + task ctxs and the
 * per-task execute cycle (latch → copy in → run → copy out → commit).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_PROGRAM_HPP
#define TPLC_HOST_PROGRAM_HPP

#include <tplc/tplc_abi.h>
#include <tplc/host/Module.hpp>
#include <tplc/host/HostServices.hpp>
#include <tplc/host/Status.hpp>

#include <cstdint>
#include <functional>
#include <memory>
#include <vector>

namespace tplc {

struct ProgramConfig {
    uint32_t host_caps   = 0;      /* caps to advertise to the module  */
    uint32_t ctx_flags   = 0;      /* extra TPLC_CTXF_* bits           */
    bool     retain_restored = false; /* host pre-loaded retain data   */
    /* hard WCET bound for watchdoggled tasks; 0 = 2x task wcet_hint   */
    uint64_t wcet_hard_ns = 0;
    /* fault policy (doc 04 §6): 0=hold, 1=auto_restart, 2=safe_unload */
    uint8_t  fault_policy = 0;
    /* auto_restart bound: at most N restarts inside the window       */
    uint32_t restart_max = 3;
    uint64_t restart_window_ns = 60ull * 1000 * 1000 * 1000;
};

class Program {
public:
    Program() = default;
    Program(Program&&) noexcept = default;
    Program& operator=(Program&&) noexcept = default;
    Program(const Program&) = delete;
    Program& operator=(const Program&) = delete;

    /* Allocate regions (≥64 B aligned), zero them, build TplcCtx. */
    static Result<Program> create(const Module& mod,
                                  const ProgramConfig& cfg);

    /* tplc_init (service plane). retain_restored comes from config —
       the host restores persisted retain bytes into the blob BEFORE
       calling this (see retainLoad()). */
    Status start();
    void   stop();                      /* tplc_shutdown, idempotent   */
    /* stop + re-init. warm: retain bytes kept (init skips the retain
       region); cold: whole blob re-zeroed. (doc 04 §4 reset)         */
    Status reset(bool warm);
    bool   running() const { return running_; }

    /* One full task cycle (doc 02 §5 / doc 03 §5.2):
     *   latch_in()        — caller wires image/whatever; may be empty
     *   copy bound %I bindings (image→blob shadow)
     *   refresh ctx fields; run task fn
     *   on rc==0: copy bound %Q bindings (shadow→image); commit_out()
     * latch_in/commit_out return host-level rc; latch failure skips
     * the task run (task not run this cycle; rc surfaced). */
    using IoHook = std::function<int32_t(Program&, uint32_t task_idx)>;
    int32_t executeTask(uint32_t task_idx, uint64_t now_ns,
                        const IoHook& latch_in, const IoHook& commit_out);

    /* Binding copies — also usable standalone (tests). */
    void copyInputsIn();     /* image_in → blob shadows, %I bindings   */
    void copyOutputsOut();   /* blob shadows → image_out, %Q bindings  */

    /* Accessors */
    const TplcProgramDescriptor& desc() const;
    TplcCtx&    ctx()            { return ctx_; }
    const TplcCtx& ctx() const   { return ctx_; }
    uint8_t*    blob()           { return blob_.p; }
    const uint8_t* blob() const  { return blob_.p; }
    uint64_t    blobSize() const;
    uint8_t*    imageIn()        { return img_in_.p; }
    uint8_t*    imageOut()       { return img_out_.p; }
    uint8_t*    markers()        { return markers_.p; }
    HostServices& services()     { return *services_; }
    const Module& module()       { return *mod_; }

    /* Abort flag: cooperative checkpoint signal (doc 02 Appendix A). */
    void requestAbort() { ctx_.abort_flag = 1; }
    void clearAbort()   { ctx_.abort_flag = 0; }

    /* duration of the last executeTask call (budget/jitter surface)  */
    uint64_t lastExecNs() const { return last_exec_ns_; }

    /* Retain: host persistence interface (doc 04 owns flush policy) */
    uint8_t* retainRegion();
    uint64_t retainSize() const;
    /* call after restoring retain bytes into the blob, before start() */
    void setRetainRestored() { ctx_.ctx_flags |= TPLC_CTXF_RETAIN_RESTORED; }

    /* Migration (doc 08): snapshot old blob, allocate new program via
       create(), then migrateFrom(). Runs module tplc_migrate. */
    int32_t migrateFrom(const Program& old_prog,
                        const void* migrate_desc,
                        uint64_t migrate_desc_size,
                        TplcMigrateCtx* out_mctx = nullptr);

    /* Conformance hooks (tests only) */
    std::vector<uint8_t> snapshotBlob() const;
    void   moveBlob();          /* relocate blob — §3.1 movability    */
    void   poisonBlob(uint8_t v); /* scrub helper                     */

private:
    struct Buf {
        uint8_t* p = nullptr;
        ~Buf() { free(p); }
        Buf() = default;
        Buf(Buf&& o) noexcept : p(o.p) { o.p = nullptr; }
        Buf& operator=(Buf&& o) noexcept { free(p); p = o.p; o.p = nullptr; return *this; }
        Buf(const Buf&) = delete;
        Buf& operator=(const Buf&) = delete;
    };

    const Module* mod_ = nullptr;
    Buf    blob_, img_in_, img_out_, markers_;
    uint64_t blob_size_ = 0;
    TplcCtx ctx_{};
    std::unique_ptr<HostServices> services_;  /* stable address: svc cookie */
    std::vector<TplcTaskCtx> tctxs_;
    ProgramConfig cfg_;
    uint64_t last_exec_ns_ = 0;
    uint64_t cycle_no_ = 0;
    bool     running_ = false;
    bool     started_ = false;
};

} /* namespace tplc */
#endif /* TPLC_HOST_PROGRAM_HPP */
