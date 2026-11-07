#include <tplc/host/RetainStore.hpp>
#include <tplc/tplc_abi.h>
#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

namespace tplc {

/* crc32 (zlib poly) — small table-free impl, retain payloads are KBs */
static uint32_t crc32_of(const uint8_t* p, size_t n) {
    uint32_t c = 0xFFFFFFFFu;
    for (size_t i = 0; i < n; i++) {
        c ^= p[i];
        for (int k = 0; k < 8; k++)
            c = (c >> 1) ^ (0xEDB88320u & (uint32_t)-(int32_t)(c & 1));
    }
    return ~c;
}

static void uuid_hex(const uint8_t uuid[16], char out[33]) {
    static const char* H = "0123456789abcdef";
    for (int i = 0; i < 16; i++) {
        out[i * 2] = H[uuid[i] >> 4];
        out[i * 2 + 1] = H[uuid[i] & 0xF];
    }
    out[32] = 0;
}

static bool read_full(int fd, void* dst, size_t n, off_t off) {
    uint8_t* p = (uint8_t*)dst;
    while (n) {
        ssize_t r = ::pread(fd, p, n, off);
        if (r <= 0) return false;
        p += r;
        n -= (size_t)r;
        off += r;
    }
    return true;
}

static bool write_full(int fd, const void* src, size_t n, off_t off) {
    const uint8_t* p = (const uint8_t*)src;
    while (n) {
        ssize_t r = ::pwrite(fd, p, n, off);
        if (r <= 0) return false;
        p += r;
        n -= (size_t)r;
        off += r;
    }
    return true;
}

RetainStore::~RetainStore() {
    if (fd_ >= 0) ::close(fd_);
}

Status RetainStore::open(const std::string& dir,
                         const uint8_t program_uuid[16],
                         uint64_t retain_layout_hash, uint64_t len) {
    if (len == 0) return Status::Ok();     /* nothing to persist */
    len_ = len;
    layout_hash_ = retain_layout_hash;
    memcpy(uuid_, program_uuid, 16);
    staging_.resize(len);

    char hex[33];
    uuid_hex(program_uuid, hex);
    ::mkdir(dir.c_str(), 0755);
    path_ = dir + "/" + hex + ".bin";
    fd_ = ::open(path_.c_str(), O_RDWR | O_CREAT, 0644);
    if (fd_ < 0)
        return Status::Err(TPLC_E_INIT_RESOURCE, "retain open failed");
    if (::ftruncate(fd_, (off_t)(sizeof(TplcRetainHeader) + len)) != 0) {
        ::close(fd_);
        fd_ = -1;
        return Status::Err(TPLC_E_INIT_RESOURCE, "retain resize failed");
    }
    return Status::Ok();
}

bool RetainStore::load(uint8_t* dst) {
    if (fd_ < 0) return false;
    TplcRetainHeader h;
    if (!read_full(fd_, &h, sizeof h, 0) ||
        memcmp(h.magic, "TRET", 4) != 0 || h.version != 1 ||
        memcmp(h.program_uuid, uuid_, 16) != 0 || h.len != len_ ||
        h.retain_layout_hash != layout_hash_ ||
        h.header_crc32 != crc32_of((const uint8_t*)&h, 52)) {
        losses_++;
        return false;
    }
    std::vector<uint8_t> buf(len_);
    if (!read_full(fd_, buf.data(), len_, (off_t)sizeof h) ||
        h.payload_crc32 != crc32_of(buf.data(), len_)) {
        losses_++;
        return false;
    }
    memcpy(dst, buf.data(), len_);
    generation_ = h.generation;
    restores_++;
    return true;
}

void RetainStore::stage(const uint8_t* region) {
    if (fd_ < 0 || staging_.empty()) return;
    memcpy(staging_.data(), region, len_);
}

Status RetainStore::flush() {
    if (fd_ < 0) return Status::Ok();
    TplcRetainHeader h;
    memcpy(h.magic, "TRET", 4);
    h.version = 1;
    memcpy(h.program_uuid, uuid_, 16);
    h.retain_layout_hash = layout_hash_;
    h.generation = ++generation_;
    h.len = len_;
    h.payload_crc32 = crc32_of(staging_.data(), len_);
    h.header_crc32 = 0;
    memset(h.pad, 0, sizeof h.pad);
    h.header_crc32 = crc32_of((const uint8_t*)&h, 52);
    if (!write_full(fd_, &h, sizeof h, 0) ||
        !write_full(fd_, staging_.data(), len_, (off_t)sizeof h))
        return Status::Err(TPLC_E_INIT_RESOURCE, "retain write failed");
    if (::fdatasync(fd_) != 0)
        return Status::Err(TPLC_E_INIT_RESOURCE, "retain fsync failed");
    dirty_ = false;
    flushes_++;
    return Status::Ok();
}

Status RetainStore::clear() {
    if (fd_ < 0) return Status::Ok();
    memset(staging_.data(), 0, len_);
    return flush();
}

} /* namespace tplc */
