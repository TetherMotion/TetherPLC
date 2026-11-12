/* SymService.hpp — tplc-sym by-name symbol access.
 * Serves reads/writes of program variables by name against the
 * live state blob, using the package's exported variable table
 * (`tplc_get_vars`).  The loaded program attaches its table and
 * blob here; lookups are a linear scan over the (small) table.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_SYM_SERVICE_HPP
#define TPLC_SYM_SERVICE_HPP

#include <tplc/tplc_sym.h>

namespace tplc {

class SymService {
public:
    SymService();
    const TplcSymService& table() const { return svc_; }

    /* attach the loaded program's table + live blob (null to
       detach when the program unloads) */
    void attach(const TplcVarDesc* vars, uint32_t count, void* blob) {
        vars_ = vars;
        count_ = count;
        blob_ = blob;
    }
    void detach() { attach(nullptr, 0, nullptr); }

private:
    static SymService* self(void* s) {
        return static_cast<SymService*>(
            reinterpret_cast<TplcSymService*>(s)->host);
    }

    const TplcVarDesc* find(const char* name) const;

    const TplcVarDesc* vars_ = nullptr;
    uint32_t count_ = 0;
    void* blob_ = nullptr;
    TplcSymService svc_{};

    uint32_t count(void*);
    int32_t at(void*, uint32_t idx, TplcVarDesc* out);
    int32_t find_fn(void*, const char* name, TplcVarDesc* out);
    int32_t read(void*, const char* name, uint8_t* out, uint32_t cap,
                 uint32_t* out_len);
    int32_t write(void*, const char* name, const uint8_t* in,
                  uint32_t len);
};

} /* namespace tplc */
#endif /* TPLC_SYM_SERVICE_HPP */
