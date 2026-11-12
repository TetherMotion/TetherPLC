/* tplc_iec61850.h — TplcIec61850Service: IEC 61850 MMS client for the
 * TwinCAT Tc3_IEC61850 client blocks.  The PLC is the client
 * (control-center side) reading/writing data attributes on a remote
 * IED's Logical Device / Logical Node tree over the full OSI stack:
 * TPKT (RFC1006) + COTP + Session + Presentation + ACSE + MMS.
 *
 * Object addressing: a data attribute is addressed by a domain
 * (Logical Device) name + an item (VariableAccess) name — both are
 * MMS visible strings like "IED1CTRL" / "LLN0$Health$stVal".  The
 * GetNameList op lists a domain's objects (MMS GetNameList).
 * Reporting/control-blocks (RCB/URCB), GOOSE/SMV, and the full data
 * model are separate surfaces and deliberately out of scope.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_IEC61850_H
#define TPLC_IEC61850_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_IEC61850_ABI_VERSION 1u
#define TPLC_IEC61850_SVC_NAME "tplc-iec61850"
#define TPLC_IEC61850_HDL_MAX 8u

/* service return codes. */
#define TPLC_61850_OK        0
#define TPLC_61850_E_PARAM  -1
#define TPLC_61850_E_HANDLE -2   /* bad/closed connection handle        */
#define TPLC_61850_E_CONN   -3   /* connect/associate failure           */
#define TPLC_61850_E_MMS    -4   /* MMS service error / nonzero status  */
#define TPLC_61850_E_IO     -5   /* transport or codec error            */

typedef struct TplcIec61850Status {
    int32_t  state;          /* 0 closed, 1 associated, -1 error    */
    uint32_t invoke_id;      /* last MMS invoke id                  */
    uint32_t last_error;     /* last MMS service-error code         */
    uint32_t reserved[5];
} TplcIec61850Status;        /* 32 bytes                            */

typedef struct TplcIec61850Service {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open TCP to host:port and run the OSI associate handshake
       (TPKT/COTP/Session/Presentation/ACSE/MMS Initiate); writes a
       connection handle. */
    int32_t (*iec61850_connect)(void* svc, const char* host,
                                uint32_t port, uint32_t* handle);
    /* release the association and close the connection. */
    int32_t (*iec61850_close)(void* svc, uint32_t handle);
    /* association status. */
    int32_t (*iec61850_status)(void* svc, uint32_t handle,
                               TplcIec61850Status* out);
    /* MMS Read on a named variable: domain + item -> bounded value
       bytes into buf (cap). Returns value length or a TPLC_61850_E_*
       code. `domain`/`item` are MMS visible strings; NULL domain
       means VMD-specific. */
    int32_t (*iec61850_read)(void* svc, uint32_t handle,
                             const char* domain, const char* item,
                             uint8_t* buf, uint32_t cap);
    /* MMS Write on a named variable: domain + item + raw value bytes.
       (Structured values travel as their ASN.1 content.) */
    int32_t (*iec61850_write)(void* svc, uint32_t handle,
                              const char* domain, const char* item,
                              const uint8_t* buf, uint32_t len);
    /* MMS GetNameList on a domain: lists object names into out as a
       newline-separated visible string (cap). Returns count or a
       TPLC_61850_E_* code. */
    int32_t (*iec61850_getnamelist)(void* svc, uint32_t handle,
                                    const char* domain,
                                    char* out, uint32_t cap);
} TplcIec61850Service;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_IEC61850_H */
