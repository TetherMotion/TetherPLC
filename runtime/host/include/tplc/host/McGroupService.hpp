#pragma once
/* In-process tplc-mcgroup service: fixed table of coordinated-
   motion groups (axis membership, enable state, override,
   setpoint source, coordinate transform). Groups are PLC-side
   bookkeeping; the trajectory engine consumes them later. */

#include "tplc/tplc_mcgroup.h"

#include <array>
#include <cstdint>

namespace tplc {

class McGroupService {
public:
    McGroupService();

    TplcMcGroupService& svc() { return svc_; }
    const TplcMcGroupService& svc() const { return svc_; }

private:
    struct Group {
        bool     used = false;
        int32_t  state = TPLC_MCGROUP_ST_EMPTY;
        int32_t  errId = 0;
        double   override = 1.0;
        int32_t  source = TPLC_MCGROUP_SRC_GROUP;
        uint32_t coord = 0;
        uint32_t nAxes = 0;
        std::array<uint32_t, TPLC_MCGROUP_MAX_AXES> axes{};
        std::array<uint32_t, TPLC_MCGROUP_MAX_AXES> idents{};
    };

    static McGroupService* self(void* h) {
        return static_cast<McGroupService*>(h);
    }
    Group* find(uint32_t grp);
    const Group* find(uint32_t grp) const;

    int32_t ensure(void*, uint32_t, uint32_t*);
    int32_t addAxis(void*, uint32_t, uint32_t, uint32_t);
    int32_t removeAxis(void*, uint32_t, uint32_t, uint32_t);
    int32_t ungroup(void*, uint32_t);
    int32_t setState(void*, uint32_t, int32_t);
    int32_t status(void*, uint32_t, int32_t*, int32_t*, int32_t*, int32_t*);
    int32_t setOverride(void*, uint32_t, double, double*);
    int32_t setSource(void*, uint32_t, int32_t);
    int32_t setTransform(void*, uint32_t, uint32_t);
    int32_t getSource(void*, uint32_t, int32_t*);
    int32_t getTransform(void*, uint32_t, uint32_t*);
    int32_t getOverride(void*, uint32_t, double*);
    int32_t axes(void*, uint32_t, uint32_t*, uint32_t, int32_t*);

    TplcMcGroupService svc_{};
    std::array<Group, TPLC_MCGROUP_MAX_GROUPS> groups_{};
};

} // namespace tplc
