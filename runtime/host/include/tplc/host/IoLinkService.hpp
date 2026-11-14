/* IoLinkService.hpp — tplc-iolink backend: IO-Link master data-link
 * protocol (IEC 61131-9) over per-port TCP byte transports.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_IOLINK_SERVICE_HPP
#define TPLC_HOST_IOLINK_SERVICE_HPP

#include <tplc/tplc_iolink.h>

#include <memory>
#include <string>
#include <cstddef>
#include <vector>

namespace tplc {

class IoLinkService {
public:
    IoLinkService();
    ~IoLinkService();
    IoLinkService(const IoLinkService&) = delete;
    IoLinkService& operator=(const IoLinkService&) = delete;

    bool active() const { return active_; }
    const TplcIoLinkService& table() const { return svc_; }

private:
    struct Port;
    Port* portAt(uint32_t i);

    int32_t state(void*);
    int32_t open(void*, uint32_t port);
    int32_t close(void*, uint32_t port);
    int32_t ident(void*, uint32_t port, uint8_t* out, uint32_t cap,
                  uint32_t* out_len);
    int32_t pdExchange(void*, uint32_t port, uint32_t mseq,
                       const uint8_t* pdo, uint32_t pdo_len,
                       uint8_t* pdi, uint32_t pdi_cap,
                       uint32_t* pdi_len, uint32_t* cks_flags);
    int32_t isduRead(void*, uint32_t port, uint8_t addr,
                     const uint8_t* od, uint32_t od_len,
                     uint8_t* resp, uint32_t resp_cap,
                     uint32_t* resp_len);
    int32_t isduWrite(void*, uint32_t port, uint8_t addr,
                      const uint8_t* od, uint32_t od_len);
    int32_t portInfo(void*, uint32_t port, uint32_t* st,
                     uint32_t* errors, uint32_t* frames);
    static int32_t exchange(Port* p, const uint8_t* tx, size_t txLen,
                            uint8_t* rx, size_t rxLen, uint8_t* cksOut,
                            uint32_t* cksFlags);
    static int32_t pageRead(Port* p, uint8_t addr, uint8_t* od);

    TplcIoLinkService svc_{};
    std::vector<std::unique_ptr<Port>> ports_;
    bool active_ = false;
};

} // namespace tplc
#endif
