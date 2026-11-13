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

/* Report control block attribute snapshot (GetRCBValues). */
typedef struct TplcIec61850Rcb {
    char     rpt_id[32];
    char     data_set[64];
    uint32_t conf_rev;
    uint32_t buf_tm;
    uint32_t intg_pd;
    uint32_t sq_num;
    uint32_t opt_flds;
    uint32_t trg_ops;
    int32_t  rpt_ena;
    int32_t  resv;
    int32_t  buffered;
    int32_t  gi;
    int32_t  purge_buf;
    uint32_t resv_tms;
    uint32_t entry_id;
    uint32_t reserved[1];
} TplcIec61850Rcb;           /* 152 bytes */

/* Control-object command-termination event (Select/Operate/
   CommandTermination+/-).  Drained by iec61850_control_poll. */
typedef struct TplcIec61850Ctl {
    uint32_t invoke_id;      /* MMS invoke id of the command        */
    int32_t  success;        /* 1 = CommandTermination+, 0 = -     */
    int32_t  err;            /* IedClientError / appl error code   */
    int32_t  type;           /* 0 select, 1 operate, 2 cancel      */
    uint32_t ctl_num;        /* ctlNum echoed by the server        */
    uint32_t pending;        /* 1 when an event was returned       */
    uint32_t reserved[2];
} TplcIec61850Ctl;           /* 32 bytes */

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
    /* MMS GetRCBValues on a report control block object reference
       ("domain/LN$BR$name" style). Fills out; returns TPLC_61850_OK
       or an error code. */
    int32_t (*iec61850_rcb_get)(void* svc, uint32_t handle,
                                const char* rcb_ref,
                                TplcIec61850Rcb* out);
    /* MMS SetRCBValues: flags bit0=rptEna, bit1=resv (URCB only),
       bit2=GI trigger.  Enabling also installs the report handler. */
    int32_t (*iec61850_rcb_set)(void* svc, uint32_t handle,
                                const char* rcb_ref, uint32_t flags);
    /* drain pending reports received since the last poll into out as
       newline-separated "dataset|reason|v0,v1,..." lines (cap).
       Returns byte count or a TPLC_61850_E_* code. */
    int32_t (*iec61850_report_poll)(void* svc, uint32_t handle,
                                    char* out, uint32_t cap);
    /* controllable data object (CDC with CO model): action 1=select,
       2=selectWithValue, 3=operate, 4=cancel.  ctl_val is the ASCII
       control value ("true"/"false", integer or float text) used by
       selectWithValue/operate; NULL otherwise.  out receives the
       ctlVal echo text.  Returns TPLC_61850_OK when the command was
       accepted for transmission. */
    int32_t (*iec61850_control)(void* svc, uint32_t handle,
                                const char* obj_ref, uint32_t action,
                                const char* ctl_val, char* out,
                                uint32_t cap);
    /* drain one pending CommandTermination event into out; pending=0
       when the queue is empty. */
    int32_t (*iec61850_control_poll)(void* svc, uint32_t handle,
                                     TplcIec61850Ctl* out);
    /* MMS ReadDataSet on a data set reference ("LD/LN.dsName" or
       "@dsName" for VMD scope): fills out with newline-separated
       "FCDA=value" lines (cap).  Returns element count or a
       TPLC_61850_E_* code. */
    int32_t (*iec61850_ds_read)(void* svc, uint32_t handle,
                                const char* ds_ref,
                                char* out, uint32_t cap);
    /* MMS GetDataSetDirectory: newline-separated FCDA object
       references of the data set members (cap).  Returns count or a
       TPLC_61850_E_* code. */
    int32_t (*iec61850_ds_directory)(void* svc, uint32_t handle,
                                     const char* ds_ref,
                                     char* out, uint32_t cap);
    /* MMS WriteDataSet: values is the same newline-separated text
       shape ds_read produces ("FCDA=value" lines or bare values in
       member order).  Member types are taken from a fresh
       ReadDataSet; returns TPLC_61850_OK when every member access
       result succeeded, TPLC_61850_E_MMS otherwise. */
    int32_t (*iec61850_ds_write)(void* svc, uint32_t handle,
                                 const char* ds_ref,
                                 const char* values, uint32_t len);
    /* MMS DefineNamedVariableList (ACSI CreateDataSet): elements is a
       newline-separated list of FCDA object references
       ("LD/LN.item[FC]" lines) defining the members.  ds_ref uses the
       "LD/LN.name" form or "@name" for an association-specific set.
       Returns TPLC_61850_OK or an error code. */
    int32_t (*iec61850_ds_create)(void* svc, uint32_t handle,
                                  const char* ds_ref,
                                  const char* elements, uint32_t len);
    /* MMS DeleteNamedVariableList (ACSI DeleteDataSet) on a deletable
       data set reference. */
    int32_t (*iec61850_ds_delete)(void* svc, uint32_t handle,
                                  const char* ds_ref);
} TplcIec61850Service;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_IEC61850_H */
