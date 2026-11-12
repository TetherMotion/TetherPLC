/* NciIoService.hpp — tplc-nci bridge over the Tether IO protocol.
 *
 * Maps TplcNciService ops to nc.* parameters on tether_io_motion_server:
 * program text/path uploads become String param writes, commands a U8
 * param write, and status reads the nc.* state params. The channel is
 * only available when TPLC_MOTION_IO names a reachable endpoint that
 * also serves nc.* (an NCI-enabled server); otherwise active() is false
 * and getService("tplc-nci") returns nullptr — interpretation lives in
 * Tether, there is no in-process fallback.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_NCI_IO_SERVICE_HPP
#define TPLC_HOST_NCI_IO_SERVICE_HPP

#include <tplc/tplc_nci.h>

#include <cstdint>
#include <memory>

namespace tplc {

class NciIoService {
public:
    using ClockFn = uint64_t (*)(void*);

    NciIoService(ClockFn clock = nullptr, void* clock_ctx = nullptr);
    ~NciIoService();
    NciIoService(const NciIoService&) = delete;
    NciIoService& operator=(const NciIoService&) = delete;

    bool active() const { return active_; }
    const TplcNciService& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcNciService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    ClockFn clock_ = nullptr;
    void* clock_ctx_ = nullptr;
    uint64_t last_time_us_ = ~0ull;
    bool active_ = false;

    void syncTime();

    int32_t loadText(void*, uint32_t, const char*, uint32_t);
    int32_t loadFile(void*, uint32_t, const char*, uint32_t);
    int32_t command(void*, uint32_t, uint32_t);
    int32_t status(void*, uint32_t, TplcNciStatus*);

    static NciIoService* self(void* p) {
        return static_cast<NciIoService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_NCI_IO_SERVICE_HPP */
