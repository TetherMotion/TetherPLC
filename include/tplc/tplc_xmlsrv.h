/* tplc_xmlsrv.h — C ABI for the tplc-xmlsrv host service.
 * File-backed XML value store for the Tc2_XmlDataSrv surface
 * (FB_XmlSrvRead / FB_XmlSrvWrite and ByName variants).
 * Documents are simple element trees:
 *   <root><key>value</key>...</root>
 * An XPath is reduced to its last element segment — the key
 * looked up (or created) directly under the root.  Values are
 * plain text (no nested elements, no attributes) — the
 * documented reduction vs. the TF6421 server.
 * Vtable is fixed — append new fns at the end.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_XMLSRV_H
#define TPLC_XMLSRV_H

#include <stdint.h>

#define TPLC_XMLSRV_ABI_VERSION 1u
#define TPLC_XMLSRV_VAL_CAP     255u

typedef struct TplcXmlsrvService {
    uint32_t struct_size;
    uint32_t abi_version;
    void*    host;

    /* read: (file, xpath) -> value text in out (cap), *out_len;
       TPLC_XMLSRV_E_NOKEY when the element is absent. */
    int32_t (*xml_read)(void*, const char* path, const char* xpath,
                        uint8_t* out, uint32_t cap, uint32_t* out_len);

    /* write: (file, xpath, value) -> 0; creates the file and the
       element when missing, replaces the text otherwise. */
    int32_t (*xml_write)(void*, const char* path, const char* xpath,
                         const uint8_t* value, uint32_t value_len);

    void* reserved[8];
} TplcXmlsrvService;

#define TPLC_XMLSRV_OK        0
#define TPLC_XMLSRV_E_PARAM  -1
#define TPLC_XMLSRV_E_IO     -2
#define TPLC_XMLSRV_E_NOKEY  -3
#define TPLC_XMLSRV_E_XML    -4

#endif /* TPLC_XMLSRV_H */
