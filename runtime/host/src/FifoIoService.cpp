/* FifoIoService.cpp — tplc-fifo bridge over the Tether IO protocol.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/FifoIoService.hpp>

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

struct FifoIoService::Impl {
    std::unique_ptr<TetherIOClient> client;
    uint64_t chan = 0, cmd = 0, state = 0, naxes = 0, maxrows = 0,
             queued = 0, seq = 0, ovr = 0;
    uint64_t integrate = 0, disintegrate = 0, write = 0, overwrite = 0;
    uint64_t timeParamId = 0;
};

FifoIoService::FifoIoService(ClockFn clock, void* clock_ctx)
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
        if (e.name == "mc.time")          impl_->timeParamId = e.id;
        else if (e.name == "fifo.chan")       impl_->chan = e.id;
        else if (e.name == "fifo.cmd")        impl_->cmd = e.id;
        else if (e.name == "fifo.state")      impl_->state = e.id;
        else if (e.name == "fifo.naxes")      impl_->naxes = e.id;
        else if (e.name == "fifo.maxrows")    impl_->maxrows = e.id;
        else if (e.name == "fifo.queued")     impl_->queued = e.id;
        else if (e.name == "fifo.seq")        impl_->seq = e.id;
        else if (e.name == "fifo.override")   impl_->ovr = e.id;
        else if (e.name == "fifo.integrate")  impl_->integrate = e.id;
        else if (e.name == "fifo.disintegrate") impl_->disintegrate = e.id;
        else if (e.name == "fifo.write")      impl_->write = e.id;
        else if (e.name == "fifo.overwrite")  impl_->overwrite = e.id;
    }
    /* The FIFO surface is present only when the server exposes it. */
    if (!impl_->chan || !impl_->cmd || !impl_->state ||
        !impl_->integrate || !impl_->write) {
        impl_.reset();
        return;
    }

    svc_.struct_size = sizeof(TplcFifoService);
    svc_.abi_version = TPLC_FIFO_ABI_VERSION;
    svc_.host = this;
    svc_.fifo_integrate = [](void* s, uint32_t c, uint32_t a, uint32_t g) {
        return self(s)->integrate(s, c, a, g);
    };
    svc_.fifo_disintegrate = [](void* s, uint32_t c) {
        return self(s)->disintegrate(s, c);
    };
    svc_.fifo_write = [](void* s, uint32_t c, const void* b, uint32_t l,
                         int32_t r) {
        return self(s)->write(s, c, b, l, r);
    };
    svc_.fifo_command = [](void* s, uint32_t c, uint32_t cmd) {
        return self(s)->command(s, c, cmd);
    };
    svc_.fifo_status = [](void* s, uint32_t c, TplcFifoStatus* o) {
        return self(s)->status(s, c, o);
    };
    svc_.fifo_set_override = [](void* s, uint32_t c, double p) {
        return self(s)->setOverride(s, c, p);
    };
    svc_.fifo_poll = [](void* s, uint32_t) {
        return self(s)->poll(s, 0);
    };
    active_ = true;
}

FifoIoService::~FifoIoService() = default;

void FifoIoService::syncTime() {
    if (!clock_ || !impl_ || !impl_->timeParamId) return;
    const uint64_t us = clock_(clock_ctx_) / 1000;
    if (us == last_time_us_) return;
    last_time_us_ = us;
    impl_->client->setParam(impl_->timeParamId, &us, 8);
}

bool FifoIoService::selectChan(uint32_t chan) {
    if (!impl_ || chan >= TPLC_FIFO_CHAN_MAX) return false;
    const uint8_t v = uint8_t(chan);
    return impl_->client->setParam(impl_->chan, &v, 1).has_value();
}

int32_t FifoIoService::integrate(void*, uint32_t chan, uint32_t axis,
                                 uint32_t gpos) {
    if (!impl_ || chan >= TPLC_FIFO_CHAN_MAX || gpos >= TPLC_FIFO_AXIS_MAX ||
        axis == 0)
        return TPLC_FIFO_E_PARAM;
    syncTime();
    if (!selectChan(chan)) return TPLC_FIFO_E_IO;
    /* iAxisId is the AXIS_REF handle; on Tether IO the handle is
       slave + 1, so the wire value is the 0-based slave index. */
    const uint32_t v = (gpos << 16) | ((axis - 1) & 0xFFFF);
    auto r = impl_->client->setParam(impl_->integrate, &v, 4);
    return r ? TPLC_FIFO_OK : TPLC_FIFO_E_IO;
}

int32_t FifoIoService::disintegrate(void*, uint32_t chan) {
    if (!impl_ || chan >= TPLC_FIFO_CHAN_MAX) return TPLC_FIFO_E_PARAM;
    syncTime();
    if (!selectChan(chan)) return TPLC_FIFO_E_IO;
    const uint8_t one = 1;
    auto r = impl_->client->setParam(impl_->disintegrate, &one, 1);
    return r ? TPLC_FIFO_OK : TPLC_FIFO_E_IO;
}

int32_t FifoIoService::write(void*, uint32_t chan, const void* blob,
                             uint32_t len, int32_t replace) {
    if (!impl_ || !blob || chan >= TPLC_FIFO_CHAN_MAX)
        return TPLC_FIFO_E_PARAM;
    syncTime();
    if (!selectChan(chan)) return TPLC_FIFO_E_IO;
    const uint64_t id = replace ? impl_->overwrite : impl_->write;
    auto r = impl_->client->setParamVar(id, blob, len);
    return r ? TPLC_FIFO_OK : TPLC_FIFO_E_IO;
}

int32_t FifoIoService::command(void*, uint32_t chan, uint32_t cmd) {
    if (!impl_ || chan >= TPLC_FIFO_CHAN_MAX) return TPLC_FIFO_E_PARAM;
    syncTime();
    if (!selectChan(chan)) return TPLC_FIFO_E_IO;
    const uint8_t v = uint8_t(cmd);
    auto r = impl_->client->setParam(impl_->cmd, &v, 1);
    return r ? TPLC_FIFO_OK : TPLC_FIFO_E_IO;
}

int32_t FifoIoService::status(void*, uint32_t chan,
                              TplcFifoStatus* out) {
    if (!impl_ || !out || chan >= TPLC_FIFO_CHAN_MAX)
        return TPLC_FIFO_E_PARAM;
    syncTime();
    if (!selectChan(chan)) return TPLC_FIFO_E_IO;
    auto rd = [&](uint64_t id, void* d, size_t n) -> bool {
        if (!id) return false;
        auto v = impl_->client->getParam(id);
        if (!v || v->size() < n) return false;
        std::memcpy(d, v->data(), n);
        return true;
    };
    uint8_t st = 0, na = 0;
    uint32_t maxr = 0, q = 0, sq = 0;
    double ovr = 0.0;
    if (!rd(impl_->state, &st, 1) || !rd(impl_->naxes, &na, 1) ||
        !rd(impl_->maxrows, &maxr, 4) || !rd(impl_->queued, &q, 4) ||
        !rd(impl_->seq, &sq, 4))
        return TPLC_FIFO_E_IO;
    rd(impl_->ovr, &ovr, 8);
    out->state = st;
    out->naxes = na;
    out->maxrows = maxr;
    out->queued = q;
    out->seq = sq;
    out->override = ovr;
    return TPLC_FIFO_OK;
}

int32_t FifoIoService::setOverride(void*, uint32_t chan, double pct) {
    if (!impl_ || chan >= TPLC_FIFO_CHAN_MAX) return TPLC_FIFO_E_PARAM;
    syncTime();
    if (!selectChan(chan)) return TPLC_FIFO_E_IO;
    auto r = impl_->client->setParam(impl_->ovr, &pct, 8);
    return r ? TPLC_FIFO_OK : TPLC_FIFO_E_IO;
}

int32_t FifoIoService::poll(void*, uint32_t) {
    /* The server consumes rows on the mc.time drive clock; syncTime
       in the other ops already advances it. */
    return TPLC_FIFO_OK;
}

} // namespace tplc

#else /* !TPLC_HAVE_TETHER_IO */

namespace tplc {

struct FifoIoService::Impl {};

FifoIoService::FifoIoService() = default;
FifoIoService::~FifoIoService() = default;

int32_t FifoIoService::integrate(void*, uint32_t, uint32_t, uint32_t) {
    return TPLC_FIFO_E_STATE;
}
int32_t FifoIoService::disintegrate(void*, uint32_t) {
    return TPLC_FIFO_E_STATE;
}
int32_t FifoIoService::write(void*, uint32_t, const void*, uint32_t,
                             int32_t) {
    return TPLC_FIFO_E_STATE;
}
int32_t FifoIoService::command(void*, uint32_t, uint32_t) {
    return TPLC_FIFO_E_STATE;
}
int32_t FifoIoService::status(void*, uint32_t, TplcFifoStatus*) {
    return TPLC_FIFO_E_STATE;
}
int32_t FifoIoService::setOverride(void*, uint32_t, double) {
    return TPLC_FIFO_E_STATE;
}
int32_t FifoIoService::poll(void*, uint32_t) { return TPLC_FIFO_E_STATE; }
void FifoIoService::syncTime() {}
bool FifoIoService::selectChan(uint32_t) { return false; }

} // namespace tplc

#endif /* TPLC_HAVE_TETHER_IO */
