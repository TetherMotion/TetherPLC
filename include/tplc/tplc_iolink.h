/* tplc_iolink.h — TplcIoLinkService: IO-Link master data-link layer
 * for PLC logic, implemented natively in TetherPLC (IEC 61131-9 /
 * SDCI wire protocol over a byte transport — TCP endpoint per port;
 * a UART/PHY adapter is the hardware path for real devices).
 *
 * Ports are configured at service start from TPLC_IOLINK_IO (comma
 * list of tcp://host:port endpoints, one per IO-Link port; port index
 * selects the endpoint). Each port runs the IO-Link M-sequence
 * exchange: wakeup, identification, then cyclic process-data
 * frames. PHY note: real IO-Link devices need 24 V-class signaling —
 * deploy behind a PHY-equipped master adapter; this stack speaks the
 * data-link/application protocol.
 *
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_IOLINK_H
#define TPLC_IOLINK_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_IOLINK_ABI_VERSION 1u
#define TPLC_IOLINK_SVC_NAME "tplc-iolink"
#define TPLC_IOLINK_MAX_PORTS 4u
#define TPLC_IOLINK_MAX_PD 32u     /* max process-data octets per direction */
#define TPLC_IOLINK_MAX_OD 2u      /* on-request octets per M-sequence */

/* service return codes */
#define TPLC_IL_OK        0
#define TPLC_IL_E_PARAM  -1
#define TPLC_IL_E_IO     -2   /* transport error                              */
#define TPLC_IL_E_NOEP   -3   /* port not configured / not open               */
#define TPLC_IL_E_STATE  -4   /* illegal state for the requested op           */
#define TPLC_IL_E_CKS    -5   /* device reply checksum mismatch               */
#define TPLC_IL_E_PROTO  -6   /* malformed device reply                       */
#define TPLC_IL_E_DEV    -7   /* device reported an error (event/abort)       */

/* port states (IL_STATE output) */
#define TPLC_IL_ST_CLOSED     0
#define TPLC_IL_ST_OPEN       1   /* transport up, pre-wakeup                   */
#define TPLC_IL_ST_IDENT      2   /* identification exchange done               */
#define TPLC_IL_ST_OPERATE    3   /* cyclic process-data exchange active        */
#define TPLC_IL_ST_ERROR      4

/* M-sequence types supported by the wire codec (Annex A) */
#define TPLC_IL_MSEQ_T0  0   /* on-request data only (no process data)       */
#define TPLC_IL_MSEQ_T12 2   /* 2 OD octets, no process data                 */
#define TPLC_IL_MSEQ_T22 5   /* 2 PD-in octets + 1 OD octet                  */
#define TPLC_IL_MSEQ_T2V 6   /* 0..32 PD in/out + 1/2 OD octets              */

typedef struct TplcIoLinkService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* number of configured endpoints (0 when TPLC_IOLINK_IO unset) */
    int32_t (*il_state)(void* svc);
    /* open port p: connect transport, send wakeup, attempt IDENT.
       Returns IL_OK when the port reaches IDENT (or better). */
    int32_t (*il_open)(void* svc, uint32_t port);
    int32_t (*il_close)(void* svc, uint32_t port);
    /* read the device identification (DPP1) into out[16]:
       [0]=master-cmd page, [1..2]=min-cycle-time BE-ish raw,
       [3]=mseq-cap, [4]=revision, [5]=pd-in desc, [6]=pd-out desc,
       [7..8]=vendor LE, [9..11]=device LE. out_len receives 12. */
    int32_t (*il_ident)(void* svc, uint32_t port, uint8_t* out,
                        uint32_t cap, uint32_t* out_len);
    /* one cyclic process-data exchange. mseq selects the frame type;
       pdi/pdo carry up to 32 octets each direction. On return
       pdi_len/pdo_len report actual octet counts and cks_flags the
       device CKS flag bits (0x80 event pending, 0x40 PD invalid). */
    int32_t (*il_pd_exchange)(void* svc, uint32_t port, uint32_t mseq,
                              const uint8_t* pdo, uint32_t pdo_len,
                              uint8_t* pdi, uint32_t pdi_cap,
                              uint32_t* pdi_len, uint32_t* cks_flags);
    /* COM2-style acyclic ISDU/parameter read: addr + od request bytes
       -> od response bytes (single M-sequence exchange). */
    int32_t (*il_isdu_read)(void* svc, uint32_t port, uint8_t addr,
                            const uint8_t* od, uint32_t od_len,
                            uint8_t* resp, uint32_t resp_cap,
                            uint32_t* resp_len);
    int32_t (*il_isdu_write)(void* svc, uint32_t port, uint8_t addr,
                             const uint8_t* od, uint32_t od_len);
    /* port diagnostics: state, error counters, last cks flags */
    int32_t (*il_port_info)(void* svc, uint32_t port,
                            uint32_t* state, uint32_t* errors,
                            uint32_t* frames);
} TplcIoLinkService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_IOLINK_H */
