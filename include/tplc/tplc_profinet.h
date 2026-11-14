/* tplc_profinet.h — TplcProfinetService: PROFINET DCP discovery/
 * configuration + cyclic IO record exchange for PLC logic,
 * implemented natively in TetherPLC.
 *
 * DCP (Discovery and Configuration Protocol) is carried in
 * PROFINET Ethernet frames (Ethertype 0x8892); this stack speaks
 * the DCP PDU — 12-byte header + option blocks — over per-port UDP
 * byte transports configured via TPLC_PROFINET_IO (comma-separated
 * host:port endpoints). On real hardware a Layer-2 PROFINET
 * transport (raw Ethertype 0x8892) is the deployment path; UDP is
 * the test/tunnel transport, matching the IO-Link TCP approach.
 *
 * Cyclic IO (RT/IRT class, IOCR/AR framing) is staged; the service
 * provides a provider/consumer-style record exchange primitive
 * that AR mapping builds on.
 *
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_PROFINET_H
#define TPLC_PROFINET_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_PN_ABI_VERSION 1u
#define TPLC_PN_SVC_NAME "tplc-profinet"
#define TPLC_PN_MAX_PORTS 4u
#define TPLC_PN_MAX_IO 64u      /* cyclic record octets per direction */
#define TPLC_PN_MAX_DCP 256u    /* DCP payload buffer */

/* service return codes */
#define TPLC_PN_OK        0
#define TPLC_PN_E_PARAM  -1
#define TPLC_PN_E_IO     -2
#define TPLC_PN_E_NOEP   -3
#define TPLC_PN_E_STATE  -4
#define TPLC_PN_E_PROTO  -5   /* malformed DCP reply                     */
#define TPLC_PN_E_DEV    -7   /* device block error status              */

/* port states */
#define TPLC_PN_ST_CLOSED   0
#define TPLC_PN_ST_OPEN     1
#define TPLC_PN_IDENTIFIED 2

/* DCP constants (scapy pnio_dcp / PROFInet spec) */
#define TPLC_PN_DCP_OPT_IP       0x01
#define TPLC_PN_DCP_OPT_PROP     0x02
#define TPLC_PN_DCP_OPT_DHCP     0x03
#define TPLC_PN_DCP_OPT_CTRL     0x05
#define TPLC_PN_DCP_OPT_ALL      0xFF
#define TPLC_PN_DCP_SUB_MAC      0x01   /* under OPT_IP                  */
#define TPLC_PN_DCP_SUB_IPPARM   0x02
#define TPLC_PN_DCP_SUB_FULLIP   0x03
#define TPLC_PN_DCP_SUB_VENDOR   0x01   /* under OPT_PROP                */
#define TPLC_PN_DCP_SUB_NAME     0x02
#define TPLC_PN_DCP_SUB_DEVID    0x03
#define TPLC_PN_DCP_SUB_ROLE     0x04
#define TPLC_PN_DCP_SUB_OPTIONS  0x05
#define TPLC_PN_DCP_SUB_ALIAS    0x06
#define TPLC_PN_DCP_SUB_INSTANCE 0x07
#define TPLC_PN_DCP_SUB_OEM      0x08

typedef struct TplcProfinetService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    int32_t (*pn_state)(void* svc);
    int32_t (*pn_open)(void* svc, uint32_t port);
    int32_t (*pn_close)(void* svc, uint32_t port);
    /* DCP Identify (all-selector): fills the identification record —
       name (NUL-terminated, cap), vendor/device/role/instance/oem
       scalars, and the raw block payload into raw (cap). */
    int32_t (*pn_identify)(void* svc, uint32_t port, uint32_t xid,
                           char* name, uint32_t name_cap,
                           uint32_t* vendor, uint32_t* device,
                           uint32_t* role, uint32_t* instance,
                           uint32_t* oem,
                           uint8_t* raw, uint32_t raw_cap,
                           uint32_t* raw_len);
    /* DCP Get: option/suboption -> data bytes (OD/DLEN) */
    int32_t (*pn_dcp_get)(void* svc, uint32_t port, uint32_t xid,
                          uint8_t option, uint8_t suboption,
                          uint8_t* data, uint32_t cap,
                          uint32_t* out_len);
    /* DCP Set: option/suboption + data -> device confirm */
    int32_t (*pn_dcp_set)(void* svc, uint32_t port, uint32_t xid,
                          uint8_t option, uint8_t suboption,
                          const uint8_t* data, uint32_t len);
    /* cyclic record exchange: send out_rec, receive in_rec
       (provider/consumer primitive; AR/IOCR framing staged) */
    int32_t (*pn_io_exchange)(void* svc, uint32_t port,
                              const uint8_t* out_rec, uint32_t out_len,
                              uint8_t* in_rec, uint32_t in_cap,
                              uint32_t* in_len);
    /* IOCR cyclic exchange (IEC 61158-6-10 APDU over the port
       transport): DataRequest/DataResponse PDU =
       [FrameID u16][C-SDU >=40B][cycle u16][dataStatus u8]
       [transferStatus u8]. The C-SDU carries a single
       provider/consumer record pair in the 'PN' tag format; the
       service keeps a per-port cycle counter (increments each
       exchange, wraps u16) and validates the reply FrameID,
       minimum C-SDU length, and record tag. frame_id 0 keeps the
       port's configured id (default 0x8000). On return cyc_out
       receives the device's cycle counter and stat_out its
       DataStatus byte (0x35 = primary/RUN/data-valid/station-ok). */
    int32_t (*pn_io_cr)(void* svc, uint32_t port, uint32_t frame_id,
                        const uint8_t* out_rec, uint32_t out_len,
                        uint8_t* in_rec, uint32_t in_cap,
                        uint32_t* in_len, uint32_t* cyc_out,
                        uint32_t* stat_out);
    int32_t (*pn_port_info)(void* svc, uint32_t port,
                            uint32_t* state, uint32_t* errors,
                            uint32_t* frames);
} TplcProfinetService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_PROFINET_H */
