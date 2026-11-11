/* S7Service.hpp — Siemens S7comm (RFC1006) client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_S7_SERVICE_HPP
#define TPLC_HOST_S7_SERVICE_HPP

#include <tplc/tplc_s7.h>

#include <netinet/in.h>

#include <cstdint>
#include <unordered_map>

namespace tplc {

class S7Service {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit S7Service(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcS7Service& table() const { return svc_; }

private:
    enum class Stage : uint8_t {
        TcpConnect,   /* waiting for connect()                */
        CotpWaitCc,   /* CR sent, waiting for COTP CC          */
        SetupWait,    /* setup job sent, waiting for ACK_DATA  */
        Ready,
        Failed,
    };

    struct Conn {
        int fd = -1;
        Stage stage = Stage::TcpConnect;
        TplcS7OpenReq req{};
        uint64_t deadline_ns = 0;
        int err = 0;
        /* outbound queue (CR / setup job / io job) */
        uint8_t tx[640]{};
        size_t tx_len = 0;
        size_t tx_off = 0;
        /* inbound TPKT frame assembly */
        uint8_t rx[2112]{};
        size_t rx_len = 0;
        uint16_t pdu_ref = 0;
        TplcS7ConnHandle self = 0;
        uint32_t active_io = 0;   /* io handle or 0                */
    };

    struct Io {
        TplcS7ConnHandle conn = 0;
        bool is_write = false;
        TplcS7Addr addr{};
        uint8_t data[TPLC_S7_MAX_DATA]{};
        uint32_t dlen = 0;
        uint64_t deadline_ns = 0;
        uint16_t pdu_ref = 0;
        bool finished = false;
        TplcS7RespInfo info{};
        uint8_t val[TPLC_S7_MAX_DATA]{};
        uint32_t val_len = 0;
    };

    static S7Service* self(void* p);

    int32_t open(void*, const TplcS7OpenReq*, TplcS7ConnHandle*);
    int32_t connPoll(void*, TplcS7ConnHandle, TplcS7ConnInfo*);
    int32_t read(void*, TplcS7ConnHandle, const TplcS7ReadReq*,
                 TplcS7IoHandle*);
    int32_t write(void*, TplcS7ConnHandle, const TplcS7WriteReq*,
                  TplcS7IoHandle*);
    int32_t poll(void*, TplcS7IoHandle, TplcS7RespInfo*,
                 uint8_t*, uint32_t);
    int32_t abort(void*, TplcS7IoHandle);
    int32_t connClose(void*, TplcS7ConnHandle);

    void failConn(Conn& c, int err);
    bool pumpTx(Conn& c);
    /* recv + frame extraction; returns complete TPKT frame payload
       (COTP part) in `frame` or 0 when none is ready / -1 on
       overflow. Consumes the frame from rx. */
    int pumpRx(Conn& c, uint8_t* frame, size_t cap, size_t* flen);
    void handleHandshake(Conn& c, const uint8_t* frame, size_t n);
    void handleIoResp(Io& io, Conn& c, const uint8_t* frame,
                      size_t n);
    bool sendJob(Conn& c, bool is_write, const TplcS7Addr& a,
                 const uint8_t* data, uint32_t dlen);
    void dropIo(Io& io);

    /* svc_ must stay the first data member: the service-table
       pointer handed to the vtable lambdas is &svc_, and self()
       recovers the S7Service* by treating it as `this`. */
    TplcS7Service svc_{};
    ClockFn clock_;
    void* clock_ctx_;
    std::unordered_map<uint32_t, Conn> conns_;
    std::unordered_map<uint32_t, Io> ios_;
    uint32_t next_conn_ = 1;
    uint32_t next_io_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_S7_SERVICE_HPP */
