/* tplc_mbus.h — C ABI for the tplc-mbus wired M-Bus (EN 13757)
 * master host service.  The PLC-side builtin FB_Mbus dispatches ops
 * to this vtable via getService("tplc-mbus"); the service owns an
 * opt-in serial port and emits/receives real M-Bus frames (10 / 68 /
 * E5 framing at 2400 8E1).  Opt-in like the other bus surfaces — the
 * serial port is only opened when the surface is used.  All calls
 * synchronous.  SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MBUS_H
#define TPLC_MBUS_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_MBUS_ABI_VERSION 1u
#define TPLC_MBUS_HDL_MAX     8u

/* ---- status struct (48 bytes, 8-byte aligned) ---- */
typedef struct TplcMbusStatus {
    int32_t  state;        /* 0 closed, 1 open */
    uint32_t tx_total;     /* frames sent */
    uint32_t rx_total;     /* responses parsed */
    int32_t  last_error;
    uint32_t reserved[8];  /* pad to 48 */
} TplcMbusStatus;

/* ---- service vtable ---- */
typedef struct TplcMbusService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* open: serial path -> handle (2400 8E1 M-Bus) */
    int32_t (*mbus_open)(void*, const char* path, uint32_t* out_handle);
    int32_t (*mbus_close)(void*, uint32_t h);
    int32_t (*mbus_status)(void*, uint32_t h, TplcMbusStatus* out);

    /* short frame: 10 <C> <A> <CS> 16 — SND_NKE(0x40)/REQ_UD2(0x5B)/
     * REQ_UD1(0x7B)/SND_UD with a user control byte */
    int32_t (*mbus_short)(void*, uint32_t h, uint32_t control,
                          uint32_t address);

    /* long frame: 68 <L> <L> 68 <C> <A> <CI> [<data>] <CS> 16 */
    int32_t (*mbus_long)(void*, uint32_t h, uint32_t control,
                         uint32_t address, uint32_t ci,
                         const uint8_t* data, uint32_t len);

    /* receive: nonblocking poll for a response — E5 ack or a 68...
       long frame.  Returns 0 ok + kind (0xE5 ack / 0x68 long) +
       payload len, or TPLC_MBUS_E_NOMSG. */
    int32_t (*mbus_receive)(void*, uint32_t h, uint32_t* out_kind,
                            uint8_t* payload, uint32_t payload_cap);

    void* reserved[8];
} TplcMbusService;

#define TPLC_MBUS_OK           0
#define TPLC_MBUS_E_PARAM     -1
#define TPLC_MBUS_E_HANDLE    -2
#define TPLC_MBUS_E_OPEN      -3
#define TPLC_MBUS_E_IO        -4
#define TPLC_MBUS_E_NOSERIAL  -5
#define TPLC_MBUS_E_NOMSG      1

#ifdef __cplusplus
}
#endif
#endif /* TPLC_MBUS_H */
