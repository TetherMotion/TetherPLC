/* MbusService.cpp — tplc-mbus wired M-Bus master: opt-in serial
 * port + real M-Bus framing (short 10 C A CS 16, long 68 L L 68 C A
 * CI data CS 16, ack E5) at 2400 8E1.  The serial port is only
 * opened when the surface is used.  All IO is bounded so a cyclic
 * scan can't stall the watchdog.  SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */

#include <tplc/host/MbusService.hpp>

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <poll.h>
#include <termios.h>
#include <unistd.h>
#include <vector>

namespace tplc {

namespace {

constexpr int kPollMs = 1500;

bool waitFd(int fd, short ev, int ms) {
    pollfd p{fd, ev, 0};
    return poll(&p, 1, ms) > 0 && (p.revents & (ev | POLLERR | POLLHUP));
}

bool sendAll(int fd, const uint8_t* p, size_t n) {
    size_t off = 0;
    while (off < n) {
        ssize_t w = write(fd, p + off, n - off);
        if (w > 0) { off += size_t(w); continue; }
        if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            if (!waitFd(fd, POLLOUT, kPollMs)) return false;
            continue;
        }
        return false;
    }
    return true;
}

bool recvN(int fd, uint8_t* p, size_t n) {
    size_t off = 0;
    while (off < n) {
        ssize_t r = read(fd, p + off, n - off);
        if (r > 0) { off += size_t(r); continue; }
        if (r < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            if (!waitFd(fd, POLLIN, kPollMs)) return false;
            continue;
        }
        return false;
    }
    return true;
}

uint8_t csum(const uint8_t* p, size_t n) {
    uint32_t s = 0;
    for (size_t i = 0; i < n; ++i) s += p[i];
    return uint8_t(s & 0xFF);
}

} // namespace

struct MbusService::Impl {
    struct Port {
        int fd = -1;
        int32_t state = 0;
        uint32_t tx = 0, rx = 0;
        int32_t last_error = 0;
    };
    Port ports[TPLC_MBUS_HDL_MAX]{};
};

MbusService::MbusService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcMbusService);
    svc_.abi_version = TPLC_MBUS_ABI_VERSION;
    svc_.host = this;
    svc_.mbus_open = [](void* s, const char* p, uint32_t* o)
        { return self(s)->open(s, p, o); };
    svc_.mbus_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.mbus_status = [](void* s, uint32_t h, TplcMbusStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.mbus_short = [](void* s, uint32_t h, uint32_t c, uint32_t a)
        { return self(s)->shortFrame(s, h, c, a); };
    svc_.mbus_long = [](void* s, uint32_t h, uint32_t c, uint32_t a,
                        uint32_t ci, const uint8_t* d, uint32_t n)
        { return self(s)->longFrame(s, h, c, a, ci, d, n); };
    svc_.mbus_receive = [](void* s, uint32_t h, uint32_t* k,
                           uint8_t* pl, uint32_t cap)
        { return self(s)->receive(s, h, k, pl, cap); };
}

MbusService::~MbusService() {
    for (auto& p : impl_->ports)
        if (p.fd >= 0) ::close(p.fd);
}

int32_t MbusService::open(void*, const char* path, uint32_t* out_handle) {
    if (!path || !*path || !out_handle) return TPLC_MBUS_E_PARAM;
    for (uint32_t i = 0; i < TPLC_MBUS_HDL_MAX; ++i) {
        if (impl_->ports[i].fd < 0) {
            /* opt-in: only now is the port touched */
            int fd = ::open(path, O_RDWR | O_NOCTTY | O_NONBLOCK);
            if (fd < 0) return TPLC_MBUS_E_NOSERIAL;
            /* M-Bus: 2400 8E1 (even parity, 1 stop) */
            termios tio{};
            cfsetispeed(&tio, B2400);
            cfsetospeed(&tio, B2400);
            tio.c_cflag = CS8 | PARENB | CREAD | CLOCAL;   /* 8E1 */
            tio.c_lflag = 0;
            tio.c_iflag = 0;
            tio.c_oflag = 0;
            tio.c_cc[VMIN]  = 0;
            tio.c_cc[VTIME] = 0;
            if (tcsetattr(fd, TCSANOW, &tio) != 0) {
                ::close(fd);
                return TPLC_MBUS_E_NOSERIAL;
            }
            auto& p = impl_->ports[i];
            p.fd = fd; p.state = 1; p.tx = 0; p.rx = 0; p.last_error = 0;
            *out_handle = i + 1;
            return TPLC_MBUS_OK;
        }
    }
    return TPLC_MBUS_E_HANDLE;
}

int32_t MbusService::close(void*, uint32_t h) {
    if (h == 0 || h > TPLC_MBUS_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_MBUS_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    ::close(p.fd); p.fd = -1; p.state = 0;
    return TPLC_MBUS_OK;
}

int32_t MbusService::status(void*, uint32_t h, TplcMbusStatus* out) {
    if (!out) return TPLC_MBUS_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_MBUS_HDL_MAX || impl_->ports[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_MBUS_OK;
    }
    const auto& p = impl_->ports[h - 1];
    out->state = p.state;
    out->tx_total = p.tx;
    out->rx_total = p.rx;
    out->last_error = p.last_error;
    return TPLC_MBUS_OK;
}

int32_t MbusService::shortFrame(void*, uint32_t h, uint32_t control,
                                uint32_t address) {
    if (h == 0 || h > TPLC_MBUS_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_MBUS_E_HANDLE;
    if (control > 0xFF || address > 0xFF) return TPLC_MBUS_E_PARAM;
    auto& p = impl_->ports[h - 1];
    uint8_t f[5] = {0x10, uint8_t(control), uint8_t(address), 0, 0x16};
    f[3] = csum(f + 1, 2);
    if (!sendAll(p.fd, f, sizeof(f))) {
        p.last_error = errno;
        return TPLC_MBUS_E_IO;
    }
    p.tx++;
    return TPLC_MBUS_OK;
}

int32_t MbusService::longFrame(void*, uint32_t h, uint32_t control,
                               uint32_t address, uint32_t ci,
                               const uint8_t* data, uint32_t len) {
    if (h == 0 || h > TPLC_MBUS_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_MBUS_E_HANDLE;
    if (control > 0xFF || address > 0xFF || ci > 0xFF || len > 252 ||
        (!data && len))
        return TPLC_MBUS_E_PARAM;
    auto& p = impl_->ports[h - 1];
    /* 68 L L 68 C A CI [data] CS 16 ; L = C+A+CI+data */
    std::vector<uint8_t> f;
    uint8_t L = uint8_t(3 + len);
    f.insert(f.end(), {0x68, L, L, 0x68});
    f.push_back(uint8_t(control));
    f.push_back(uint8_t(address));
    f.push_back(uint8_t(ci));
    f.insert(f.end(), data, data + len);
    f.push_back(csum(f.data() + 4, size_t(3 + len)));
    f.push_back(0x16);
    if (!sendAll(p.fd, f.data(), f.size())) {
        p.last_error = errno;
        return TPLC_MBUS_E_IO;
    }
    p.tx++;
    return TPLC_MBUS_OK;
}

int32_t MbusService::receive(void*, uint32_t h, uint32_t* out_kind,
                             uint8_t* payload, uint32_t cap) {
    if (h == 0 || h > TPLC_MBUS_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_MBUS_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    /* drain whatever's buffered nonblocking — an M-Bus response is
       an E5 ack or a 68 L L 68 ... CS 16 long frame */
    uint8_t buf[512];
    ssize_t r = read(p.fd, buf, sizeof(buf));
    if (r < 0 && (errno == EAGAIN || errno == EWOULDBLOCK))
        return TPLC_MBUS_E_NOMSG;
    if (r <= 0) { p.last_error = errno; return TPLC_MBUS_E_IO; }
    /* scan the burst for the first frame */
    for (ssize_t i = 0; i < r; ++i) {
        uint8_t b = buf[i];
        if (b == 0xE5) {
            if (out_kind) *out_kind = 0xE5;
            p.rx++;
            return TPLC_MBUS_OK;
        }
        if (b == 0x68 && i + 4 < r && buf[i + 3] == 0x68) {
            /* 68 L L 68 C A CI data CS 16 */
            uint8_t L = buf[i + 1];
            size_t need = size_t(i) + 4 + size_t(L) + 2;
            if (need > size_t(r)) {
                p.last_error = -3;             /* truncated frame */
                return TPLC_MBUS_E_IO;
            }
            /* payload = C A CI + data = L bytes after the second 68 */
            const uint8_t* body = buf + i + 4;
            if (out_kind) *out_kind = 0x68;
            uint32_t n = L < cap ? L : cap;
            if (payload && n) std::memcpy(payload, body, n);
            p.rx++;
            return int32_t(n);
        }
    }
    return TPLC_MBUS_E_NOMSG;
}

} // namespace tplc
