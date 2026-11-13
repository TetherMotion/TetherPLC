/* FifoService.cpp — in-process FiFoAxes channel table.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/FifoService.hpp>

#include <cmath>
#include <cstring>

namespace tplc {

void FifoService::Channel::reset() {
    for (auto& a : axes) a = -1;
    naxes = 0;
    rows.clear();
    seq = 0;
    state = TPLC_FIFO_STATE_IDLE;
}

FifoService::FifoService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcFifoService);
    svc_.abi_version = TPLC_FIFO_ABI_VERSION;
    svc_.host = this;
    svc_.fifo_integrate = [](void* s, uint32_t c, uint32_t a, uint32_t g) {
        return self(s)->integrate(s, c, a, g);
    };
    svc_.fifo_disintegrate = [](void* s, uint32_t c) {
        return self(s)->disintegrate(s, c);
    };
    svc_.fifo_write = [](void* s, uint32_t c, const void* b, uint32_t l,
                         int32_t r) {
        return self(s)->write(s, c, b, l, r);
    };
    svc_.fifo_command = [](void* s, uint32_t c, uint32_t cmd) {
        return self(s)->command(s, c, cmd);
    };
    svc_.fifo_status = [](void* s, uint32_t c, TplcFifoStatus* o) {
        return self(s)->status(s, c, o);
    };
    svc_.fifo_set_override = [](void* s, uint32_t c, double p) {
        return self(s)->setOverride(s, c, p);
    };
    svc_.fifo_poll = [](void* s, uint32_t c) {
        return self(s)->poll(s, c);
    };
    active_ = true;
}

FifoService::~FifoService() = default;

int32_t FifoService::integrate(void*, uint32_t chan, uint32_t axis,
                               uint32_t gpos) {
    if (chan >= TPLC_FIFO_CHAN_MAX || gpos >= TPLC_FIFO_AXIS_MAX ||
        axis == 0)
        return TPLC_FIFO_E_PARAM;
    Channel& c = chans_[chan];
    c.axes[gpos] = int32_t(axis);
    uint32_t n = 0;
    for (auto a : c.axes)
        if (a >= 0) ++n;
    c.naxes = n;
    return TPLC_FIFO_OK;
}

int32_t FifoService::disintegrate(void*, uint32_t chan) {
    if (chan >= TPLC_FIFO_CHAN_MAX) return TPLC_FIFO_E_PARAM;
    chans_[chan].reset();
    return TPLC_FIFO_OK;
}

int32_t FifoService::write(void*, uint32_t chan, const void* blob,
                           uint32_t len, int32_t replace) {
    if (chan >= TPLC_FIFO_CHAN_MAX || !blob) return TPLC_FIFO_E_PARAM;
    Channel& c = chans_[chan];
    if (c.naxes == 0 || len < 4) return TPLC_FIFO_E_STATE;
    uint32_t nrows;
    std::memcpy(&nrows, blob, 4);
    const uint8_t* p = static_cast<const uint8_t*>(blob) + 4;
    if (uint64_t(len) < 4ull + uint64_t(nrows) * TPLC_FIFO_ROW_COLS * 8 ||
        nrows > TPLC_FIFO_MAX_ROWS)
        return TPLC_FIFO_E_PARAM;
    if (replace) c.rows.clear();
    if (c.rows.size() + nrows > TPLC_FIFO_MAX_ROWS)
        return TPLC_FIFO_E_FULL;
    for (uint32_t r = 0; r < nrows; ++r) {
        std::array<double, TPLC_FIFO_ROW_COLS> row{};
        for (uint32_t col = 0; col < TPLC_FIFO_ROW_COLS; ++col) {
            double v;
            std::memcpy(&v, p, 8);
            p += 8;
            row[col] = v;
        }
        c.rows.push_back(row);
    }
    if (c.state == TPLC_FIFO_STATE_FINISHED)
        c.state = TPLC_FIFO_STATE_IDLE;
    return TPLC_FIFO_OK;
}

int32_t FifoService::command(void*, uint32_t chan, uint32_t cmd) {
    if (chan >= TPLC_FIFO_CHAN_MAX) return TPLC_FIFO_E_PARAM;
    Channel& c = chans_[chan];
    switch (cmd) {
    case TPLC_FIFO_CMD_START:
        if (c.state == TPLC_FIFO_STATE_RUNNING || c.naxes == 0) {
            c.state = TPLC_FIFO_STATE_ERROR;
            return TPLC_FIFO_E_STATE;
        }
        if (c.rows.empty()) {
            c.state = TPLC_FIFO_STATE_FINISHED;
            return TPLC_FIFO_OK;
        }
        /* CSP mode + enable operation on member axes, same as the
           IO backend. */
        if (motion_) {
            for (auto a : c.axes) {
                if (a < 0) continue;
                motion_->motion_obj_write(motion_->host, uint32_t(a),
                                          TPLC_MOTION_OBJ_MODES_OP, 8);
                motion_->motion_obj_write(motion_->host, uint32_t(a),
                                          TPLC_MOTION_OBJ_CONTROLWORD,
                                          0x000F);
            }
        }
        c.state = TPLC_FIFO_STATE_RUNNING;
        c.seq = 0;
        return TPLC_FIFO_OK;
    case TPLC_FIFO_CMD_STOP:
        c.state = TPLC_FIFO_STATE_IDLE;
        return TPLC_FIFO_OK;
    case TPLC_FIFO_CMD_RESET:
        c.rows.clear();
        c.seq = 0;
        c.state = TPLC_FIFO_STATE_IDLE;
        return TPLC_FIFO_OK;
    default:
        c.state = TPLC_FIFO_STATE_ERROR;
        return TPLC_FIFO_E_PARAM;
    }
}

int32_t FifoService::status(void*, uint32_t chan,
                            TplcFifoStatus* out) {
    if (chan >= TPLC_FIFO_CHAN_MAX || !out) return TPLC_FIFO_E_PARAM;
    const Channel& c = chans_[chan];
    out->state = c.state;
    out->naxes = int32_t(c.naxes);
    out->maxrows = TPLC_FIFO_MAX_ROWS;
    out->queued = uint32_t(c.rows.size());
    out->seq = c.seq;
    out->override = c.override_pct;
    return TPLC_FIFO_OK;
}

int32_t FifoService::setOverride(void*, uint32_t chan, double pct) {
    if (chan >= TPLC_FIFO_CHAN_MAX) return TPLC_FIFO_E_PARAM;
    chans_[chan].override_pct = pct < 0.0 ? 0.0 : (pct > 500.0 ? 500.0 : pct);
    return TPLC_FIFO_OK;
}

int32_t FifoService::poll(void*, uint32_t chan) {
    if (chan >= TPLC_FIFO_CHAN_MAX) return TPLC_FIFO_E_PARAM;
    Channel& c = chans_[chan];
    if (c.state != TPLC_FIFO_STATE_RUNNING || c.rows.empty())
        return TPLC_FIFO_OK;
    const auto row = c.rows.front();
    c.rows.pop_front();
    ++c.seq;
    for (uint32_t col = 0; col < TPLC_FIFO_AXIS_MAX; ++col) {
        const int32_t a = c.axes[col];
        if (a < 0 || !motion_) continue;
        const int32_t counts = int32_t(std::lround(row[col]));
        motion_->motion_obj_write(motion_->host, uint32_t(a),
                                  TPLC_MOTION_OBJ_POS_TARGET, counts);
    }
    if (c.rows.empty()) c.state = TPLC_FIFO_STATE_FINISHED;
    return TPLC_FIFO_OK;
}

} // namespace tplc
