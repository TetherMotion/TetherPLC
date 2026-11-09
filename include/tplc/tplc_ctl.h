/* tplc_ctl.h — control-channel wire protocol (plan doc 04 Appendix I).
 * SOCK_STREAM UDS framing shared by tether-plc (server), tplc-ctl
 * (client CLI), and tests. C-visible; P0 op subset implemented.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_CTL_H
#define TPLC_CTL_H

#include <stdint.h>
#include <stddef.h>
#include <tplc/tplc_abi.h>   /* TPLC_PROG_NAME_MAX / TASK_NAME_MAX */

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_CTL_PROTO_VER   3u
#define TPLC_CTL_HDR_SIZE    16u
#define TPLC_CTL_MAX_PAYLOAD (8u * 1024u * 1024u)

/* frame header (all LE):
 *  +0  u16 msg_type     +2  u16 flags
 *  +4  u32 payload_len  +8  u64 req_id                                     */
enum {
    TPLC_CTL_F_MORE    = 1u << 0,   /* chunked: more frames follow */
    TPLC_CTL_F_EVENT   = 1u << 1,   /* unsolicited event           */
    TPLC_CTL_F_CANCEL  = 1u << 2    /* cancel outstanding chunked  */
};

/* msg_type ranges (doc 04 App I.1) — P0 implements the marked subset */
enum TplcCtlOp {
    /* session 0x0100 */
    TPLC_OP_HELLO          = 0x0101,  /* P0 */
    TPLC_OP_HELLO_ACK      = 0x0102,  /* P0 (server→client)          */
    TPLC_OP_PING           = 0x0103,  /* P0                          */
    TPLC_OP_SET_NOTIFY     = 0x0104,  /* P0                          */
    /* deploy 0x0200 — P2 */
    /* deploy 0x0200 */
    TPLC_OP_DEPLOY_BEGIN   = 0x0201,  /* P2 {size,sha256,mode}         */
    TPLC_OP_DEPLOY_CHUNK   = 0x0202,  /* P2 {upload_id,off,data}       */
    TPLC_OP_DEPLOY_COMMIT  = 0x0203,  /* P2 {upload_id}                */
    TPLC_OP_DEPLOY_ABORT   = 0x0204,  /* P2 {upload_id}                */
    TPLC_OP_PKG_LIST       = 0x0205,  /* P2 */
    TPLC_OP_PKG_INFO       = 0x0206,  /* P2 {package_uuid}             */

    /* program 0x0300 */
    TPLC_OP_PROG_PREPARE   = 0x0301,  /* P2 */
    TPLC_OP_PROG_START     = 0x0302,  /* P0 */
    TPLC_OP_PROG_STOP      = 0x0303,  /* P0 */
    TPLC_OP_PROG_KILL      = 0x0304,  /* P2 */
    TPLC_OP_PROG_RESET     = 0x0305,  /* P2 */
    TPLC_OP_PROG_UNLOAD    = 0x0306,  /* P2 */
    TPLC_OP_PROG_STATUS    = 0x0307,  /* P0 */
    TPLC_OP_PROG_LIST      = 0x0308,  /* P0 */
    TPLC_OP_PROG_LAYOUT    = 0x0309,  /* P3 → {uuid,layout_hash,layd}  */
    TPLC_OP_PROG_SWAP      = 0x030A,  /* P3 {pkg uuid|path}            */
    TPLC_OP_PROG_ROLLBACK  = 0x030B,  /* P3 */
    /* task 0x0400 */
    TPLC_OP_TASK_STATS     = 0x0401,  /* P0 */
    TPLC_OP_TASK_SUSPEND   = 0x0402,  /* P6 {u32 task_idx}          */
    TPLC_OP_TASK_RESUME    = 0x0403,  /* P6 {u32 task_idx}          */
    TPLC_OP_TASK_STEP      = 0x0404,  /* P6 {u32 task_idx, u32 n}   */
    /* debug 0x0500 */
    TPLC_OP_DBG_RESOLVE    = 0x0501,  /* P6 {u16 nlen,name}; -410
                                         unknown, -411 removed by
                                         the last committed swap   */
    TPLC_OP_DBG_READ       = 0x0502,  /* P6 {u64 off, u32 len}      */
    TPLC_OP_DBG_WRITE      = 0x0503,  /* P6 {u64 off, u32 len,bytes}*/
    TPLC_OP_DBG_STATE      = 0x0504,  /* P6 → blob/retain extents   */
    TPLC_OP_DBG_FORCE      = 0x0505,  /* P6 {u64 off, u32 len,bytes}*/
    TPLC_OP_DBG_FORCE_LIST = 0x0506,  /* P6 → {u32 n, entries}      */
    TPLC_OP_DBG_FORCE_CLEAR= 0x0507,  /* P6 {u32 handle} 0xFF..=all */
    /* break_when: conditional suspend — {u32 task_idx, u64 off,
       u32 len(1|2|4|8), u8 cmp(0== 1!= 2< 3<= 4> 5>=), u8 pad[3],
       u64 value} → {u32 handle}                                   */
    TPLC_OP_DBG_BREAK      = 0x0508,
    /* list entries: {u32 handle, u32 task, u64 off, u32 len,
       u8 cmp, u8 kind(0 suspend|1 capture), u8 stale, u8 pad,
       u64 val, u64 hits}                                          */
    TPLC_OP_DBG_BREAK_LIST = 0x0509,  /* → {u32 n, entries}         */
    TPLC_OP_DBG_BREAK_CLEAR= 0x050A,  /* {u32 handle} 0xFF..=all    */
    /* flight recorder (doc 12 §7): fixed 4096-entry ring of per-scan
       channel samples. One frame per scan that ran ≥1 task; frames
       carry a seq so a wrapped ring shows as a gap, never silent
       loss.
       CONF:   {u32 n_ch, u32 flags, {u64 off, u32 len, u32 pad}*}
               n_ch ≤ 8, per-ch len ≤ 8, Σlen ≤ 64 → {u32 frame_len};
               n_ch = 0 disarms.
               flags bit0 NO_FAULT_FREEZE: opt out of the default
               freeze-on-fault (the ring keeps post-mortem samples)
       STATUS: → {u32 n_ch, u32 frame_len, u64 written, u64 avail,
                  u32 flags}  flags bit0 = frozen
       READ:   {u64 from_seq, u32 max} → {u64 first_seq, u32 n,
               u32 frame_len, {u64 seq, u64 ts, u8 data[f..]}*}
       CLEAR:  {} — disarm and drop the ring                        */
    TPLC_OP_TRACE_CONF     = 0x050B,
    TPLC_OP_TRACE_STATUS   = 0x050C,
    TPLC_OP_TRACE_READ     = 0x050D,
    TPLC_OP_TRACE_CLEAR    = 0x050E,
    /* capture trigger (doc 12 §5 dbg.capture_on): same 28-byte
       request payload as DBG_BREAK; on hit the flight recorder
       freezes instead of suspending the task. Listed in
       DBG_BREAK_LIST with the flag byte set.                       */
    TPLC_OP_DBG_CAPTURE    = 0x050F,
    /* watch (doc 12 §3): named read sets over blob ranges. ADD:
       {u32 n, {u64 off, u32 len, u16 nlen, char name[nlen]}*},
       n ≤ 16 items, per-item len 1..64, Σlen ≤ 512 → {u32 handle}.
       nlen>0 resolves the path through the symmap at arm time and
       keeps it: after a swap the item re-resolves, while unnamed
       (offset-only) items and vanished paths go stale (GET/stream
       report len=0). GET returns boundary-consistent bytes
       {u32 n, {u32 len, u8 data[len]}*}. STREAM arms a push: at
       most rate_hz (1..50; 0 disarms) EV_DBG_WATCH frames per
       second, sampled at task boundaries. REMOVE takes a handle
       or 0xFFFFFFFF for all. LIST reports
       {handle, n_items, rate_hz, total_len, n_stale} rows.       */
    TPLC_OP_DBG_WATCH_ADD    = 0x0510,
    TPLC_OP_DBG_WATCH_GET    = 0x0511,
    TPLC_OP_DBG_WATCH_REMOVE = 0x0512,
    TPLC_OP_DBG_WATCH_STREAM = 0x0513,
    TPLC_OP_DBG_WATCH_LIST   = 0x0514,
    /* expression triggers (doc 12 §M): {u32 task_idx, u16 elen,
       char expr[]} → {u32 handle}. expr compiles to a bounded stack
       machine over resolved symmap paths (integer/boolean only,
       ≤64 ops); paths become per-scan sampled slots so changed()/
       rising()/falling() see a stable previous value. BREAK_EXPR
       suspends the task on true; CAPTURE_EXPR freezes the recorder.
       Listed in DBG_BREAK_LIST with len=0/cmp=0xFF.               */
    TPLC_OP_DBG_BREAK_EXPR   = 0x0515,
    TPLC_OP_DBG_CAPTURE_EXPR = 0x0516,
    /* post-migration report (doc 12 §7): what the last swap's
       migrate rules did. Replies {u8 same_layout, u8 mig_status,
       u16 pad, u32 n_rules, rules*} — each rule {u8 kind, u8 sev,
       u16 tid, u32 aux, u64 old_off, u32 old_len, u32 old_sym,
       u64 new_off, u32 new_len, u32 new_sym, u8 old_vl, u8 new_vl,
       u16 pad, u8 old_v[8], u8 new_v[8], str old_name, str new_name}.
       kind: 0 copy 1 transform 2 default-init 3 dropped 4 deny
       5 alias. Errors DBG_STATE when no swap has committed.      */
    TPLC_OP_DBG_STATE_DIFF   = 0x0517,
    /* blob layout browser (doc 12 §7): per-field rows from LAYD +
       SMAP. Replies {u32 n, rows*} — each row {u64 off, u64 size,
       u32 type_ref, u8 kind, u8 flags, u8 region, u8 access,
       u8 name_len, u8 name[name_len]}.
       LAYD field flags: bit0 retain, bit1 const (read-only: writes
       and forces on the field are denied). SMAP access: 0 rw,
       1 ro (VAR_CONSTANT), 2 wo marker (%Q channel entries).
       Errors DBG_STATE when the loaded program carries no layout
       descriptors.                                                */
    TPLC_OP_DBG_FIELDS       = 0x0518,
    /* retain 0x0600 — P2 */
    TPLC_OP_RETAIN_STATUS  = 0x0601,
    TPLC_OP_RETAIN_FLUSH   = 0x0602,
    TPLC_OP_RETAIN_CLEAR   = 0x0603,
    TPLC_OP_RETAIN_DUMP    = 0x0604,
    /* image 0x0700 */
    TPLC_OP_IMAGE_STATS    = 0x0701,  /* P0 */
    /* shadow run 0x0800 (doc 19): candidate program on identical
       inputs; its %Q lands in a private region, never the image   */
    TPLC_OP_SHADOW_LOAD    = 0x0801,  /* {u16 plen,path} → status    */
    TPLC_OP_SHADOW_STATUS  = 0x0802,  /* → TplcCtlShadowStatus       */
    TPLC_OP_SHADOW_COMPARE = 0x0803,  /* {u32 max} → diff ring tail  */
    TPLC_OP_SHADOW_PROMOTE = 0x0804,  /* ∅ → post-promote status     */
    TPLC_OP_SHADOW_DROP    = 0x0805,  /* ∅                           */
    TPLC_OP_SHADOW_RESET   = 0x0806,  /* ∅ → migrate result          */
    TPLC_OP_SHADOW_IMAGE   = 0x0807,  /* {u64 off,u32 len} → shadow %Q */
    /* runtime 0x0900 */
    TPLC_OP_RT_STATUS      = 0x0901,  /* P0 */
    TPLC_OP_RT_SHUTDOWN    = 0x0905,  /* P0 {u8 mode}                */
    /* object dictionary 0x0A00 (doc 16 §7 iteration-2): a
       CiA-405-style index/subindex view of the management state
       model. READ {u16 index, u8 sub, u8 pad} →
       {u8 type, u8 access, u16 len, u8 data[len]};
       LIST ∅ → {u32 n, {u16 index, u8 sub, u8 access, u16 len,
                        u8 nlen, char name[nlen]}*};
       WRITE {u16 index, u8 sub, u8 pad, u16 len, u8 data[len]} →
       {u8 ok}. type: 0 u8 1 u16 2 u32 3 u64 4 i32 5 string
       6 bytes. access: 0 ro 1 rw 2 wo.
       Errors: MGMT_OP (-500) unknown index/sub, MGMT_SCHEMA (-503)
       type/length mismatch or write to a read-only entry,
       MGMT_AUTHZ (-502) entry needs a higher role than the session. */
    TPLC_OP_OD_READ        = 0x0A01,
    TPLC_OP_OD_LIST        = 0x0A02,
    TPLC_OP_OD_WRITE       = 0x0A03,
    /* map 0x0C00 */
    TPLC_OP_MAP_GET        = 0x0C01,  /* → deploy-map section bytes   */
    /* journal 0x0D00 */
    TPLC_OP_JOURNAL_TAIL   = 0x0D01,  /* {u32 n} → u32 n, str lines[] */
    /* events 0x0E00 */
    TPLC_EV_PROG_TRANSITION = 0x0E01,
    TPLC_EV_FAULT           = 0x0E02,
    TPLC_EV_DBG_BREAK       = 0x0E03, /* {u32 handle, u32 task, u64 v}*/
    /* {u32 handle, u64 seq, u32 n, {u32 len, u8 data[len]}*}        */
    TPLC_EV_DBG_WATCH       = 0x0E04,
    /* error frame — response with F_EVENT? No: same msg_type as the
       request, payload = TplcCtlError, flags bit15 marks error        */
    TPLC_CTL_F_ERROR       = 1u << 15
};

/* client→server */
#pragma pack(push, 1)
typedef struct TplcCtlHello {
    uint16_t proto_ver;
    uint16_t _pad;
    uint32_t caps;
    /* str client_name: u16 len + bytes follows */
} TplcCtlHello;

/* server→client */
typedef struct TplcCtlHelloAck {
    uint16_t proto_ver;
    uint16_t _pad;
    uint32_t runtime_ver;
    uint32_t caps;
    uint64_t session_id;
    uint8_t  authz;
    uint8_t  _pad2[7];
} TplcCtlHelloAck;

typedef struct TplcCtlError {
    uint32_t code;
    uint32_t subcode;
    /* str msg follows */
} TplcCtlError;

typedef struct TplcCtlTaskStat {
    uint8_t  name[TPLC_PROG_NAME_MAX]; /* task name, NUL-padded      */
    uint8_t  kind;
    uint8_t  state;                    /* 0 idle 1 scheduled 2 susp  */
    uint16_t _pad;
    uint32_t _pad2;
    uint64_t period_ns;
    uint64_t runs;
    uint64_t late;
    uint64_t faults;
    int32_t  last_rc;
    int32_t  last_late_ns;             /* clamped                    */
    /* proto v2 tail */
    uint64_t overruns;
    uint64_t exec_max_ns;
    uint64_t exec_avg_ns;
} TplcCtlTaskStat;

typedef struct TplcCtlProgStatus {
    uint8_t  program_uuid[16];
    uint8_t  name[TPLC_PROG_NAME_MAX];
    uint8_t  state;                    /* 0 stopped 1 running 2 fault */
    uint8_t  _pad0;
    uint16_t _pad1;
    uint32_t task_count;
    uint64_t cycles;                   /* program cycle_no            */
    int32_t  last_fault_code;
    uint32_t force_count;              /* doc 12 §8: globally visible */
    /* proto v2 — doc 16 §3.3 "405 answer" */
    uint8_t  package_hash[32];         /* pkg content_hash            */
    uint8_t  layout_hash[32];
    uint8_t  version[32];              /* manifest version string     */
    uint32_t fault_task;
    uint32_t _pad2;
    uint64_t fault_cycle;
    uint8_t  retain_restored;
    uint8_t  _pad3[7];
    uint64_t retain_dirty_bytes;
    uint64_t retain_last_flush_ns;
    uint64_t deployed_at_ns;           /* realtime                    */
    uint32_t deployed_by_uid;
    uint32_t _pad4;
    uint8_t  predecessor_uuid[16];
    uint8_t  swap_stage;               /* 0 none 1 quarantine         */
    uint8_t  _pad5[7];
    uint32_t quarantine_left;
    uint32_t _pad6;
    uint64_t epoch;                    /* bumps on program-set change */
    /* proto v3 tail — shadow run (doc 19) */
    uint8_t  shadow_state;             /* 0 none, 1 running           */
    uint8_t  _pad7[7];
    uint64_t shadow_cycles;
    uint64_t shadow_missed;
    uint64_t shadow_diff_count;
    uint64_t shadow_last_diff_cycle;
    /* TplcCtlTaskStat tasks[task_count] follows */
} TplcCtlProgStatus;

/* SHADOW_STATUS reply */
typedef struct TplcCtlShadowStatus {
    uint8_t  program_uuid[16];
    uint8_t  name[TPLC_PROG_NAME_MAX];
    uint8_t  state;                    /* 0 none, 1 running           */
    uint8_t  _pad[7];
    uint64_t cycles;                   /* shadow task runs            */
    uint64_t missed;                   /* budget-skipped cycles       */
    uint64_t diff_count;               /* total diff entries          */
    uint64_t last_diff_cycle;
    uint64_t runs;                     /* sum of task runs            */
} TplcCtlShadowStatus;

/* SHADOW_COMPARE ring entry (one per differing byte-run) */
typedef struct TplcCtlShadowDiff {
    uint64_t cycle;
    uint64_t img_off;
    uint32_t len;                      /* ≤ 32                        */
    uint8_t  active_bytes[32];
    uint8_t  shadow_bytes[32];
} TplcCtlShadowDiff;

typedef struct TplcCtlRtStatus {
    uint32_t proto_ver;
    uint32_t uptime_s;
    uint32_t prog_count;
    uint32_t running_count;
    uint32_t master_status;
    uint32_t plc_status;
    uint64_t master_hb_ns;
    uint64_t plc_hb_ns;
    uint8_t  version[TPLC_PROG_NAME_MAX];
} TplcCtlRtStatus;

typedef struct TplcCtlProgStateChange {
    uint8_t prev_state;
    uint8_t new_state;
    uint16_t _pad;
} TplcCtlProgStateChange;

typedef struct TplcCtlImageStats {
    uint64_t epoch;
    uint64_t in_seq, out_seq;
    uint64_t in_writes, out_writes;
    uint64_t in_retries, out_retries;
    uint64_t in_size, out_size;
} TplcCtlImageStats;
#pragma pack(pop)

/* error codes for TplcCtlError.code — reuse TPLC_* spaces where they
   exist; ctl-local codes at -400.. */
enum {
    TPLC_E_CTL_STATE    = -400,   /* op illegal in current state    */
    TPLC_E_CTL_AUTHZ    = -401,
    TPLC_E_CTL_UNKNOWN_OP = -402,
    TPLC_E_CTL_PROTO    = -403,
    TPLC_E_CTL_PAYLOAD  = -404,
    /* debug plane (doc 12 §9) */
    TPLC_E_DBG_RESOLVE  = -410,
    TPLC_E_DBG_STALE    = -411,
    TPLC_E_DBG_OVERLOAD = -412,
    TPLC_E_DBG_RATE     = -413,
    TPLC_E_FORCE_DENIED = -414,
    TPLC_E_DBG_ACCESS   = -415,
    TPLC_E_DBG_STATE    = -416,
    TPLC_E_TRACE_CONFIG = -417,
    /* management plane (doc 16 §8) */
    TPLC_E_MGMT_OP      = -500,
    TPLC_E_MGMT_STATE   = -501,   /* illegal transition; msg carries
                                   current+requested states         */
    TPLC_E_MGMT_AUTHZ   = -502,
    TPLC_E_MGMT_SCHEMA  = -503
};

/* TPLC_EV_PROG_TRANSITION payload (proto v2): enough for a client to
   run the §2 state machine purely from events (doc 16 §4).         */
#pragma pack(push, 1)
typedef struct TplcCtlEvTransition {
    uint8_t  program_uuid[16];
    uint8_t  from_state;
    uint8_t  to_state;
    uint8_t  reason;                /* 0 op, 1 fault, 2 rollback,
                                       3 promote (doc 19)          */
    uint8_t  _pad;
    uint32_t _pad2;
    uint64_t ts_ns;                 /* realtime                      */
    uint64_t epoch;
} TplcCtlEvTransition;
#pragma pack(pop)

/* TLV string helper: u16 len + bytes (no NUL on the wire) */
static inline uint32_t tplc_ctl_str_len(const char* s) {
    uint32_t n = 0;
    while (s[n] && n < 0xFFFFu) n++;
    return n;
}

#ifdef __cplusplus
} /* extern "C" */
#endif
#endif /* TPLC_CTL_H */
