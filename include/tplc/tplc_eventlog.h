/* tplc_eventlog.h — C ABI for the tplc-eventlog host service.
 * Bounded event journal for the Tc3_EventLogger surface
 * (FB_TcMessage / FB_TcAlarm / FB_TcEvent).  The journal holds
 * a fixed ring of event records with TwinCAT-style state
 * (raised -> cleared -> confirmed); entries are addressed by
 * small integer ids, never pointers.
 * Vtable is fixed — append new fns at the end.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_EVENTLOG_H
#define TPLC_EVENTLOG_H

#include <stdint.h>

#define TPLC_EVENTLOG_ABI_VERSION 2u

#define TPLC_EVENTLOG_MAX_EVENTS 128u
#define TPLC_EVENTLOG_TEXT_CAP   80u
#define TPLC_EVENTLOG_HIST_CAP   512u

/* TwinCAT-compatible severity levels */
#define TPLC_EVENTLOG_SEV_VERBOSE   0u
#define TPLC_EVENTLOG_SEV_INFO      1u
#define TPLC_EVENTLOG_SEV_WARNING   2u
#define TPLC_EVENTLOG_SEV_ERROR     3u
#define TPLC_EVENTLOG_SEV_CRITICAL  4u

/* journal entry states */
#define TPLC_EVENTLOG_ST_RAISED     1u
#define TPLC_EVENTLOG_ST_CLEARED    2u
#define TPLC_EVENTLOG_ST_CONFIRMED  3u   /* terminal */

typedef struct TplcEventlogInfo {
    uint32_t event_class;   /* event class/category id    */
    uint32_t event_id;      /* caller's event id          */
    uint32_t severity;      /* TPLC_EVENTLOG_SEV_*        */
    uint32_t state;         /* TPLC_EVENTLOG_ST_*         */
    uint32_t args;          /* up to 4 format args packed */
    uint8_t  text[TPLC_EVENTLOG_TEXT_CAP];
    uint8_t  text_len;
    uint8_t  _pad0[3];
    uint64_t raised_ns;     /* CLOCK_REALTIME ns at raise */
    uint64_t cleared_ns;    /* ns at clear (0 if none)    */
    uint64_t confirmed_ns;  /* ns at confirm (0 if none)  */
    uint8_t  reserved[8];
} TplcEventlogInfo;

/* append-only transition record: every raise/clear/confirm lands
   here even after the live slot is released, so alarm history
   survives confirmation (shift reports, post-mortems).          */
typedef struct TplcEventlogRecord {
    uint64_t ts_ns;         /* transition time, realtime ns */
    uint32_t journal_id;    /* live-slot id at transition    */
    uint32_t event_class;
    uint32_t event_id;
    uint32_t severity;
    uint32_t state;         /* state entered                */
    uint32_t text_len;
    uint8_t  text[TPLC_EVENTLOG_TEXT_CAP];
    uint8_t  reserved[12];
} TplcEventlogRecord;

typedef struct TplcEventlogStats {
    uint32_t live_events;    /* raised+cleared, unconfirmed */
    uint32_t raised_total;
    uint32_t cleared_total;
    uint32_t confirmed_total;
    uint32_t dropped_total;  /* ring overflow drops        */
    uint8_t  reserved[28];
} TplcEventlogStats;

typedef struct TplcEventlogService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* raise: register a new event (class/id/severity/text) ->
       journal entry id (1..N) or negative error. */
    int32_t (*ev_raise)(void*, uint32_t event_class,
                        uint32_t event_id, uint32_t severity,
                        const uint8_t* text, uint32_t text_len,
                        uint32_t args);

    /* clear: id -> 0 or error; marks the entry CLEARED. */
    int32_t (*ev_clear)(void*, uint32_t id);

    /* confirm: id -> 0 or error; marks CONFIRMED (terminal) and
       releases the slot. */
    int32_t (*ev_confirm)(void*, uint32_t id);

    /* query: id -> 0 and fills *out, or negative error. */
    int32_t (*ev_query)(void*, uint32_t id, TplcEventlogInfo* out);

    /* stats: -> 0 and fills *out */
    int32_t (*ev_stats)(void*, TplcEventlogStats* out);

    /* list: snapshot of live (unconfirmed) entries. Fills up to
       `cap` records into `out`, stores the live count in `count`
       (which may exceed cap — caller pages by re-calling with a
       larger cap; entries are unordered slot order). -> 0.      */
    int32_t (*ev_list)(void*, TplcEventlogInfo* out, uint32_t cap,
                       uint32_t* count);

    /* history: last `cap` transition records, oldest→newest.
       `count` receives the number written (≤ cap). -> 0.         */
    int32_t (*ev_history)(void*, TplcEventlogRecord* out,
                          uint32_t cap, uint32_t* count);

    /* import: append foreign transition records (e.g. the outgoing
       program's history across an online swap) to the ring.
       Records are copied verbatim; -> 0 or E_PARAM.             */
    int32_t (*ev_import)(void*, const TplcEventlogRecord* recs,
                         uint32_t n);

    void* reserved[5];
} TplcEventlogService;

#define TPLC_EVENTLOG_OK        0
#define TPLC_EVENTLOG_E_PARAM  -1
#define TPLC_EVENTLOG_E_HANDLE -2
#define TPLC_EVENTLOG_E_FULL   -3
#define TPLC_EVENTLOG_E_STATE  -4   /* wrong state for op      */

#endif /* TPLC_EVENTLOG_H */
