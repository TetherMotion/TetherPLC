/* RtmonService.hpp — tplc-rtmon host implementation.
 * Fixed-capacity ring of timestamped marks (steady-clock ns),
 * appended from the scan path and readable by index. Oldest record
 * is overwritten when full. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_RTMONSERVICE_HPP
#define TPLC_HOST_RTMONSERVICE_HPP

#include <tplc/tplc_rtmon.h>

#include <cstdint>
#include <vector>

namespace tplc {

class RtmonService {
public:
    static constexpr uint32_t kCapacity = 4096;
    /* steady-clock ns source; HostServices injects its steadyNs so
       sim time drives mark timestamps deterministically */
    using ClockFn = uint64_t (*)(void* ctx);

    RtmonService(ClockFn clock, void* clock_ctx);

    const TplcRtmonService& table() const { return svc_; }

private:
    int32_t rtmonMark(void* s, uint32_t context, uint32_t group,
                      uint32_t event, uint32_t markctrl);
    int32_t rtmonCount(void* s, uint32_t* count);
    int32_t rtmonGet(void* s, uint32_t index, TplcRtmonMark* out);
    int32_t rtmonClear(void* s);

    uint64_t steadyNs() const;

    TplcRtmonService        svc_{};
    std::vector<TplcRtmonMark> ring_;
    ClockFn                 clock_ = nullptr;
    void*                   clock_ctx_ = nullptr;
    uint32_t                head_ = 0;   /* next write slot            */
    uint32_t                size_ = 0;   /* retained records          */
    uint64_t                total_ = 0;  /* appended since clear      */
};

} /* namespace tplc */

#endif /* TPLC_HOST_RTMONSERVICE_HPP */
