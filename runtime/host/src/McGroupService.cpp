#include "tplc/host/McGroupService.hpp"

namespace tplc {

McGroupService::McGroupService() {
    svc_.struct_size = sizeof(TplcMcGroupService);
    svc_.abi_version = TPLC_MCGROUP_ABI_VERSION;
    svc_.host = this;
    svc_.grp_ensure = [](void* h, uint32_t g, uint32_t* o) {
        return self(h)->ensure(h, g, o);
    };
    svc_.grp_add_axis = [](void* h, uint32_t g, uint32_t a, uint32_t i) {
        return self(h)->addAxis(h, g, a, i);
    };
    svc_.grp_remove_axis = [](void* h, uint32_t g, uint32_t a, uint32_t i) {
        return self(h)->removeAxis(h, g, a, i);
    };
    svc_.grp_ungroup = [](void* h, uint32_t g) {
        return self(h)->ungroup(h, g);
    };
    svc_.grp_set_state = [](void* h, uint32_t g, int32_t op) {
        return self(h)->setState(h, g, op);
    };
    svc_.grp_status = [](void* h, uint32_t g, int32_t* s, int32_t* n,
                         int32_t* e, int32_t* err) {
        return self(h)->status(h, g, s, n, e, err);
    };
    svc_.grp_set_override = [](void* h, uint32_t g, double o, double* a) {
        return self(h)->setOverride(h, g, o, a);
    };
    svc_.grp_set_source = [](void* h, uint32_t g, int32_t s) {
        return self(h)->setSource(h, g, s);
    };
    svc_.grp_set_transform = [](void* h, uint32_t g, uint32_t c) {
        return self(h)->setTransform(h, g, c);
    };
    svc_.grp_get_source = [](void* h, uint32_t g, int32_t* s) {
        return self(h)->getSource(h, g, s);
    };
    svc_.grp_get_transform = [](void* h, uint32_t g, uint32_t* c) {
        return self(h)->getTransform(h, g, c);
    };
    svc_.grp_get_override = [](void* h, uint32_t g, double* o) {
        return self(h)->getOverride(h, g, o);
    };
    svc_.grp_axes = [](void* h, uint32_t g, uint32_t* a, uint32_t cap,
                       int32_t* n) {
        return self(h)->axes(h, g, a, cap, n);
    };
}

McGroupService::Group* McGroupService::find(uint32_t grp) {
    if (grp == 0 || grp > TPLC_MCGROUP_MAX_GROUPS) return nullptr;
    Group& g = groups_[grp - 1];
    return g.used ? &g : nullptr;
}

const McGroupService::Group* McGroupService::find(uint32_t grp) const {
    if (grp == 0 || grp > TPLC_MCGROUP_MAX_GROUPS) return nullptr;
    const Group& g = groups_[grp - 1];
    return g.used ? &g : nullptr;
}

int32_t McGroupService::ensure(void*, uint32_t grp, uint32_t* out) {
    if (!out) return TPLC_MCGROUP_E_PARAM;
    if (grp == 0 || grp > TPLC_MCGROUP_MAX_GROUPS)
        return TPLC_MCGROUP_E_PARAM;
    Group& g = groups_[grp - 1];
    if (!g.used) {
        g = Group{};
        g.used = true;
        g.state = TPLC_MCGROUP_ST_DISABLED;
    }
    *out = grp;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::addAxis(void*, uint32_t grp, uint32_t axis,
                                uint32_t) {
    Group* g = find(grp);
    if (!g || axis == 0) return TPLC_MCGROUP_E_STATE;
    if (g->state != TPLC_MCGROUP_ST_DISABLED &&
        g->state != TPLC_MCGROUP_ST_EMPTY &&
        g->state != TPLC_MCGROUP_ST_ERRDIS)
        return TPLC_MCGROUP_E_STATEG;
    for (uint32_t i = 0; i < g->nAxes; ++i)
        if (g->axes[i] == axis) return TPLC_MCGROUP_OK; /* idempotent */
    if (g->nAxes >= TPLC_MCGROUP_MAX_AXES) return TPLC_MCGROUP_E_FULL;
    g->axes[g->nAxes] = axis;
    ++g->nAxes;
    if (g->state == TPLC_MCGROUP_ST_EMPTY) g->state = TPLC_MCGROUP_ST_DISABLED;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::removeAxis(void*, uint32_t grp, uint32_t axis,
                                   uint32_t) {
    Group* g = find(grp);
    if (!g) return TPLC_MCGROUP_E_STATE;
    if (g->state == TPLC_MCGROUP_ST_MOVING) return TPLC_MCGROUP_E_STATEG;
    for (uint32_t i = 0; i < g->nAxes; ++i) {
        if (g->axes[i] == axis) {
            for (uint32_t j = i + 1; j < g->nAxes; ++j) {
                g->axes[j - 1] = g->axes[j];
                g->idents[j - 1] = g->idents[j];
            }
            --g->nAxes;
            if (g->nAxes == 0) g->state = TPLC_MCGROUP_ST_EMPTY;
            return TPLC_MCGROUP_OK;
        }
    }
    return TPLC_MCGROUP_OK; /* not a member: DONE per TwinCAT */
}

int32_t McGroupService::ungroup(void*, uint32_t grp) {
    Group* g = find(grp);
    if (!g) return TPLC_MCGROUP_E_STATE;
    if (g->state == TPLC_MCGROUP_ST_MOVING) return TPLC_MCGROUP_E_STATEG;
    *g = Group{};
    g->used = true;
    g->state = TPLC_MCGROUP_ST_EMPTY;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::setState(void*, uint32_t grp, int32_t op) {
    Group* g = find(grp);
    if (!g) return TPLC_MCGROUP_E_STATE;
    switch (op) {
    case 1: /* enable */
        if (g->nAxes == 0) { g->errId = 1; return TPLC_MCGROUP_E_STATEG; }
        if (g->state == TPLC_MCGROUP_ST_ERRDIS ||
            g->state == TPLC_MCGROUP_ST_GRPERR) {
            g->errId = 2; return TPLC_MCGROUP_E_STATEG;
        }
        if (g->state == TPLC_MCGROUP_ST_MOVING) return TPLC_MCGROUP_E_STATEG;
        g->state = TPLC_MCGROUP_ST_STANDBY;
        g->errId = 0;
        return TPLC_MCGROUP_OK;
    case 0: /* disable */
        if (g->state == TPLC_MCGROUP_ST_MOVING) {
            /* halt semantics deferred to the motion tranche */
            g->state = TPLC_MCGROUP_ST_STANDBY;
        } else if (g->state != TPLC_MCGROUP_ST_EMPTY) {
            g->state = g->nAxes ? TPLC_MCGROUP_ST_DISABLED
                                : TPLC_MCGROUP_ST_EMPTY;
        }
        return TPLC_MCGROUP_OK;
    case 2: /* reset: back to disabled, clear error */
        g->state = g->nAxes ? TPLC_MCGROUP_ST_DISABLED
                            : TPLC_MCGROUP_ST_EMPTY;
        g->errId = 0;
        return TPLC_MCGROUP_OK;
    default:
        return TPLC_MCGROUP_E_PARAM;
    }
}

int32_t McGroupService::status(void*, uint32_t grp, int32_t* state,
                               int32_t* n, int32_t* en, int32_t* err) {
    const Group* g = find(grp);
    if (!g) return TPLC_MCGROUP_E_STATE;
    if (state) *state = g->state;
    if (n) *n = int32_t(g->nAxes);
    if (en) *en = (g->state == TPLC_MCGROUP_ST_STANDBY ||
                   g->state == TPLC_MCGROUP_ST_MOVING) ? 1 : 0;
    if (err) *err = g->errId;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::setOverride(void*, uint32_t grp, double ovr,
                                    double* actual) {
    Group* g = find(grp);
    if (!g) return TPLC_MCGROUP_E_STATE;
    if (ovr < 0.0) ovr = 0.0;
    if (ovr > 2.0) ovr = 2.0;
    g->override = ovr;
    if (actual) *actual = g->override;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::setSource(void*, uint32_t grp, int32_t src) {
    Group* g = find(grp);
    if (!g || src < 0 || src > 2) return TPLC_MCGROUP_E_PARAM;
    if (g->state == TPLC_MCGROUP_ST_MOVING) return TPLC_MCGROUP_E_STATEG;
    g->source = src;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::setTransform(void*, uint32_t grp, uint32_t coord) {
    Group* g = find(grp);
    if (!g) return TPLC_MCGROUP_E_STATE;
    g->coord = coord;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::getSource(void*, uint32_t grp, int32_t* src) {
    const Group* g = find(grp);
    if (!g || !src) return TPLC_MCGROUP_E_STATE;
    *src = g->source;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::getTransform(void*, uint32_t grp, uint32_t* c) {
    const Group* g = find(grp);
    if (!g || !c) return TPLC_MCGROUP_E_STATE;
    *c = g->coord;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::getOverride(void*, uint32_t grp, double* ovr) {
    const Group* g = find(grp);
    if (!g || !ovr) return TPLC_MCGROUP_E_STATE;
    *ovr = g->override;
    return TPLC_MCGROUP_OK;
}

int32_t McGroupService::axes(void*, uint32_t grp, uint32_t* out,
                             uint32_t cap, int32_t* n) {
    const Group* g = find(grp);
    if (!g) return TPLC_MCGROUP_E_STATE;
    if (n) *n = int32_t(g->nAxes);
    if (!out || cap == 0) return TPLC_MCGROUP_OK;
    const uint32_t c = g->nAxes < cap ? g->nAxes : cap;
    for (uint32_t i = 0; i < c; ++i) out[i] = g->axes[i];
    return TPLC_MCGROUP_OK;
}

} // namespace tplc
