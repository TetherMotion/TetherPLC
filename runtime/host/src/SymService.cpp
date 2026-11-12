/* SymService.cpp — tplc-sym by-name symbol access.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/SymService.hpp>

#include <cstring>

namespace tplc {

SymService::SymService() {
    svc_.struct_size = sizeof(TplcSymService);
    svc_.abi_version = TPLC_SYMSVC_ABI_VERSION;
    svc_.host = this;
    svc_.sym_count = [](void* s) { return self(s)->count(s); };
    svc_.sym_at = [](void* s, uint32_t i, TplcVarDesc* o)
        { return self(s)->at(s, i, o); };
    svc_.sym_find = [](void* s, const char* n, TplcVarDesc* o)
        { return self(s)->find_fn(s, n, o); };
    svc_.sym_read = [](void* s, const char* n, uint8_t* o, uint32_t c,
                       uint32_t* l)
        { return self(s)->read(s, n, o, c, l); };
    svc_.sym_write = [](void* s, const char* n, const uint8_t* i,
                        uint32_t l)
        { return self(s)->write(s, n, i, l); };
}

const TplcVarDesc* SymService::find(const char* name) const {
    if (!vars_ || !name) return nullptr;
    for (uint32_t i = 0; i < count_; i++) {
        const char* n = (const char*)vars_[i].name;
        if (std::strncmp(n, name, TPLC_SYM_NAME_MAX) == 0)
            return &vars_[i];
    }
    return nullptr;
}

uint32_t SymService::count(void*) { return count_; }

int32_t SymService::at(void*, uint32_t idx, TplcVarDesc* out) {
    if (!out) return TPLC_SYM_E_PARAM;
    if (idx >= count_ || !vars_) return TPLC_SYM_E_NONAME;
    *out = vars_[idx];
    return TPLC_SYM_OK;
}

int32_t SymService::find_fn(void*, const char* name, TplcVarDesc* out) {
    if (!name || !out) return TPLC_SYM_E_PARAM;
    const TplcVarDesc* v = find(name);
    if (!v) return TPLC_SYM_E_NONAME;
    *out = *v;
    return TPLC_SYM_OK;
}

int32_t SymService::read(void*, const char* name, uint8_t* out,
                         uint32_t cap, uint32_t* out_len) {
    if (!name || !out || cap == 0) return TPLC_SYM_E_PARAM;
    const TplcVarDesc* v = find(name);
    if (!v) return TPLC_SYM_E_NONAME;
    if (!blob_) return TPLC_SYM_E_NOBLOB;
    uint32_t n = v->size;
    if (n > cap) n = cap;
    std::memcpy(out, (const uint8_t*)blob_ + v->offset, n);
    if (out_len) *out_len = n;
    return TPLC_SYM_OK;
}

int32_t SymService::write(void*, const char* name, const uint8_t* in,
                          uint32_t len) {
    if (!name || (!in && len)) return TPLC_SYM_E_PARAM;
    const TplcVarDesc* v = find(name);
    if (!v) return TPLC_SYM_E_NONAME;
    if (!blob_) return TPLC_SYM_E_NOBLOB;
    if (len > v->size) return TPLC_SYM_E_SIZE;
    std::memcpy((uint8_t*)blob_ + v->offset, in, len);
    return TPLC_SYM_OK;
}

} /* namespace tplc */
