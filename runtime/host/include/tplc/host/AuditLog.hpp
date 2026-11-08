/* AuditLog.hpp — durable append-only audit journal (doc 15 §5).
 * One fsync'd line per mutating op: timestamp, peer identity, op,
 * argument hash, result. Survives restart by construction (append
 * mode). SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_AUDIT_LOG_HPP
#define TPLC_HOST_AUDIT_LOG_HPP

#include <tplc/host/Status.hpp>
#include <cstdint>
#include <string>

namespace tplc {

class AuditLog {
public:
    AuditLog() = default;
    ~AuditLog();
    AuditLog(const AuditLog&) = delete;
    AuditLog& operator=(const AuditLog&) = delete;

    Status open(const std::string& path);   /* O_APPEND|O_CREAT 0640 */
    void   close();
    bool   isOpen() const { return fd_ >= 0; }

    /* One record; fsync per write (mutating ops are rare — durability
       beats throughput). */
    void record(uint32_t pid, uint32_t uid, uint32_t gid,
                uint64_t session, const char* name,
                const char* op, const std::string& detail,
                int32_t rc);

    /* FNV-1a over the rendered argument string — tamper-evident
       reference for the journal, not a crypto commitment. */
    static uint64_t argsHash(const std::string& s);

private:
    int fd_ = -1;
};

} /* namespace tplc */
#endif /* TPLC_HOST_AUDIT_LOG_HPP */
