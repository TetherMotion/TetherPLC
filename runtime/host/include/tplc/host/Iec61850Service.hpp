/* Iec61850Service.hpp — tplc-iec61850 IEC 61850 MMS client.
 *
 * The PLC is the client (control-center side) over the full OSI
 * stack: TPKT (RFC1006) + COTP + Session + Presentation + ACSE +
 * MMS.  iec61850_connect runs the associate handshake;
 * iec61850_read/write carry Confirmed-Request Read/Write on
 * domain+item named variables; iec61850_getnamelist lists a domain's
 * objects.  RCB ops carry Get/SetRCBValues plus report reception
 * into a per-connection queue drained by report_poll.  Synchronous
 * per-transaction model on nonblocking poll-bounded IO.
 * GOOSE/SMV and the full data model are deliberately out of scope.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_IEC61850_SERVICE_HPP
#define TPLC_HOST_IEC61850_SERVICE_HPP

#include <tplc/tplc_iec61850.h>

#include <cstdint>
#include <memory>

namespace tplc {

class Iec61850Service {
public:
    Iec61850Service();
    ~Iec61850Service();
    Iec61850Service(const Iec61850Service&) = delete;
    Iec61850Service& operator=(const Iec61850Service&) = delete;

    bool active() const { return true; }
    const TplcIec61850Service& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcIec61850Service svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;

    int32_t connect(void*, const char*, uint32_t, uint32_t*);
    int32_t close(void*, uint32_t);
    int32_t status(void*, uint32_t, TplcIec61850Status*);
    int32_t read(void*, uint32_t, const char*, const char*,
                 uint8_t*, uint32_t);
    int32_t write(void*, uint32_t, const char*, const char*,
                  const uint8_t*, uint32_t);
    int32_t getNameList(void*, uint32_t, const char*, char*, uint32_t);
    int32_t rcbGet(void*, uint32_t, const char*, TplcIec61850Rcb*);
    int32_t rcbSet(void*, uint32_t, const char*, uint32_t);
    int32_t reportPoll(void*, uint32_t, char*, uint32_t);
    int32_t control(void*, uint32_t, const char*, uint32_t,
                    const char*, char*, uint32_t);
    int32_t controlPoll(void*, uint32_t, TplcIec61850Ctl*);
    int32_t dsRead(void*, uint32_t, const char*, char*, uint32_t);
    int32_t dsDirectory(void*, uint32_t, const char*, char*, uint32_t);
    int32_t dsWrite(void*, uint32_t, const char*, const char*, uint32_t);
    int32_t dsCreate(void*, uint32_t, const char*, const char*, uint32_t);
    int32_t dsDelete(void*, uint32_t, const char*);

    static Iec61850Service* self(void* p) {
        return static_cast<Iec61850Service*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_IEC61850_SERVICE_HPP */
