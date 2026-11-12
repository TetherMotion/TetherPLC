/* tplc_mpbus.h — C ABI for the tplc-mpbus host service.
 * Belimo MP-Bus master over an opt-in serial adapter (9600 8N1,
 * e.g. USB MP-Bus gateway presenting a tty).  Emits/parses
 * MP-Bus request/response frames
 *   [DA][CMD][LEN][data..][CHK]   CHK = XOR of all prior bytes
 * and captures the slave reply.  Opt-in: the port is only
 * opened when the surface is used.  Vtable is fixed — append
 * new fns at the end.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MPBUS_H
#define TPLC_MPBUS_H

#include <stdint.h>

#define TPLC_MPBUS_ABI_VERSION 1u
#define TPLC_MPBUS_HDL_MAX     8u

typedef struct TplcMpbusStatus {
    int32_t  state;
    uint32_t tx_total;
    uint32_t rx_total;
    int32_t  last_error;
    uint8_t  reserved[32];
} TplcMpbusStatus;

typedef struct TplcMpbusService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* open: serial path -> handle (9600 8N1) */
    int32_t (*mpbus_open)(void*, const char* path, uint32_t* out_handle);
    int32_t (*mpbus_close)(void*, uint32_t h);
    int32_t (*mpbus_status)(void*, uint32_t h, TplcMpbusStatus* out);

    /* transact: send a request frame [DA][CMD][LEN][data][CHK]
       then poll briefly for the slave's reply frame.  Returns
       >=0: reply payload length (reply DA/CMD in outs), or
       TPLC_MPBUS_E_NOMSG when no complete reply arrived within
       the poll window.  Bounded — a PLC scan is never held
       longer than the small internal deadline. */
    int32_t (*mpbus_transact)(void*, uint32_t h, uint32_t da,
                              uint32_t cmd, const uint8_t* data,
                              uint32_t data_len,
                              uint32_t* out_da, uint32_t* out_cmd,
                              uint8_t* resp, uint32_t resp_cap);

    /* receive: nonblocking poll for any buffered frame (e.g. an
       unsolicited slave message).  Same return contract as
       transact. */
    int32_t (*mpbus_receive)(void*, uint32_t h, uint32_t* out_da,
                             uint32_t* out_cmd, uint8_t* payload,
                             uint32_t payload_cap);

    void* reserved[8];
} TplcMpbusService;

#define TPLC_MPBUS_OK           0
#define TPLC_MPBUS_E_PARAM     -1
#define TPLC_MPBUS_E_HANDLE    -2
#define TPLC_MPBUS_E_OPEN      -3
#define TPLC_MPBUS_E_IO        -4
#define TPLC_MPBUS_E_NOSERIAL  -5
/* NOMSG is positive but outside the 0..250 payload range. */
#define TPLC_MPBUS_E_NOMSG      0x100

#endif /* TPLC_MPBUS_H */
