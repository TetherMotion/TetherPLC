/* EventlogService.hpp — tplc-eventlog bounded event journal.
 * Fixed ring of event records with TwinCAT-style lifecycle
 * (raised -> cleared -> confirmed); deterministic memory,
 * no host heap after startup.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_EVENTLOG_SERVICE_HPP
#define TPLC_EVENTLOG_SERVICE_HPP

#include <tplc/tplc_eventlog.h>
#include <memory>

namespace tplc {

class EventlogService {
public:
    EventlogService();
    ~EventlogService();
    const TplcEventlogService& table() const { return svc_; }

private:
    static EventlogService* self(void* s) {
        return static_cast<EventlogService*>(
            reinterpret_cast<TplcEventlogService*>(s)->host);
    }

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcEventlogService svc_{};

    int32_t raise(void*, uint32_t cls, uint32_t id, uint32_t sev,
                  const uint8_t* text, uint32_t len, uint32_t args);
    int32_t clear(void*, uint32_t id);
    int32_t confirm(void*, uint32_t id);
    int32_t query(void*, uint32_t id, TplcEventlogInfo* out);
    int32_t stats(void*, TplcEventlogStats* out);
};

} /* namespace tplc */
#endif /* TPLC_EVENTLOG_SERVICE_HPP */
