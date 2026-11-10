/* RtmonService.cpp — real-time monitor mark ring.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/RtmonService.hpp>

#include <ctime>

namespace tplc {

namespace {
RtmonService* self(void* p) { return static_cast<RtmonService*>(p); }
} /* namespace */

RtmonService::RtmonService(ClockFn clock, void* clock_ctx)
    : ring_(kCapacity), clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size   = sizeof(TplcRtmonService);
    svc_.abi_version   = TPLC_RTMON_ABI_VERSION;
    svc_.host          = this;
    svc_.rtmon_mark    = [](void* s, uint32_t c, uint32_t g, uint32_t e,
                            uint32_t m) {
        return self(s)->rtmonMark(s, c, g, e, m);
    };
    svc_.rtmon_count   = [](void* s, uint32_t* c) {
        return self(s)->rtmonCount(s, c);
    };
    svc_.rtmon_get     = [](void* s, uint32_t i, TplcRtmonMark* o) {
        return self(s)->rtmonGet(s, i, o);
    };
    svc_.rtmon_clear   = [](void* s) { return self(s)->rtmonClear(s); };
}

uint64_t RtmonService::steadyNs() const {
    if (clock_) return clock_(clock_ctx_);
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

int32_t RtmonService::rtmonMark(void*, uint32_t context, uint32_t group,
                                uint32_t event, uint32_t markctrl) {
    TplcRtmonMark& m = ring_[head_];
    m.ts_ns     = steadyNs();
    m.context   = context;
    m.group     = group;
    m.event     = event;
    m.markctrl  = markctrl;
    head_ = (head_ + 1) % kCapacity;
    if (size_ < kCapacity) size_++;
    total_++;
    return TPLC_RTMON_OK;
}

int32_t RtmonService::rtmonCount(void*, uint32_t* count) {
    if (!count) return TPLC_RTMON_E_PARAM;
    /* retained records; index domain of rtmon_get */
    *count = size_;
    return TPLC_RTMON_OK;
}

int32_t RtmonService::rtmonGet(void*, uint32_t index,
                               TplcRtmonMark* out) {
    if (!out) return TPLC_RTMON_E_PARAM;
    /* index 0 = oldest retained record */
    if (index >= size_) return TPLC_RTMON_E_RANGE;
    uint32_t slot;
    if (size_ < kCapacity) {
        slot = index;
    } else {
        /* full: oldest is at head_ (next overwrite) */
        slot = (head_ + index) % kCapacity;
    }
    *out = ring_[slot];
    return TPLC_RTMON_OK;
}

int32_t RtmonService::rtmonClear(void*) {
    head_  = 0;
    size_  = 0;
    total_ = 0;
    return TPLC_RTMON_OK;
}

} /* namespace tplc */
