#include "tplc/host/CamService.hpp"

namespace tplc {

CamService::CamService() {
    svc_.struct_size = sizeof(TplcCamService);
    svc_.abi_version = TPLC_CAM_ABI_VERSION;
    svc_.host = this;
    svc_.cam_create = [](void* h, uint32_t* o) {
        return self(h)->create(h, o);
    };
    svc_.cam_clear = [](void* h, uint32_t c) {
        return self(h)->clear(h, c);
    };
    svc_.cam_write = [](void* h, uint32_t c, uint32_t i, double m,
                        double s) {
        return self(h)->writePt(h, c, i, m, s);
    };
    svc_.cam_config = [](void* h, uint32_t c, uint32_t n, uint32_t f) {
        return self(h)->config(h, c, n, f);
    };
    svc_.cam_eval = [](void* h, uint32_t c, double m, double* s) {
        return self(h)->eval(h, c, m, s);
    };
    svc_.cam_info = [](void* h, uint32_t c, uint32_t* n, uint32_t* f) {
        return self(h)->info(h, c, n, f);
    };
    svc_.cam_bind = [](void* h, uint32_t sl, uint32_t ma, uint32_t c,
                       double ms, double mo, double ss, double so) {
        return self(h)->bind(h, sl, ma, c, ms, mo, ss, so);
    };
    svc_.cam_unbind = [](void* h, uint32_t sl, uint32_t c) {
        return self(h)->unbind(h, sl, c);
    };
    svc_.cam_bound = [](void* h, uint32_t sl, uint32_t* c, uint32_t* ma,
                        double* ms, double* mo, double* ss, double* so) {
        return self(h)->bound(h, sl, c, ma, ms, mo, ss, so);
    };
    svc_.cam_scaling = [](void* h, uint32_t sl, double ms, double mo,
                          double ss, double so) {
        return self(h)->scaling(h, sl, ms, mo, ss, so);
    };
}

CamService::CamObj* CamService::findCam(uint32_t cam) {
    if (cam == 0 || cam > TPLC_CAM_MAX_OBJS) return nullptr;
    CamObj& o = objs_[cam - 1];
    return o.used ? &o : nullptr;
}

const CamService::Binding* CamService::findBind(uint32_t slave) const {
    for (const Binding& b : binds_)
        if (b.used && b.slave == slave) return &b;
    return nullptr;
}

CamService::Binding* CamService::findBind(uint32_t slave) {
    for (Binding& b : binds_)
        if (b.used && b.slave == slave) return &b;
    return nullptr;
}

int32_t CamService::create(void*, uint32_t* out) {
    if (out == nullptr) return TPLC_CAM_E_PARAM;
    for (uint32_t i = 0; i < TPLC_CAM_MAX_OBJS; ++i) {
        if (!objs_[i].used) {
            objs_[i] = CamObj{};
            objs_[i].used = true;
            *out = i + 1;
            return TPLC_CAM_OK;
        }
    }
    return TPLC_CAM_E_FULL;
}

int32_t CamService::clear(void*, uint32_t cam) {
    CamObj* o = findCam(cam);
    if (o == nullptr) return TPLC_CAM_E_STATE;
    o->npts = 0;
    o->flags = 0;
    return TPLC_CAM_OK;
}

int32_t CamService::writePt(void*, uint32_t cam, uint32_t idx,
                            double m, double s) {
    CamObj* o = findCam(cam);
    if (o == nullptr || idx >= TPLC_CAM_MAX_PTS) return TPLC_CAM_E_STATE;
    o->m[idx] = m;
    o->s[idx] = s;
    if (idx + 1 > o->npts) o->npts = idx + 1;
    return TPLC_CAM_OK;
}

int32_t CamService::config(void*, uint32_t cam, uint32_t npts,
                           uint32_t flags) {
    CamObj* o = findCam(cam);
    if (o == nullptr) return TPLC_CAM_E_STATE;
    if (npts > TPLC_CAM_MAX_PTS) return TPLC_CAM_E_PARAM;
    if (npts != 0) o->npts = npts;
    o->flags = flags & (TPLC_CAM_F_PERIODIC | TPLC_CAM_F_CONTMASK);
    return TPLC_CAM_OK;
}

int32_t CamService::eval(void*, uint32_t cam, double mpos,
                         double* spos) {
    const CamObj* o = findCam(cam);
    if (o == nullptr || spos == nullptr) return TPLC_CAM_E_STATE;
    if (o->npts < 2) {
        *spos = (o->npts == 1) ? o->s[0] : 0.0;
        return TPLC_CAM_OK;
    }
    double mt = mpos;
    const bool periodic = (o->flags & TPLC_CAM_F_PERIODIC) != 0;
    const double lo = o->m[0];
    const double hi = o->m[o->npts - 1];
    const double range = hi - lo;
    if (mt < lo) {
        if (periodic && range > 0.0) {
            /* wrap upward into the table range */
            while (mt < lo) mt += range;
        } else {
            *spos = o->s[0];
            return TPLC_CAM_OK;
        }
    }
    if (mt > hi) {
        if (periodic && range > 0.0) {
            while (mt > hi) mt -= range;
            if (mt < lo) mt = lo;
        } else {
            *spos = o->s[o->npts - 1];
            return TPLC_CAM_OK;
        }
    }
    /* linear interpolation over the (ascending) master axis */
    uint32_t seg = 0;
    for (uint32_t i = 0; i + 1 < o->npts; ++i) {
        if (mt <= o->m[i + 1]) { seg = i; break; }
        seg = i;
    }
    const double m0 = o->m[seg], m1 = o->m[seg + 1];
    const double s0 = o->s[seg], s1 = o->s[seg + 1];
    if (m1 <= m0) {
        *spos = s0;
    } else {
        *spos = s0 + (mt - m0) / (m1 - m0) * (s1 - s0);
    }
    return TPLC_CAM_OK;
}

int32_t CamService::info(void*, uint32_t cam, uint32_t* npts,
                         uint32_t* flags) {
    const CamObj* o = findCam(cam);
    if (o == nullptr) return TPLC_CAM_E_STATE;
    if (npts != nullptr) *npts = o->npts;
    if (flags != nullptr) *flags = o->flags;
    return TPLC_CAM_OK;
}

int32_t CamService::bind(void*, uint32_t slave, uint32_t master,
                         uint32_t cam, double mscale, double moff,
                         double sscale, double soff) {
    if (slave == 0 || master == 0 || cam == 0) return TPLC_CAM_E_PARAM;
    if (findCam(cam) == nullptr) return TPLC_CAM_E_STATE;
    if (findBind(slave) != nullptr) return TPLC_CAM_E_BOUND;
    for (Binding& b : binds_) {
        if (!b.used) {
            b = Binding{};
            b.used = true;
            b.slave = slave;
            b.master = master;
            b.cam = cam;
            b.mscale = mscale;
            b.moff = moff;
            b.sscale = sscale;
            b.soff = soff;
            return TPLC_CAM_OK;
        }
    }
    return TPLC_CAM_E_FULL;
}

int32_t CamService::unbind(void*, uint32_t slave, uint32_t cam) {
    Binding* b = findBind(slave);
    if (b == nullptr) return TPLC_CAM_E_NOCAM;
    if (cam != 0 && b->cam != cam) return TPLC_CAM_E_STATE;
    *b = Binding{};
    return TPLC_CAM_OK;
}

int32_t CamService::bound(void*, uint32_t slave, uint32_t* cam,
                          uint32_t* master, double* mscale,
                          double* moff, double* sscale, double* soff) {
    const Binding* b = findBind(slave);
    if (cam != nullptr) *cam = b ? b->cam : 0;
    if (master != nullptr) *master = b ? b->master : 0;
    if (mscale != nullptr) *mscale = b ? b->mscale : 1.0;
    if (moff != nullptr) *moff = b ? b->moff : 0.0;
    if (sscale != nullptr) *sscale = b ? b->sscale : 1.0;
    if (soff != nullptr) *soff = b ? b->soff : 0.0;
    return TPLC_CAM_OK;
}

int32_t CamService::scaling(void*, uint32_t slave, double mscale,
                            double moff, double sscale, double soff) {
    Binding* b = findBind(slave);
    if (b == nullptr) return TPLC_CAM_E_NOCAM;
    b->mscale = mscale;
    b->moff = moff;
    b->sscale = sscale;
    b->soff = soff;
    return TPLC_CAM_OK;
}

} // namespace tplc
