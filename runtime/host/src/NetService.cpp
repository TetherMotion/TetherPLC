/* NetService.cpp — POSIX nonblocking socket service (plan 21 C2).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/NetService.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

namespace tplc {

static NetService* self(void* p) { return static_cast<NetService*>(p); }
static NetService* svc_of(void* svc) {
    return self(((TplcNetService*)svc)->host);
}

NetService::NetService() {
    svc_.struct_size    = sizeof(TplcNetService);
    svc_.abi_version    = TPLC_NET_ABI_VERSION;
    svc_.host           = this;
    svc_.tcp_connect    = [](void* s, const char* h, uint16_t p,
                             TplcNetHandle* o) {
        return self(svc_of(s))->tcpConnect(s, h, p, o);
    };
    svc_.tcp_connect_poll = [](void* s, TplcNetHandle h) {
        return self(svc_of(s))->tcpConnectPoll(s, h);
    };
    svc_.tcp_listen     = [](void* s, const char* h, uint16_t p,
                             TplcNetHandle* o) {
        return self(svc_of(s))->tcpListen(s, h, p, o);
    };
    svc_.tcp_accept     = [](void* s, TplcNetHandle l, TplcNetHandle* o) {
        return self(svc_of(s))->tcpAccept(s, l, o);
    };
    svc_.tcp_send       = [](void* s, TplcNetHandle h, const void* b,
                             uint32_t l, uint32_t* d) {
        return self(svc_of(s))->tcpSend(s, h, b, l, d);
    };
    svc_.tcp_recv       = [](void* s, TplcNetHandle h, void* b, uint32_t c,
                             uint32_t* d) {
        return self(svc_of(s))->tcpRecv(s, h, b, c, d);
    };
    svc_.tcp_close      = [](void* s, TplcNetHandle h) {
        return self(svc_of(s))->tcpClose(s, h);
    };
    svc_.udp_bind       = [](void* s, const char* h, uint16_t p,
                             TplcNetHandle* o) {
        return self(svc_of(s))->udpBind(s, h, p, o);
    };
    svc_.udp_sendto     = [](void* s, TplcNetHandle h, const char* rh,
                             uint16_t rp, const void* b, uint32_t l,
                             uint32_t* d) {
        return self(svc_of(s))->udpSendTo(s, h, rh, rp, b, l, d);
    };
    svc_.udp_recvfrom   = [](void* s, TplcNetHandle h, void* b, uint32_t c,
                             char* fh, uint16_t* fp, uint32_t* d) {
        return self(svc_of(s))->udpRecvFrom(s, h, b, c, fh, fp, d);
    };
    svc_.udp_close      = [](void* s, TplcNetHandle h) {
        return self(svc_of(s))->udpClose(s, h);
    };
}

/* dotted-quad only in v1 — no DNS on the scan path (would block) */
static bool parse_v4(const char* host, struct in_addr* out) {
    if (!host || !*host) return false;
    return inet_pton(AF_INET, host, out) == 1;
}

static void make_addr(struct sockaddr_in* sa, const char* host,
                      uint16_t port, bool any) {
    memset(sa, 0, sizeof(*sa));
    sa->sin_family = AF_INET;
    sa->sin_port   = htons(port);
    if (any || !host || !*host) {
        sa->sin_addr.s_addr = htonl(INADDR_ANY);
    } else {
        inet_pton(AF_INET, host, &sa->sin_addr);
    }
}

static int nonblock(int fd) {
    int fl = fcntl(fd, F_GETFL, 0);
    return fl < 0 ? -1 : fcntl(fd, F_SETFL, fl | O_NONBLOCK);
}

TplcNetHandle NetService::add(Kind k, int fd, uint16_t port) {
    TplcNetHandle h = next_handle_++;
    if (h == TPLC_NET_HANDLE_INVALID) h = next_handle_++;
    entries_[h] = Entry{k, fd, port};
    return h;
}

NetService::Entry* NetService::find(TplcNetHandle h) {
    auto it = entries_.find(h);
    return it == entries_.end() ? nullptr : &it->second;
}

void NetService::drop(TplcNetHandle h) {
    auto it = entries_.find(h);
    if (it != entries_.end()) {
        ::close(it->second.fd);
        entries_.erase(it);
    }
}

int32_t NetService::mapErr(int e) {
    switch (e) {
    case EAGAIN: return TPLC_NET_E_AGAIN; /* EWOULDBLOCK == EAGAIN */
    case ECONNREFUSED:
    case ECONNRESET:
    case EHOSTUNREACH:
    case ENETUNREACH:
    case ETIMEDOUT:   return TPLC_NET_E_CONNECT;
    case EBADF:
    case ENOTCONN:    return TPLC_NET_E_CLOSED;
    case EINVAL:
    case EOPNOTSUPP:  return TPLC_NET_E_STATE;
    default:          return TPLC_NET_E_SOCKET;
    }
}

int32_t NetService::tcpConnect(void* s, const char* host, uint16_t port,
                               TplcNetHandle* out) {
    (void)s;
    struct in_addr a;
    if (!out || !parse_v4(host, &a)) return TPLC_NET_E_PARAM;
    int fd = ::socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_NET_E_SOCKET;
    if (nonblock(fd) < 0) { ::close(fd); return TPLC_NET_E_SOCKET; }
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));
    struct sockaddr_in sa;
    make_addr(&sa, host, port, false);
    sa.sin_addr = a;
    if (::connect(fd, (struct sockaddr*)&sa, sizeof(sa)) == 0) {
        *out = add(Kind::Tcp, fd);
        return TPLC_NET_OK;
    }
    if (errno == EINPROGRESS) {
        *out = add(Kind::TcpConnecting, fd);
        return TPLC_NET_OK;
    }
    int e = errno;
    ::close(fd);
    return mapErr(e);
}

int32_t NetService::tcpConnectPoll(void* s, TplcNetHandle h) {
    (void)s;
    Entry* e = find(h);
    if (!e) return TPLC_NET_E_CLOSED;
    if (e->kind == Kind::Tcp) return TPLC_NET_OK;
    if (e->kind != Kind::TcpConnecting) return TPLC_NET_E_STATE;
    int err = 0;
    socklen_t l = sizeof(err);
    if (getsockopt(e->fd, SOL_SOCKET, SO_ERROR, &err, &l) < 0)
        return mapErr(errno);
    if (err == 0) { e->kind = Kind::Tcp; return TPLC_NET_OK; }
    if (err == EINPROGRESS || err == EALREADY || err == EWOULDBLOCK)
        return TPLC_NET_E_AGAIN;
    drop(h);
    return mapErr(err);
}

int32_t NetService::tcpListen(void* s, const char* host, uint16_t port,
                              TplcNetHandle* out) {
    (void)s;
    if (!out || port == 0) return TPLC_NET_E_PARAM;
    struct in_addr a;
    if (host && *host && !parse_v4(host, &a)) return TPLC_NET_E_PARAM;
    int fd = ::socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return TPLC_NET_E_SOCKET;
    if (nonblock(fd) < 0) { ::close(fd); return TPLC_NET_E_SOCKET; }
    int one = 1;
    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));
    struct sockaddr_in sa;
    make_addr(&sa, host, port, host && *host);
    if (host && *host) sa.sin_addr = a;
    if (::bind(fd, (struct sockaddr*)&sa, sizeof(sa)) < 0 ||
        ::listen(fd, 8) < 0) {
        int e = errno;
        ::close(fd);
        return mapErr(e);
    }
    *out = add(Kind::Listen, fd, port);
    return TPLC_NET_OK;
}

int32_t NetService::tcpAccept(void* s, TplcNetHandle listener,
                              TplcNetHandle* out) {
    (void)s;
    Entry* e = find(listener);
    if (!e || e->kind != Kind::Listen || !out) return TPLC_NET_E_STATE;
    struct sockaddr_in peer;
    socklen_t pl = sizeof(peer);
    int fd = ::accept4(e->fd, (struct sockaddr*)&peer, &pl,
                       SOCK_NONBLOCK);
    if (fd < 0) return mapErr(errno);
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));
    *out = add(Kind::Tcp, fd, ntohs(peer.sin_port));
    return TPLC_NET_OK;
}

int32_t NetService::tcpSend(void* s, TplcNetHandle h, const void* buf,
                            uint32_t len, uint32_t* done) {
    (void)s;
    Entry* e = find(h);
    if (!e || !buf || !done || e->kind == Kind::Listen ||
        e->kind == Kind::TcpConnecting)
        return TPLC_NET_E_STATE;
    if (len == 0) { *done = 0; return TPLC_NET_OK; }
    ssize_t n = ::send(e->fd, buf, len, MSG_NOSIGNAL);
    if (n < 0) return mapErr(errno);
    *done = (uint32_t)n;
    return TPLC_NET_OK;
}

int32_t NetService::tcpRecv(void* s, TplcNetHandle h, void* buf,
                            uint32_t cap, uint32_t* done) {
    (void)s;
    Entry* e = find(h);
    if (!e || !buf || !done || e->kind == Kind::Listen ||
        e->kind == Kind::TcpConnecting)
        return TPLC_NET_E_STATE;
    if (cap == 0) { *done = 0; return TPLC_NET_OK; }
    ssize_t n = ::recv(e->fd, buf, cap, 0);
    if (n < 0) return mapErr(errno);
    *done = (uint32_t)n;   /* 0 = peer closed cleanly */
    return TPLC_NET_OK;
}

int32_t NetService::tcpClose(void* s, TplcNetHandle h) {
    (void)s;
    drop(h);
    return TPLC_NET_OK;
}

int32_t NetService::udpBind(void* s, const char* host, uint16_t port,
                            TplcNetHandle* out) {
    (void)s;
    if (!out || port == 0) return TPLC_NET_E_PARAM;
    struct in_addr a;
    if (host && *host && !parse_v4(host, &a)) return TPLC_NET_E_PARAM;
    int fd = ::socket(AF_INET, SOCK_DGRAM, 0);
    if (fd < 0) return TPLC_NET_E_SOCKET;
    if (nonblock(fd) < 0) { ::close(fd); return TPLC_NET_E_SOCKET; }
    struct sockaddr_in sa;
    make_addr(&sa, host, port, !(host && *host));
    if (host && *host) sa.sin_addr = a;
    if (::bind(fd, (struct sockaddr*)&sa, sizeof(sa)) < 0) {
        int e = errno;
        ::close(fd);
        return mapErr(e);
    }
    *out = add(Kind::Udp, fd, port);
    return TPLC_NET_OK;
}

int32_t NetService::udpSendTo(void* s, TplcNetHandle h, const char* host,
                              uint16_t port, const void* buf, uint32_t len,
                              uint32_t* done) {
    (void)s;
    Entry* e = find(h);
    struct in_addr a;
    if (!e || e->kind != Kind::Udp || !buf || !done || port == 0 ||
        !parse_v4(host, &a))
        return TPLC_NET_E_STATE;
    struct sockaddr_in sa;
    make_addr(&sa, host, port, false);
    sa.sin_addr = a;
    ssize_t n = ::sendto(e->fd, buf, len, 0,
                         (struct sockaddr*)&sa, sizeof(sa));
    if (n < 0) return mapErr(errno);
    *done = (uint32_t)n;
    return TPLC_NET_OK;
}

int32_t NetService::udpRecvFrom(void* s, TplcNetHandle h, void* buf,
                                uint32_t cap, char* from_host,
                                uint16_t* from_port, uint32_t* done) {
    (void)s;
    Entry* e = find(h);
    if (!e || e->kind != Kind::Udp || !buf || !done)
        return TPLC_NET_E_STATE;
    struct sockaddr_in peer;
    socklen_t pl = sizeof(peer);
    ssize_t n = ::recvfrom(e->fd, buf, cap, 0,
                           (struct sockaddr*)&peer, &pl);
    if (n < 0) return mapErr(errno);
    *done = (uint32_t)n;
    if (from_host && from_port) {
        inet_ntop(AF_INET, &peer.sin_addr, from_host, 16);
        *from_port = ntohs(peer.sin_port);
    }
    return TPLC_NET_OK;
}

int32_t NetService::udpClose(void* s, TplcNetHandle h) {
    (void)s;
    drop(h);
    return TPLC_NET_OK;
}

} /* namespace tplc */
