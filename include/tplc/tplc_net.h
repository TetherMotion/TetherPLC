/* tplc_net.h — tplc-net socket service ABI (plan 21 C2).
 * Returned by TplcServices.get_service("tplc-net"). All ops are
 * nonblocking and bounded: they run on the scan path under the task
 * WCET budget, so they must never wait. Long operations (connect,
 * accept) are driven by repeated calls until READY/ERRID.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_NET_H
#define TPLC_NET_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_NET_SERVICE_NAME "tplc-net"
#define TPLC_NET_ABI_VERSION  1u

/* socket handle: service-assigned u32, 0 = invalid                        */
typedef uint32_t TplcNetHandle;
#define TPLC_NET_HANDLE_INVALID 0u

/* op result codes (mirrored into FB nErrId on failure)                   */
enum {
    TPLC_NET_OK            = 0,
    TPLC_NET_E_PARAM       = -1,  /* bad args (null buf, len, host)      */
    TPLC_NET_E_NOSERVICE   = -2,  /* service unavailable on this target  */
    TPLC_NET_E_SOCKET      = -3,  /* socket()/bind() failed              */
    TPLC_NET_E_CONNECT     = -4,  /* connect refused/failed              */
    TPLC_NET_E_TIMEOUT     = -5,  /* deadline expired                    */
    TPLC_NET_E_CLOSED      = -6,  /* peer closed / handle invalid        */
    TPLC_NET_E_AGAIN       = -7,  /* would block — retry next scan       */
    TPLC_NET_E_STATE       = -8,  /* op not valid in current state       */
};

/* FB_Socket OP selector values (builtin FB member OP). The FB owns
   the state machine; these only name which service op it drives.  */
enum {
    TPLC_SOCK_OP_CONNECT    = 1,  /* TCP client connect              */
    TPLC_SOCK_OP_LISTEN     = 2,  /* TCP listener                    */
    TPLC_SOCK_OP_ACCEPT     = 3,  /* accept on listener (AUX in)     */
    TPLC_SOCK_OP_SEND       = 4,  /* TCP send DATA[0..DLEN)          */
    TPLC_SOCK_OP_RECV       = 5,  /* TCP recv into DATA              */
    TPLC_SOCK_OP_CLOSE      = 6,  /* close HANDLE                    */
    TPLC_SOCK_OP_UDP_BIND   = 7,  /* UDP bind                        */
    TPLC_SOCK_OP_UDP_SENDTO = 8,  /* UDP sendto HOST:PORT peer       */
    TPLC_SOCK_OP_UDP_RECV   = 9,  /* UDP recvfrom (peer -> HOST/AUX) */
    TPLC_SOCK_OP_UDP_CLOSE  = 10, /* close UDP HANDLE                */
};

typedef struct TplcNetService {
    uint32_t struct_size;      /* sizeof(TplcNetService)               */
    uint32_t abi_version;      /* TPLC_NET_ABI_VERSION                 */
    void*    host;             /* service-private cookie               */

    /* TCP client: begin nonblocking connect to host:port.
       Returns TPLC_NET_OK with *out set on success; the connection
       may still be in progress — poll tcp_connect_poll until it
       returns TPLC_NET_OK (connected) or an error. *out is 0 while
       the attempt is pending state owned by the service.            */
    int32_t (*tcp_connect)(void* svc, const char* host, uint16_t port,
                           TplcNetHandle* out);
    /* poll an in-progress connect: TPLC_NET_OK = established,
       TPLC_NET_E_AGAIN = still pending, <0 = failed                  */
    int32_t (*tcp_connect_poll)(void* svc, TplcNetHandle h);

    /* TCP server: listen on host:port (host NULL/"" = any).
       Returns listener handle via *out.                              */
    int32_t (*tcp_listen)(void* svc, const char* host, uint16_t port,
                          TplcNetHandle* out);
    /* accept one pending connection: TPLC_NET_OK + *out on success,
       TPLC_NET_E_AGAIN when no client is queued                      */
    int32_t (*tcp_accept)(void* svc, TplcNetHandle listener,
                          TplcNetHandle* out);

    /* send/recv up to len bytes; *done receives actual count.
       TPLC_NET_E_AGAIN = nothing transferred this scan (retry);
       TPLC_NET_OK with *done>0 = progress; *done==0 && OK = peer
       closed cleanly. Partial progress is reported, never buffered
       inside the service — the caller owns the data.               */
    int32_t (*tcp_send)(void* svc, TplcNetHandle h, const void* buf,
                        uint32_t len, uint32_t* done);
    int32_t (*tcp_recv)(void* svc, TplcNetHandle h, void* buf,
                        uint32_t cap, uint32_t* done);

    /* close handle; idempotent (OK on invalid handle)                */
    int32_t (*tcp_close)(void* svc, TplcNetHandle h);

    /* UDP: bind local socket; udp_sendto/udp_recvfrom take a peer
       host:port per call. host may be NULL for bind = any.         */
    int32_t (*udp_bind)(void* svc, const char* host, uint16_t port,
                        TplcNetHandle* out);
    int32_t (*udp_sendto)(void* svc, TplcNetHandle h, const char* host,
                          uint16_t port, const void* buf, uint32_t len,
                          uint32_t* done);
    int32_t (*udp_recvfrom)(void* svc, TplcNetHandle h, void* buf,
                            uint32_t cap, char* from_host, uint16_t* from_port,
                            uint32_t* done);
    int32_t (*udp_close)(void* svc, TplcNetHandle h);
} TplcNetService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_NET_H */
