/* MbusService.hpp — tplc-mbus wired M-Bus (EN 13757) master host
 * service.  Opt-in: owns a serial port and emits/parses real M-Bus
 * frames (10 / 68 / E5 at 2400 8E1) behind the tplc-mbus vtable.
 * The PLC-side builtin FB_Mbus dispatches ops to it via
 * getService("tplc-mbus").  Each op is bounded so a cyclic scan
 * can't block past the watchdog.  SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MBUS_SERVICE_HPP
#define TPLC_MBUS_SERVICE_HPP

#include <tplc/tplc_mbus.h>
#include <memory>

namespace tplc {

class MbusService {
public:
    MbusService();
    ~MbusService();

    MbusService(const MbusService&) = delete;
    MbusService& operator=(const MbusService&) = delete;

    const TplcMbusService& table() const { return svc_; }
    bool active() const { return true; }

private:
    static MbusService* self(void* s) {
        return static_cast<MbusService*>(
            reinterpret_cast<TplcMbusService*>(s)->host);
    }

    int32_t open(void*, const char* path, uint32_t* out_handle);
    int32_t close(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcMbusStatus* out);
    int32_t shortFrame(void*, uint32_t h, uint32_t control,
                       uint32_t address);
    int32_t longFrame(void*, uint32_t h, uint32_t control,
                      uint32_t address, uint32_t ci,
                      const uint8_t* data, uint32_t len);
    int32_t receive(void*, uint32_t h, uint32_t* out_kind,
                    uint8_t* payload, uint32_t cap);

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcMbusService svc_{};
};

} /* namespace tplc */
#endif /* TPLC_MBUS_SERVICE_HPP */
