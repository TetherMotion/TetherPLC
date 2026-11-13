/* Iec61850Service.cpp — tplc-iec61850 IEC 61850 MMS client via
 * libiec61850's IedConnection_* API (opt-in like the other special
 * features — inert when TPLC_HAVE_IEC61850 is off).
 *
 * The PLC is the client (control-center side); libiec61850 owns the
 * full OSI stack (TPKT/COTP/Session/Presentation/ACSE/MMS).  The
 * surface maps connect -> IedConnection_connect, read -> readObject,
 * write -> writeObject, getnamelist -> getDataDirectory.  Values widen
 * through MmsValue to an LREAL/STRING pair like the OPC UA service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/Iec61850Service.hpp>

#ifdef TPLC_HAVE_IEC61850
#include <iec61850_client.h>
#include <mms_value.h>
#include <linked_list.h>
#endif

#include <cstdio>
#include <cstring>
#include <deque>
#include <map>
#include <mutex>
#include <string>
#include <vector>

namespace tplc {

namespace {

struct Conn {
#ifdef TPLC_HAVE_IEC61850
    IedConnection conn = nullptr;
#else
    void* conn = nullptr;
#endif
    int32_t  state = 0;
    uint32_t invoke_id = 0;
    uint32_t last_error = 0;
    /* reports received on the libiec61850 receiver thread, drained
       by report_poll on the PLC thread */
    std::mutex           reportsMtx;
    std::vector<std::string> reports;
#ifdef TPLC_HAVE_IEC61850
    /* controllable data objects keyed by object reference; command
       terminations queue here for control_poll */
    std::map<std::string, ControlObjectClient> ctls;
    std::mutex           ctlMtx;
    struct CtlEvent {
        uint32_t invoke, ctl_num;
        int32_t  success, err, type;
    };
    std::deque<CtlEvent> ctlEvents;
#endif
};

/* build the MMS objectReference "domain/item" or just "item" when
   domain is empty (VMD-level).  An "#FC" suffix on the item selects
   the functional constraint (ST, MX, CO, SP, SV, CF, DC, SG, SE,
   SR, OR, BL, EX, US, MS, RP, BR, LG, GO, ALL) — default ST. */
#ifdef TPLC_HAVE_IEC61850
FunctionalConstraint parseFc(const char* item, char* base, size_t cap) {
    std::snprintf(base, cap, "%s", item);
    char* h = std::strchr(base, '#');
    FunctionalConstraint fc = IEC61850_FC_ST;
    if (!h) return fc;
    *h = 0;
    const char* s = h + 1;
    static const struct { const char* n; FunctionalConstraint f; } m[] = {
        {"MX", IEC61850_FC_MX}, {"CO", IEC61850_FC_CO},
        {"SP", IEC61850_FC_SP}, {"SV", IEC61850_FC_SV},
        {"CF", IEC61850_FC_CF}, {"DC", IEC61850_FC_DC},
        {"SG", IEC61850_FC_SG}, {"SE", IEC61850_FC_SE},
        {"SR", IEC61850_FC_SR}, {"OR", IEC61850_FC_OR},
        {"BL", IEC61850_FC_BL}, {"EX", IEC61850_FC_EX},
        {"US", IEC61850_FC_US}, {"MS", IEC61850_FC_MS},
        {"RP", IEC61850_FC_RP}, {"BR", IEC61850_FC_BR},
        {"LG", IEC61850_FC_LG}, {"GO", IEC61850_FC_GO},
        {"ST", IEC61850_FC_ST}, {"ALL", IEC61850_FC_ALL},
    };
    for (auto& e : m)
        if (std::strcmp(s, e.n) == 0) { fc = e.f; break; }
    return fc;
}
#endif

void makeRef(const char* domain, const char* item, char* out, size_t cap) {
    if (domain && *domain)
        std::snprintf(out, cap, "%s/%s", domain, item);
    else
        std::snprintf(out, cap, "%s", item);
}

#ifdef TPLC_HAVE_IEC61850
/* MmsValue -> bounded STRING/LREAL pair.  Strings land in `s`
   (cap), numerics into `v`. */
void mmsToValue(MmsValue* mv, char* s, uint32_t cap, double* v, int* kind) {
    char buf[256];
    *kind = 0;
    if (!mv) return;
    switch (MmsValue_getType(mv)) {
        case MMS_BOOLEAN:  *v = MmsValue_getBoolean(mv) ? 1.0 : 0.0; *kind = 1; return;
        case MMS_INTEGER:
        case MMS_UNSIGNED: *v = double(MmsValue_toInt64(mv));       *kind = 2; return;
        case MMS_FLOAT:    *v = double(MmsValue_toFloat(mv));       *kind = 3; return;
        case MMS_BIT_STRING:
            *v = double(MmsValue_getBitStringAsInteger(mv));       *kind = 2; return;
        default:
            *kind = 4;
            if (s && cap) {
                const char* t = MmsValue_toString(mv);
                std::snprintf(s, cap, "%s", t ? t : "");
            }
            return;
    }
}
#endif

} // namespace

struct Iec61850Service::Impl {
    Conn conns[TPLC_IEC61850_HDL_MAX]{};
};

Iec61850Service::Iec61850Service() {
    impl_ = std::make_unique<Impl>();
    svc_.struct_size = sizeof(TplcIec61850Service);
    svc_.abi_version = TPLC_IEC61850_ABI_VERSION;
    svc_.host = this;
    svc_.iec61850_connect = [](void* s, const char* h, uint32_t p, uint32_t* o)
        { return self(s)->connect(s, h, p, o); };
    svc_.iec61850_close = [](void* s, uint32_t h)
        { return self(s)->close(s, h); };
    svc_.iec61850_status = [](void* s, uint32_t h, TplcIec61850Status* o)
        { return self(s)->status(s, h, o); };
    svc_.iec61850_read = [](void* s, uint32_t h, const char* d, const char* i,
                            uint8_t* b, uint32_t cap)
        { return self(s)->read(s, h, d, i, b, cap); };
    svc_.iec61850_write = [](void* s, uint32_t h, const char* d, const char* i,
                             const uint8_t* b, uint32_t n)
        { return self(s)->write(s, h, d, i, b, n); };
    svc_.iec61850_getnamelist = [](void* s, uint32_t h, const char* d,
                                   char* o, uint32_t cap)
        { return self(s)->getNameList(s, h, d, o, cap); };
    svc_.iec61850_rcb_get = [](void* s, uint32_t h, const char* r,
                               TplcIec61850Rcb* o)
        { return self(s)->rcbGet(s, h, r, o); };
    svc_.iec61850_rcb_set = [](void* s, uint32_t h, const char* r, uint32_t f)
        { return self(s)->rcbSet(s, h, r, f); };
    svc_.iec61850_report_poll = [](void* s, uint32_t h, char* o, uint32_t c)
        { return self(s)->reportPoll(s, h, o, c); };
    svc_.iec61850_control = [](void* s, uint32_t h, const char* r,
                               uint32_t a, const char* v, char* o,
                               uint32_t c)
        { return self(s)->control(s, h, r, a, v, o, c); };
    svc_.iec61850_control_poll = [](void* s, uint32_t h,
                                    TplcIec61850Ctl* o)
        { return self(s)->controlPoll(s, h, o); };
}

Iec61850Service::~Iec61850Service() {
#ifdef TPLC_HAVE_IEC61850
    for (Conn& c : impl_->conns) {
        for (auto& kv : c.ctls)
            if (kv.second) ControlObjectClient_destroy(kv.second);
        if (c.conn) IedConnection_destroy(c.conn);
    }
#endif
}

int32_t Iec61850Service::connect(void*, const char* host, uint32_t port,
                                 uint32_t* handle) {
    if (!host || !*host || !handle) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    for (uint32_t i = 0; i < TPLC_IEC61850_HDL_MAX; ++i) {
        if (!impl_->conns[i].conn) {
            IedConnection c = IedConnection_create();
            /* bound every blocking call under the PLC scan-stall
               watchdog (~2 s) — a stuck peer must fail fast, not
               stall the task. */
            IedConnection_setConnectTimeout(c, 800);
            IedConnection_setRequestTimeout(c, 800);
            IedClientError err;
            IedConnection_connect(c, &err, host, int(port ? port : 102));
            if (err != IED_ERROR_OK) {
                IedConnection_destroy(c);
                return TPLC_61850_E_CONN;
            }
            Conn& cn = impl_->conns[i];
            cn.conn = c; cn.state = 1; cn.invoke_id = 0; cn.last_error = 0;
            *handle = i + 1;
            return TPLC_61850_OK;
        }
    }
    return TPLC_61850_E_HANDLE;
#else
    (void)host; (void)port;
    return TPLC_61850_E_CONN;
#endif
}

int32_t Iec61850Service::close(void*, uint32_t h) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    Conn& c = impl_->conns[h - 1];
#ifdef TPLC_HAVE_IEC61850
    for (auto& kv : c.ctls)
        if (kv.second) ControlObjectClient_destroy(kv.second);
    c.ctls.clear();
    IedConnection_close(c.conn);
    IedConnection_destroy(c.conn);
#endif
    c.conn = nullptr; c.state = 0;
    return TPLC_61850_OK;
}

int32_t Iec61850Service::status(void*, uint32_t h, TplcIec61850Status* out) {
    if (!out) return TPLC_61850_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn) {
        out->state = 0;
        return TPLC_61850_OK;
    }
    const Conn& c = impl_->conns[h - 1];
    out->state = c.state;
    out->invoke_id = c.invoke_id;
    out->last_error = c.last_error;
    return TPLC_61850_OK;
}

int32_t Iec61850Service::read(void*, uint32_t h, const char* domain,
                              const char* item, uint8_t* buf, uint32_t cap) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    if (!item || !*item || !buf || !cap) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    char ref[256];
    char base[128];
    FunctionalConstraint fc = parseFc(item, base, sizeof(base));
    makeRef(domain, base, ref, sizeof(ref));
    IedClientError err;
    MmsValue* obj =
        IedConnection_readObject(impl_->conns[h - 1].conn, &err, ref,
                                 fc);
    if (err != IED_ERROR_OK || !obj) {
        impl_->conns[h - 1].last_error = uint32_t(err);
        return TPLC_61850_E_MMS;
    }
    /* MmsValue -> bounded string. */
    double v = 0.0; int kind = 0;
    char sbuf[256]; sbuf[0] = 0;
    mmsToValue(obj, sbuf, sizeof(sbuf), &v, &kind);
    int32_t n;
    if (kind == 4) {
        n = int32_t(std::strlen(sbuf));
        if (uint32_t(n) > cap) n = int32_t(cap);
        std::memcpy(buf, sbuf, uint32_t(n));
    } else {
        /* numerics go as their ASCII repr for the PLC surface */
        char nb[32];
        std::snprintf(nb, sizeof(nb), "%.6g", v);
        n = int32_t(std::strlen(nb));
        if (uint32_t(n) > cap) n = int32_t(cap);
        std::memcpy(buf, nb, uint32_t(n));
    }
    MmsValue_delete(obj);
    impl_->conns[h - 1].invoke_id++;
    return n;
#else
    return TPLC_61850_E_CONN;
#endif
}

int32_t Iec61850Service::write(void*, uint32_t h, const char* domain,
                               const char* item, const uint8_t* buf,
                               uint32_t len) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    if (!item || !*item || (!buf && len)) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    char ref[256];
    char base[128];
    FunctionalConstraint fc = parseFc(item, base, sizeof(base));
    makeRef(domain, base, ref, sizeof(ref));
    /* numeric when the text parses as a number, else a visible string */
    std::string txt(reinterpret_cast<const char*>(buf), len);
    char* endp = nullptr;
    double dv = std::strtod(txt.c_str(), &endp);
    MmsValue* mv = nullptr;
    if (endp && *endp == 0 && !txt.empty() &&
        txt.find_first_not_of("0123456789.+-eE") == std::string::npos)
        mv = MmsValue_newFloat(float(dv));
    else
        mv = MmsValue_newVisibleString(txt.c_str());
    IedClientError err;
    IedConnection_writeObject(impl_->conns[h - 1].conn, &err, ref,
                              fc, mv);
    MmsValue_delete(mv);
    if (err != IED_ERROR_OK) {
        impl_->conns[h - 1].last_error = uint32_t(err);
        return TPLC_61850_E_MMS;
    }
    impl_->conns[h - 1].invoke_id++;
    return TPLC_61850_OK;
#else
    return TPLC_61850_E_CONN;
#endif
}

int32_t Iec61850Service::getNameList(void*, uint32_t h, const char* domain,
                                     char* out, uint32_t cap) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    if (!domain || !*domain || !out || !cap) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    IedClientError err;
    /* getLogicalDeviceDirectory lists a domain's logical nodes —
       getDataDirectory is for a data-object ref (wrong level). */
    LinkedList names = IedConnection_getLogicalDeviceDirectory(
        impl_->conns[h - 1].conn, &err, domain);
    if (err != IED_ERROR_OK || !names) {
        impl_->conns[h - 1].last_error = uint32_t(err);
        return TPLC_61850_E_MMS;
    }
    size_t off = 0; int32_t count = 0;
    /* the returned list can carry a sentinel head with NULL data */
    for (LinkedList e = names; e; e = LinkedList_getNext(e)) {
        const char* n = (const char*)LinkedList_getData(e);
        if (!n) continue;
        size_t nl = std::strlen(n);
        if (off + nl + 1 < cap) {
            std::memcpy(out + off, n, nl);
            off += nl;
            out[off++] = '\n';
        }
        count++;
    }
    out[off < cap ? off : cap - 1] = 0;
    LinkedList_destroy(names);
    return count;
#else
    return TPLC_61850_E_CONN;
#endif
}

#ifdef TPLC_HAVE_IEC61850
namespace {

void cpyStr(char* dst, size_t cap, const char* src) {
    if (!dst || !cap) return;
    if (!src) { dst[0] = 0; return; }
    std::snprintf(dst, cap, "%s", src);
}

/* report handler: runs on the libiec61850 receiver thread; formats
   "dataset|reason|v0,v1,..." into the connection queue. */
void onIecReport(void* param, ClientReport report) {
    Conn* c = static_cast<Conn*>(param);
    if (!c || !report) return;
    std::string line;
    const char* ds = ClientReport_getDataSetName(report);
    line += ds ? ds : "?";
    line += '|';
    MmsValue* vals = ClientReport_getDataSetValues(report);
    if (vals && MmsValue_getType(vals) == MMS_ARRAY) {
        const int n = MmsValue_getArraySize(vals);
        for (int i = 0; i < n; ++i) {
            if (i) line += ',';
            MmsValue* e = MmsValue_getElement(vals, i);
            if (!e) { line += '-'; continue; }
            char vb[96];
            vb[0] = 0;
            MmsValue_printToBuffer(e, vb, sizeof(vb));
            line += vb[0] ? vb : "?";
        }
    }
    std::lock_guard<std::mutex> lk(c->reportsMtx);
    if (c->reports.size() < 64) c->reports.push_back(std::move(line));
}

/* CommandTermination handler: runs on the libiec61850 receiver
   thread; queues one event per terminated command. */
void onIecCtl(uint32_t invokeId, void* param, IedClientError err,
              ControlActionType type, bool success) {
    Conn* c = static_cast<Conn*>(param);
    if (!c) return;
    Conn::CtlEvent ev{};
    ev.invoke = invokeId;
    ev.success = success ? 1 : 0;
    ev.err = int32_t(err);
    ev.type = int32_t(type);
    std::lock_guard<std::mutex> lk(c->ctlMtx);
    if (err == IED_ERROR_OK && !success && !c->ctls.empty()) {
        /* pull the server's LastApplError ctlNum from any live
           control client on this connection (single-ctl conns are
           the common case). */
        LastApplError la = ControlObjectClient_getLastApplError(
            c->ctls.begin()->second);
        ev.ctl_num = uint32_t(la.ctlNum);
        ev.err = int32_t(la.error);
    }
    if (c->ctlEvents.size() < 32) c->ctlEvents.push_back(ev);
}

/* ASCII ctlVal text -> MmsValue matching the control object's
   declared ctlVal type.  Structure ctlVals (APC analogue) are not
   supported and return nullptr. */
MmsValue* ctlValFromText(ControlObjectClient ctl, const char* text) {
    if (!ctl || !text) return nullptr;
    switch (ControlObjectClient_getCtlValType(ctl)) {
        case MMS_BOOLEAN:
            return MmsValue_newBoolean(
                std::strcmp(text, "true") == 0 || std::strcmp(text, "1") == 0);
        case MMS_INTEGER:
            return MmsValue_newInteger(int32_t(std::strtol(text, nullptr, 10)));
        case MMS_UNSIGNED:
            return MmsValue_newUnsigned(uint32_t(std::strtoul(text, nullptr, 10)));
        case MMS_BIT_STRING: {
            /* coded-enum ctlVals (BSC/DPC/ISC) are 2-bit strings;
               "true"/nonzero -> value 1. */
            MmsValue* bs = MmsValue_newBitString(2);
            const bool on = std::strcmp(text, "true") == 0 ||
                            std::strcmp(text, "1") == 0 ||
                            std::strtol(text, nullptr, 10) != 0;
            MmsValue_setBitStringFromInteger(bs, on ? 1u : 0u);
            return bs;
        }
        case MMS_FLOAT:
            return MmsValue_newFloat(float(std::strtod(text, nullptr)));
        default:
            return nullptr;
    }
}

} // namespace
#endif

int32_t Iec61850Service::rcbGet(void*, uint32_t h, const char* rcbRef,
                                TplcIec61850Rcb* out) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    if (!rcbRef || !*rcbRef || !out) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    std::memset(out, 0, sizeof(*out));
    IedClientError err;
    ClientReportControlBlock rcb = IedConnection_getRCBValues(
        impl_->conns[h - 1].conn, &err, rcbRef, nullptr);
    if (err != IED_ERROR_OK || !rcb) {
        impl_->conns[h - 1].last_error = uint32_t(err);
        return TPLC_61850_E_MMS;
    }
    cpyStr(out->rpt_id, sizeof(out->rpt_id),
           ClientReportControlBlock_getRptId(rcb));
    cpyStr(out->data_set, sizeof(out->data_set),
           ClientReportControlBlock_getDataSetReference(rcb));
    out->conf_rev  = ClientReportControlBlock_getConfRev(rcb);
    out->buf_tm    = ClientReportControlBlock_getBufTm(rcb);
    out->intg_pd   = ClientReportControlBlock_getIntgPd(rcb);
    out->sq_num    = ClientReportControlBlock_getSqNum(rcb);
    out->opt_flds  = uint32_t(ClientReportControlBlock_getOptFlds(rcb));
    out->trg_ops   = uint32_t(ClientReportControlBlock_getTrgOps(rcb));
    out->rpt_ena   = ClientReportControlBlock_getRptEna(rcb) ? 1 : 0;
    out->resv      = ClientReportControlBlock_getResv(rcb) ? 1 : 0;
    out->buffered  = ClientReportControlBlock_isBuffered(rcb) ? 1 : 0;
    out->gi        = ClientReportControlBlock_getGI(rcb) ? 1 : 0;
    out->purge_buf = ClientReportControlBlock_getPurgeBuf(rcb) ? 1 : 0;
    out->resv_tms  = uint32_t(ClientReportControlBlock_getResvTms(rcb));
    /* EntryId is an opaque MMS octet string; fold it into a stable
       u32 digest (0 = absent). */
    if (MmsValue* eid =
            ClientReportControlBlock_getEntryId(rcb)) {
        uint32_t h = 2166136261u;
        const int n = MmsValue_getOctetStringSize(eid);
        for (int i = 0; i < n; ++i) {
            h ^= uint32_t(MmsValue_getOctetStringOctet(eid, i));
            h *= 16777619u;
        }
        out->entry_id = h;
        MmsValue_delete(eid);
    }
    ClientReportControlBlock_destroy(rcb);
    impl_->conns[h - 1].invoke_id++;
    return TPLC_61850_OK;
#else
    return TPLC_61850_E_CONN;
#endif
}

int32_t Iec61850Service::rcbSet(void*, uint32_t h, const char* rcbRef,
                                uint32_t flags) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    if (!rcbRef || !*rcbRef) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    Conn& c = impl_->conns[h - 1];
    ClientReportControlBlock rcb =
        ClientReportControlBlock_create(rcbRef);
    if (!rcb) return TPLC_61850_E_PARAM;
    const bool ena = (flags & 1) != 0;
    const bool resv = (flags & 2) != 0;
    const bool gi = (flags & 4) != 0;
    const bool purge = (flags & 8) != 0;
    ClientReportControlBlock_setRptEna(rcb, ena);
    ClientReportControlBlock_setResv(rcb, resv);
    ClientReportControlBlock_setGI(rcb, gi);
    if (purge) ClientReportControlBlock_setPurgeBuf(rcb, true);
    /* mask selection depends on the server-side RptEna state and on
       buffered vs unbuffered: $Resv is URCB-only ($ResvTms is the
       BRCB reservation attribute), and enabled RCBs accept only
       RptEna(disable)/GI/PurgeBuf writes. */
    const bool buffered = ClientReportControlBlock_isBuffered(rcb);
    if (buffered && resv)
        ClientReportControlBlock_setResvTms(rcb, 60);
    uint32_t mask = RCB_ELEMENT_RPT_ENA | RCB_ELEMENT_SQ_NUM |
                    (buffered ? (resv ? uint32_t(RCB_ELEMENT_RESV_TMS) : 0)
                              : uint32_t(RCB_ELEMENT_RESV));
    if (gi) mask |= RCB_ELEMENT_GI;
    if (purge) mask |= RCB_ELEMENT_PURGE_BUF;
    bool skipWrite = false;
    TplcIec61850Rcb snap{};
    const bool haveSnap =
        rcbGet(nullptr, h, rcbRef, &snap) == TPLC_61850_OK &&
        snap.rpt_id[0];
    if (haveSnap) {
        IedConnection_installReportHandler(
            c.conn, rcbRef, snap.rpt_id, onIecReport, &c);
        if (snap.rpt_ena) {
            mask = 0;
            if (!ena) mask |= uint32_t(RCB_ELEMENT_RPT_ENA);
            if (gi) mask |= uint32_t(RCB_ELEMENT_GI);
            if (purge) mask |= uint32_t(RCB_ELEMENT_PURGE_BUF);
            if (ena && mask == 0) skipWrite = true;
        } else if (!ena) {
            /* already disabled: a redundant RptEna=false is skipped
               so PurgeBuf-style calls stay clean. */
            mask &= ~uint32_t(RCB_ELEMENT_RPT_ENA);
        }
    }
    if (skipWrite) {
        ClientReportControlBlock_destroy(rcb);
        c.invoke_id++;
        return TPLC_61850_OK;
    }
    IedClientError err;
    IedConnection_setRCBValues(c.conn, &err, rcb, mask, true);
    ClientReportControlBlock_destroy(rcb);
    if (err != IED_ERROR_OK) {
        c.last_error = uint32_t(err);
        return TPLC_61850_E_MMS;
    }
    c.invoke_id++;
    return TPLC_61850_OK;
#else
    return TPLC_61850_E_CONN;
#endif
}

int32_t Iec61850Service::reportPoll(void*, uint32_t h, char* out,
                                    uint32_t cap) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    if (!out || !cap) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    Conn& c = impl_->conns[h - 1];
    std::vector<std::string> pending;
    {
        std::lock_guard<std::mutex> lk(c.reportsMtx);
        pending.swap(c.reports);
    }
    size_t off = 0;
    for (const auto& r : pending) {
        const size_t need = r.size() + 1;
        if (off + need > cap) break;
        std::memcpy(out + off, r.data(), r.size());
        off += r.size();
        out[off++] = '\n';
    }
    out[off < cap ? off : cap - 1] = 0;
    return int32_t(off);
#else
    return TPLC_61850_E_CONN;
#endif
}

int32_t Iec61850Service::control(void*, uint32_t h, const char* objRef,
                                 uint32_t action, const char* ctlVal,
                                 char* out, uint32_t cap) {
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
    if (!objRef || !*objRef || !out || !cap) return TPLC_61850_E_PARAM;
    if (action < 1 || action > 4) return TPLC_61850_E_PARAM;
#ifdef TPLC_HAVE_IEC61850
    Conn& c = impl_->conns[h - 1];
    /* objectReference is the full "domain/LN.DO" form the client
       library expects (same string space as RCB refs minus $FC$). */
    ControlObjectClient ctl = nullptr;
    {
        std::lock_guard<std::mutex> lk(c.ctlMtx);
        auto it = c.ctls.find(objRef);
        if (it != c.ctls.end()) ctl = it->second;
    }
    if (!ctl) {
        ctl = ControlObjectClient_create(objRef, c.conn);
        if (!ctl) {
            c.last_error = uint32_t(
                IedConnection_getLastApplError(c.conn).error);
            return TPLC_61850_E_MMS;
        }
        /* ctlVal echo: the text the caller supplied, or the server
           default for value-less commands. */
        std::snprintf(out, cap, "%s", ctlVal ? ctlVal : "");
        std::lock_guard<std::mutex> lk(c.ctlMtx);
        c.ctls[objRef] = ctl;
        /* NOTE: the map write happens after ControlObjectClient_create
           (which is synchronous) — terminations only arrive once a
           command is issued below. */
    }
    bool ok = false;
    IedClientError err = IED_ERROR_OK;
    MmsValue* mv = nullptr;
    if (action == 2 || action == 3) {
        mv = ctlValFromText(ctl, ctlVal);
        if (!mv) return TPLC_61850_E_PARAM;
    }
    switch (action) {
        case 1: /* select (SBO normal) */
            ok = ControlObjectClient_selectAsync(
                ctl, &err, onIecCtl, &c) != 0;
            break;
        case 2: /* select with value (SBO enhanced) */
            ok = ControlObjectClient_selectWithValueAsync(
                ctl, &err, mv, onIecCtl, &c) != 0;
            break;
        case 3: /* operate */
            ok = ControlObjectClient_operateAsync(
                ctl, &err, mv, 0, onIecCtl, &c) != 0;
            break;
        case 4: /* cancel */
            ok = ControlObjectClient_cancelAsync(
                ctl, &err, onIecCtl, &c) != 0;
            break;
    }
    if (mv) MmsValue_delete(mv);
    if (!ok || err != IED_ERROR_OK) {
        c.last_error = uint32_t(err);
        return TPLC_61850_E_MMS;
    }
    c.invoke_id++;
    return TPLC_61850_OK;
#else
    return TPLC_61850_E_CONN;
#endif
}

int32_t Iec61850Service::controlPoll(void*, uint32_t h,
                                     TplcIec61850Ctl* out) {
    if (!out) return TPLC_61850_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    if (h == 0 || h > TPLC_IEC61850_HDL_MAX || !impl_->conns[h - 1].conn)
        return TPLC_61850_E_HANDLE;
#ifdef TPLC_HAVE_IEC61850
    Conn& c = impl_->conns[h - 1];
    std::lock_guard<std::mutex> lk(c.ctlMtx);
    if (c.ctlEvents.empty()) return TPLC_61850_OK;
    const Conn::CtlEvent& ev = c.ctlEvents.front();
    out->invoke_id = ev.invoke;
    out->success   = ev.success;
    out->err       = uint32_t(ev.err);
    out->type      = uint32_t(ev.type);
    out->ctl_num   = ev.ctl_num;
    out->pending   = 1;
    c.ctlEvents.pop_front();
    return TPLC_61850_OK;
#else
    return TPLC_61850_E_CONN;
#endif
}

} // namespace tplc
