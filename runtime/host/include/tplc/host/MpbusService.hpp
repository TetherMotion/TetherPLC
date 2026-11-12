/* MpbusService.hpp — tplc-mpbus MP-Bus master host service.
 * Opt-in: owns a serial port (9600 8N1, e.g. USB MP-Bus
 * gateway) and exchanges MP-Bus request/response frames behind
 * the tplc-mpbus vtable.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_MPBUS_SERVICE_HPP
#define TPLC_MPBUS_SERVICE_HPP

#include <tplc/tplc_mpbus.h>
#include <memory>

namespace tplc {

class MpbusService {
public:
    MpbusService();
    ~MpbusService();
    const TplcMpbusService& table() const { return svc_; }

private:
    static MpbusService* self(void* s) {
        return static_cast<MpbusService*>(
            reinterpret_cast<TplcMpbusService*>(s)->host);
    }

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcMpbusService svc_{};

    int32_t open(void*, const char* path, uint32_t* out_handle);
    int32_t close(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcMpbusStatus* out);
    int32_t transact(void*, uint32_t h, uint32_t da, uint32_t cmd,
                     const uint8_t* data, uint32_t len,
                     uint32_t* out_da, uint32_t* out_cmd,
                     uint8_t* resp, uint32_t cap);
    int32_t receive(void*, uint32_t h, uint32_t* out_da,
                    uint32_t* out_cmd, uint8_t* payload,
                    uint32_t cap);
};

} /* namespace tplc */
#endif /* TPLC_MPBUS_SERVICE_HPP */
