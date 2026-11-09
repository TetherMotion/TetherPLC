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

75 declared codes | 68 emitted by runtime/tools | 74 test-hit (98%)

Every code with a real emit site is covered except race-only
`IMG_EPOCH`. `TPLC_TASK_E_DIVZERO` (-19), `TPLC_TASK_E_OVERFLOW`
(-20), and `TPLC_TASK_E_RANGE` (-21) are emitted: `tplcc
--checked-arith` switches integer div/mod by zero from the default
saturating-to-0 to a task fault, int add/sub/mul/neg overflow and
`INT_MIN / -1` to -20, and out-of-range conversions to -21; the
default mode stays bit-compatible with unflagged packages.

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
- reference faults: `TASK_E_PANIC` (-17) and `TASK_E_BOUNDS` (-18)
  via `reference_semantics_through_jit` (null / past-end deref)
- arithmetic faults: `TASK_E_DIVZERO` (-19), `TASK_E_OVERFLOW`
  (-20), and `TASK_E_RANGE` (-21) via `checked_arith_divzero_jit` /
  `saturating_arith_divmod_jit` / `checked_arith_overflow_range_jit`
  / `saturating_arith_overflow_range_jit` (LLVM JIT) and the
  `div.zero.sat` / `div.zero.checked` / `ovf.*` / `rng.*` oracle
  vectors, which `vector_run.py` replays through both the L1
  interpreter and the L2 process sim (`program fault rc=-NN` on
  stderr, frozen `%Q` after the faulted scan)
- package semantics: `PKG_ABI_MISMATCH` (-213) and `SYMMAP_RANGE`
  (-211) via `error_paths` payload mutations (manifest abi_version,
  out-of-blob symmap leaf); `validateForLoad` now also gates the
  swap/prepare/shadow-load ctl paths, not just boot
- object dictionary: `MGMT_OP`/`MGMT_SCHEMA`/`MGMT_AUTHZ` via
  `tests/scan/od_surface.sh` (unknown index, read-only write, entry
  below the session role)
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

- (`TASK_E_OVERFLOW` -20 and `TASK_E_RANGE` -21 left this list:
  `--checked-arith` emits them for int add/sub/mul/neg overflow,
  `INT_MIN / -1`, and out-of-range conversions. `TASK_E_DIVZERO`
  -19 and `TASK_E_PANIC`/`TASK_E_BOUNDS` are likewise covered.)
- `TPLC_E_INIT_LAYOUT`/`INIT_INVALID`/`INIT_VALUE` (-1..-3): the
  `tplc_init` contract codes; the host validates descriptors
  itself (`HOST_*`) and generated init cannot fail today, so they
  stay reserved for targets where init does validate.
- (`SYMMAP_RANGE` -211, `PKG_ABI_MISMATCH` -213, `PKG_HIDDEN_STATE`
  -210, and `REQUIRES_UNMET` -214 are all emitted by
  `validateForLoad`/`checkRequires` and covered by `error_paths` —
  the hidden-state check scans `target_obj` for program symbols in
  writable allocated `.data`/`.bss`-family sections, and
  `checkRequires` compares manifest `requires.image_*_bytes`
  against the attached image on every load path.)
- (The former `-500` alias between `TPLC_E_MGMT_*` and
  `TPLC_E_MAP_*` is resolved — MAP codes moved to `-600..`, and the
  MGMT codes are now emitted by the object dictionary, covered by
  `od_surface`.)
- (`TPLC_E_MAP_PDO` -604 is emitted: `pdo=` map channels are
  validated against the bus catalog the EtherCAT pump publishes as
  `<simdir>/.bus` — unknown endpoint, wrong direction, size
  mismatch, or missing catalog are all violations. `MAP_EPOCH`
  -609 is emitted: a `hash <sha256>` directive pins the map's
  canonical content hash — `tether-plc-bridge --map-hash` prints it
  for stamping, and content drift after stamping is a violation.
  `MAP_DIR` -601 is emitted: a `bind` whose AT space contradicts
  the channel direction (%I↔in, %Q↔out, %M has no channel) is a
  violation.)
