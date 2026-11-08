/* Sandbox.hpp — tether-plc process containment (doc 15 §3).
 * installSandbox() applies resource limits and (when libseccomp is
 * available) a syscall whitelist: steady-state scan/ctl/file-io
 * syscalls plus AF_UNIX-only socket(); everything else kills the
 * process (SIGSYS). Default-on in tether-plc; --no-sandbox is the
 * journaled debug escape. SPDX-License-Identifier: Apache-2.0 OR
 * MPL-2.0 */
#ifndef TPLC_HOST_SANDBOX_HPP
#define TPLC_HOST_SANDBOX_HPP

#include <tplc/host/Status.hpp>

namespace tplc {

/* Apply rlimits + seccomp filter. Call once, early in main (the
 * whitelist keeps dlopen/file/ctl paths working). Without
 * libseccomp (TPLC_HAVE_SECCOMP undefined) only rlimits apply and a
 * warning is emitted — containment is degraded, never silently
 * absent. */
Status installSandbox();

} /* namespace tplc */
#endif /* TPLC_HOST_SANDBOX_HPP */
