/* tplc_rtmon.h — tplc-rtmon service ABI.
 * Returned by TplcServices.get_service("tplc-rtmon"). Real-time
 * monitor mark buffer: timestamped records appended from the scan
 * path into a bounded ring, queryable by index. Backs the TwinCAT
 * Tc3_RealtimeMonitoring FBs (FB_RTMon_LogMark/Base).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_RTMON_H
#define TPLC_RTMON_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_RTMON_SERVICE_NAME "tplc-rtmon"
#define TPLC_RTMON_ABI_VERSION  1u

/* op result codes (mirrored into FB nErrId on failure) */
enum {
    TPLC_RTMON_OK        = 0,
    TPLC_RTMON_E_PARAM   = -1,  /* bad args                              */
    TPLC_RTMON_E_NOSERVICE = -2,/* service unavailable                   */
    TPLC_RTMON_E_RANGE   = -3,  /* index out of range on read            */
    TPLC_RTMON_E_FULL    = -4,  /* ring full (overwrite policy: drop)    */
};

/* one mark record; ts is steady-clock ns, ids are the TwinCAT
   UINT-width context/group/event plus the UDINT mark-control mask */
typedef struct TplcRtmonMark {
    uint64_t ts_ns;
    uint32_t context;
    uint32_t group;
    uint32_t event;
    uint32_t markctrl;
} TplcRtmonMark;

typedef struct TplcRtmonService {
    uint32_t struct_size;      /* sizeof(TplcRtmonService)             */
    uint32_t abi_version;      /* TPLC_RTMON_ABI_VERSION               */
    void*    host;             /* service-private cookie              */

    /* append a mark; nonblocking, O(1). Returns TPLC_RTMON_OK or
       TPLC_RTMON_E_PARAM. When the ring is full the oldest record
       is overwritten (TwinCAT keeps a fixed window too).          */
    int32_t (*rtmon_mark)(void* svc, uint32_t context, uint32_t group,
                          uint32_t event, uint32_t markctrl);

    /* retained marks available via rtmon_get (0 = oldest retained);
       equals the valid index domain of rtmon_get              */
    int32_t (*rtmon_count)(void* svc, uint32_t* count);

    /* read record `index` (0 = oldest retained). Fails with
       TPLC_RTMON_E_RANGE when index >= rtmon_count, i.e. the
       record has already been overwritten.                       */
    int32_t (*rtmon_get)(void* svc, uint32_t index, TplcRtmonMark* out);

    /* drop all retained marks; count resets to 0.                   */
    int32_t (*rtmon_clear)(void* svc);
} TplcRtmonService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_RTMON_H */
