/* KinService.hpp — in-process kinematic transformation objects.
 *
 * Mirrors the kin.* semantics of tether_io_motion_server: each object
 * owns a configured tether::kinematics::ForwardKinematicsBase chain
 * plus ACS/MCS axis lists, a parameter lock, and a rotation preset.
 * The transform math is the same Tether header code the server uses,
 * so in-process and IO results are bit-identical. Used when no
 * Tether IO endpoint is configured; the IO-backed variant is
 * KinIoService.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_KIN_SERVICE_HPP
#define TPLC_HOST_KIN_SERVICE_HPP

#include <tplc/tplc_kin.h>

#include <cstdint>
#include <memory>

namespace tplc {

class KinService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit KinService(ClockFn clock = nullptr, void* clock_ctx = nullptr);
    ~KinService();
    KinService(const KinService&) = delete;
    KinService& operator=(const KinService&) = delete;

    bool active() const { return active_; }
    const TplcKinService& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcKinService svc_{};
    struct Obj;
    struct Impl;
    std::unique_ptr<Impl> impl_;
    ClockFn clock_ = nullptr;
    void* clock_ctx_ = nullptr;
    bool active_ = false;

    static bool rebuildObj(Obj& o);
    static void commandObj(Obj& o, uint32_t cmd);

    int32_t configure(void*, uint32_t, int32_t, uint32_t,
                      const double*, uint32_t);
    int32_t setAxes(void*, uint32_t, const uint32_t*, uint32_t,
                    const uint32_t*, uint32_t);
    int32_t commandOp(void*, uint32_t, uint32_t);
    int32_t calcInput(void*, uint32_t, const double*, uint32_t);
    int32_t result(void*, uint32_t, double*, double*);
    int32_t setLocked(void*, uint32_t, uint32_t);
    int32_t rotation(void*, uint32_t, uint32_t, const double*,
                     double*);
    int32_t status(void*, uint32_t, TplcKinStatus*);
    int32_t setChannel(void*, uint32_t, uint32_t);
    int32_t getAxes(void*, uint32_t, uint32_t*, uint32_t*,
                    uint32_t*, uint32_t*);

    static KinService* self(void* p) {
        return static_cast<KinService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_KIN_SERVICE_HPP */
