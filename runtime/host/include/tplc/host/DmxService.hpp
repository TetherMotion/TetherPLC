/* DmxService.hpp — tplc-dmx DMX512 output host service.  Opt-in:
 * owns a serial port and emits real DMX512 frames (break +
 * mark-after-break + start code + 512 channel bytes at 250 kbaud)
 * behind the tplc-dmx vtable.  The PLC-side builtin FB_Dmx
 * dispatches ops to it via getService("tplc-dmx").  Each op is
 * bounded so a cyclic scan can't block past the watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_DMX_SERVICE_HPP
#define TPLC_DMX_SERVICE_HPP

#include <tplc/tplc_dmx.h>
#include <memory>

namespace tplc {

class DmxService {
public:
    DmxService();
    ~DmxService();

    DmxService(const DmxService&) = delete;
    DmxService& operator=(const DmxService&) = delete;

    const TplcDmxService& table() const { return svc_; }
    bool active() const { return true; }

private:
    static DmxService* self(void* s) {
        return static_cast<DmxService*>(
            reinterpret_cast<TplcDmxService*>(s)->host);
    }

    int32_t open(void*, const char* path, uint32_t baud,
                 uint32_t* out_handle);
    int32_t close(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcDmxStatus* out);
    int32_t setChannel(void*, uint32_t h, uint32_t channel,
                       uint32_t value);
    int32_t setChannels(void*, uint32_t h, uint32_t start,
                        const uint8_t* values, uint32_t count);
    int32_t sendFrame(void*, uint32_t h);

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcDmxService svc_{};
};

} /* namespace tplc */
#endif /* TPLC_DMX_SERVICE_HPP */
