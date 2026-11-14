/* EventlogService.hpp — tplc-eventlog bounded event journal.
 * Fixed ring of event records with TwinCAT-style lifecycle
 * (raised -> cleared -> confirmed); deterministic memory,
 * no host heap after startup.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_EVENTLOG_SERVICE_HPP
#define TPLC_EVENTLOG_SERVICE_HPP

#include <tplc/tplc_eventlog.h>
#include <functional>
#include <memory>

namespace tplc {

class EventlogService {
public:
    EventlogService();
    ~EventlogService();
    const TplcEventlogService& table() const { return svc_; }
    /* mutable view for ctl-side confirm/clear; the vtable fns are
       stateless dispatchers so casting away const is safe        */
    TplcEventlogService& tableMut() { return svc_; }

    /* transition hook: fired after every successful raise/clear/
       confirm with (journal_id, new_state, info). Used by the
       runtime to broadcast ctl events + audit records.          */
    void setNotify(std::function<void(uint32_t id, uint32_t state,
                                      const TplcEventlogInfo*)> fn) {
        notify_ = std::move(fn);
    }

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
    int32_t list(void*, TplcEventlogInfo* out, uint32_t cap,
                 uint32_t* count);
    int32_t history(void*, TplcEventlogRecord* out, uint32_t cap,
                    uint32_t* count);
    int32_t importRecs(const TplcEventlogRecord* recs, uint32_t n);
    void note(uint32_t id, uint32_t state);

    std::function<void(uint32_t, uint32_t, const TplcEventlogInfo*)>
        notify_;
};

} /* namespace tplc */
#endif /* TPLC_EVENTLOG_SERVICE_HPP */
