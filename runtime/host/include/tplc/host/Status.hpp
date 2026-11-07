/* Status.hpp — host-side status/result plumbing (C++17).
 * Codes reuse the TPLC_* return-code space; host-only errors live in
 * the -300.. range. SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_STATUS_HPP
#define TPLC_HOST_STATUS_HPP

#include <cstdint>
#include <string>
#include <utility>

namespace tplc {

/* host-only error codes (not part of the ABI) */
enum HostErr : int32_t {
    TPLC_E_HOST_DLOPEN = -300,   /* dlopen failed                    */
    TPLC_E_HOST_SYM    = -301,   /* required dlsym missing           */
    TPLC_E_HOST_DESC   = -302,   /* descriptor failed validation     */
    TPLC_E_HOST_RANGE  = -303,   /* descriptor field out of range    */
    TPLC_E_HOST_STATE  = -304    /* wrong lifecycle state            */
};

struct Status {
    int32_t     code = 0;
    std::string msg;

    bool ok() const { return code == 0; }
    explicit operator bool() const { return ok(); }

    static Status Ok() { return {}; }
    static Status Err(int32_t c, std::string m) {
        Status s; s.code = c; s.msg = std::move(m); return s;
    }
};

template <class T>
struct Result {
    Status st;
    T      value{};

    bool ok() const { return st.ok(); }
    explicit operator bool() const { return ok(); }

    static Result Ok(T v)   { Result r; r.value = std::move(v); return r; }
    static Result Err(Status s) { Result r; r.st = std::move(s); return r; }
    static Result Err(int32_t c, std::string m) {
        return Err(Status::Err(c, std::move(m)));
    }
};

} /* namespace tplc */
#endif /* TPLC_HOST_STATUS_HPP */
