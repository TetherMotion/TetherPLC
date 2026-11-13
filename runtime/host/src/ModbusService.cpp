/* ModbusService.cpp — Modbus client service (TCP, UDP, RTU).
 * TCP/UDP: MBAP tid u16be, proto u16be=0, length u16be, unit u8;
 * then PDU func u8 + data (multi-byte fields big-endian).
 * RTU: unit u8 + PDU + CRC16 (little-endian), no MBAP.
 * One nonblocking transaction per handle; connect-per-transaction
 * like the TwinCAT FB_MB* blocks.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/ModbusService.hpp>

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <string.h>
#include <sys/socket.h>
#include <termios.h>
#include <time.h>
#include <unistd.h>

/* glibc <termios.h> does not export BOTHER (kernel UAPI c_cflag value
   0x1000) — required to select a literal baud rate via c_ispeed/
   c_ospeed for non-standard speeds */
#ifndef BOTHER
#define BOTHER 0x1000
#endif

namespace tplc {

namespace {
constexpr size_t MBAP_LEN = 7;
constexpr size_t MAX_ADU  = MBAP_LEN + 1 + TPLC_MODBUS_MAX_PDU;
constexpr size_t RTU_MAX_FRAME = 256; /* Modbus RTU ADU limit      */
constexpr uint16_t MB_PROTO = 0;
} /* namespace */

static speed_t rtuBaudOf(uint32_t baud);

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
    svc_.mb_begin_rtu = [](void* s, const TplcModbusRtuReq* r,
                           TplcModbusHandle* o) {
        return self(s)->mbBeginRtu(s, r, o);
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

int32_t ModbusService::checkRtuReq(const TplcModbusRtuReq* req) {
    if (!req || !req->dev[0] || req->func == 0 || req->func > 0x7f ||
        req->req_len > TPLC_MODBUS_MAX_PDU ||
        (req->req_len && !req->req) || req->parity > 2 ||
        req->databits < 5 || req->databits > 8 ||
        (req->stopbits != 1 && req->stopbits != 2) ||
        rtuBaudOf(req->baud) == 0)
        return TPLC_MB_E_PARAM;
    return TPLC_MB_OK;
}

/* unit | func | req data | crc16 (low byte first) */
void ModbusService::buildRtuFrame(Txn& t, const TplcModbusRtuReq* req) {
    t.unit = req->unit_id;
    t.func = req->func;
    t.baud = req->baud;
    t.tx.clear();
    t.tx.reserve(2 + req->req_len + 2);
    t.tx.push_back(req->unit_id);
    t.tx.push_back(req->func);
    if (req->req_len)
        t.tx.insert(t.tx.end(), req->req, req->req + req->req_len);
    uint16_t crc = crc16(t.tx.data(), t.tx.size());
    t.tx.push_back(crc & 0xff);
    t.tx.push_back((crc >> 8) & 0xff);
}

/* baud lookup shared with tplc-serial's termios setup; speed_t is
   unsigned int on Linux, 0 = unsupported */
static speed_t rtuBaudOf(uint32_t baud) {
    switch (baud) {
    case 50:     return B50;
    case 75:     return B75;
    case 110:    return B110;
    case 134:    return B134;
    case 150:    return B150;
    case 200:    return B200;
    case 300:    return B300;
    case 600:    return B600;
    case 1200:   return B1200;
    case 1800:   return B1800;
    case 2400:   return B2400;
    case 4800:   return B4800;
    case 9600:   return B9600;
    case 19200:  return B19200;
    case 38400:  return B38400;
    case 57600:  return B57600;
    case 115200: return B115200;
    case 230400: return B230400;
    case 460800: return B460800;
    case 921600: return B921600;
    default:     return static_cast<speed_t>(0);
    }
}

/* open the serial device with Modbus RTU termios settings; returns
   fd >= 0 or a negative TPLC_MB_* code */
static int openRtuSerial(const char* dev, uint32_t baud, uint8_t parity,
                         uint8_t databits, uint8_t stopbits) {
    int fd = ::open(dev, O_RDWR | O_NOCTTY | O_NONBLOCK);
    if (fd < 0) return TPLC_MB_E_CONNECT;
    struct termios tio;
    if (tcgetattr(fd, &tio) < 0) {
        ::close(fd);
        return TPLC_MB_E_IO;
    }
    cfmakeraw(&tio);
    tio.c_cflag &= ~(tcflag_t)(CBAUD | CSIZE | PARENB | PARODD | CSTOPB);
    speed_t sp = rtuBaudOf(baud);
    if (sp) {
        /* standard rate: the B* constant goes in the c_cflag baud field */
        tio.c_cflag |= sp;
    } else {
        /* non-standard rate: BOTHER selects c_ispeed/c_ospeed literally */
        tio.c_cflag |= (tcflag_t)BOTHER;
        tio.c_ispeed = (speed_t)baud;
        tio.c_ospeed = (speed_t)baud;
    }
    switch (databits) {
    case 5: tio.c_cflag |= CS5; break;
    case 6: tio.c_cflag |= CS6; break;
    case 7: tio.c_cflag |= CS7; break;
    default: tio.c_cflag |= CS8; break;
    }
    if (parity == 1) tio.c_cflag |= PARENB;
    if (parity == 2) tio.c_cflag |= PARENB | PARODD;
    if (stopbits == 2) tio.c_cflag |= CSTOPB;
    tio.c_cc[VMIN]  = 0;
    tio.c_cc[VTIME] = 0;
    if (tcsetattr(fd, TCSANOW, &tio) < 0) {
        ::close(fd);
        return TPLC_MB_E_IO;
    }
    return fd;
}

int32_t ModbusService::mbBeginRtu(void* s, const TplcModbusRtuReq* req,
                                  TplcModbusHandle* out) {
    (void)s;
    if (!out) return TPLC_MB_E_PARAM;
    int32_t v = checkRtuReq(req);
    if (v != TPLC_MB_OK) return v;
    int fd = openRtuSerial(req->dev, req->baud, req->parity,
                           req->databits, req->stopbits);
    if (fd < 0) return fd;

    Txn t;
    t.fd = fd;
    t.rtu = true;
    if (req->timeout_ms) {
        t.has_deadline = true;
        t.deadline = nowNs() + (uint64_t)req->timeout_ms * 1000000ull;
    }
    buildRtuFrame(t, req);

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

/* PDU tail shared by all transports: pdu points at the function
   byte, plen counts func+data bytes. Exception responses are normal
   completions reported via resp->status/exc. */
int32_t ModbusService::finishPdu(const Txn& t, const uint8_t* pdu,
                                 size_t plen, TplcModbusResp* resp) {
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
    return finishPdu(t, pdu, plen, resp);
}

/* CRC16-Modbus (poly 0xA001, init 0xFFFF) over the frame bytes */
uint16_t ModbusService::crc16(const uint8_t* p, size_t n) {
    uint16_t c = 0xFFFF;
    for (size_t i = 0; i < n; i++) {
        c ^= p[i];
        for (int b = 0; b < 8; b++)
            c = (c & 1) ? (uint16_t)((c >> 1) ^ 0xA001) : (uint16_t)(c >> 1);
    }
    return c;
}

bool ModbusService::rtuCrcOk(const uint8_t* f, size_t n) {
    if (n < 4) return false;
    return crc16(f, n - 2) == (uint16_t)(f[n - 2] | (f[n - 1] << 8));
}

/* T3.5 inter-frame gap scaled to the line speed, doubled for host
   scheduling jitter; used only for function codes whose response
   length is not fixed by the request. */
uint64_t ModbusService::rtuGapNs(const Txn& t) {
    uint64_t char_ns = t.baud ? 10000000000ull / t.baud : 1000000ull;
    uint64_t gap = char_ns * 7; /* 3.5 chars x 2 margin */
    return gap < 5000000ull ? 5000000ull : gap;
}

/* expected total RTU response size once the buffered bytes determine
   it; 0 = keep accumulating, SIZE_MAX = malformed */
size_t ModbusService::rtuNeed(const Txn& t) {
    const auto& rx = t.rx;
    if (rx.size() < 2) return 0;
    uint8_t f = rx[1];
    if (f == (uint8_t)(t.func | 0x80)) return 5; /* exception frame */
    if (f != t.func) return SIZE_MAX;
    switch (t.func) {
    case 1: case 2: case 3: case 4: case 23:
        /* addr|func|bytecount|data|crc — length known at 3 bytes */
        if (rx.size() < 3) return 0;
        {
            size_t need = 5 + rx[2];
            return need > RTU_MAX_FRAME ? SIZE_MAX : need;
        }
    case 5: case 6: case 15: case 16:
        return 8; /* addr echo + value/quantity, fixed size */
    default:
        return 0; /* unknown func: frame end by silence gap */
    }
}

/* RTU transaction: write the request frame, then accumulate
   response bytes until the expected length arrives (fixed and
   byte-count functions) or the inter-byte gap elapses (other
   functions). CRC is verified in rtuFinish. */
int32_t ModbusService::rtuDrive(Txn& t) {
    while (t.sent < t.tx.size()) {
        ssize_t n = ::write(t.fd, t.tx.data() + t.sent,
                            t.tx.size() - t.sent);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return TPLC_MB_BUSY;
            return TPLC_MB_E_IO;
        }
        t.sent += (size_t)n;
    }
    for (;;) {
        size_t need = rtuNeed(t);
        if (need == SIZE_MAX) return TPLC_MB_E_PROTO;
        if (need && t.rx.size() >= need) {
            if (!rtuCrcOk(t.rx.data(), need)) return TPLC_MB_E_PROTO;
            t.rx.resize(need);
            return TPLC_MB_OK;
        }
        if (t.rx.size() >= RTU_MAX_FRAME) return TPLC_MB_E_PROTO;
        /* gap-based completion for functions without a derivable
           length: only once a plausible frame (>=4 bytes) sits
           untouched for the inter-frame gap */
        if (!need && t.rx.size() >= 4 && t.last_rx_ns) {
            uint64_t now = nowNs();
            if (now > t.last_rx_ns && now - t.last_rx_ns > rtuGapNs(t)) {
                if (rtuCrcOk(t.rx.data(), t.rx.size()))
                    return TPLC_MB_OK;
                return TPLC_MB_E_PROTO;
            }
        }
        uint8_t buf[256];
        ssize_t n = ::read(t.fd, buf, sizeof(buf));
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return TPLC_MB_BUSY;
            return TPLC_MB_E_IO;
        }
        if (n == 0) return TPLC_MB_BUSY; /* VMIN=0: no data this scan */
        t.rx.insert(t.rx.end(), buf, buf + n);
        t.last_rx_ns = nowNs();
    }
}

/* complete RTU frame in rx: validate unit id + CRC, then hand the
   PDU (func + data) to the shared parser */
int32_t ModbusService::rtuFinish(Txn& t, TplcModbusResp* resp) {
    const uint8_t* f = t.rx.data();
    if (t.rx.size() < 4 || f[0] != t.unit || !rtuCrcOk(f, t.rx.size()))
        return TPLC_MB_E_PROTO;
    /* pdu = func byte + data, excluding addr and trailing crc */
    return finishPdu(t, f + 1, t.rx.size() - 3, resp);
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
    int32_t rc = t->rtu   ? rtuDrive(*t)
                : t->udp ? udpDrive(*t)
                         : drive(*t);
    if (rc == TPLC_MB_BUSY) return TPLC_MB_BUSY;
    int32_t frc = TPLC_MB_E_PROTO;
    if (rc == TPLC_MB_OK)
        frc = t->rtu ? rtuFinish(*t, resp) : finish(*t, resp);
    drop(h);
    return rc == TPLC_MB_OK ? frc : rc;
}

int32_t ModbusService::mbAbort(void* s, TplcModbusHandle h) {
    (void)s;
    drop(h);
    return TPLC_MB_OK;
}

} /* namespace tplc */
