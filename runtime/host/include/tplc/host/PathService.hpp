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
        uint32_t circMode = 0; // TPLC_PATH_CIRC_* for CIRC segs
        double   circChoice = 1.0; // 1=CCW, 2=CW
        std::array<double, TPLC_PATH_MAX_AXES> to{};
        std::array<double, TPLC_PATH_MAX_AXES> aux{};
    };
    struct Path {
        bool     used = false;
        bool     started = false;
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

    TplcPathService svc_{};
    std::array<Path, TPLC_PATH_MAX_PATHS> paths_{};
};

} // namespace tplc
