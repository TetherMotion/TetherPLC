#include "tplc/host/PathService.hpp"

#include <cmath>
#include <algorithm>

namespace tplc {

PathService::PathService() {
    svc_.struct_size = sizeof(TplcPathService);
    svc_.abi_version = TPLC_PATH_ABI_VERSION;
    svc_.host = this;
    svc_.path_create = [](void* h, uint32_t* o) {
        return self(h)->create(h, o);
    };
    svc_.path_clear = [](void* h, uint32_t p) {
        return self(h)->clear(h, p);
    };
    svc_.path_add = [](void* h, uint32_t p, uint32_t ty,
                       const double* pos, uint32_t n, double v,
                       double a, double dm) {
        return self(h)->add(h, p, ty, pos, n, v, a, dm);
    };
    svc_.path_set_start = [](void* h, uint32_t p, const double* pos,
                             uint32_t n) {
        return self(h)->setStart(h, p, pos, n);
    };
    svc_.path_eval = [](void* h, uint32_t p, double t, double* pos,
                        uint32_t cap, uint32_t* seg, int32_t* done,
                        uint32_t* nout) {
        return self(h)->eval(h, p, t, pos, cap, seg, done, nout);
    };
    svc_.path_info = [](void* h, uint32_t p, uint32_t* ns,
                        double* dur) {
        return self(h)->info(h, p, ns, dur);
    };
    svc_.path_add_circ = [](void* h, uint32_t p, uint32_t mode,
                            const double* aux, uint32_t na,
                            const double* end, uint32_t ne,
                            double v, double a, double ch) {
        return self(h)->addCirc(h, p, mode, aux, na, end, ne, v, a,
                                ch);
    };
}

PathService::Path* PathService::find(uint32_t path) {
    if (path == 0 || path > TPLC_PATH_MAX_PATHS) return nullptr;
    Path& p = paths_[path - 1];
    return p.used ? &p : nullptr;
}

/* Trapezoidal velocity profile, rest-to-rest, on a scalar distance.
   Returns progress fraction in [0,1] at local time lt and the peak
   velocity reached (for callers that want it). */
double PathService::trapProgress(double d, double vel, double acc,
                                 double lt, double* peakVel) {
    if (d <= 0.0 || lt <= 0.0) {
        if (peakVel) *peakVel = 0.0;
        return (lt <= 0.0) ? 0.0 : 1.0;
    }
    if (vel <= 0.0) vel = 1.0;
    if (acc <= 0.0) acc = vel; /* degenerate: instant ramp */
    const double dAcc = vel * vel / (2.0 * acc);
    double vPeak = vel;
    double tAcc = vel / acc;
    double tDec = tAcc;
    double dTot = d;
    if (2.0 * dAcc >= d) {
        /* triangular: never reaches vel */
        vPeak = std::sqrt(d * acc);
        tAcc = vPeak / acc;
        tDec = tAcc;
        dTot = d;
    } else {
        dTot = d;
    }
    const double tCruise = (dTot - 2.0 * (vPeak * vPeak) / (2.0 * acc)) / vPeak;
    const double dur = 2.0 * tAcc + (tCruise > 0.0 ? tCruise : 0.0);
    if (peakVel) *peakVel = vPeak;
    if (lt >= dur) return 1.0;
    /* distance covered by time lt under the trapezoid */
    double dist;
    if (lt < tAcc) {
        dist = 0.5 * acc * lt * lt;
    } else if (lt < tAcc + (tCruise > 0.0 ? tCruise : 0.0)) {
        const double lc = lt - tAcc;
        dist = 0.5 * acc * tAcc * tAcc + vPeak * lc;
    } else {
        const double ld = lt - tAcc - (tCruise > 0.0 ? tCruise : 0.0);
        const double rem = vPeak * vPeak / (2.0 * acc);
        dist = 0.5 * acc * tAcc * tAcc + vPeak * (tCruise > 0.0 ? tCruise : 0.0)
               + vPeak * ld - 0.5 * acc * ld * ld;
        (void)rem;
    }
    return dist / d;
}

double PathService::segDuration(uint32_t type, uint32_t n,
                                const double* to, const double* from,
                                double vel, double acc,
                                double dwellMs) {
    if (type == TPLC_PATH_SEG_DWELL || type == TPLC_PATH_SEG_BLEND)
        return dwellMs / 1000.0;
    double dmax = 0.0;
    for (uint32_t i = 0; i < n && i < TPLC_PATH_MAX_AXES; ++i) {
        const double d = std::fabs(to[i] - from[i]);
        if (d > dmax) dmax = d;
    }
    if (dmax <= 0.0) return 0.0;
    double v = vel > 0.0 ? vel : 1.0;
    double a = acc > 0.0 ? acc : v;
    const double dAcc = v * v / (2.0 * a);
    if (2.0 * dAcc >= dmax) {
        const double vp = std::sqrt(dmax * a);
        return 2.0 * vp / a;
    }
    const double tAcc = v / a;
    const double tCruise = (dmax - 2.0 * dAcc) / v;
    return 2.0 * tAcc + tCruise;
}

namespace {

/* Circle geometry in the (axis0, axis1) plane. Fills cx, cy, r and
   the signed sweep angle (radians, sign = direction). Returns
   false when the description is degenerate. */
bool circGeom(uint32_t mode, double ax, double ay, double sx,
              double sy, double ex, double ey, double choice,
              double* cx, double* cy, double* r, double* sweep) {
    double ccx, ccy;
    if (mode == TPLC_PATH_CIRC_CENTER) {
        ccx = ax; ccy = ay;
        const double rs = std::hypot(sx - ccx, sy - ccy);
        const double re = std::hypot(ex - ccx, ey - ccy);
        if (rs <= 0.0) return false;
        /* TwinCAT adjusts slightly unequal radii; reject large
           mismatches */
        if (std::fabs(re - rs) > std::max(1.0, 0.05 * rs))
            return false;
    } else if (mode == TPLC_PATH_CIRC_BORDER) {
        /* circumcenter of S, A(border point), E in 2D */
        const double d = 2.0 * (sx * (ay - ey) + ax * (ey - sy) +
                                ex * (sy - ay));
        if (std::fabs(d) < 1e-12) return false;
        const double a2 = sx * sx + sy * sy;
        const double b2 = ax * ax + ay * ay;
        const double c2 = ex * ex + ey * ey;
        ccx = (a2 * (ay - ey) + b2 * (ey - sy) + c2 * (sy - ay)) / d;
        ccy = (a2 * (ex - sx) + b2 * (sx - ex) + c2 * (ax - sx)) / d;
    } else {
        return false;
    }
    const double rr = std::hypot(sx - ccx, sy - ccy);
    if (rr <= 0.0) return false;
    double aS = std::atan2(sy - ccy, sx - ccx);
    double aE = std::atan2(ey - ccy, ex - ccx);
    double sw;
    if (choice == 2.0) {           /* clockwise */
        sw = aS - aE;
        while (sw <= 0.0) sw += 2.0 * M_PI;
        sw = -sw;
        if (sw == 0.0) sw = -2.0 * M_PI;
    } else {                       /* counter-clockwise */
        sw = aE - aS;
        while (sw <= 0.0) sw += 2.0 * M_PI;
        if (sw == 0.0) sw = 2.0 * M_PI;
    }
    *cx = ccx; *cy = ccy; *r = rr; *sweep = sw;
    return true;
}

} // namespace

void PathService::recompute(Path& p) {
    double acc_t = 0.0;
    for (uint32_t i = 0; i < p.nseg; ++i) {
        Segment& q = p.segs[i];
        const double* from = (i == 0) ? p.start.data()
                                      : p.segs[i - 1].to.data();
        if (q.type == TPLC_PATH_SEG_CIRC) {
            double cx, cy, rr, sw;
            if (circGeom(q.circMode, q.aux[0], q.aux[1], from[0],
                         from[1], q.to[0], q.to[1], q.circChoice,
                         &cx, &cy, &rr, &sw)) {
                const double arc = rr * std::fabs(sw);
                const double v = q.vel > 0.0 ? q.vel : 1.0;
                const double a = q.acc > 0.0 ? q.acc : v;
                const double dAcc = v * v / (2.0 * a);
                if (2.0 * dAcc >= arc) {
                    const double vp = std::sqrt(arc * a);
                    q.dur = 2.0 * vp / a;
                } else {
                    q.dur = 2.0 * (v / a) + (arc - 2.0 * dAcc) / v;
                }
            } else {
                q.dur = 0.0;
            }
        } else {
            q.dur = segDuration(q.type, q.n, q.to.data(), from, q.vel,
                                q.acc, q.dwellMs);
        }
        q.t0 = acc_t;
        acc_t += q.dur;
    }
    p.duration = acc_t;
}

int32_t PathService::create(void*, uint32_t* out) {
    if (out == nullptr) return TPLC_PATH_E_PARAM;
    for (uint32_t i = 0; i < TPLC_PATH_MAX_PATHS; ++i) {
        if (!paths_[i].used) {
            paths_[i] = Path{};
            paths_[i].used = true;
            *out = i + 1;
            return TPLC_PATH_OK;
        }
    }
    return TPLC_PATH_E_FULL;
}

int32_t PathService::clear(void*, uint32_t path) {
    Path* p = find(path);
    if (p == nullptr) return TPLC_PATH_E_STATE;
    *p = Path{};
    p->used = true;
    return TPLC_PATH_OK;
}

int32_t PathService::add(void*, uint32_t path, uint32_t type,
                         const double* pos, uint32_t n, double vel,
                         double acc, double dwellMs) {
    Path* p = find(path);
    if (p == nullptr) return TPLC_PATH_E_STATE;
    if (p->nseg >= TPLC_PATH_MAX_SEGS) return TPLC_PATH_E_FULL;
    if (n > TPLC_PATH_MAX_AXES) return TPLC_PATH_E_PARAM;
    if (type != TPLC_PATH_SEG_LINEAR && type != TPLC_PATH_SEG_DWELL &&
        type != TPLC_PATH_SEG_BLEND)
        return TPLC_PATH_E_PARAM;
    if (type == TPLC_PATH_SEG_LINEAR && (pos == nullptr || n == 0))
        return TPLC_PATH_E_PARAM;
    Segment& s = p->segs[p->nseg];
    s = Segment{};
    s.type = type;
    s.n = n;
    s.vel = vel;
    s.acc = acc;
    s.dwellMs = dwellMs;
    if (type == TPLC_PATH_SEG_LINEAR) {
        for (uint32_t i = 0; i < n; ++i) s.to[i] = pos[i];
    } else {
        /* DWELL and BLEND hold the previous segment target */
        s.n = p->nseg > 0 ? p->segs[p->nseg - 1].n : n;
        if (s.n > TPLC_PATH_MAX_AXES) s.n = TPLC_PATH_MAX_AXES;
        for (uint32_t i = 0; i < s.n; ++i)
            s.to[i] = p->nseg > 0 ? p->segs[p->nseg - 1].to[i]
                                  : (p->started ? p->start[i] : 0.0);
    }
    p->nseg += 1;
    /* full recompute keeps CIRC durations (which depend on the
       segment start) consistent when segments are appended after a
       circular move */
    recompute(*p);
    return TPLC_PATH_OK;
}

int32_t PathService::setStart(void*, uint32_t path, const double* pos,
                              uint32_t n) {
    Path* p = find(path);
    if (p == nullptr) return TPLC_PATH_E_STATE;
    if (pos == nullptr) return TPLC_PATH_E_PARAM;
    if (n > TPLC_PATH_MAX_AXES) n = TPLC_PATH_MAX_AXES;
    for (uint32_t i = 0; i < n; ++i) p->start[i] = pos[i];
    /* recompute durations/t0 in case segments were added before the
       start was known */
    recompute(*p);
    p->started = true;
    return TPLC_PATH_OK;
}

int32_t PathService::eval(void*, uint32_t path, double t, double* pos,
                          uint32_t cap, uint32_t* seg, int32_t* done,
                          uint32_t* n_out) {
    Path* p = find(path);
    if (p == nullptr) return TPLC_PATH_E_STATE;
    if (pos == nullptr || seg == nullptr || done == nullptr ||
        n_out == nullptr)
        return TPLC_PATH_E_PARAM;
    if (p->nseg == 0) return TPLC_PATH_E_EMPTY;
    if (t < 0.0) t = 0.0;
    uint32_t idx = p->nseg - 1;
    for (uint32_t i = 0; i < p->nseg; ++i) {
        if (t < p->segs[i].t0 + p->segs[i].dur || i == p->nseg - 1) {
            idx = i;
            break;
        }
    }
    const Segment& s = p->segs[idx];
    std::array<double, TPLC_PATH_MAX_AXES> from{};
    if (idx == 0) from = p->start;
    else from = p->segs[idx - 1].to;
    const double lt = t - s.t0;
    double prog = 1.0;
    if (s.type == TPLC_PATH_SEG_LINEAR) {
        double dmax = 0.0;
        for (uint32_t i = 0; i < s.n && i < TPLC_PATH_MAX_AXES; ++i) {
            const double d = std::fabs(s.to[i] - from[i]);
            if (d > dmax) dmax = d;
        }
        prog = trapProgress(dmax, s.vel, s.acc, lt, nullptr);
    }
    const uint32_t n = s.n < cap ? s.n : cap;
    if (s.type == TPLC_PATH_SEG_CIRC) {
        double cx, cy, rr, sw;
        if (circGeom(s.circMode, s.aux[0], s.aux[1], from[0],
                     from[1], s.to[0], s.to[1], s.circChoice,
                     &cx, &cy, &rr, &sw)) {
            const double arc = rr * std::fabs(sw);
            prog = trapProgress(arc, s.vel, s.acc, lt, nullptr);
            const double aS = std::atan2(from[1] - cy, from[0] - cx);
            const double ang = aS + sw * prog;
            if (n > 0) pos[0] = cx + rr * std::cos(ang);
            if (n > 1) pos[1] = cy + rr * std::sin(ang);
            for (uint32_t i = 2; i < n; ++i) {
                pos[i] = from[i] + (s.to[i] - from[i]) * prog;
            }
        } else {
            for (uint32_t i = 0; i < n; ++i) {
                pos[i] = from[i] + (s.to[i] - from[i]) * prog;
            }
        }
    } else {
        for (uint32_t i = 0; i < n; ++i) {
            pos[i] = from[i] + (s.to[i] - from[i]) * prog;
        }
    }
    for (uint32_t i = n; i < cap; ++i) pos[i] = 0.0;
    *seg = idx;
    *done = (t >= p->duration) ? 1 : 0;
    *n_out = s.n;
    return TPLC_PATH_OK;
}

int32_t PathService::addCirc(void*, uint32_t path, uint32_t mode,
                             const double* aux, uint32_t naux,
                             const double* end, uint32_t nend,
                             double vel, double acc,
                             double pathChoice) {
    Path* p = find(path);
    if (p == nullptr) return TPLC_PATH_E_STATE;
    if (p->nseg >= TPLC_PATH_MAX_SEGS) return TPLC_PATH_E_FULL;
    if ((mode != TPLC_PATH_CIRC_CENTER &&
         mode != TPLC_PATH_CIRC_BORDER) ||
        aux == nullptr || end == nullptr || nend == 0 ||
        nend > TPLC_PATH_MAX_AXES || naux > TPLC_PATH_MAX_AXES)
        return TPLC_PATH_E_PARAM;
    Segment& s = p->segs[p->nseg];
    s = Segment{};
    s.type = TPLC_PATH_SEG_CIRC;
    s.circMode = mode;
    s.circChoice = pathChoice;
    s.n = nend;
    s.vel = vel;
    s.acc = acc;
    for (uint32_t i = 0; i < nend; ++i) s.to[i] = end[i];
    const uint32_t na = naux < TPLC_PATH_MAX_AXES ? naux
                                                 : TPLC_PATH_MAX_AXES;
    for (uint32_t i = 0; i < na; ++i) s.aux[i] = aux[i];
    s.dur = 0.0; /* resolved in setStart/eval once start is known */
    s.t0 = p->duration;
    p->segs[p->nseg] = s;
    p->nseg += 1;
    recompute(*p);
    return TPLC_PATH_OK;
}

int32_t PathService::info(void*, uint32_t path, uint32_t* nseg,
                          double* duration) {
    Path* p = find(path);
    if (p == nullptr) return TPLC_PATH_E_STATE;
    if (nseg != nullptr) *nseg = p->nseg;
    if (duration != nullptr) *duration = p->duration;
    return TPLC_PATH_OK;
}

} // namespace tplc
