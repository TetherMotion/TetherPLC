/* XmlsrvService.hpp — tplc-xmlsrv file-backed XML value store.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_XMLSRV_SERVICE_HPP
#define TPLC_XMLSRV_SERVICE_HPP

#include <tplc/tplc_xmlsrv.h>
#include <memory>

namespace tplc {

class XmlsrvService {
public:
    XmlsrvService();
    ~XmlsrvService();
    const TplcXmlsrvService& table() const { return svc_; }

private:
    static XmlsrvService* self(void* s) {
        return static_cast<XmlsrvService*>(
            reinterpret_cast<TplcXmlsrvService*>(s)->host);
    }

    struct Impl;
    std::unique_ptr<Impl> impl_;
    TplcXmlsrvService svc_{};

    int32_t read(void*, const char* path, const char* xpath,
                 uint8_t* out, uint32_t cap, uint32_t* out_len);
    int32_t write(void*, const char* path, const char* xpath,
                  const uint8_t* value, uint32_t value_len);
};

} /* namespace tplc */
#endif /* TPLC_XMLSRV_SERVICE_HPP */
