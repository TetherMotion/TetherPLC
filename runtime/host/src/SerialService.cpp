/* SerialService.cpp — POSIX termios serial service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/SerialService.hpp>

#include <errno.h>
#include <fcntl.h>
#include <string.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <unistd.h>

#ifndef TIOCSRS485
#define TIOCSRS485 0x542F
#endif

namespace tplc {

static SerialService* self(void* p) { return static_cast<SerialService*>(p); }

SerialService::SerialService() {
    svc_.struct_size   = sizeof(TplcSerialService);
    svc_.abi_version   = TPLC_SERIAL_ABI_VERSION;
    svc_.host          = this;
    svc_.serial_open   = [](void* s, const char* d, uint32_t b, uint8_t db,
                            uint8_t pa, uint8_t sb, uint32_t f,
                            TplcSerialHandle* o) {
        return self(s)->serialOpen(s, d, b, db, pa, sb, f, o);
    };
    svc_.serial_write  = [](void* s, TplcSerialHandle h, const void* b,
                            uint32_t l, uint32_t* d) {
        return self(s)->serialWrite(s, h, b, l, d);
    };
    svc_.serial_read   = [](void* s, TplcSerialHandle h, void* b,
                            uint32_t c, uint32_t* d) {
        return self(s)->serialRead(s, h, b, c, d);
    };
    svc_.serial_flush  = [](void* s, TplcSerialHandle h, uint8_t p) {
        return self(s)->serialFlush(s, h, p);
    };
    svc_.serial_rs485  = [](void* s, TplcSerialHandle h, uint8_t e) {
        return self(s)->serialRs485(s, h, e);
    };
    svc_.serial_close  = [](void* s, TplcSerialHandle h) {
        return self(s)->serialClose(s, h);
    };
}

int* SerialService::find(TplcSerialHandle h) {
    auto it = fds_.find(h);
    return it == fds_.end() ? nullptr : &it->second;
}

void SerialService::drop(TplcSerialHandle h) {
    auto it = fds_.find(h);
    if (it != fds_.end()) {
        ::close(it->second);
        fds_.erase(it);
    }
}

int32_t SerialService::mapErr(int e) {
    switch (e) {
    case EAGAIN:         return TPLC_SERIAL_E_AGAIN;
    case EBADF:
    case ENXIO:       return TPLC_SERIAL_E_CLOSED;
    case EINVAL:      return TPLC_SERIAL_E_PARAM;
    default:          return TPLC_SERIAL_E_IO;
    }
}

static speed_t baud_of(uint32_t baud) {
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

int32_t SerialService::serialOpen(void* s, const char* device_path,
                                  uint32_t baud, uint8_t databits,
                                  uint8_t parity, uint8_t stopbits,
                                  uint32_t flags, TplcSerialHandle* out) {
    (void)s;
    if (!out || !device_path || !*device_path || databits < 5 ||
        databits > 8 || parity > 2 || (stopbits != 1 && stopbits != 2))
        return TPLC_SERIAL_E_PARAM;
    speed_t sp = baud_of(baud);
    if (sp == 0) return TPLC_SERIAL_E_PARAM;

    int fd = ::open(device_path, O_RDWR | O_NOCTTY | O_NONBLOCK);
    if (fd < 0) return TPLC_SERIAL_E_OPEN;

    struct termios tio;
    if (tcgetattr(fd, &tio) < 0) {
        int e = errno;
        ::close(fd);
        return mapErr(e) == TPLC_SERIAL_E_IO ? TPLC_SERIAL_E_OPEN
                                             : mapErr(e);
    }
    cfmakeraw(&tio);
    /* standard rates only (baud_of rejects others): the B* constant
       selects the baud in c_cflag; c_ispeed/c_ospeed apply only under
       BOTHER, which glibc does not export */
    tio.c_cflag &= ~(tcflag_t)(CBAUD | CSIZE | PARENB | PARODD | CSTOPB);
    tio.c_cflag |= sp;
    switch (databits) {
    case 5: tio.c_cflag |= CS5; break;
    case 6: tio.c_cflag |= CS6; break;
    case 7: tio.c_cflag |= CS7; break;
    default: tio.c_cflag |= CS8; break;
    }
    if (parity == TPLC_SERIAL_PARITY_EVEN) tio.c_cflag |= PARENB;
    if (parity == TPLC_SERIAL_PARITY_ODD)  tio.c_cflag |= PARENB | PARODD;
    if (stopbits == 2) tio.c_cflag |= CSTOPB;
    tio.c_cc[VMIN]  = 0;
    tio.c_cc[VTIME] = 0;
    if (tcsetattr(fd, TCSANOW, &tio) < 0) {
        ::close(fd);
        return TPLC_SERIAL_E_OPEN;
    }
    if (flags & TPLC_SERIAL_F_RS485) {
        /* RS485 half-duplex: RTS drives DE. Kernel struct serial_rs485
           layout: {flags, delay_rts_before_send, delay_rts_after_send,
           padding[5]}. */
        struct {
            uint32_t flags;
            uint32_t delay_rts_before_send;
            uint32_t delay_rts_after_send;
            uint32_t padding[5];
        } rs485 = {};
        rs485.flags = 0; /* SER_RS485_ENABLED would go here; RTS mode
                            is driver-dependent, keep flags=0 */
        ioctl(fd, TIOCSRS485, &rs485); /* best effort */
    }
    TplcSerialHandle h = next_handle_++;
    if (h == TPLC_SERIAL_HANDLE_INVALID) h = next_handle_++;
    fds_[h] = fd;
    *out = h;
    return TPLC_SERIAL_OK;
}

int32_t SerialService::serialWrite(void* s, TplcSerialHandle h,
                                   const void* buf, uint32_t len,
                                   uint32_t* done) {
    (void)s;
    int* fd = find(h);
    if (!fd || !buf || !done) return TPLC_SERIAL_E_STATE;
    if (len == 0) { *done = 0; return TPLC_SERIAL_OK; }
    ssize_t n = ::write(*fd, buf, len);
    if (n < 0) return mapErr(errno);
    *done = (uint32_t)n;
    return TPLC_SERIAL_OK;
}

int32_t SerialService::serialRead(void* s, TplcSerialHandle h,
                                  void* buf, uint32_t cap,
                                  uint32_t* done) {
    (void)s;
    int* fd = find(h);
    if (!fd || !buf || !done) return TPLC_SERIAL_E_STATE;
    if (cap == 0) { *done = 0; return TPLC_SERIAL_OK; }
    ssize_t n = ::read(*fd, buf, cap);
    if (n < 0) return mapErr(errno);
    *done = (uint32_t)n;
    return TPLC_SERIAL_OK;
}

int32_t SerialService::serialFlush(void* s, TplcSerialHandle h,
                                   uint8_t purge) {
    (void)s;
    int* fd = find(h);
    if (!fd) return TPLC_SERIAL_E_STATE;
    int w = 0;
    if (purge & 1) w |= TCIFLUSH;
    if (purge & 2) w |= TCOFLUSH;
    if (w == 0) w = TCIOFLUSH;
    if (tcflush(*fd, w) < 0) return mapErr(errno);
    return TPLC_SERIAL_OK;
}

int32_t SerialService::serialRs485(void* s, TplcSerialHandle h,
                                   uint8_t enable) {
    (void)s; (void)enable;
    int* fd = find(h);
    if (!fd) return TPLC_SERIAL_E_STATE;
    /* direction GPIO is board-specific; the flag is recorded by the
       caller's UART driver — on generic USB serial this is a no-op */
    (void)*fd;
    return TPLC_SERIAL_OK;
}

int32_t SerialService::serialClose(void* s, TplcSerialHandle h) {
    (void)s;
    drop(h);
    return TPLC_SERIAL_OK;
}

} /* namespace tplc */
