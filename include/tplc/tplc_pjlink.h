/* tplc_pjlink.h — TplcPjlinkService: nonblocking PJLink projector
 * client (JBMIA PJLink protocol, Class 1/2) over TCP port 4352.
 *
 * Wire format: ASCII command lines `%<class><CMD4> <param>$<CR>`
 * (class = one digit 0..9, command body = 4 ASCII chars,
 * separator = space, transmission parameter <= 128 bytes,
 * terminator = `$` + CR). Responses are `%<class><CMD4>=<value><CR>`
 * lines; command failures report `%...=ERR<n>` values and an
 * unauthenticated session is rejected with a standalone
 * `PJLINK ERRA<CR>` line.
 *
 * Authentication: on TCP connect a PJLink-capable projector sends
 * a greeting line `PJLINK <0|1> [<8-hex-random>]<CR><LF>`. `1`
 * means password auth is required: the client prepends
 * MD5(random_ascii + password) as 32 lowercase hex chars directly
 * to the first command line (no separator). `0` or no greeting
 * means no auth. The service performs this transparently when
 * req->password is non-empty; an empty password still completes
 * the hash exchange so a wrong/missing password surfaces as
 * PJLINK ERRA rather than a silent hang.
 *
 * One transaction per handle, mirroring tplc-modbus/tplc-bacnet:
 * pjl_begin connects and queues the command, pjl_poll drives the
 * nonblocking connect/greeting/send/receive state machine and
 * reports progress in info->status. All calls are nonblocking;
 * the service clock is injected by the host so --sim stays
 * deterministic. The TCP session is per-transaction
 * (connect-per-command); PJLink keep-alive sessions are not yet
 * exposed.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_PJLINK_H
#define TPLC_PJLINK_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_PJLINK_ABI_VERSION 1u
#define TPLC_PJLINK_SVC_NAME "tplc-pjlink"
#define TPLC_PJLINK_HANDLE_INVALID 0u
#define TPLC_PJLINK_PORT 4352u
#define TPLC_PJLINK_MAX_PARAM 128u
#define TPLC_PJLINK_MAX_VALUE 160u

/* request fields */
#define TPLC_PJLINK_MAX_HOST 16u
#define TPLC_PJLINK_MAX_PASS 32u   /* spec: 32 ASCII alnum max   */
#define TPLC_PJLINK_CMD_LEN 4u     /* command body is 4 chars    */

/* poll status (info->status) */
#define TPLC_PJLINK_ST_PENDING   0 /* still working               */
#define TPLC_PJLINK_ST_OK        1 /* got a `%cls cmd=value` line */
#define TPLC_PJLINK_ST_TIMEOUT   2 /* deadline expired            */
#define TPLC_PJLINK_ST_ERROR     3 /* connect/transport failure   */
#define TPLC_PJLINK_ST_BADRESP   4 /* malformed response line     */
#define TPLC_PJLINK_ST_AUTHFAIL  5 /* PJLINK ERRA / bad digest   */

/* service return codes (pjl_begin/pjl_poll/pjl_abort) */
#define TPLC_PJLINK_OK        0
#define TPLC_PJLINK_E_PARAM  -1
#define TPLC_PJLINK_E_IO     -2
#define TPLC_PJLINK_E_STATE  -3   /* bad handle                     */
#define TPLC_PJLINK_E_RANGE  -4   /* request out of range           */

/* field order avoids interior padding: host/port/reserved,
   cls/cmd/param/timeout, password last. Size = 196 bytes. */
typedef struct TplcPjlinkReq {
    char     host[TPLC_PJLINK_MAX_HOST]; /* dotted-quad IPv4        */
    uint16_t port;          /* 0 = TPLC_PJLINK_PORT (4352)      */
    uint16_t reserved;
    uint32_t cls;           /* PJLink class digit 0..9          */
    char     cmd[TPLC_PJLINK_CMD_LEN + 1]; /* 4-char body, e.g. POWR */
    char     param[TPLC_PJLINK_MAX_PARAM + 1]; /* "?" = query   */
    uint16_t timeout_ms;    /* 0 = 2000 ms default              */
    uint16_t reserved2;
    char     password[TPLC_PJLINK_MAX_PASS + 1]; /* "" = no auth */
} TplcPjlinkReq;             /* 196 bytes                        */

/* poll output: scalar state only (fixed 64-byte layout). The
   response value text is copied into the caller's buffer by
   pjl_poll — see its signature. For `%1POWR=1` the value is "1";
   for `%0POWR=ERR3`-style failures the value is "ERR3" with
   status still OK (the projector understood the command). */
typedef struct TplcPjlinkRespInfo {
    int32_t  status;        /* TPLC_PJLINK_ST_*                  */
    int32_t  err;           /* transport errno or 0              */
    uint32_t resp_cls;      /* class digit of the response       */
    uint32_t resp_cmd;      /* 4-char body packed big-endian     */
    uint32_t value_len;     /* bytes available in the val buffer */
    uint32_t authed;        /* 1 when the password exchange ran  */
    uint32_t reserved[10];
} TplcPjlinkRespInfo;         /* 64 bytes                        */

typedef uint32_t TplcPjlinkHandle;

typedef struct TplcPjlinkService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open a transaction: nonblocking connect + queue the
       command. Returns TPLC_PJLINK_OK and a live handle, or a
       negative code for bad parameters. */
    int32_t (*pjl_begin)(void* svc, const TplcPjlinkReq* req,
                         TplcPjlinkHandle* out);
    /* drive the transaction: connect -> greeting -> (auth) ->
       send -> receive -> parse. info reports the latest scalar
       state; the response value text is copied into val as a
       NUL-terminated string (up to val_cap-1 chars + NUL;
       value_len reports the full length even when truncated).
       val may be NULL to skip the copy. Always returns
       TPLC_PJLINK_OK on a live handle; info->status reports
       PENDING/OK/TIMEOUT/ERROR/BADRESP/AUTHFAIL. */
    int32_t (*pjl_poll)(void* svc, TplcPjlinkHandle h,
                        TplcPjlinkRespInfo* info,
                        char* val, uint32_t val_cap);
    int32_t (*pjl_abort)(void* svc, TplcPjlinkHandle h);
} TplcPjlinkService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_PJLINK_H */
