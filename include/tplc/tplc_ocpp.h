/* tplc_ocpp.h — C ABI for the tplc-ocpp OCPP client host service.
 * The PLC-side builtin FB_Ocpp dispatches ops to this vtable via
 * getService("tplc-ocpp"); the service is a real OCPP-J client —
 * WebSocket (RFC6455) transport + OCPP JSON-RPC framing
 * ([2,msgId,action,payload] call, [3,msgId,payload] result,
 * [4,msgId,errcode,desc,details] error).  All calls synchronous.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_OCPP_H
#define TPLC_OCPP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_OCPP_ABI_VERSION 1u
#define TPLC_OCPP_HDL_MAX     16u

/* ---- status struct (48 bytes, 8-byte aligned) ---- */
typedef struct TplcOcppStatus {
    int32_t  state;        /* 0 closed, 1 ws-up, 2 session */
    uint32_t rx_total;     /* rx packets since connect */
    uint32_t tx_total;     /* tx packets */
    int32_t  last_error;   /* last callerror/ws error */
    uint32_t reserved[8];  /* pad to 48 */
} TplcOcppStatus;

/* ---- rx message struct ---- */
typedef struct TplcOcppRx {
    /* in: buffer capacity; out: actual sizes */
    uint32_t action_cap;
    uint32_t payload_cap;
    uint32_t action_len;   /* out */
    uint32_t payload_len;  /* out */
    uint32_t msg_type;     /* out: 2 call / 3 result / 4 callerror */
    uint32_t reserved;
    /* out buffers follow the struct */
    char*    action;
    uint8_t* payload;
} TplcOcppRx;

/* ---- service vtable ---- */
typedef struct TplcOcppService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* ws connect: host + port + path -> session handle */
    int32_t (*ocpp_connect)(void*, const char* host, uint32_t port,
                            const char* path, const char* ocpp_id,
                            uint32_t* out_handle);
    int32_t (*ocpp_disconnect)(void*, uint32_t h);
    int32_t (*ocpp_status)(void*, uint32_t h, TplcOcppStatus* out);

    /* send an OCPP call: [2, msgId, action, payload-json] */
    int32_t (*ocpp_call)(void*, uint32_t h, const char* msg_id,
                         const char* action, const uint8_t* payload,
                         uint32_t payload_len);

    /* send an OCPP result: [3, msgId, payload-json] */
    int32_t (*ocpp_result)(void*, uint32_t h, const char* msg_id,
                           const uint8_t* payload, uint32_t len);

    /* send an OCPP callerror: [4, msgId, code, desc, details-json] */
    int32_t (*ocpp_callerror)(void*, uint32_t h, const char* msg_id,
                              const char* code, const char* desc,
                              const uint8_t* details, uint32_t len);

    /* receive: nonblocking poll for a pending OCPP-J message.
       Returns 0 ok+nothing, TPLC_OCPP_E_NOMSG when empty, or fills
       rx on a message. */
    int32_t (*ocpp_receive)(void*, uint32_t h, TplcOcppRx* rx);

    void* reserved[8];
} TplcOcppService;

#define TPLC_OCPP_OK           0
#define TPLC_OCPP_E_PARAM     -1
#define TPLC_OCPP_E_HANDLE    -2
#define TPLC_OCPP_E_CONN      -3
#define TPLC_OCPP_E_PROTO     -4
#define TPLC_OCPP_E_TIMEOUT   -5
#define TPLC_OCPP_E_NOBUF     -6
#define TPLC_OCPP_E_NOMSG      1

#ifdef __cplusplus
}
#endif
#endif /* TPLC_OCPP_H */
