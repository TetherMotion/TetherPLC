/* CtlServer.hpp — control-channel server (plan doc 04 App I, P0 slice).
 * Nonblocking UDS listener + per-client frame state machine; ops are
 * dispatched to registered handlers. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_CTL_SERVER_HPP
#define TPLC_HOST_CTL_SERVER_HPP

#include <tplc/tplc_ctl.h>
#include <tplc/host/Status.hpp>

#include <cstdint>
#include <cstring>
#include <functional>
#include <string>
#include <unordered_map>
#include <vector>

namespace tplc {

class CtlServer {
public:
    /* Handler: (client_fd, msg_type, payload, req_id) — reply via
       reply()/error(). Server may be re-entered (events). */
    using OpFn = std::function<void(int fd, uint16_t type,
                                  const uint8_t* payload, uint32_t len,
                                  uint64_t req_id)>;

    CtlServer() = default;
    ~CtlServer();
    CtlServer(const CtlServer&) = delete;
    CtlServer& operator=(const CtlServer&) = delete;

    Status open(const std::string& path, mode_t mode = 0660);
    void   close();
    bool   isOpen() const { return listen_fd_ >= 0; }
    const std::string& path() const { return path_; }

    void on(uint16_t msg_type, OpFn fn) { ops_[msg_type] = std::move(fn); }
    void onUnknown(OpFn fn) { unknown_ = std::move(fn); }

    /* Service pending accepts + reads. timeout_ms bounds the poll. */
    int  poll(int timeout_ms);
    size_t clientCount() const { return clients_.size(); }

    /* Send helpers (usable inside handlers; also for events) */
    void reply(int fd, uint16_t type, const void* payload,
               uint32_t len, uint64_t req_id);
    void errorReply(int fd, uint16_t type, int32_t code,
                    const char* msg, uint64_t req_id);
    void replyEmpty(int fd, uint16_t type, uint64_t req_id) {
        reply(fd, type, nullptr, 0, req_id);
    }
    /* Broadcast an event frame to clients whose notify_mask covers
       category bit `cat`. */
    void event(uint16_t type, uint32_t cat, const void* payload,
               uint32_t len);

    /* Payload writer helpers (little-endian, bounds-checked) */
    struct W {
        std::vector<uint8_t> b;
        void u8(uint8_t v)  { b.push_back(v); }
        void u16(uint16_t v){ b.push_back(v & 0xFF); b.push_back(v >> 8); }
        void u32(uint32_t v){ for (int i=0;i<4;i++) b.push_back(v>>(8*i)); }
        void u64(uint64_t v){ for (int i=0;i<8;i++) b.push_back(v>>(8*i)); }
        void i32(int32_t v) { u32((uint32_t)v); }
        void bytes(const void* p, size_t n) {
            const uint8_t* c = (const uint8_t*)p; b.insert(b.end(), c, c+n);
        }
        void str(const char* s, uint32_t n) { u16(n > 0xFFFF ? 0xFFFF : n); bytes(s, n > 0xFFFF ? 0xFFFF : n); }
        void strn(const char* s, size_t max) { str(s, (uint32_t)strnlen(s, max)); }
        const uint8_t* data() const { return b.data(); }
        uint32_t size() const { return (uint32_t)b.size(); }
    };

private:
    struct Client {
        int fd = -1;
        bool greeted = false;
        uint32_t notify_mask = 0;
        uint64_t session_id = 0;
        std::vector<uint8_t> inbuf;
        std::vector<uint8_t> outbuf;
    };

    int         listen_fd_ = -1;
    std::string path_;
    std::vector<Client> clients_;
    std::unordered_map<uint16_t, OpFn> ops_;
    OpFn        unknown_;
    uint64_t    next_session_ = 1;

    void acceptAll();
    void dropClient(size_t idx);
    void feed(Client& c);
    void dispatch(Client& c, uint16_t type, uint16_t flags,
                  const uint8_t* payload, uint32_t len, uint64_t req_id);
    void sendFrame(Client& c, uint16_t type, uint16_t flags,
                   const void* payload, uint32_t len, uint64_t req_id);
    void flush(Client& c);
};

} /* namespace tplc */
#endif /* TPLC_HOST_CTL_SERVER_HPP */
