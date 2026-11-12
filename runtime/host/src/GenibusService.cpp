/* GenibusService.cpp — tplc-genibus: GENIbus telegrams over an
 * opt-in serial port (9600 8E1).  GENIbus framing:
 *   SD(0x27) LE LEr DA SA [APDU..] CRChi CRClo
 * LE counts DA+SA+APDU; LEr repeats LE.  CRC-16 (CCITT, poly
 * 0x1021, init 0xFFFF) covers LE..end-of-APDU.  Bounded and
 * nonblocking — cyclic PLC tasks must never stall on the bus.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#include <tplc/host/GenibusService.hpp>

#include <cstring>
#include <vector>
#include <cerrno>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>

namespace tplc {
namespace {

constexpr uint32_t kHdlMax = TPLC_GENIBUS_HDL_MAX;

uint16_t crc16(const uint8_t* d, size_t n) {
    uint16_t crc = 0xFFFF;
    for (size_t i = 0; i < n; ++i) {
        crc ^= uint16_t(d[i]) << 8;
        for (int k = 0; k < 8; ++k)
            crc = (crc & 0x8000) ? uint16_t((crc << 1) ^ 0x1021)
                                 : uint16_t(crc << 1);
    }
    return crc;
}

struct Port {
    int fd = -1;
    int state = 0;
    uint32_t tx = 0, rx = 0;
    int32_t last_error = 0;
    /* rx accumulation — a telegram may split across reads, and
       one read may carry several telegrams */
    uint8_t rxbuf[512];
    size_t rxlen = 0;
};

} /* namespace */

struct GenibusService::Impl {
    Port ports[kHdlMax];
};

GenibusService::GenibusService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcGenibusService);
    svc_.abi_version = TPLC_GENIBUS_ABI_VERSION;
    svc_.host = this;
    svc_.genibus_open = [](void* s, const char* p, uint32_t* o)
        { return self(s)->open(s, p, o); };
    svc_.genibus_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.genibus_status = [](void* s, uint32_t h, TplcGenibusStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.genibus_send = [](void* s, uint32_t h, uint32_t da,
                           uint32_t sa, const uint8_t* p, uint32_t n)
        { return self(s)->send(s, h, da, sa, p, n); };
    svc_.genibus_receive = [](void* s, uint32_t h, uint32_t* da,
                            uint32_t* sa, uint8_t* p, uint32_t c)
        { return self(s)->receive(s, h, da, sa, p, c); };
}

GenibusService::~GenibusService() {
    for (auto& p : impl_->ports)
        if (p.fd >= 0) ::close(p.fd);
}

int32_t GenibusService::open(void*, const char* path,
                             uint32_t* out_handle) {
    if (!path || !*path || !out_handle) return TPLC_GENIBUS_E_PARAM;
    for (uint32_t i = 0; i < kHdlMax; ++i) {
        if (impl_->ports[i].fd < 0) {
            int fd = ::open(path, O_RDWR | O_NOCTTY | O_NONBLOCK);
            if (fd < 0) return TPLC_GENIBUS_E_NOSERIAL;
            /* GENIbus UART: 9600 8E1 */
            termios tio{};
            cfsetispeed(&tio, B9600);
            cfsetospeed(&tio, B9600);
            tio.c_cflag = CS8 | CREAD | CLOCAL | PARENB;
            tio.c_lflag = 0;
            tio.c_iflag = 0;
            tio.c_oflag = 0;
            tio.c_cc[VMIN]  = 0;
            tio.c_cc[VTIME] = 0;
            if (tcsetattr(fd, TCSANOW, &tio) != 0) {
                ::close(fd);
                return TPLC_GENIBUS_E_OPEN;
            }
            auto& p = impl_->ports[i];
            p.fd = fd; p.state = 1; p.tx = 0; p.rx = 0;
            p.last_error = 0; p.rxlen = 0;
            *out_handle = i + 1;
            return TPLC_GENIBUS_OK;
        }
    }
    return TPLC_GENIBUS_E_HANDLE;
}

int32_t GenibusService::close(void*, uint32_t h) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_GENIBUS_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    ::close(p.fd); p.fd = -1; p.state = 0;
    return TPLC_GENIBUS_OK;
}

int32_t GenibusService::status(void*, uint32_t h,
                               TplcGenibusStatus* out) {
    if (!out) return TPLC_GENIBUS_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_GENIBUS_OK;
    }
    const auto& p = impl_->ports[h - 1];
    out->state = p.state;
    out->tx_total = p.tx;
    out->rx_total = p.rx;
    out->last_error = p.last_error;
    return TPLC_GENIBUS_OK;
}

int32_t GenibusService::send(void*, uint32_t h, uint32_t da,
                             uint32_t sa, const uint8_t* apdu,
                             uint32_t len) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_GENIBUS_E_HANDLE;
    if (da > 0xFF || sa > 0xFF || len > 248 || (!apdu && len))
        return TPLC_GENIBUS_E_PARAM;
    auto& p = impl_->ports[h - 1];
    /* SD 0x27 | LE(=DA+SA+APDU len) | LEr | DA | SA | APDU | CRC16
       (CCITT over LE..end of APDU) hi lo */
    std::vector<uint8_t> f;
    uint8_t le = uint8_t(2 + len);
    f.push_back(0x27);
    f.push_back(le);
    f.push_back(le);
    f.push_back(uint8_t(da));
    f.push_back(uint8_t(sa));
    f.insert(f.end(), apdu, apdu + len);
    uint16_t crc = crc16(f.data() + 1, f.size() - 1);
    f.push_back(uint8_t(crc >> 8));
    f.push_back(uint8_t(crc & 0xFF));

    size_t off = 0;
    while (off < f.size()) {
        ssize_t w = write(p.fd, f.data() + off, f.size() - off);
        if (w > 0) { off += size_t(w); continue; }
        if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            p.last_error = errno;
            return TPLC_GENIBUS_E_IO;
        }
        p.last_error = errno;
        return TPLC_GENIBUS_E_IO;
    }
    p.tx++;
    return TPLC_GENIBUS_OK;
}

int32_t GenibusService::receive(void*, uint32_t h, uint32_t* out_da,
                                uint32_t* out_sa, uint8_t* payload,
                                uint32_t cap) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_GENIBUS_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    /* accumulate, then always scan buffered bytes — one read may
       carry several telegrams */
    uint8_t tmp[256];
    ssize_t r = read(p.fd, tmp, sizeof(tmp));
    if (r < 0 && errno != EAGAIN && errno != EWOULDBLOCK) {
        p.last_error = errno;
        return TPLC_GENIBUS_E_IO;
    }
    /* VMIN=0/VTIME=0 raw mode: empty read returns 0, not EAGAIN */
    if (r > 0) {
        if (p.rxlen + size_t(r) > sizeof(p.rxbuf)) p.rxlen = 0;
        std::memcpy(p.rxbuf + p.rxlen, tmp, size_t(r));
        p.rxlen += size_t(r);
    }

    /* scan for 0x27 LE LEr + complete telegram */
    for (size_t i = 0; i + 5 <= p.rxlen; ++i) {
        if (p.rxbuf[i] != 0x27) continue;
        uint8_t le = p.rxbuf[i + 1];
        if (p.rxbuf[i + 2] != le) continue;      /* LEr mismatch — resync */
        if (le < 2 || le > 250) continue;
        size_t need = i + 3 + size_t(le) + 2;    /* SD LE LEr [le] crc2 */
        if (need > p.rxlen) break;               /* wait for more */
        if (crc16(p.rxbuf + i + 1, 2 + le) !=
            (uint16_t(p.rxbuf[i + 3 + le]) << 8 | p.rxbuf[i + 4 + le])) {
            continue;                            /* bad crc — resync */
        }
        const uint8_t* d = p.rxbuf + i + 3;      /* DA SA APDU.. */
        if (out_da) *out_da = d[0];
        if (out_sa) *out_sa = d[1];
        uint32_t n = uint32_t(le) - 2;           /* APDU len */
        if (n > cap) n = cap;
        if (payload && n) std::memcpy(payload, d + 2, n);
        std::memmove(p.rxbuf, p.rxbuf + need, p.rxlen - need);
        p.rxlen -= need;
        p.rx++;
        return int32_t(uint32_t(le) - 2 < cap ? uint32_t(le) - 2 : cap);
    }
    return TPLC_GENIBUS_E_NOMSG;
}

} /* namespace tplc */
