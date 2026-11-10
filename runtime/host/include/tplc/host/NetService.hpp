/* NetService.hpp — tplc-net host implementation (plan 21 C2).
 * POSIX nonblocking sockets behind the TplcNetService vtable.
 * Every op returns immediately; connect/accept progress is polled
 * by the caller (the FB_Socket state machine) across scans.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_NETSERVICE_HPP
#define TPLC_HOST_NETSERVICE_HPP

#include <tplc/tplc_net.h>

#include <unordered_map>

namespace tplc {

class NetService {
public:
    NetService();

    const TplcNetService& table() const { return svc_; }

private:
    enum class Kind : uint8_t { Tcp, TcpConnecting, Listen, Udp };
    struct Entry {
        Kind     kind;
        int      fd;
        uint16_t port;   /* bound/local port for udp_recvfrom peers    */
    };

    int32_t tcpConnect(void* s, const char* host, uint16_t port,
                       TplcNetHandle* out);
    int32_t tcpConnectPoll(void* s, TplcNetHandle h);
    int32_t tcpListen(void* s, const char* host, uint16_t port,
                      TplcNetHandle* out);
    int32_t tcpAccept(void* s, TplcNetHandle listener,
                      TplcNetHandle* out);
    int32_t tcpSend(void* s, TplcNetHandle h, const void* buf,
                    uint32_t len, uint32_t* done);
    int32_t tcpRecv(void* s, TplcNetHandle h, void* buf, uint32_t cap,
                    uint32_t* done);
    int32_t tcpClose(void* s, TplcNetHandle h);
    int32_t udpBind(void* s, const char* host, uint16_t port,
                    TplcNetHandle* out);
    int32_t udpSendTo(void* s, TplcNetHandle h, const char* host,
                      uint16_t port, const void* buf, uint32_t len,
                      uint32_t* done);
    int32_t udpRecvFrom(void* s, TplcNetHandle h, void* buf,
                        uint32_t cap, char* from_host, uint16_t* from_port,
                        uint32_t* done);
    int32_t udpClose(void* s, TplcNetHandle h);

    TplcNetHandle add(Kind k, int fd, uint16_t port = 0);
    Entry*        find(TplcNetHandle h);
    void          drop(TplcNetHandle h);
    static int32_t mapErr(int e);

    TplcNetService            svc_{};
    std::unordered_map<uint32_t, Entry> entries_;
    uint32_t                  next_handle_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_NETSERVICE_HPP */
