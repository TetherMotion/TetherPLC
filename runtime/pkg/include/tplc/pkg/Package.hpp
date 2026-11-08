/* Package.hpp — C++ reader for the .tplc container (doc 01 §2).
 * Zero-copy parse over a caller-owned buffer; verifies header_hash and
 * content_hash (TOC + payloads, D-IMPL-13) and every structural invariant
 * before any section is handed out.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_PKG_PACKAGE_HPP
#define TPLC_PKG_PACKAGE_HPP

#include <tplc/tplc_pkg.h>
#include <tplc/host/Status.hpp>

#include <cstddef>
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

namespace tplc {

struct PkgSection {
    uint32_t         type;
    uint32_t         flags;
    uint32_t         sectionVersion;
    uint32_t         aux;
    const uint8_t*   data;
    uint64_t         size;

    bool required()   const { return (flags & TPLC_PKG_SECF_REQUIRED) != 0; }
    bool compressed() const { return (flags & TPLC_PKG_SECF_COMPRESSED) != 0; }
};

class Package {
public:
    Package() = default; /* empty package; use parse()/open() for real use */

    /* Parse + fully verify a buffer containing a .tplc file image.
     * The buffer must outlive the Package (sections point into it). */
    static Result<Package> parse(const void* data, size_t size);

    /* Load a file into memory and parse. */
    static Result<Package> open(const std::string& path);

    const TplcPkgHeader& header() const { return hdr_; }
    std::string uuidStr() const;

    /* TOC entries in canonical order. */
    const std::vector<TplcPkgTocEntry>& toc() const { return toc_; }

    /* Section access. */
    bool section(uint32_t type, PkgSection& out) const;
    void sectionsOf(uint32_t type,
                    const std::function<void(const PkgSection&)>& fn) const;
    void forEach(const std::function<void(const PkgSection&)>& fn) const;

    /* doc 01 §2.1 required-for-load set (format level only). */
    Status validateForLoad(bool irStripped) const;

    /* doc 15 §4.1 signature policy check. policy:
     *   "open"            — unsigned passes (loud warn via caller),
     *                       signed packages are verified
     *   "require"         — valid ed25519 signature required
     *   "require-trusted" — valid signature AND signer key_id must
     *                       exist in trustDir as <hex-keyid>.pub
     * Errors: TPLC_E_SIG_REQUIRED / _INVALID / _UNTRUSTED. */
    Status verifySignature(const std::string& policy,
                           const std::string& trustDir) const;

    std::string manifestJson() const;

    /* The backing buffer, if the Package owns it (file-opened). */
    const std::vector<uint8_t>& buffer() const { return owned_; }

private:
    static Result<Package> parseImpl(const uint8_t* data, size_t size);

    TplcPkgHeader                  hdr_{};
    std::vector<TplcPkgTocEntry>   toc_;
    const uint8_t*                 data_ = nullptr;
    size_t                         size_ = 0;
    std::vector<uint8_t>           owned_; /* open()-loaded bytes */
};

} /* namespace tplc */
#endif /* TPLC_PKG_PACKAGE_HPP */
