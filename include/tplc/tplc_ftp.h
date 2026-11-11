/* tplc_ftp.h — TplcFtpService: nonblocking FTP client (RFC 959)
 * over TCP 21 with PASV data connections.
 *
 * Model: ftp_open creates a connection handle and drives
 * TCP connect -> USER -> PASS asynchronously; ftp_conn_poll
 * reports 0 connecting / 1 ready / 2 failed. While READY,
 * ftp_cmd runs one control command per handle (MKD, RMD,
 * DELE, RNFR/RNTO sequences via two commands, TYPE, SIZE,
 * CWD, PWD, SYST, NOOP, ...); ftp_xfer_begin starts a data
 * transfer (RETR download, STOR upload, LIST/NLST listing)
 * which internally runs TYPE I -> PASV -> data connect ->
 * command -> 150/125 -> stream -> 226. Downloads stream OUT
 * through repeated ftp_xfer_poll calls (each copies up to
 * val_cap bytes); uploads stream IN through ftp_xfer_feed
 * calls followed by ftp_xfer_finish. One outstanding
 * transaction per connection. All calls nonblocking;
 * deadlines use the injected host clock so --sim stays
 * deterministic.
 *
 * Gaps vs full FTP: PASV data connections only (no PORT/active
 * mode, no EPSV, no FTPS/TLS, no SFTP/SSH — TwinCAT's FTP/SFTP
 * profiles map to plain FTP here), no REST resume, no
 * MLSD/MLST, IPv4 only.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_FTP_H
#define TPLC_FTP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_FTP_ABI_VERSION 1u
#define TPLC_FTP_SVC_NAME "tplc-ftp"
#define TPLC_FTP_PORT 21u
#define TPLC_FTP_MAX_HOST 16u
#define TPLC_FTP_MAX_STR 128u   /* user/pass/path/cmd cap          */
#define TPLC_FTP_MAX_TEXT 256u  /* control reply text cap          */

typedef uint32_t TplcFtpConnHandle;
typedef uint32_t TplcFtpTxnHandle;

/* conn status (ftp_conn_poll info->status) */
#define TPLC_FTP_CONN_PENDING 0
#define TPLC_FTP_CONN_READY   1
#define TPLC_FTP_CONN_ERROR   2

/* txn/xfer status (ftp_txn_poll / ftp_xfer_poll info->status) */
#define TPLC_FTP_ST_PENDING  0
#define TPLC_FTP_ST_OK       1   /* terminal, FTP code 2xx          */
#define TPLC_FTP_ST_FAIL     2   /* terminal, FTP code 4xx/5xx      */
#define TPLC_FTP_ST_TIMEOUT  3
#define TPLC_FTP_ST_ERROR    4   /* transport failure               */
#define TPLC_FTP_ST_BADRESP  5
#define TPLC_FTP_ST_MORE     6   /* xfer only: more data expected   */

/* service return codes */
#define TPLC_FTP_OK        0
#define TPLC_FTP_E_PARAM  -1
#define TPLC_FTP_E_IO     -2
#define TPLC_FTP_E_STATE  -3
#define TPLC_FTP_E_BUSY   -4
#define TPLC_FTP_E_RANGE  -5

/* xfer op codes */
#define TPLC_FTP_XFER_RETR 0u   /* download file -> stream out      */
#define TPLC_FTP_XFER_STOR 1u   /* upload stream -> file            */
#define TPLC_FTP_XFER_LIST 2u   /* LIST/NLST -> stream out          */

/* open request — field order avoids interior padding */
typedef struct TplcFtpOpenReq {
    char     host[TPLC_FTP_MAX_HOST];  /* dotted-quad IPv4         */
    uint16_t port;          /* 0 = TPLC_FTP_PORT (21)           */
    uint16_t reserved;
    uint32_t timeout_ms;    /* 0 = 10000                         */
    char     user[TPLC_FTP_MAX_STR];    /* "anonymous" ok         */
    char     pass[TPLC_FTP_MAX_STR];
} TplcFtpOpenReq;            /* 280 bytes                         */

/* scalar conn status (ftp_conn_poll) */
typedef struct TplcFtpConnInfo {
    int32_t  status;        /* TPLC_FTP_CONN_*                   */
    int32_t  err;           /* transport errno                   */
    uint32_t resp_code;     /* last FTP reply code (conn phase)  */
    uint32_t reserved[13];
} TplcFtpConnInfo;           /* 64 bytes                          */

/* scalar txn/xfer status (poll outputs, 64 bytes) */
typedef struct TplcFtpTxnInfo {
    int32_t  status;        /* TPLC_FTP_ST_*                     */
    int32_t  err;           /* transport errno                   */
    uint32_t resp_code;     /* final FTP reply code              */
    uint32_t reserved0;
    uint64_t total;         /* bytes transferred so far          */
    uint32_t chunk;         /* bytes copied out this poll (dl)   */
    uint32_t reserved[8];
} TplcFtpTxnInfo;            /* 64 bytes                          */

/* control command request */
typedef struct TplcFtpCmdReq {
    char     cmd[TPLC_FTP_MAX_STR];   /* e.g. "MKD /dir"          */
    uint32_t timeout_ms;              /* 0 = 10000                */
    uint32_t reserved;
} TplcFtpCmdReq;             /* 136 bytes                         */

/* xfer request — path + op; uploads feed via ftp_xfer_feed */
typedef struct TplcFtpXferReq {
    uint32_t op;            /* TPLC_FTP_XFER_*                   */
    uint32_t timeout_ms;    /* 0 = 30000 (whole transfer)        */
    char     path[TPLC_FTP_MAX_STR];   /* remote path            */
} TplcFtpXferReq;            /* 136 bytes                         */

typedef struct TplcFtpService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open a connection: nonblocking TCP connect + USER/PASS
       login. Returns TPLC_FTP_OK and a live handle; drive it
       with ftp_conn_poll until status leaves PENDING. */
    int32_t (*ftp_open)(void* svc, const TplcFtpOpenReq* req,
                        TplcFtpConnHandle* out);
    /* drive the connect/login handshake */
    int32_t (*ftp_conn_poll)(void* svc, TplcFtpConnHandle ch,
                             TplcFtpConnInfo* info);
    /* run one control command on a READY connection
       (RETR/STOR/LIST/PASV must go through ftp_xfer_begin —
       data commands here return the interim 1xx/2xx/5xx code
       without a data channel and will fail server-side).
       ftp_txn_poll reports status + resp_code; the reply text
       is copied into the caller's text buffer. */
    int32_t (*ftp_cmd)(void* svc, TplcFtpConnHandle ch,
                       const TplcFtpCmdReq* req,
                       TplcFtpTxnHandle* out);
    /* drive a control-command transaction; textbuf receives the
       last reply line (NUL-terminated, up to text_cap-1) */
    int32_t (*ftp_txn_poll)(void* svc, TplcFtpTxnHandle h,
                            TplcFtpTxnInfo* info,
                            char* textbuf, uint32_t text_cap);
    /* begin a RETR/STOR/LIST data transfer on a READY conn */
    int32_t (*ftp_xfer_begin)(void* svc, TplcFtpConnHandle ch,
                              const TplcFtpXferReq* req,
                              TplcFtpTxnHandle* out);
    /* drive a transfer: for RETR/LIST each call copies up to
       val_cap received bytes into val (info.chunk = bytes this
       call, info.total = cumulative); status MORE while data
       flows, OK/FAIL/TIMEOUT/ERROR terminal. For STOR feed
       bytes with ftp_xfer_feed then call ftp_xfer_finish. */
    int32_t (*ftp_xfer_poll)(void* svc, TplcFtpTxnHandle h,
                             TplcFtpTxnInfo* info,
                             uint8_t* val, uint32_t val_cap);
    /* queue upload bytes for a pending STOR transfer; returns
       bytes accepted (may be < len when the socket backs up) */
    int32_t (*ftp_xfer_feed)(void* svc, TplcFtpTxnHandle h,
                             const uint8_t* buf, uint32_t len);
    /* signal end-of-upload-data for a STOR transfer */
    int32_t (*ftp_xfer_finish)(void* svc, TplcFtpTxnHandle h);
    int32_t (*ftp_abort)(void* svc, TplcFtpTxnHandle h);
    int32_t (*ftp_close)(void* svc, TplcFtpConnHandle ch);
} TplcFtpService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_FTP_H */
