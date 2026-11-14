#pragma once
/* In-process tplc-cam service: fixed table of cam objects (master
   position -> slave position point tables) and per-slave cam
   bindings with coupling scalings. Pure PLC-side bookkeeping; axis
   motion stays in tplc-motion / Tether IO. */

#include "tplc/tplc_cam.h"

#include <array>
#include <cstdint>

namespace tplc {

class CamService {
public:
    CamService();

    TplcCamService& svc() { return svc_; }
    const TplcCamService& svc() const { return svc_; }

private:
    struct CamObj {
        bool     used = false;
        uint32_t npts = 0;
        uint32_t flags = 0;
        std::array<double, TPLC_CAM_MAX_PTS> m{};
        std::array<double, TPLC_CAM_MAX_PTS> s{};
    };
    struct Binding {
        bool     used = false;
        uint32_t slave = 0;
        uint32_t master = 0;
        uint32_t cam = 0;
        double   mscale = 1.0, moff = 0.0;
        double   sscale = 1.0, soff = 0.0;
    };

    static CamService* self(void* h) {
        return static_cast<CamService*>(h);
    }
    CamObj* findCam(uint32_t cam);
    Binding* findBind(uint32_t slave);
    const Binding* findBind(uint32_t slave) const;

    int32_t create(void*, uint32_t*);
    int32_t clear(void*, uint32_t);
    int32_t writePt(void*, uint32_t, uint32_t, double, double);
    int32_t config(void*, uint32_t, uint32_t, uint32_t);
    int32_t eval(void*, uint32_t, double, double*);
    int32_t info(void*, uint32_t, uint32_t*, uint32_t*);
    int32_t bind(void*, uint32_t, uint32_t, uint32_t,
                 double, double, double, double);
    int32_t unbind(void*, uint32_t, uint32_t);
    int32_t bound(void*, uint32_t, uint32_t*, uint32_t*, double*,
                  double*, double*, double*);
    int32_t scaling(void*, uint32_t, double, double, double, double);

    TplcCamService svc_{};
    std::array<CamObj, TPLC_CAM_MAX_OBJS> objs_{};
    std::array<Binding, TPLC_CAM_MAX_BIND> binds_{};
};

} // namespace tplc
