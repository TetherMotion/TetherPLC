/* KinIoService.hpp — tplc-kin bridge over the Tether IO protocol.
 *
 * Maps TplcKinService ops to kin.* parameters on
 * tether_io_motion_server: object select, type/preset/cfg writes,
 * axis-list blobs, transform input, command, result reads, lock,
 * rotation, and status. The server owns the ForwardKinematicsBase
 * models and runs transforms synchronously inside param writes.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_KIN_IO_SERVICE_HPP
#define TPLC_HOST_KIN_IO_SERVICE_HPP

#include <tplc/tplc_kin.h>

#include <cstdint>
#include <memory>

namespace tplc {

class KinIoService {
public:
    using ClockFn = uint64_t (*)(void*);

    KinIoService(ClockFn clock = nullptr, void* clock_ctx = nullptr);
    ~KinIoService();
    KinIoService(const KinIoService&) = delete;
    KinIoService& operator=(const KinIoService&) = delete;

    bool active() const { return active_; }
    const TplcKinService& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcKinService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    ClockFn clock_ = nullptr;
    void* clock_ctx_ = nullptr;
    uint64_t last_time_us_ = ~0ull;
    bool active_ = false;

    void syncTime();
    /* select the kin.obj object before each op; tracks last obj. */
    bool selectObj(uint32_t obj);
    uint32_t lastObj_ = ~0u;

    int32_t configure(void*, uint32_t, int32_t, uint32_t,
                      const double*, uint32_t);
    int32_t setAxes(void*, uint32_t, const uint32_t*, uint32_t,
                    const uint32_t*, uint32_t);
    int32_t commandOp(void*, uint32_t, uint32_t);
    int32_t calcInput(void*, uint32_t, const double*, uint32_t);
    int32_t result(void*, uint32_t, double*, double*);
    int32_t setLocked(void*, uint32_t, uint32_t);
    int32_t rotation(void*, uint32_t, uint32_t, const double*,
                     double*);
    int32_t status(void*, uint32_t, TplcKinStatus*);
    int32_t setChannel(void*, uint32_t, uint32_t);
    int32_t getAxes(void*, uint32_t, uint32_t*, uint32_t*,
                    uint32_t*, uint32_t*);

    static KinIoService* self(void* p) {
        return static_cast<KinIoService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_KIN_IO_SERVICE_HPP */
