/* SmtpService.hpp — SMTP client (RFC 5321/5322) service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_SMTP_SERVICE_HPP
#define TPLC_HOST_SMTP_SERVICE_HPP

#include <tplc/tplc_smtp.h>

#include <netinet/in.h>

#include <cstdint>
#include <unordered_map>

namespace tplc {

class SmtpService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit SmtpService(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcSmtpService& table() const { return svc_; }

private:
    enum class Stage : uint8_t {
        TcpConnect,   /* nonblocking connect                   */
        GreetWait,    /* expect 220 greeting                   */
        EhloWait,     /* EHLO -> multiline 250                 */
        AuthWait,     /* AUTH LOGIN sub-states (user[0] set)  */
        MailWait,     /* MAIL FROM -> 250                      */
        RcptWait,     /* RCPT TO -> 250                        */
        DataWait,     /* DATA -> 354                           */
        BodySend,     /* stream headers+body with dot-stuff   */
        EndDataWait,  /* "." sent -> expect 250                */
        QuitWait,     /* QUIT -> expect 221                    */
        Done,
    };

    struct Txn {
        int fd = -1;
        Stage stage = Stage::TcpConnect;
        TplcSmtpReq req{};
        uint64_t deadline_ns = 0;
        bool finished = false;
        TplcSmtpInfo info{};
        /* control channel line assembly + tx queue */
        uint8_t rx[1024]{};
        size_t rx_len = 0;
        uint8_t tx[4608]{};      /* headers + dot-stuffed lines   */
        size_t tx_len = 0;
        size_t tx_off = 0;
        /* body feed buffer (dot-stuff processed per line) */
        uint8_t body[4096]{};
        size_t body_len = 0;
        size_t body_off = 0;     /* consumed up to here           */
        bool feed_done = false;  /* smtp_feed_finish called       */
        bool headers_sent = false;
        bool body_ended_nl = true; /* last body byte was '\n'      */
        uint64_t body_total = 0;
        uint32_t auth_step = 0;  /* 0=AUTH sent 1=user 2=pass    */
        char b64_user[96]{};
        char b64_pass[96]{};
        char line[512]{};
    };

    static SmtpService* self(void* p);

    int32_t begin(void*, const TplcSmtpReq*, TplcSmtpHandle*);
    int32_t poll(void*, TplcSmtpHandle, TplcSmtpInfo*);
    int32_t feed(void*, TplcSmtpHandle, const uint8_t*, uint32_t);
    int32_t feedFinish(void*, TplcSmtpHandle);
    int32_t abort(void*, TplcSmtpHandle);

    void failTxn(Txn& t, int status, int err);
    void dropTxn(Txn& t);
    bool pumpTx(Txn& t);
    int pumpRxLine(Txn& t, char* line, size_t cap, size_t* llen);
    bool sendLine(Txn& t, const char* s);
    void queueRaw(Txn& t, const char* s, size_t n);
    void queueLine(Txn& t, const char* s);
    /* move complete body lines through dot-stuffing into tx;
       flush also emits an unterminated tail (after feed_finish) */
    void pumpBody(Txn& t, bool flush);
    /* build the header block into tx once DATA is accepted */
    void queueHeaders(Txn& t);

    /* svc_ must stay the first data member: the service-table
       pointer handed to the vtable lambdas is &svc_, and self()
       recovers the SmtpService* by treating it as `this`. */
    TplcSmtpService svc_{};
    ClockFn clock_;
    void* clock_ctx_;
    std::unordered_map<uint32_t, Txn> txns_;
    uint32_t next_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_SMTP_SERVICE_HPP */
