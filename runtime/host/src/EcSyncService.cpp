/* EcSyncService.cpp — tplc-ecsync: ec.dc.* param reads/writes over
 * the Tether IO protocol.  SPDX-License-Identifier: Apache-2.0
 * OR MPL-2.0 */
#include <tplc/host/EcSyncService.hpp>

#include <cstdlib>
#include <cstring>

#include "tether/io/TetherIOClient.hpp"
#include "tether/io/TcpTransport.hpp"
#include "tether/io/SlipTransport.hpp"

namespace tplc {

using namespace tether::io;

namespace {

EcSyncService* self(void* s) { return static_cast<EcSyncService*>(s); }

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

struct EcSyncService::Impl {
    std::unique_ptr<TetherIOClient> client;
    uint64_t idConnected = 0, idSyncEn = 0, idDiff = 0, idRef = 0;
};

EcSyncService::EcSyncService() {
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
        if (e.name == "ec.dc.connected")      impl_->idConnected = e.id;
        else if (e.name == "ec.dc.sync_enabled") impl_->idSyncEn = e.id;
        else if (e.name == "ec.dc.time_diff_ns") impl_->idDiff = e.id;
        else if (e.name == "ec.dc.ref_time_ns")  impl_->idRef  = e.id;
    }
    if (!impl_->idConnected || !impl_->idSyncEn || !impl_->idDiff
        || !impl_->idRef) {
        impl_.reset();
        return;
    }
    active_ = true;

    svc_.struct_size = sizeof(TplcEcsyncService);
    svc_.abi_version = TPLC_ECSYNC_ABI_VERSION;
    svc_.host = this;
    svc_.ec_status = [](void* s, TplcEcsyncStatus* out) {
        return self(s)->status(s, out);
    };
    svc_.ec_set_sync = [](void* s, uint32_t en) {
        return self(s)->setSync(s, en);
    };
}

EcSyncService::~EcSyncService() = default;

static bool rdU8(TetherIOClient& c, uint64_t id, uint8_t& v) {
    auto r = c.getParam(id);
    if (!r || r->empty()) return false;
    v = (*r)[0];
    return true;
}
static bool rdF64(TetherIOClient& c, uint64_t id, double& v) {
    auto r = c.getParam(id);
    if (!r || r->size() < 8) return false;
    std::memcpy(&v, r->data(), 8);
    return true;
}
static bool rdU64(TetherIOClient& c, uint64_t id, uint64_t& v) {
    auto r = c.getParam(id);
    if (!r || r->size() < 8) return false;
    std::memcpy(&v, r->data(), 8);
    return true;
}

int32_t EcSyncService::status(void*, TplcEcsyncStatus* out) {
    if (!out) return TPLC_ECSYNC_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_ECSYNC_E_NOEP;
    auto& c = *impl_->client;
    uint8_t conn = 0, en = 0;
    bool ok = rdU8(c, impl_->idConnected, conn)
           && rdU8(c, impl_->idSyncEn, en)
           && rdF64(c, impl_->idDiff, out->time_diff_ns)
           && rdU64(c, impl_->idRef, out->ref_time_ns);
    out->connected   = conn;
    out->sync_enabled = en;
    out->params_ok   = ok ? 1 : 0;
    out->last_error  = ok ? 0 : TPLC_ECSYNC_E_IO;
    return ok ? TPLC_ECSYNC_OK : TPLC_ECSYNC_E_IO;
}

int32_t EcSyncService::setSync(void*, uint32_t enable) {
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_ECSYNC_E_NOEP;
    const uint8_t v = enable ? 1 : 0;
    auto r = impl_->client->setParam(impl_->idSyncEn, &v, 1);
    return r ? TPLC_ECSYNC_OK : TPLC_ECSYNC_E_IO;
}

} // namespace tplc
