/* EnoceanService.hpp — tplc-enocean ESP3 host service.  Opt-in:
 * owns a serial port (57600 8N1) and emits/parses real EnOcean
 * Serial Protocol 3 frames behind the tplc-enocean vtable.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_ENOCEAN_SERVICE_HPP
#define TPLC_ENOCEAN_SERVICE_HPP

#include <tplc/tplc_enocean.h>
#include <memory>

namespace tplc {

class EnoceanService {
public:
    EnoceanService();
    ~EnoceanService();
    const TplcEnoceanService& table() const { return svc_; }

private:
    static EnoceanService* self(void* s) {
        return static_cast<EnoceanService*>(
            reinterpret_cast<TplcEnoceanService*>(s)->host);
    }

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcEnoceanService svc_{};

    int32_t open(void*, const char* path, uint32_t* out_handle);
    int32_t close(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcEnoceanStatus* out);
    int32_t sendRadio(void*, uint32_t h, uint32_t org, uint32_t sender,
                      uint32_t dest, const uint8_t* data, uint32_t len);
    int32_t receive(void*, uint32_t h, uint32_t* out_ptype,
                    uint32_t* out_org, uint32_t* out_sender,
                    uint8_t* payload, uint32_t cap);
};

} /* namespace tplc */
#endif /* TPLC_ENOCEAN_SERVICE_HPP */
