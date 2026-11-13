/* S5Service.hpp — Siemens S5 client service (RK512/3964 TCP).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_S5_SERVICE_HPP
#define TPLC_HOST_S5_SERVICE_HPP

#include <tplc/tplc_s5.h>

#include <cstdint>
#include <unordered_map>

namespace tplc {

class S5Service {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit S5Service(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcS5Service& table() const { return svc_; }

private:
    enum class Stage : uint8_t {
        TcpConnect,   /* waiting for connect()                */
        Ready,
        Failed,
    };

    struct Conn {
        int fd = -1;
        Stage stage = Stage::TcpConnect;
        TplcS5OpenReq req{};
        uint64_t deadline_ns = 0;
        int err = 0;
        /* outbound queue (one 3964 frame at a time) */
        uint8_t tx[1024]{};
        size_t tx_len = 0;
        size_t tx_off = 0;
        /* inbound 3964 frame assembly */
        uint8_t rx[1088]{};
        size_t rx_len = 0;
        TplcS5ConnHandle self = 0;
        uint32_t active_io = 0;   /* io handle or 0                */
    };

    struct Io {
        TplcS5ConnHandle conn = 0;
        bool is_write = false;
        TplcS5Addr addr{};
        uint8_t data[TPLC_S5_MAX_DATA]{};
        uint32_t dlen = 0;
        uint64_t deadline_ns = 0;
        bool finished = false;
        TplcS5RespInfo info{};
        uint8_t val[TPLC_S5_MAX_DATA]{};
        uint32_t val_len = 0;
    };

    static S5Service* self(void* p);

    int32_t open(void*, const TplcS5OpenReq*, TplcS5ConnHandle*);
    int32_t connPoll(void*, TplcS5ConnHandle, TplcS5ConnInfo*);
    int32_t read(void*, TplcS5ConnHandle, const TplcS5ReadReq*,
                 TplcS5IoHandle*);
    int32_t write(void*, TplcS5ConnHandle, const TplcS5WriteReq*,
                  TplcS5IoHandle*);
    int32_t poll(void*, TplcS5IoHandle, TplcS5RespInfo*,
                 uint8_t*, uint32_t);
    int32_t abort(void*, TplcS5IoHandle);
    int32_t connClose(void*, TplcS5ConnHandle);

    void failConn(Conn& c, int err);
    bool pumpTx(Conn& c);
    /* recv + 3964 frame extraction; returns the RK512 payload
       (between STX and BCC) in `frame`, 0 incomplete, -1
       malformed/overflow, -2 peer closed. */
    int pumpRx(Conn& c, uint8_t* frame, size_t cap, size_t* flen);
    void handleIoResp(Io& io, Conn& c, const uint8_t* frame,
                      size_t n);
    bool sendJob(Conn& c, bool is_write, const TplcS5Addr& a,
                 const uint8_t* data, uint32_t dlen);
    void dropIo(Io& io);

    /* svc_ must stay the first data member: the service-table
       pointer handed to the vtable lambdas is &svc_, and self()
       recovers the S5Service* by treating it as `this`. */
    TplcS5Service svc_{};
    ClockFn clock_;
    void* clock_ctx_;
    std::unordered_map<uint32_t, Conn> conns_;
    std::unordered_map<uint32_t, Io> ios_;
    uint32_t next_conn_ = 1;
    uint32_t next_io_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_S5_SERVICE_HPP */
