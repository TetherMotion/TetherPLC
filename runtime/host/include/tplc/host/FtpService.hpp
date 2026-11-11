/* FtpService.hpp — FTP client (RFC 959) service, PASV data mode.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_FTP_SERVICE_HPP
#define TPLC_HOST_FTP_SERVICE_HPP

#include <tplc/tplc_ftp.h>

#include <netinet/in.h>

#include <cstdint>
#include <unordered_map>

namespace tplc {

class FtpService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit FtpService(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcFtpService& table() const { return svc_; }

private:
    enum class ConnStage : uint8_t {
        TcpConnect, UserWait, PassWait, Ready, Failed,
    };
    enum class XferStage : uint8_t {
        TypeCmd,      /* TYPE I control txn               */
        PasvCmd,      /* PASV control txn                 */
        PasvParse,    /* parse 227 -> data addr           */
        DataConnect,  /* nonblocking connect to data ep   */
        XferCmd,      /* RETR/STOR/LIST control txn       */
        Wait150,      /* interim reply before data flows  */
        Transfer,     /* streaming data                   */
        FinishWait,   /* STOR: wait for final 2xx/5xx     */
        Done,
    };

    struct Conn {
        int fd = -1;
        ConnStage stage = ConnStage::TcpConnect;
        TplcFtpOpenReq req{};
        uint64_t deadline_ns = 0;
        int err = 0;
        uint32_t resp_code = 0;
        /* control channel line assembly + tx queue */
        uint8_t rx[1024]{};
        size_t rx_len = 0;
        uint8_t tx[320]{};
        size_t tx_len = 0;
        size_t tx_off = 0;
        /* pending login command state */
        bool user_sent = false;
        uint32_t active_txn = 0;   /* cmd/xfer handle or 0        */
    };

    struct Txn {
        TplcFtpConnHandle conn = 0;
        bool is_xfer = false;
        uint64_t deadline_ns = 0;
        bool finished = false;
        TplcFtpTxnInfo info{};
        char text[TPLC_FTP_MAX_TEXT]{};
        /* cmd txn state */
        /* xfer state */
        XferStage xs = XferStage::TypeCmd;
        uint32_t op = 0;
        char path[TPLC_FTP_MAX_STR]{};
        int data_fd = -1;
        uint64_t data_total = 0;
        bool feed_done = false;    /* STOR: finish() called       */
        uint8_t up_pending[4096]{};
        size_t up_len = 0;
        size_t up_off = 0;
        uint32_t sub_state = 0;    /* internal ctrl txn step      */
        uint32_t pasv_port = 0;
        struct in_addr pasv_addr{};
    };

    static FtpService* self(void* p);

    int32_t open(void*, const TplcFtpOpenReq*, TplcFtpConnHandle*);
    int32_t connPoll(void*, TplcFtpConnHandle, TplcFtpConnInfo*);
    int32_t cmd(void*, TplcFtpConnHandle, const TplcFtpCmdReq*,
                TplcFtpTxnHandle*);
    int32_t txnPoll(void*, TplcFtpTxnHandle, TplcFtpTxnInfo*,
                    char*, uint32_t);
    int32_t xferBegin(void*, TplcFtpConnHandle, const TplcFtpXferReq*,
                      TplcFtpTxnHandle*);
    int32_t xferPoll(void*, TplcFtpTxnHandle, TplcFtpTxnInfo*,
                     uint8_t*, uint32_t);
    int32_t xferFeed(void*, TplcFtpTxnHandle, const uint8_t*, uint32_t);
    int32_t xferFinish(void*, TplcFtpTxnHandle);
    int32_t abort(void*, TplcFtpTxnHandle);
    int32_t connClose(void*, TplcFtpConnHandle);

    void failConn(Conn& c, int err);
    void failTxn(Txn& t, int status, int err);
    bool pumpTx(Conn& c);
    /* returns 1 = complete reply line parsed into `line` (len),
       0 = incomplete, -1/-2 = closed/error, -3 = overflow */
    int pumpRxLine(Conn& c, char* line, size_t cap, size_t* llen);
    bool sendLine(Conn& c, const char* s);
    void dropTxn(Txn& t);

    /* svc_ must stay the first data member: the service-table
       pointer handed to the vtable lambdas is &svc_, and self()
       recovers the FtpService* by treating it as `this`. */
    TplcFtpService svc_{};
    ClockFn clock_;
    void* clock_ctx_;
    std::unordered_map<uint32_t, Conn> conns_;
    std::unordered_map<uint32_t, Txn> txns_;
    uint32_t next_conn_ = 1;
    uint32_t next_txn_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_FTP_SERVICE_HPP */
