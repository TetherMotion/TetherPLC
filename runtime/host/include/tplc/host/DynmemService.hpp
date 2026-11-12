/* DynmemService.hpp — tplc-dynmem bounded dynamic-memory arena.
 * Deterministic allocator over a fixed compile-time arena:
 * block handles are small integers, no host-heap interaction,
 * worst-case memory cost is fixed.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_DYNMEM_SERVICE_HPP
#define TPLC_DYNMEM_SERVICE_HPP

#include <tplc/tplc_dynmem.h>
#include <memory>

namespace tplc {

class DynmemService {
public:
    DynmemService();
    ~DynmemService();
    const TplcDynmemService& table() const { return svc_; }

private:
    static DynmemService* self(void* s) {
        return static_cast<DynmemService*>(
            reinterpret_cast<TplcDynmemService*>(s)->host);
    }

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcDynmemService svc_{};

    int32_t alloc(void*, uint32_t size, uint32_t reset);
    int32_t free(void*, uint32_t id);
    int32_t resize(void*, uint32_t id, uint32_t new_size,
                   uint32_t preserve, uint32_t reset);
    int32_t read(void*, uint32_t id, uint32_t offset);
    int32_t write(void*, uint32_t id, uint32_t offset,
                  uint32_t byte);
    int32_t stats(void*, TplcDynmemStats* out);
};

} /* namespace tplc */
#endif /* TPLC_DYNMEM_SERVICE_HPP */
