/* Aes70Service.hpp — tplc-aes70 OCA (AES70/Open Control
 * Architecture) client host service.  A real OSCP+OCP client over
 * TCP — sync header "OCP1" + OcaMessage PDUs — behind the
 * tplc-aes70 vtable.  The PLC-side builtin FB_Aes70 dispatches ops
 * to it via getService("tplc-aes70").  Each op is bounded so a
 * cyclic scan can't block past the watchdog.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_AES70_SERVICE_HPP
#define TPLC_AES70_SERVICE_HPP

#include <tplc/tplc_aes70.h>
#include <memory>

namespace tplc {

class Aes70Service {
public:
    Aes70Service();
    ~Aes70Service();

    Aes70Service(const Aes70Service&) = delete;
    Aes70Service& operator=(const Aes70Service&) = delete;

    const TplcAes70Service& table() const { return svc_; }
    bool active() const { return true; }

private:
    static Aes70Service* self(void* s) {
        return static_cast<Aes70Service*>(
            reinterpret_cast<TplcAes70Service*>(s)->host);
    }

    int32_t connect(void*, const char* host, uint32_t port,
                    uint32_t* out_handle);
    int32_t disconnect(void*, uint32_t h);
    int32_t status(void*, uint32_t h, TplcAes70Status* out);
    int32_t call(void*, uint32_t h, uint32_t target_ono,
                 uint32_t method_id, const uint8_t* params,
                 uint32_t params_len, uint32_t* out_handle2);
    int32_t keepalive(void*, uint32_t h);
    int32_t receive(void*, uint32_t h, uint32_t* out_handle2,
                    uint32_t* out_status_code,
                    uint8_t* payload, uint32_t payload_cap);

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcAes70Service svc_{};
};

} /* namespace tplc */
#endif /* TPLC_AES70_SERVICE_HPP */
