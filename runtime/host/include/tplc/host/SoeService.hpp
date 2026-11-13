/* SoeService.hpp — tplc-soe backend over the Tether IO protocol.
 * Reads/writes the ec.soe.* params (slave selector, 128-byte state
 * blob, 16-byte command mailbox) that tether_io_motion_server (sim)
 * or a real master bridge exposes.  Endpoint: TPLC_EC_IO, else
 * TPLC_MOTION_IO.  The sim clock is synced via mc.time before each
 * op so command-state timing stays deterministic under --clock sim.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_SOE_SERVICE_HPP
#define TPLC_HOST_SOE_SERVICE_HPP

#include <tplc/tplc_soe.h>

#include <cstdint>
#include <memory>

namespace tplc {

class SoeService {
public:
    using ClockFn = uint64_t (*)(void*);

    SoeService(ClockFn clock = nullptr, void* clock_ctx = nullptr);
    ~SoeService();
    SoeService(const SoeService&) = delete;
    SoeService& operator=(const SoeService&) = delete;

    bool active() const { return active_; }
    const TplcSoeService& table() const { return svc_; }

private:
    int32_t read(void*, uint32_t slave, TplcSoeState* st);
    int32_t writeOp(void*, uint32_t slave, uint16_t op, uint32_t v);
    int32_t writePassword(void*, uint32_t slave, uint32_t pw);
    int32_t writeCmdCtrl(void*, uint32_t slave, uint32_t ctrl);
    int32_t execCommand(void*, uint32_t slave, uint32_t id);
    int32_t reset(void*, uint32_t slave);
    int32_t writeAxCtrl(void*, uint32_t slave, uint32_t ctrl);
    int32_t writeAxPark(void*, uint32_t slave, uint32_t park);
    void syncTime();

    TplcSoeService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    ClockFn clock_ = nullptr;
    void* clock_ctx_ = nullptr;
    uint64_t last_time_us_ = 0;
    bool active_ = false;
};

} // namespace tplc
#endif
