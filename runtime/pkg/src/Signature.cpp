/* Signature.cpp — ed25519 package signature policy (doc 15 §4.1).
 *
 * Signature section (0x60) payload, 132 bytes:
 *   u32 sig_version = 1 | key_id[32] | pubkey[32] | sig[64]
 * sig = ed25519.sk.sign(package content_hash); key_id = sha256(pubkey).
 * The content hash excludes the signature section (see Package.cpp).
 * Trust store: <dir>/<hex-keyid>.pub holding the raw 32-byte pubkey.
 * Verification uses libsodium when TPLC_HAVE_SODIUM is set; without a
 * crypto backend any "require*" policy fails closed.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/pkg/Package.hpp>
#include <tplc/pkg/Sha256.hpp>

#include <cctype>
#include <cstdio>
#include <cstring>
#include <vector>

#ifdef TPLC_HAVE_SODIUM
#include <sodium.h>
#endif

namespace tplc {
namespace {

std::string hex64(const uint8_t* b, size_t n) {
    static const char* h = "0123456789abcdef";
    std::string s(n * 2, '0');
    for (size_t i = 0; i < n; i++) {
        s[2 * i] = h[b[i] >> 4]; s[2 * i + 1] = h[b[i] & 15];
    }
    return s;
}

} /* namespace */

Status Package::verifySignature(const std::string& policy,
                                const std::string& trustDir) const {
    PkgSection sig{};
    bool have = section(TPLC_SEC_SIGNATURE, sig);
    bool wantTrusted = policy == "require-trusted";
    bool wantSigned = wantTrusted || policy == "require";
    if (!have) {
        if (wantSigned)
            return Status::Err(TPLC_E_SIG_REQUIRED, "unsigned package");
        return Status();                            /* open: unsigned ok */
    }
    if (sig.size != 132)
        return Status::Err(TPLC_E_SIG_INVALID, "sig section size");
    uint32_t ver;
    std::memcpy(&ver, sig.data, 4);
    if (ver != 1)
        return Status::Err(TPLC_E_SIG_INVALID, "sig version");
    const uint8_t* kid = sig.data + 4;
    const uint8_t* pk  = sig.data + 36;
    const uint8_t* sg  = sig.data + 68;

    /* key_id binding */
    uint8_t kd[32];
    {
        Sha256 s;
        s.update(pk, 32);
        s.finish(kd);
    }
    if (std::memcmp(kd, kid, 32) != 0)
        return Status::Err(TPLC_E_SIG_INVALID, "key_id mismatch");

    /* signature over signing_hash: sha256 of non-signature TOC
       entries (offset field zeroed) + non-signature payloads —
       layout-independent so appending the signature does not change
       the signed message (tplc-pkg reader::signing_hash) */
    Sha256 sh;
    for (const auto& e : toc_) {
        if (e.section_type == TPLC_SEC_SIGNATURE) continue;
        TplcPkgTocEntry z = e;
        z.offset = 0;
        sh.update(&z, sizeof z);
    }
    for (const auto& e : toc_) {
        if (e.section_type == TPLC_SEC_SIGNATURE) continue;
        PkgSection sec{};
        if (!section(e.section_type, sec) && e.size)
            return Status::Err(TPLC_E_SIG_INVALID, "section resolve");
        sh.update(sec.data, (size_t)e.size);
    }
    uint8_t shb[32];
    sh.finish(shb);
#ifdef TPLC_HAVE_SODIUM
    if (crypto_sign_verify_detached(sg, shb, 32, pk) != 0)
        return Status::Err(TPLC_E_SIG_INVALID, "signature invalid");
#else
    return Status::Err(TPLC_E_SIG_INVALID,
                       "signed package but no crypto backend");
#endif

    /* trust-store gate */
    if (wantTrusted) {
        /* revocation: <trustDir>/revoked lists hex key-ids (any
           whitespace-separated run); a revoked signer is rejected
           even while its .pub remains in the store */
        std::string rf = trustDir + "/revoked";
        if (FILE* rfp = std::fopen(rf.c_str(), "rb")) {
            std::string want = hex64(kid, 32);
            char buf[256];
            bool revoked = false;
            while (!revoked && std::fgets(buf, sizeof buf, rfp)) {
                for (char* p = buf; *p && !revoked; p++) {
                    if (!isxdigit((unsigned char)*p)) continue;
                    char id[65];
                    size_t n = 0;
                    while (n < 64 && isxdigit((unsigned char)p[n])) {
                        id[n] = p[n];
                        n++;
                    }
                    id[n] = 0;
                    p += n - 1;
                    if (n == 64 && want == id) revoked = true;
                }
            }
            std::fclose(rfp);
            if (revoked)
                return Status::Err(TPLC_E_SIG_UNTRUSTED,
                                   "signer key revoked");
        }
        std::string f = trustDir + "/" + hex64(kid, 32) + ".pub";
        FILE* fp = std::fopen(f.c_str(), "rb");
        if (!fp)
            return Status::Err(TPLC_E_SIG_UNTRUSTED,
                               "signer not in trust store");
        uint8_t tpk[32];
        size_t got = std::fread(tpk, 1, 32, fp);
        uint8_t extra;
        bool more = std::fread(&extra, 1, 1, fp) == 1;
        std::fclose(fp);
        if (got != 32 || more || std::memcmp(tpk, pk, 32) != 0)
            return Status::Err(TPLC_E_SIG_UNTRUSTED,
                               "trust store key mismatch");
    }
    return Status();
}

} /* namespace tplc */
