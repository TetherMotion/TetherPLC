/* CanopenService.cpp — tplc-canopen: CiA 301 SDO over tether_io
 * machine.sdo.* functions.  The tagged-struct request/reply codec is
 * the same detail::encodeTagged/parseTagged format MachineSdoService
 * uses: varint field-count, then [key varint][len varint][bytes].
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/CanopenService.hpp>

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <string>
#include <vector>

#include "tether/io/TetherIOClient.hpp"
#include "tether/io/TcpTransport.hpp"
#include "tether/io/SlipTransport.hpp"

namespace tplc {

using namespace tether::io;

namespace {

CanopenService* self(void* s) { return static_cast<CanopenService*>(s); }

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

void putVarint(std::vector<uint8_t>& b, uint64_t v) {
    while (v >= 0x80) { b.push_back(uint8_t(v) | 0x80); v >>= 7; }
    b.push_back(uint8_t(v));
}
bool getVarint(const std::vector<uint8_t>& b, size_t& pos, uint64_t& v) {
    v = 0;
    for (unsigned i = 0; i < 10; ++i) {
        if (pos >= b.size()) return false;
        uint8_t c = b[pos++];
        v |= uint64_t(c & 0x7F) << (i * 7);
        if (!(c & 0x80)) return true;
    }
    return false;
}
void putScalar(std::vector<uint8_t>& b, uint64_t v, unsigned n) {
    for (unsigned i = 0; i < n; ++i) b.push_back(uint8_t(v >> (8 * i)));
}
std::vector<uint8_t> fieldBytes(const std::vector<uint8_t>& v) {
    std::vector<uint8_t> b;
    putVarint(b, v.size());
    b.insert(b.end(), v.begin(), v.end());
    return b;
}
using Fields = std::vector<std::pair<uint32_t, std::vector<uint8_t>>>;
std::vector<uint8_t> encodeTagged(const Fields& fields) {
    std::vector<uint8_t> b;
    putVarint(b, fields.size());
    for (auto& [k, v] : fields) {
        putVarint(b, k);
        putVarint(b, v.size());
        b.insert(b.end(), v.begin(), v.end());
    }
    return b;
}
bool parseTagged(const std::vector<uint8_t>& b,
                 std::map<uint32_t, std::vector<uint8_t>>& out) {
    size_t pos = 0;
    uint64_t count = 0;
    if (!getVarint(b, pos, count) || count > 1024) return false;
    for (uint64_t i = 0; i < count; ++i) {
        uint64_t k = 0, len = 0;
        if (!getVarint(b, pos, k) || !getVarint(b, pos, len) ||
            len > b.size() - pos) return false;
        out[uint32_t(k)] = std::vector<uint8_t>(
            b.begin() + pos, b.begin() + pos + len);
        pos += size_t(len);
    }
    return pos == b.size();
}
uint64_t scalarOf(const std::vector<uint8_t>& v) {
    uint64_t r = 0;
    for (size_t i = 0; i < v.size() && i < 8; ++i)
        r |= uint64_t(v[i]) << (8 * i);
    return r;
}
/* unwrap a fieldBytes payload (varint length + bytes) */
bool blobOf(const std::vector<uint8_t>& v, std::vector<uint8_t>& out) {
    size_t pos = 0;
    uint64_t len = 0;
    if (!getVarint(v, pos, len) || len != v.size() - pos) return false;
    out.assign(v.begin() + pos, v.end());
    return true;
}
std::string cstrOf(const std::vector<uint8_t>& v) {
    /* fieldString appends a NUL terminator — strip trailing NULs */
    size_t n = v.size();
    while (n && v[n - 1] == 0) --n;
    return std::string(v.begin(), v.begin() + n);
}

struct SdoIds { uint64_t read = 0, write = 0; };

} // namespace

struct CanopenService::Impl {
    std::unique_ptr<TetherIOClient> client;
    SdoIds ids;
};

CanopenService::CanopenService() {
    const char* ep = std::getenv("TPLC_CANOPEN_IO");
    if (!ep || !*ep) ep = std::getenv("TPLC_EC_IO");
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

    auto fns = impl_->client->listFunctions(0, 4096);
    if (!fns) { impl_.reset(); return; }
    for (const auto& e : *fns) {
        if (e.name == "machine.sdo.read" ||
            e.name == "machine.cia402.machine.sdo.read")
            impl_->ids.read = e.id;
        else if (e.name == "machine.sdo.write" ||
                 e.name == "machine.cia402.machine.sdo.write")
            impl_->ids.write = e.id;
    }
    if (!impl_->ids.read || !impl_->ids.write) { impl_.reset(); return; }
    active_ = true;

    svc_.struct_size = sizeof(TplcCanopenService);
    svc_.abi_version = TPLC_CANOPEN_ABI_VERSION;
    svc_.host = this;
    svc_.co_state = [](void* s) { return self(s)->state(s); };
    svc_.co_sdo_read = [](void* s, uint16_t sl, uint16_t ix,
                          uint8_t sb, uint8_t* o, uint32_t cap,
                          uint32_t* ol, uint32_t* ac) {
        return self(s)->sdoRead(s, sl, ix, sb, o, cap, ol, ac);
    };
    svc_.co_sdo_write = [](void* s, uint16_t sl, uint16_t ix,
                           uint8_t sb, const uint8_t* d, uint32_t l,
                           uint8_t v, uint32_t* ac) {
        return self(s)->sdoWrite(s, sl, ix, sb, d, l, v, ac);
    };
}

CanopenService::~CanopenService() = default;

int32_t CanopenService::state(void*) {
    return active_ ? TPLC_CO_OK : TPLC_CO_E_NOEP;
}

namespace {
/* shared result decode: tagged SdoResultV1 → rc + abort + payload */
int32_t decodeResult(const std::vector<uint8_t>& ret,
                     std::vector<uint8_t>& data, uint32_t& abort,
                     std::string& err) {
    std::map<uint32_t, std::vector<uint8_t>> f;
    if (!parseTagged(ret, f)) return TPLC_CO_E_PROTO;
    const bool ok = f.count(1) && !f[1].empty() && f[1][0] != 0;
    abort = f.count(2) ? uint32_t(scalarOf(f[2])) : 0;
    if (f.count(4)) err = cstrOf(f[4]);
    if (!ok) {
        /* a CoE abort code is the more specific failure — the slave
           answered and refused the transfer */
        if (abort) return TPLC_CO_E_ABORT;
        return TPLC_CO_E_IO;
    }
    if (f.count(3)) blobOf(f[3], data);
    return TPLC_CO_OK;
}
} // namespace

int32_t CanopenService::sdoRead(void*, uint16_t slave, uint16_t index,
                                uint8_t subindex, uint8_t* out,
                                uint32_t cap, uint32_t* out_len,
                                uint32_t* abort_code) {
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_CO_E_NOEP;
    if (out_len) *out_len = 0;
    if (abort_code) *abort_code = 0;
    if (!out || cap == 0 || cap > TPLC_CANOPEN_MAX_DATA)
        return TPLC_CO_E_PARAM;

    std::vector<uint8_t> req;
    {
        std::vector<uint8_t> s, i, sb, mb;
        putScalar(s, slave, 2);
        putScalar(i, index, 2);
        sb.push_back(subindex);
        putScalar(mb, cap, 2);
        req = encodeTagged({{1, s}, {2, i}, {3, sb}, {4, mb}});
    }
    TetherIOClient::FunctionArg arg;
    arg.position = 0;
    arg.key = 1;
    arg.type = ValueType::Struct;
    arg.value = req;
    auto r = impl_->client->callFunction(impl_->ids.read, {arg});
    if (!r) return TPLC_CO_E_IO;
    if (!r->success) return TPLC_CO_E_IO;

    std::vector<uint8_t> data;
    std::string err;
    uint32_t abort = 0;
    int32_t rc = decodeResult(r->returnValue, data, abort, err);
    if (abort_code) *abort_code = abort;
    if (rc != TPLC_CO_OK) return rc;
    uint32_t n = uint32_t(data.size()) < cap ? uint32_t(data.size()) : cap;
    if (n) memcpy(out, data.data(), n);
    if (out_len) *out_len = n;
    return TPLC_CO_OK;
}

int32_t CanopenService::sdoWrite(void*, uint16_t slave, uint16_t index,
                                 uint8_t subindex, const uint8_t* data,
                                 uint32_t len, uint8_t verify,
                                 uint32_t* abort_code) {
    if (!active_ || !impl_ || !impl_->client)
        return TPLC_CO_E_NOEP;
    if (abort_code) *abort_code = 0;
    if (!data || len == 0 || len > TPLC_CANOPEN_MAX_DATA)
        return TPLC_CO_E_PARAM;

    std::vector<uint8_t> payload(data, data + len);
    std::vector<uint8_t> req;
    {
        std::vector<uint8_t> s, i, sb, vb;
        putScalar(s, slave, 2);
        putScalar(i, index, 2);
        sb.push_back(subindex);
        vb = fieldBytes(payload);
        std::vector<uint8_t> vf{verify ? 1 : 0};
        req = encodeTagged({{1, s}, {2, i}, {3, sb}, {4, vb}, {5, vf}});
    }
    TetherIOClient::FunctionArg arg;
    arg.position = 0;
    arg.key = 1;
    arg.type = ValueType::Struct;
    arg.value = req;
    auto r = impl_->client->callFunction(impl_->ids.write, {arg});
    if (!r) return TPLC_CO_E_IO;
    if (!r->success) return TPLC_CO_E_IO;

    std::vector<uint8_t> dummy;
    std::string err;
    uint32_t abort = 0;
    int32_t rc = decodeResult(r->returnValue, dummy, abort, err);
    if (abort_code) *abort_code = abort;
    return rc;
}

} // namespace tplc
