/* ModbusService.cpp — Modbus TCP client service.
 * MBAP: tid u16be, proto u16be=0, length u16be, unit u8; then PDU:
 * func u8 + data (all multi-byte fields big-endian). One nonblocking
 * transaction per handle; connect-per-transaction like the TwinCAT
 * FB_MB* blocks.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/ModbusService.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

namespace tplc {

namespace {
constexpr size_t MBAP_LEN = 7;
constexpr size_t MAX_ADU  = MBAP_LEN + 1 + TPLC_MODBUS_MAX_PDU;
constexpr uint16_t MB_PROTO = 0;
} /* namespace */

static ModbusService* self(void* p) { return static_cast<ModbusService*>(p); }

ModbusService::ModbusService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size  = sizeof(TplcModbusService);
    svc_.abi_version  = TPLC_MODBUS_ABI_VERSION;
    svc_.host         = this;
    svc_.mb_begin     = [](void* s, const TplcModbusReq* r,
                           TplcModbusHandle* o) {
        return self(s)->mbBegin(s, r, o);
    };
    svc_.mb_poll      = [](void* s, TplcModbusHandle h,
                           TplcModbusResp* r) {
        return self(s)->mbPoll(s, h, r);
    };
    svc_.mb_abort     = [](void* s, TplcModbusHandle h) {
        return self(s)->mbAbort(s, h);
    };
    svc_.mb_begin_udp = [](void* s, const TplcModbusReq* r,
                           TplcModbusHandle* o) {
        return self(s)->mbBeginUdp(s, r, o);
    };
}

void ModbusService::put16be(std::vector<uint8_t>& v, uint16_t x) {
    v.push_back((x >> 8) & 0xff);
    v.push_back(x & 0xff);
}
uint16_t ModbusService::get16be(const uint8_t* p) {
    return (uint16_t)((p[0] << 8) | p[1]);
}

int32_t ModbusService::mapErr(int e) {
    switch (e) {
    case EAGAIN:
        return TPLC_MB_BUSY;
    case EBADF:
    case ENOTCONN:
    case ECONNRESET:
    case EPIPE:
        return TPLC_MB_E_CLOSED;
    case ECONNREFUSED:
    case EHOSTUNREACH:
    case ENETUNREACH:
        return TPLC_MB_E_CONNECT;
    case EINVAL:
        return TPLC_MB_E_PARAM;
    default:
        return TPLC_MB_E_IO;
    }
}

uint64_t ModbusService::nowNs() const {
    if (clock_) return clock_(clock_ctx_);
    struct timespec ts{};
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

ModbusService::Txn* ModbusService::find(TplcModbusHandle h) {
    auto it = txns_.find(h);
    return it == txns_.end() ? nullptr : &it->second;
}

void ModbusService::drop(TplcModbusHandle h) {
    auto it = txns_.find(h);
    if (it != txns_.end()) {
        if (it->second.fd >= 0) ::close(it->second.fd);
        txns_.erase(it);
    }
}

int32_t ModbusService::checkReq(const TplcModbusReq* req) {
    if (!req || !req->host[0] || req->port == 0 || req->func == 0 ||
        req->func > 0x7f || req->req_len > TPLC_MODBUS_MAX_PDU ||
        (req->req_len && !req->req))
        return TPLC_MB_E_PARAM;
    struct in_addr a4{};
    if (inet_pton(AF_INET, req->host, &a4) != 1) return TPLC_MB_E_PARAM;
    return TPLC_MB_OK;
}

/* MBAP + func + request data; tid/proto/unit/func/len filled here */
void ModbusService::buildAdu(Txn& t, const TplcModbusReq* req) {
    t.unit = req->unit_id;
    t.func = req->func;
    t.tx.reserve(MBAP_LEN + 1 + req->req_len);
    uint16_t tid = next_tid_++;
    if (tid == 0) tid = next_tid_++;
    t.tid = tid;
    put16be(t.tx, tid);
    put16be(t.tx, MB_PROTO);
    put16be(t.tx, (uint16_t)(1 + req->req_len + 1)); /* unit+pdu len */
    t.tx.push_back(req->unit_id);
    t.tx.push_back(req->func);
    if (req->req_len)
        t.tx.insert(t.tx.end(), req->req, req->req + req->req_len);
}

int32_t ModbusService::mbBegin(void* s, const TplcModbusReq* req,
                               TplcModbusHandle* out) {
    (void)s;
    if (!out) return TPLC_MB_E_PARAM;
    int32_t v = checkReq(req);
    if (v != TPLC_MB_OK) return v;
    struct in_addr a4{};
    inet_pton(AF_INET, req->host, &a4);

    int fd = ::socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if (fd < 0) return TPLC_MB_E_IO;
    int one = 1;
    setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, &one, sizeof(one));
    struct sockaddr_in sa{};
    sa.sin_family = AF_INET;
    sa.sin_port = htons(req->port);
    sa.sin_addr = a4;
    if (::connect(fd, (struct sockaddr*)&sa, sizeof(sa)) < 0 &&
        errno != EINPROGRESS) {
        int e = errno;
        ::close(fd);
        return mapErr(e);
    }

    Txn t;
    t.fd = fd;
    if (req->timeout_ms) {
        t.has_deadline = true;
        t.deadline = nowNs() + (uint64_t)req->timeout_ms * 1000000ull;
    }
    buildAdu(t, req);

    TplcModbusHandle h = next_handle_++;
    if (h == TPLC_MODBUS_HANDLE_INVALID) h = next_handle_++;
    txns_[h] = std::move(t);
    *out = h;
    return TPLC_MB_OK;
}

int32_t ModbusService::mbBeginUdp(void* s, const TplcModbusReq* req,
                                  TplcModbusHandle* out) {
    (void)s;
    if (!out) return TPLC_MB_E_PARAM;
    int32_t v = checkReq(req);
    if (v != TPLC_MB_OK) return v;
    struct in_addr a4{};
    inet_pton(AF_INET, req->host, &a4);

    int fd = ::socket(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK, 0);
    if (fd < 0) return TPLC_MB_E_IO;

    Txn t;
    t.fd = fd;
    t.udp = true;
    t.peer.sin_family = AF_INET;
    t.peer.sin_port = htons(req->port);
    t.peer.sin_addr = a4;
    if (req->timeout_ms) {
        t.has_deadline = true;
        t.deadline = nowNs() + (uint64_t)req->timeout_ms * 1000000ull;
    }
    buildAdu(t, req);

    TplcModbusHandle h = next_handle_++;
    if (h == TPLC_MODBUS_HANDLE_INVALID) h = next_handle_++;
    txns_[h] = std::move(t);
    *out = h;
    return TPLC_MB_OK;
}

/* nonblocking connect check; TPLC_MB_OK once connected */
static int32_t connState(int fd) {
    int err = 0;
    socklen_t l = sizeof(err);
    if (getsockopt(fd, SOL_SOCKET, SO_ERROR, &err, &l) < 0)
        return TPLC_MB_E_IO;
    if (err == 0) return TPLC_MB_OK;
    if (err == EINPROGRESS || err == EALREADY) return TPLC_MB_BUSY;
    if (err == ECONNREFUSED || err == EHOSTUNREACH || err == ENETUNREACH)
        return TPLC_MB_E_CONNECT;
    return TPLC_MB_E_IO;
}

int32_t ModbusService::drive(Txn& t) {
    if (t.cstate == 0) {
        int32_t rc = connState(t.fd);
        if (rc != TPLC_MB_OK) return rc;
        t.cstate = 1;
    }
    while (t.sent < t.tx.size()) {
        ssize_t n = ::send(t.fd, t.tx.data() + t.sent,
                           t.tx.size() - t.sent, MSG_NOSIGNAL);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return TPLC_MB_BUSY;
            return mapErr(errno);
        }
        t.sent += (size_t)n;
    }
    for (;;) {
        /* once the MBAP length is known, wait for the full ADU */
        if (t.rx.size() >= MBAP_LEN) {
            uint16_t mlen = get16be(t.rx.data() + 4);
            if (mlen < 2 || mlen > 1 + 1 + TPLC_MODBUS_MAX_PDU)
                return TPLC_MB_E_PROTO;
            size_t need = MBAP_LEN + (size_t)mlen - 1;
            if (t.rx.size() >= need) return TPLC_MB_OK;
        }
        if (t.rx.size() >= MAX_ADU) return TPLC_MB_E_PROTO;
        uint8_t buf[260];
        ssize_t n = ::recv(t.fd, buf, sizeof(buf), 0);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return TPLC_MB_BUSY;
            return mapErr(errno);
        }
        if (n == 0) return TPLC_MB_E_CLOSED;
        t.rx.insert(t.rx.end(), buf, buf + n);
    }
}

/* response ADU is complete: validate and fill resp. Returns
   TPLC_MB_OK always (exception is reported via resp->status). */
int32_t ModbusService::finish(Txn& t, TplcModbusResp* resp) {
    const uint8_t* adu = t.rx.data();
    uint16_t rtid = get16be(adu);
    uint16_t rproto = get16be(adu + 2);
    uint16_t mlen = get16be(adu + 4);
    uint8_t runit = adu[6];
    if (rtid != t.tid || rproto != MB_PROTO || runit != t.unit ||
        mlen < 2)
        return TPLC_MB_E_PROTO;
    const uint8_t* pdu = adu + MBAP_LEN;
    size_t plen = (size_t)mlen - 1; /* bytes after unit id */
    if (plen < 1) return TPLC_MB_E_PROTO;
    uint8_t rfunc = pdu[0];
    resp->status = 0;
    resp->exc = 0;
    resp->len = 0;
    if (rfunc == (uint8_t)(t.func | 0x80)) {
        if (plen != 2) return TPLC_MB_E_PROTO;
        resp->status = 1;
        resp->exc = pdu[1];
        return TPLC_MB_OK;
    }
    if (rfunc != t.func) return TPLC_MB_E_PROTO;
    size_t dlen = plen - 1;
    if (dlen > resp->cap) return TPLC_MB_E_RANGE;
    if (dlen && resp->data) memcpy(resp->data, pdu + 1, dlen);
    resp->len = (uint16_t)dlen;
    return TPLC_MB_OK;
}

/* UDP transaction: one request datagram, then response datagrams
   until one arrives from the request peer. The whole datagram is
   the ADU; finish() validates MBAP/PDU fields as for TCP. */
int32_t ModbusService::udpDrive(Txn& t) {
    if (!t.sent) {
        ssize_t n = ::sendto(t.fd, t.tx.data(), t.tx.size(), MSG_NOSIGNAL,
                             (struct sockaddr*)&t.peer, sizeof(t.peer));
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return TPLC_MB_BUSY;
            return mapErr(errno);
        }
        t.sent = 1;
    }
    for (;;) {
        uint8_t buf[512];
        struct sockaddr_in from{};
        socklen_t fl = sizeof(from);
        ssize_t n = ::recvfrom(t.fd, buf, sizeof(buf), MSG_DONTWAIT,
                               (struct sockaddr*)&from, &fl);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return TPLC_MB_BUSY;
            return mapErr(errno);
        }
        if (from.sin_addr.s_addr != t.peer.sin_addr.s_addr ||
            from.sin_port != t.peer.sin_port)
            continue;                       /* stray datagram */
        t.rx.assign(buf, buf + n);
        if (t.rx.size() < MBAP_LEN) return TPLC_MB_E_PROTO;
        /* the datagram must cover the MBAP-declared ADU length;
           trailing bytes beyond it are ignored */
        uint16_t mlen = get16be(t.rx.data() + 4);
        size_t need = MBAP_LEN + (size_t)mlen - 1;
        if (mlen < 2 || t.rx.size() < need) return TPLC_MB_E_PROTO;
        t.rx.resize(need);
        return TPLC_MB_OK;
    }
}

int32_t ModbusService::mbPoll(void* s, TplcModbusHandle h,
                              TplcModbusResp* resp) {
    (void)s;
    if (!resp) return TPLC_MB_E_PARAM;
    Txn* t = find(h);
    if (!t) return TPLC_MB_E_STATE;
    if (t->has_deadline && nowNs() > t->deadline) {
        drop(h);
        return TPLC_MB_E_TIMEOUT;
    }
    /* drive() returns BUSY on EAGAIN at any stage, OK only once the
       full response ADU is buffered, negative on transport/proto
       errors. */
    int32_t rc = t->udp ? udpDrive(*t) : drive(*t);
    if (rc == TPLC_MB_BUSY) return TPLC_MB_BUSY;
    int32_t frc = TPLC_MB_E_PROTO;
    if (rc == TPLC_MB_OK) frc = finish(*t, resp);
    drop(h);
    return rc == TPLC_MB_OK ? frc : rc;
}

int32_t ModbusService::mbAbort(void* s, TplcModbusHandle h) {
    (void)s;
    drop(h);
    return TPLC_MB_OK;
}

} /* namespace tplc */
