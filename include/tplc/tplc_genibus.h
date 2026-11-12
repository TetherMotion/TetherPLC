/* tplc_genibus.h — C ABI for the tplc-genibus host service.
 * Grunfos GENIbus (RS485 serial, 9600 8E1): emits/parses real
 * GENIbus telegrams   SD(0x27) LE LEr DA SA APDU.. CRC16hi lo.
 * Opt-in: the serial port is only opened when the surface is
 * used.  Vtable layout is fixed — append new fns at the end.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_GENIBUS_H
#define TPLC_GENIBUS_H

#include <stdint.h>

#define TPLC_GENIBUS_ABI_VERSION 1u
#define TPLC_GENIBUS_HDL_MAX     8u

typedef struct TplcGenibusStatus {
    int32_t  state;       /* 0 closed, 1 open */
    uint32_t tx_total;    /* telegrams transmitted */
    uint32_t rx_total;    /* telegrams decoded */
    int32_t  last_error;  /* last errno or E_* */
    uint8_t  reserved[32];
} TplcGenibusStatus;

typedef struct TplcGenibusService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* open: serial path -> handle (9600 8E1 GENIbus) */
    int32_t (*genibus_open)(void*, const char* path, uint32_t* out_handle);
    int32_t (*genibus_close)(void*, uint32_t h);
    int32_t (*genibus_status)(void*, uint32_t h, TplcGenibusStatus* out);

    /* send: emit a GENIbus telegram to dest addr `da` from `sa`
       with a raw APDU (may be len 0 — a ping).  CRC16 appended. */
    int32_t (*genibus_send)(void*, uint32_t h, uint32_t da,
                            uint32_t sa, const uint8_t* apdu,
                            uint32_t apdu_len);

    /* receive: nonblocking poll — parses the byte stream for a
       complete telegram.  Returns TPLC_GENIBUS_E_NOMSG, or 0..250
       (APDU length; out of NOMSG range): out_da/out_sa carry the
       telegram header, payload the raw APDU. */
    int32_t (*genibus_receive)(void*, uint32_t h, uint32_t* out_da,
                               uint32_t* out_sa, uint8_t* payload,
                               uint32_t payload_cap);

    void* reserved[8];
} TplcGenibusService;

#define TPLC_GENIBUS_OK           0
#define TPLC_GENIBUS_E_PARAM     -1
#define TPLC_GENIBUS_E_HANDLE    -2
#define TPLC_GENIBUS_E_OPEN      -3
#define TPLC_GENIBUS_E_IO        -4
#define TPLC_GENIBUS_E_NOSERIAL  -5
/* NOMSG is positive but outside the 0..250 APDU-length range so a
   short telegram cannot collide. */
#define TPLC_GENIBUS_E_NOMSG      0x100

#endif /* TPLC_GENIBUS_H */
