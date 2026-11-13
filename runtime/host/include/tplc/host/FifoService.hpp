/* FifoService.hpp — in-process FiFoAxes channel table.
 *
 * Mirrors the fifo.* semantics of tether_io_motion_server: each
 * channel owns a queue of LREAL rows (8 columns, group position =
 * column), consumes one row per fifo_poll call, and writes the row
 * values as 0x607A position targets through the in-process
 * MotionService. Used when no Tether IO endpoint is configured;
 * the IO-backed variant is FifoIoService.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_FIFO_SERVICE_HPP
#define TPLC_HOST_FIFO_SERVICE_HPP

#include <tplc/tplc_fifo.h>
#include <tplc/tplc_motion.h>

#include <array>
#include <cstdint>
#include <deque>
#include <memory>

namespace tplc {

class FifoService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit FifoService(ClockFn clock = nullptr, void* clock_ctx = nullptr);
    ~FifoService();
    FifoService(const FifoService&) = delete;
    FifoService& operator=(const FifoService&) = delete;

    bool active() const { return active_; }
    const TplcFifoService& table() const { return svc_; }

    /* Wire the axis backend row targets are written through. */
    void setMotion(const TplcMotionService* motion) { motion_ = motion; }

private:
    struct Channel {
        int32_t state = TPLC_FIFO_STATE_IDLE;
        int32_t axes[TPLC_FIFO_AXIS_MAX];
        uint32_t naxes = 0;
        std::deque<std::array<double, TPLC_FIFO_ROW_COLS>> rows;
        uint32_t seq = 0;
        double override_pct = 100.0;
        Channel() { reset(); }
        void reset();
    };

    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcFifoService svc_{};
    Channel chans_[TPLC_FIFO_CHAN_MAX];
    const TplcMotionService* motion_ = nullptr;
    ClockFn clock_ = nullptr;
    void* clock_ctx_ = nullptr;
    bool active_ = false;

    int32_t integrate(void*, uint32_t, uint32_t, uint32_t);
    int32_t disintegrate(void*, uint32_t);
    int32_t write(void*, uint32_t, const void*, uint32_t, int32_t);
    int32_t command(void*, uint32_t, uint32_t);
    int32_t status(void*, uint32_t, TplcFifoStatus*);
    int32_t setOverride(void*, uint32_t, double);
    int32_t poll(void*, uint32_t);

    static FifoService* self(void* p) {
        return static_cast<FifoService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_FIFO_SERVICE_HPP */
