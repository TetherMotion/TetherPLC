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

#include <cstring>
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
}

Iec61850Service::~Iec61850Service() {
#ifdef TPLC_HAVE_IEC61850
    for (Conn& c : impl_->conns)
        if (c.conn) IedConnection_destroy(c.conn);
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
    LinkedList names = IedConnection_getDataDirectory(
        impl_->conns[h - 1].conn, &err, domain);
    if (err != IED_ERROR_OK || !names) {
        impl_->conns[h - 1].last_error = uint32_t(err);
        return TPLC_61850_E_MMS;
    }
    size_t off = 0; int32_t count = 0;
    for (LinkedList e = names; e; e = LinkedList_getNext(e)) {
        const char* n = (const char*)LinkedList_getData(e);
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

} // namespace tplc
