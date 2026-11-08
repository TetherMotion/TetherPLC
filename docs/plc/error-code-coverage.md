# Error-code coverage

Snapshot of the `TPLC_E_*` / `TPLC_TASK_E_*` space: which codes the
runtime emits and which the tests exercise. Regenerate with
`tools/error_code_report.py` (`--gaps` for the uncovered list,
`--json` for machine-readable output).

## Method and caveats

- Codes are enumerated from `include/tplc/tplc_abi.h`,
  `tplc_ctl.h`, `tplc_image.h`, `tplc_pkg.h`,
  `runtime/host/include/tplc/host/Status.hpp`, and
  `runtime/bridge/include/tplc/bridge/MapRunner.hpp` (the `MAP_*`
  report-domain block at `-600..-609`).
- *Emit sites* are exact-symbol references in `runtime/`, `tools/`,
  `compiler/` — authoritative for the C side. The Rust package
  reader reports `PkgError` variants by name rather than the C
  symbols, so compiler-emitted codes can show 0 emit sites here
  while still being exercised (see `PKG_SECTION_FORMAT`).
- *Test hits* are symbol references plus numeric renderings tests
  grep for (`"-501"`, `ctl 415`, `error 401`, `code=415`) —
  heuristic; a small-code literal like `-21` in an FBD fixture
  counts as a hit even though it is data, not an assertion.
- Test directories scanned: `tests/` (shell/C++ e2e) and
  `compiler/*/tests/` (Rust unit tests). `TPLC_TASK_E_*` codes are
  produced by *generated module code*, so the Rust codegen tests
  are where they are asserted.
- `TPLC_E_MODULE_MIN/MAX` are range sentinels and are excluded.

## Current numbers

75 declared codes | 53 emitted by runtime/tools | 62 test-hit (~83%)

Every code that has a host/tool emit site is covered except
`IMG_EPOCH`; the remainder are declared-but-unemitted reservations.

## Induced-error conformance test

`tests/scan/error_paths.sh` (ctest `error_paths`) exercises every
reachable error surface and asserts the numeric magnitude on stderr
or the wire reply:

- protocol: `CTL_STATE`, `CTL_UNKNOWN_OP`, `CTL_PAYLOAD` via
  crafted frames (op before HELLO, bogus opcode, short payload)
- ctl/debug plane: `DBG_STATE`, `DBG_OVERLOAD`, `TRACE_CONFIG`,
  `DBG_ACCESS`, `MIGRATE_LAYOUT`, `HOST_STATE`
- host loader: `HOST_SYM`, `HOST_DESC`, `HOST_RANGE`,
  `ABI_EXT_UNMET` via gcc-built stub modules
- package reader: `PKG_VERSION`, `PKG_TOC`, `PKG_LIMIT`,
  `PKG_UNKNOWN_SECTION`, `TARGET_MISMATCH`, `SIG_INVALID`,
  `PKG_SECTION_FORMAT` via byte-level package mutation (the
  `pkgmut.py` helper repatches `content_hash`/`header_hash` so
  surgical corruptions survive the integrity checks)
- image: `IMG_VERSION`, `IMG_TOO_SMALL` via crafted shm headers
- map validator: `MAP_RANGE`/`TYPE`/`SAFE`/`OWNER`/`SAFETY`/`AT`/`BIND`
  via a map file hitting every implemented violation rule

## Remaining gaps and why they stay

**Emitted but not deterministically inducible:**

- `TPLC_E_IMG_EPOCH` (-104) — fires inside the bounded seqlock
  read-retry when `image_epoch` changes mid-read. In v1 the epoch
  is written exactly once at `tplc_image_create`, so there is no
  in-place re-init path a test can race. Becomes testable when a
  master re-init op exists.

**Declared but never emitted** (reserved — wire or prune):

- `TPLC_TASK_E_PANIC`, `TASK_E_BOUNDS`, `TASK_E_DIVZERO`
  (-17/-18/-19) — generated code saturates arithmetic and has no
  bounds-check aborts today; reserved for checked-code modes.
- `TPLC_E_MGMT_OP`, `MGMT_AUTHZ`, `MGMT_SCHEMA` (-500/-502/-503) —
  reserved for the CiA-405 management surface; ctl authz denials
  use `CTL_AUTHZ` (-401) uniformly today. (The former `-500` alias
  with `TPLC_E_MAP_*` is resolved — MAP codes moved to `-600..`.)
- `TPLC_E_PKG_HIDDEN_STATE`, `SYMMAP_RANGE`, `PKG_ABI_MISMATCH`,
  `REQUIRES_UNMET` (-210/-211/-213/-214) — declared in
  `tplc-pkg`'s `PkgError` but no site returns them yet (manifest
  validation depth not implemented).
- `TPLC_E_MAP_DIR`, `MAP_PDO`, `MAP_EPOCH` (-601/-604/-609) —
  declared in the bridge validator's block for map checks v1 does
  not implement (direction/PDO/epoch rules).
