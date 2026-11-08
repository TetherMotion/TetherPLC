/* CtlServer.cpp — UDS control channel (plan doc 04 App I).
 * Nonblocking throughout: partial frames accumulate per client; the RT
 * loop polls with a bounded timeout. SPDX-License-Identifier:
 * Apache-2.0 OR MPL-2.0 */

#include <tplc/host/CtlServer.hpp>

#include <cerrno>
#include <ctime>
#include <fcntl.h>
#include <poll.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/un.h>
#include <unistd.h>

namespace tplc {

CtlServer::~CtlServer() { close(); }

Status CtlServer::open(const std::string& path, mode_t mode) {
    close();
    int fd = socket(AF_UNIX, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if (fd < 0) return Status::Err(TPLC_E_CTL_PROTO, strerror(errno));

    sockaddr_un addr{};
    addr.sun_family = AF_UNIX;
    if (path.size() >= sizeof addr.sun_path) {
        ::close(fd);
        return Status::Err(TPLC_E_CTL_PROTO, "socket path too long");
    }
    strncpy(addr.sun_path, path.c_str(), sizeof addr.sun_path - 1);
    ::unlink(path.c_str());                 /* reclaim stale socket    */
    mode_t old = umask(0);
    if (bind(fd, (sockaddr*)&addr, sizeof addr) != 0) {
        Status st = Status::Err(TPLC_E_CTL_PROTO, strerror(errno));
        umask(old); ::close(fd);
        return st;
    }
    umask(old);
    chmod(path.c_str(), mode);
    if (listen(fd, 8) != 0) {
        Status st = Status::Err(TPLC_E_CTL_PROTO, strerror(errno));
        ::close(fd); ::unlink(path.c_str());
        return st;
    }
    listen_fd_ = fd;
    path_ = path;
    return Status::Ok();
}

void CtlServer::close() {
    for (auto& c : clients_) ::close(c.fd);
    clients_.clear();
    if (listen_fd_ >= 0) ::close(listen_fd_);
    listen_fd_ = -1;
    if (!path_.empty()) { ::unlink(path_.c_str()); path_.clear(); }
}

void CtlServer::acceptAll() {
    for (;;) {
        int c = accept4(listen_fd_, nullptr, nullptr,
                        SOCK_NONBLOCK | SOCK_CLOEXEC);
        if (c < 0) return;
        Client cl;
        cl.fd = c;
        struct ucred uc{};
        socklen_t ul = sizeof uc;
        if (getsockopt(c, SOL_SOCKET, SO_PEERCRED, &uc, &ul) == 0) {
            cl.pid = (uint32_t)uc.pid;
            cl.uid = uc.uid; cl.gid = uc.gid;
        }
        clients_.push_back(std::move(cl));
    }
}

void CtlServer::dropClient(size_t idx) {
    ::close(clients_[idx].fd);
    clients_.erase(clients_.begin() + (long)idx);
}

void CtlServer::sendFrame(Client& c, uint16_t type, uint16_t flags,
                        const void* payload, uint32_t len,
                        uint64_t req_id) {
    uint8_t hdr[TPLC_CTL_HDR_SIZE];
    hdr[0] = type & 0xFF; hdr[1] = type >> 8;
    hdr[2] = flags & 0xFF; hdr[3] = flags >> 8;
    for (int i = 0; i < 4; i++) hdr[4 + i] = len >> (8 * i);
    for (int i = 0; i < 8; i++) hdr[8 + i] = req_id >> (8 * i);
    c.outbuf.insert(c.outbuf.end(), hdr, hdr + sizeof hdr);
    if (len)
        c.outbuf.insert(c.outbuf.end(),
                        (const uint8_t*)payload,
                        (const uint8_t*)payload + len);
    flush(c);
}

void CtlServer::flush(Client& c) {
    while (!c.outbuf.empty()) {
        ssize_t n = send(c.fd, c.outbuf.data(), c.outbuf.size(),
                         MSG_NOSIGNAL);
        if (n < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) return;
            c.outbuf.clear();
            return;    /* poll loop drops the client on next HUP */
        }
        c.outbuf.erase(c.outbuf.begin(), c.outbuf.begin() + n);
    }
}

void CtlServer::reply(int fd, uint16_t type, const void* payload,
                      uint32_t len, uint64_t req_id) {
    for (auto& c : clients_)
        if (c.fd == fd) { sendFrame(c, type, 0, payload, len, req_id); return; }
}

void CtlServer::errorReply(int fd, uint16_t type, int32_t code,
                           const char* msg, uint64_t req_id) {
    W w;
    w.u32((uint32_t)-code);         /* wire: positive magnitude */
    w.u32(0);
    w.str(msg, tplc_ctl_str_len(msg));
    for (auto& c : clients_)
        if (c.fd == fd) {
            sendFrame(c, type, TPLC_CTL_F_ERROR, w.data(), w.size(),
                      req_id);
            return;
        }
}

void CtlServer::event(uint16_t type, uint32_t cat, const void* payload,
                      uint32_t len) {
    for (auto& c : clients_)
        if (c.greeted && (c.notify_mask & cat))
            sendFrame(c, type, TPLC_CTL_F_EVENT, payload, len, 0);
}

void CtlServer::dispatch(Client& c, uint16_t type, uint16_t flags,
                         const uint8_t* payload, uint32_t len,
                         uint64_t req_id) {
    (void)flags;
    /* session bootstrap ops are built-in (hello gate before others) */
    if (type == TPLC_OP_HELLO) {
        if (len < sizeof(TplcCtlHello)) {
            errorReply(c.fd, type, TPLC_E_CTL_PAYLOAD, "short hello",
                       req_id);
            return;
        }
        const TplcCtlHello* h = (const TplcCtlHello*)payload;
        c.greeted = true;
        c.session_id = next_session_++;
        /* trailing `str client_name` identifies the session in audit
           lines (doc 12 §8)                                        */
        if (len >= sizeof(TplcCtlHello) + 2) {
            const uint8_t* np = payload + sizeof(TplcCtlHello);
            uint16_t nl = (uint16_t)(np[0] | (np[1] << 8));
            if (nl > 0 && sizeof(TplcCtlHello) + 2 + nl <= len) {
                c.name.assign((const char*)np + 2, nl);
                if (c.name.size() > 48) c.name.resize(48);
            }
        }
        /* resolve the client's role once, from kernel-attested creds;
           unresolvable → observer (read-only fail-safe)              */
        c.role = (uint8_t)(role_for_uid_
               ? role_for_uid_(c.uid)
               : (int)TPLC_ROLE_COMMISSIONER);
        if (c.role == TPLC_ROLE_NONE) c.role = TPLC_ROLE_OBSERVER;
        TplcCtlHelloAck ack{};
        ack.proto_ver = h->proto_ver < TPLC_CTL_PROTO_VER
                          ? h->proto_ver : TPLC_CTL_PROTO_VER;
        ack.runtime_ver = TPLC_CTL_PROTO_VER;
        ack.caps = 0;
        ack.session_id = c.session_id;
        ack.authz = c.role;
        sendFrame(c, TPLC_OP_HELLO_ACK, 0, &ack, sizeof ack, req_id);
        return;
    }
    if (!c.greeted) {
        errorReply(c.fd, type, TPLC_E_CTL_STATE, "hello first", req_id);
        return;
    }
    if (type == TPLC_OP_SET_NOTIFY) {
        if (len >= 4) {
            uint32_t m;
            memcpy(&m, payload, 4);
            c.notify_mask = m;
        }
        replyEmpty(c.fd, type, req_id);
        return;
    }
    /* authZ gate (doc 15 §5): every op has a minimum role; the
       client's role was bound at HELLO from peer creds. Mutating ops
       also spend a rate-limit token. */
    int min_role = min_role_for_op_ ? min_role_for_op_(type)
                                    : TPLC_ROLE_COMMISSIONER;
    if (c.role < min_role) {
        errorReply(c.fd, type, TPLC_E_CTL_AUTHZ,
                   "insufficient role", req_id);
        if (deny_) deny_(c.fd, type, TPLC_E_CTL_AUTHZ);
        return;
    }
    if (min_role >= TPLC_ROLE_ENGINEER && mut_rate_ > 0) {
        timespec ts{}; clock_gettime(CLOCK_MONOTONIC, &ts);
        uint64_t now = (uint64_t)ts.tv_sec * 1000000000ull + ts.tv_nsec;
        Bucket& b = mut_buckets_[c.uid];
        if (!b.refill_ns) { b.refill_ns = now; b.tokens = mut_burst_; }
        double dt = (double)(now - b.refill_ns) / 1e9;
        b.tokens += dt * mut_rate_;
        if (b.tokens > mut_burst_) b.tokens = mut_burst_;
        b.refill_ns = now;
        if (b.tokens < 1.0) {
            errorReply(c.fd, type, TPLC_E_DBG_RATE,
                       "mutating-op rate exceeded", req_id);
            if (deny_) deny_(c.fd, type, TPLC_E_DBG_RATE);
            return;
        }
        b.tokens -= 1.0;
    }
    auto it = ops_.find(type);
    if (it != ops_.end()) { it->second(c.fd, type, payload, len, req_id); return; }
    if (unknown_) { unknown_(c.fd, type, payload, len, req_id); return; }
    errorReply(c.fd, type, TPLC_E_CTL_UNKNOWN_OP, "unknown op", req_id);
}

void CtlServer::fill(Client& c) {
    uint8_t buf[8192];
    ssize_t n = recv(c.fd, buf, sizeof buf, 0);
    if (n <= 0) return;                    /* poll() handles close     */
    c.inbuf.insert(c.inbuf.end(), buf, buf + n);
}

bool CtlServer::hasFrame(const Client& c) const {
    if (c.inbuf.size() < TPLC_CTL_HDR_SIZE) return false;
    const uint8_t* h = c.inbuf.data();
    uint32_t len = h[4] | (h[5] << 8) | (h[6] << 16) | (h[7] << 24);
    if (len > TPLC_CTL_MAX_PAYLOAD) return true;   /* error drains too */
    return c.inbuf.size() >= TPLC_CTL_HDR_SIZE + len;
}

bool CtlServer::drainOne(Client& c) {
    if (c.inbuf.size() < TPLC_CTL_HDR_SIZE) return false;
    const uint8_t* h = c.inbuf.data();
    uint16_t type  = h[0] | (h[1] << 8);
    uint16_t flags = h[2] | (h[3] << 8);
    uint32_t len   = h[4] | (h[5] << 8) | (h[6] << 16) | (h[7] << 24);
    uint64_t req   = 0;
    for (int i = 0; i < 8; i++) req |= (uint64_t)h[8 + i] << (8 * i);
    if (len > TPLC_CTL_MAX_PAYLOAD) {
        errorReply(c.fd, type, TPLC_E_CTL_PAYLOAD,
                   "payload too large", req);
        c.inbuf.clear();
        return true;
    }
    if (c.inbuf.size() < TPLC_CTL_HDR_SIZE + len) return false;
    dispatch(c, type, flags, h + TPLC_CTL_HDR_SIZE, len, req);
    c.inbuf.erase(c.inbuf.begin(),
                  c.inbuf.begin() + TPLC_CTL_HDR_SIZE + len);
    return true;
}

bool CtlServer::clientInfo(int fd, uint64_t* session,
                           std::string* name) const {
    for (auto& c : clients_)
        if (c.fd == fd) {
            if (session) *session = c.session_id;
            if (name)    *name    = c.name;
            return true;
        }
    return false;
}

bool CtlServer::peerCred(int fd, uint32_t* pid, uint32_t* uid,
                         uint32_t* gid) const {
    for (auto& c : clients_)
        if (c.fd == fd) {
            if (pid) *pid = c.pid;
            if (uid) *uid = c.uid;
            if (gid) *gid = c.gid;
            return true;
        }
    return false;
}

/* Bounded op drain (doc 12 §1): at most OPS_PER_POLL dispatches per
   poll() call so a flooding client cannot stall a task release.
   Leftover frames stay buffered and drain on later polls; the socket
   itself is the watermark queue (full → the sender blocks).       */
static const int OPS_PER_POLL = 32;

int CtlServer::poll(int timeout_ms) {
    if (listen_fd_ < 0) return -1;
    acceptAll();

    /* first drain previously buffered frames, bounded */
    int budget = OPS_PER_POLL;
    for (auto& c : clients_)
        while (budget > 0 && drainOne(c)) budget--;
    /* buffered-but-undispatched frames must not sleep — they are
       already received and must drain on the next iteration        */
    bool pending = false;
    for (auto& c : clients_)
        if (hasFrame(c)) { pending = true; break; }

    /* snapshot client fds — acceptAll/dropClient mutate clients_ */
    std::vector<pollfd> fds;
    std::vector<int>    cfds;
    cfds.reserve(clients_.size());
    for (auto& c : clients_) {
        cfds.push_back(c.fd);
        short ev = POLLIN;
        if (!c.outbuf.empty()) ev |= POLLOUT;
        fds.push_back({c.fd, ev, 0});
    }
    int r = ::poll(fds.data(), (nfds_t)fds.size(),
                   pending ? 0 : timeout_ms);
    if (r < 0) return -1;
    acceptAll();   /* new arrivals aren't in the snapshot — next poll */

    for (size_t i = 0; i < cfds.size(); i++) {
        short re = fds[i].revents;
        size_t ci;
        for (ci = 0; ci < clients_.size(); ci++)
            if (clients_[ci].fd == cfds[i]) break;
        if (ci == clients_.size()) continue;      /* already dropped  */
        Client& c = clients_[ci];
        if (re & (POLLERR | POLLHUP | POLLNVAL)) { dropClient(ci); continue; }
        if (re & POLLOUT) flush(c);
        if (re & POLLIN)  fill(c);
    }

    /* dispatch newly arrived frames, same bound; beyond 2× the max
       frame the backlog can only be abuse — send DBG_OVERLOAD with
       the head frame's identity (when parseable) and drop         */
    budget = OPS_PER_POLL;
    for (size_t i = 0; i < clients_.size();) {
        if (clients_[i].inbuf.size() >
            TPLC_CTL_MAX_PAYLOAD * 2 + TPLC_CTL_HDR_SIZE) {
            const uint8_t* h = clients_[i].inbuf.data();
            if (clients_[i].inbuf.size() >= TPLC_CTL_HDR_SIZE) {
                uint16_t type = h[0] | (h[1] << 8);
                uint64_t req  = 0;
                for (int k = 0; k < 8; k++)
                    req |= (uint64_t)h[8 + k] << (8 * k);
                errorReply(clients_[i].fd, type, TPLC_E_DBG_OVERLOAD,
                           "op shed: backlog exceeds watermark", req);
                flush(clients_[i]);
            }
            dropClient(i);
            continue;
        }
        while (budget > 0 && drainOne(clients_[i])) budget--;
        i++;
    }
    return r;
}

} /* namespace tplc */
