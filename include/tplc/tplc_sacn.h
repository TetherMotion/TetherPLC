/* tplc_sacn.h — TplcSacnService: nonblocking sACN (ANSI E1.31)
 * source and receiver over UDP multicast.
 *
 * Wire format (E1.31): ACN root layer (16B preamble "ASC-E1.17",
 * flags+length, vector, 16B sender CID), then a framing layer and
 * packet-specific layers. Data packets use root vector 0x00000004,
 * framing vector 0x00000002, DMP vector 0x02; extended packets
 * (sync 0x00000001, universe discovery 0x00000002 under framing
 * vector 0x00000002 of root vector 0x00000008) share the extended
 * root. Multicast group per universe is 239.255.(u>>8).(u&0xFF),
 * port 5568; discovery packets are broadcast.
 *
 * Handles are per-role: source handles own one UDP socket and a
 * rate-limited transmit state; receiver handles own one socket
 * bound to the sACN port with IP_ADD_MEMBERSHIP for the data
 * universe (and optionally a second group for the sync universe).
 * All calls are nonblocking; the service clock is injected by the
 * host so --sim stays deterministic.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_SACN_H
#define TPLC_SACN_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_SACN_ABI_VERSION 1u
#define TPLC_SACN_SVC_NAME "tplc-sacn"
#define TPLC_SACN_HANDLE_INVALID 0u
#define TPLC_SACN_PORT 5568u
#define TPLC_SACN_MAX_DMX 512u
#define TPLC_SACN_NAME_LEN 64u
#define TPLC_SACN_CID_LEN 16u
#define TPLC_SACN_UNI_WINDOW 64u /* discovery universes exposed
                                    per poll (spec allows 512)   */

/* service return codes */
#define TPLC_SACN_OK        0
#define TPLC_SACN_E_PARAM  -1
#define TPLC_SACN_E_IO     -2
#define TPLC_SACN_E_STATE  -3   /* bad handle                          */
#define TPLC_SACN_E_RANGE  -4   /* request out of range                */

/* source request; all fields fixed at sacn_src_begin except the
   DMX frame, which is supplied per sacn_src_poll. name is a
   NUL-terminated UTF-8 string (<=63 bytes used); cid all-zero
   makes the service generate a per-handle CID. options uses the
   wire bits: bit5 forceSync, bit6 streamTerminated, bit7 preview.
   flags: bit0 also emit sync packets (needs sync_addr != 0),
   bit1 also emit universe-discovery broadcasts. */
typedef struct TplcSacnSrcReq {
    uint32_t universe;      /* 1..63999                            */
    uint32_t priority;      /* 0..200                              */
    uint32_t sync_addr;     /* sync universe, 0 = unsynchronised   */
    uint32_t interval_ms;   /* min spacing between data packets;
                               0 = send on every poll               */
    uint32_t ttl;           /* multicast TTL, 0 = 8                 */
    uint8_t  options;       /* wire option bits                    */
    uint8_t  flags;         /* TX aux packet enables               */
    uint8_t  reserved[2];
    char     name[TPLC_SACN_NAME_LEN];
    uint8_t  cid[TPLC_SACN_CID_LEN];
} TplcSacnSrcReq;

/* per-poll DMX frame; dmx points at slot data (start code is
   separate). The service copies the bytes, so the caller buffer
   only needs to stay valid for the call. */
typedef struct TplcSacnFrame {
    const uint8_t* dmx;     /* slot values, up to 512 bytes         */
    uint16_t dmx_len;       /* 1..512                              */
    uint8_t  start_code;    /* 0 = level data                      */
} TplcSacnFrame;

/* source poll output */
typedef struct TplcSacnSrcInfo {
    uint64_t sent;          /* data packets transmitted            */
    uint64_t aux_sent;      /* sync + discovery packets sent       */
    uint32_t seq;           /* current data-packet sequence        */
    int32_t  last_err;      /* last socket error code, 0 = none    */
} TplcSacnSrcInfo;

/* receiver request: join the multicast group of `universe` on
   `port` (0 = 5568); when sync_universe != 0 a second group is
   joined so sync packets for that universe are received. */
typedef struct TplcSacnRcvReq {
    uint32_t universe;      /* data universe, 1..63999             */
    uint32_t sync_universe; /* 0 = no sync group                   */
    uint32_t port;          /* 0 = TPLC_SACN_PORT                  */
} TplcSacnRcvReq;

/* receiver poll output: scalar state only (64 bytes, fixed
   layout). The DMX payload, source name, and discovery universe
   window are copied into caller buffers by sacn_rcv_poll — see
   its signature. data_rx counts accepted data packets for the
   configured universe (use it to edge-detect new frames). */
typedef struct TplcSacnRcvInfo {
    uint64_t data_rx;       /* accepted data packets             */
    uint64_t data_dropped;  /* wrong universe / malformed         */
    uint64_t sync_rx;       /* sync packets received             */
    uint64_t disc_rx;       /* discovery packets received        */
    uint16_t uni_count;     /* universes in last discovery list  */
    uint16_t uni_page;      /* discovery page number             */
    uint16_t uni_last_page; /* discovery last page               */
    uint16_t sync_addr;     /* sync universe of latest data pkt  */
    uint16_t universe;      /* universe of latest data packet    */
    uint16_t dmx_len;       /* bytes in latest DMX payload       */
    uint16_t sync_addr_seen;/* sync packet's address field       */
    uint8_t  have_data;     /* a data packet was accepted        */
    uint8_t  preview;
    uint8_t  stream_terminated;
    uint8_t  force_sync;
    uint8_t  start_code;
    uint8_t  seq;
    uint8_t  priority;
    uint8_t  have_sync;
    uint8_t  sync_seq;
    uint8_t  have_discovery;
    uint8_t  remote_ip[4];  /* sender of latest datagram         */
    uint8_t  reserved[4];
} TplcSacnRcvInfo;           /* 64 bytes                          */

typedef uint32_t TplcSacnHandle;

typedef struct TplcSacnService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* source role: open a UDP socket for multicast TX. */
    int32_t (*sacn_src_begin)(void* svc, const TplcSacnSrcReq* req,
                              TplcSacnHandle* out);
    /* offer the latest DMX frame and transmit when the interval
       has elapsed; frame may be NULL to reuse the stored frame.
       Always returns TPLC_SACN_OK on a live handle; send errors
       surface via info->last_err. */
    int32_t (*sacn_src_poll)(void* svc, TplcSacnHandle h,
                             const TplcSacnFrame* frame,
                             TplcSacnSrcInfo* info);
    int32_t (*sacn_src_close)(void* svc, TplcSacnHandle h);
    /* receiver role: bind the sACN port and join the universe
       multicast group(s). */
    int32_t (*sacn_rcv_begin)(void* svc, const TplcSacnRcvReq* req,
                              TplcSacnHandle* out);
    /* drain pending datagrams, fill info with the latest
       scalar state, and copy the latest DMX payload (up to
       dmx_cap bytes), source name (up to name_cap-1 chars plus
       NUL), and discovery universe window (up to unis_cap
       entries) into the caller buffers. Any buffer may be NULL
       to skip that copy. Always returns TPLC_SACN_OK on a live
       handle. */
    int32_t (*sacn_rcv_poll)(void* svc, TplcSacnHandle h,
                             TplcSacnRcvInfo* info,
                             uint8_t* dmx, uint16_t dmx_cap,
                             uint16_t* unis, uint16_t unis_cap,
                             char* name, uint32_t name_cap);
    int32_t (*sacn_rcv_close)(void* svc, TplcSacnHandle h);
} TplcSacnService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_SACN_H */
