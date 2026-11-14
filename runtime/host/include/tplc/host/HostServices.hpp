/* HostServices.hpp — TplcServices implementation (plan doc 02 §2.8).
 * Every entry nonblocking/bounded; log + counters go to host-owned
 * sinks the service plane drains. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_SERVICES_HPP
#define TPLC_HOST_SERVICES_HPP

#include <tplc/tplc_abi.h>
#include <tplc/host/AdsService.hpp>
#include <tplc/host/Aes70Service.hpp>
#include <tplc/host/BacnetService.hpp>
#include <tplc/host/DbService.hpp>
#include <tplc/host/DmxService.hpp>
#include <tplc/host/DynmemService.hpp>
#include <tplc/host/EcDiagService.hpp>
#include <tplc/host/EcSyncService.hpp>
#include <tplc/host/EipService.hpp>
#include <tplc/host/FifoIoService.hpp>
#include <tplc/host/FifoService.hpp>
#include <tplc/host/KinIoService.hpp>
#include <tplc/host/KinService.hpp>
#include <tplc/host/McGroupService.hpp>
#include <tplc/host/CamService.hpp>
#include <tplc/host/PathService.hpp>
#include <tplc/host/FtpService.hpp>
#include <tplc/host/Iec61850Service.hpp>
#include <tplc/host/Iec870Service.hpp>
#include <tplc/host/ModbusService.hpp>
#include <tplc/host/EnoceanService.hpp>
#include <tplc/host/EventlogService.hpp>
#include <tplc/host/GenibusService.hpp>
#include <tplc/host/MpbusService.hpp>
#include <tplc/host/MbusService.hpp>
#include <tplc/host/MotionIoService.hpp>
#include <tplc/host/MqttService.hpp>
#include <tplc/host/MotionService.hpp>
#include <tplc/host/NciIoService.hpp>
#include <tplc/host/NetService.hpp>
#include <tplc/host/OcppService.hpp>
#include <tplc/host/OpcUaService.hpp>
#include <tplc/host/PjlinkService.hpp>
#include <tplc/host/RtmonService.hpp>
#include <tplc/host/SacnService.hpp>
#include <tplc/host/S7Service.hpp>
#include <tplc/host/S5Service.hpp>
#include <tplc/host/SoeService.hpp>
#include <tplc/host/SmtpService.hpp>
#include <tplc/host/SerialService.hpp>
#include <tplc/host/SymService.hpp>
#include <tplc/host/XmlsrvService.hpp>

#include <cstdint>
#include <functional>
#include <unordered_map>
#include <utility>
#include <vector>

namespace tplc {

class HostServices {
public:
    using LogFn = std::function<void(uint8_t level, uint32_t code,
                                     const void* args, uint32_t len)>;

    explicit HostServices(uint32_t caps);

    TplcServices& table() { return svc_; }
    /* by-name symbol access: the loaded program attaches its
       variable table + live blob here */
    SymService& symbols() { return sym_; }
    uint32_t      caps()  const { return svc_.caps; }

    void onLog(LogFn fn) { log_fn_ = std::move(fn); }

    /* observability for the service plane / tests */
    const std::unordered_map<uint32_t, int64_t>& diagCounters() const {
        return diag_;
    }
    const std::vector<std::pair<uint64_t, uint64_t>>&
    retainDirtyRanges() const { return retain_dirty_; }
    void clearRetainDirty() { retain_dirty_.clear(); }
    uint64_t logCount() const { return log_count_; }

    /* C-callback trampolines */
    void     log(uint8_t level, uint32_t code, const void* args,
                 uint32_t len);
    void     diagCounter(uint32_t id, int64_t delta);
    uint64_t steadyNs() const;
    /* sim clock (doc 14 §2): when enabled, steadyNs returns the
       master-published sim time — drives ctx->now_ns and deadline
       checks deterministically in --sim mode                       */
    void     setSimNow(uint64_t ns) { sim_now_ = ns; sim_on_ = true; }
    bool     simClock() const { return sim_on_; }
    void     retainDirty(uint64_t off, uint64_t len);
    int32_t  raiseEvent(uint32_t mask);
    void*    getService(const char* name, uint32_t len);

private:
    TplcServices svc_{};
    LogFn        log_fn_;
    std::unordered_map<uint32_t, int64_t> diag_;
    std::vector<std::pair<uint64_t, uint64_t>> retain_dirty_;
    uint64_t     log_count_ = 0;
    uint64_t     sim_now_ = 0;
    bool         sim_on_ = false;
    NetService   net_;
    SerialService serial_;
    AdsService   ads_;
    RtmonService rtmon_;
    ModbusService modbus_;
    SacnService  sacn_;
    BacnetService bacnet_;
    PjlinkService pjlink_;
    S7Service    s7_;
    S5Service    s5_;
    FtpService   ftp_;
    SmtpService  smtp_;
    Iec870Service iec870_;
    MotionService motion_;
    /* IO-backed DS402 backend; active() only when TPLC_MOTION_IO names a
       reachable Tether IO endpoint and the protocol lib is linked. */
    MotionIoService motion_io_;
    /* IO-backed NC interpreter channel; active() only when the
       TPLC_MOTION_IO endpoint also exposes nc.* params. */
    NciIoService  nci_io_;
    /* FiFoAxes numeric setpoint FIFOs: IO-backed when the endpoint
       exposes fifo.* params, otherwise the in-process channel table
       (only meaningful while tplc-motion is the local backend). */
    FifoIoService fifo_io_;
    FifoService   fifo_;
    /* Kinematic transformation objects: IO-backed when the endpoint
       exposes kin.* params, otherwise the in-process Tether
       kinematics engine (backend-independent, always valid). */
    KinIoService  kin_io_;
    KinService    kin_;
    /* Coordinated-motion group bookkeeping; in-process only (groups
       are PLC-side state, consumed by the trajectory engine later). */
    McGroupService mcgroup_;
    /* Cam objects + per-slave cam bindings; in-process only (table
       data and binding registry are PLC-side state). */
    CamService   cam_;
    /* Coordinated path objects (MC_PATH_DATA_REF); in-process only. */
    PathService  path_;
    /* OPC UA client; active() only when built with libopen62541. */
    OpcUaService  opcua_;
    /* EtherNet/IP explicit-messaging client; always available. */
    EipService    eip_;
    /* IEC 61850 MMS client; inert when built without libiec61850. */
    Iec61850Service iec61850_;
    /* MQTT client; inert when built without libpaho-mqtt3c. */
    MqttService   mqtt_;
    /* EtherCAT DC sync state over Tether IO ec.dc.* params. */
    EcSyncService ecsync_;
    /* EtherCAT CoE diag history over Tether IO ec.coediag.* params. */
    EcDiagService ecdiag_;
    /* EtherCAT SoE mailbox over Tether IO ec.soe.* params. */
    SoeService   soe_;
    /* OCPP-J client (WebSocket + JSON-RPC); always available. */
    OcppService   ocpp_;
    /* AES70/OCA audio-control client; always available. */
    Aes70Service  aes70_;
    /* DMX512 output (opt-in serial port); always available. */
    DmxService    dmx_;
    /* M-Bus wired metering master (opt-in serial); always on. */
    MbusService   mbus_;
    /* EnOcean ESP3 transceiver (opt-in serial); always on. */
    EnoceanService enocean_;
    /* GENIbus transceiver (opt-in serial); always on. */
    GenibusService genibus_;
    /* MP-Bus master (opt-in serial adapter); always on. */
    MpbusService mpbus_;
    DynmemService dynmem_;
    EventlogService eventlog_;
    DbService db_;
    XmlsrvService xmlsrv_;
    SymService sym_;
};

} /* namespace tplc */
#endif /* TPLC_HOST_SERVICES_HPP */
