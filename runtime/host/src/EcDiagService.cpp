/* EcDiagService.cpp — tplc-ecdiag: CoE 0x10F3 diag-history reads over
 * the ec.coediag.* params on the Tether IO protocol.  SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#include <tplc/host/EcDiagService.hpp>

#include <cstdio>
#include <cstdlib>
#include <cstring>

#include "tether/io/TetherIOClient.hpp"
#include "tether/io/TcpTransport.hpp"
#include "tether/io/SlipTransport.hpp"

namespace tplc {

using namespace tether::io;

namespace {

EcDiagService* self(void* s) { return static_cast<EcDiagService*>(s); }

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
uint64_t rd64(const uint8_t* p) {
    uint64_t v = 0;
    for (int i = 0; i < 8; ++i) v |= uint64_t(p[i]) << (8 * i);
    return v;
}

} // namespace

struct EcDiagService::Impl {
    std::unique_ptr<TetherIOClient> client;
    uint64_t idSlave = 0, idHistory = 0, idAck = 0;
    bool selectSlave(TetherIOClient& c, uint32_t slave);
};

EcDiagService::EcDiagService() {
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
        if (e.name == "ec.coediag.slave")      impl_->idSlave = e.id;
        else if (e.name == "ec.coediag.history") impl_->idHistory = e.id;
        else if (e.name == "ec.coediag.ack")     impl_->idAck = e.id;
    }
    if (!impl_->idSlave || !impl_->idHistory || !impl_->idAck) {
        impl_.reset();
        return;
    }
    active_ = true;

    svc_.struct_size = sizeof(TplcEcDiagService);
    svc_.abi_version = TPLC_ECDIAG_ABI_VERSION;
    svc_.host = this;
    svc_.diag_read = [](void* s, uint32_t slave, TplcEcDiagEntry* entries,
                        uint32_t max, uint32_t* count,
                        TplcEcDiagInfo* info) {
        return self(s)->read(s, slave, entries, max, count, info);
    };
    svc_.diag_ack = [](void* s, uint32_t slave, uint32_t acked) {
        return self(s)->ack(s, slave, acked);
    };
}

EcDiagService::~EcDiagService() = default;

bool EcDiagService::Impl::selectSlave(TetherIOClient& c, uint32_t slave) {
    uint8_t b[2] = {uint8_t(slave), uint8_t(slave >> 8)};
    auto r = c.setParam(idSlave, b, 2);
    return static_cast<bool>(r);
}

int32_t EcDiagService::read(void*, uint32_t slave, TplcEcDiagEntry* entries,
                            uint32_t max, uint32_t* count,
                            TplcEcDiagInfo* info) {
    if (!entries || !count || !info) return TPLC_ECDIAG_E_PARAM;
    *count = 0;
    std::memset(info, 0, sizeof(*info));
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_ECDIAG_E_NOEP;
    auto& c = *impl_->client;
    if (!impl_->selectSlave(c, slave)) return TPLC_ECDIAG_E_IO;

    auto blob = c.getParam(impl_->idHistory);
    if (!blob || blob->size() < 8) return TPLC_ECDIAG_E_IO;
    const uint8_t* p = blob->data();
    const uint32_t newest = p[1];
    const uint32_t acked = p[2];
    const uint32_t nrec = newest > TPLC_ECDIAG_MAX_ENTRIES
                              ? TPLC_ECDIAG_MAX_ENTRIES
                              : newest;
    if (blob->size() < 8 + size_t(nrec) * 22) return TPLC_ECDIAG_E_IO;

    info->newest = newest;
    info->newest_acked = acked;
    info->new_msgs = p[3];
    info->flags = rd16(p + 4);

    for (uint32_t i = 0; i < nrec && *count < max; ++i, ++*count) {
        const uint8_t* r = p + 8 + size_t(i) * 22;
        TplcEcDiagEntry& e = entries[*count];
        std::memset(&e, 0, sizeof(e));
        e.timestamp_ns = rd64(r + 8);
        e.diag_code = rd32(r + 0);
        const uint32_t rflags = rd16(r + 4);
        const uint32_t sev = rflags & 3u;
        e.msg_type = sev <= 3 ? sev : 0;
        e.text_id = rd16(r + 6);
        e.ack_status = (i + 1) <= acked ? 1u : 0u;
        e.diag_code_type = (rflags & 0x8000u)
                               ? TPLC_ECDIAG_CODETYPE_EMERGENCY
                               : (e.diag_code ? TPLC_ECDIAG_CODETYPE_DEVICE
                                              : TPLC_ECDIAG_CODETYPE_UNKNOWN);
        e.module_no = rd16(r + 16);
        e.channel_no = rd16(r + 18);
        std::snprintf(e.msg_text, sizeof(e.msg_text),
                      "TID=0x%04X CODE=0x%08X", e.text_id, e.diag_code);
    }
    return TPLC_ECDIAG_OK;
}

int32_t EcDiagService::ack(void*, uint32_t slave, uint32_t newest_acked) {
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_ECDIAG_E_NOEP;
    auto& c = *impl_->client;
    if (!impl_->selectSlave(c, slave)) return TPLC_ECDIAG_E_IO;
    uint8_t v = uint8_t(newest_acked > TPLC_ECDIAG_MAX_ENTRIES
                            ? TPLC_ECDIAG_MAX_ENTRIES
                            : newest_acked);
    auto r = c.setParam(impl_->idAck, &v, 1);
    return r ? TPLC_ECDIAG_OK : TPLC_ECDIAG_E_IO;
}

} // namespace tplc
