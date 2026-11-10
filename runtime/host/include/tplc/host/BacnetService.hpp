/* BacnetService.hpp — BACnet/IP (ASHRAE 135) client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_BACNET_SERVICE_HPP
#define TPLC_HOST_BACNET_SERVICE_HPP

#include <tplc/tplc_bacnet.h>

#include <netinet/in.h>

#include <cstdint>
#include <unordered_map>

namespace tplc {

class BacnetService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit BacnetService(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcBacnetService& table() const { return svc_; }

private:
    struct Txn {
        int fd = -1;
        TplcBacnetReq req{};
        uint8_t pkt[2048]{};
        uint32_t pkt_len = 0;
        uint32_t invoke_id = 0;
        uint64_t deadline_ns = 0;
        bool sent = false;
        bool finished = false;
        TplcBacnetResp resp{};
    };

    static BacnetService* self(void* p);

    int32_t begin(void*, const TplcBacnetReq*, TplcBacnetHandle*);
    int32_t poll(void*, TplcBacnetHandle, TplcBacnetResp*);
    int32_t abort(void*, TplcBacnetHandle);

    bool encode(Txn& t);
    void handleDatagram(Txn& t, const uint8_t* d, size_t n,
                        const struct sockaddr_in& from);
    void drain(Txn& t);
    void finish(Txn& t, int32_t status);

    ClockFn clock_;
    void* clock_ctx_;
    TplcBacnetService svc_{};
    std::unordered_map<uint32_t, Txn> txns_;
    uint32_t next_handle_ = 1;
    uint32_t next_invoke_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_BACNET_SERVICE_HPP */
