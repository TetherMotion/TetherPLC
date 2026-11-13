/* tplc_ecdiag.h — C ABI for the tplc-ecdiag EtherCAT CoE diag-history
 * host service.  Reads the ec.coediag.* params exposed by a Tether IO
 * endpoint (tether_io_motion_server sim contract; a real master would
 * bridge per-slave SDO reads of CoE object 0x10F3 under the same
 * names).  Endpoint from TPLC_EC_IO, falling back to TPLC_MOTION_IO.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_ECDIAG_H
#define TPLC_ECDIAG_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_ECDIAG_ABI_VERSION 1u
#define TPLC_ECDIAG_MAX_ENTRIES 16u
#define TPLC_ECDIAG_MSG_TEXT 64u

/* decoded diagnosis-history entry (one per 22-byte ETG.1020 record) */
typedef struct TplcEcDiagEntry {
    uint64_t timestamp_ns;      /* slave timestamp, ns */
    uint32_t msg_type;          /* 0 unknown, 1 info, 2 warning, 3 error */
    uint32_t ack_status;        /* 1 = acknowledged */
    uint32_t diag_code;         /* record diag code */
    uint32_t diag_code_type;    /* see TPLC_ECDIAG_CODETYPE_* */
    uint32_t text_id;           /* ESI/XML message-text id */
    uint32_t module_no;         /* params word 0, 1-based */
    uint32_t channel_no;        /* params word 1, 1-based */
    char     msg_text[TPLC_ECDIAG_MSG_TEXT]; /* NUL-terminated */
} TplcEcDiagEntry;

/* object-level info from the 0x10F3 header */
typedef struct TplcEcDiagInfo {
    uint32_t newest;            /* newest message number (1-based) */
    uint32_t newest_acked;      /* newest acknowledged message */
    uint32_t new_msgs;          /* new messages available flag */
    uint32_t flags;             /* object flags */
    uint32_t reserved[3];
} TplcEcDiagInfo;

/* diag_code_type values (approximation of the ETG typing; module/
 * profile-specific refinement needs device profile data) */
#define TPLC_ECDIAG_CODETYPE_UNKNOWN        0u
#define TPLC_ECDIAG_CODETYPE_NOTUSED        1u
#define TPLC_ECDIAG_CODETYPE_MANUFACTURER   2u
#define TPLC_ECDIAG_CODETYPE_EMERGENCY      3u
#define TPLC_ECDIAG_CODETYPE_RESERVED       4u
#define TPLC_ECDIAG_CODETYPE_MODULE         5u
#define TPLC_ECDIAG_CODETYPE_DEVICE         6u
#define TPLC_ECDIAG_CODETYPE_PROFILE        7u

/* ---- service vtable ---- */
typedef struct TplcEcDiagService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* read + decode the diag history for slave_addr.
     * entries receives up to max_entries decoded records (wire order,
     * oldest first); *count_out gets the number written (<= max).
     * info receives the 0x10F3 header fields. */
    int32_t (*diag_read)(void*, uint32_t slave_addr,
                         TplcEcDiagEntry* entries, uint32_t max_entries,
                         uint32_t* count_out, TplcEcDiagInfo* info);

    /* write newest_acked (0..16) for slave_addr */
    int32_t (*diag_ack)(void*, uint32_t slave_addr,
                        uint32_t newest_acked);

    void* reserved[6];
} TplcEcDiagService;

/* status codes */
#define TPLC_ECDIAG_OK         0
#define TPLC_ECDIAG_E_PARAM   -1
#define TPLC_ECDIAG_E_NOEP    -2   /* no endpoint configured/reachable */
#define TPLC_ECDIAG_E_PARAM_MISS -3 /* ec.coediag.* params absent */
#define TPLC_ECDIAG_E_IO      -4

#ifdef __cplusplus
}
#endif
#endif /* TPLC_ECDIAG_H */
