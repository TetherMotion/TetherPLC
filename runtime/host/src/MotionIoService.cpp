/* MotionIoService.cpp — tplc-motion backend over the Tether IO protocol.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/MotionIoService.hpp>

#include <cstdlib>
#include <cstring>

#if defined(TPLC_HAVE_TETHER_IO)

#include "tether/io/TetherIOClient.hpp"
#include "tether/io/TcpTransport.hpp"
#include "tether/io/SlipTransport.hpp"

#include <string>
#include <vector>

namespace tplc {

using namespace tether::io;

namespace {

/* CiA402 state enum values (mirror MotionService / tplc_motion.h). */
enum IoState : int32_t {
    kNotReady        = 0x00,
    kSwitchOnDis     = 0x40,
    kReadyToSwitchOn = 0x21,
    kSwitchedOn      = 0x23,
    kOpEnabled       = 0x27,
    kQuickStopActive = 0x07,
    kFaultReact      = 0x0F,
    kFault           = 0x08,
};

/* Decode a CiA402 statusword into the state enum the MC layer expects.
   Statusword bits: 0=ready,1=switched-on,2=op-enabled,3=fault,
   5=quickstop-ok,6=switch-on-disabled. */
int32_t decodeState(uint16_t sw) {
    const bool fault = (sw & 0x0008) != 0;
    const bool opEn  = (sw & 0x0004) != 0;
    const bool swOn  = (sw & 0x0002) != 0;
    const bool rdy   = (sw & 0x0001) != 0;
    const bool qsOk  = (sw & 0x0020) != 0;
    const bool dis   = (sw & 0x0040) != 0;
    if (fault)
        return ((sw & 0x0007) == 0x0007) ? kFaultReact : kFault;
    if (dis)   return kSwitchOnDis;
    if (opEn)  return qsOk ? kOpEnabled : kQuickStopActive;
    if (swOn)  return kSwitchedOn;
    if (rdy)   return kReadyToSwitchOn;
    return kNotReady;
}

/* Native byte width of a DS402 object for get/setParam encoding. */
size_t objWidth(uint32_t index) {
    switch (index) {
        case 0x6060: case 0x6061:                 return 1; /* mode i8    */
        case 0x603F: case 0x6040: case 0x6041:
        case 0x6071: case 0x6077:                 return 2; /* u16/i16   */
        default:                                  return 4; /* i32/u32   */
    }
}

/* Decode little-endian param bytes to int32. U16 objects
   (error/controlword/statusword) zero-extend; narrower signed
   objects sign-extend per the DS402 definition. */
int32_t decodeLE(uint32_t index, const std::vector<uint8_t>& b) {
    if (b.empty()) return 0;
    const size_t n = std::min<size_t>(b.size(), 4);
    uint32_t v = 0;
    for (size_t i = 0; i < n; ++i) v |= uint32_t(b[i]) << (i * 8);
    switch (index) {
        case 0x603F: case 0x6040: case 0x6041:
            return int32_t(v & 0xFFFF);              /* U16: zero-ext */
        default: break;
    }
    switch (n) {
        case 1: return int32_t(int8_t(v & 0xFF));
        case 2: return int32_t(int16_t(v & 0xFFFF));
        case 3: return int32_t(int32_t(v << 8) >> 8);   /* sign-extend */
        default: return int32_t(v);
    }
}

void encodeLE(uint32_t index, int32_t value, uint8_t* out) {
    const size_t n = objWidth(index);
    const uint32_t v = uint32_t(value);
    for (size_t i = 0; i < n; ++i) out[i] = uint8_t(v >> (i * 8));
}

/* Parse "tcp://host:port" or "host:port" into host/port. */
bool parseEndpoint(const std::string& ep, std::string& host, uint16_t& port) {
    std::string s = ep;
    const std::string p = "tcp://";
    if (s.rfind(p, 0) == 0) s = s.substr(p.size());
    const size_t colon = s.rfind(':');
    if (colon == std::string::npos) return false;
    host = s.substr(0, colon);
    port = uint16_t(std::stoi(s.substr(colon + 1)));
    return !host.empty();
}

/* Catalog key for (slave,index): slave in high 16 bits, index low 16. */
uint64_t objKey(uint16_t slave, uint16_t index) {
    return (uint64_t(slave) << 16) | index;
}

} // namespace

struct MotionIoService::Impl {
    std::unique_ptr<TetherIOClient> client;
    /* (slave,index) -> catalog param id */
    std::map<uint64_t, uint64_t> paramId;
    /* axis handle -> slave index */
    std::map<uint32_t, uint16_t> axisSlave;
    /* slave -> in-use flag */
    std::vector<bool> used;
    /* mc.time param id (0 = absent) */
    uint64_t timeParamId = 0;
};

MotionIoService::MotionIoService(ClockFn clock, void* clock_ctx)
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

    /* Discover mc.<slave>.<INDEX> params and the slave count. */
    auto params = impl_->client->listParams(0, 4096);
    if (!params) { impl_.reset(); return; }
    uint16_t maxSlave = 0;
    for (const auto& e : *params) {
        if (e.name == "mc.time") { impl_->timeParamId = e.id; continue; }
        /* name form: mc.<slave>.<HEXINDEX> (group may prefix display) */
        const std::string& n = e.name;
        const size_t d1 = n.find('.');
        const size_t d2 = n.rfind('.');
        if (d1 == std::string::npos || d2 == d1) continue;
        if (n.compare(0, 3, "mc.") != 0) continue;
        const uint16_t slave = uint16_t(std::stoi(n.substr(3, d2 - 3)));
        const uint16_t index = uint16_t(std::stoul(n.substr(d2 + 1), nullptr, 16));
        impl_->paramId[objKey(slave, index)] = e.id;
        if (slave + 1 > maxSlave) maxSlave = slave + 1;
    }
    if (impl_->paramId.empty()) { impl_.reset(); return; }
    impl_->used.assign(maxSlave, false);

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
    active_ = true;
}

MotionIoService::~MotionIoService() = default;

int32_t MotionIoService::axisCreate(void*, const TPLC_MotionAxisCfg* cfg,
                                    uint32_t* out) {
    if (!cfg || !out || !impl_) return TPLC_MOTION_E_PARAM;
    if (cfg->max_velocity <= 0 || cfg->max_accel <= 0 ||
        cfg->max_decelerate <= 0)
        return TPLC_MOTION_E_PARAM;
    for (uint16_t s = 0; s < impl_->used.size(); ++s) {
        if (!impl_->used[s]) {
            impl_->used[s] = true;
            impl_->axisSlave[uint32_t(s) + 1] = s;
            *out = uint32_t(s) + 1;   /* handle = slave + 1 */
            return TPLC_MOTION_OK;
        }
    }
    return TPLC_MOTION_E_FULL;
}

int32_t MotionIoService::axisDestroy(void*, uint32_t axis) {
    if (!impl_) return TPLC_MOTION_E_STATE;
    const auto it = impl_->axisSlave.find(axis);
    if (it == impl_->axisSlave.end()) return TPLC_MOTION_E_STATE;
    impl_->used[it->second] = false;
    impl_->axisSlave.erase(it);
    return TPLC_MOTION_OK;
}

int32_t MotionIoService::axisPoll(void*) {
    /* Advance the simulated drives to the current PLC --sim time via
       mc.time; the Tether server integrates on that clock. */
    if (!impl_) return TPLC_MOTION_E_STATE;
    syncTime();
    return TPLC_MOTION_OK;
}

void MotionIoService::syncTime() {
    if (!clock_ || !impl_ || !impl_->timeParamId) return;
    const uint64_t us = clock_(clock_ctx_) / 1000;   /* ns -> us */
    if (us == last_time_us_) return;
    last_time_us_ = us;
    impl_->client->setParam(impl_->timeParamId, &us, 8);
}

int32_t MotionIoService::axisInfo(void*, uint32_t axis,
                                  TPLC_MotionAxisInfo* info) {
    if (!info || !impl_) return TPLC_MOTION_E_PARAM;
    syncTime();
    int32_t sw = 0, mode = 0, err = 0, pos = 0, vel = 0;
    const auto it = impl_->axisSlave.find(axis);
    if (it == impl_->axisSlave.end()) return TPLC_MOTION_E_STATE;
    const uint16_t slave = it->second;
    auto rd = [&](uint16_t idx, int32_t* out) -> bool {
        const auto p = impl_->paramId.find(objKey(slave, idx));
        if (p == impl_->paramId.end()) return false;
        auto v = impl_->client->getParam(p->second);
        if (!v) return false;
        *out = decodeLE(idx, *v);
        return true;
    };
    if (!rd(0x6041, &sw) || !rd(0x6061, &mode) || !rd(0x603F, &err) ||
        !rd(0x6064, &pos) || !rd(0x606C, &vel))
        return TPLC_MOTION_E_STATE;
    const uint16_t swu = uint16_t(sw);
    info->status = decodeState(swu);
    info->mode = mode;
    info->fault = (info->status == kFault || info->status == kFaultReact)
                      ? (err ? err : 1) : 0;
    info->target_reached = (swu & 0x0400) ? 1 : 0;
    info->busy = (info->status == kOpEnabled && !info->target_reached) ? 1 : 0;
    info->pos_actual = pos;
    info->vel_actual = vel;
    return TPLC_MOTION_OK;
}

int32_t MotionIoService::objRead(void*, uint32_t axis, uint32_t index,
                                 int32_t* out) {
    if (!out || !impl_ || index > 0xFFFF) return TPLC_MOTION_E_PARAM;
    syncTime();
    const auto it = impl_->axisSlave.find(axis);
    if (it == impl_->axisSlave.end()) return TPLC_MOTION_E_STATE;
    const auto p = impl_->paramId.find(objKey(it->second, uint16_t(index)));
    if (p == impl_->paramId.end()) return TPLC_MOTION_E_STATE;
    auto v = impl_->client->getParam(p->second);
    if (!v) return TPLC_MOTION_E_STATE;
    *out = decodeLE(index, *v);
    return TPLC_MOTION_OK;
}

int32_t MotionIoService::objWrite(void*, uint32_t axis, uint32_t index,
                                  int32_t value) {
    if (!impl_ || index > 0xFFFF) return TPLC_MOTION_E_PARAM;
    syncTime();
    const auto it = impl_->axisSlave.find(axis);
    if (it == impl_->axisSlave.end()) return TPLC_MOTION_E_STATE;
    const auto p = impl_->paramId.find(objKey(it->second, uint16_t(index)));
    if (p == impl_->paramId.end()) return TPLC_MOTION_E_STATE;
    /* The wire cannot carry drive-side write errors (param writeFn is
       void), so constrain the objects the in-process backend rejects. */
    if (index == 0x6060) {
        switch (value) {
            case 0: case 1: case 2: case 3: case 4:
            case 6: case 8: case 9: case 10:
                break;
            default:
                return TPLC_MOTION_E_MODE;
        }
    } else if (index == 0x6065 && value < 0) {
        value = 0;   /* follow window clamps like the local backend */
    }
    uint8_t buf[4] = {0, 0, 0, 0};
    encodeLE(uint32_t(index), value, buf);
    const size_t n = objWidth(uint32_t(index));
    auto r = impl_->client->setParam(p->second, buf, uint8_t(n));
    return r ? TPLC_MOTION_OK : TPLC_MOTION_E_STATE;
}

} // namespace tplc

#else /* !TPLC_HAVE_TETHER_IO */

namespace tplc {

/* Keep Impl complete so the unique_ptr member destructs cleanly. */
struct MotionIoService::Impl {};

MotionIoService::MotionIoService() = default;
MotionIoService::~MotionIoService() = default;

int32_t MotionIoService::axisCreate(void*, const TPLC_MotionAxisCfg*,
                                    uint32_t*) { return TPLC_MOTION_E_STATE; }
int32_t MotionIoService::axisDestroy(void*, uint32_t) {
    return TPLC_MOTION_E_STATE;
}
int32_t MotionIoService::axisPoll(void*) { return TPLC_MOTION_E_STATE; }
void MotionIoService::syncTime() {}
int32_t MotionIoService::axisInfo(void*, uint32_t, TPLC_MotionAxisInfo*) {
    return TPLC_MOTION_E_STATE;
}
int32_t MotionIoService::objRead(void*, uint32_t, uint32_t, int32_t*) {
    return TPLC_MOTION_E_STATE;
}
int32_t MotionIoService::objWrite(void*, uint32_t, uint32_t, int32_t) {
    return TPLC_MOTION_E_STATE;
}

} // namespace tplc

#endif /* TPLC_HAVE_TETHER_IO */
