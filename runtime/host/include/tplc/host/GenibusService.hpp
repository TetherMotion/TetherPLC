/* GenibusService.hpp — tplc-genibus GENIbus host service.
 * Opt-in: owns a serial port (9600 8E1) and emits/parses real
 * GENIbus telegrams behind the tplc-genibus vtable.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_GENIBUS_SERVICE_HPP
#define TPLC_GENIBUS_SERVICE_HPP

#include <tplc/tplc_genibus.h>
#include <memory>

namespace tplc {

class GenibusService {
public:
    GenibusService();
    ~GenibusService();
    const TplcGenibusService& table() const { return svc_; }

private:
    static GenibusService* self(void* s) {
        return static_cast<GenibusService*>(
            reinterpret_cast<TplcGenibusService*>(s)->host);
    }

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcGenibusService svc_{};

    int32_t open(void*, const char* path, uint32_t* out_handle);
    int32_t close(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcGenibusStatus* out);
    int32_t send(void*, uint32_t h, uint32_t da, uint32_t sa,
                 const uint8_t* apdu, uint32_t len);
    int32_t receive(void*, uint32_t h, uint32_t* out_da,
                    uint32_t* out_sa, uint8_t* payload,
                    uint32_t cap);
};

} /* namespace tplc */
#endif /* TPLC_GENIBUS_SERVICE_HPP */
