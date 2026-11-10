# As-Built: `.tplc` Package Format

> `doc 15`/`doc 14` section references below are provenance to the
> retired plan chapters; this document is authoritative (see
> `architecture.md` §7).

> Status: partial — the container, LAYD and MIGD codecs are implemented
> and conformance-tested on both implementations. SMAP/TCAT section
> readers, signature, zstd compression, manifest schema validation, and
> `target_obj` is a linked `.so` for the host triple; for a non-host
> `--target` triple it is the raw relocatable ELF object and the
> manifest carries `target_triple` (plan 18 §7.1). `--pic-blob`
> swaps the raw object for a TPLB v1 loadable image (see
> `mcu-path.md`) and the manifest records
> `"target_obj_format":"tplb"`.
> `validateForLoad` scans `target_obj` for hidden state: defined,
> sized symbols in writable allocated `.data`/`.bss`-family sections
> must be known runtime/crt symbols or the package is rejected with
> `TPLC_E_PKG_HIDDEN_STATE`. NEEDED-entry checks remain future work.

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
- `header_hash`/`content_hash` are integrity (tamper-evident); the
  `signature` section (0x60) adds authenticity (doc 15 §4.1): payload
  `u32 ver=1 | key_id[32] | pubkey[32] | sig[64]` where
  `key_id = sha256(pubkey)` and `sig` is an ed25519 signature over
  `signing_hash` — sha256 of the non-signature TOC entries with their
  `offset` fields zeroed plus the non-signature payloads. Zeroing
  offset makes the signed message layout-independent: appending the
  signature section shifts every payload offset, so `content_hash`
  itself cannot be the signed message. `content_hash` still covers
  real offsets at parse time, so the two checks compose.
  `tplcc sign keygen|sign|verify` manages keys and trust stores
  (`<dir>/<hex-keyid>.pub`, 32 raw bytes); `tether-plc
  --sig-policy open|require|require-trusted --trust-dir D` enforces
  policy at every package load path (errors `TPLC_E_SIG_*`).
- `DEPLOY_MAP` (0x31) carries the bridge-format deploy map text
  (`in|out|safety` lines) injected at deploy time via
  `tplcc pkg add-section`; `tether-plc` parses its `safety` lines to
  deny `DBG_WRITE`/`DBG_FORCE` on safety-backed blob ranges (doc 15 §2).
  Distinct from `IO_MAP` (0x30), which is the compiler-emitted JSON
  binding table.
- `TOOLCHAIN` (0x70) is emitted by `tplcc build`: JSON
  `{tplcc, rustc, llvm, host}` baked at compiler build time — supply-
  chain provenance per doc 15 §6.
- `zstd` (SECF_COMPRESSED) is declared but no compressor/decompressor
  is linked yet; readers reject nothing about it today.
- `validate_for_load` now also runs the package-byte semantic checks:
  manifest `abi_version` must be 1 (`PKG_ABI_MISMATCH` -213), every
  blob-resident symmap leaf must lie inside the LAYD state blob
  (`SYMMAP_RANGE` -211; `SK_IO_CHANNEL` entries are exempt — their
  offsets address the process image), and `target_obj` must not hide
  program state in `.data`/`.bss` (`PKG_HIDDEN_STATE` -210).
- `tplcc` emits a manifest `requires` object
  (`image_in_bytes`/`image_out_bytes`/`marker_bytes` from `%`-AT
  binding extents, `retain_bytes`/`state_blob_bytes` from the
  layout). `Package::checkRequires` compares the image fields
  against the attached process image on every load path — boot,
  `PROG_PREPARE`, `PROG_SWAP`, `SHADOW_LOAD` — and fails with
  `REQUIRES_UNMET` (-214) before any module bytes are touched.
  `targets[]` triple selection remains unwired.
- `tplcc --checked-arith` is recorded as `"checked_arith":true` in
  the manifest (absent means the default saturating arithmetic).
  The flag is baked into the generated code (bitcode and `.so`
  both carry the guarded div/mod), so the manifest field is
  provenance for tools, not a runtime switch.
- `tplcc build --target <foreign> --pic-blob` stores a TPLB v1
  image in `target_obj` instead of the raw ET_REL and records
  `"target_obj_format":"tplb"` in the manifest (absent means the
  legacy raw-object/`.so` payload). TPLB layout: 32-byte header
  (magic `TPLB`, version, section/symbol/import/fixup counts),
  then section records (24 B: name_off, load_off, size, align,
  flags, reserved — flags bit0 writable, bit1 executable),
  symbol records (16 B: name_off, value_off, size, info, pad),
  import records (8 B: name_off, reserved), fixup records
  (16 B: place_off, type, value, addend), a deduplicated string
  table, then section contents — each section's `load_off` is also
  its offset from the RAM base the loader chooses. Fixup types: `ABS32/ABS64`
  store `S+A`, `MOVW/MOVT` patch the Thumb-2 imm field,
  `THM_CALL` re-encodes the BL pair, `REL32` stores `S+A-P`;
  `value` bit31 set means the low 31 bits index the import table
  and `S` is the loader-resolved import address. Full contract in
  `mcu-path.md` and `tests/abi/tplb_load.h`.
- Manifest is carried as opaque JSON bytes; schema validation is a
  P1-era concern (`tplcc` owns the producer side).
- `tplcc build --library` emits a **library package**: manifest
  `"kind":"library"` plus only Manifest, IrBitcode, TargetObj (the
  relocatable object — `__tplc_pou_*` symbols stay linkable for a
  future precompiled-consumption path; TPLB with `--pic-blob`),
  StSources, and Toolchain sections. No LAYD/SMAP/TYPC/IO_MAP —
  there is no program state to describe — so `validate_for_load`
  rejects library packages by construction: they are link-time
  artifacts, never deployed to `tether-plc`. Consumption today is
  `tplcc build <app.st> --lib <lib.tplc>`, which extracts the
  package's ST_SOURCES into the program's compilation unit (GVL
  declarations merge into the consumer's module correctly). The
  manifest `kind` field is absent on program packages.

## Verification

- `cargo test -p tplc-pkg` → 10 tests.
- `ctest -R pkg_conformance` → 25 checks, golden fixture + tampering.
- Fixture regeneration: `cargo run -p tplc-pkg --bin tplc-pkg -- gen
  tests/pkg/fixtures/sample.tplc`.
