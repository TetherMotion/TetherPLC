# Error-code coverage

Snapshot of the `TPLC_E_*` / `TPLC_TASK_E_*` space: which codes the
runtime emits and which the tests exercise. Regenerate with
`tools/error_code_report.py` (`--gaps` for the uncovered list,
`--json` for machine-readable output).

## Method and caveats

- Codes are enumerated from `include/tplc/tplc_abi.h`,
  `tplc_ctl.h`, `tplc_image.h`, `tplc_pkg.h`, and
  `runtime/host/include/tplc/host/Status.hpp`.
- *Emit sites* are exact-symbol references in `runtime/`, `tools/`,
  `compiler/` — authoritative.
- *Test hits* are symbol references plus numeric renderings tests
  grep for (`"-501"`, `ctl 415`, `error 401`, `code=415`) —
  heuristic; a small-code literal like `-21` in an FBD fixture
  counts as a hit even though it is data, not an assertion.
- `TPLC_E_MODULE_MIN/MAX` are range sentinels and are excluded.
- Task codes (`TPLC_TASK_E_*`) are produced by *generated module
  code*, not the host — host emit count of 0 is expected for most of
  them.

## Current numbers

65 declared codes | 45 emitted by runtime/tools | 29 test-hit (~44%)

## Categories of gap

**Emitted but untested.** Real coverage gaps, worth a targeted test:

- ctl plumbing: `CTL_STATE`, `CTL_UNKNOWN_OP`, `CTL_PROTO`,
  `CTL_PAYLOAD` — malformed/state-mismatched requests
- debug plane: `DBG_RESOLVE`, `DBG_OVERLOAD`, `DBG_STATE`,
  `TRACE_CONFIG`
- package validation: `PKG_VERSION`, `PKG_TOC`,
  `PKG_UNKNOWN_SECTION`, `PKG_LIMIT`, `SIG_INVALID`,
  `TARGET_MISMATCH`
- image: `IMG_VERSION`, `IMG_TOO_SMALL`, `IMG_EPOCH`
- host loader: `HOST_SYM`, `HOST_DESC`, `HOST_RANGE`, `HOST_STATE`
- `MIGRATE_LAYOUT`, `ABI_EXT_UNMET`, `TASK_E_OVERRUN`

**Declared but never emitted** (dead reservations — prune or wire):

- `TPLC_E_DBG_STALE` (-411)
- `TPLC_E_MGMT_OP`, `MGMT_AUTHZ`, `MGMT_SCHEMA` (-500/-502/-503) —
  ctl authz denials use `CTL_AUTHZ` (-401) uniformly today
- `TPLC_E_PKG_HIDDEN_STATE`, `SYMMAP_RANGE`, `PKG_ABI_MISMATCH`,
  `REQUIRES_UNMET`, `PKG_SECTION_FORMAT`

**Module-side codes the host cannot emit.** `TASK_E_PANIC`,
`TASK_E_BOUNDS`, `TASK_E_DIVZERO`: the checked-arithmetic fault
model is deferred — codegen currently saturates div-zero to 0
(`llvm_emit.rs` "div-zero guard → 0 (deterministic fault model is
P2)"). These come live with the checked-mode work.
