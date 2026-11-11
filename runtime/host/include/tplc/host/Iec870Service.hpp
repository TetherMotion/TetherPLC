/* Iec870Service.hpp — IEC 60870-5-104 client service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_IEC870_SERVICE_HPP
#define TPLC_HOST_IEC870_SERVICE_HPP

#include <tplc/tplc_iec870.h>

#include <cstdint>
#include <unordered_map>

namespace tplc {

class Iec870Service {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit Iec870Service(ClockFn clock = nullptr,
                           void* clock_ctx = nullptr);

    const TplcIec870Service& table() const { return svc_; }

private:
    enum class Stage : uint8_t {
        TcpConnect,   /* waiting for connect()                */
        StartDtWait,  /* STARTDT act sent, waiting for con     */
        Active,       /* data transfer active                  */
        Failed,
    };

    struct RxAsdu {
        uint32_t type_id = 0;
        uint32_t vsq = 0;
        uint32_t cot = 0;
        uint32_t ca = 0;
        uint32_t i_frame_seq = 0;
        uint8_t  payload[TPLC_IEC870_MAX_ASDU]{};
        uint16_t payload_len = 0;
    };

    struct Conn {
        int fd = -1;
        Stage stage = Stage::TcpConnect;
        TplcIec870OpenReq req{};
        uint64_t deadline_ns = 0;
        int err = 0;
        /* outbound I-frame (one outstanding tx per conn) */
        uint8_t tx[320]{};
        size_t tx_len = 0;
        size_t tx_off = 0;
        bool tx_frame = false;    /* tx[] holds an I-frame      */
        /* U/S frame stashed while an I-frame is mid-write */
        uint8_t ctrl_pending[8]{};
        size_t ctrl_pending_len = 0;
        /* inbound APCI frame assembly */
        uint8_t rx[320]{};
        size_t rx_len = 0;
        /* 104 link-layer state */
        uint16_t send_count = 0;      /* next I-frame send seq   */
        uint16_t recv_count = 0;      /* next expected I-frame
                                         send seq (our recv)     */
        uint16_t confirmed_send = 0;  /* peer NR = our acked     */
        uint16_t unconfirmed_rx = 0;  /* I-frames awaiting S-ack */
        bool testfr_outstanding = false;
        uint64_t testfr_deadline_ns = 0;
        uint64_t last_rx_ns = 0;
        /* received-ASDU queue (consume-on-read via rx_poll) */
        RxAsdu queue[TPLC_IEC870_RX_QUEUE]{};
        uint32_t q_head = 0;
        uint32_t q_count = 0;
        uint32_t rx_dropped = 0;
        TplcIec870ConnHandle self = 0;
        uint32_t active_tx = 0;       /* tx handle or 0           */
    };

    struct Tx {
        TplcIec870ConnHandle conn = 0;
        bool finished = false;
        TplcIec870TxInfo info{};
    };

    static Iec870Service* self(void* p);

    int32_t open(void*, const TplcIec870OpenReq*, TplcIec870ConnHandle*);
    int32_t connPoll(void*, TplcIec870ConnHandle, TplcIec870ConnInfo*);
    int32_t send(void*, TplcIec870ConnHandle, const TplcIec870TxReq*,
                 TplcIec870TxHandle*);
    int32_t txPoll(void*, TplcIec870TxHandle, TplcIec870TxInfo*);
    int32_t txAbort(void*, TplcIec870TxHandle);
    int32_t rxPoll(void*, TplcIec870ConnHandle, TplcIec870RxInfo*,
                   uint8_t*, uint32_t);
    int32_t connClose(void*, TplcIec870ConnHandle);

    void failConn(Conn& c, int err);
    bool pumpTx(Conn& c);
    /* Active-stage pump shared by conn/tx/rx polls: flush tx,
       drain rx frames into the ASDU queue, TESTFR keepalive */
    bool driveActive(Conn& c);
    /* recv + APCI frame extraction; returns one complete frame in
       `frame` and its length in `flen`, 0 when none is ready /
       -1 on overflow or peer close. Consumes the frame from rx. */
    int pumpRx(Conn& c, uint8_t* frame, size_t cap, size_t* flen);
    void handleFrame(Conn& c, const uint8_t* f, size_t n);
    void queueAsdu(Conn& c, const uint8_t* asdu, size_t n,
                   uint32_t i_seq);
    void sendUFrame(Conn& c, uint8_t func);
    void sendSFrame(Conn& c);
    void finishTx(Conn& c, int status, int err);

    /* svc_ must stay the first data member: the service-table
       pointer handed to the vtable lambdas is &svc_, and self()
       recovers the Iec870Service* by treating it as `this`. */
    TplcIec870Service svc_{};
    ClockFn clock_;
    void* clock_ctx_;
    std::unordered_map<uint32_t, Conn> conns_;
    std::unordered_map<uint32_t, Tx> txs_;
    uint32_t next_conn_ = 1;
    uint32_t next_tx_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_IEC870_SERVICE_HPP */
