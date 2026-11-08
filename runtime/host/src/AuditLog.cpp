/* AuditLog.cpp — append-only audit journal sink (doc 15 §5).
 * Format per line:
 *   ts=<ns> uid=<u> gid=<g> pid=<p> sess=<s> name=<n> op=<op>
 *   args="<detail>" ahash=<fnv64-hex> rc=<r>
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#include <tplc/host/AuditLog.hpp>

#include <fcntl.h>
#include <unistd.h>
#include <cerrno>
#include <cstdio>
#include <cstring>
#include <ctime>

namespace tplc {

AuditLog::~AuditLog() { close(); }

Status AuditLog::open(const std::string& path) {
    close();
    int fd = ::open(path.c_str(), O_WRONLY | O_APPEND | O_CREAT | O_CLOEXEC,
                    0640);
    if (fd < 0)
        return Status::Err(TPLC_E_HOST_DLOPEN,
                           "audit log open: " + std::string(strerror(errno)));
    fd_ = fd;
    return Status::Ok();
}

void AuditLog::close() {
    if (fd_ >= 0) ::close(fd_);
    fd_ = -1;
}

uint64_t AuditLog::argsHash(const std::string& s) {
    uint64_t h = 1469598103934665603ull;      /* FNV-1a offset basis */
    for (uint8_t c : s) { h ^= c; h *= 1099511628211ull; }
    return h;
}

void AuditLog::record(uint32_t pid, uint32_t uid, uint32_t gid,
                      uint64_t session, const char* name,
                      const char* op, const std::string& detail,
                      int32_t rc) {
    if (fd_ < 0) return;
    /* peer-controlled text must not forge newlines or quotes */
    std::string d = detail, nm = (name && *name) ? name : "-";
    for (char& c : d)  if (c == '"' || c == '\n' || c == '\r') c = '\'';
    for (char& c : nm) if (c == ' ' || c == '\n' || c == '\r') c = '_';
    timespec ts{};
    clock_gettime(CLOCK_REALTIME, &ts);
    char buf[2048];
    int n = snprintf(buf, sizeof buf,
        "ts=%lld%09ld uid=%u gid=%u pid=%u sess=%llu name=%s op=%s "
        "args=\"%s\" ahash=%016llx rc=%d\n",
        (long long)ts.tv_sec, ts.tv_nsec, uid, gid, pid,
        (unsigned long long)session, nm.c_str(), op, d.c_str(),
        (unsigned long long)argsHash(detail), rc);
    if (n <= 0) return;
    if (n >= (int)sizeof buf) n = (int)sizeof buf - 1;
    /* single write() keeps concurrent lines atomic on O_APPEND */
    ssize_t w = write(fd_, buf, (size_t)n);
    (void)w;
    fsync(fd_);
}

} /* namespace tplc */
