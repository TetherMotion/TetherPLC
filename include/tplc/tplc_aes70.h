/* tplc_aes70.h — C ABI for the tplc-aes70 OCA (AES70/Open Control
 * Architecture) client host service.  The PLC-side builtin FB_Aes70
 * dispatches ops to this vtable via getService("tplc-aes70"); the
 * service is a real OSCP+OCP client over TCP — sync header "OCP1" +
 * OcaMessage PDUs (OcaCommand/OcaResponse).  All calls synchronous.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_AES70_H
#define TPLC_AES70_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_AES70_ABI_VERSION 1u
#define TPLC_AES70_HDL_MAX     16u

/* ---- status struct (48 bytes, 8-byte aligned) ---- */
typedef struct TplcAes70Status {
    int32_t  state;        /* 0 closed, 1 connected */
    uint32_t rx_total;     /* rx OCP PDUs since connect */
    uint32_t tx_total;
    int32_t  last_error;
    uint32_t reserved[8];  /* pad to 48 */
} TplcAes70Status;

/* ---- service vtable ---- */
typedef struct TplcAes70Service {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* connect: host + port (OCP TCP, default 65000) -> handle */
    int32_t (*aes70_connect)(void*, const char* host, uint32_t port,
                             uint32_t* out_handle);
    int32_t (*aes70_disconnect)(void*, uint32_t h);
    int32_t (*aes70_status)(void*, uint32_t h, TplcAes70Status* out);

    /* OcaCommand on an object: targetONo (object number) +
     * methodID (defLevel<<8 + methodIndex) + parameter bytes.
     * returns the result handle (for status/response correlation)
     * or a negative error. */
    int32_t (*aes70_call)(void*, uint32_t h, uint32_t target_ono,
                          uint32_t method_id, const uint8_t* params,
                          uint32_t params_len, uint32_t* out_handle2);

    /* keepalive: OSCP keepalive PDU (heartbeat) */
    int32_t (*aes70_keepalive)(void*, uint32_t h);

    /* receive: nonblocking poll for a pending OcaResponse.
       Returns 0 ok+len>0, TPLC_AES70_E_NOMSG when empty; fills
       handle2 + result-code + the response parameter bytes. */
    int32_t (*aes70_receive)(void*, uint32_t h, uint32_t* out_handle2,
                             uint32_t* out_status_code,
                             uint8_t* payload, uint32_t payload_cap);

    void* reserved[8];
} TplcAes70Service;

#define TPLC_AES70_OK           0
#define TPLC_AES70_E_PARAM     -1
#define TPLC_AES70_E_HANDLE    -2
#define TPLC_AES70_E_CONN      -3
#define TPLC_AES70_E_PROTO     -4
#define TPLC_AES70_E_TIMEOUT   -5
#define TPLC_AES70_E_NOBUF     -6
#define TPLC_AES70_E_NOMSG      1

#ifdef __cplusplus
}
#endif
#endif /* TPLC_AES70_H */
