/* CanopenService.hpp — tplc-canopen backend over the Tether IO
 * protocol.  Wraps machine.sdo.read/write (CiA 301 object-dictionary
 * access over EtherCAT CoE) on a tether_io server such as
 * tether_io_ec_server.  Endpoint: TPLC_CANOPEN_IO, else TPLC_EC_IO,
 * else TPLC_MOTION_IO.  SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_CANOPEN_SERVICE_HPP
#define TPLC_HOST_CANOPEN_SERVICE_HPP

#include <tplc/tplc_canopen.h>

#include <memory>

namespace tplc {

class CanopenService {
public:
    CanopenService();
    ~CanopenService();
    CanopenService(const CanopenService&) = delete;
    CanopenService& operator=(const CanopenService&) = delete;

    bool active() const { return active_; }
    const TplcCanopenService& table() const { return svc_; }

private:
    int32_t state(void*);
    int32_t sdoRead(void*, uint16_t slave, uint16_t index,
                    uint8_t subindex, uint8_t* out, uint32_t cap,
                    uint32_t* out_len, uint32_t* abort_code);
    int32_t sdoWrite(void*, uint16_t slave, uint16_t index,
                     uint8_t subindex, const uint8_t* data,
                     uint32_t len, uint8_t verify,
                     uint32_t* abort_code);

    TplcCanopenService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    bool active_ = false;
};

} // namespace tplc
#endif
