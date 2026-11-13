/* EcSyncService.hpp — tplc-ecsync backend over the Tether IO
 * protocol.  Reads the ec.dc.* params (connected, sync_enabled,
 * time_diff_ns, ref_time_ns) that tether_io_motion_server (sim) or a
 * real master bridge exposes.  Endpoint: TPLC_EC_IO, else
 * TPLC_MOTION_IO.  SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_ECSYNC_SERVICE_HPP
#define TPLC_HOST_ECSYNC_SERVICE_HPP

#include <tplc/tplc_ecsync.h>

#include <memory>

namespace tplc {

class EcSyncService {
public:
    EcSyncService();
    ~EcSyncService();
    EcSyncService(const EcSyncService&) = delete;
    EcSyncService& operator=(const EcSyncService&) = delete;

    bool active() const { return active_; }
    const TplcEcsyncService& table() const { return svc_; }

private:
    int32_t status(void*, TplcEcsyncStatus* out);
    int32_t setSync(void*, uint32_t enable);
    TplcEcsyncService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    bool active_ = false;
};

} // namespace tplc
#endif
