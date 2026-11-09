/* HiddenState.hpp — internal: target_obj hidden-state check.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_PKG_HIDDENSTATE_HPP
#define TPLC_PKG_HIDDENSTATE_HPP

#include <tplc/host/Status.hpp>

#include <cstddef>
#include <cstdint>

namespace tplc {

/* Verifies the package's target_obj keeps program mutable state out
 * of the ELF image: every defined, sized symbol landing in a writable
 * allocated section (.data/.bss family) must be a known runtime/crt
 * symbol. Generated modules place 100% of program state in the
 * runtime-allocated state blob, so anything else is hidden state that
 * retain/migration cannot see. Returns TPLC_E_PKG_HIDDEN_STATE on a
 * violation; unparseable or non-ELF input passes (other layers own
 * malformed-object errors). */
Status pkgCheckHiddenState(const uint8_t* obj, size_t size);

} /* namespace tplc */
#endif /* TPLC_PKG_HIDDENSTATE_HPP */
