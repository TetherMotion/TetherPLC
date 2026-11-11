/* MotionIoService.hpp — tplc-motion backend over the Tether IO protocol.
 *
 * Implements the same TplcMotionService vtable as MotionService but maps
 * each axis to a simulated CiA402 drive served by a Tether IO server
 * (tether_io_motion_server). DS402 object reads/writes become IO
 * parameter get/set calls on mc.<slave>.<index>; the PLC-facing ABI and
 * the MC wrappers are unchanged. When no Tether IO endpoint is
 * configured (TPLC_MOTION_IO) the service stays inactive and HostServices
 * falls back to the in-process MotionService.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_MOTION_IO_SERVICE_HPP
#define TPLC_HOST_MOTION_IO_SERVICE_HPP

#include <tplc/tplc_motion.h>

#include <cstdint>
#include <memory>

namespace tplc {

class MotionIoService {
public:
    using ClockFn = uint64_t (*)(void*);

    /* Reads TPLC_MOTION_IO ("tcp://host:port" or "host:port") and connects
       when the Tether IO protocol library is available. active() reports
       whether the vtable is usable. When clock is given, its value (ns)
       is written to the server's mc.time param before each object op so
       the simulated drives advance on the PLC --sim clock deterministically. */
    MotionIoService(ClockFn clock = nullptr, void* clock_ctx = nullptr);
    ~MotionIoService();
    MotionIoService(const MotionIoService&) = delete;
    MotionIoService& operator=(const MotionIoService&) = delete;

    bool active() const { return active_; }
    const TplcMotionService& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member: the rt bridge passes &svc_
       (not svc.host) as the service pointer, and self() recovers the
       enclosing object only because &svc_ == this at offset 0. */
    TplcMotionService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    ClockFn clock_ = nullptr;
    void* clock_ctx_ = nullptr;
    uint64_t last_time_us_ = ~0ull;
    bool active_ = false;

    /* Push the current sim time (us) to mc.time when it changed. */
    void syncTime();

    int32_t axisCreate(void*, const TPLC_MotionAxisCfg*, uint32_t*);
    int32_t axisDestroy(void*, uint32_t);
    int32_t axisPoll(void*);
    int32_t axisInfo(void*, uint32_t, TPLC_MotionAxisInfo*);
    int32_t objRead(void*, uint32_t, uint32_t, int32_t*);
    int32_t objWrite(void*, uint32_t, uint32_t, int32_t);

    static MotionIoService* self(void* p) {
        return static_cast<MotionIoService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_MOTION_IO_SERVICE_HPP */
