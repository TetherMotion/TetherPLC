/* AdsService.hpp — tplc-ads host implementation.
 * AMS/TCP ADS client (Beckhoff wire format) behind the
 * TplcAdsService vtable. One outstanding request per connection;
 * every op returns immediately. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_ADSSERVICE_HPP
#define TPLC_HOST_ADSSERVICE_HPP

#include <tplc/tplc_ads.h>

#include <string>
#include <unordered_map>
#include <vector>

namespace tplc {

class AdsService {
public:
    AdsService();

    const TplcAdsService& table() const { return svc_; }

private:
    struct Conn {
        int         fd = -1;
        uint32_t    cstate = 0;   /* 0 connecting, 1 ready, 2 awaiting */
        uint16_t    cmd = 0;      /* pending ADS command id */
        uint32_t    invoke = 0;
        uint16_t    tport = 0;    /* AMS port for the AMS header */
        std::string tnetid;       /* target AMS NetID bytes source */
        std::vector<uint8_t> pending;  /* request bytes not yet sent */
        size_t      sent = 0;
        std::vector<uint8_t> rx;      /* accumulated response */
    };

    int32_t adsOpen(void* s, const char* host, uint16_t port,
                    TplcAdsHandle* out);
    int32_t adsRead(void* s, TplcAdsHandle h, uint32_t grp, uint32_t off,
                    uint32_t len, void* buf, uint32_t cap,
                    uint32_t* rlen, uint32_t* adserr);
    int32_t adsWrite(void* s, TplcAdsHandle h, uint32_t grp, uint32_t off,
                     const void* data, uint32_t len, uint32_t* adserr);
    int32_t adsReadState(void* s, TplcAdsHandle h, uint32_t* ads_state,
                         uint32_t* dev_state, uint32_t* adserr);
    int32_t adsWriteControl(void* s, TplcAdsHandle h, uint32_t ads_state,
                            uint32_t dev_state, uint32_t* adserr);
    int32_t adsClose(void* s, TplcAdsHandle h);

    Conn*       find(TplcAdsHandle h);
    void        drop(TplcAdsHandle h);
    int32_t     ensureConnected(Conn& c);
    int32_t     exchange(Conn& c, uint16_t cmd,
                         const std::vector<uint8_t>& req_payload,
                         std::vector<uint8_t>& resp);
    int32_t     opResult(Conn& c, const std::vector<uint8_t>& payload,
                         uint16_t cmd, uint32_t* adserr,
                         const uint8_t** data, uint32_t* dlen,
                         std::vector<uint8_t>& respbuf);
    static int32_t mapErr(int e);
    static void  put16(std::vector<uint8_t>& v, uint16_t x);
    static void  put32(std::vector<uint8_t>& v, uint32_t x);
    static uint16_t get16(const uint8_t* p);
    static uint32_t get32(const uint8_t* p);
    static void  putNetId(std::vector<uint8_t>& v, const char* netid);

    TplcAdsService                    svc_{};
    std::unordered_map<uint32_t, Conn> conns_;
    uint32_t                          next_handle_ = 1;
};

} /* namespace tplc */

#endif /* TPLC_HOST_ADSSERVICE_HPP */
