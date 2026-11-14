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
    int32_t isduReadEx(void*, uint32_t port, uint16_t index,
                       uint8_t subindex, uint8_t* data, uint32_t cap,
                       uint32_t* len, uint32_t* err);
    int32_t isduWriteEx(void*, uint32_t port, uint16_t index,
                        uint8_t subindex, const uint8_t* data,
                        uint32_t len, uint32_t* err);
    int32_t masterCmd(void*, uint32_t port, uint8_t cmd);
    int32_t diagRead(void*, uint32_t port, uint8_t* mem,
                     uint32_t cap, uint32_t* len);
    int32_t alNeg(void*, uint32_t port, uint32_t pdo_req,
                  uint32_t pdi_req, uint32_t* pdo_out,
                  uint32_t* pdi_out);
    int32_t isduService(Port* p, bool wr, uint16_t index,
                        uint8_t subindex, const uint8_t* data,
                        uint32_t dlen, uint8_t* resp, uint32_t rcap,
                        uint32_t* rlen, uint32_t* err);
    static int32_t isduSeg(Port* p, bool wr, uint8_t fc,
                           const uint8_t* od, uint8_t odlen,
                           uint8_t* rod, uint8_t* rolen,
                           uint32_t* cksFlags);
    static void isduAbort(Port* p);
    static int32_t exchange(Port* p, const uint8_t* tx, size_t txLen,
                            uint8_t* rx, size_t rxLen, uint8_t* cksOut,
                            uint32_t* cksFlags);
    static int32_t pageRead(Port* p, uint8_t addr, uint8_t* od);
    static int32_t diagFetch2(Port* p, uint8_t addr, uint8_t* m);
    static void portPdLens(Port* p, uint32_t* pdo, uint32_t* pdi);

    TplcIoLinkService svc_{};
    std::vector<std::unique_ptr<Port>> ports_;
    bool active_ = false;
};

} // namespace tplc
#endif
