/* RetainStore — retain persistence per plan doc 04 §5. The retain
 * region inside the state blob is mirrored to retain/<uuid>.bin:
 * staged copies are taken at task boundaries (scan path only memcpy's
 * the staging buffer); flush() writes the staged image and fsyncs.
 * File header: {magic 'TRET', version, program_uuid,
 * retain_layout_hash, generation, len, crc32} — uuid+hash match
 * restores data and sets retain_restored; mismatch yields defaults.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#pragma once

#include <tplc/host/Status.hpp>
#include <cstdint>
#include <string>
#include <vector>

namespace tplc {

/* file header — little-endian, 64 B, packed */
#pragma pack(push, 1)
struct TplcRetainHeader {
    char     magic[4];            /* 'TRET'              */
    uint32_t version;             /* = 1                 */
    uint8_t  program_uuid[16];
    uint64_t retain_layout_hash;
    uint64_t generation;          /* +1 per flush        */
    uint64_t len;                 /* payload bytes       */
    uint32_t payload_crc32;
    uint32_t header_crc32;        /* over bytes [0,52)   */
    uint8_t  pad[8];
};
#pragma pack(pop)
static_assert(sizeof(TplcRetainHeader) == 64, "retain header must be 64 B");

class RetainStore {
public:
    RetainStore() = default;
    ~RetainStore();
    RetainStore(const RetainStore&) = delete;
    RetainStore& operator=(const RetainStore&) = delete;

    /* open/create <dir>/<uuid-hex>.bin sized for `len` payload bytes.
       len==0 → attached but inert (no file). */
    Status open(const std::string& dir, const uint8_t program_uuid[16],
                uint64_t retain_layout_hash, uint64_t len);

    /* validate file (magic/version/uuid/layout_hash/len/crc) and copy
       the payload to `dst`. Returns true only on a full match. */
    bool load(uint8_t* dst);

    void noteDirty() { dirty_ = true; }

    /* memcpy the live retain region into the staging buffer — call at
       a task boundary when dirty() or ahead of a periodic flush. */
    void stage(const uint8_t* region);

    /* staged image → file + fdatasync; bumps generation. */
    Status flush();

    /* persist zeroed payload (IEC cold reset of retain). */
    Status clear();

    /* close the file and reset — used before re-open on swap. */
    void close();

    bool     attached()    const { return fd_ >= 0; }
    bool     dirty()       const { return dirty_; }
    uint64_t len()         const { return len_; }
    uint64_t generation()  const { return generation_; }
    uint64_t flushes()     const { return flushes_; }
    uint64_t restores()    const { return restores_; }
    uint64_t losses()      const { return losses_; }
    const std::string& path() const { return path_; }

private:
    int         fd_ = -1;
    std::string path_;
    uint8_t     uuid_[16] = {};
    uint64_t    layout_hash_ = 0;
    uint64_t    len_ = 0;
    uint64_t    generation_ = 0;
    bool        dirty_ = false;
    uint64_t    flushes_ = 0;
    uint64_t    restores_ = 0;
    uint64_t    losses_ = 0;
    std::vector<uint8_t> staging_;
};

} /* namespace tplc */
