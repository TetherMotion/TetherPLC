/* FifoIoService.hpp — tplc-fifo bridge over the Tether IO protocol.
 *
 * Maps TplcFifoService ops to fifo.* parameters on
 * tether_io_motion_server: channel select, integrate/disintegrate,
 * row-blob writes, commands, status reads, and override. Row data
 * travels as the u32 nrows + nrows*8 f64 LE blob the server expects.
 * The server consumes rows on the mc.time drive clock; fifo_poll is
 * a no-op here (syncTime advances the clock like NciIoService).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_FIFO_IO_SERVICE_HPP
#define TPLC_HOST_FIFO_IO_SERVICE_HPP

#include <tplc/tplc_fifo.h>

#include <cstdint>
#include <memory>

namespace tplc {

class FifoIoService {
public:
    using ClockFn = uint64_t (*)(void*);

    FifoIoService(ClockFn clock = nullptr, void* clock_ctx = nullptr);
    ~FifoIoService();
    FifoIoService(const FifoIoService&) = delete;
    FifoIoService& operator=(const FifoIoService&) = delete;

    bool active() const { return active_; }
    const TplcFifoService& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcFifoService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    ClockFn clock_ = nullptr;
    void* clock_ctx_ = nullptr;
    uint64_t last_time_us_ = ~0ull;
    bool active_ = false;

    void syncTime();
    /* select the fifo.chan channel before each op. */
    bool selectChan(uint32_t chan);

    int32_t integrate(void*, uint32_t, uint32_t, uint32_t);
    int32_t disintegrate(void*, uint32_t);
    int32_t write(void*, uint32_t, const void*, uint32_t, int32_t);
    int32_t command(void*, uint32_t, uint32_t);
    int32_t status(void*, uint32_t, TplcFifoStatus*);
    int32_t setOverride(void*, uint32_t, double);
    int32_t poll(void*, uint32_t);

    static FifoIoService* self(void* p) {
        return static_cast<FifoIoService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_FIFO_IO_SERVICE_HPP */
