/* KinService.cpp — in-process kinematic transformation objects.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/KinService.hpp>

#include <cstring>

#if defined(TPLC_HAVE_TETHER_IO)

#include "tether/kinematics/ForwardKinematics.hpp"

#include <array>
#include <cmath>

namespace tplc {

using namespace tether::kinematics;

namespace {

constexpr int32_t kTypeMin = -1;
constexpr int32_t kTypeMax = TPLC_KIN_T_STEWART;

float cfgAt(const std::vector<double>& cfg, size_t i, float dflt) {
    return i < cfg.size() ? float(cfg[i]) : dflt;
}

} // namespace

struct KinService::Obj {
    std::unique_ptr<ForwardKinematicsBase> model;
    int32_t  type = -1;
    uint32_t preset = 0;
    std::vector<double> cfg;
    uint32_t locked = 0;
    std::array<double, TPLC_KIN_MAX_DIM> in{}, out{}, q{};
    uint32_t acs[TPLC_KIN_MAX_DIM]{};
    int32_t  nacs = 0;
    uint32_t mcs[TPLC_KIN_MAX_DIM]{};
    int32_t  nmcs = 0;
    int32_t  state = TPLC_KIN_STATE_IDLE;
    int32_t  err = 0;
    std::array<double, 3> rot{}, rotsaved{};
    uint32_t chn = 0;
};

struct KinService::Impl {
    Obj objs[TPLC_KIN_OBJ_MAX];
};

/* (Re)build the FK model from type/preset/cfg — same rules as the
   kin.* params on tether_io_motion_server. */
bool KinService::rebuildObj(Obj& o) {
    o.model.reset();
    if (o.type < 0) { o.state = TPLC_KIN_STATE_IDLE; return false; }
    const auto& c = o.cfg;
    switch (o.type) {
        case TPLC_KIN_T_CARTESIAN:
            o.model = std::make_unique<CartesianRobot>(); break;
        case TPLC_KIN_T_PLANAR2D:
            o.model = std::make_unique<Planar2DOF>(
                cfgAt(c,0,0.5f), cfgAt(c,1,0.5f)); break;
        case TPLC_KIN_T_ARTICULATED3:
            o.model = std::make_unique<Articulated3DOF>(
                cfgAt(c,0,0.3f), cfgAt(c,1,0.4f), cfgAt(c,2,0.3f));
            break;
        case TPLC_KIN_T_SCARA:
            o.model = std::make_unique<SCARA>(
                cfgAt(c,0,0.35f), cfgAt(c,1,0.35f),
                cfgAt(c,2,0.4f), cfgAt(c,3,0.2f)); break;
        case TPLC_KIN_T_SERIAL6: {
            auto m = std::make_unique<Serial6DOF>();
            if (o.preset == TPLC_KIN_PRESET_KUKA)
                m->setKukaKR6Parameters();
            else
                m->setUR5Parameters();
            if (c.size() >= 24) {
                DHParameters dh[6];
                for (int i = 0; i < 6; ++i)
                    dh[i] = {float(c[i*4]),   float(c[i*4+1]),
                             float(c[i*4+2]), float(c[i*4+3])};
                m->setDHParameters(dh);
            }
            o.model = std::move(m); break;
        }
        case TPLC_KIN_T_SERIAL7: {
            auto m = std::make_unique<Serial7DOF>(); /* Panda default */
            if (c.size() >= 28) {
                DHParameters dh[7];
                for (int i = 0; i < 7; ++i)
                    dh[i] = {float(c[i*4]),   float(c[i*4+1]),
                             float(c[i*4+2]), float(c[i*4+3])};
                m->setDHParameters(dh);
            }
            o.model = std::move(m); break;
        }
        case TPLC_KIN_T_GANTRY5:
            o.model = std::make_unique<Gantry5DOF>(); break;
        case TPLC_KIN_T_DELTA:
            o.model = std::make_unique<DeltaRobot>(
                cfgAt(c,0,0.15f), cfgAt(c,1,0.05f),
                cfgAt(c,2,0.2f),  cfgAt(c,3,0.4f)); break;
        case TPLC_KIN_T_STEWART:
            o.model = std::make_unique<StewartPlatform>(
                cfgAt(c,0,0.3f), cfgAt(c,1,0.15f),
                cfgAt(c,2,10.0*DEG_TO_RAD),
                cfgAt(c,3,10.0*DEG_TO_RAD)); break;
        default: return false;
    }
    o.state = TPLC_KIN_STATE_READY; o.err = 0;
    return true;
}

void KinService::commandObj(Obj& o, uint32_t cmd) {
    switch (cmd) {
        case TPLC_KIN_CMD_FWD: {
            if (!o.model) { o.state = TPLC_KIN_STATE_ERROR; o.err = 1; return; }
            const size_t n = o.model->getDOF();
            float q[8]{};
            for (size_t i = 0; i < n && i < TPLC_KIN_MAX_DIM; ++i)
                q[i] = float(o.in[i]);
            const Pose6D p = o.model->forwardKinematics(q);
            const EulerAngles e = p.euler();
            o.out = {double(p.position.x), double(p.position.y),
                     double(p.position.z), double(e.roll),
                     double(e.pitch), double(e.yaw), 0, 0};
            for (size_t i = 0; i < TPLC_KIN_MAX_DIM; ++i)
                o.q[i] = i < n ? double(q[i]) : 0.0;
            o.state = TPLC_KIN_STATE_READY; o.err = 0;
            return;
        }
        case TPLC_KIN_CMD_INV: {
            if (!o.model) { o.state = TPLC_KIN_STATE_ERROR; o.err = 1; return; }
            const size_t n = o.model->getDOF();
            if (n == 0 || n > TPLC_KIN_MAX_DIM) {
                o.state = TPLC_KIN_STATE_ERROR; o.err = 2; return;
            }
            float q[8]{};
            for (size_t i = 0; i < n; ++i)
                q[i] = i + 3 < TPLC_KIN_MAX_DIM
                           ? float(o.in[i + 3]) : 0.0f;
            const Position3D target(float(o.in[0]), float(o.in[1]),
                                    float(o.in[2]));
            const bool ok = numericPositionIK(*o.model, target, q);
            const Pose6D p = o.model->forwardKinematics(q);
            const EulerAngles e = p.euler();
            o.out = {double(p.position.x), double(p.position.y),
                     double(p.position.z), double(e.roll),
                     double(e.pitch), double(e.yaw),
                     ok ? 1.0 : 0.0, 0};
            for (size_t i = 0; i < TPLC_KIN_MAX_DIM; ++i)
                o.q[i] = i < n ? double(q[i]) : 0.0;
            o.state = ok ? TPLC_KIN_STATE_READY : TPLC_KIN_STATE_ERROR;
            o.err = ok ? 0 : 3;
            return;
        }
        case TPLC_KIN_CMD_RESET:
            o.model.reset(); o.type = -1; o.state = TPLC_KIN_STATE_IDLE;
            o.err = 0; o.nacs = 0; o.nmcs = 0;
            o.in = {}; o.out = {}; o.q = {}; o.rot = {}; o.rotsaved = {};
            return;
        case TPLC_KIN_CMD_ROT_SAVE:   o.rotsaved = o.rot; return;
        case TPLC_KIN_CMD_ROT_RESTORE: o.rot = o.rotsaved; return;
        default: o.state = TPLC_KIN_STATE_ERROR; o.err = 4; return;
    }
}

KinService::KinService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    impl_ = std::make_unique<Impl>();
    active_ = true;

    svc_.struct_size = sizeof(TplcKinService);
    svc_.abi_version = TPLC_KIN_ABI_VERSION;
    svc_.host = this;
    svc_.kin_configure = [](void* s, uint32_t o, int32_t t, uint32_t p,
                            const double* c, uint32_t n) {
        return self(s)->configure(s, o, t, p, c, n);
    };
    svc_.kin_set_axes = [](void* s, uint32_t o, const uint32_t* a,
                           uint32_t na, const uint32_t* m, uint32_t nm) {
        return self(s)->setAxes(s, o, a, na, m, nm);
    };
    svc_.kin_command = [](void* s, uint32_t o, uint32_t c) {
        return self(s)->commandOp(s, o, c);
    };
    svc_.kin_calc_input = [](void* s, uint32_t o, const double* in,
                             uint32_t n) {
        return self(s)->calcInput(s, o, in, n);
    };
    svc_.kin_result = [](void* s, uint32_t o, double* out, double* q) {
        return self(s)->result(s, o, out, q);
    };
    svc_.kin_set_locked = [](void* s, uint32_t o, uint32_t l) {
        return self(s)->setLocked(s, o, l);
    };
    svc_.kin_rotation = [](void* s, uint32_t o, uint32_t op,
                           const double* r, double* out) {
        return self(s)->rotation(s, o, op, r, out);
    };
    svc_.kin_status = [](void* s, uint32_t o, TplcKinStatus* st) {
        return self(s)->status(s, o, st);
    };
    svc_.kin_set_channel = [](void* s, uint32_t o, uint32_t c) {
        return self(s)->setChannel(s, o, c);
    };
    svc_.kin_get_axes = [](void* s, uint32_t o, uint32_t* a,
                           uint32_t* na, uint32_t* m, uint32_t* nm) {
        return self(s)->getAxes(s, o, a, na, m, nm);
    };
}

KinService::~KinService() = default;

static bool kinObjOk(uint32_t obj) { return obj < TPLC_KIN_OBJ_MAX; }

int32_t KinService::configure(void*, uint32_t obj, int32_t type,
                              uint32_t preset, const double* cfg,
                              uint32_t ncfg) {
    if (!kinObjOk(obj) || type < kTypeMin || type > kTypeMax ||
        ncfg > TPLC_KIN_MAX_CFG)
        return TPLC_KIN_E_PARAM;
    Obj& o = impl_->objs[obj];
    if (o.locked) return TPLC_KIN_E_LOCKED;
    o.type = type;
    o.preset = preset;
    o.cfg.assign(cfg, cfg + ncfg);
    rebuildObj(o);
    return TPLC_KIN_OK;
}

int32_t KinService::setAxes(void*, uint32_t obj, const uint32_t* acs,
                            uint32_t nacs, const uint32_t* mcs,
                            uint32_t nmcs) {
    if (!kinObjOk(obj) || nacs > TPLC_KIN_MAX_DIM ||
        nmcs > TPLC_KIN_MAX_DIM)
        return TPLC_KIN_E_PARAM;
    Obj& o = impl_->objs[obj];
    o.nacs = int32_t(nacs);
    for (uint32_t i = 0; i < TPLC_KIN_MAX_DIM; ++i)
        o.acs[i] = i < nacs ? acs[i] : 0;
    o.nmcs = int32_t(nmcs);
    for (uint32_t i = 0; i < TPLC_KIN_MAX_DIM; ++i)
        o.mcs[i] = i < nmcs ? mcs[i] : 0;
    return TPLC_KIN_OK;
}

int32_t KinService::commandOp(void*, uint32_t obj, uint32_t cmd) {
    if (!kinObjOk(obj)) return TPLC_KIN_E_PARAM;
    commandObj(impl_->objs[obj], cmd);
    return TPLC_KIN_OK;
}

int32_t KinService::calcInput(void*, uint32_t obj, const double* in,
                              uint32_t n) {
    if (!kinObjOk(obj) || !in || n > TPLC_KIN_MAX_DIM)
        return TPLC_KIN_E_PARAM;
    Obj& o = impl_->objs[obj];
    for (uint32_t i = 0; i < TPLC_KIN_MAX_DIM; ++i)
        o.in[i] = i < n ? in[i] : 0.0;
    return TPLC_KIN_OK;
}

int32_t KinService::result(void*, uint32_t obj, double* out8,
                           double* q8) {
    if (!kinObjOk(obj)) return TPLC_KIN_E_PARAM;
    const Obj& o = impl_->objs[obj];
    if (out8) std::memcpy(out8, o.out.data(), o.out.size() * 8);
    if (q8)   std::memcpy(q8,   o.q.data(),   o.q.size() * 8);
    return TPLC_KIN_OK;
}

int32_t KinService::setLocked(void*, uint32_t obj, uint32_t locked) {
    if (!kinObjOk(obj)) return TPLC_KIN_E_PARAM;
    impl_->objs[obj].locked = locked ? 1 : 0;
    return TPLC_KIN_OK;
}

int32_t KinService::rotation(void*, uint32_t obj, uint32_t op,
                             const double* rot3, double* out3) {
    if (!kinObjOk(obj)) return TPLC_KIN_E_PARAM;
    Obj& o = impl_->objs[obj];
    switch (op) {
        case TPLC_KIN_ROT_GET:
            if (out3) std::memcpy(out3, o.rot.data(), 24);
            return TPLC_KIN_OK;
        case TPLC_KIN_ROT_PRESET:
            if (!rot3) return TPLC_KIN_E_PARAM;
            std::memcpy(o.rot.data(), rot3, 24);
            return TPLC_KIN_OK;
        case TPLC_KIN_ROT_SAVE:
            commandObj(o, TPLC_KIN_CMD_ROT_SAVE);
            return TPLC_KIN_OK;
        case TPLC_KIN_ROT_RESTORE:
            commandObj(o, TPLC_KIN_CMD_ROT_RESTORE);
            return TPLC_KIN_OK;
        default: return TPLC_KIN_E_PARAM;
    }
}

int32_t KinService::status(void*, uint32_t obj, TplcKinStatus* out) {
    if (!kinObjOk(obj) || !out) return TPLC_KIN_E_PARAM;
    const Obj& o = impl_->objs[obj];
    out->state = o.state;
    out->type = o.model ? o.type : -1;
    out->dof = o.model ? int32_t(o.model->getDOF()) : 0;
    out->nacs = o.nacs;
    out->nmcs = o.nmcs;
    out->locked = o.locked;
    out->err = o.err;
    out->chnstate = o.model ? o.state : TPLC_KIN_STATE_IDLE;
    out->chn = int32_t(o.chn);
    return TPLC_KIN_OK;
}

int32_t KinService::setChannel(void*, uint32_t obj, uint32_t chn) {
    if (!kinObjOk(obj) || chn > 255) return TPLC_KIN_E_PARAM;
    impl_->objs[obj].chn = chn;
    return TPLC_KIN_OK;
}

int32_t KinService::getAxes(void*, uint32_t obj, uint32_t* acs,
                            uint32_t* nacs, uint32_t* mcs,
                            uint32_t* nmcs) {
    if (!kinObjOk(obj)) return TPLC_KIN_E_PARAM;
    const Obj& o = impl_->objs[obj];
    if (acs && nacs) {
        const uint32_t n = uint32_t(o.nacs < 0 ? 0 : o.nacs);
        for (uint32_t i = 0; i < n && i < TPLC_KIN_MAX_DIM; ++i)
            acs[i] = o.acs[i];
        *nacs = n;
    }
    if (mcs && nmcs) {
        const uint32_t n = uint32_t(o.nmcs < 0 ? 0 : o.nmcs);
        for (uint32_t i = 0; i < n && i < TPLC_KIN_MAX_DIM; ++i)
            mcs[i] = o.mcs[i];
        *nmcs = n;
    }
    return TPLC_KIN_OK;
}

} // namespace tplc

#else /* !TPLC_HAVE_TETHER_IO */

namespace tplc {

struct KinService::Impl {};

KinService::KinService(ClockFn, void*) = default;
KinService::~KinService() = default;

int32_t KinService::configure(void*, uint32_t, int32_t, uint32_t,
                              const double*, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::setAxes(void*, uint32_t, const uint32_t*, uint32_t,
                            const uint32_t*, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::commandOp(void*, uint32_t, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::calcInput(void*, uint32_t, const double*, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::result(void*, uint32_t, double*, double*) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::setLocked(void*, uint32_t, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::rotation(void*, uint32_t, uint32_t, const double*,
                             double*) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::status(void*, uint32_t, TplcKinStatus*) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::setChannel(void*, uint32_t, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinService::getAxes(void*, uint32_t, uint32_t*, uint32_t*,
                            uint32_t*, uint32_t*) {
    return TPLC_KIN_E_STATE;
}

} // namespace tplc

#endif /* TPLC_HAVE_TETHER_IO */
