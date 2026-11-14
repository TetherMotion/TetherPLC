/* EventlogService.cpp — tplc-eventlog bounded event journal.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/EventlogService.hpp>

#include <cstring>
#include <ctime>

namespace tplc {

static uint64_t rt_now_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

struct EventlogService::Impl {
    TplcEventlogInfo ev[TPLC_EVENTLOG_MAX_EVENTS]{};
    bool     used[TPLC_EVENTLOG_MAX_EVENTS]{};
    /* append-only transition history: survives slot release so
       confirmed alarms remain auditable (shift reports etc.)    */
    TplcEventlogRecord hist[TPLC_EVENTLOG_HIST_CAP]{};
    uint32_t hist_wr = 0;          /* next write slot            */
    uint32_t hist_len = 0;         /* valid records (≤ CAP)      */
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
    svc_.ev_list = [](void* s, TplcEventlogInfo* o, uint32_t cap,
                      uint32_t* n)
        { return self(s)->list(s, o, cap, n); };
    svc_.ev_history = [](void* s, TplcEventlogRecord* o, uint32_t cap,
                         uint32_t* n)
        { return self(s)->history(s, o, cap, n); };
    svc_.ev_import = [](void* s, const TplcEventlogRecord* r,
                        uint32_t n) {
        return self(s)->importRecs(r, n);
    };
}
EventlogService::~EventlogService() = default;

void EventlogService::note(uint32_t id, uint32_t state) {
    if (id == 0 || id > TPLC_EVENTLOG_MAX_EVENTS ||
        !impl_->used[id - 1])
        return;
    TplcEventlogInfo& e = impl_->ev[id - 1];
    TplcEventlogRecord& r =
        impl_->hist[impl_->hist_wr % TPLC_EVENTLOG_HIST_CAP];
    impl_->hist_wr++;
    if (impl_->hist_len < TPLC_EVENTLOG_HIST_CAP) impl_->hist_len++;
    std::memset(&r, 0, sizeof(r));
    r.ts_ns = state == TPLC_EVENTLOG_ST_RAISED ? e.raised_ns
            : state == TPLC_EVENTLOG_ST_CLEARED ? e.cleared_ns
            : e.confirmed_ns;
    r.journal_id = id;
    r.event_class = e.event_class;
    r.event_id = e.event_id;
    r.severity = e.severity;
    r.state = state;
    r.text_len = e.text_len;
    if (e.text_len) std::memcpy(r.text, e.text, e.text_len);
    if (notify_) notify_(id, state, &e);
}

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
    e.raised_ns = rt_now_ns();
    impl_->used[slot] = true;
    impl_->raised_total++;
    note((uint32_t)slot + 1, TPLC_EVENTLOG_ST_RAISED);
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
    e.cleared_ns = rt_now_ns();
    impl_->cleared_total++;
    note(id, TPLC_EVENTLOG_ST_CLEARED);
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
    e.confirmed_ns = rt_now_ns();
    impl_->confirmed_total++;
    note(id, TPLC_EVENTLOG_ST_CONFIRMED);
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

int32_t EventlogService::list(void*, TplcEventlogInfo* out, uint32_t cap,
                              uint32_t* count) {
    if (!count) return TPLC_EVENTLOG_E_PARAM;
    uint32_t n = 0;
    for (uint32_t i = 0; i < TPLC_EVENTLOG_MAX_EVENTS; i++)
        if (impl_->used[i]) n++;
    *count = n;
    if (!out || !cap) return TPLC_EVENTLOG_OK;
    uint32_t w = 0;
    for (uint32_t i = 0; i < TPLC_EVENTLOG_MAX_EVENTS && w < cap; i++)
        if (impl_->used[i]) out[w++] = impl_->ev[i];
    return TPLC_EVENTLOG_OK;
}

int32_t EventlogService::history(void*, TplcEventlogRecord* out,
                                 uint32_t cap, uint32_t* count) {
    if (!count) return TPLC_EVENTLOG_E_PARAM;
    uint32_t n = impl_->hist_len < cap ? impl_->hist_len : cap;
    *count = n;
    if (!out || !n) return TPLC_EVENTLOG_OK;
    /* oldest retained record sits at (hist_wr - hist_len) mod CAP;
       emit the last n of the window, oldest→newest               */
    uint32_t base = (impl_->hist_wr + TPLC_EVENTLOG_HIST_CAP -
                     impl_->hist_len) % TPLC_EVENTLOG_HIST_CAP;
    for (uint32_t i = 0; i < n; i++)
        out[i] = impl_->hist[(base + impl_->hist_len - n + i) %
                             TPLC_EVENTLOG_HIST_CAP];
    return TPLC_EVENTLOG_OK;
}

int32_t EventlogService::importRecs(const TplcEventlogRecord* recs,
                                    uint32_t n) {
    if (!recs && n) return TPLC_EVENTLOG_E_PARAM;
    for (uint32_t i = 0; i < n; i++) {
        TplcEventlogRecord& r =
            impl_->hist[impl_->hist_wr % TPLC_EVENTLOG_HIST_CAP];
        impl_->hist_wr++;
        if (impl_->hist_len < TPLC_EVENTLOG_HIST_CAP) impl_->hist_len++;
        r = recs[i];
    }
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
