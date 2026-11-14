/* HostServices.cpp — SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/HostServices.hpp>

#include <cstdio>
#include <cstring>
#include <ctime>

namespace tplc {

static uint64_t mono_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

static HostServices* self(void* p) { return static_cast<HostServices*>(p); }

static uint64_t host_steady_ns(void* p) {
    return self(p)->steadyNs();
}

HostServices::HostServices(uint32_t caps)
    : rtmon_(&host_steady_ns, this), modbus_(&host_steady_ns, this),
      sacn_(&host_steady_ns, this), bacnet_(&host_steady_ns, this),
      pjlink_(&host_steady_ns, this), s7_(&host_steady_ns, this),
      s5_(&host_steady_ns, this),
      ftp_(&host_steady_ns, this), smtp_(&host_steady_ns, this),
      iec870_(&host_steady_ns, this), motion_(&host_steady_ns, this),
      motion_io_(&host_steady_ns, this), nci_io_(&host_steady_ns, this),
      fifo_io_(&host_steady_ns, this), fifo_(&host_steady_ns, this),
      kin_io_(&host_steady_ns, this), kin_(&host_steady_ns, this),
      mcgroup_(), cam_(), path_(),
      soe_(&host_steady_ns, this) {
    fifo_.setMotion(&motion_.table());
    svc_.struct_size  = sizeof(TplcServices);
    svc_.caps         = caps | TPLC_EXT_MOVABLE_BETWEEN;
    svc_.host         = this;
    svc_.log = [](void* h, uint8_t lvl, uint32_t code,
                  const void* args, uint32_t len) {
        self(h)->log(lvl, code, args, len);
    };
    svc_.diag_counter = [](void* h, uint32_t id, int64_t delta) {
        self(h)->diagCounter(id, delta);
    };
    svc_.steady_ns = [](void* h) { return self(h)->steadyNs(); };
    svc_.retain_dirty = [](void* h, uint64_t off, uint64_t len) {
        self(h)->retainDirty(off, len);
    };
    svc_.raise_event = [](void* h, uint32_t mask) {
        return self(h)->raiseEvent(mask);
    };
    svc_.get_service = [](void* h, const char* name, uint32_t len) {
        return self(h)->getService(name, len);
    };
}

void HostServices::log(uint8_t level, uint32_t code, const void* args,
                       uint32_t len) {
    log_count_++;
    if (log_fn_) { log_fn_(level, code, args, len); return; }
    fprintf(stderr, "[tplc log lvl=%u code=%u]", level, code);
    if (args && len) {
        const uint8_t* b = static_cast<const uint8_t*>(args);
        fprintf(stderr, " args=");
        for (uint32_t i = 0; i < len && i < 24; i++)
            fprintf(stderr, "%02x", b[i]);
    }
    fputc('\n', stderr);
}

void HostServices::diagCounter(uint32_t id, int64_t delta) {
    diag_[id] += delta;
}

uint64_t HostServices::steadyNs() const {
    return sim_on_ ? sim_now_ : mono_ns();
}

void HostServices::retainDirty(uint64_t off, uint64_t len) {
    retain_dirty_.emplace_back(off, len);
}

int32_t HostServices::raiseEvent(uint32_t) {
    return -1;  /* EVENT tasks unimplemented in v1 */
}

void* HostServices::getService(const char* name, uint32_t len) {
    if (len == 8 && memcmp(name, "tplc-net", 8) == 0)
        return const_cast<TplcNetService*>(&net_.table());
    if (len == 11 && memcmp(name, "tplc-serial", 11) == 0)
        return const_cast<TplcSerialService*>(&serial_.table());
    if (len == 8 && memcmp(name, "tplc-ads", 8) == 0)
        return const_cast<TplcAdsService*>(&ads_.table());
    if (len == 10 && memcmp(name, "tplc-rtmon", 10) == 0)
        return const_cast<TplcRtmonService*>(&rtmon_.table());
    if (len == 11 && memcmp(name, "tplc-modbus", 11) == 0)
        return const_cast<TplcModbusService*>(&modbus_.table());
    if (len == 9 && memcmp(name, "tplc-sacn", 9) == 0)
        return const_cast<TplcSacnService*>(&sacn_.table());
    if (len == 11 && memcmp(name, "tplc-bacnet", 11) == 0)
        return const_cast<TplcBacnetService*>(&bacnet_.table());
    if (len == 11 && memcmp(name, "tplc-pjlink", 11) == 0)
        return const_cast<TplcPjlinkService*>(&pjlink_.table());
    if (len == 7 && memcmp(name, "tplc-s7", 7) == 0)
        return const_cast<TplcS7Service*>(&s7_.table());
    if (len == 7 && memcmp(name, "tplc-s5", 7) == 0)
        return const_cast<TplcS5Service*>(&s5_.table());
    if (len == 8 && memcmp(name, "tplc-ftp", 8) == 0)
        return const_cast<TplcFtpService*>(&ftp_.table());
    if (len == 9 && memcmp(name, "tplc-smtp", 9) == 0)
        return const_cast<TplcSmtpService*>(&smtp_.table());
    if (len == 11 && memcmp(name, "tplc-iec870", 11) == 0)
        return const_cast<TplcIec870Service*>(&iec870_.table());
    if (len == 11 && memcmp(name, "tplc-motion", 11) == 0) {
        /* Prefer the Tether-IO-backed simulated CiA402 drives when a
           TPLC_MOTION_IO endpoint is configured and reachable; otherwise
           fall back to the in-process MotionService. */
        if (motion_io_.active())
            return const_cast<TplcMotionService*>(&motion_io_.table());
        return const_cast<TplcMotionService*>(&motion_.table());
    }
    if (len == 8 && memcmp(name, "tplc-nci", 8) == 0) {
        /* G-code interpretation lives in Tether: the channel exists
           only when the IO endpoint exposes nc.* — no in-process
           fallback. */
        if (nci_io_.active())
            return const_cast<TplcNciService*>(&nci_io_.table());
        return nullptr;
    }
    if (len == 9 && memcmp(name, "tplc-fifo", 9) == 0) {
        /* FiFoAxes setpoint FIFOs: prefer the Tether IO fifo.* surface;
           the in-process table only applies when tplc-motion is the
           local MotionService (IO-backed axes would not see rows). */
        if (fifo_io_.active())
            return const_cast<TplcFifoService*>(&fifo_io_.table());
        if (!motion_io_.active())
            return const_cast<TplcFifoService*>(&fifo_.table());
        return nullptr;
    }
    if (len == 8 && memcmp(name, "tplc-kin", 8) == 0) {
        /* Kinematic transformation objects: prefer the Tether IO
           kin.* surface; the in-process engine is valid regardless
           of the motion backend (it stores axis lists only). */
        if (kin_io_.active())
            return const_cast<TplcKinService*>(&kin_io_.table());
        return const_cast<TplcKinService*>(&kin_.table());
    }
    if (len == 11 && memcmp(name, "tplc-mcgroup", 11) == 0) {
        /* Coordinated-motion groups: pure PLC-side bookkeeping,
           always served in-process. */
        return const_cast<TplcMcGroupService*>(&mcgroup_.svc());
    }
    if (len == 8 && memcmp(name, "tplc-cam", 8) == 0) {
        /* Cam objects and slave bindings: pure PLC-side
           bookkeeping, always served in-process. */
        return const_cast<TplcCamService*>(&cam_.svc());
    }
    if (len == 9 && memcmp(name, "tplc-path", 9) == 0) {
        /* Coordinated path objects: pure PLC-side bookkeeping,
           always served in-process. */
        return const_cast<TplcPathService*>(&path_.svc());
    }
    if (len == 10 && memcmp(name, "tplc-opcua", 10) == 0) {
        /* OPC UA client: only when built with libopen62541. */
        if (opcua_.active())
            return const_cast<TplcOpcUaService*>(&opcua_.table());
        return nullptr;
    }
    if (len == 8 && memcmp(name, "tplc-eip", 8) == 0) {
        return const_cast<TplcEipService*>(&eip_.table());
    }
    if (len == 12 && memcmp(name, "tplc-iec61850", 12) == 0) {
        /* IEC 61850 MMS client: only when built with libiec61850. */
        if (iec61850_.active())
            return const_cast<TplcIec61850Service*>(&iec61850_.table());
        return nullptr;
    }
    if (len == 9 && memcmp(name, "tplc-mqtt", 9) == 0) {
        /* MQTT client: only when built with libpaho-mqtt3c. */
        if (mqtt_.active())
            return const_cast<TplcMqttService*>(&mqtt_.table());
        return nullptr;
    }
    if (len == 9 && memcmp(name, "tplc-ocpp", 9) == 0) {
        /* OCPP-J client: WebSocket + JSON-RPC, always available. */
        return const_cast<TplcOcppService*>(&ocpp_.table());
    }
    if (len == 10 && memcmp(name, "tplc-aes70", 10) == 0) {
        /* AES70/OCA audio-control client, always available. */
        return const_cast<TplcAes70Service*>(&aes70_.table());
    }
    if (len == 8 && memcmp(name, "tplc-dmx", 8) == 0) {
        /* DMX512 output: opt-in serial port, always available. */
        return const_cast<TplcDmxService*>(&dmx_.table());
    }
    if (len == 9 && memcmp(name, "tplc-mbus", 9) == 0) {
        /* M-Bus wired metering master: opt-in serial. */
        return const_cast<TplcMbusService*>(&mbus_.table());
    }
    if (len == 12 && memcmp(name, "tplc-enocean", 12) == 0) {
        /* EnOcean ESP3 transceiver: opt-in serial. */
        return const_cast<TplcEnoceanService*>(&enocean_.table());
    }
    if (len == 12 && memcmp(name, "tplc-genibus", 12) == 0) {
        /* GENIbus serial transceiver: opt-in serial. */
        return const_cast<TplcGenibusService*>(&genibus_.table());
    }
    if (len == 10 && memcmp(name, "tplc-mpbus", 10) == 0) {
        /* MP-Bus master: opt-in serial adapter. */
        return const_cast<TplcMpbusService*>(&mpbus_.table());
    }
    if (len == 11 && memcmp(name, "tplc-dynmem", 11) == 0) {
        /* Bounded deterministic dynamic-memory arena. */
        return const_cast<TplcDynmemService*>(&dynmem_.table());
    }
    if (len == 13 && memcmp(name, "tplc-eventlog", 13) == 0) {
        /* Bounded event journal (Tc3_EventLogger). */
        return const_cast<TplcEventlogService*>(&eventlog_.table());
    }
    if (len == 7 && memcmp(name, "tplc-db", 7) == 0) {
        /* SQLite-backed SQL service (Tc2/Tc3_Database). */
        return const_cast<TplcDbService*>(&db_.table());
    }
    if (len == 11 && memcmp(name, "tplc-xmlsrv", 11) == 0) {
        /* File-backed XML value store (Tc2_XmlDataSrv). */
        return const_cast<TplcXmlsrvService*>(&xmlsrv_.table());
    }
    if (len == 11 && memcmp(name, "tplc-ecsync", 11) == 0) {
        /* EtherCAT DC sync: only when an ec.dc.* endpoint is live. */
        if (ecsync_.active())
            return const_cast<TplcEcsyncService*>(&ecsync_.table());
        return nullptr;
    }
    if (len == 11 && memcmp(name, "tplc-ecdiag", 11) == 0) {
        /* EtherCAT CoE diag history: only when ec.coediag.* live. */
        if (ecdiag_.active())
            return const_cast<TplcEcDiagService*>(&ecdiag_.table());
        return nullptr;
    }
    if (len == 8 && memcmp(name, "tplc-soe", 8) == 0) {
        /* EtherCAT SoE mailbox: only when ec.soe.* live. */
        if (soe_.active())
            return const_cast<TplcSoeService*>(&soe_.table());
        return nullptr;
    }
    if (len == 8 && memcmp(name, "tplc-sym", 8) == 0) {
        /* By-name symbol access (Tc2_DataExchange). */
        return const_cast<TplcSymService*>(&sym_.table());
    }
    return nullptr;
}

} /* namespace tplc */
