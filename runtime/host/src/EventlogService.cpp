/* EventlogService.cpp — tplc-eventlog bounded event journal.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/EventlogService.hpp>

#include <cstring>

namespace tplc {

struct EventlogService::Impl {
    TplcEventlogInfo ev[TPLC_EVENTLOG_MAX_EVENTS]{};
    bool     used[TPLC_EVENTLOG_MAX_EVENTS]{};
    uint32_t raised_total = 0;
    uint32_t cleared_total = 0;
    uint32_t confirmed_total = 0;
    uint32_t dropped_total = 0;
};

EventlogService::EventlogService() : impl_(new Impl) {
    svc_.struct_size = sizeof(TplcEventlogService);
    svc_.abi_version = TPLC_EVENTLOG_ABI_VERSION;
    svc_.host = this;
    svc_.ev_raise = [](void* s, uint32_t c, uint32_t i, uint32_t v,
                       const uint8_t* t, uint32_t n, uint32_t a)
        { return self(s)->raise(s, c, i, v, t, n, a); };
    svc_.ev_clear = [](void* s, uint32_t id)
        { return self(s)->clear(s, id); };
    svc_.ev_confirm = [](void* s, uint32_t id)
        { return self(s)->confirm(s, id); };
    svc_.ev_query = [](void* s, uint32_t id, TplcEventlogInfo* o)
        { return self(s)->query(s, id, o); };
    svc_.ev_stats = [](void* s, TplcEventlogStats* o)
        { return self(s)->stats(s, o); };
}
EventlogService::~EventlogService() = default;

int32_t EventlogService::raise(void*, uint32_t cls, uint32_t eid,
                               uint32_t sev, const uint8_t* text,
                               uint32_t len, uint32_t args) {
    if (sev > TPLC_EVENTLOG_SEV_CRITICAL || len > TPLC_EVENTLOG_TEXT_CAP)
        return TPLC_EVENTLOG_E_PARAM;
    int slot = -1;
    for (uint32_t i = 0; i < TPLC_EVENTLOG_MAX_EVENTS; i++)
        if (!impl_->used[i]) { slot = (int)i; break; }
    if (slot < 0) {
        impl_->dropped_total++;
        return TPLC_EVENTLOG_E_FULL;
    }
    TplcEventlogInfo& e = impl_->ev[slot];
    std::memset(&e, 0, sizeof(e));
    e.event_class = cls;
    e.event_id = eid;
    e.severity = sev;
    e.state = TPLC_EVENTLOG_ST_RAISED;
    e.args = args;
    e.text_len = (uint8_t)len;
    if (text && len) std::memcpy(e.text, text, len);
    impl_->used[slot] = true;
    impl_->raised_total++;
    return slot + 1;
}

int32_t EventlogService::clear(void*, uint32_t id) {
    if (id == 0 || id > TPLC_EVENTLOG_MAX_EVENTS ||
        !impl_->used[id - 1])
        return TPLC_EVENTLOG_E_HANDLE;
    TplcEventlogInfo& e = impl_->ev[id - 1];
    if (e.state != TPLC_EVENTLOG_ST_RAISED)
        return TPLC_EVENTLOG_E_STATE;
    e.state = TPLC_EVENTLOG_ST_CLEARED;
    impl_->cleared_total++;
    return TPLC_EVENTLOG_OK;
}

int32_t EventlogService::confirm(void*, uint32_t id) {
    if (id == 0 || id > TPLC_EVENTLOG_MAX_EVENTS ||
        !impl_->used[id - 1])
        return TPLC_EVENTLOG_E_HANDLE;
    TplcEventlogInfo& e = impl_->ev[id - 1];
    if (e.state == TPLC_EVENTLOG_ST_CONFIRMED)
        return TPLC_EVENTLOG_E_STATE;
    e.state = TPLC_EVENTLOG_ST_CONFIRMED;
    impl_->confirmed_total++;
    impl_->used[id - 1] = false;   /* release the slot */
    return TPLC_EVENTLOG_OK;
}

int32_t EventlogService::query(void*, uint32_t id,
                               TplcEventlogInfo* out) {
    if (!out) return TPLC_EVENTLOG_E_PARAM;
    if (id == 0 || id > TPLC_EVENTLOG_MAX_EVENTS ||
        !impl_->used[id - 1])
        return TPLC_EVENTLOG_E_HANDLE;
    *out = impl_->ev[id - 1];
    return TPLC_EVENTLOG_OK;
}

int32_t EventlogService::stats(void*, TplcEventlogStats* out) {
    if (!out) return TPLC_EVENTLOG_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    for (uint32_t i = 0; i < TPLC_EVENTLOG_MAX_EVENTS; i++)
        if (impl_->used[i]) out->live_events++;
    out->raised_total = impl_->raised_total;
    out->cleared_total = impl_->cleared_total;
    out->confirmed_total = impl_->confirmed_total;
    out->dropped_total = impl_->dropped_total;
    return TPLC_EVENTLOG_OK;
}

} /* namespace tplc */
