/* CtlClient — synchronous client for the tether-plc control channel.
 * One request/response round-trip per call(); not thread-safe, so the
 * provider serializes calls behind a mutex (io sessions may invoke
 * read/write functions concurrently).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#pragma once

#include <tplc/tplc_ctl.h>

#include <cstdint>
#include <cstring>
#include <mutex>
#include <string>
#include <vector>

#include <errno.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

namespace tplcio {

class CtlClient {
public:
    struct Error {
        int32_t     code = 0;   /* TPLC_E_* (negative) or 0         */
        uint32_t    subcode = 0;
        std::string msg;
    };

    ~CtlClient() { disconnect(); }

    bool connectSock(const std::string& path, int timeout_ms,
                     const std::string& name = "tplc-io") {
        disconnect();
        int fd = socket(AF_UNIX, SOCK_STREAM, 0);
        if (fd < 0) return false;
        sockaddr_un addr{};
        addr.sun_family = AF_UNIX;
        strncpy(addr.sun_path, path.c_str(), sizeof addr.sun_path - 1);
        long waited = 0;
        for (;;) {
            if (::connect(fd, (sockaddr*)&addr, sizeof addr) == 0)
                break;
            if (errno != ENOENT && errno != ECONNREFUSED) {
                close(fd);
                return false;
            }
            if (waited >= timeout_ms) { close(fd); return false; }
            usleep(20 * 1000);
            waited += 20;
        }
        fd_ = fd;
        /* bound every recv: a lost reply must not wedge the caller
           (and every session queued behind the ctl mutex) forever —
           the watchdog reconnects within one poll tick            */
        timeval tv{2, 0};
        setsockopt(fd_, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof tv);
        return hello(name);
    }

    void disconnect() {
        if (fd_ >= 0) { close(fd_); fd_ = -1; }
    }

    bool connected() const { return fd_ >= 0; }

    /* op round-trip: returns true on a non-error reply. err->code is
       the ctl error (negative) when false; also false on transport
       failure (code==0, msg="transport").                          */
    bool call(uint16_t op, const void* payload, uint32_t len,
              std::vector<uint8_t>& out, Error* err = nullptr) {
        std::lock_guard<std::mutex> lk(mu_);
        return callLocked(op, payload, len, out, err);
    }

    bool call(uint16_t op, std::vector<uint8_t>& out,
              Error* err = nullptr) {
        return call(op, nullptr, 0, out, err);
    }

    /* convenience: send a LE scalar payload */
    template <typename T>
    static std::vector<uint8_t> pack(T v) {
        std::vector<uint8_t> b(sizeof v);
        memcpy(b.data(), &v, sizeof v);
        return b;
    }

private:
    bool hello(const std::string& name) {
        uint8_t buf[256];
        TplcCtlHello h{};
        h.proto_ver = TPLC_CTL_PROTO_VER;
        memcpy(buf, &h, sizeof h);
        uint16_t nl = (uint16_t)name.size();
        memcpy(buf + sizeof h, &nl, 2);
        memcpy(buf + sizeof h + 2, name.data(), nl);
        std::vector<uint8_t> p(buf, buf + sizeof h + 2 + nl);
        std::vector<uint8_t> out;
        if (!sendFrame(TPLC_OP_HELLO, p.data(), (uint32_t)p.size()))
            return false;
        uint16_t type, flags;
        if (!recvFrame(out, type, flags) || type != TPLC_OP_HELLO_ACK)
            return false;
        if (out.size() >= sizeof(TplcCtlHelloAck)) {
            const TplcCtlHelloAck* a =
                (const TplcCtlHelloAck*)out.data();
            authz_role_ = a->authz;
        }
        return true;
    }

    bool callLocked(uint16_t op, const void* payload, uint32_t len,
                    std::vector<uint8_t>& out, Error* err) {
        if (fd_ < 0) { setErr(err, 0, "not connected"); return false; }
        if (!sendFrame(op, payload, len)) {
            setErr(err, 0, "transport"); disconnect(); return false;
        }
        uint16_t type, flags;
        for (;;) {
            if (!recvFrame(out, type, flags)) {
                setErr(err, 0, "transport"); disconnect();
                return false;
            }
            if (!(flags & TPLC_CTL_F_EVENT)) break;   /* skip events */
        }
        if (flags & TPLC_CTL_F_ERROR) {
            int32_t code = -1; uint32_t sub = 0; std::string m;
            if (out.size() >= 8) {
                memcpy(&code, out.data(), 4);
                memcpy(&sub, out.data() + 4, 4);
                if (out.size() >= 10) {
                    uint16_t ml;
                    memcpy(&ml, out.data() + 8, 2);
                    size_t n = ml < out.size() - 10 ? ml
                                                    : out.size() - 10;
                    m.assign((const char*)out.data() + 10, n);
                }
            }
            if (err) { err->code = code; err->subcode = sub;
                       err->msg = m; }
            return false;
        }
        return true;
    }

    static void setErr(Error* e, int32_t c, const char* m) {
        if (e) { e->code = c; e->msg = m; }
    }

    bool sendFrame(uint16_t type, const void* payload, uint32_t len) {
        uint8_t hdr[TPLC_CTL_HDR_SIZE];
        uint64_t req = next_req_++;
        hdr[0] = type & 0xFF; hdr[1] = type >> 8;
        hdr[2] = hdr[3] = 0;
        for (int i = 0; i < 4; i++) hdr[4 + i] = len >> (8 * i);
        for (int i = 0; i < 8; i++) hdr[8 + i] = req >> (8 * i);
        if (!sendAll(hdr, sizeof hdr)) return false;
        return !len || sendAll(payload, len);
    }

    bool recvFrame(std::vector<uint8_t>& out, uint16_t& type,
                   uint16_t& flags) {
        uint8_t hdr[TPLC_CTL_HDR_SIZE];
        if (!recvAll(hdr, sizeof hdr)) return false;
        type  = hdr[0] | (hdr[1] << 8);
        flags = hdr[2] | (hdr[3] << 8);
        uint32_t len = hdr[4] | (hdr[5] << 8) | (hdr[6] << 16) |
                       ((uint32_t)hdr[7] << 24);
        out.assign(len, 0);
        return !len || recvAll(out.data(), len);
    }

    bool sendAll(const void* p, size_t n) {
        const uint8_t* b = (const uint8_t*)p;
        while (n) {
            ssize_t w = send(fd_, b, n, MSG_NOSIGNAL);
            if (w <= 0) return false;
            b += w; n -= (size_t)w;
        }
        return true;
    }

    bool recvAll(void* p, size_t n) {
        uint8_t* b = (uint8_t*)p;
        while (n) {
            ssize_t r = read(fd_, b, n);
            if (r <= 0) return false;
            b += r; n -= (size_t)r;
        }
        return true;
    }

    int           fd_ = -1;
    uint64_t      next_req_ = 1;
    uint8_t       authz_role_ = 0;
    std::mutex    mu_;
};

} /* namespace tplcio */
