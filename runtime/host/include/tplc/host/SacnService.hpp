/* SacnService.hpp — sACN (ANSI E1.31) source/receiver service.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_SACN_SERVICE_HPP
#define TPLC_HOST_SACN_SERVICE_HPP

#include <tplc/tplc_sacn.h>

#include <netinet/in.h>

#include <cstdint>
#include <unordered_map>
#include <vector>

namespace tplc {

class SacnService {
public:
    using ClockFn = uint64_t (*)(void*);

    explicit SacnService(ClockFn clock = nullptr, void* clock_ctx = nullptr);

    const TplcSacnService& table() const { return svc_; }

private:
    struct Src {
        int fd = -1;
        TplcSacnSrcReq req{};
        uint8_t dmx[TPLC_SACN_MAX_DMX]{};
        uint16_t dmx_len = 0;
        uint8_t start_code = 0;
        bool have_frame = false;
        uint64_t last_send_ns = 0;
        uint64_t sent = 0;
        uint64_t aux_sent = 0;
        uint32_t seq = 0;
        uint32_t sync_seq = 0;
        int32_t last_err = 0;
    };

    struct Rcv {
        int fd = -1;
        TplcSacnRcvReq req{};
        struct sockaddr_in peer{};
        bool have_peer = false;
        /* latest accepted data packet */
        bool have_data = false;
        uint8_t preview = 0, stream_term = 0, force_sync = 0;
        uint8_t start_code = 0, seq = 0, priority = 0;
        uint16_t sync_addr = 0, universe = 0, dmx_len = 0;
        uint8_t remote_ip[4]{};
        char source_name[TPLC_SACN_NAME_LEN]{};
        uint8_t dmx[TPLC_SACN_MAX_DMX]{};
        uint64_t data_rx = 0, data_dropped = 0;
        /* latest sync packet */
        bool have_sync = false;
        uint8_t sync_seq = 0;
        uint16_t sync_addr_seen = 0;
        uint64_t sync_rx = 0;
        /* latest discovery packet */
        bool have_discovery = false;
        uint16_t uni_count = 0, uni_page = 0, uni_last_page = 0;
        uint64_t disc_rx = 0;
        uint16_t universes[TPLC_SACN_UNI_WINDOW]{};
    };

    int32_t srcBegin(void* s, const TplcSacnSrcReq* req,
                     TplcSacnHandle* out);
    int32_t srcPoll(void* s, TplcSacnHandle h, const TplcSacnFrame* frame,
                    TplcSacnSrcInfo* info);
    int32_t srcClose(void* s, TplcSacnHandle h);
    int32_t rcvBegin(void* s, const TplcSacnRcvReq* req,
                     TplcSacnHandle* out);
    int32_t rcvPoll(void* s, TplcSacnHandle h, TplcSacnRcvInfo* info,
                    uint8_t* dmx, uint16_t dmx_cap,
                    uint16_t* unis, uint16_t unis_cap,
                    char* name, uint32_t name_cap);
    int32_t rcvClose(void* s, TplcSacnHandle h);

    uint64_t nowNs() const;
    Src* findSrc(uint32_t h);
    Rcv* findRcv(uint32_t h);
    void fillSrcInfo(const Src& t, TplcSacnSrcInfo* info) const;

    /* E1.31 codec */
    static void put16(std::vector<uint8_t>& v, uint16_t x);
    static void put32(std::vector<uint8_t>& v, uint32_t x);
    static uint16_t get16be(const uint8_t* p);
    static uint32_t get32be(const uint8_t* p);
    static void flagsLen(std::vector<uint8_t>& v, size_t off,
                         size_t total);
    static void rootHdr(std::vector<uint8_t>& v, uint32_t vector,
                        const uint8_t cid[16], size_t total);
    static std::vector<uint8_t> buildData(const Src& s);
    static std::vector<uint8_t> buildSync(const Src& s);
    static std::vector<uint8_t> buildDiscovery(const Src& s);
    static void mcastAddr(uint32_t universe, uint8_t ip[4]);
    static bool parseData(const uint8_t* p, size_t n, Rcv& r);
    static bool parseSync(const uint8_t* p, size_t n, Rcv& r);
    static bool parseDiscovery(const uint8_t* p, size_t n, Rcv& r);

    TplcSacnService svc_{};
    std::unordered_map<uint32_t, Src> srcs_;
    std::unordered_map<uint32_t, Rcv> rcvs_;
    uint32_t next_src_ = 1;
    uint32_t next_rcv_ = 1;
    ClockFn clock_;
    void* clock_ctx_;
};

} /* namespace tplc */

#endif /* TPLC_HOST_SACN_SERVICE_HPP */
