/* ProfinetService.hpp — tplc-profinet backend: PROFINET DCP
 * discovery/configuration + cyclic IO record exchange over per-port
 * UDP transports.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_PROFINET_SERVICE_HPP
#define TPLC_HOST_PROFINET_SERVICE_HPP

#include <tplc/tplc_profinet.h>

#include <memory>
#include <string>
#include <vector>

namespace tplc {

class ProfinetService {
public:
    ProfinetService();
    ~ProfinetService();
    ProfinetService(const ProfinetService&) = delete;
    ProfinetService& operator=(const ProfinetService&) = delete;

    bool active() const { return active_; }
    const TplcProfinetService& table() const { return svc_; }

private:
    struct Port;
    Port* portAt(uint32_t i);
    static int32_t exchangeDcp(Port* p, const uint8_t* req,
                               size_t reqLen, uint8_t* resp,
                               size_t respCap, uint32_t* respLen);

    int32_t state(void*);
    int32_t open(void*, uint32_t port);
    int32_t close(void*, uint32_t port);
    int32_t identify(void*, uint32_t port, uint32_t xid,
                     char* name, uint32_t name_cap,
                     uint32_t* vendor, uint32_t* device,
                     uint32_t* role, uint32_t* instance,
                     uint32_t* oem,
                     uint8_t* raw, uint32_t raw_cap,
                     uint32_t* raw_len);
    int32_t dcpGet(void*, uint32_t port, uint32_t xid,
                   uint8_t option, uint8_t suboption,
                   uint8_t* data, uint32_t cap, uint32_t* out_len);
    int32_t dcpSet(void*, uint32_t port, uint32_t xid,
                   uint8_t option, uint8_t suboption,
                   const uint8_t* data, uint32_t len);
    int32_t ioExchange(void*, uint32_t port,
                       const uint8_t* out_rec, uint32_t out_len,
                       uint8_t* in_rec, uint32_t in_cap,
                       uint32_t* in_len);
    int32_t ioCr(void*, uint32_t port, uint32_t frame_id,
                 const uint8_t* out_rec, uint32_t out_len,
                 uint8_t* in_rec, uint32_t in_cap, uint32_t* in_len,
                 uint32_t* cyc_out, uint32_t* stat_out);
    int32_t portInfo(void*, uint32_t port, uint32_t* st,
                     uint32_t* errors, uint32_t* frames);

    /* PNIO-CM AR session ops (UDP/34964 CL DCE/RPC) */
    int32_t arEnsure(Port* p);
    int32_t arConnect(void*, uint32_t port,
                      const uint8_t dev[16], const uint8_t act[16],
                      const uint8_t aru[16],
                      const uint8_t* blocks, uint32_t blen,
                      uint8_t* res, uint32_t resCap, uint32_t* resLen,
                      uint32_t* pnioStatus);
    int32_t arExchange(void*, uint32_t port, uint16_t opnum,
                       const uint8_t* blocks, uint32_t blen,
                       uint8_t* res, uint32_t resCap, uint32_t* resLen,
                       uint32_t* pnioStatus);
    int32_t arService(void*, uint32_t port, uint32_t timeoutMs,
                      uint32_t* arState);
    int32_t arState(void*, uint32_t port, uint32_t* arState,
                    uint16_t* sessionKey, uint8_t arUuid[16]);

    TplcProfinetService svc_{};
    std::vector<std::unique_ptr<Port>> ports_;
    bool active_ = false;
};

} // namespace tplc
#endif
