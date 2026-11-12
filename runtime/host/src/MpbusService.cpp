/* MpbusService.cpp — tplc-mpbus: MP-Bus master over an opt-in
 * serial adapter (9600 8N1).  Frame format:
 *   [DA][CMD][LEN][data(LEN)][CHK]   CHK = XOR of bytes DA..data
 * Bounded and nonblocking — a PLC scan is never held; replies
 * accumulate across polls.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#include <tplc/host/MpbusService.hpp>

#include <cstring>
#include <vector>
#include <cerrno>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>

namespace tplc {
namespace {

constexpr uint32_t kHdlMax = TPLC_MPBUS_HDL_MAX;

uint8_t chk8(const uint8_t* d, size_t n) {
    uint8_t c = 0;
    for (size_t i = 0; i < n; ++i) c ^= d[i];
    return c;
}

struct Port {
    int fd = -1;
    int state = 0;
    uint32_t tx = 0, rx = 0;
    int32_t last_error = 0;
    uint8_t rxbuf[512];
    size_t rxlen = 0;
};

/* pull one complete frame out of the buffer, or return -1 */
int32_t pop_frame(Port& p, uint32_t* out_da, uint32_t* out_cmd,
                  uint8_t* payload, uint32_t cap) {
    for (size_t i = 0; i + 3 <= p.rxlen; ) {
        uint8_t len = p.rxbuf[i + 2];
        if (len > 250) { ++i; continue; }       /* implausible — resync */
        size_t need = i + 4 + size_t(len);
        if (need > p.rxlen) return -1;           /* wait for more */
        const uint8_t* f = p.rxbuf + i;
        if (chk8(f, 3 + len) != f[3 + len]) { ++i; continue; }
        if (out_da)  *out_da = f[0];
        if (out_cmd) *out_cmd = f[1];
        uint32_t n = len < cap ? len : cap;
        if (payload && n) std::memcpy(payload, f + 3, n);
        std::memmove(p.rxbuf, p.rxbuf + need, p.rxlen - need);
        p.rxlen -= need;
        p.rx++;
        return int32_t(n);
    }
    return -1;
}

} /* namespace */

struct MpbusService::Impl {
    Port ports[kHdlMax];
};

MpbusService::MpbusService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcMpbusService);
    svc_.abi_version = TPLC_MPBUS_ABI_VERSION;
    svc_.host = this;
    svc_.mpbus_open = [](void* s, const char* p, uint32_t* o)
        { return self(s)->open(s, p, o); };
    svc_.mpbus_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.mpbus_status = [](void* s, uint32_t h, TplcMpbusStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.mpbus_transact = [](void* s, uint32_t h, uint32_t da,
                             uint32_t c, const uint8_t* d, uint32_t n,
                             uint32_t* oda, uint32_t* oc,
                             uint8_t* r, uint32_t rc)
        { return self(s)->transact(s, h, da, c, d, n, oda, oc, r, rc); };
    svc_.mpbus_receive = [](void* s, uint32_t h, uint32_t* da,
                            uint32_t* c, uint8_t* p, uint32_t n)
        { return self(s)->receive(s, h, da, c, p, n); };
}

MpbusService::~MpbusService() {
    for (auto& p : impl_->ports)
        if (p.fd >= 0) ::close(p.fd);
}

int32_t MpbusService::open(void*, const char* path,
                           uint32_t* out_handle) {
    if (!path || !*path || !out_handle) return TPLC_MPBUS_E_PARAM;
    for (uint32_t i = 0; i < kHdlMax; ++i) {
        if (impl_->ports[i].fd < 0) {
            int fd = ::open(path, O_RDWR | O_NOCTTY | O_NONBLOCK);
            if (fd < 0) return TPLC_MPBUS_E_NOSERIAL;
            /* MP-Bus adapter UART: 9600 8N1 */
            termios tio{};
            cfsetispeed(&tio, B9600);
            cfsetospeed(&tio, B9600);
            tio.c_cflag = CS8 | CREAD | CLOCAL;
            tio.c_lflag = 0;
            tio.c_iflag = 0;
            tio.c_oflag = 0;
            tio.c_cc[VMIN]  = 0;
            tio.c_cc[VTIME] = 0;
            if (tcsetattr(fd, TCSANOW, &tio) != 0) {
                ::close(fd);
                return TPLC_MPBUS_E_OPEN;
            }
            auto& p = impl_->ports[i];
            p.fd = fd; p.state = 1; p.tx = 0; p.rx = 0;
            p.last_error = 0; p.rxlen = 0;
            *out_handle = i + 1;
            return TPLC_MPBUS_OK;
        }
    }
    return TPLC_MPBUS_E_HANDLE;
}

int32_t MpbusService::close(void*, uint32_t h) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_MPBUS_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    ::close(p.fd); p.fd = -1; p.state = 0;
    return TPLC_MPBUS_OK;
}

int32_t MpbusService::status(void*, uint32_t h,
                             TplcMpbusStatus* out) {
    if (!out) return TPLC_MPBUS_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_MPBUS_OK;
    }
    const auto& p = impl_->ports[h - 1];
    out->state = p.state;
    out->tx_total = p.tx;
    out->rx_total = p.rx;
    out->last_error = p.last_error;
    return TPLC_MPBUS_OK;
}

int32_t MpbusService::receive(void*, uint32_t h, uint32_t* out_da,
                              uint32_t* out_cmd, uint8_t* payload,
                              uint32_t cap) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_MPBUS_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    /* accumulate, then always scan buffered bytes */
    uint8_t tmp[256];
    ssize_t r = read(p.fd, tmp, sizeof(tmp));
    if (r < 0 && errno != EAGAIN && errno != EWOULDBLOCK) {
        p.last_error = errno;
        return TPLC_MPBUS_E_IO;
    }
    if (r > 0) {
        if (p.rxlen + size_t(r) > sizeof(p.rxbuf)) p.rxlen = 0;
        std::memcpy(p.rxbuf + p.rxlen, tmp, size_t(r));
        p.rxlen += size_t(r);
    }
    int32_t n = pop_frame(p, out_da, out_cmd, payload, cap);
    return n >= 0 ? n : TPLC_MPBUS_E_NOMSG;
}

int32_t MpbusService::transact(void*, uint32_t h, uint32_t da,
                               uint32_t cmd, const uint8_t* data,
                               uint32_t len,
                               uint32_t* out_da, uint32_t* out_cmd,
                               uint8_t* resp, uint32_t cap) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_MPBUS_E_HANDLE;
    if (da > 0xFF || cmd > 0xFF || len > 250 || (!data && len))
        return TPLC_MPBUS_E_PARAM;
    auto& p = impl_->ports[h - 1];
    /* request: [DA][CMD][LEN][data][CHK] */
    std::vector<uint8_t> f;
    f.push_back(uint8_t(da));
    f.push_back(uint8_t(cmd));
    f.push_back(uint8_t(len));
    f.insert(f.end(), data, data + len);
    f.push_back(chk8(f.data(), f.size()));

    size_t off = 0;
    while (off < f.size()) {
        ssize_t w = write(p.fd, f.data() + off, f.size() - off);
        if (w > 0) { off += size_t(w); continue; }
        p.last_error = errno;
        return TPLC_MPBUS_E_IO;
    }
    p.tx++;
    /* poll once — the reply may already be in flight */
    return receive(nullptr, h, out_da, out_cmd, resp, cap);
}

} /* namespace tplc */
