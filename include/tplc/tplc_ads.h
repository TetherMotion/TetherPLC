/* tplc_ads.h — tplc-ads service ABI.
 * Returned by TplcServices.get_service("tplc-ads"). ADS client over
 * AMS/TCP (Beckhoff wire format): 6-byte TCP header, 32-byte AMS
 * header, ADS payload. All ops are nonblocking and bounded: one
 * outstanding request per connection, polled from the scan path.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_ADS_H
#define TPLC_ADS_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_ADS_SERVICE_NAME "tplc-ads"
#define TPLC_ADS_ABI_VERSION  1u

/* ads handle: service-assigned u32, 0 = invalid                      */
typedef uint32_t TplcAdsHandle;
#define TPLC_ADS_HANDLE_INVALID 0u

/* op result codes (mirrored into FB nErrId on failure)                */
enum {
    TPLC_ADS_OK          = 0,
    TPLC_ADS_E_PARAM     = -1,  /* bad args (null buf, len, host)   */
    TPLC_ADS_E_NOSERVICE = -2,  /* service unavailable            */
    TPLC_ADS_E_CONNECT   = -3,  /* TCP connect failed             */
    TPLC_ADS_E_IO        = -4,  /* socket syscall failed          */
    TPLC_ADS_E_CLOSED    = -5,  /* connection closed by peer     */
    TPLC_ADS_E_AGAIN     = -6,  /* still pending, retry next scan */
    TPLC_ADS_E_STATE     = -7,  /* op not valid in current state  */
    TPLC_ADS_E_PROTO     = -8,  /* malformed response frame       */
};

/* ADS command ids (AMS header command_id)                             */
enum {
    TPLC_ADS_CMD_READ         = 0x02,
    TPLC_ADS_CMD_WRITE        = 0x03,
    TPLC_ADS_CMD_READ_STATE   = 0x04,
    TPLC_ADS_CMD_WRITE_CTRL   = 0x05,
    TPLC_ADS_CMD_READ_WRITE   = 0x09,
};

/* ADS device states (read-state / write-control)                      */
enum {
    TPLC_ADS_STATE_INVALID = 0,
    TPLC_ADS_STATE_IDLE    = 1,
    TPLC_ADS_STATE_RESET   = 2,
    TPLC_ADS_STATE_INIT    = 3,
    TPLC_ADS_STATE_START   = 4,
    TPLC_ADS_STATE_RUN     = 5,
};

typedef struct TplcAdsService {
    uint32_t struct_size;      /* sizeof(TplcAdsService)            */
    uint32_t abi_version;      /* TPLC_ADS_ABI_VERSION              */
    void*    host;             /* service-private cookie           */

    /* open a TCP connection to host:port (typically the ADS port
       851). The connect is nonblocking: the handle is returned
       immediately and the connection completes inside the first
       request, which reports E_CONNECT on failure.               */
    int32_t (*ads_open)(void* svc, const char* host, uint16_t port,
                        TplcAdsHandle* out);

    /* ADS Read: request grp/off/len bytes; on completion *rlen is
       the byte count copied into buf (<= cap) and *adserr the
       ADS-layer error (0 = ok). Returns TPLC_ADS_OK when the
       response is complete, TPLC_ADS_E_AGAIN while pending.      */
    int32_t (*ads_read)(void* svc, TplcAdsHandle h,
                        uint32_t grp, uint32_t off, uint32_t len,
                        void* buf, uint32_t cap,
                        uint32_t* rlen, uint32_t* adserr);

    /* ADS Write: send len bytes from data at grp/off. *adserr gets
       the ADS-layer error on completion.                         */
    int32_t (*ads_write)(void* svc, TplcAdsHandle h,
                         uint32_t grp, uint32_t off,
                         const void* data, uint32_t len,
                         uint32_t* adserr);

    /* ADS Read State: on completion *ads_state and *dev_state get
       the device states; *adserr the ADS-layer error.            */
    int32_t (*ads_read_state)(void* svc, TplcAdsHandle h,
                              uint32_t* ads_state, uint32_t* dev_state,
                              uint32_t* adserr);

    /* ADS Write Control: request the given ADS/device states.
       v1 sends no additional control data (length 0).            */
    int32_t (*ads_write_control)(void* svc, TplcAdsHandle h,
                                 uint32_t ads_state, uint32_t dev_state,
                                 uint32_t* adserr);

    /* close handle; idempotent (OK on invalid handle)               */
    int32_t (*ads_close)(void* svc, TplcAdsHandle h);
} TplcAdsService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_ADS_H */
