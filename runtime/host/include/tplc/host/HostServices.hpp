/* HostServices.hpp — TplcServices implementation (plan doc 02 §2.8).
 * Every entry nonblocking/bounded; log + counters go to host-owned
 * sinks the service plane drains. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_SERVICES_HPP
#define TPLC_HOST_SERVICES_HPP

#include <tplc/tplc_abi.h>

#include <cstdint>
#include <functional>
#include <unordered_map>
#include <utility>
#include <vector>

namespace tplc {

class HostServices {
public:
    using LogFn = std::function<void(uint8_t level, uint32_t code,
                                     const void* args, uint32_t len)>;

    explicit HostServices(uint32_t caps);

    TplcServices& table() { return svc_; }
    uint32_t      caps()  const { return svc_.caps; }

    void onLog(LogFn fn) { log_fn_ = std::move(fn); }

    /* observability for the service plane / tests */
    const std::unordered_map<uint32_t, int64_t>& diagCounters() const {
        return diag_;
    }
    const std::vector<std::pair<uint64_t, uint64_t>>&
    retainDirtyRanges() const { return retain_dirty_; }
    void clearRetainDirty() { retain_dirty_.clear(); }
    uint64_t logCount() const { return log_count_; }

    /* C-callback trampolines */
    void     log(uint8_t level, uint32_t code, const void* args,
                 uint32_t len);
    void     diagCounter(uint32_t id, int64_t delta);
    uint64_t steadyNs() const;
    /* sim clock (doc 14 §2): when enabled, steadyNs returns the
       master-published sim time — drives ctx->now_ns and deadline
       checks deterministically in --sim mode                       */
    void     setSimNow(uint64_t ns) { sim_now_ = ns; sim_on_ = true; }
    bool     simClock() const { return sim_on_; }
    void     retainDirty(uint64_t off, uint64_t len);
    int32_t  raiseEvent(uint32_t mask);
    void*    getService(const char* name, uint32_t len);

private:
    TplcServices svc_{};
    LogFn        log_fn_;
    std::unordered_map<uint32_t, int64_t> diag_;
    std::vector<std::pair<uint64_t, uint64_t>> retain_dirty_;
    uint64_t     log_count_ = 0;
    uint64_t     sim_now_ = 0;
    bool         sim_on_ = false;
};

} /* namespace tplc */
#endif /* TPLC_HOST_SERVICES_HPP */
