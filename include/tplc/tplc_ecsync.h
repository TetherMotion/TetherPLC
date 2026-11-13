/* tplc_ecsync.h — C ABI for the tplc-ecsync EtherCAT DC sync host
 * service.  Reads the ec.dc.* params exposed by a Tether IO endpoint
 * (tether_io_motion_server, or a real master bridging DCManager stats
 * under the same names).  Endpoint from TPLC_EC_IO, falling back to
 * TPLC_MOTION_IO.  SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_ECSYNC_H
#define TPLC_ECSYNC_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_ECSYNC_ABI_VERSION 1u

/* ---- status struct (32 bytes, 8-byte aligned) ---- */
typedef struct TplcEcsyncStatus {
    int32_t  connected;      /* master link reported up */
    int32_t  sync_enabled;   /* DC sync enable flag */
    int32_t  params_ok;      /* all four ec.dc.* params resolved */
    int32_t  last_error;     /* last service/io error code */
    double   time_diff_ns;   /* internal-vs-external clock diff, ns */
    uint64_t ref_time_ns;    /* external reference clock, ns */
} TplcEcsyncStatus;

/* ---- service vtable ---- */
typedef struct TplcEcsyncService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* poll all ec.dc.* params into out (fresh IO each call) */
    int32_t (*ec_status)(void*, TplcEcsyncStatus* out);

    /* write ec.dc.sync_enabled (0/1) */
    int32_t (*ec_set_sync)(void*, uint32_t enable);

    void* reserved[6];
} TplcEcsyncService;

/* status codes */
#define TPLC_ECSYNC_OK        0
#define TPLC_ECSYNC_E_PARAM  -1
#define TPLC_ECSYNC_E_NOEP   -2   /* no endpoint configured/reachable */
#define TPLC_ECSYNC_E_PARAM_MISS -3 /* ec.dc.* params absent */
#define TPLC_ECSYNC_E_IO     -4

#ifdef __cplusplus
}
#endif
#endif /* TPLC_ECSYNC_H */
