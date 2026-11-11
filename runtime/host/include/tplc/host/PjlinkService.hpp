/* PjlinkService.hpp — PJLink (JBMIA) projector client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_PJLINK_SERVICE_HPP
#define TPLC_HOST_PJLINK_SERVICE_HPP

#include <tplc/tplc_pjlink.h>

#include <netinet/in.h>

#include <cstdint>
#include <unordered_map>

namespace tplc {

class PjlinkService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit PjlinkService(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcPjlinkService& table() const { return svc_; }

private:
    enum class Stage : uint8_t { Connect, Greet, Send, Recv };

    struct Txn {
        int fd = -1;
        Stage stage = Stage::Connect;
        TplcPjlinkReq req{};
        uint64_t deadline_ns = 0;
        uint64_t greet_until_ns = 0;   /* give up waiting for the
                                          PJLINK greeting by then */
        /* outbound: optional auth digest + command line */
        char out[384]{};
        size_t out_len = 0;
        size_t out_off = 0;
        bool authed = false;
        /* inbound line assembly */
        char in[320]{};
        size_t in_len = 0;
        bool finished = false;
        TplcPjlinkRespInfo info{};
        char value[TPLC_PJLINK_MAX_VALUE + 1]{};
        uint32_t value_total = 0;
    };

    static PjlinkService* self(void* p);

    int32_t begin(void*, const TplcPjlinkReq*, TplcPjlinkHandle*);
    int32_t poll(void*, TplcPjlinkHandle, TplcPjlinkRespInfo*,
                 char*, uint32_t);
    int32_t abort(void*, TplcPjlinkHandle);

    void handleLine(Txn& t);
    void finish(Txn& t, int32_t status, int32_t err = 0);

    /* svc_ must stay the first data member: the service-table
       pointer handed to the vtable lambdas is &svc_, and self()
       recovers the PjlinkService* by treating it as `this`. */
    TplcPjlinkService svc_{};
    ClockFn clock_;
    void* clock_ctx_;
    std::unordered_map<uint32_t, Txn> txns_;
    uint32_t next_handle_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_PJLINK_SERVICE_HPP */
