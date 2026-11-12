/* tplc_enocean.h — C ABI for the tplc-enocean host service:
 * EnOcean serial protocol 3 (ESP3) over an opt-in UART (57600
 * 8N1, e.g. TCM 310 / USB 300 transceiver).  The PLC-side
 * builtin FB_EnOcean dispatches ops to this vtable via
 * getService("tplc-enocean").  All calls synchronous and
 * bounded.  SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_ENOCEAN_H
#define TPLC_ENOCEAN_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_ENOCEAN_ABI_VERSION 1u
#define TPLC_ENOCEAN_HDL_MAX     8u

/* ESP3 packet types (subset — ENOCEAN_SPEC §1.6) */
#define TPLC_ENOCEAN_PT_RADIO       1u   /* ERP1 radio telegram  */
#define TPLC_ENOCEAN_PT_RESPONSE    2u   /* command response     */
#define TPLC_ENOCEAN_PT_EVENT       4u
#define TPLC_ENOCEAN_PT_COMMAND     5u

/* ---- status struct (48 bytes, 8-byte aligned) ---- */
typedef struct TplcEnoceanStatus {
    int32_t  state;        /* 0 closed, 1 open */
    uint32_t tx_total;     /* frames sent */
    uint32_t rx_total;     /* telegrams parsed */
    int32_t  last_error;
    uint32_t reserved[8];
} TplcEnoceanStatus;

/* ---- service vtable ---- */
typedef struct TplcEnoceanService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* open: serial path -> handle (57600 8N1 ESP3) */
    int32_t (*enocean_open)(void*, const char* path,
                            uint32_t* out_handle);
    int32_t (*enocean_close)(void*, uint32_t h);
    int32_t (*enocean_status)(void*, uint32_t h,
                              TplcEnoceanStatus* out);

    /* send_radio: build + emit a RADIO_ERP1 telegram.
     * org=RORG, sender=own 32-bit id, dest=telegram destination
     * (0xFFFFFFFF broadcast), data=payload bytes. */
    int32_t (*enocean_send_radio)(void*, uint32_t h, uint32_t org,
                                  uint32_t sender, uint32_t dest,
                                  const uint8_t* data, uint32_t len);

    /* receive: nonblocking poll — parses the ESP3 stream for a
     * complete packet.  Returns TPLC_ENOCEAN_E_NOMSG, or 0..255
     * (payload length; out of NOMSG range):
     * out_ptype = packet type, out_org = RORG (radio only),
     * out_sender = sender id (radio only), payload = user data
     * (radio payload minus org/sender/dest/status). */
    int32_t (*enocean_receive)(void*, uint32_t h, uint32_t* out_ptype,
                               uint32_t* out_org, uint32_t* out_sender,
                               uint8_t* payload, uint32_t payload_cap);

    void* reserved[8];
} TplcEnoceanService;

#define TPLC_ENOCEAN_OK           0
#define TPLC_ENOCEAN_E_PARAM     -1
#define TPLC_ENOCEAN_E_HANDLE    -2
#define TPLC_ENOCEAN_E_OPEN      -3
#define TPLC_ENOCEAN_E_IO        -4
#define TPLC_ENOCEAN_E_NOSERIAL  -5
/* TPLC_ENOCEAN_E_NOMSG is positive but outside the 0..255
   payload-length range so a 1-byte payload cannot collide. */
#define TPLC_ENOCEAN_E_NOMSG      0x100

#ifdef __cplusplus
}
#endif
#endif /* TPLC_ENOCEAN_H */
