/* tplc_opcua.h — TplcOpcUaService: OPC UA client bridge for the TC2/TC3
 * OPC UA function blocks (TwinCAT Tc2_OPCUA / Tc3_PlcOpenOpcUa shape).
 *
 * The PLC is the OPC UA *client*; the endpoint is any OPC UA server —
 * for the simulated path it is Tether's tether_io_motion_server --opcua
 * endpoint, which exposes the whole param registry as ns=2;s=<name>
 * nodes. The service is only built when libopen62541 is available
 * (-DTPLC_HAVE_OPCUA); without it getService("tplc-opcua") returns
 * nullptr — honest absence, no stub.
 *
 * Node ids on the wire use the OPC UA text form ("ns=2;s=mc.time",
 * "i=85", "ns=1;i=1001") parsed by the backend.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_OPCUA_H
#define TPLC_OPCUA_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_OPCUA_ABI_VERSION 1u
#define TPLC_OPCUA_SVC_NAME "tplc-opcua"
#define TPLC_OPCUA_HDL_MAX 8u

/* connection states (ua_status). */
#define TPLC_UA_CONN_CLOSED    0
#define TPLC_UA_CONN_OPEN      1
#define TPLC_UA_CONN_ERROR    -1

/* service return codes. */
#define TPLC_UA_OK        0
#define TPLC_UA_E_PARAM  -1
#define TPLC_UA_E_HANDLE -2   /* bad/closed connection handle      */
#define TPLC_UA_E_CONN   -3   /* connect/session failure           */
#define TPLC_UA_E_NODE   -4   /* node id or service error          */
#define TPLC_UA_E_TYPE   -5   /* unsupported value type            */

/* value kinds (TplcOpcUaValue::type). */
#define TPLC_UA_TYPE_NONE    0
#define TPLC_UA_TYPE_BOOL    1
#define TPLC_UA_TYPE_I64     2
#define TPLC_UA_TYPE_U64     3
#define TPLC_UA_TYPE_F64     4
#define TPLC_UA_TYPE_STRING  5

/* scalar value container — numeric kinds widen into the union fields;
   STRING carries a bounded UTF-8 payload. */
typedef struct TplcOpcUaValue {
    int32_t  type;
    int32_t  reserved;
    union {
        int64_t  i64;
        uint64_t u64;
        double   f64;
    };
    char     str[232];
} TplcOpcUaValue;            /* 248 bytes                          */

typedef struct TplcOpcUaService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open a session to url (opc.tcp://host:port); writes a handle. */
    int32_t (*ua_connect)(void* svc, const char* url, uint32_t* handle);
    /* close the session + secure channel, release the handle. */
    int32_t (*ua_disconnect)(void* svc, uint32_t handle);
    /* connection state (TPLC_UA_CONN_*). */
    int32_t (*ua_status)(void* svc, uint32_t handle, int32_t* state);
    /* read one node's value ("ns=2;s=name", "i=85", ...). */
    int32_t (*ua_read)(void* svc, uint32_t handle, const char* nodeid,
                       TplcOpcUaValue* out);
    /* write one node's scalar value. */
    int32_t (*ua_write)(void* svc, uint32_t handle, const char* nodeid,
                        const TplcOpcUaValue* in);
    /* resolve a namespace URI to its index (for UA_GetNamespaceIndex). */
    int32_t (*ua_ns_index)(void* svc, uint32_t handle, const char* uri,
                           uint32_t* index);
} TplcOpcUaService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_OPCUA_H */
