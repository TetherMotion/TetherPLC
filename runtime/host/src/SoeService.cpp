/* SoeService.cpp — tplc-soe: EtherCAT SoE mailbox over the
 * ec.soe.* params on the Tether IO protocol.  SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#include <tplc/host/SoeService.hpp>

#include <cstdio>
#include <cstdlib>
#include <cstring>

#include "tether/io/TetherIOClient.hpp"
#include "tether/io/TcpTransport.hpp"
#include "tether/io/SlipTransport.hpp"

namespace tplc {

using namespace tether::io;

/* the rt bridge decodes TplcSoeState at fixed offsets; pin them */
static_assert(offsetof(TplcSoeState, status) == 0);
static_assert(offsetof(TplcSoeState, cmd_state) == 2);
static_assert(offsetof(TplcSoeState, password) == 4);
static_assert(offsetof(TplcSoeState, diag_num) == 8);
static_assert(offsetof(TplcSoeState, classx) == 10);
static_assert(offsetof(TplcSoeState, diag_count) == 12);
static_assert(offsetof(TplcSoeState, diag_list) == 14);
static_assert(offsetof(TplcSoeState, amp_temp) == 32);
static_assert(offsetof(TplcSoeState, ax_mains) == 48);
static_assert(offsetof(TplcSoeState, ax_ctrl) == 52);
static_assert(offsetof(TplcSoeState, cmd_ctrl) == 60);
static_assert(offsetof(TplcSoeState, diagmsg) == 62);
static_assert(sizeof(TplcSoeState) == 128);

namespace {

SoeService* self(void* s) { return static_cast<SoeService*>(s); }

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

uint16_t rd16(const uint8_t* p) {
    return uint16_t(p[0]) | (uint16_t(p[1]) << 8);
}
uint32_t rd32(const uint8_t* p) {
    uint32_t v = 0;
    for (int i = 0; i < 4; ++i) v |= uint32_t(p[i]) << (8 * i);
    return v;
}
float rdF32(const uint8_t* p) {
    uint32_t v = rd32(p);
    float f;
    std::memcpy(&f, &v, 4);
    return f;
}
void wr16(uint8_t* p, uint16_t v) {
    p[0] = uint8_t(v); p[1] = uint8_t(v >> 8);
}
void wr32(uint8_t* p, uint32_t v) {
    for (int i = 0; i < 4; ++i) p[i] = uint8_t(v >> (8 * i));
}

} // namespace

struct SoeService::Impl {
    std::unique_ptr<TetherIOClient> client;
    uint64_t idSlave = 0, idState = 0, idCmd = 0, idTime = 0;
    bool selectSlave(TetherIOClient& c, uint32_t slave);
    bool writeCmd(TetherIOClient& c, uint16_t op, uint32_t v);
};

SoeService::SoeService(ClockFn clock, void* clock_ctx)
    : clock_(clock), clock_ctx_(clock_ctx) {
    const char* ep = std::getenv("TPLC_EC_IO");
    if (!ep || !*ep) ep = std::getenv("TPLC_MOTION_IO");
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
        if (e.name == "ec.soe.slave") impl_->idSlave = e.id;
        else if (e.name == "ec.soe.state") impl_->idState = e.id;
        else if (e.name == "ec.soe.cmd") impl_->idCmd = e.id;
        else if (e.name == "mc.time") impl_->idTime = e.id;
    }
    if (!impl_->idSlave || !impl_->idState || !impl_->idCmd) {
        impl_.reset();
        return;
    }
    active_ = true;

    svc_.struct_size = sizeof(TplcSoeService);
    svc_.abi_version = TPLC_SOE_ABI_VERSION;
    svc_.host = this;
    svc_.soe_read = [](void* s, uint32_t slave, TplcSoeState* st) {
        return self(s)->read(s, slave, st);
    };
    svc_.soe_write_password = [](void* s, uint32_t slave, uint32_t pw) {
        return self(s)->writePassword(s, slave, pw);
    };
    svc_.soe_write_cmd_ctrl = [](void* s, uint32_t slave, uint32_t ctrl) {
        return self(s)->writeCmdCtrl(s, slave, ctrl);
    };
    svc_.soe_exec_command = [](void* s, uint32_t slave, uint32_t id) {
        return self(s)->execCommand(s, slave, id);
    };
    svc_.soe_reset = [](void* s, uint32_t slave) {
        return self(s)->reset(s, slave);
    };
    svc_.soe_write_ax_ctrl = [](void* s, uint32_t slave, uint32_t ctrl) {
        return self(s)->writeAxCtrl(s, slave, ctrl);
    };
    svc_.soe_write_ax_park = [](void* s, uint32_t slave, uint32_t park) {
        return self(s)->writeAxPark(s, slave, park);
    };
}

SoeService::~SoeService() = default;

bool SoeService::Impl::selectSlave(TetherIOClient& c, uint32_t slave) {
    uint8_t b[2];
    wr16(b, uint16_t(slave));
    auto r = c.setParam(idSlave, b, 2);
    return static_cast<bool>(r);
}

bool SoeService::Impl::writeCmd(TetherIOClient& c, uint16_t op,
                                uint32_t v) {
    uint8_t b[16];
    std::memset(b, 0, sizeof(b));
    wr16(b + 0, op);
    wr32(b + 4, v);
    auto r = c.setParam(idCmd, b, 16);
    return static_cast<bool>(r);
}

int32_t SoeService::read(void*, uint32_t slave, TplcSoeState* st) {
    if (!st) return TPLC_SOE_E_PARAM;
    std::memset(st, 0, sizeof(*st));
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_SOE_E_NOEP;
    syncTime();
    auto& c = *impl_->client;
    if (!impl_->selectSlave(c, slave)) return TPLC_SOE_E_IO;

    auto blob = c.getParam(impl_->idState);
    if (!blob || blob->size() < 128) return TPLC_SOE_E_IO;
    const uint8_t* p = blob->data();
    st->status = rd16(p + 0);
    st->cmd_state = rd16(p + 2);
    st->password = rd32(p + 4);
    st->diag_num = rd16(p + 8);
    st->classx = rd16(p + 10);
    st->diag_count = rd16(p + 12);
    if (st->diag_count > TPLC_SOE_DIAG_LIST)
        st->diag_count = TPLC_SOE_DIAG_LIST;
    for (uint32_t i = 0; i < st->diag_count; ++i)
        st->diag_list[i] = rd16(p + 14 + i * 2);
    st->amp_temp = rdF32(p + 30);
    st->motor_temp = rdF32(p + 34);
    st->dc_volt = rdF32(p + 38);
    st->dc_curr = rdF32(p + 42);
    st->ax_mains = rdF32(p + 46);
    st->ax_ctrl = rd16(p + 50);
    st->ax_park = rd16(p + 52);
    st->ax_fw_rev = rd16(p + 54);
    st->ax_fw_build = rd16(p + 56);
    st->cmd_ctrl = rd16(p + 58);
    std::snprintf(st->diagmsg, sizeof(st->diagmsg), "%s",
                  reinterpret_cast<const char*>(p + 60));
    return TPLC_SOE_OK;
}

int32_t SoeService::writeOp(void*, uint32_t slave, uint16_t op,
                            uint32_t v) {
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_SOE_E_NOEP;
    syncTime();
    auto& c = *impl_->client;
    if (!impl_->selectSlave(c, slave)) return TPLC_SOE_E_IO;
    return impl_->writeCmd(c, op, v) ? TPLC_SOE_OK : TPLC_SOE_E_IO;
}

void SoeService::syncTime() {
    if (!clock_ || !impl_ || !impl_->client || !impl_->idTime) return;
    const uint64_t us = clock_(clock_ctx_) / 1000;   /* ns -> us */
    if (us == last_time_us_) return;
    last_time_us_ = us;
    impl_->client->setParam(impl_->idTime, &us, 8);
}

int32_t SoeService::writePassword(void* s, uint32_t slave, uint32_t pw) {
    return writeOp(s, slave, 1, pw);
}
int32_t SoeService::writeCmdCtrl(void* s, uint32_t slave, uint32_t ctrl) {
    return writeOp(s, slave, 2, ctrl);
}
int32_t SoeService::execCommand(void* s, uint32_t slave, uint32_t id) {
    return writeOp(s, slave, 3, id);
}
int32_t SoeService::reset(void* s, uint32_t slave) {
    return writeOp(s, slave, 4, 0);
}
int32_t SoeService::writeAxCtrl(void* s, uint32_t slave, uint32_t ctrl) {
    return writeOp(s, slave, 5, ctrl);
}
int32_t SoeService::writeAxPark(void* s, uint32_t slave, uint32_t park) {
    return writeOp(s, slave, 6, park);
}

} // namespace tplc
