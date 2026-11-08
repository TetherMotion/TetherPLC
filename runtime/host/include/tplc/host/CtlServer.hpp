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

/* control-plane roles (doc 15 §5): observer < engineer <
   commissioner. Numeric order is the comparison. */
enum CtlRole : uint8_t {
    TPLC_ROLE_NONE         = 0,
    TPLC_ROLE_OBSERVER     = 1,
    TPLC_ROLE_ENGINEER     = 2,
    TPLC_ROLE_COMMISSIONER = 3,
};

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

    /* authZ wiring (doc 15 §5): role_for_uid maps a peer uid → role;
       min_role_for_op gives the floor per op (unknown ops default to
       commissioner). Unset → everyone commissioner (pre-authz
       behaviour). */
    void setRoleResolver(std::function<int(uint32_t uid)> fn) {
        role_for_uid_ = std::move(fn);
    }
    void setOpMinRole(std::function<int(uint16_t type)> fn) {
        min_role_for_op_ = std::move(fn);
    }
    /* mutating-op rate limit, tokens/s + burst (0 disables) */
    void setMutRate(double per_sec, double burst) {
        mut_rate_ = per_sec; mut_burst_ = burst;
    }
    /* fired when an op is denied at the gate (authz / rate) — the
       runtime journals the denial with the real error code */
    void setDenyHook(std::function<void(int fd, uint16_t type,
                                      int32_t code)> fn) {
        deny_ = std::move(fn);
    }

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

    /* Identity of a client for audit lines; false = fd unknown. */
    bool clientInfo(int fd, uint64_t* session,
                    std::string* name) const;
    /* UDS peer credentials (SO_PEERCRED at accept) for audit/authZ. */
    bool peerCred(int fd, uint32_t* pid, uint32_t* uid,
                  uint32_t* gid) const;

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
        std::string name;
        /* SO_PEERCRED — kernel-attested, unforgeable by the client */
        uint32_t pid = 0, uid = 0, gid = 0;
        uint8_t  role = TPLC_ROLE_NONE;      /* resolved at HELLO     */
        std::vector<uint8_t> inbuf;
        std::vector<uint8_t> outbuf;
    };

    int         listen_fd_ = -1;
    std::string path_;
    std::vector<Client> clients_;
    std::unordered_map<uint16_t, OpFn> ops_;
    OpFn        unknown_;
    uint64_t    next_session_ = 1;
    std::function<int(uint32_t)> role_for_uid_;
    std::function<int(uint16_t)> min_role_for_op_;
    std::function<void(int, uint16_t, int32_t)> deny_;
    double      mut_rate_ = 0, mut_burst_ = 0;
    /* mutating-op token buckets keyed by kernel-attested uid —
       reconnecting does not reset the budget (doc 15 §5)          */
    struct Bucket { double tokens; uint64_t refill_ns; };
    std::unordered_map<uint32_t, Bucket> mut_buckets_;

    void acceptAll();
    void dropClient(size_t idx);
    void fill(Client& c);          /* recv into inbuf                */
    bool drainOne(Client& c);      /* dispatch one buffered frame    */
    bool hasFrame(const Client& c) const;
    void dispatch(Client& c, uint16_t type, uint16_t flags,
                  const uint8_t* payload, uint32_t len, uint64_t req_id);
    void sendFrame(Client& c, uint16_t type, uint16_t flags,
                   const void* payload, uint32_t len, uint64_t req_id);
    void flush(Client& c);
};

} /* namespace tplc */
#endif /* TPLC_HOST_CTL_SERVER_HPP */
