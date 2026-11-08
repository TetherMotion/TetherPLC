# As-Built: `.tplc` Package Format

> Status: partial — the container, LAYD and MIGD codecs are implemented
> and conformance-tested on both implementations. SMAP/TCAT section
> readers, signature, zstd compression, manifest schema validation, and
> NEEDED/hidden-state ELF checks remain future work.

## Implemented

### Container (`include/tplc/tplc_pkg.h`)

Wire constants + error codes shared by both implementations:

- 160-byte fixed header: magic `TPLCPKG\0`, `format_version=1`,
  `header_size`, `toc_count`, `flags`, `total_size`, `package_uuid`
  (64 B ASCII NUL-padded, D-IMPL-14), `content_hash`, `header_hash`.
- TOC entries 32 B each, sorted strictly by `section_type`, payloads
  16-byte aligned starting at `header_size`.
- `header_hash` = SHA-256 over bytes 0..128.
- `content_hash` = SHA-256 over raw TOC bytes then all payloads in TOC
  order — amended from plan (D-IMPL-13): covers TOC semantics, not
  just payloads. Inter-section padding is intentionally not hashed.
- Reader invariants enforced identically on both sides: version cap,
  size match, TOC sort/alignment/overlap/bounds, per-type limits,
  unknown+required → `TPLC_E_PKG_UNKNOWN_SECTION`, then both hashes.
- Error block −200..−215 (see `tplc_pkg.h`); −215 `SECTION_FORMAT` is
  an implementation addition for malformed section-internal payloads.

### Rust writer/reader (`compiler/tplc-pkg`)

`tplc_pkg` crate, zero-dep besides `sha2`:

- `PackageWriter` — out-of-order section adds, canonical sort on build,
  per-type limits, `add_section_raw` for forward-compat types.
- `Package::parse` — full verification, zero-copy `Section` views,
  `validate_for_load`, `manifest_json`.
- `sections` module — `Layd`/`Migd` encode+decode, byte-identical to
  `tests/abi/desc_formats.h` (the C modules parse these buffers).
- `tplc-pkg` bin — `dump`, `verify`, `gen` (deterministic fixture).
- Tests: 10 cases — round-trip, LAYD-in-package, MIGD round-trip,
  payload/header/TOC tamper detection, sorted-TOC rejection,
  unknown-required rejection, unknown-optional pass.

### C++ reader (`runtime/pkg`, lib `tplc_pkg`)

- `tplc::Sha256` — header-only FIPS 180-4 (self-tested against "abc"/"").
- `tplc::Package` — `parse(buffer)` / `open(path)`; same invariants and
  error codes as the Rust side; `validateForLoad`, `manifestJson`,
  `sectionsOf`, `forEach`.
- `tests/pkg/pkg_conformance.cpp` — 25 checks against the golden
  fixture `tests/pkg/fixtures/sample.tplc` (Rust-written), incl. the
  same tamper cases — proves byte-level cross-implementation parity.

## Deviations / notes

- `MigdHeader` on the wire is `hash[32] | vlen u16 | version | pad4 |
  rule_count u32 | rules[16 B] | embedded LAYD` — matching
  `tests/abi/desc_formats.h` and the stub module's parser.
- `header_hash`/`content_hash` are integrity (tamper-evident), not
  authenticity — the `signature` section + trust policy are P8.
- `zstd` (SECF_COMPRESSED) is declared but no compressor/decompressor
  is linked yet; readers reject nothing about it today.
- `validate_for_load` is format-level only. Semantic load checks —
  `requires` vs image size, `targets[]` triple match, ABI version —
  belong to the host loader path (toolchain.md), not yet wired.
- Manifest is carried as opaque JSON bytes; schema validation is a
  P1-era concern (`tplcc` owns the producer side).

## Verification

- `cargo test -p tplc-pkg` → 10 tests.
- `ctest -R pkg_conformance` → 25 checks, golden fixture + tampering.
- Fixture regeneration: `cargo run -p tplc-pkg --bin tplc-pkg -- gen
  tests/pkg/fixtures/sample.tplc`.
