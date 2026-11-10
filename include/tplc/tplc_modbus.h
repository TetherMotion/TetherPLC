/* tplc_modbus.h — TplcModbusService: nonblocking Modbus TCP client.
 * One transaction per begin/poll cycle; the service owns the socket
 * and protocol state, the caller owns the payload buffers.
 * Wire format: MBAP header (tid u16be, proto u16be=0, length u16be,
 * unit u8) + PDU (func u8 + data), all multi-byte fields big-endian.
 * Service ABI structs use native layout (little-endian hosts).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MODBUS_H
#define TPLC_MODBUS_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_MODBUS_ABI_VERSION 1u
#define TPLC_MODBUS_SVC_NAME "tplc-modbus"
#define TPLC_MODBUS_HANDLE_INVALID 0u
#define TPLC_MODBUS_MAX_PDU 253u /* max PDU bytes after the func code */

/* service return codes (poll/begin) */
#define TPLC_MB_OK        0   /* transaction complete, resp filled   */
#define TPLC_MB_BUSY      1   /* still in progress, poll again       */
#define TPLC_MB_E_PARAM  -1
#define TPLC_MB_E_IO     -2
#define TPLC_MB_E_CONNECT -3
#define TPLC_MB_E_TIMEOUT -4
#define TPLC_MB_E_CLOSED -5
#define TPLC_MB_E_PROTO  -6   /* malformed/ mismatched response       */
#define TPLC_MB_E_STATE  -7   /* bad handle/state                    */
#define TPLC_MB_E_RANGE  -8   /* request too large                   */

/* request descriptor; req bytes are copied at mb_begin, so the
   caller buffer only needs to stay valid for that call. */
typedef struct TplcModbusReq {
    char     host[16];        /* IPv4 dotted quad, NUL-terminated  */
    uint16_t port;            /* TCP port (502 standard)           */
    uint16_t timeout_ms;      /* 0 = no deadline                   */
    uint8_t  unit_id;         /* Modbus unit id (0xFF direct TCP)  */
    uint8_t  func;            /* Modbus function code              */
    uint16_t req_len;         /* PDU data bytes after func, <=253  */
    const uint8_t* req;       /* request PDU data (may be NULL)    */
} TplcModbusReq;

/* response descriptor; data/cap describe the caller's receive
   buffer, the remaining fields are outputs written on TPLC_MB_OK. */
typedef struct TplcModbusResp {
    uint8_t* data;            /* caller buffer for response data   */
    uint16_t cap;             /* capacity of data                  */
    uint16_t len;             /* out: response PDU data length     */
    uint8_t  status;          /* out: 0 = normal, 1 = exception    */
    uint8_t  exc;             /* out: Modbus exception code        */
} TplcModbusResp;

typedef uint32_t TplcModbusHandle;

typedef struct TplcModbusService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* begin a transaction: validates req, opens a nonblocking
       connection, returns a handle. Does not block. */
    int32_t (*mb_begin)(void* svc, const TplcModbusReq* req,
                        TplcModbusHandle* out);
    /* drive the transaction one step; TPLC_MB_BUSY while pending,
       TPLC_MB_OK with resp filled on completion, negative on error.
       Terminal results free the handle automatically. */
    int32_t (*mb_poll)(void* svc, TplcModbusHandle h,
                       TplcModbusResp* resp);
    /* cancel a pending transaction and free the handle. */
    int32_t (*mb_abort)(void* svc, TplcModbusHandle h);
} TplcModbusService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_MODBUS_H */
