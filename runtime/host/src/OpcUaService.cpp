/* OpcUaService.cpp — tplc-opcua OPC UA client over libopen62541.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/OpcUaService.hpp>

#include <cstring>

#if defined(TPLC_HAVE_OPCUA)

#include <open62541/client.h>
#include <open62541/client_config_default.h>
#include <open62541/client_highlevel.h>

#include <string>

namespace tplc {

namespace {

/* Parse "ns=2;s=name" / "i=85" / "ns=1;i=1001" into a UA_NodeId. */
bool parseNodeId(const char* text, UA_NodeId* out) {
    if (!text || !*text) return false;
    *out = UA_NODEID_NULL;
    const std::string s(text);
    /* minimal textual parser: ns=<n>;(s|i)=<rest> */
    uint16_t ns = 0;
    std::string rest = s;
    const size_t semi = s.find(';');
    if (semi != std::string::npos && s.rfind("ns=", 0) == 0) {
        ns = static_cast<uint16_t>(std::stoi(s.substr(3, semi - 3)));
        rest = s.substr(semi + 1);
    }
    if (rest.rfind("s=", 0) == 0) {
        *out = UA_NODEID_STRING_ALLOC(ns, rest.c_str() + 2);
        return true;
    }
    if (rest.rfind("i=", 0) == 0) {
        *out = UA_NODEID_NUMERIC(ns, static_cast<UA_UInt32>(std::stoul(rest.substr(2))));
        return true;
    }
    if (rest.rfind("g=", 0) == 0 || rest.rfind("b=", 0) == 0) return false;
    /* bare "name" => ns=2;s=name (our motion-server params) */
    if (!rest.empty() && rest.find('=') == std::string::npos) {
        *out = UA_NODEID_STRING_ALLOC(2, rest.c_str());
        return true;
    }
    return false;
}

int32_t mapRead(const UA_Variant& v, TplcOpcUaValue* out) {
    out->type = TPLC_UA_TYPE_NONE;
    if (!v.data) return TPLC_UA_E_NODE;
    const UA_DataType* t = v.type;
    if (t == &UA_TYPES[UA_TYPES_BOOLEAN]) {
        out->type = TPLC_UA_TYPE_BOOL;
        out->u64 = *static_cast<const UA_Boolean*>(v.data) ? 1 : 0;
    } else if (t == &UA_TYPES[UA_TYPES_SBYTE] || t == &UA_TYPES[UA_TYPES_INT16]
            || t == &UA_TYPES[UA_TYPES_INT32] || t == &UA_TYPES[UA_TYPES_INT64]) {
        out->type = TPLC_UA_TYPE_I64;
        if (t == &UA_TYPES[UA_TYPES_SBYTE]) out->i64 = *static_cast<const int8_t*>(v.data);
        else if (t == &UA_TYPES[UA_TYPES_INT16]) out->i64 = *static_cast<const int16_t*>(v.data);
        else if (t == &UA_TYPES[UA_TYPES_INT32]) out->i64 = *static_cast<const int32_t*>(v.data);
        else out->i64 = *static_cast<const int64_t*>(v.data);
    } else if (t == &UA_TYPES[UA_TYPES_BYTE] || t == &UA_TYPES[UA_TYPES_UINT16]
            || t == &UA_TYPES[UA_TYPES_UINT32] || t == &UA_TYPES[UA_TYPES_UINT64]) {
        out->type = TPLC_UA_TYPE_U64;
        if (t == &UA_TYPES[UA_TYPES_BYTE]) out->u64 = *static_cast<const uint8_t*>(v.data);
        else if (t == &UA_TYPES[UA_TYPES_UINT16]) out->u64 = *static_cast<const uint16_t*>(v.data);
        else if (t == &UA_TYPES[UA_TYPES_UINT32]) out->u64 = *static_cast<const uint32_t*>(v.data);
        else out->u64 = *static_cast<const uint64_t*>(v.data);
    } else if (t == &UA_TYPES[UA_TYPES_FLOAT] || t == &UA_TYPES[UA_TYPES_DOUBLE]) {
        out->type = TPLC_UA_TYPE_F64;
        out->f64 = (t == &UA_TYPES[UA_TYPES_FLOAT])
            ? *static_cast<const float*>(v.data)
            : *static_cast<const double*>(v.data);
    } else if (t == &UA_TYPES[UA_TYPES_STRING]) {
        const UA_String* us = static_cast<const UA_String*>(v.data);
        out->type = TPLC_UA_TYPE_STRING;
        const size_t n = us->length < sizeof(out->str) - 1
            ? us->length : sizeof(out->str) - 1;
        std::memcpy(out->str, us->data, n);
        out->str[n] = 0;
    } else {
        return TPLC_UA_E_TYPE;
    }
    return TPLC_UA_OK;
}

} // namespace

struct OpcUaService::Impl {
    UA_Client* clients[TPLC_OPCUA_HDL_MAX] = {};
};

OpcUaService::OpcUaService() {
    impl_ = std::make_unique<Impl>();
    active_ = true;

    svc_.struct_size = sizeof(TplcOpcUaService);
    svc_.abi_version = TPLC_OPCUA_ABI_VERSION;
    svc_.host = this;
    svc_.ua_connect    = [](void* s, const char* u, uint32_t* h)
        { return self(s)->connect(s, u, h); };
    svc_.ua_disconnect = [](void* s, uint32_t h)
        { return self(s)->disconnect(s, h); };
    svc_.ua_status     = [](void* s, uint32_t h, int32_t* st)
        { return self(s)->status(s, h, st); };
    svc_.ua_read       = [](void* s, uint32_t h, const char* n, TplcOpcUaValue* v)
        { return self(s)->read(s, h, n, v); };
    svc_.ua_write      = [](void* s, uint32_t h, const char* n,
                            const TplcOpcUaValue* v)
        { return self(s)->write(s, h, n, v); };
    svc_.ua_ns_index   = [](void* s, uint32_t h, const char* u, uint32_t* i)
        { return self(s)->nsIndex(s, h, u, i); };
}

OpcUaService::~OpcUaService() {
    for (UA_Client* c : impl_->clients)
        if (c) { UA_Client_disconnect(c); UA_Client_delete(c); }
}

int32_t OpcUaService::connect(void*, const char* url, uint32_t* handle) {
    if (!url || !*url || !handle) return TPLC_UA_E_PARAM;
    for (uint32_t i = 0; i < TPLC_OPCUA_HDL_MAX; ++i) {
        if (!impl_->clients[i]) {
            UA_Client* c = UA_Client_new();
            UA_ClientConfig_setDefault(UA_Client_getConfig(c));
            const UA_StatusCode st = UA_Client_connect(c, url);
            if (st != UA_STATUSCODE_GOOD) { UA_Client_delete(c); return TPLC_UA_E_CONN; }
            impl_->clients[i] = c;
            *handle = i + 1;
            return TPLC_UA_OK;
        }
    }
    return TPLC_UA_E_HANDLE;
}

int32_t OpcUaService::disconnect(void*, uint32_t h) {
    if (h == 0 || h > TPLC_OPCUA_HDL_MAX || !impl_->clients[h - 1])
        return TPLC_UA_E_HANDLE;
    UA_Client_disconnect(impl_->clients[h - 1]);
    UA_Client_delete(impl_->clients[h - 1]);
    impl_->clients[h - 1] = nullptr;
    return TPLC_UA_OK;
}

int32_t OpcUaService::status(void*, uint32_t h, int32_t* state) {
    if (!state) return TPLC_UA_E_PARAM;
    if (h == 0 || h > TPLC_OPCUA_HDL_MAX || !impl_->clients[h - 1]) {
        *state = TPLC_UA_CONN_CLOSED;
        return TPLC_UA_OK;
    }
    UA_SecureChannelState scs;
    UA_SessionState ss;
    UA_StatusCode cs;
    UA_Client_getState(impl_->clients[h - 1], &scs, &ss, &cs);
    *state = (ss == UA_SESSIONSTATE_ACTIVATED && cs == UA_STATUSCODE_GOOD)
        ? TPLC_UA_CONN_OPEN : TPLC_UA_CONN_ERROR;
    return TPLC_UA_OK;
}

int32_t OpcUaService::read(void*, uint32_t h, const char* nid, TplcOpcUaValue* out) {
    if (h == 0 || h > TPLC_OPCUA_HDL_MAX || !impl_->clients[h - 1])
        return TPLC_UA_E_HANDLE;
    if (!out) return TPLC_UA_E_PARAM;
    UA_NodeId id;
    if (!parseNodeId(nid, &id)) return TPLC_UA_E_NODE;
    UA_Variant v;
    UA_Variant_init(&v);
    const UA_StatusCode st =
        UA_Client_readValueAttribute(impl_->clients[h - 1], id, &v);
    UA_NodeId_clear(&id);
    if (st != UA_STATUSCODE_GOOD) { UA_Variant_clear(&v); return TPLC_UA_E_NODE; }
    const int32_t rc = mapRead(v, out);
    UA_Variant_clear(&v);
    return rc;
}

int32_t OpcUaService::write(void*, uint32_t h, const char* nid,
                            const TplcOpcUaValue* in) {
    if (h == 0 || h > TPLC_OPCUA_HDL_MAX || !impl_->clients[h - 1])
        return TPLC_UA_E_HANDLE;
    if (!in) return TPLC_UA_E_PARAM;
    UA_NodeId id;
    if (!parseNodeId(nid, &id)) return TPLC_UA_E_NODE;
    UA_Variant v;
    UA_Variant_init(&v);
    switch (in->type) {
        case TPLC_UA_TYPE_BOOL: {
            UA_Boolean b = in->u64 != 0;
            UA_Variant_setScalarCopy(&v, &b, &UA_TYPES[UA_TYPES_BOOLEAN]);
            break;
        }
        case TPLC_UA_TYPE_I64:
            UA_Variant_setScalarCopy(&v, &in->i64, &UA_TYPES[UA_TYPES_INT64]);
            break;
        case TPLC_UA_TYPE_U64:
            UA_Variant_setScalarCopy(&v, &in->u64, &UA_TYPES[UA_TYPES_UINT64]);
            break;
        case TPLC_UA_TYPE_F64:
            UA_Variant_setScalarCopy(&v, &in->f64, &UA_TYPES[UA_TYPES_DOUBLE]);
            break;
        case TPLC_UA_TYPE_STRING: {
            UA_String us = UA_STRING(const_cast<char*>(in->str));
            UA_Variant_setScalarCopy(&v, &us, &UA_TYPES[UA_TYPES_STRING]);
            break;
        }
        default:
            UA_NodeId_clear(&id);
            return TPLC_UA_E_TYPE;
    }
    const UA_StatusCode st =
        UA_Client_writeValueAttribute(impl_->clients[h - 1], id, &v);
    UA_Variant_clear(&v);
    UA_NodeId_clear(&id);
    return st == UA_STATUSCODE_GOOD ? TPLC_UA_OK : TPLC_UA_E_NODE;
}

int32_t OpcUaService::nsIndex(void*, uint32_t h, const char* uri,
                              uint32_t* index) {
    if (h == 0 || h > TPLC_OPCUA_HDL_MAX || !impl_->clients[h - 1])
        return TPLC_UA_E_HANDLE;
    if (!uri || !index) return TPLC_UA_E_PARAM;
    /* read Server_NamespaceArray (i=2255) and find the URI. */
    UA_Variant v;
    UA_Variant_init(&v);
    const UA_StatusCode st = UA_Client_readValueAttribute(
        impl_->clients[h - 1],
        UA_NODEID_NUMERIC(0, UA_NS0ID_SERVER_NAMESPACEARRAY), &v);
    if (st != UA_STATUSCODE_GOOD) { UA_Variant_clear(&v); return TPLC_UA_E_NODE; }
    int32_t rc = TPLC_UA_E_NODE;
    if (UA_Variant_hasArrayType(&v, &UA_TYPES[UA_TYPES_STRING])) {
        const UA_String* arr = static_cast<const UA_String*>(v.data);
        for (size_t i = 0; i < v.arrayLength; ++i) {
            if (std::strlen(uri) == arr[i].length &&
                std::memcmp(uri, arr[i].data, arr[i].length) == 0) {
                *index = static_cast<uint32_t>(i);
                rc = TPLC_UA_OK;
                break;
            }
        }
    }
    UA_Variant_clear(&v);
    return rc;
}

} // namespace tplc

#else /* !TPLC_HAVE_OPCUA */

namespace tplc {

struct OpcUaService::Impl {};

OpcUaService::OpcUaService() : active_(false) {
    svc_.struct_size = sizeof(TplcOpcUaService);
    svc_.abi_version = TPLC_OPCUA_ABI_VERSION;
    svc_.host = this;
}
OpcUaService::~OpcUaService() = default;
int32_t OpcUaService::connect(void*, const char*, uint32_t*) { return TPLC_UA_E_CONN; }
int32_t OpcUaService::disconnect(void*, uint32_t) { return TPLC_UA_E_HANDLE; }
int32_t OpcUaService::status(void*, uint32_t, int32_t*) { return TPLC_UA_E_HANDLE; }
int32_t OpcUaService::read(void*, uint32_t, const char*, TplcOpcUaValue*) { return TPLC_UA_E_NODE; }
int32_t OpcUaService::write(void*, uint32_t, const char*, const TplcOpcUaValue*) { return TPLC_UA_E_NODE; }
int32_t OpcUaService::nsIndex(void*, uint32_t, const char*, uint32_t*) { return TPLC_UA_E_NODE; }

} // namespace tplc

#endif /* TPLC_HAVE_OPCUA */
