/* tplc_s5.h — TplcS5Service: nonblocking Siemens S5 client
 * (RK512 job protocol over TCP, 3964 STX/ETX+BCC framing).
 *
 * Wire stack (RK512 computer protocol per Siemens STEP7 RK512
 * documentation and the zxc.wiki RK512 telegram reference):
 *   3964 frame: `02 <rk512 message> <bcc> 03` where bcc is the
 *   XOR of every message byte plus the trailing ETX. Over TCP
 *   the DLE link-layer handshakes of the serial 3964(R)
 *   procedure are not used.
 *   RK512 command telegram: 10-byte header
 *     [0..1] telegram id (00 00 start, FF 00 continuation —
 *            continuation is not implemented, see below)
 *     [2]    order type: 'E' (0x45) FETCH, 'A' (0x41) SEND
 *     [3]    data area:  'D' (0x44) data block (DB)
 *     [4]    DB number (0..255)
 *     [5]    word offset within the DB (0..255 words)
 *     [6..7] data length in words, big-endian
 *     [8]    communication flag byte (0xFF = none)
 *     [9]    flag bit + CPU number (0xFF = unspecified)
 *     [10..] SEND payload (words, big-endian byte order)
 *   RK512 response telegram: 4-byte header
 *     [0..1] telegram id echo, [2] order type echo,
 *     [3]    status (0x00 = ok, otherwise error code),
 *     [4..] FETCH payload (words, big-endian byte order)
 *
 * Documented subset: DB-area word access only, word offset and
 * DB number are each one wire byte (word offset 0..255 words,
 * i.e. byte offsets 0..510 within a DB; DB numbers 0..255),
 * and one job per transaction of at most 255 words. SEND data
 * words are transferred most-significant-byte first; the
 * service returns wire-order bytes so callers see the PLC's
 * word order verbatim. Other RK512 areas (inputs, outputs,
 * memory, flags, timers, counters, DX) and continuation
 * telegrams are not implemented.
 *
 * Model: s5_open creates a connection handle and drives the
 * TCP connect asynchronously; s5_conn_poll reports
 * 0 connecting / 1 ready / 2 failed. While READY, s5_read /
 * s5_write start one transaction each (one outstanding I/O per
 * connection); s5_poll drives it and reports the response.
 * s5_close tears the connection down. All calls are
 * nonblocking; deadlines use the injected host clock so --sim
 * stays deterministic.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_S5_H
#define TPLC_S5_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_S5_ABI_VERSION 1u
#define TPLC_S5_SVC_NAME "tplc-s5"
#define TPLC_S5_HANDLE_INVALID 0u
#define TPLC_S5_PORT 102u
#define TPLC_S5_MAX_DATA 510u   /* 255 words * 2 bytes           */
#define TPLC_S5_MAX_HOST 16u

/* conn status (s5_conn_poll info->status) */
#define TPLC_S5_CONN_PENDING 0
#define TPLC_S5_CONN_READY   1
#define TPLC_S5_CONN_ERROR   2

/* io status (s5_poll info->status) */
#define TPLC_S5_ST_PENDING  0
#define TPLC_S5_ST_OK       1
#define TPLC_S5_ST_TIMEOUT  2
#define TPLC_S5_ST_ERROR    3   /* transport/conn failure          */
#define TPLC_S5_ST_BADRESP  4   /* malformed or mismatched frame   */
#define TPLC_S5_ST_S5ERR    5   /* RK512 status byte nonzero      */

/* service return codes */
#define TPLC_S5_OK        0
#define TPLC_S5_E_PARAM  -1
#define TPLC_S5_E_IO     -2
#define TPLC_S5_E_STATE  -3   /* bad handle / conn not ready       */
#define TPLC_S5_E_BUSY   -4   /* conn already has an io txn        */
#define TPLC_S5_E_RANGE  -5

typedef uint32_t TplcS5ConnHandle;
typedef uint32_t TplcS5IoHandle;

/* open request — field order avoids interior padding (32B) */
typedef struct TplcS5OpenReq {
    char     host[TPLC_S5_MAX_HOST];  /* dotted-quad IPv4           */
    uint16_t port;          /* 0 = TPLC_S5_PORT (102)           */
    uint16_t reserved;
    uint32_t timeout_ms;    /* 0 = 5000                          */
    uint32_t reserved2;
    uint32_t reserved3;
} TplcS5OpenReq;             /* 32 bytes                          */

/* scalar conn status (s5_conn_poll) */
typedef struct TplcS5ConnInfo {
    int32_t  status;        /* TPLC_S5_CONN_*                    */
    int32_t  err;           /* transport errno or RK512 status   */
    uint32_t reserved[14];
} TplcS5ConnInfo;            /* 64 bytes                          */

/* address spec shared by read/write: DB number + byte offset
   within the DB (must be even; the service converts to the
   wire's word addressing) + byte count. */
typedef struct TplcS5Addr {
    uint32_t db;            /* data block number (0..255)        */
    uint32_t byte_off;      /* even byte offset, <= 510          */
    uint32_t len;           /* bytes, 1..TPLC_S5_MAX_DATA        */
    uint32_t reserved;
} TplcS5Addr;                /* 16 bytes                          */

/* read request */
typedef struct TplcS5ReadReq {
    TplcS5Addr addr;
    uint32_t timeout_ms;    /* 0 = 2000                          */
    uint32_t reserved;
} TplcS5ReadReq;             /* 24 bytes                          */

/* write request — data inline (510B cap) */
typedef struct TplcS5WriteReq {
    TplcS5Addr addr;
    uint32_t dlen;          /* <= TPLC_S5_MAX_DATA               */
    uint32_t timeout_ms;
    uint32_t reserved;
    uint8_t  data[TPLC_S5_MAX_DATA];
} TplcS5WriteReq;            /* 538 bytes                         */

/* io poll output: scalar state only (64 bytes) */
typedef struct TplcS5RespInfo {
    int32_t  status;        /* TPLC_S5_ST_*                      */
    int32_t  err;           /* transport errno or RK512 status   */
    uint32_t data_len;      /* payload bytes in the val buffer   */
    uint32_t reserved[13];
} TplcS5RespInfo;            /* 64 bytes                          */

typedef struct TplcS5Service {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open a connection: nonblocking TCP connect; the RK512
       layer needs no handshake, so the conn is READY as soon
       as TCP connects. */
    int32_t (*s5_open)(void* svc, const TplcS5OpenReq* req,
                       TplcS5ConnHandle* out);
    /* drive the connection: status 0 pending / 1 ready /
       2 error (err carries errno or RK512 status) */
    int32_t (*s5_conn_poll)(void* svc, TplcS5ConnHandle ch,
                            TplcS5ConnInfo* info);
    /* start a FETCH transaction on a READY connection.
       Returns TPLC_S5_OK and an io handle, or a negative code
       (E_STATE if the conn is not ready, E_BUSY if it already
       has an outstanding io). */
    int32_t (*s5_read)(void* svc, TplcS5ConnHandle ch,
                       const TplcS5ReadReq* req,
                       TplcS5IoHandle* out);
    int32_t (*s5_write)(void* svc, TplcS5ConnHandle ch,
                        const TplcS5WriteReq* req,
                        TplcS5IoHandle* out);
    /* drive an io transaction: info reports scalar state; the
       FETCH payload is copied into val as raw wire-order bytes
       (data_len bytes, up to val_cap). For writes val may be
       NULL. Always returns TPLC_S5_OK on a live handle;
       completed transactions are dropped. */
    int32_t (*s5_poll)(void* svc, TplcS5IoHandle h,
                       TplcS5RespInfo* info,
                       uint8_t* val, uint32_t val_cap);
    int32_t (*s5_abort)(void* svc, TplcS5IoHandle h);
    int32_t (*s5_close)(void* svc, TplcS5ConnHandle ch);
} TplcS5Service;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_S5_H */
