#pragma once
/* In-process tplc-path service: fixed table of path objects
   (ordered linear/dwell segments with trapezoidal timing).
   Pure PLC-side bookkeeping; axis motion stays in
   tplc-motion / Tether IO. */

#include "tplc/tplc_path.h"

#include <array>
#include <cstdint>

namespace tplc {

class PathService {
public:
    PathService();

    TplcPathService& svc() { return svc_; }
    const TplcPathService& svc() const { return svc_; }

private:
    struct Segment {
        uint32_t type = 0;
        uint32_t n = 0;
        double   vel = 0.0;    // user units / s
        double   acc = 0.0;    // user units / s^2
        double   dwellMs = 0.0;
        double   dur = 0.0;    // seconds, computed at add
        double   t0 = 0.0;     // cumulative start time
        double   len = 0.0;    // dominant-axis length (arc for CIRC)
        uint32_t circMode = 0; // TPLC_PATH_CIRC_* for CIRC segs
        double   circChoice = 1.0; // 1=CCW, 2=CW
        bool     released = false; // BLOCKER gate opened
        std::array<double, TPLC_PATH_MAX_AXES> to{};
        std::array<double, TPLC_PATH_MAX_AXES> aux{};
    };
    struct Path {
        bool     used = false;
        bool     started = false;
        uint32_t group = 0;    // bound group handle (0 = none)
        double   lastT = 0.0;  // t of most recent eval
        uint32_t lastSeg = 0;  // segment of most recent eval
        double   pendTrafo = 0.0; // pending transform id
        uint32_t trafoGen = 0;    // bumps on pendTrafo change
        uint32_t nseg = 0;
        double   duration = 0.0;
        std::array<double, TPLC_PATH_MAX_AXES> start{};
        std::array<double, TPLC_PATH_MAX_AXES> last{};
        std::array<Segment, TPLC_PATH_MAX_SEGS> segs{};
    };

    static PathService* self(void* h) {
        return static_cast<PathService*>(h);
    }
    Path* find(uint32_t path);

    /* trapezoid progress in [0,1] for distance d at time lt with
       limits vel/acc (rest-to-rest) */
    static double trapProgress(double d, double vel, double acc,
                               double lt, double* peakVel);
    /* segment duration in seconds (linear: trapezoid on the
       dominant-axis distance; dwell: dwellMs) */
    static double segDuration(uint32_t type, uint32_t n,
                              const double* to,
                              const double* from, double vel,
                              double acc, double dwellMs);
    /* recompute every segment's duration and cumulative start
       time (CIRC durations depend on the segment start, which is
       only known after set_start) */
    void recompute(Path& p);

    int32_t create(void*, uint32_t*);
    int32_t clear(void*, uint32_t);
    int32_t add(void*, uint32_t, uint32_t, const double*, uint32_t,
                double, double, double);
    int32_t setStart(void*, uint32_t, const double*, uint32_t);
    int32_t eval(void*, uint32_t, double, double*, uint32_t,
                 uint32_t*, int32_t*, uint32_t*);
    int32_t info(void*, uint32_t, uint32_t*, double*);
    int32_t addCirc(void*, uint32_t, uint32_t, const double*,
                    uint32_t, const double*, uint32_t, double,
                    double, double);
    int32_t release(void*, double);
    int32_t blocker(void*, double*, int32_t*);
    int32_t bindGroup(void*, uint32_t, uint32_t);
    int32_t trafo(void*, uint32_t, double*, uint32_t*);
    int32_t traj(void*, uint32_t, double*, double*, uint32_t*,
                 uint32_t*, uint32_t, uint32_t*);

    TplcPathService svc_{};
    std::array<Path, TPLC_PATH_MAX_PATHS> paths_{};
    /* blocker wait state mirrored from the most recent eval */
    double   blkId_ = 0.0;
    int32_t  blkWait_ = 0;
};

} // namespace tplc
