/* NciIoService.cpp — tplc-nci bridge over the Tether IO protocol.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/NciIoService.hpp>

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

} // namespace

struct NciIoService::Impl {
    std::unique_ptr<TetherIOClient> client;
    uint64_t prog = 0, progPath = 0, cmd = 0, state = 0, line = 0,
             total = 0, errcode = 0, progress = 0, duration = 0;
    uint64_t timeParamId = 0;
};

NciIoService::NciIoService(ClockFn clock, void* clock_ctx)
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
        if (e.name == "mc.time")        impl_->timeParamId = e.id;
        else if (e.name == "nc.prog")       impl_->prog = e.id;
        else if (e.name == "nc.prog_path")  impl_->progPath = e.id;
        else if (e.name == "nc.cmd")        impl_->cmd = e.id;
        else if (e.name == "nc.state")      impl_->state = e.id;
        else if (e.name == "nc.line")       impl_->line = e.id;
        else if (e.name == "nc.total")      impl_->total = e.id;
        else if (e.name == "nc.errcode")    impl_->errcode = e.id;
        else if (e.name == "nc.progress")   impl_->progress = e.id;
        else if (e.name == "nc.duration")   impl_->duration = e.id;
    }
    /* The channel is present only when the server exposes nc.* */
    if (!impl_->prog || !impl_->cmd || !impl_->state ||
        !impl_->line || !impl_->total || !impl_->errcode) {
        impl_.reset();
        return;
    }

    svc_.struct_size = sizeof(TplcNciService);
    svc_.abi_version = TPLC_NCI_ABI_VERSION;
    svc_.host = this;
    svc_.nci_load_text = [](void* s, uint32_t ch, const char* t, uint32_t l) {
        return self(s)->loadText(s, ch, t, l);
    };
    svc_.nci_load_file = [](void* s, uint32_t ch, const char* p, uint32_t l) {
        return self(s)->loadFile(s, ch, p, l);
    };
    svc_.nci_command = [](void* s, uint32_t ch, uint32_t c) {
        return self(s)->command(s, ch, c);
    };
    svc_.nci_status = [](void* s, uint32_t ch, TplcNciStatus* o) {
        return self(s)->status(s, ch, o);
    };
    active_ = true;
}

NciIoService::~NciIoService() = default;

void NciIoService::syncTime() {
    if (!clock_ || !impl_ || !impl_->timeParamId) return;
    const uint64_t us = clock_(clock_ctx_) / 1000;
    if (us == last_time_us_) return;
    last_time_us_ = us;
    impl_->client->setParam(impl_->timeParamId, &us, 8);
}

int32_t NciIoService::loadText(void*, uint32_t chan, const char* text,
                               uint32_t len) {
    if (!impl_ || !text || chan >= TPLC_NCI_CHAN_MAX)
        return TPLC_NCI_E_PARAM;
    syncTime();
    auto r = impl_->client->setParamVar(impl_->prog, text, len);
    return r ? TPLC_NCI_OK : TPLC_NCI_E_IO;
}

int32_t NciIoService::loadFile(void*, uint32_t chan, const char* path,
                               uint32_t len) {
    if (!impl_ || !path || chan >= TPLC_NCI_CHAN_MAX)
        return TPLC_NCI_E_PARAM;
    syncTime();
    auto r = impl_->client->setParamVar(impl_->progPath, path, len);
    return r ? TPLC_NCI_OK : TPLC_NCI_E_IO;
}

int32_t NciIoService::command(void*, uint32_t chan, uint32_t cmd) {
    if (!impl_ || chan >= TPLC_NCI_CHAN_MAX) return TPLC_NCI_E_PARAM;
    syncTime();
    const uint8_t v = uint8_t(cmd);
    auto r = impl_->client->setParam(impl_->cmd, &v, 1);
    return r ? TPLC_NCI_OK : TPLC_NCI_E_IO;
}

int32_t NciIoService::status(void*, uint32_t chan, TplcNciStatus* out) {
    if (!impl_ || !out || chan >= TPLC_NCI_CHAN_MAX)
        return TPLC_NCI_E_PARAM;
    syncTime();
    auto rd = [&](uint64_t id, void* d, size_t n) -> bool {
        if (!id) return false;
        auto v = impl_->client->getParam(id);
        if (!v || v->size() < n) return false;
        std::memcpy(d, v->data(), n);
        return true;
    };
    uint8_t st = 0;
    uint32_t line = 0, total = 0, err = 0;
    double prog = 0.0, dur = 0.0;
    if (!rd(impl_->state, &st, 1) || !rd(impl_->line, &line, 4) ||
        !rd(impl_->total, &total, 4) || !rd(impl_->errcode, &err, 4))
        return TPLC_NCI_E_IO;
    rd(impl_->progress, &prog, 8);
    rd(impl_->duration, &dur, 8);
    out->state = st;
    out->finished = (st == TPLC_NCI_STATE_FINISHED ||
                     st == TPLC_NCI_STATE_ERROR ||
                     st == TPLC_NCI_STATE_STOPPED) ? 1 : 0;
    out->errcode = int32_t(err);
    out->line = line;
    out->total = total;
    out->progress_ms = int32_t(prog * 1000.0);
    out->duration_ms = int32_t(dur * 1000.0);
    return TPLC_NCI_OK;
}

} // namespace tplc

#else /* !TPLC_HAVE_TETHER_IO */

namespace tplc {

struct NciIoService::Impl {};

NciIoService::NciIoService() = default;
NciIoService::~NciIoService() = default;

int32_t NciIoService::loadText(void*, uint32_t, const char*, uint32_t) {
    return TPLC_NCI_E_STATE;
}
int32_t NciIoService::loadFile(void*, uint32_t, const char*, uint32_t) {
    return TPLC_NCI_E_STATE;
}
int32_t NciIoService::command(void*, uint32_t, uint32_t) {
    return TPLC_NCI_E_STATE;
}
int32_t NciIoService::status(void*, uint32_t, TplcNciStatus*) {
    return TPLC_NCI_E_STATE;
}
void NciIoService::syncTime() {}

} // namespace tplc

#endif /* TPLC_HAVE_TETHER_IO */
