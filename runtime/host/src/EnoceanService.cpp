/* EnoceanService.cpp — tplc-enocean ESP3 serial service.
 * Opt-in serial port (57600 8N1), EnOcean Serial Protocol 3
 * framing: 0x55 DLEN_hi DLEN_lo OLEN PTYPE CRC8H <data> <opt>
 * CRC8D.  CRC8 is EnOcean's table/polynomial 0x07 init 0x00.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#include <tplc/host/EnoceanService.hpp>

#include <cerrno>
#include <cstring>
#include <termios.h>
#include <fcntl.h>
#include <unistd.h>
#include <vector>

namespace tplc {
namespace {

constexpr uint32_t kHdlMax = TPLC_ENOCEAN_HDL_MAX;

struct Port {
    int fd = -1;
    int state = 0;
    uint32_t tx = 0, rx = 0;
    int last_error = 0;
    uint8_t rxbuf[1024];
    size_t rxlen = 0;
};

/* EnOcean CRC8 — polynomial 0x07, init 0x00 */
uint8_t crc8(const uint8_t* p, size_t n) {
    uint8_t crc = 0;
    for (size_t i = 0; i < n; ++i) {
        crc ^= p[i];
        for (int b = 0; b < 8; ++b)
            crc = (crc & 0x80) ? uint8_t((crc << 1) ^ 0x07)
                               : uint8_t(crc << 1);
    }
    return crc;
}

} /* namespace */

struct EnoceanService::Impl {
    Port ports[kHdlMax];
};

EnoceanService::EnoceanService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcEnoceanService);
    svc_.abi_version = TPLC_ENOCEAN_ABI_VERSION;
    svc_.host = this;
    svc_.enocean_open = [](void* s, const char* p, uint32_t* o)
        { return self(s)->open(s, p, o); };
    svc_.enocean_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.enocean_status = [](void* s, uint32_t h, TplcEnoceanStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.enocean_send_radio = [](void* s, uint32_t h, uint32_t o,
                                 uint32_t se, uint32_t d,
                                 const uint8_t* p, uint32_t n)
        { return self(s)
              ->sendRadio(s, h, o, se, d, p, n); };
    svc_.enocean_receive = [](void* s, uint32_t h, uint32_t* pt,
                              uint32_t* o, uint32_t* se,
                              uint8_t* p, uint32_t c)
        { return self(s)
              ->receive(s, h, pt, o, se, p, c); };
}

EnoceanService::~EnoceanService() {
    for (auto& p : impl_->ports)
        if (p.fd >= 0) ::close(p.fd);
}

int32_t EnoceanService::open(void*, const char* path,
                             uint32_t* out_handle) {
    if (!path || !*path || !out_handle) return TPLC_ENOCEAN_E_PARAM;
    for (uint32_t i = 0; i < kHdlMax; ++i) {
        if (impl_->ports[i].fd < 0) {
            int fd = ::open(path, O_RDWR | O_NOCTTY | O_NONBLOCK);
            if (fd < 0) return TPLC_ENOCEAN_E_NOSERIAL;
            /* EnOcean UART: 57600 8N1 */
            termios tio{};
            cfsetispeed(&tio, B57600);
            cfsetospeed(&tio, B57600);
            tio.c_cflag = CS8 | CREAD | CLOCAL;
            tio.c_lflag = 0;
            tio.c_iflag = 0;
            tio.c_oflag = 0;
            tio.c_cc[VMIN]  = 0;
            tio.c_cc[VTIME] = 0;
            if (tcsetattr(fd, TCSANOW, &tio) != 0) {
                ::close(fd);
                return TPLC_ENOCEAN_E_NOSERIAL;
            }
            auto& p = impl_->ports[i];
            p.fd = fd; p.state = 1; p.tx = 0; p.rx = 0;
            p.last_error = 0; p.rxlen = 0;
            *out_handle = i + 1;
            return TPLC_ENOCEAN_OK;
        }
    }
    return TPLC_ENOCEAN_E_HANDLE;
}

int32_t EnoceanService::close(void*, uint32_t h) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_ENOCEAN_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    ::close(p.fd); p.fd = -1; p.state = 0;
    return TPLC_ENOCEAN_OK;
}

int32_t EnoceanService::status(void*, uint32_t h,
                               TplcEnoceanStatus* out) {
    if (!out) return TPLC_ENOCEAN_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_ENOCEAN_OK;
    }
    const auto& p = impl_->ports[h - 1];
    out->state = p.state;
    out->tx_total = p.tx;
    out->rx_total = p.rx;
    out->last_error = p.last_error;
    return TPLC_ENOCEAN_OK;
}

int32_t EnoceanService::sendRadio(void*, uint32_t h, uint32_t org,
                                  uint32_t sender, uint32_t dest,
                                  const uint8_t* data, uint32_t len) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_ENOCEAN_E_HANDLE;
    if (org > 0xFF || len > 200 || (!data && len))
        return TPLC_ENOCEAN_E_PARAM;
    auto& p = impl_->ports[h - 1];
    /* ERP1 radio: [RORG data... sender(4) status(1)] + opt
       [dest(4) dBm security] */
    std::vector<uint8_t> d, opt;
    d.push_back(uint8_t(org));
    d.insert(d.end(), data, data + len);
    for (int i = 3; i >= 0; --i) d.push_back(uint8_t(sender >> (i * 8)));
    d.push_back(0x00);                          /* status */
    for (int i = 3; i >= 0; --i) opt.push_back(uint8_t(dest >> (i * 8)));
    opt.push_back(0x30);                        /* dBm */
    opt.push_back(0x00);                        /* security */

    std::vector<uint8_t> f;
    f.push_back(0x55);
    f.push_back(uint8_t(d.size() >> 8));
    f.push_back(uint8_t(d.size()));
    f.push_back(uint8_t(opt.size()));
    f.push_back(TPLC_ENOCEAN_PT_RADIO);
    f.push_back(crc8(f.data() + 1, 4));
    std::vector<uint8_t> dopt = d;
    dopt.insert(dopt.end(), opt.begin(), opt.end());
    f.insert(f.end(), dopt.begin(), dopt.end());
    f.push_back(crc8(dopt.data(), dopt.size()));

    size_t off = 0;
    while (off < f.size()) {
        ssize_t w = write(p.fd, f.data() + off, f.size() - off);
        if (w > 0) { off += size_t(w); continue; }
        if (w < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            p.last_error = errno;
            return TPLC_ENOCEAN_E_IO;
        }
        p.last_error = errno;
        return TPLC_ENOCEAN_E_IO;
    }
    p.tx++;
    return TPLC_ENOCEAN_OK;
}

int32_t EnoceanService::receive(void*, uint32_t h, uint32_t* out_ptype,
                                uint32_t* out_org, uint32_t* out_sender,
                                uint8_t* payload, uint32_t cap) {
    if (h == 0 || h > kHdlMax || impl_->ports[h - 1].fd < 0)
        return TPLC_ENOCEAN_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    /* accumulate: a telegram may split across reads, and one read
       may carry several packets — always scan buffered data, even
       when this poll returns no new bytes */
    uint8_t tmp[256];
    ssize_t r = read(p.fd, tmp, sizeof(tmp));
    if (r < 0 && errno != EAGAIN && errno != EWOULDBLOCK) {
        p.last_error = errno;
        return TPLC_ENOCEAN_E_IO;
    }
    /* VMIN=0/VTIME=0 raw mode: empty read returns 0, not EAGAIN */
    if (r > 0) {
        if (p.rxlen + size_t(r) > sizeof(p.rxbuf)) p.rxlen = 0;
        std::memcpy(p.rxbuf + p.rxlen, tmp, size_t(r));
        p.rxlen += size_t(r);
    }

    /* scan for 0x55 + complete packet */
    for (size_t i = 0; i + 6 <= p.rxlen; ++i) {
        if (p.rxbuf[i] != 0x55) continue;
        uint16_t dlen = (uint16_t(p.rxbuf[i + 1]) << 8) | p.rxbuf[i + 2];
        uint8_t olen = p.rxbuf[i + 3];
        uint8_t ptype = p.rxbuf[i + 4];
        size_t need = i + 6 + size_t(dlen) + size_t(olen) + 1;
        if (need > p.rxlen) break;               /* wait for more */
        if (crc8(p.rxbuf + i + 1, 4) != p.rxbuf[i + 5] ||
            crc8(p.rxbuf + i + 6, dlen + olen) !=
                p.rxbuf[i + 6 + dlen + olen]) {
            continue;                            /* bad crc — resync */
        }
        const uint8_t* d = p.rxbuf + i + 6;
        if (out_ptype) *out_ptype = ptype;
        uint32_t n = 0;
        if (ptype == TPLC_ENOCEAN_PT_RADIO && dlen >= 6) {
            /* [RORG payload... sender(4) status(1)] */
            if (out_org) *out_org = d[0];
            uint32_t sid = 0;
            for (int k = 0; k < 4; ++k)
                sid = (sid << 8) | d[dlen - 5 + k];
            if (out_sender) *out_sender = sid;
            uint32_t plen = dlen - 6;            /* RORG+sender+status */
            n = plen < cap ? plen : cap;
            if (payload && n) std::memcpy(payload, d + 1, n);
        } else {
            n = dlen < cap ? dlen : cap;
            if (payload && n) std::memcpy(payload, d, n);
        }
        std::memmove(p.rxbuf, p.rxbuf + need, p.rxlen - need);
        p.rxlen -= need;
        p.rx++;
        return int32_t(n);
    }
    return TPLC_ENOCEAN_E_NOMSG;
}

} /* namespace tplc */
