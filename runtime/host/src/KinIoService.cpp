/* KinIoService.cpp — tplc-kin bridge over the Tether IO protocol.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/KinIoService.hpp>

#include <cstdlib>
#include <cstring>

#if defined(TPLC_HAVE_TETHER_IO)

#include "tether/io/TetherIOClient.hpp"
#include "tether/io/TcpTransport.hpp"
#include "tether/io/SlipTransport.hpp"

#include <string>

namespace tplc {

using namespace tether::io;

namespace {

bool parseEndpoint(const std::string& ep, std::string& host,
                   uint16_t& port) {
    std::string s = ep;
    const std::string p = "tcp://";
    if (s.rfind(p, 0) == 0) s = s.substr(p.size());
    const size_t colon = s.rfind(':');
    if (colon == std::string::npos) return false;
    host = s.substr(0, colon);
    port = uint16_t(std::stoi(s.substr(colon + 1)));
    return !host.empty();
}

} // namespace

struct KinIoService::Impl {
    std::unique_ptr<TetherIOClient> client;
    uint64_t obj = 0, type = 0, preset = 0, cfg = 0, dof = 0,
             locked = 0, in = 0, out = 0, q = 0, cmd = 0, state = 0,
             err = 0, acs = 0, mcs = 0, nacs = 0, nmcs = 0, rot = 0,
             chn = 0, chnst = 0;
    uint64_t timeParamId = 0;
};

KinIoService::KinIoService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    const char* ep = std::getenv("TPLC_MOTION_IO");
    if (!ep || !*ep) return;

    std::string host;
    uint16_t port = 0;
    if (!parseEndpoint(ep, host, port)) return;

    auto tcp = tcpConnect(host.c_str(), port, 2000);
    if (!tcp) return;
    impl_ = std::make_unique<Impl>();
    impl_->client = std::make_unique<TetherIOClient>(
        std::make_unique<SlipFramedTransport>(std::move(tcp)), 2000);
    if (!impl_->client->isConnected()) { impl_.reset(); return; }

    auto params = impl_->client->listParams(0, 4096);
    if (!params) { impl_.reset(); return; }
    for (const auto& e : *params) {
        if      (e.name == "mc.time")        impl_->timeParamId = e.id;
        else if (e.name == "kin.obj")        impl_->obj = e.id;
        else if (e.name == "kin.type")       impl_->type = e.id;
        else if (e.name == "kin.preset")     impl_->preset = e.id;
        else if (e.name == "kin.cfg")        impl_->cfg = e.id;
        else if (e.name == "kin.dof")        impl_->dof = e.id;
        else if (e.name == "kin.locked")     impl_->locked = e.id;
        else if (e.name == "kin.in")         impl_->in = e.id;
        else if (e.name == "kin.out")        impl_->out = e.id;
        else if (e.name == "kin.q")          impl_->q = e.id;
        else if (e.name == "kin.cmd")        impl_->cmd = e.id;
        else if (e.name == "kin.state")      impl_->state = e.id;
        else if (e.name == "kin.err")        impl_->err = e.id;
        else if (e.name == "kin.acs")        impl_->acs = e.id;
        else if (e.name == "kin.mcs")        impl_->mcs = e.id;
        else if (e.name == "kin.nacs")       impl_->nacs = e.id;
        else if (e.name == "kin.nmcs")       impl_->nmcs = e.id;
        else if (e.name == "kin.rot")        impl_->rot = e.id;
        else if (e.name == "kin.chn")        impl_->chn = e.id;
        else if (e.name == "kin.chnstate")   impl_->chnst = e.id;
    }
    /* The kin surface is present only when the server exposes it. */
    if (!impl_->obj || !impl_->type || !impl_->cfg || !impl_->in ||
        !impl_->out || !impl_->cmd || !impl_->state) {
        impl_.reset();
        return;
    }

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
    active_ = true;
}

KinIoService::~KinIoService() = default;

void KinIoService::syncTime() {
    if (!clock_ || !impl_ || !impl_->timeParamId) return;
    const uint64_t us = clock_(clock_ctx_) / 1000;
    if (us == last_time_us_) return;
    last_time_us_ = us;
    impl_->client->setParam(impl_->timeParamId, &us, 8);
}

bool KinIoService::selectObj(uint32_t obj) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX) return false;
    if (obj == lastObj_) return true;
    const uint8_t v = uint8_t(obj);
    if (!impl_->client->setParam(impl_->obj, &v, 1).has_value())
        return false;
    lastObj_ = obj;
    return true;
}

static bool getBytes(TetherIOClient& c, uint64_t id, void* dst,
                     size_t n) {
    if (!id) return false;
    auto v = c.getParam(id);
    if (!v || v->size() < n) return false;
    std::memcpy(dst, v->data(), n);
    return true;
}

int32_t KinIoService::configure(void*, uint32_t obj, int32_t type,
                                uint32_t preset, const double* cfg,
                                uint32_t ncfg) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX || type < -1 ||
        type > TPLC_KIN_T_STEWART || ncfg > TPLC_KIN_MAX_CFG)
        return TPLC_KIN_E_PARAM;
    if (!cfg && ncfg) return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    /* Write order: cfg first (ungated content), then preset, then
       type last so the server rebuilds once with all values in. */
    if (ncfg) {
        uint8_t blob[4 + TPLC_KIN_MAX_CFG * 8];
        const uint32_t n = ncfg;
        std::memcpy(blob, &n, 4);
        for (uint32_t i = 0; i < n; ++i)
            std::memcpy(blob + 4 + i * 8, &cfg[i], 8);
        if (!impl_->client->setParamVar(impl_->cfg, blob,
                                        4 + size_t(n) * 8))
            return TPLC_KIN_E_IO;
    }
    const uint8_t pv = uint8_t(preset & 0xFF);
    if (!impl_->client->setParam(impl_->preset, &pv, 1))
        return TPLC_KIN_E_IO;
    const int32_t tv = type;
    if (!impl_->client->setParam(impl_->type, &tv, 4))
        return TPLC_KIN_E_IO;
    /* The server silently drops type/preset/cfg writes while locked;
       read back to distinguish a lock rejection. */
    int32_t back = -2;
    if (!getBytes(*impl_->client, impl_->type, &back, 4))
        return TPLC_KIN_E_IO;
    if (back != type) return TPLC_KIN_E_LOCKED;
    return TPLC_KIN_OK;
}

int32_t KinIoService::setAxes(void*, uint32_t obj, const uint32_t* acs,
                              uint32_t nacs, const uint32_t* mcs,
                              uint32_t nmcs) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX || nacs > TPLC_KIN_MAX_DIM ||
        nmcs > TPLC_KIN_MAX_DIM)
        return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    if (impl_->acs) {
        uint8_t blob[4 + TPLC_KIN_MAX_DIM * 4];
        const uint32_t n = nacs;
        std::memcpy(blob, &n, 4);
        for (uint32_t i = 0; i < n; ++i)
            std::memcpy(blob + 4 + i * 4, &acs[i], 4);
        if (!impl_->client->setParamVar(impl_->acs, blob,
                                        4 + size_t(n) * 4))
            return TPLC_KIN_E_IO;
    }
    if (impl_->mcs) {
        uint8_t blob[4 + TPLC_KIN_MAX_DIM * 4];
        const uint32_t n = nmcs;
        std::memcpy(blob, &n, 4);
        for (uint32_t i = 0; i < n; ++i)
            std::memcpy(blob + 4 + i * 4, &mcs[i], 4);
        if (!impl_->client->setParamVar(impl_->mcs, blob,
                                        4 + size_t(n) * 4))
            return TPLC_KIN_E_IO;
    }
    return TPLC_KIN_OK;
}

int32_t KinIoService::commandOp(void*, uint32_t obj, uint32_t cmd) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX) return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    const uint8_t v = uint8_t(cmd);
    auto r = impl_->client->setParam(impl_->cmd, &v, 1);
    return r ? TPLC_KIN_OK : TPLC_KIN_E_IO;
}

int32_t KinIoService::calcInput(void*, uint32_t obj, const double* in,
                                uint32_t n) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX || !in ||
        n > TPLC_KIN_MAX_DIM)
        return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    uint8_t blob[4 + TPLC_KIN_MAX_DIM * 8];
    const uint32_t nn = n;
    std::memcpy(blob, &nn, 4);
    for (uint32_t i = 0; i < n; ++i)
        std::memcpy(blob + 4 + i * 8, &in[i], 8);
    auto r = impl_->client->setParamVar(impl_->in, blob,
                                        4 + size_t(n) * 8);
    return r ? TPLC_KIN_OK : TPLC_KIN_E_IO;
}

int32_t KinIoService::result(void*, uint32_t obj, double* out8,
                             double* q8) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX) return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    if (out8 &&
        !getBytes(*impl_->client, impl_->out, out8,
                  TPLC_KIN_MAX_DIM * 8))
        return TPLC_KIN_E_IO;
    if (q8 &&
        !getBytes(*impl_->client, impl_->q, q8,
                  TPLC_KIN_MAX_DIM * 8))
        return TPLC_KIN_E_IO;
    return TPLC_KIN_OK;
}

int32_t KinIoService::setLocked(void*, uint32_t obj, uint32_t locked) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX) return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    const uint8_t v = locked ? 1 : 0;
    auto r = impl_->client->setParam(impl_->locked, &v, 1);
    return r ? TPLC_KIN_OK : TPLC_KIN_E_IO;
}

int32_t KinIoService::rotation(void*, uint32_t obj, uint32_t op,
                               const double* rot3, double* out3) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX) return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    switch (op) {
        case TPLC_KIN_ROT_GET:
            if (!out3 ||
                !getBytes(*impl_->client, impl_->rot, out3, 24))
                return TPLC_KIN_E_IO;
            return TPLC_KIN_OK;
        case TPLC_KIN_ROT_PRESET: {
            if (!rot3) return TPLC_KIN_E_PARAM;
            auto rv = impl_->client->setParamVar(impl_->rot, rot3, 24);
            return rv ? TPLC_KIN_OK : TPLC_KIN_E_IO;
        }
        case TPLC_KIN_ROT_SAVE:
        case TPLC_KIN_ROT_RESTORE: {
            const uint8_t v = uint8_t(op == TPLC_KIN_ROT_SAVE
                                          ? TPLC_KIN_CMD_ROT_SAVE
                                          : TPLC_KIN_CMD_ROT_RESTORE);
            auto r = impl_->client->setParam(impl_->cmd, &v, 1);
            return r ? TPLC_KIN_OK : TPLC_KIN_E_IO;
        }
        default: return TPLC_KIN_E_PARAM;
    }
}

int32_t KinIoService::status(void*, uint32_t obj,
                             TplcKinStatus* out) {
    if (!impl_ || !out || obj >= TPLC_KIN_OBJ_MAX)
        return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    auto& c = *impl_->client;
    int32_t state = 0, type = -1, dof = 0, nacs = 0, nmcs = 0, err = 0,
            chn = 0, chnst = 0;
    uint32_t locked = 0;
    uint8_t b8;
    if (!getBytes(c, impl_->state, &b8, 1)) return TPLC_KIN_E_IO;
    state = b8;
    if (!getBytes(c, impl_->type, &type, 4) ||
        !getBytes(c, impl_->dof, &b8, 1))
        dof = 0;
    else
        dof = b8;
    if (impl_->nacs && getBytes(c, impl_->nacs, &b8, 1)) nacs = b8;
    if (impl_->nmcs && getBytes(c, impl_->nmcs, &b8, 1)) nmcs = b8;
    if (impl_->locked && getBytes(c, impl_->locked, &b8, 1))
        locked = b8;
    if (impl_->err && getBytes(c, impl_->err, &err, 2)) {}
    if (impl_->chn && getBytes(c, impl_->chn, &b8, 1)) chn = b8;
    if (impl_->chnst && getBytes(c, impl_->chnst, &b8, 1)) chnst = b8;
    out->state = state;
    out->type = type;
    out->dof = dof;
    out->nacs = nacs;
    out->nmcs = nmcs;
    out->locked = locked;
    out->err = err;
    out->chnstate = chnst;
    out->chn = chn;
    return TPLC_KIN_OK;
}

int32_t KinIoService::setChannel(void*, uint32_t obj, uint32_t chn) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX || chn > 255)
        return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    const uint8_t v = uint8_t(chn);
    auto r = impl_->client->setParam(impl_->chn, &v, 1);
    return r ? TPLC_KIN_OK : TPLC_KIN_E_IO;
}

/* kin.acs / kin.mcs blobs: u32 n + n u32. */
static bool getU32List(TetherIOClient& c, uint64_t id, uint32_t* dst,
                       uint32_t* n) {
    if (!id || !dst || !n) return false;
    auto v = c.getParam(id);
    if (!v || v->size() < 4) return false;
    uint32_t cnt;
    std::memcpy(&cnt, v->data(), 4);
    cnt = cnt > TPLC_KIN_MAX_DIM ? TPLC_KIN_MAX_DIM : cnt;
    const size_t have = (v->size() - 4) / 4;
    const uint32_t use = cnt < have ? cnt : uint32_t(have);
    for (uint32_t i = 0; i < use; ++i)
        std::memcpy(dst + i, v->data() + 4 + i * 4, 4);
    *n = use;
    return true;
}

int32_t KinIoService::getAxes(void*, uint32_t obj, uint32_t* acs,
                              uint32_t* nacs, uint32_t* mcs,
                              uint32_t* nmcs) {
    if (!impl_ || obj >= TPLC_KIN_OBJ_MAX)
        return TPLC_KIN_E_PARAM;
    syncTime();
    if (!selectObj(obj)) return TPLC_KIN_E_IO;
    if (acs && nacs &&
        !getU32List(*impl_->client, impl_->acs, acs, nacs))
        return TPLC_KIN_E_IO;
    if (mcs && nmcs &&
        !getU32List(*impl_->client, impl_->mcs, mcs, nmcs))
        return TPLC_KIN_E_IO;
    return TPLC_KIN_OK;
}

} // namespace tplc

#else /* !TPLC_HAVE_TETHER_IO */

namespace tplc {

struct KinIoService::Impl {};

KinIoService::KinIoService(ClockFn, void*) = default;
KinIoService::~KinIoService() = default;
void KinIoService::syncTime() {}
bool KinIoService::selectObj(uint32_t) { return false; }

int32_t KinIoService::configure(void*, uint32_t, int32_t, uint32_t,
                                const double*, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::setAxes(void*, uint32_t, const uint32_t*, uint32_t,
                              const uint32_t*, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::commandOp(void*, uint32_t, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::calcInput(void*, uint32_t, const double*, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::result(void*, uint32_t, double*, double*) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::setLocked(void*, uint32_t, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::rotation(void*, uint32_t, uint32_t, const double*,
                               double*) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::status(void*, uint32_t, TplcKinStatus*) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::setChannel(void*, uint32_t, uint32_t) {
    return TPLC_KIN_E_STATE;
}
int32_t KinIoService::getAxes(void*, uint32_t, uint32_t*, uint32_t*,
                              uint32_t*, uint32_t*) {
    return TPLC_KIN_E_STATE;
}

} // namespace tplc

#endif /* TPLC_HAVE_TETHER_IO */
