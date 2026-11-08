/* ElfScan.hpp — post-link module scanner (doc 15 §4.3).
 * Static safety scan of an ELF shared object before dlopen:
 * no syscall/sysenter/int-family opcodes at instruction boundaries,
 * no RWX segments, no executable stack, no PT_INTERP, and a
 * DT_NEEDED whitelist (libc/libm family only).
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_HOST_ELFSCAN_HPP
#define TPLC_HOST_ELFSCAN_HPP

#include <tplc/host/Status.hpp>

#include <string>

namespace tplc {

/* Returns ok when the file passes all scanner checks, else
 * TPLC_E_PKG_SYSCALL (or TPLC_E_PKG_* for malformed input). */
Status scanModuleFile(const std::string& path);

} /* namespace tplc */
#endif /* TPLC_HOST_ELFSCAN_HPP */
