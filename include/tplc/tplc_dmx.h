/* tplc_dmx.h — C ABI for the tplc-dmx DMX512 output host service.
 * The PLC-side builtin FB_Dmx dispatches ops to this vtable via
 * getService("tplc-dmx"); the service owns an opt-in serial port and
 * emits real DMX512 frames (break + mark-after-break + start code +
 * 512 channel bytes at 250 kbaud).  Opt-in like the other special
 * features — a serial port is only opened when the surface is used.
 * All calls synchronous.  SPDX-License-Identifier: Apache-2.0 OR
 * MPL-2.0 */
#ifndef TPLC_DMX_H
#define TPLC_DMX_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_DMX_ABI_VERSION 1u
#define TPLC_DMX_HDL_MAX     8u
#define TPLC_DMX_CHANNELS    512u

/* ---- status struct (48 bytes, 8-byte aligned) ---- */
typedef struct TplcDmxStatus {
    int32_t  state;        /* 0 closed, 1 open, 2 streaming */
    uint32_t frames_sent;  /* complete DMX frames sent */
    int32_t  last_error;
    uint32_t reserved[9];  /* pad to 48 */
} TplcDmxStatus;

/* ---- service vtable ---- */
typedef struct TplcDmxService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* open: serial path (e.g. /dev/ttyUSB0) + optional baud (0 ->
       250k) -> handle */
    int32_t (*dmx_open)(void*, const char* path, uint32_t baud,
                        uint32_t* out_handle);
    int32_t (*dmx_close)(void*, uint32_t h);
    int32_t (*dmx_status)(void*, uint32_t h, TplcDmxStatus* out);

    /* set a single channel value (1..512, 0..255) */
    int32_t (*dmx_set_channel)(void*, uint32_t h, uint32_t channel,
                               uint32_t value);
    /* set a run of channels (start 1..512, count, values) */
    int32_t (*dmx_set_channels)(void*, uint32_t h, uint32_t start,
                                const uint8_t* values, uint32_t count);

    /* emit one complete DMX512 frame (break+MAB+startcode+512 ch).
       Nonblocking; bounded under the scan-stall watchdog. */
    int32_t (*dmx_send_frame)(void*, uint32_t h);

    void* reserved[8];
} TplcDmxService;

#define TPLC_DMX_OK           0
#define TPLC_DMX_E_PARAM     -1
#define TPLC_DMX_E_HANDLE    -2
#define TPLC_DMX_E_OPEN      -3
#define TPLC_DMX_E_IO        -4
#define TPLC_DMX_E_NOSERIAL  -5

#ifdef __cplusplus
}
#endif
#endif /* TPLC_DMX_H */
