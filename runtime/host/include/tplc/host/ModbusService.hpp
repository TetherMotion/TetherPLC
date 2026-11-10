/* ModbusService.hpp — host implementation of TplcModbusService.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_MODBUS_SERVICE_HPP
#define TPLC_HOST_MODBUS_SERVICE_HPP

#include <tplc/tplc_modbus.h>

#include <cstdint>
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
        uint64_t deadline = 0;
        bool has_deadline = false;
        uint16_t tid = 0;
        uint8_t unit = 0;
        uint8_t func = 0;
        std::vector<uint8_t> tx;   /* full ADU awaiting send      */
        size_t sent = 0;
        std::vector<uint8_t> rx;   /* accumulated response ADU    */
    };

    int32_t mbBegin(void* s, const TplcModbusReq* req, TplcModbusHandle* out);
    int32_t mbPoll(void* s, TplcModbusHandle h, TplcModbusResp* resp);
    int32_t mbAbort(void* s, TplcModbusHandle h);

    Txn* find(TplcModbusHandle h);
    void drop(TplcModbusHandle h);
    int32_t drive(Txn& t);
    int32_t finish(Txn& t, TplcModbusResp* resp);
    uint64_t nowNs() const;

    static void put16be(std::vector<uint8_t>& v, uint16_t x);
    static uint16_t get16be(const uint8_t* p);
    static int32_t mapErr(int e);

    TplcModbusService svc_{};
    std::unordered_map<uint32_t, Txn> txns_;
    uint32_t next_handle_ = 1;
    uint16_t next_tid_ = 1;
    ClockFn clock_;
    void* clock_ctx_;
};

} /* namespace tplc */

#endif /* TPLC_HOST_MODBUS_SERVICE_HPP */
