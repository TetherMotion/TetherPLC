/* Program.cpp — program instance lifecycle + the per-task execute
 * sequence (plan doc 02 §4.2–§4.4, §5; doc 03 §5.2).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Program.hpp>
#include <ctime>

static uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

#include <cstdlib>
#include <cstring>

namespace tplc {

static uint8_t* alloc64(uint64_t size) {
    if (size == 0) return nullptr;
    void* p = nullptr;
    if (posix_memalign(&p, 64, (size + 63u) & ~63ull) != 0) return nullptr;
    memset(p, 0, (size_t)size);
    return static_cast<uint8_t*>(p);
}

Result<Program> Program::create(const Module& mod,
                                const ProgramConfig& cfg) {
    Program p;
    p.mod_ = &mod;
    const TplcProgramDescriptor& d = mod.descriptor();

    p.blob_size_ = d.state_blob_size;
    p.blob_.p    = alloc64(d.state_blob_size);
    p.img_in_.p  = alloc64(d.image_in_bytes ? d.image_in_bytes : 1);
    p.img_out_.p = alloc64(d.image_out_bytes ? d.image_out_bytes : 1);
    p.markers_.p = alloc64(d.marker_bytes ? d.marker_bytes : 1);
    if ((d.state_blob_size && !p.blob_.p) || !p.img_in_.p ||
        !p.img_out_.p || !p.markers_.p)
        return Result<Program>::Err(TPLC_E_INIT_RESOURCE,
                                    "region allocation failed");

    p.cfg_ = cfg;
    p.services_ = std::make_unique<HostServices>(cfg.host_caps);

    TplcCtx& c = p.ctx_;
    memset(&c, 0, sizeof c);
    c.struct_size    = sizeof(TplcCtx);
    c.ctx_flags      = cfg.ctx_flags;
    c.state_blob     = p.blob_.p;
    c.retain_region  = p.blob_.p + d.retain_offset;
    c.image_in       = p.img_in_.p;
    c.image_out      = p.img_out_.p;
    c.markers        = p.markers_.p;
    c.svc            = &p.services_->table();
    if (cfg.retain_restored) c.ctx_flags |= TPLC_CTXF_RETAIN_RESTORED;

    p.attachSymbols();

    p.tctxs_.resize(d.task_count);
    return Result<Program>::Ok(std::move(p));
}

Status Program::start() {
    if (started_) return Status::Err(TPLC_E_HOST_STATE, "already started");
    attachSymbols();
    int32_t rc = mod_->init(&ctx_);
    if (rc != 0)
        return Status::Err(rc, "tplc_init failed");
    started_ = true;
    running_ = true;
    return Status::Ok();
}

void Program::attachSymbols() {
    if (!services_) return;
    uint32_t nvars = 0;
    const TplcVarDesc* vars = mod_ ? mod_->vars(&nvars) : nullptr;
    services_->symbols().attach(vars, nvars, blob_.p);
}

void Program::stop() {
    if (services_) services_->symbols().detach();
    if (running_) mod_->shutdown(&ctx_);
    running_ = false;
}

Status Program::reset(bool warm) {
    stop();
    ctx_.abort_flag = 0;
    if (warm) ctx_.ctx_flags |=  TPLC_CTXF_RETAIN_RESTORED;
    else      ctx_.ctx_flags &= ~((uint32_t)TPLC_CTXF_RETAIN_RESTORED);
    attachSymbols();     /* stop() detached; reset skips start() */
    int32_t rc = mod_->init(&ctx_);
    if (rc != 0) return Status::Err(rc, "tplc_init failed (reset)");
    running_ = true;
    return Status::Ok();
}

/* ---- binding copies (doc 02 §2.4 semantics) ----------------------- */

static uint32_t type_size(uint8_t tc) {
    switch (tc) {
    case TPLC_IOT_BOOL: case TPLC_IOT_U8: case TPLC_IOT_S8: return 1;
    case TPLC_IOT_U16: case TPLC_IOT_S16: return 2;
    case TPLC_IOT_U32: case TPLC_IOT_S32: case TPLC_IOT_F32: return 4;
    case TPLC_IOT_U64: case TPLC_IOT_S64: case TPLC_IOT_F64: return 8;
    default: return 0;
    }
}

static void copy_one(const TplcIoBinding& b,
                     const uint8_t* src_img, uint8_t* dst_blob,
                     const uint8_t* src_blob, uint8_t* dst_img,
                     bool to_image) {
    const uint8_t* src = to_image ? src_blob + b.blob_offset
                                : src_img + b.image_offset;
    uint8_t* dst       = to_image ? dst_img + b.image_offset
                                : dst_blob + b.blob_offset;
    if (b.type_code == TPLC_IOT_BOOL) {
        uint8_t bit = (uint8_t)(1u << (b.unit & 7u));
        if (to_image) {
            if (*src) *dst |= bit; else *dst &= (uint8_t)~bit;
        } else {
            *dst = (*src & bit) ? 1 : 0;
        }
    } else {
        uint32_t n = b.size ? b.size : type_size(b.type_code);
        memcpy(dst, src, n);
    }
}

void Program::copyInputsIn() {
    const TplcProgramDescriptor& d = desc();
    for (uint32_t i = 0; i < d.io_binding_count; i++) {
        const TplcIoBinding& b = d.io_bindings[i];
        if (b.region == TPLC_REGION_IMAGE_IN)
            copy_one(b, img_in_.p, blob_.p, blob_.p, img_out_.p, false);
        /* MARKER bindings: direct map — generated code reads markers
           through ctx->markers; no shadow copy (doc 02 §2.4) */
    }
}

void Program::copyOutputsOut() {
    const TplcProgramDescriptor& d = desc();
    for (uint32_t i = 0; i < d.io_binding_count; i++) {
        const TplcIoBinding& b = d.io_bindings[i];
        if (b.region == TPLC_REGION_IMAGE_OUT)
            copy_one(b, img_in_.p, blob_.p, blob_.p, img_out_.p, true);
    }
}

/* ---- the per-task execute cycle (doc 02 §5) ----------------------- */

int32_t Program::executeTask(uint32_t task_idx, uint64_t now_ns,
                             const IoHook& latch_in,
                             const IoHook& commit_out) {
    const TplcProgramDescriptor& d = desc();
    if (!running_ || task_idx >= d.task_count)
        return TPLC_TASK_E_FAULT;
    const TplcTaskDesc& td = d.tasks[task_idx];
    if (td.entry_index >= d.task_fn_count) return TPLC_TASK_E_FAULT;
    TplcTaskFn fn = d.task_fns[td.entry_index];
    if (!fn) return TPLC_TASK_E_FAULT;

    if (latch_in) {
        int32_t rc = latch_in(*this, task_idx);
        if (rc != 0) return rc;
    }
    copyInputsIn();

    /* refresh invocation context (doc 02 §2.5) */
    ctx_.now_ns = now_ns;
    ctx_.cycle_no = ++cycle_no_;

    /* WCET arm: checkpoints compare svc->steady_ns() against this */
    if (td.flags & TPLC_TASKF_WATCHDOGGED) {
        uint64_t hard = cfg_.wcet_hard_ns ? cfg_.wcet_hard_ns
                                          : 2 * td.wcet_hint_ns;
        ctx_.budget_deadline_ns = hard ? now_ns + hard : 0;
    } else {
        ctx_.budget_deadline_ns = 0;
    }

    TplcTaskCtx& tc = tctxs_[task_idx];
    memset(&tc, 0, sizeof tc);
    tc.prog = &ctx_;
    tc.task_index = task_idx;
    tc.scheduled_ns = now_ns;
    tc.deadline_ns = td.period_ns ? now_ns + td.period_ns : 0;
    tc.task_local = nullptr;  /* task-local regions: layout_desc-sourced,
                                 P2 item (stub uses task_local_size=0) */
    uint64_t t0 = mono_ns();
    int32_t rc = fn(&tc);
    uint64_t t1 = mono_ns();
    last_exec_ns_ = t1 - t0;
    /* signed overrun: how far past scheduled+period this cycle ran
       (negative = finished early); module-facing per doc 02 §2.6   */
    if (td.kind == TPLC_TASK_CYCLIC && td.period_ns)
        ctx_.last_cycle_overrun_ns =
            (int64_t)(t1 - (td.period_ns + now_ns));
    else
        ctx_.last_cycle_overrun_ns = 0;
    if (rc != 0) return rc;

    copyOutputsOut();
    if (commit_out) {
        int32_t crc = commit_out(*this, task_idx);
        if (crc != 0) return crc;
    }
    return TPLC_OK;
}

/* ---- accessors ----------------------------------------------------- */

const TplcProgramDescriptor& Program::desc() const {
    return mod_->descriptor();
}
uint64_t Program::blobSize() const { return blob_size_; }
uint8_t* Program::retainRegion() { return (uint8_t*)ctx_.retain_region; }
uint64_t Program::retainSize() const { return desc().retain_size; }

/* ---- migration ------------------------------------------------------ */

int32_t Program::migrateFrom(const Program& old_prog,
                             const void* migrate_desc,
                             uint64_t migrate_desc_size,
                             TplcMigrateCtx* out_mctx) {
    if (!mod_->hasMigrate()) return TPLC_E_MIGRATE_LAYOUT;

    TplcMigrateCtx mctx;
    memset(&mctx, 0, sizeof mctx);
    mctx.new_prog = &ctx_;
    mctx.old_blob = old_prog.blob();
    mctx.old_blob_size = old_prog.blobSize();
    memcpy(mctx.old_layout_hash,
           old_prog.desc().layout_hash, 32);
    mctx.old_abi_version = old_prog.desc().abi_version;
    mctx.migrate_desc = migrate_desc;
    mctx.migrate_desc_size = migrate_desc_size;

    int32_t rc = mod_->migrate(&mctx);
    if (out_mctx) *out_mctx = mctx;
    if (rc == TPLC_OK)
        running_ = true;   /* migrate produces a fully-init'd blob —
                              the instance is live (doc 02 §4.4) */
    return rc;
}

/* ---- conformance hooks --------------------------------------------- */

std::vector<uint8_t> Program::snapshotBlob() const {
    return std::vector<uint8_t>(blob_.p, blob_.p + blob_size_);
}

void Program::moveBlob() {
    Buf nb;
    nb.p = alloc64(blob_size_);
    memcpy(nb.p, blob_.p, (size_t)blob_size_);
    blob_ = std::move(nb);
    ctx_.state_blob = blob_.p;
    ctx_.retain_region = blob_.p + desc().retain_offset;
}

void Program::poisonBlob(uint8_t v) {
    memset(blob_.p, v, (size_t)blob_size_);
}

} /* namespace tplc */
