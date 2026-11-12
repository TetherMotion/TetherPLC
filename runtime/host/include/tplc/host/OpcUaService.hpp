/* OpcUaService.hpp — tplc-opcua OPC UA client over libopen62541.
 *
 * The PLC is the UA client: ua_connect opens a session to an arbitrary
 * opc.tcp:// endpoint (Tether's motion server for the simulated path),
 * ua_read/ua_write map ns=2;s=<name> nodes through the DataSource
 * layer. Built only when TPLC_HAVE_OPCUA is defined (libopen62541);
 * without it active() is false and getService("tplc-opcua") returns
 * nullptr — honest absence, no stub.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_OPCUA_SERVICE_HPP
#define TPLC_HOST_OPCUA_SERVICE_HPP

#include <tplc/tplc_opcua.h>

#include <cstdint>
#include <memory>

namespace tplc {

class OpcUaService {
public:
    OpcUaService();
    ~OpcUaService();
    OpcUaService(const OpcUaService&) = delete;
    OpcUaService& operator=(const OpcUaService&) = delete;

    bool active() const { return active_; }
    const TplcOpcUaService& table() const { return svc_; }

private:
    /* svc_ MUST stay the first member (see MotionIoService). */
    TplcOpcUaService svc_{};
    struct Impl;
    std::unique_ptr<Impl> impl_;
    bool active_ = false;

    int32_t connect(void*, const char*, uint32_t*);
    int32_t disconnect(void*, uint32_t);
    int32_t status(void*, uint32_t, int32_t*);
    int32_t read(void*, uint32_t, const char*, TplcOpcUaValue*);
    int32_t write(void*, uint32_t, const char*, const TplcOpcUaValue*);
    int32_t nsIndex(void*, uint32_t, const char*, uint32_t*);

    static OpcUaService* self(void* p) {
        return static_cast<OpcUaService*>(p);
    }
};

} // namespace tplc

#endif /* TPLC_HOST_OPCUA_SERVICE_HPP */
