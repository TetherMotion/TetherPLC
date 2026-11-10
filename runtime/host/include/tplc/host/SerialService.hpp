/* SerialService.hpp — tplc-serial host implementation.
 * POSIX termios behind the TplcSerialService vtable. Every op
 * returns immediately; the FB_Serial state machine polls across
 * scans. SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_SERIALSERVICE_HPP
#define TPLC_HOST_SERIALSERVICE_HPP

#include <tplc/tplc_serial.h>

#include <unordered_map>

namespace tplc {

class SerialService {
public:
    SerialService();

    const TplcSerialService& table() const { return svc_; }

private:
    int32_t serialOpen(void* s, const char* device_path, uint32_t baud,
                       uint8_t databits, uint8_t parity, uint8_t stopbits,
                       uint32_t flags, TplcSerialHandle* out);
    int32_t serialWrite(void* s, TplcSerialHandle h, const void* buf,
                        uint32_t len, uint32_t* done);
    int32_t serialRead(void* s, TplcSerialHandle h, void* buf,
                       uint32_t cap, uint32_t* done);
    int32_t serialFlush(void* s, TplcSerialHandle h, uint8_t purge);
    int32_t serialRs485(void* s, TplcSerialHandle h, uint8_t enable);
    int32_t serialClose(void* s, TplcSerialHandle h);

    int*            find(TplcSerialHandle h);
    void            drop(TplcSerialHandle h);
    static int32_t  mapErr(int e);

    TplcSerialService               svc_{};
    std::unordered_map<uint32_t, int> fds_;
    uint32_t                        next_handle_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_SERIALSERVICE_HPP */
