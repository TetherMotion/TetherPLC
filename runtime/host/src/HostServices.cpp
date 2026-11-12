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
      ftp_(&host_steady_ns, this), smtp_(&host_steady_ns, this),
      iec870_(&host_steady_ns, this), motion_(&host_steady_ns, this),
      motion_io_(&host_steady_ns, this), nci_io_(&host_steady_ns, this) {
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
    return nullptr;
}

} /* namespace tplc */
