/* tplc_canopen.h — TplcCanopenService: CANopen/CoE object-dictionary
 * access for PLC logic. Wraps the tether_io machine.sdo.* functions
 * (CiA 301 SDO upload/download by explicit index) against a running
 * EtherCAT master — CANopen's object dictionary carried over EtherCAT
 * CoE. Endpoint: TPLC_CANOPEN_IO, else TPLC_EC_IO, else
 * TPLC_MOTION_IO (same convention as tplc-ecdiag).
 * Synchronous request/reply per call (the IO session thread performs
 * the mailbox round-trip); not a realtime-cyclic operation.
 * Service ABI structs use native layout (little-endian hosts).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_CANOPEN_H
#define TPLC_CANOPEN_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_CANOPEN_ABI_VERSION 1u
#define TPLC_CANOPEN_SVC_NAME "tplc-canopen"
#define TPLC_CANOPEN_MAX_DATA 512u

/* service return codes */
#define TPLC_CO_OK        0   /* transfer completed                */
#define TPLC_CO_E_PARAM  -1
#define TPLC_CO_E_IO     -2
#define TPLC_CO_E_NOEP   -3   /* no tether_io endpoint configured  */
#define TPLC_CO_E_ABORT  -4   /* slave CoE abort; abort_code set   */
#define TPLC_CO_E_PROTO  -5   /* malformed reply                   */
#define TPLC_CO_E_STATE  -6   /* service/catalog unavailable      */

typedef struct TplcCanopenService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* TPLC_CO_OK when the tether_io endpoint is connected and the
       machine.sdo.* functions were discovered, else an error code. */
    int32_t (*co_state)(void* svc);
    /* CiA 301 SDO upload: read `cap` bytes of the object at
       slave/index/subindex into out; out_len reports the byte
       count. abort_code receives the CoE abort code (0 on success). */
    int32_t (*co_sdo_read)(void* svc, uint16_t slave, uint16_t index,
                           uint8_t subindex, uint8_t* out,
                           uint32_t cap, uint32_t* out_len,
                           uint32_t* abort_code);
    /* CiA 301 SDO download: write len bytes of data to
       slave/index/subindex. verify!=0 requests the server-side
       post-write readback check. abort_code receives the CoE abort
       code (0 on success). */
    int32_t (*co_sdo_write)(void* svc, uint16_t slave, uint16_t index,
                            uint8_t subindex, const uint8_t* data,
                            uint32_t len, uint8_t verify,
                            uint32_t* abort_code);
} TplcCanopenService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_CANOPEN_H */
