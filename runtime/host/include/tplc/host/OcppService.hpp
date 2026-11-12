/* OcppService.hpp — tplc-ocpp OCPP-J client host service.  A real
 * OCPP-over-WebSocket client: RFC6455 transport + OCPP JSON-RPC
 * framing — behind the tplc-ocpp vtable.  The PLC-side builtin
 * FB_Ocpp dispatches ops to it via getService("tplc-ocpp").  Each op
 * is bounded so a cyclic scan can't block past the watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_OCPP_SERVICE_HPP
#define TPLC_OCPP_SERVICE_HPP

#include <tplc/tplc_ocpp.h>
#include <memory>

namespace tplc {

class OcppService {
public:
    OcppService();
    ~OcppService();

    OcppService(const OcppService&) = delete;
    OcppService& operator=(const OcppService&) = delete;

    const TplcOcppService& table() const { return svc_; }
    bool active() const { return true; }

private:
    static OcppService* self(void* s) {
        return static_cast<OcppService*>(
            reinterpret_cast<TplcOcppService*>(s)->host);
    }

    int32_t connect(void*, const char* host, uint32_t port,
                    const char* path, const char* ocpp_id,
                    uint32_t* out_handle);
    int32_t disconnect(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcOcppStatus* out);
    int32_t call(void*, uint32_t h, const char* msg_id,
                 const char* action, const uint8_t* payload,
                 uint32_t payload_len);
    int32_t result(void*, uint32_t h, const char* msg_id,
                   const uint8_t* payload, uint32_t len);
    int32_t callerror(void*, uint32_t h, const char* msg_id,
                      const char* code, const char* desc,
                      const uint8_t* details, uint32_t len);
    int32_t receive(void*, uint32_t h, TplcOcppRx* rx);

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcOcppService svc_{};
};

} /* namespace tplc */
#endif /* TPLC_OCPP_SERVICE_HPP */
