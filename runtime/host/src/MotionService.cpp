/* MotionService.cpp — simulated CiA402/DS402-shaped axis backend.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/MotionService.hpp>

#include <math.h>

namespace tplc {

namespace {
constexpr double NS_PER_SEC = 1e9;
/* settle epsilon for velocity/position (user units) */
constexpr double VEL_EPS = 1e-6;
constexpr double POS_EPS = 1e-6;
} /* namespace */

MotionService* MotionService::self(void* p) {
    return static_cast<MotionService*>(p);
}

MotionService::MotionService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    svc_.struct_size = sizeof(TplcMotionService);
    svc_.abi_version = TPLC_MOTION_ABI_VERSION;
    svc_.host = this;
    svc_.motion_axis_create = [](void* s, const TPLC_MotionAxisCfg* c,
                                 uint32_t* o) {
        return self(s)->axisCreate(s, c, o);
    };
    svc_.motion_axis_destroy = [](void* s, uint32_t a) {
        return self(s)->axisDestroy(s, a);
    };
    svc_.motion_axis_poll = [](void* s) { return self(s)->axisPoll(s); };
    svc_.motion_axis_info = [](void* s, uint32_t a,
                               TPLC_MotionAxisInfo* i) {
        return self(s)->axisInfo(s, a, i);
    };
    svc_.motion_obj_read = [](void* s, uint32_t a, uint32_t idx,
                              int32_t* o) {
        return self(s)->objRead(s, a, idx, o);
    };
    svc_.motion_obj_write = [](void* s, uint32_t a, uint32_t idx,
                               int32_t v) {
        return self(s)->objWrite(s, a, idx, v);
    };
}

MotionService::Axis* MotionService::find(uint32_t handle) {
    if (handle == 0 || handle > TPLC_MOTION_AXIS_MAX)
        return nullptr;
    Axis& a = axes_[handle - 1];
    return a.used ? &a : nullptr;
}

int32_t MotionService::axisCreate(void*, const TPLC_MotionAxisCfg* cfg,
                                  uint32_t* out) {
    if (!cfg || !out)
        return TPLC_MOTION_E_PARAM;
    if (cfg->max_velocity <= 0 || cfg->max_accel <= 0 ||
        cfg->max_decelerate <= 0)
        return TPLC_MOTION_E_PARAM;
    for (uint32_t i = 0; i < TPLC_MOTION_AXIS_MAX; ++i) {
        Axis& a = axes_[i];
        if (a.used)
            continue;
        a = Axis{};
        a.used = true;
        a.handle = i + 1;
        a.cfg = *cfg;
        a.follow_window = cfg->follow_window > 0
                              ? uint32_t(cfg->follow_window)
                              : 1u;
        a.home_offset = cfg->home_offset;
        a.state = kSwitchOnDis;
        *out = a.handle;
        return TPLC_MOTION_OK;
    }
    return TPLC_MOTION_E_FULL;
}

int32_t MotionService::axisDestroy(void*, uint32_t axis) {
    Axis* a = find(axis);
    if (!a)
        return TPLC_MOTION_E_STATE;
    *a = Axis{};
    return TPLC_MOTION_OK;
}

int32_t MotionService::axisPoll(void*) {
    uint64_t now = clock_ ? clock_(clock_ctx_) : 0;
    advance(now);
    return TPLC_MOTION_OK;
}

void MotionService::advance(uint64_t now_ns) {
    if (!tick_primed_) {
        last_tick_ns_ = now_ns;
        tick_primed_ = true;
        return;
    }
    double dt = now_ns > last_tick_ns_
                    ? double(now_ns - last_tick_ns_) / NS_PER_SEC
                    : 0.0;
    last_tick_ns_ = now_ns;
    if (dt <= 0.0)
        return;
    for (auto& a : axes_)
        if (a.used)
            step(a, dt);
}

/* CiA402 state machine. The controlword low nibble is a command,
   not independent bits: EnableVoltage(1)/QuickStop(2)/SwitchOn(0)/
   EnableOperation(3) select the target state. Bit7 FaultReset
   clears a latched fault. The simulated drive transitions to the
   commanded state on the next poll (no comm delay), which keeps
   --sim replay deterministic while still exposing every state via
   the statusword for the MC layer to sequence against. */
void MotionService::updateState(Axis& a) {
    const uint16_t cw = a.controlword;
    if (a.state == kFault || a.state == kFaultReact) {
        if (cw & TPLC_MOTION_CW_FAULT_RESET) {
            a.state = kSwitchOnDis;
            a.error_code = 0;
        }
        return;
    }
    if (a.state == kNotReady) {
        a.state = kSwitchOnDis; /* simulated drive is always ready */
        return;
    }

    const bool ev = (cw & TPLC_MOTION_CW_ENABLE_VOLTAGE) != 0;
    const bool qs = (cw & TPLC_MOTION_CW_QUICKSTOP) != 0;
    const bool so = (cw & TPLC_MOTION_CW_SWITCHON) != 0;
    const bool eo = (cw & TPLC_MOTION_CW_ENABLE_OP) != 0;

    if (!ev)
        a.state = kSwitchOnDis;     /* DisableVoltage 0x0000 */
    else if (!qs)
        a.state = kQuickStopActive; /* QuickStop      0x0002 */
    else if (!so)
        a.state = kReadyToSwitchOn; /* Shutdown       0x0006 */
    else if (!eo)
        a.state = kSwitchedOn;      /* SwitchOn       0x0007 */
    else
        a.state = kOpEnabled;       /* EnableOperation 0x000F */
}

uint16_t MotionService::statusword(const Axis& a) const {
    uint16_t sw = uint16_t(a.state) & 0x7F;
    /* VoltageEnabled once past SwitchOnDisabled */
    if (a.state != kSwitchOnDis && a.state != kNotReady &&
        a.state != kFault)
        sw |= TPLC_MOTION_SW_VOLTAGE_ENABLED;
    /* Remote (bit9): simulated drive is always remote-controlled */
    sw |= 0x0200u;
    /* QuickStop bit5: 1 when quick-stop is not active */
    if (a.state != kQuickStopActive)
        sw |= TPLC_MOTION_SW_QUICKSTOP;
    /* Warning bit7 unused; TargetReached bit10 */
    if (a.target_reached)
        sw |= TPLC_MOTION_SW_TARGET_REACHED;
    /* mode-specific bit13: following error while a PP/HM move is
       active and outside the window */
    if (a.move_active && !a.target_reached) {
        double goal = (a.mode_op == TPLC_MOTION_MODE_HM)
                          ? double(a.home_offset)
                          : double(a.target_pos);
        if (fabs(goal - a.pos) > double(a.follow_window))
            sw |= TPLC_MOTION_SW_FOLLOWING_ERR;
    }
    return sw;
}

/* trapezoidal velocity profile toward `goal`; completes (snaps to
   goal, sets target_reached) inside the follow window. */
void MotionService::profileStep(Axis& a, double goal, double dt) {
    const double window = double(a.follow_window) > 0.0
                              ? double(a.follow_window)
                              : 1.0;
    double dist = goal - a.pos;
    if (fabs(dist) <= window) {
        a.pos = goal;
        a.vel = 0.0;
        a.target_reached = true;
        a.move_active = false;
        return;
    }
    const double dir = dist >= 0.0 ? 1.0 : -1.0;
    const double adist = fabs(dist);
    /* fastest speed from which we can still stop at goal */
    const double vmax_decel =
        sqrt(2.0 * double(a.cfg.max_decelerate) * adist);
    double vmax = double(a.cfg.max_velocity);
    if (vmax_decel < vmax)
        vmax = vmax_decel;
    const double desired_vel = dir * vmax;
    /* accelerate (or decelerate) velocity toward desired at
       max_accel, using max_decelerate when shedding speed */
    double dv = desired_vel - a.vel;
    const double rate = dv * desired_vel < 0.0
                            ? double(a.cfg.max_decelerate)
                            : double(a.cfg.max_accel);
    const double max_dv = rate * dt;
    if (dv > max_dv) dv = max_dv;
    else if (dv < -max_dv) dv = -max_dv;
    a.vel += dv;
    a.pos += a.vel * dt;
    /* software position limit enforcement */
    if (a.cfg.flags & 1u) {
        if (a.pos < double(a.cfg.pos_min)) {
            a.pos = double(a.cfg.pos_min);
            if (a.vel < 0) a.vel = 0;
        } else if (a.pos > double(a.cfg.pos_max)) {
            a.pos = double(a.cfg.pos_max);
            if (a.vel > 0) a.vel = 0;
        }
    }
}

void MotionService::clampAxisPos(Axis& a) {
    if (!(a.cfg.flags & 1u))
        return;
    if (a.pos < double(a.cfg.pos_min)) {
        a.pos = double(a.cfg.pos_min);
        if (a.vel < 0) a.vel = 0;
    } else if (a.pos > double(a.cfg.pos_max)) {
        a.pos = double(a.cfg.pos_max);
        if (a.vel > 0) a.vel = 0;
    }
}

void MotionService::step(Axis& a, double dt) {
    updateState(a);

    /* rising edge on NewSetPoint (bit4) / HomingOperationStart
       arms a PP or HM profile move while operation-enabled */
    const bool new_sp = (a.controlword & TPLC_MOTION_CW_NEW_SETPOINT) &&
                        !(a.prev_controlword & TPLC_MOTION_CW_NEW_SETPOINT);
    a.prev_controlword = a.controlword;
    if (new_sp && a.state == kOpEnabled) {
        if (a.mode_op == TPLC_MOTION_MODE_PP) {
            a.move_active = true;
            a.target_reached = false;
        } else if (a.mode_op == TPLC_MOTION_MODE_HM) {
            a.move_active = true;
            a.target_reached = false;
            a.homing_done = false;
        }
    }

    /* not operation-enabled: no powered motion; bleed off
       velocity so a quick-stop / disable actually stops the axis */
    if (a.state != kOpEnabled) {
        a.vel *= (1.0 - fmin(1.0, 8.0 * dt));
        a.pos += a.vel * dt;
        if (fabs(a.vel) < VEL_EPS) {
            a.vel = 0.0;
            a.move_active = false;
            if (a.mode_op == TPLC_MOTION_MODE_PP)
                a.target_reached = true;
        }
        return;
    }

    switch (a.mode_op) {
    case TPLC_MOTION_MODE_PP:
        if (a.move_active)
            profileStep(a, double(a.target_pos), dt);
        break;
    case TPLC_MOTION_MODE_HM:
        if (a.move_active)
            profileStep(a, double(a.home_offset), dt);
        if (a.target_reached && !a.homing_done) {
            /* homing complete: latch position to the home offset
               and mark the HM statusword bit (bit12) via a zero
               error code; target_reached already reflects it */
            a.pos = double(a.home_offset);
            a.homing_done = true;
        }
        break;
    case TPLC_MOTION_MODE_VL:
        /* velocity mode: immediate velocity command */
        a.vel = double(a.target_vel);
        a.pos += a.vel * dt;
        clampAxisPos(a);
        a.target_reached = (a.target_vel == 0);
        break;
    case TPLC_MOTION_MODE_PV:
        /* profile velocity: ramp velocity toward target */
        {
            double dv = double(a.target_vel) - a.vel;
            double rate = dv * double(a.target_vel) < 0.0
                              ? double(a.cfg.max_decelerate)
                              : double(a.cfg.max_accel);
            double max_dv = rate * dt;
            if (dv > max_dv) dv = max_dv;
            else if (dv < -max_dv) dv = -max_dv;
            a.vel += dv;
            a.pos += a.vel * dt;
            clampAxisPos(a);
            a.target_reached =
                (fabs(a.vel - double(a.target_vel)) < 1.0);
        }
        break;
    case TPLC_MOTION_MODE_TQ:
        /* profile torque: velocity responds to applied torque */
        {
            a.torque = double(a.target_torque);
            a.vel += a.torque * 0.001 * dt; /* sim gain */
            const double vmax = double(a.cfg.max_velocity);
            if (a.vel > vmax) a.vel = vmax;
            else if (a.vel < -vmax) a.vel = -vmax;
            a.pos += a.vel * dt;
            clampAxisPos(a);
            a.target_reached = true;
        }
        break;
    case TPLC_MOTION_MODE_CSP:
        /* cyclic sync position: snap to the per-cycle target */
        a.pos = double(a.target_pos);
        a.vel = 0.0;
        a.target_reached = true;
        break;
    case TPLC_MOTION_MODE_CSV:
        /* cyclic sync velocity */
        a.vel = double(a.target_vel);
        a.pos += a.vel * dt;
        clampAxisPos(a);
        a.target_reached = true;
        break;
    case TPLC_MOTION_MODE_CST:
        /* cyclic sync torque */
        {
            a.torque = double(a.target_torque);
            a.vel += a.torque * 0.001 * dt;
            const double vmax2 = double(a.cfg.max_velocity);
            if (a.vel > vmax2) a.vel = vmax2;
            else if (a.vel < -vmax2) a.vel = -vmax2;
            a.pos += a.vel * dt;
            clampAxisPos(a);
            a.target_reached = true;
        }
        break;
    default:
        break;
    }
}

int32_t MotionService::axisInfo(void*, uint32_t axis,
                                TPLC_MotionAxisInfo* info) {
    if (!info)
        return TPLC_MOTION_E_PARAM;
    Axis* a = find(axis);
    if (!a)
        return TPLC_MOTION_E_STATE;
    *info = TPLC_MotionAxisInfo{};
    info->status = a->state;
    info->mode = a->mode_op;
    info->fault = (a->state == kFault || a->state == kFaultReact)
                      ? (a->error_code ? a->error_code : 1)
                      : 0;
    info->target_reached = a->target_reached ? 1 : 0;
    info->busy = (a->move_active || fabs(a->vel) > 1.0) ? 1 : 0;
    info->pos_actual = int32_t(lrint(a->pos));
    info->vel_actual = int32_t(lrint(a->vel));
    return TPLC_MOTION_OK;
}

int32_t MotionService::objRead(void*, uint32_t axis, uint32_t index,
                               int32_t* out) {
    if (!out)
        return TPLC_MOTION_E_PARAM;
    Axis* a = find(axis);
    if (!a)
        return TPLC_MOTION_E_STATE;
    const uint64_t now = clock_ ? clock_(clock_ctx_) : 0;
    advance(now);
    switch (index) {
    case TPLC_MOTION_OBJ_ERROR_CODE:
        *out = a->error_code;
        break;
    case TPLC_MOTION_OBJ_CONTROLWORD:
        *out = a->controlword;
        break;
    case TPLC_MOTION_OBJ_STATUSWORD:
        *out = statusword(*a);
        break;
    case TPLC_MOTION_OBJ_MODES_OP:
        *out = a->mode_op;
        break;
    case TPLC_MOTION_OBJ_MODES_DISP:
        *out = a->mode_op;
        break;
    case TPLC_MOTION_OBJ_POS_ACTUAL:
        *out = int32_t(lrint(a->pos));
        break;
    case TPLC_MOTION_OBJ_VEL_ACTUAL:
        *out = int32_t(lrint(a->vel));
        break;
    case TPLC_MOTION_OBJ_TORQUE_ACTUAL:
        *out = int32_t(lrint(a->torque));
        break;
    case TPLC_MOTION_OBJ_FOLLOW_WIN:
        *out = int32_t(a->follow_window);
        break;
    case TPLC_MOTION_OBJ_POS_TARGET:
        *out = a->target_pos;
        break;
    case TPLC_MOTION_OBJ_VEL_TARGET:
        *out = a->target_vel;
        break;
    case TPLC_MOTION_OBJ_TORQUE_TARGET:
        *out = a->target_torque;
        break;
    case TPLC_MOTION_OBJ_HOME_OFFSET:
        *out = a->home_offset;
        break;
    case TPLC_MOTION_OBJ_PROBE_ABORT:
        *out = a->probe_abort;
        break;
    case TPLC_MOTION_OBJ_POS_OFFSET:
        *out = a->pos_offset;
        break;
    case TPLC_MOTION_OBJ_BRAKE_CMD:
        *out = a->brake_cmd;
        break;
    case TPLC_MOTION_OBJ_PARK_CMD:
        *out = a->park_cmd;
        break;
    case TPLC_MOTION_OBJ_COE_CMD:
        *out = a->coe_cmd;
        break;
    case TPLC_MOTION_OBJ_SAW_CHAR0:
    case TPLC_MOTION_OBJ_SAW_CHAR0 + 1:
    case TPLC_MOTION_OBJ_SAW_CHAR0 + 2:
    case TPLC_MOTION_OBJ_SAW_CHAR0 + 3:
        *out = a->saw_char[index - TPLC_MOTION_OBJ_SAW_CHAR0];
        break;
    case TPLC_MOTION_OBJ_POS_MIN:
        *out = a->cfg.pos_min;
        break;
    case TPLC_MOTION_OBJ_POS_MAX:
        *out = a->cfg.pos_max;
        break;
    case TPLC_MOTION_OBJ_AXIS_FLAGS:
        *out = int32_t(a->cfg.flags);
        break;
    case TPLC_MOTION_OBJ_ERR_INJECT:
        *out = a->error_code;
        break;
    default:
        return TPLC_MOTION_E_STATE;
    }
    return TPLC_MOTION_OK;
}

int32_t MotionService::objWrite(void*, uint32_t axis, uint32_t index,
                                int32_t value) {
    Axis* a = find(axis);
    if (!a)
        return TPLC_MOTION_E_STATE;
    const uint64_t now = clock_ ? clock_(clock_ctx_) : 0;
    advance(now);
    switch (index) {
    case TPLC_MOTION_OBJ_CONTROLWORD:
        a->controlword = uint16_t(value & 0xFFFF);
        break;
    case TPLC_MOTION_OBJ_MODES_OP:
        /* only accept modes the simulated axis supports */
        switch (value) {
        case TPLC_MOTION_MODE_NONE:
        case TPLC_MOTION_MODE_PP:
        case TPLC_MOTION_MODE_VL:
        case TPLC_MOTION_MODE_PV:
        case TPLC_MOTION_MODE_TQ:
        case TPLC_MOTION_MODE_HM:
        case TPLC_MOTION_MODE_CSP:
        case TPLC_MOTION_MODE_CSV:
        case TPLC_MOTION_MODE_CST:
            a->mode_op = int8_t(value);
            break;
        default:
            return TPLC_MOTION_E_MODE;
        }
        break;
    case TPLC_MOTION_OBJ_POS_ACTUAL:
        /* simulated axis: allow direct position preset (encoder
           set / MC_SetPosition equivalent) */
        a->pos = double(value);
        break;
    case TPLC_MOTION_OBJ_POS_TARGET:
        a->target_pos = value;
        break;
    case TPLC_MOTION_OBJ_VEL_TARGET:
        a->target_vel = value;
        break;
    case TPLC_MOTION_OBJ_TORQUE_TARGET:
        a->target_torque = value;
        break;
    case TPLC_MOTION_OBJ_FOLLOW_WIN:
        a->follow_window = uint32_t(value < 0 ? 0 : value);
        break;
    case TPLC_MOTION_OBJ_HOME_OFFSET:
        a->home_offset = value;
        break;
    case TPLC_MOTION_OBJ_PROBE_ABORT:
        a->probe_abort = value;
        break;
    case TPLC_MOTION_OBJ_POS_OFFSET:
        a->pos_offset = value;
        break;
    case TPLC_MOTION_OBJ_BRAKE_CMD:
        a->brake_cmd = value;
        break;
    case TPLC_MOTION_OBJ_PARK_CMD:
        a->park_cmd = value;
        break;
    case TPLC_MOTION_OBJ_COE_CMD:
        a->coe_cmd = value;
        break;
    case TPLC_MOTION_OBJ_SAW_CHAR0:
    case TPLC_MOTION_OBJ_SAW_CHAR0 + 1:
    case TPLC_MOTION_OBJ_SAW_CHAR0 + 2:
    case TPLC_MOTION_OBJ_SAW_CHAR0 + 3:
        a->saw_char[index - TPLC_MOTION_OBJ_SAW_CHAR0] = value;
        break;
    case TPLC_MOTION_OBJ_POS_MIN:
        a->cfg.pos_min = value;
        break;
    case TPLC_MOTION_OBJ_POS_MAX:
        a->cfg.pos_max = value;
        break;
    case TPLC_MOTION_OBJ_AXIS_FLAGS:
        a->cfg.flags = uint32_t(value);
        break;
    case TPLC_MOTION_OBJ_ERR_INJECT:
        a->error_code = value;
        if (value != 0) a->state = kFault;
        break;
    default:
        /* controlword/statusword/mode-display/actuals are not
           writable through the generic path except as above */
        return TPLC_MOTION_E_STATE;
    }
    return TPLC_MOTION_OK;
}

} /* namespace tplc */
