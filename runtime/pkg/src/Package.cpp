/* Package.cpp — .tplc reader. Mirrors compiler/tplc-pkg/src/reader.rs;
 * every check here has a Rust-side counterpart and vice versa.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/pkg/Package.hpp>
#include <tplc/pkg/Sha256.hpp>

#include <cstdio>
#include <cstring>

namespace tplc {

static uint64_t alignUp(uint64_t v, uint64_t a) { return (v + a - 1) & ~(a - 1); }

static Status perr(int32_t code, const char* msg) {
    return Status::Err(code, msg);
}

/* per-type payload size limit (doc 01 §2.2); 0 = unbounded beyond file */
static uint64_t sectionLimit(uint32_t type) {
    switch (type) {
    case TPLC_SEC_MANIFEST:     return TPLC_PKG_MAX_MANIFEST;
    case TPLC_SEC_IR_BITCODE:   return TPLC_PKG_MAX_IR;
    case TPLC_SEC_TARGET_OBJ:   return TPLC_PKG_MAX_TARGET_OBJ;
    case TPLC_SEC_SYMMAP:
    case TPLC_SEC_TYPE_CATALOG: return TPLC_PKG_MAX_SYMMAP_CAT;
    default:                    return 0;
    }
}

static bool knownType(uint32_t t) {
    switch (t) {
    case TPLC_SEC_MANIFEST: case TPLC_SEC_IR_BITCODE:
    case TPLC_SEC_TARGET_OBJ: case TPLC_SEC_SYMMAP:
    case TPLC_SEC_TYPE_CATALOG: case TPLC_SEC_LAYOUT_DESC:
    case TPLC_SEC_MIGRATE_DESC: case TPLC_SEC_IO_MAP:
    case TPLC_SEC_DEPLOY_MAP:
    case TPLC_SEC_SOURCE_MANIFEST: case TPLC_SEC_PLCOPEN_XML:
    case TPLC_SEC_ST_SOURCES: case TPLC_SEC_DEBUG_LINE:
    case TPLC_SEC_SIGNATURE: case TPLC_SEC_TOOLCHAIN:
        return true;
    default:
        return false;
    }
}

Result<Package> Package::parse(const void* data, size_t size) {
    return parseImpl(static_cast<const uint8_t*>(data), size);
}

Result<Package> Package::open(const std::string& path) {
    FILE* f = std::fopen(path.c_str(), "rb");
    if (!f) return Result<Package>::Err(TPLC_E_PKG_MAGIC, "cannot open file");
    std::fseek(f, 0, SEEK_END);
    long n = std::ftell(f);
    std::fseek(f, 0, SEEK_SET);
    std::vector<uint8_t> buf;
    if (n > 0) {
        buf.resize((size_t)n);
        if (std::fread(buf.data(), 1, (size_t)n, f) != (size_t)n) {
            std::fclose(f);
            return Result<Package>::Err(TPLC_E_PKG_LIMIT, "short read");
        }
    }
    std::fclose(f);

    auto r = parseImpl(buf.data(), buf.size());
    if (!r.ok()) return r;
    r.value.owned_ = std::move(buf);
    r.value.data_  = r.value.owned_.data();
    r.value.size_  = r.value.owned_.size();
    return r;
}

Result<Package> Package::parseImpl(const uint8_t* data, size_t size) {
    if (size < TPLC_PKG_HEADER_FIXED)
        return Result<Package>::Err(TPLC_E_PKG_MAGIC, "file too small");

    Package pkg;
    std::memcpy(&pkg.hdr_, data, sizeof pkg.hdr_);

    static const uint8_t wantMagic[8] = {'T','P','L','C','P','K','G',0};
    if (std::memcmp(pkg.hdr_.magic, wantMagic, 8) != 0)
        return Result<Package>::Err(TPLC_E_PKG_MAGIC, "bad magic");
    if (pkg.hdr_.format_version > TPLC_PKG_FORMAT_VERSION)
        return Result<Package>::Err(TPLC_E_PKG_VERSION, "format too new");
    if (pkg.hdr_.total_size != size || size > TPLC_PKG_MAX_FILE)
        return Result<Package>::Err(TPLC_E_PKG_LIMIT, "size mismatch/limit");
    if (pkg.hdr_.toc_count > TPLC_PKG_MAX_TOC)
        return Result<Package>::Err(TPLC_E_PKG_TOC, "toc too large");

    /* header_hash over bytes 0..128 */
    uint8_t hh[TPLC_PKG_HASH_LEN];
    Sha256::hash(data, 128, hh);
    if (std::memcmp(hh, pkg.hdr_.header_hash, TPLC_PKG_HASH_LEN) != 0)
        return Result<Package>::Err(TPLC_E_PKG_HASH_HEADER, "header hash");

    const uint64_t wantHdr =
        alignUp(TPLC_PKG_HEADER_FIXED + (uint64_t)pkg.hdr_.toc_count * TPLC_PKG_TOC_ENTRY_SIZE,
                TPLC_PKG_SECTION_ALIGN);
    if (pkg.hdr_.header_size != wantHdr || size < pkg.hdr_.header_size)
        return Result<Package>::Err(TPLC_E_PKG_TOC, "header_size wrong");

    pkg.toc_.resize(pkg.hdr_.toc_count);
    for (uint32_t i = 0; i < pkg.hdr_.toc_count; i++) {
        std::memcpy(&pkg.toc_[i],
                    data + TPLC_PKG_HEADER_FIXED + i * TPLC_PKG_TOC_ENTRY_SIZE,
                    sizeof(TplcPkgTocEntry));
    }

    /* TOC invariants: strictly increasing type (sorted+unique), aligned,
     * non-overlapping, in-bounds, per-type limits, unknown+required reject. */
    uint64_t prevEnd = pkg.hdr_.header_size;
    uint32_t prevType = 0;
    for (uint32_t i = 0; i < pkg.hdr_.toc_count; i++) {
        const TplcPkgTocEntry& e = pkg.toc_[i];
        if (i > 0 && e.section_type <= prevType)
            return Result<Package>::Err(TPLC_E_PKG_TOC, "toc not sorted");
        prevType = e.section_type;
        if (e.offset % TPLC_PKG_SECTION_ALIGN != 0 ||
            e.offset < prevEnd || e.offset > pkg.hdr_.total_size ||
            e.size > pkg.hdr_.total_size - e.offset)
            return Result<Package>::Err(TPLC_E_PKG_TOC, "section range bad");
        uint64_t lim = sectionLimit(e.section_type);
        if (lim && e.size > lim)
            return Result<Package>::Err(TPLC_E_PKG_LIMIT, "section too big");
        if (!knownType(e.section_type) &&
            (e.section_flags & TPLC_PKG_SECF_REQUIRED))
            return Result<Package>::Err(TPLC_E_PKG_UNKNOWN_SECTION,
                                        "unknown required section");
        prevEnd = e.offset + e.size;
    }

    /* content_hash over raw TOC bytes then payloads (D-IMPL-13);
       SIGNATURE sections are excluded from both passes — the sig
       authenticates content_hash itself and cannot cover its own
       bytes (doc 15 §4.1). */
    Sha256 ch;
    for (uint32_t i = 0; i < pkg.hdr_.toc_count; i++) {
        if (pkg.toc_[i].section_type == TPLC_SEC_SIGNATURE) continue;
        ch.update(data + TPLC_PKG_HEADER_FIXED + i * TPLC_PKG_TOC_ENTRY_SIZE,
                  TPLC_PKG_TOC_ENTRY_SIZE);
    }
    for (const auto& e : pkg.toc_) {
        if (e.section_type == TPLC_SEC_SIGNATURE) continue;
        ch.update(data + e.offset, (size_t)e.size);
    }
    uint8_t chd[TPLC_PKG_HASH_LEN];
    ch.finish(chd);
    if (std::memcmp(chd, pkg.hdr_.content_hash, TPLC_PKG_HASH_LEN) != 0)
        return Result<Package>::Err(TPLC_E_PKG_HASH_CONTENT, "content hash");

    pkg.data_ = data;
    pkg.size_ = size;
    return Result<Package>::Ok(std::move(pkg));
}

std::string Package::uuidStr() const {
    size_t n = 0;
    while (n < TPLC_PKG_UUID_FIELD_LEN && hdr_.package_uuid[n]) n++;
    return std::string((const char*)hdr_.package_uuid, n);
}

bool Package::section(uint32_t type, PkgSection& out) const {
    for (const auto& e : toc_) {
        if (e.section_type == type) {
            out = PkgSection{e.section_type, e.section_flags,
                             e.section_version, e.aux,
                             data_ + e.offset, e.size};
            return true;
        }
    }
    return false;
}

void Package::sectionsOf(uint32_t type,
    const std::function<void(const PkgSection&)>& fn) const {
    for (const auto& e : toc_)
        if (e.section_type == type)
            fn(PkgSection{e.section_type, e.section_flags,
                          e.section_version, e.aux,
                          data_ + e.offset, e.size});
}

void Package::forEach(const std::function<void(const PkgSection&)>& fn) const {
    for (const auto& e : toc_)
        fn(PkgSection{e.section_type, e.section_flags,
                      e.section_version, e.aux,
                      data_ + e.offset, e.size});
}

Status Package::validateForLoad(bool irStripped) const {
    static const uint32_t req[] = {
        TPLC_SEC_MANIFEST, TPLC_SEC_SYMMAP, TPLC_SEC_TYPE_CATALOG,
        TPLC_SEC_LAYOUT_DESC
    };
    PkgSection s;
    for (uint32_t t : req)
        if (!section(t, s))
            return perr(TPLC_E_PKG_TOC, "required section missing");
    if (!irStripped && !section(TPLC_SEC_IR_BITCODE, s))
        return perr(TPLC_E_PKG_TOC, "ir_bitcode missing (not stripped)");
    if (!section(TPLC_SEC_TARGET_OBJ, s))
        return perr(TPLC_E_TARGET_MISMATCH, "no target_obj");
    return Status::Ok();
}

std::string Package::manifestJson() const {
    PkgSection s;
    if (!section(TPLC_SEC_MANIFEST, s)) return {};
    return std::string((const char*)s.data, (size_t)s.size);
}

} /* namespace tplc */
