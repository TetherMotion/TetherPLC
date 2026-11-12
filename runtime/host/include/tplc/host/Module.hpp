/* Module.hpp — dlopen loader + ABI verification (plan doc 02 §1, §4.1).
 * Owns the module handle; resolves the 6-symbol ABI surface; validates
 * the descriptor against the host's capability set.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_MODULE_HPP
#define TPLC_HOST_MODULE_HPP

#include <tplc/tplc_abi.h>
#include <tplc/tplc_sym.h>
#include <tplc/host/Status.hpp>

#include <string>

namespace tplc {

struct ModuleSymbols {
    uint32_t                     (*abi_version)(void)    = nullptr;
    const TplcProgramDescriptor* (*get_descriptor)(void) = nullptr;
    int32_t                      (*init)(TplcCtx*)       = nullptr;
    void                         (*shutdown)(TplcCtx*)   = nullptr;
    int32_t                      (*task_run)(TplcTaskCtx*)   = nullptr;
    int32_t                      (*migrate)(TplcMigrateCtx*) = nullptr; /* optional */
    TplcGetVarsFn                 get_vars = nullptr;   /* optional */
};

class Module {
public:
    Module() = default;
    ~Module();
    Module(const Module&) = delete;
    Module& operator=(const Module&) = delete;
    Module(Module&& o) noexcept;
    Module& operator=(Module&& o) noexcept;

    /* dlopen + resolve + validate.
     * host_caps: the TplcAbiExt bits this host implements; open fails
     * with TPLC_E_ABI_EXT_UNMET if desc.abi_ext asks for more. */
    static Result<Module> open(const std::string& path,
                               uint32_t host_caps);

    const TplcProgramDescriptor& descriptor() const { return *desc_; }
    const ModuleSymbols&         symbols()    const { return sym_; }
    bool                         hasMigrate() const { return sym_.migrate != nullptr; }
    bool                         hasVars()    const { return sym_.get_vars != nullptr; }
    /* top-level variable table (name -> blob offset/size/type);
       null for library packages (no PROGRAM) */
    const TplcVarDesc* vars(uint32_t* count) const {
        if (!sym_.get_vars) { if (count) *count = 0; return nullptr; }
        return sym_.get_vars(count);
    }
    bool                         isOpen()     const { return handle_ != nullptr; }

    /* thin forwards */
    int32_t init(TplcCtx* c)      const { return sym_.init(c); }
    void    shutdown(TplcCtx* c)  const { sym_.shutdown(c); }
    int32_t migrate(TplcMigrateCtx* m) const { return sym_.migrate(m); }

private:
    void*                        handle_ = nullptr;
    ModuleSymbols                sym_{};
    const TplcProgramDescriptor* desc_   = nullptr;
};

} /* namespace tplc */
#endif /* TPLC_HOST_MODULE_HPP */
