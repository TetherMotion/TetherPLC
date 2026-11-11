/* MotionService.hpp — simulated CiA402/DS402-shaped axis backend.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_MOTION_SERVICE_HPP
#define TPLC_HOST_MOTION_SERVICE_HPP

#include <tplc/tplc_motion.h>

#include <cstdint>

namespace tplc {

class MotionService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit MotionService(ClockFn clock = nullptr,
                           void* clock_ctx = nullptr);

    const TplcMotionService& table() const { return svc_; }

private:
    /* CiA402State values (== statusword low bits). */
    enum State : int32_t {
        kNotReady        = 0x00,
        kSwitchOnDis     = 0x40,
        kReadyToSwitchOn = 0x21,
        kSwitchedOn      = 0x23,
        kOpEnabled       = 0x27,
        kQuickStopActive = 0x07,
        kFaultReact      = 0x0F,
        kFault           = 0x08,
    };

    struct Axis {
        uint32_t handle = 0;
        bool used = false;
        TPLC_MotionAxisCfg cfg{};
        /* DS402 interface */
        uint16_t controlword = 0;
        uint16_t prev_controlword = 0;
        int8_t  mode_op = TPLC_MOTION_MODE_NONE;
        int32_t state = kSwitchOnDis;
        int32_t error_code = 0;
        /* motion targets (object dictionary values) */
        int32_t target_pos = 0;
        int32_t target_vel = 0;
        int32_t target_torque = 0;
        uint32_t follow_window = 0;
        int32_t home_offset = 0;
        /* simulated dynamics (double for deterministic profile
           math; reported to the object dictionary as int32) */
        double pos = 0.0;
        double vel = 0.0;
        double torque = 0.0;
        /* motion bookkeeping */
        bool move_active = false;    /* PP/HM profile in progress */
        bool target_reached = false;
        bool homing_done = false;
    };

    static MotionService* self(void* p);

    int32_t axisCreate(void*, const TPLC_MotionAxisCfg*,
                       uint32_t* out);
    int32_t axisDestroy(void*, uint32_t axis);
    int32_t axisPoll(void*);
    int32_t axisInfo(void*, uint32_t axis, TPLC_MotionAxisInfo*);
    int32_t objRead(void*, uint32_t axis, uint32_t index,
                    int32_t* out);
    int32_t objWrite(void*, uint32_t axis, uint32_t index,
                     int32_t value);

    /* advance all axes by the clock delta since the last tick */
    void advance(uint64_t now_ns);
    /* drive one axis's dynamics for dt seconds */
    void step(Axis& a, double dt);
    /* CiA402 state machine: recompute `state` from controlword */
    void updateState(Axis& a);
    /* statusword value for an axis (state | high bits) */
    uint16_t statusword(const Axis& a) const;
    /* trapezoidal PP/HM profile step toward `goal` */
    void profileStep(Axis& a, double goal, double dt);
    Axis* find(uint32_t handle);

    /* svc_ must stay the first data member: the service-table
       pointer handed to the vtable lambdas is &svc_, and self()
       recovers the MotionService* by treating it as `this`. */
    TplcMotionService svc_{};
    ClockFn clock_;
    void* clock_ctx_;
    Axis axes_[TPLC_MOTION_AXIS_MAX]{};
    uint64_t last_tick_ns_ = 0;
    bool tick_primed_ = false;
};

} /* namespace tplc */

#endif /* TPLC_HOST_MOTION_SERVICE_HPP */
