/* DmxService.cpp — tplc-dmx DMX512 output: opt-in serial port +
 * real DMX512 frame emission (break + MAB + start code + 512
 * channel bytes at 250 kbaud).  The service opens the UART only
 * when the surface is used — no port is touched until dmx_open.
 * All IO is bounded so a cyclic scan can't stall the watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/DmxService.hpp>

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <poll.h>
#include <sys/ioctl.h>
#include <sys/stat.h>
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

} // namespace

struct DmxService::Impl {
    struct Port {
        int fd = -1;
        int32_t state = 0;
        uint32_t frames = 0;
        int32_t last_error = 0;
        /* start code + 512 channels */
        uint8_t data[513]{};
    };
    Port ports[TPLC_DMX_HDL_MAX]{};
};

DmxService::DmxService() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcDmxService);
    svc_.abi_version = TPLC_DMX_ABI_VERSION;
    svc_.host = this;
    svc_.dmx_open = [](void* s, const char* p, uint32_t b, uint32_t* o)
        { return self(s)->open(s, p, b, o); };
    svc_.dmx_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.dmx_status = [](void* s, uint32_t h, TplcDmxStatus* o)
        { return self(s)->status(s, h, o); };
    svc_.dmx_set_channel = [](void* s, uint32_t h, uint32_t c, uint32_t v)
        { return self(s)->setChannel(s, h, c, v); };
    svc_.dmx_set_channels = [](void* s, uint32_t h, uint32_t st,
                               const uint8_t* v, uint32_t n)
        { return self(s)->setChannels(s, h, st, v, n); };
    svc_.dmx_send_frame = [](void* s, uint32_t h)
        { return self(s)->sendFrame(s, h); };
}

DmxService::~DmxService() {
    for (auto& p : impl_->ports)
        if (p.fd >= 0) ::close(p.fd);
}

int32_t DmxService::open(void*, const char* path, uint32_t baud,
                         uint32_t* out_handle) {
    if (!path || !*path || !out_handle) return TPLC_DMX_E_PARAM;
    for (uint32_t i = 0; i < TPLC_DMX_HDL_MAX; ++i) {
        if (impl_->ports[i].fd < 0) {
            /* opt-in: only now is the port touched */
            int fd = ::open(path, O_WRONLY | O_NOCTTY | O_NONBLOCK);
            if (fd < 0) return TPLC_DMX_E_NOSERIAL;
            /* 250 kbaud unless asked otherwise — DMX512 is 8N2 @ 250k */
            termios tio{};
            speed_t spd = B38400;
            if (baud == 250000 || baud == 0) spd = B38400; /* see below */
            else if (baud == 115200) spd = B115200;
            else if (baud == 57600)  spd = B57600;
            else if (baud == 38400)  spd = B38400;
            cfsetispeed(&tio, spd);
            cfsetospeed(&tio, spd);
            tio.c_cflag = CS8 | CSTOPB | CREAD | CLOCAL;  /* 8N2 */
            tio.c_lflag = 0;
            tio.c_iflag = 0;
            tio.c_oflag = 0;
            tio.c_cc[VMIN]  = 0;
            tio.c_cc[VTIME] = 0;
            if (tcsetattr(fd, TCSANOW, &tio) != 0) {
                ::close(fd);
                return TPLC_DMX_E_NOSERIAL;
            }
            auto& p = impl_->ports[i];
            p.fd = fd; p.state = 1; p.frames = 0; p.last_error = 0;
            std::memset(p.data, 0, sizeof(p.data));  /* start code 0 */
            *out_handle = i + 1;
            return TPLC_DMX_OK;
        }
    }
    return TPLC_DMX_E_HANDLE;
}

int32_t DmxService::close(void*, uint32_t h) {
    if (h == 0 || h > TPLC_DMX_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_DMX_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    ::close(p.fd); p.fd = -1; p.state = 0;
    return TPLC_DMX_OK;
}

int32_t DmxService::status(void*, uint32_t h, TplcDmxStatus* out) {
    if (!out) return TPLC_DMX_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_DMX_HDL_MAX || impl_->ports[h - 1].fd < 0) {
        out->state = 0;
        return TPLC_DMX_OK;
    }
    const auto& p = impl_->ports[h - 1];
    out->state = p.state;
    out->frames_sent = p.frames;
    out->last_error = p.last_error;
    return TPLC_DMX_OK;
}

int32_t DmxService::setChannel(void*, uint32_t h, uint32_t channel,
                               uint32_t value) {
    if (h == 0 || h > TPLC_DMX_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_DMX_E_HANDLE;
    if (channel < 1 || channel > TPLC_DMX_CHANNELS || value > 255)
        return TPLC_DMX_E_PARAM;
    auto& p = impl_->ports[h - 1];
    p.data[channel] = uint8_t(value);       /* data[0]=start, 1..512 */
    return TPLC_DMX_OK;
}

int32_t DmxService::setChannels(void*, uint32_t h, uint32_t start,
                                const uint8_t* values, uint32_t count) {
    if (h == 0 || h > TPLC_DMX_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_DMX_E_HANDLE;
    if (start < 1 || start > TPLC_DMX_CHANNELS || (!values && count))
        return TPLC_DMX_E_PARAM;
    auto& p = impl_->ports[h - 1];
    uint32_t end = start + count > TPLC_DMX_CHANNELS + 1
                   ? TPLC_DMX_CHANNELS + 1 : start + count;
    for (uint32_t i = start; i < end; ++i)
        p.data[i] = values[i - start];
    return TPLC_DMX_OK;
}

int32_t DmxService::sendFrame(void*, uint32_t h) {
    if (h == 0 || h > TPLC_DMX_HDL_MAX || impl_->ports[h - 1].fd < 0)
        return TPLC_DMX_E_HANDLE;
    auto& p = impl_->ports[h - 1];
    /* break + MAB: pull the line low ~100us then release ~12us,
       then the start code + 512 channel bytes. */
    ioctl(p.fd, TIOCSBRK, 0);
    tcdrain(p.fd);
    usleep(120);
    ioctl(p.fd, TIOCCBRK, 0);
    usleep(12);
    if (!sendAll(p.fd, p.data, 513)) {
        p.last_error = errno;
        return TPLC_DMX_E_IO;
    }
    p.frames++;
    p.state = 2;
    return TPLC_DMX_OK;
}

} // namespace tplc
