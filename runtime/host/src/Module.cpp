/* Module.cpp — dlopen + ABI validation (plan doc 02 §4.1, §9).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Module.hpp>
#include <tplc/host/ElfScan.hpp>

#include <dlfcn.h>
#include <cstring>

namespace tplc {

Module::~Module() {
    if (handle_) dlclose(handle_);
}
Module::Module(Module&& o) noexcept
    : handle_(o.handle_), sym_(o.sym_), desc_(o.desc_) {
    o.handle_ = nullptr; o.sym_ = {}; o.desc_ = nullptr;
}
Module& Module::operator=(Module&& o) noexcept {
    if (this != &o) {
        if (handle_) dlclose(handle_);
        handle_ = o.handle_; sym_ = o.sym_; desc_ = o.desc_;
        o.handle_ = nullptr; o.sym_ = {}; o.desc_ = nullptr;
    }
    return *this;
}

static void* req_sym(void* h, const char* name, Status& st) {
    dlerror();
    void* p = dlsym(h, name);
    const char* e = dlerror();
    if (e || !p) {
        st = Status::Err(TPLC_E_HOST_SYM,
                         std::string("missing symbol ") + name);
        return nullptr;
    }
    return p;
}

Result<Module> Module::open(const std::string& path, uint32_t host_caps) {
    /* post-link scanner first (doc 15 §4.3): never dlopen bytes that
       carry kernel-gate opcodes or violate segment/library policy */
    Status sst = scanModuleFile(path);
    if (!sst.ok()) return Result<Module>::Err(sst);

    Module m;
    void* h = dlopen(path.c_str(), RTLD_NOW | RTLD_LOCAL);
    if (!h)
        return Result<Module>::Err(TPLC_E_HOST_DLOPEN, dlerror());
    m.handle_ = h;

    Status st;
    m.sym_.abi_version =
        (decltype(m.sym_.abi_version))req_sym(h, TPLC_SYM_ABI_VERSION, st);
    m.sym_.get_descriptor =
        (decltype(m.sym_.get_descriptor))req_sym(h, TPLC_SYM_DESCRIPTOR, st);
    m.sym_.init =
        (decltype(m.sym_.init))req_sym(h, TPLC_SYM_INIT, st);
    m.sym_.shutdown =
        (decltype(m.sym_.shutdown))req_sym(h, TPLC_SYM_SHUTDOWN, st);
    m.sym_.task_run =
        (decltype(m.sym_.task_run))req_sym(h, TPLC_SYM_TASK_RUN, st);
    if (!st.ok()) return Result<Module>::Err(st);

    dlerror();
    m.sym_.migrate =
        (decltype(m.sym_.migrate))dlsym(h, TPLC_SYM_MIGRATE);
    dlerror(); /* optional — absence is legal */

    /* ABI version gate (doc 02 §1.2: exact match in v1) */
    if (m.sym_.abi_version() != TPLC_ABI_VERSION)
        return Result<Module>::Err(TPLC_E_ABI_MISMATCH,
                                   "module abi_version " +
                                   std::to_string(m.sym_.abi_version()));

    /* descriptor validation */
    const TplcProgramDescriptor* d = m.sym_.get_descriptor();
    if (!d)
        return Result<Module>::Err(TPLC_E_HOST_DESC, "null descriptor");
    if (d->struct_size != sizeof(TplcProgramDescriptor))
        return Result<Module>::Err(TPLC_E_HOST_DESC, "descriptor size");
    if (d->abi_version != TPLC_ABI_VERSION)
        return Result<Module>::Err(TPLC_E_ABI_MISMATCH,
                                   "descriptor abi_version");
    if (d->abi_ext & ~host_caps)
        return Result<Module>::Err(TPLC_E_ABI_EXT_UNMET,
                                   "module requires abi_ext " +
                                   std::to_string(d->abi_ext & ~host_caps));
    /* name must be NUL-terminated within the fixed field */
    if (!memchr(d->name, 0, TPLC_PROG_NAME_MAX))
        return Result<Module>::Err(TPLC_E_HOST_DESC, "name not terminated");
    if ((d->flags & 1u) && !m.sym_.migrate)
        return Result<Module>::Err(TPLC_E_HOST_DESC,
                                   "has_migrate flag but no symbol");
    if (d->task_count == 0 || !d->tasks || !d->task_fns ||
        d->task_fn_count == 0)
        return Result<Module>::Err(TPLC_E_HOST_DESC, "empty task table");
    for (uint32_t i = 0; i < d->task_count; i++) {
        const TplcTaskDesc& t = d->tasks[i];
        if (t.entry_index >= d->task_fn_count)
            return Result<Module>::Err(TPLC_E_HOST_DESC,
                                       "task entry_index out of table");
        if (t.kind == TPLC_TASK_CYCLIC && t.period_ns == 0)
            return Result<Module>::Err(TPLC_E_HOST_DESC,
                                       "cyclic task without period");
        if (t.kind == TPLC_TASK_EVENT || t.kind == TPLC_TASK_RESERVED_SFC)
            return Result<Module>::Err(TPLC_E_ABI_EXT_UNMET,
                                       "task kind not implemented in v1");
    }
    if (d->io_binding_count && !d->io_bindings)
        return Result<Module>::Err(TPLC_E_HOST_DESC, "null io_bindings");
    if (d->retain_size &&
        d->retain_offset + d->retain_size > d->state_blob_size)
        return Result<Module>::Err(TPLC_E_HOST_RANGE, "retain range");

    m.desc_ = d;
    return Result<Module>::Ok(std::move(m));
}

} /* namespace tplc */
