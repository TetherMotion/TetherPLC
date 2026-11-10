/* ModbusService.hpp — host implementation of TplcModbusService.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_MODBUS_SERVICE_HPP
#define TPLC_HOST_MODBUS_SERVICE_HPP

#include <tplc/tplc_modbus.h>

#include <cstdint>
#include <netinet/in.h>
#include <unordered_map>
#include <vector>

namespace tplc {

class ModbusService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit ModbusService(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcModbusService& table() const { return svc_; }

private:
    struct Txn {
        int fd = -1;
        int cstate = 0;      /* 0 connecting, 1 sending, 2 receiving */
        bool udp = false;    /* UDP datagram transport vs TCP stream */
        bool rtu = false;    /* RTU serial transport vs TCP stream  */
        uint32_t baud = 0;   /* rtu: line speed for gap timing     */
        uint64_t last_rx_ns = 0; /* rtu: last byte arrival time     */
        uint64_t deadline = 0;
        bool has_deadline = false;
        uint16_t tid = 0;
        uint8_t unit = 0;
        uint8_t func = 0;
        std::vector<uint8_t> tx;   /* full ADU awaiting send      */
        size_t sent = 0;           /* tcp: bytes sent; udp: 0/1   */
        std::vector<uint8_t> rx;   /* accumulated response ADU    */
        struct sockaddr_in peer{}; /* udp: request destination   */
    };

    int32_t mbBegin(void* s, const TplcModbusReq* req, TplcModbusHandle* out);
    int32_t mbBeginUdp(void* s, const TplcModbusReq* req,
                       TplcModbusHandle* out);
    int32_t mbBeginRtu(void* s, const TplcModbusRtuReq* req,
                       TplcModbusHandle* out);
    int32_t mbPoll(void* s, TplcModbusHandle h, TplcModbusResp* resp);
    int32_t mbAbort(void* s, TplcModbusHandle h);

    Txn* find(TplcModbusHandle h);
    void drop(TplcModbusHandle h);
    int32_t drive(Txn& t);
    int32_t udpDrive(Txn& t);
    int32_t rtuDrive(Txn& t);
    int32_t finish(Txn& t, TplcModbusResp* resp);
    int32_t rtuFinish(Txn& t, TplcModbusResp* resp);
    int32_t finishPdu(const Txn& t, const uint8_t* pdu, size_t plen,
                      TplcModbusResp* resp);
    uint64_t nowNs() const;

    static int32_t checkReq(const TplcModbusReq* req);
    static int32_t checkRtuReq(const TplcModbusRtuReq* req);
    void buildAdu(Txn& t, const TplcModbusReq* req);
    void buildRtuFrame(Txn& t, const TplcModbusRtuReq* req);
    static void put16be(std::vector<uint8_t>& v, uint16_t x);
    static uint16_t get16be(const uint8_t* p);
    static int32_t mapErr(int e);
    static uint16_t crc16(const uint8_t* p, size_t n);
    /* expected RTU response size once determinable; 0 = keep
       accumulating (gap-based framing), SIZE_MAX = malformed */
    static size_t rtuNeed(const Txn& t);
    static bool rtuCrcOk(const uint8_t* f, size_t n);
    static uint64_t rtuGapNs(const Txn& t);

    TplcModbusService svc_{};
    std::unordered_map<uint32_t, Txn> txns_;
    uint32_t next_handle_ = 1;
    uint16_t next_tid_ = 1;
    ClockFn clock_;
    void* clock_ctx_;
};

} /* namespace tplc */

#endif /* TPLC_HOST_MODBUS_SERVICE_HPP */
