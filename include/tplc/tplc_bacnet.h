/* tplc_bacnet.h — TplcBacnetService: nonblocking BACnet/IP client
 * (ASHRAE 135) over UDP port 47808.
 *
 * Wire format: BVLC header (type 0x81, function 0x0A unicast /
 * 0x0B broadcast, u16 length incl. header), then NPDU (version 1,
 * control octet: bit7 net-layer msg, bit5 DNET present, bit3 SNET
 * present, bit2 expecting-reply, bits0-1 network priority), then
 * the NSDU/APDU. Confirmed services use APDU type 0 (Confirmed-
 * Request) and expect Simple-ACK (2), Complex-ACK (3), Error (5),
 * Reject (6), or Abort (7); unconfirmed services use type 1.
 *
 * Supported services: Who-Is (8) / I-Am (0) discovery and the
 * confirmed ReadProperty (12) / WriteProperty (15) pair. Values
 * cross the ABI as raw application-tag payload bytes: `val_type`
 * selects the BACnet application tag (0=null, 1=boolean, 2=
 * unsigned, 3=integer, 4=real, 5=double, 6=octetString, 7=
 * characterString, 9=enumerated, 12=objectIdentifier) and `val`
 * holds the tag payload verbatim in wire byte order (e.g. REAL is
 * 4-byte big-endian IEEE-754, OBJECT IDENTIFIER is
 * (type<<22)|instance as u32 BE, UNSIGNED is minimal big-endian).
 * The service wraps/unwraps the tag framing; it does not interpret
 * payloads.
 *
 * One transaction per handle, mirroring tplc-modbus: bac_begin
 * encodes and sends the request, bac_poll drives the nonblocking
 * receive and reports progress in resp->status. All calls are
 * nonblocking; the service clock is injected by the host so
 * --sim stays deterministic.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_BACNET_H
#define TPLC_BACNET_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_BACNET_ABI_VERSION 1u
#define TPLC_BACNET_SVC_NAME "tplc-bacnet"
#define TPLC_BACNET_HANDLE_INVALID 0u
#define TPLC_BACNET_PORT 47808u
#define TPLC_BACNET_MAX_VAL 256u
#define TPLC_BACNET_MAX_RESPONDERS 8u

/* service functions */
#define TPLC_BACNET_F_WHO_IS      1u /* unconfirmed Who-Is          */
#define TPLC_BACNET_F_READ_PROP   2u /* confirmed ReadProperty      */
#define TPLC_BACNET_F_WRITE_PROP  3u /* confirmed WriteProperty     */

/* request flags */
#define TPLC_BACNET_F_HAS_IDX     0x01u /* prop_idx carries an array index */
#define TPLC_BACNET_F_HAS_PRIO    0x02u /* aux0 carries write priority    */
#define TPLC_BACNET_F_HAS_RANGE   0x04u /* aux0/aux1 = who-is range      */

/* poll status (resp->status) */
#define TPLC_BACNET_ST_PENDING   0 /* still waiting for a response  */
#define TPLC_BACNET_ST_OK        1 /* request satisfied             */
#define TPLC_BACNET_ST_TIMEOUT   2 /* no response before timeout    */
#define TPLC_BACNET_ST_ERROR     3 /* BACnet Error/Reject/Abort APDU */
#define TPLC_BACNET_ST_BADRESP   4 /* malformed/unrelated datagram  */

/* service return codes (bac_begin/bac_poll/bac_abort) */
#define TPLC_BACNET_OK        0
#define TPLC_BACNET_E_PARAM  -1
#define TPLC_BACNET_E_IO     -2
#define TPLC_BACNET_E_STATE  -3   /* bad handle                          */
#define TPLC_BACNET_E_RANGE  -4   /* request out of range                */

typedef struct TplcBacnetPeer {
    char     host[16];      /* dotted-quad IPv4 or "255.255.255.255"
                               for broadcast Who-Is; "" = broadcast */
    uint16_t port;          /* 0 = TPLC_BACNET_PORT                 */
    uint16_t reserved;
} TplcBacnetPeer;

/* one request; all fields fixed at bac_begin. For WHO_IS the
   object/property fields are unused; aux0/aux1 carry the optional
   device-instance range when F_HAS_RANGE is set (0xFFFFFFFF =
   absent bound is not needed — the flag gates both bounds).
   For WRITE_PROP val_type/val describe the propertyValue; aux0
   is the optional priority when F_HAS_PRIO is set. */
typedef struct TplcBacnetReq {
    uint32_t func;                          /* TPLC_BACNET_F_*     */
    TplcBacnetPeer peer;
    uint32_t obj_type;      /* BACnet object type, e.g. device=8,
                               analogInput=0, analogValue=2,
                               binaryValue=5                          */
    uint32_t obj_inst;      /* object instance 0..4194303            */
    uint32_t prop_id;       /* property identifier, e.g. 85 =
                               present-value, 77 = object-name        */
    uint32_t prop_idx;      /* array index when F_HAS_IDX            */
    uint32_t val_type;      /* app tag for WRITE_PROP value          */
    uint32_t flags;         /* TPLC_BACNET_F_HAS_*                   */
    uint32_t aux0;          /* write priority / who-is range low     */
    uint32_t aux1;          /* who-is range high                     */
    uint16_t val_len;       /* payload bytes in val[], <=256         */
    uint16_t timeout_ms;    /* 0 = 1000 ms default                   */
    uint8_t  val[TPLC_BACNET_MAX_VAL];
} TplcBacnetReq;

/* poll output: scalar state only (fixed 64-byte layout). The
   value payload and who-is responder table are copied into caller
   buffers by bac_poll — see its signature. For READ_PROP the
   Complex-ACK value lands in val_type/val; for WHO_IS each I-Am
   fills dev_id[i] = (type<<22)|instance and dev_ip[i*4..i*4+3]
   the responder IPv4 address. */
typedef struct TplcBacnetRespInfo {
    int32_t  status;        /* TPLC_BACNET_ST_*                      */
    uint32_t service;       /* echoed service choice / apdu detail   */
    uint32_t apdu_type;     /* last APDU type seen (0..15)           */
    uint32_t obj_type;      /* read-ack object type                  */
    uint32_t obj_inst;
    uint32_t prop_id;
    uint32_t val_type;      /* app tag number of returned value      */
    uint32_t val_len;       /* bytes available in the val buffer     */
    uint32_t dev_count;     /* who-is: responders collected          */
    uint32_t dev_vendor;    /* last responder's vendor id            */
    uint32_t dev_max_apdu;
    uint32_t dev_seg;
    uint32_t reserved[4];
} TplcBacnetRespInfo;         /* 64 bytes                            */

typedef uint32_t TplcBacnetHandle;

typedef struct TplcBacnetService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open a transaction: encode + queue the request datagram.
       Returns TPLC_BACNET_OK and a live handle, or a negative
       code for bad parameters. */
    int32_t (*bac_begin)(void* svc, const TplcBacnetReq* req,
                         TplcBacnetHandle* out);
    /* drain pending datagrams for the handle, match responses,
       and fill info with the latest scalar state. The value
       payload is copied into val (up to val_cap bytes; val_len in
       info reports the full length even when truncated), and the
       who-is responder table into dev_ids/dev_ips (up to
       dev_cap entries each; dev_count reports the total seen).
       Any buffer may be NULL to skip that copy. Always returns
       TPLC_BACNET_OK on a live handle; info->status reports
       PENDING/OK/TIMEOUT/ERROR. */
    int32_t (*bac_poll)(void* svc, TplcBacnetHandle h,
                        TplcBacnetRespInfo* info,
                        uint8_t* val, uint32_t val_cap,
                        uint32_t* dev_ids, uint8_t* dev_ips,
                        uint32_t dev_cap);
    int32_t (*bac_abort)(void* svc, TplcBacnetHandle h);
} TplcBacnetService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_BACNET_H */
