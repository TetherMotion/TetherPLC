/* tplc_eip.h — TplcEipService: EtherNet/IP explicit-messaging client
 * for the TC3 EtherNet/IP function blocks (TwinCAT TF6281 / Tc3_EIP
 * adapter shape).
 *
 * The PLC is the EtherNet/IP client (scanner-side explicit messages):
 * eip_open opens a TCP session on 44818 and registers it
 * (RegisterSession), eip_get_attr / eip_set_attr carry unconnected
 * CIP Get_Attribute_Single / Set_Attribute_Single inside SendRRData,
 * and eip_close unregisters. eip_list_identity issues the
 * ListIdentity discovery command. Cyclic implicit (class-1) I/O is a
 * separate surface and deliberately not in this vtable.
 *
 * Node/object addressing: class/instance/attribute are u16 numbers;
 * the codec expands them to the CIP path segment form (8-bit when
 * < 256, else 16-bit padded).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_EIP_H
#define TPLC_EIP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define TPLC_EIP_ABI_VERSION 1u
#define TPLC_EIP_SVC_NAME "tplc-eip"
#define TPLC_EIP_HDL_MAX 8u

/* service return codes. */
#define TPLC_EIP_OK        0
#define TPLC_EIP_E_PARAM  -1
#define TPLC_EIP_E_HANDLE -2   /* bad/closed connection handle        */
#define TPLC_EIP_E_CONN   -3   /* connect/session failure             */
#define TPLC_EIP_E_CIP    -4   /* CIP general-status nonzero / timeout*/
#define TPLC_EIP_E_IO     -5   /* transport or encapsulation error    */

/* CIP general status the service exposes in eip_status. */
typedef struct TplcEipStatus {
    int32_t  state;          /* 0 closed, 1 registered, -1 error    */
    uint32_t session;        /* encapsulation session handle        */
    uint32_t last_cip;       /* last CIP general status seen        */
    uint32_t reserved[5];
} TplcEipStatus;             /* 32 bytes                            */

typedef struct TplcEipService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;
    /* open TCP to host:port, RegisterSession; writes a handle. */
    int32_t (*eip_open)(void* svc, const char* host, uint32_t port,
                        uint32_t* handle);
    /* unregister the session and close the connection. */
    int32_t (*eip_close)(void* svc, uint32_t handle);
    /* session status. */
    int32_t (*eip_status)(void* svc, uint32_t handle, TplcEipStatus* out);
    /* ListIdentity -> bounded ASCII summary into out (product name,
       vendor, ip). Returns TPLC_EIP_E_IO when unsupported. */
    int32_t (*eip_list_identity)(void* svc, const char* host,
                                 uint32_t port, char* out, uint32_t cap);
    /* Get_Attribute_Single: class/inst/attr -> raw bytes into buf
       (cap). Returns attribute length or a TPLC_EIP_E_* code. */
    int32_t (*eip_get_attr)(void* svc, uint32_t handle,
                            uint32_t cls, uint32_t inst, uint32_t attr,
                            uint8_t* buf, uint32_t cap);
    /* Set_Attribute_Single: class/inst/attr + raw bytes. */
    int32_t (*eip_set_attr)(void* svc, uint32_t handle,
                            uint32_t cls, uint32_t inst, uint32_t attr,
                            const uint8_t* buf, uint32_t len);
} TplcEipService;

#ifdef __cplusplus
}
#endif

#endif /* TPLC_EIP_H */
