/* EipService.hpp — tplc-eip EtherNet/IP explicit-messaging client.
 *
 * The PLC is the scanner-side client: eip_open opens TCP 44818 and
 * RegisterSession's it; eip_get_attr/eip_set_attr carry unconnected
 * CIP Get/Set_Attribute_Single inside SendRRData; eip_list_identity
 * issues ListIdentity discovery; eip_close unregisters. Synchronous
 * per-transaction model like the other C4 services. Cyclic implicit
 * (class-1) I/O is deliberately out of scope for this tranche.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_EIP_SERVICE_HPP
#define TPLC_HOST_EIP_SERVICE_HPP

#include <tplc/tplc_eip.h>

#include <cstdint>
#include <memory>

namespace tplc {

class EipService {
public:
    EipService();
    ~EipService();
    EipService(const EipService&) = delete;
    EipService& operator=(const EipService&) = delete;

    bool active() const { return true; }
    const TplcEipService& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcEipService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;

    int32_t open(void*, const char*, uint32_t, uint32_t*);
    int32_t close(void*, uint32_t);
    int32_t status(void*, uint32_t, TplcEipStatus*);
    int32_t listIdentity(void*, const char*, uint32_t, char*, uint32_t);
    int32_t getAttr(void*, uint32_t, uint32_t, uint32_t, uint32_t,
                    uint8_t*, uint32_t);
    int32_t setAttr(void*, uint32_t, uint32_t, uint32_t, uint32_t,
                    const uint8_t*, uint32_t);

    static EipService* self(void* p) {
        return static_cast<EipService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_EIP_SERVICE_HPP */
