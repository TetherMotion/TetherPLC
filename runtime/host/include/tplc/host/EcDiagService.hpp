/* EcDiagService.hpp — tplc-ecdiag backend over the Tether IO
 * protocol.  Reads the ec.coediag.* params (slave selector, 0x10F3
 * history blob, ack) that tether_io_motion_server (sim) or a real
 * master bridge exposes.  Endpoint: TPLC_EC_IO, else
 * TPLC_MOTION_IO.  SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_ECDIAG_SERVICE_HPP
#define TPLC_HOST_ECDIAG_SERVICE_HPP

#include <tplc/tplc_ecdiag.h>

#include <memory>

namespace tplc {

class EcDiagService {
public:
    EcDiagService();
    ~EcDiagService();
    EcDiagService(const EcDiagService&) = delete;
    EcDiagService& operator=(const EcDiagService&) = delete;

    bool active() const { return active_; }
    const TplcEcDiagService& table() const { return svc_; }

private:
    int32_t read(void*, uint32_t slave, TplcEcDiagEntry* entries,
                 uint32_t max, uint32_t* count, TplcEcDiagInfo* info);
    int32_t ack(void*, uint32_t slave, uint32_t newest_acked);
    TplcEcDiagService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    bool active_ = false;
};

} // namespace tplc
#endif
