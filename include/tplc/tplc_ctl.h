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

#define TPLC_CTL_PROTO_VER   1u
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
    /* program 0x0300 */
    TPLC_OP_PROG_PREPARE   = 0x0301,  /* P2 */
    TPLC_OP_PROG_START     = 0x0302,  /* P0 */
    TPLC_OP_PROG_STOP      = 0x0303,  /* P0 */
    TPLC_OP_PROG_KILL      = 0x0304,  /* P2 */
    TPLC_OP_PROG_RESET     = 0x0305,  /* P2 */
    TPLC_OP_PROG_UNLOAD    = 0x0306,  /* P2 */
    TPLC_OP_PROG_STATUS    = 0x0307,  /* P0 */
    TPLC_OP_PROG_LIST      = 0x0308,  /* P0 */
    /* task 0x0400 */
    TPLC_OP_TASK_STATS     = 0x0401,  /* P0 */
    /* debug 0x0500 — P6 */
    /* retain 0x0600 — P2 */
    TPLC_OP_RETAIN_STATUS  = 0x0601,
    TPLC_OP_RETAIN_FLUSH   = 0x0602,
    TPLC_OP_RETAIN_CLEAR   = 0x0603,
    TPLC_OP_RETAIN_DUMP    = 0x0604,
    /* image 0x0700 */
    TPLC_OP_IMAGE_STATS    = 0x0701,  /* P0 */
    /* runtime 0x0900 */
    TPLC_OP_RT_STATUS      = 0x0901,  /* P0 */
    TPLC_OP_RT_SHUTDOWN    = 0x0905,  /* P0 {u8 mode}                */
    /* events 0x0E00 */
    TPLC_EV_PROG_TRANSITION = 0x0E01,
    TPLC_EV_FAULT           = 0x0E02,
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
    uint32_t _pad2;
    /* TplcCtlTaskStat tasks[task_count] follows */
} TplcCtlProgStatus;

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
    TPLC_E_CTL_PAYLOAD  = -404
};

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
