/* tplc_s7.h — TplcS7Service: nonblocking Siemens S7comm client
 * (RFC1006 ISO-on-TCP, port 102) with a persistent COTP
 * connection.
 *
 * Wire stack (verified against python-snap7 / snap7 conventions):
 *   TPKT   `03 00 <len16>` + payload
 *   COTP   CR `06 E0 0000 <srcref> 00 C1 02 <l-tsap> C2 02
 *           <r-tsap> C0 01 <pdu-exp>` on connect, DT `02 F0 80`
 *           + data for every S7 PDU afterwards
 *   S7     Job header is 10 bytes `32 01 0000 <ref> <plen>
 *           <dlen>` (no error field); Ack/Ack_Data responses
 *           carry a 12-byte header `32 02|03 0000 <ref> <plen>
 *           <dlen> <errclass> <errcode>`.
 *   Read Var job (func 0x04): param `04 <nitems>` + per-item
 *   S7-ANY spec `12 0A 10 <ts> <count16> <db16> <area> <addr24>`.
 *   Write Var job (func 0x05): same param + data section
 *   `00 <ts> <len16> <data>` (len in bits for byte-family
 *   transport sizes, bytes otherwise).
 *   Read response data items: `rc ts len16 <data>` with odd
 *   items padded by a fill byte except the last; write response
 *   data is a per-item return-code array.
 *
 * Model: s7_open creates a connection handle and drives
 * TCP-connect -> COTP CR/CC -> S7 Setup Communication
 * asynchronously; s7_conn_poll reports 0 connecting / 1 ready /
 * 2 failed. While READY, s7_read/s7_begin_write start one
 * transaction each (one outstanding I/O per connection);
 * s7_poll drives it and reports the response. s7_close tears
 * the connection down. All calls are nonblocking; deadlines use
 * the injected host clock so --sim stays deterministic.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_S7_H
#define TPLC_S7_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_S7_ABI_VERSION 1u
#define TPLC_S7_SVC_NAME "tplc-s7"
#define TPLC_S7_HANDLE_INVALID 0u
#define TPLC_S7_PORT 102u
#define TPLC_S7_MAX_DATA 240u    /* single-item PDU payload cap  */
#define TPLC_S7_MAX_HOST 16u

/* S7 areas (S7-Any area codes) */
#define TPLC_S7_AREA_PE  0x81u  /* process input (I)              */
#define TPLC_S7_AREA_PA  0x82u  /* process output (Q)             */
#define TPLC_S7_AREA_MK  0x83u  /* memory/markers (M)             */
#define TPLC_S7_AREA_DB  0x84u  /* data block                     */
#define TPLC_S7_AREA_CT  0x1Cu  /* counter                        */
#define TPLC_S7_AREA_TM  0x1Du  /* timer                          */

/* conn status (s7_conn_poll info->status) */
#define TPLC_S7_CONN_PENDING 0
#define TPLC_S7_CONN_READY   1
#define TPLC_S7_CONN_ERROR   2

/* io status (s7_poll info->status) */
#define TPLC_S7_ST_PENDING  0
#define TPLC_S7_ST_OK       1
#define TPLC_S7_ST_TIMEOUT  2
#define TPLC_S7_ST_ERROR    3   /* transport/conn failure          */
#define TPLC_S7_ST_BADRESP  4   /* malformed or mismatched PDU     */
#define TPLC_S7_ST_S7ERR    5   /* S7 error class/code or item rc  */

/* service return codes */
#define TPLC_S7_OK        0
#define TPLC_S7_E_PARAM  -1
#define TPLC_S7_E_IO     -2
#define TPLC_S7_E_STATE  -3   /* bad handle / conn not ready       */
#define TPLC_S7_E_BUSY   -4   /* conn already has an io txn        */
#define TPLC_S7_E_RANGE  -5

typedef uint32_t TplcS7ConnHandle;
typedef uint32_t TplcS7IoHandle;

/* open request — field order avoids interior padding (32B) */
typedef struct TplcS7OpenReq {
    char     host[TPLC_S7_MAX_HOST];  /* dotted-quad IPv4           */
    uint16_t port;          /* 0 = TPLC_S7_PORT (102)           */
    uint16_t rack;          /* S7 rack (0..15) — folded into the
                               COTP called TSAP as (rack<<5)|slot
                               under the 0x01 high byte          */
    uint16_t slot;          /* S7 slot (0..31)                   */
    uint16_t reserved;
    uint32_t timeout_ms;    /* 0 = 5000                          */
    uint32_t reserved2;
} TplcS7OpenReq;             /* 32 bytes                          */

/* scalar conn status (s7_conn_poll) */
typedef struct TplcS7ConnInfo {
    int32_t  status;        /* TPLC_S7_CONN_*                    */
    int32_t  err;           /* transport errno or S7 err code    */
    uint32_t reserved[14];
} TplcS7ConnInfo;            /* 64 bytes                          */

/* address spec shared by read/write (packed into the S7-ANY item) */
typedef struct TplcS7Addr {
    uint32_t area;          /* TPLC_S7_AREA_*                    */
    uint32_t db;            /* data block number (DB area only)  */
    uint32_t byte_off;      /* byte offset (24-bit on the wire)  */
    uint32_t bit_off;       /* 0..7 = bit access; 0xFFFFFFFF =
                               word/byte access                  */
    uint32_t len;           /* bytes for word access, ignored
                               for bit access                    */
    uint32_t reserved;
} TplcS7Addr;                /* 24 bytes                          */

/* read request */
typedef struct TplcS7ReadReq {
    TplcS7Addr addr;
    uint32_t timeout_ms;    /* 0 = 2000                          */
    uint32_t reserved;
} TplcS7ReadReq;             /* 32 bytes                          */

/* write request — data inline (240B cap) */
typedef struct TplcS7WriteReq {
    TplcS7Addr addr;
    uint32_t dlen;          /* <= TPLC_S7_MAX_DATA               */
    uint32_t timeout_ms;
    uint32_t reserved;
    uint8_t  data[TPLC_S7_MAX_DATA];
} TplcS7WriteReq;            /* 276 bytes                         */

/* io poll output: scalar state only (64 bytes). Response item
   return code and byte count are scalars; the payload is copied
   into the caller's buffer by s7_poll. */
typedef struct TplcS7RespInfo {
    int32_t  status;        /* TPLC_S7_ST_*                      */
    int32_t  err;           /* transport errno or S7 err word    */
    uint32_t item_rc;       /* response item return code (0xFF ok)*/
    uint32_t data_len;      /* payload bytes in the val buffer   */
    uint32_t reserved[12];
} TplcS7RespInfo;            /* 64 bytes                          */

typedef struct TplcS7Service {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open a connection: nonblocking TCP connect + COTP CR/CC +
       S7 Setup Communication. Returns TPLC_S7_OK and a live
       connection handle; drive it with s7_conn_poll until
       status leaves PENDING. */
    int32_t (*s7_open)(void* svc, const TplcS7OpenReq* req,
                       TplcS7ConnHandle* out);
    /* drive the connection handshake: status 0 pending /
       1 ready / 2 error (err carries errno or S7 error word) */
    int32_t (*s7_conn_poll)(void* svc, TplcS7ConnHandle ch,
                            TplcS7ConnInfo* info);
    /* start a Read Var transaction on a READY connection.
       Returns TPLC_S7_OK and an io handle, or a negative code
       (E_STATE if the conn is not ready, E_BUSY if it already
       has an outstanding io). */
    int32_t (*s7_read)(void* svc, TplcS7ConnHandle ch,
                       const TplcS7ReadReq* req,
                       TplcS7IoHandle* out);
    int32_t (*s7_write)(void* svc, TplcS7ConnHandle ch,
                        const TplcS7WriteReq* req,
                        TplcS7IoHandle* out);
    /* drive an io transaction: info reports scalar state; the
       response payload is copied into val as raw bytes
       (data_len bytes, up to val_cap; value_len-style truncation
       does not apply — data_len is the copied length).
       For writes val may be NULL. Always returns TPLC_S7_OK on
       a live handle; completed transactions are dropped. */
    int32_t (*s7_poll)(void* svc, TplcS7IoHandle h,
                       TplcS7RespInfo* info,
                       uint8_t* val, uint32_t val_cap);
    int32_t (*s7_abort)(void* svc, TplcS7IoHandle h);
    int32_t (*s7_close)(void* svc, TplcS7ConnHandle ch);
} TplcS7Service;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_S7_H */
