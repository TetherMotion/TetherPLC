/* tplc_iec870.h — TplcIec870Service: nonblocking IEC 60870-5-104
 * client (master / control station) over TCP port 2404.
 *
 * Wire stack (verified against lib60870-CS104):
 *   APCI  `68 <len> <control> [seq] [ASDU]` — len counts the
 *         bytes after the len octet.
 *   I fmt `68 LL CSlo CShi NRlo NRhi <asdu>`: 15-bit send/recv
 *         sequence numbers stored shifted left 1 (bit0 = 0).
 *   U fmt `68 04 <func> 00 00 00`: STARTDT act 0x07 / con 0x0B,
 *         STOPDT act 0x13 / con 0x23, TESTFR act 0x43 / con 0x83.
 *   S fmt `68 04 01 00 NRlo NRhi`: TESTFR act carrying the
 *         receive sequence number, used to acknowledge I-frames.
 *   Handshake: TCP connect -> STARTDT act -> STARTDT con ->
 *         data transfer active. STOPDT act -> con -> close.
 *   ASDU  `type vsq cot[1..2] ca[1..2] <info objects>`; the
 *         service frames/deframes the APCI and link layer and
 *         passes the ASDU header fields + raw info-object
 *         payload to the caller.
 *
 * Model: iec870_open creates a connection handle and drives
 * TCP-connect -> STARTDT asynchronously; iec870_conn_poll
 * reports 0 pending / 1 ready / 2 failed plus a substate
 * (1=tcp, 2=startdt, 3=active, 4=stopdt). While READY,
 * iec870_send queues one ASDU as an I-frame (one outstanding
 * tx per connection, k-window enforced) and iec870_tx_poll
 * reports the flush; iec870_rx_poll pops the next received
 * ASDU (consume-on-read). iec870_close sends STOPDT act and
 * closes without awaiting the con. All calls are nonblocking;
 * deadlines use the injected host clock so --sim stays
 * deterministic.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_IEC870_H
#define TPLC_IEC870_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_IEC870_ABI_VERSION 1u
#define TPLC_IEC870_SVC_NAME "tplc-iec870"
#define TPLC_IEC870_HANDLE_INVALID 0u
#define TPLC_IEC870_PORT 2404u
#define TPLC_IEC870_MAX_ASDU 240u   /* info-object payload cap     */
#define TPLC_IEC870_MAX_HOST 16u
#define TPLC_IEC870_RX_QUEUE 4u     /* received-ASDU queue depth   */

/* conn status (iec870_conn_poll info->status) */
#define TPLC_IEC870_CONN_PENDING 0
#define TPLC_IEC870_CONN_READY   1
#define TPLC_IEC870_CONN_ERROR   2

/* conn substate (iec870_conn_poll info->substate) */
#define TPLC_IEC870_SUB_NONE    0
#define TPLC_IEC870_SUB_TCP     1
#define TPLC_IEC870_SUB_STARTDT 2
#define TPLC_IEC870_SUB_ACTIVE  3
#define TPLC_IEC870_SUB_STOPDT  4

/* tx status (iec870_tx_poll info->status) */
#define TPLC_IEC870_TX_PENDING 0
#define TPLC_IEC870_TX_OK      1
#define TPLC_IEC870_TX_ERROR   2
#define TPLC_IEC870_TX_KWINDOW 3   /* unacked I-frames >= k        */

/* rx status (iec870_rx_poll info->status) */
#define TPLC_IEC870_RX_NODATA  0
#define TPLC_IEC870_RX_ASDU    1

/* service return codes */
#define TPLC_IEC870_OK        0
#define TPLC_IEC870_E_PARAM  -1
#define TPLC_IEC870_E_IO     -2
#define TPLC_IEC870_E_STATE  -3   /* bad handle / conn not ready   */
#define TPLC_IEC870_E_BUSY   -4   /* conn already has a tx         */
#define TPLC_IEC870_E_RANGE  -5

typedef uint32_t TplcIec870ConnHandle;
typedef uint32_t TplcIec870TxHandle;

/* open request — field order avoids interior padding (36B) */
typedef struct TplcIec870OpenReq {
    char     host[TPLC_IEC870_MAX_HOST];  /* dotted-quad IPv4    */
    uint16_t port;          /* 0 = TPLC_IEC870_PORT (2404)      */
    uint16_t ca_size;       /* common-address octets 1 or 2,
                               0 = 2                                */
    uint16_t cot_size;      /* cause-of-transmission octets 1 or
                               2 (2nd octet = originator addr),
                               0 = 2                                */
    uint16_t reserved;
    uint32_t timeout_ms;    /* 0 = 5000 (connect + handshake)    */
    uint32_t keepalive_ms;  /* TESTFR interval, 0 = 10000        */
    uint32_t reserved2;
} TplcIec870OpenReq;        /* 36 bytes                          */

/* scalar conn status (iec870_conn_poll) */
typedef struct TplcIec870ConnInfo {
    int32_t  status;        /* TPLC_IEC870_CONN_*                */
    int32_t  substate;      /* TPLC_IEC870_SUB_*                 */
    int32_t  err;           /* transport errno or protocol code  */
    uint32_t rx_dropped;    /* ASDUs dropped (queue overflow)    */
    uint32_t reserved[11];
} TplcIec870ConnInfo;       /* 64 bytes                          */

/* send request — ASDU header fields + raw info-object payload.
   cot: bits 0..7 = encoded COT octet (cause type bits 0..5,
   PN bit6, TN bit7), bits 8..15 = originator address (sent
   only when the conn cot_size is 2). ca: low octets per the
   conn ca_size (LE). vsq: raw octet (bit7 sequence flag,
   bits 0..6 object count). */
typedef struct TplcIec870TxReq {
    uint32_t type_id;       /* ASDU type identifier              */
    uint32_t vsq;           /* raw VSQ octet                     */
    uint32_t cot;           /* encoded COT (+OA in bits 8..15)   */
    uint32_t ca;            /* common address                    */
    uint32_t plen;          /* payload bytes <= MAX_ASDU         */
    uint32_t reserved;
    uint8_t  payload[TPLC_IEC870_MAX_ASDU];
} TplcIec870TxReq;          /* 264 bytes                         */

/* tx poll output: scalar state only (64 bytes) */
typedef struct TplcIec870TxInfo {
    int32_t  status;        /* TPLC_IEC870_TX_*                  */
    int32_t  err;           /* transport errno                   */
    uint32_t reserved[14];
} TplcIec870TxInfo;         /* 64 bytes                          */

/* rx poll output (iec870_rx_poll) — scalar fields; the info-
   object payload is copied into the caller's buffer.
   status 0 = no ASDU pending, 1 = ASDU delivered (payload_len
   bytes copied, truncated to cap). cot/ca echo the raw encoded
   values parsed per the conn's cot_size/ca_size. */
typedef struct TplcIec870RxInfo {
    int32_t  status;        /* TPLC_IEC870_RX_*                  */
    int32_t  err;           /* transport/protocol error          */
    uint32_t type_id;       /* ASDU type identifier              */
    uint32_t vsq;           /* raw VSQ octet                     */
    uint32_t cot;           /* raw COT field (cot octet + OA)    */
    uint32_t ca;            /* raw CA field                      */
    uint32_t payload_len;   /* info-object bytes in payload      */
    uint32_t i_frame_seq;   /* I-frame send sequence number      */
    uint32_t reserved[8];
} TplcIec870RxInfo;         /* 64 bytes                          */

typedef struct TplcIec870Service {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open a connection: nonblocking TCP connect + STARTDT
       handshake. Returns TPLC_IEC870_OK and a live conn handle;
       drive it with iec870_conn_poll until status leaves
       PENDING. */
    int32_t (*iec870_open)(void* svc, const TplcIec870OpenReq* req,
                           TplcIec870ConnHandle* out);
    /* drive connect/handshake/data-transfer: status 0 pending /
       1 ready / 2 error (err carries errno or a protocol code);
       substate 1=tcp 2=startdt 3=active 4=stopdt. */
    int32_t (*iec870_conn_poll)(void* svc, TplcIec870ConnHandle ch,
                                TplcIec870ConnInfo* info);
    /* queue one ASDU as an I-frame on a READY conn. Returns
       TPLC_IEC870_OK and a tx handle, or a negative code
       (E_STATE if not ready, E_BUSY if a tx is outstanding). */
    int32_t (*iec870_send)(void* svc, TplcIec870ConnHandle ch,
                           const TplcIec870TxReq* req,
                           TplcIec870TxHandle* out);
    /* drive a tx: info reports scalar state; completed txns are
       dropped. status 0 pending / 1 ok / 2 error / 3 k-window. */
    int32_t (*iec870_tx_poll)(void* svc, TplcIec870TxHandle h,
                              TplcIec870TxInfo* info);
    int32_t (*iec870_tx_abort)(void* svc, TplcIec870TxHandle h);
    /* pop the next received ASDU (consume-on-read): info->status
       1 = ASDU delivered with payload_len bytes copied into
       payload (cap-bounded); 0 = none pending. */
    int32_t (*iec870_rx_poll)(void* svc, TplcIec870ConnHandle ch,
                              TplcIec870RxInfo* info,
                              uint8_t* payload, uint32_t cap);
    /* send STOPDT act (best effort) and close the connection. */
    int32_t (*iec870_close)(void* svc, TplcIec870ConnHandle ch);
} TplcIec870Service;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_IEC870_H */
