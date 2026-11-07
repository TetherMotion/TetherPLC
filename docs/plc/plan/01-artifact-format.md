# 01 — Artifact Format: the `.tplc` Package

> **Status:** work-item document; deleted when `docs/plc/artifact-format.md`
> exists and `libtplc-pkg` (Rust writer + C++ reader) round-trips the spec.
>
> **Satisfies:** G1 (canonical IR), G2 (target-native payload), G4 (migration
> metadata), G7 (sim carries same bitcode), G8 (symbol map), G11 (provenance/
> signature), G12 (no GPL payloads).
> **Depends on:** 00. **Informs:** 02, 07, 08, 10, 14.

---

## 1. Purpose and requirements

The `.tplc` file is the **single deployable unit** produced by `tplcc` and
consumed by `tether-plc` (on-target), the sim harness (on host), and the web
IDE (read-only, for diagnostics). It must:

1. Carry the **canonical LLVM bitcode** so that simulation and future target
   backends (incl. MCU, doc 18) work from identical IR.
2. Carry **one or more target shared objects** compiled from that bitcode —
   a package may embed multiple target builds (e.g. `x86_64-linux-gnu` +
   `aarch64-linux-gnu`) so one package serves a heterogeneous fleet.
3. Carry the **symbol map** — the complete debugging/monitoring/migration
   metadata — in a form readable without LLVM.
4. Carry everything online change needs **without consulting the previous
   package**: the *current* layout descriptor always; a *migration descriptor*
   for the immediate predecessor when the package was produced as an online
   change.
5. Be **verifiable**: per-section hashes, whole-file integrity, optional
   cryptographic signature; the runtime must be able to reject tampered or
   mismatched artifacts deterministically.
6. Be **streamable**: parseable header-first so a runtime can reject an
   incompatible artifact before reading the payload; sections aligned for
   direct `mmap`/extraction.
7. Be **self-describing**: `tplcc info pkg.tplc` (and the runtime's load
   diagnostics) can dump every section without external schemas.
8. Stay **GPL-clean** (G12): all payload producers are our own toolchain.

Non-goals: it is not a project file (sources/projects are PLCopen XML, doc
06/11); not a container for arbitrary resources; not encrypted (integrity and
authenticity only — obfuscation is out of scope).

## 2. Container layout

Custom fixed-format container — no external archive library required on
either side (Rust writer ~300 LOC, C++ reader ~200 LOC). All fields LE.

```
offset  size  field
0       8     magic        "TPLCPKG\0"
8       u32   format_version      (= 1)
12      u32   header_size         (bytes; header + TOC, aligned to 16)
16      u32   toc_count
20      u32   flags               (bit0: signed, bit1: compressed, rest rsvd=0)
24      u64   total_size          (file size; sanity check)
32      64    package_uuid        (UUIDv7 or random; unique per build)
96      32    content_hash        (SHA-256 over raw TOC bytes then all
                                   section payloads, canonical TOC order —
                                   covers section type/flags/aux tampering)
128     32    header_hash         (SHA-256 over bytes 0..128)
160     ...   TOC[toc_count]      (32 bytes each, see below)
...     ...   (padding to 16B)
hdrsz   ...   section payloads, each aligned to 16 B
```

TOC entry (32 B):

```
+0   u32   section_type      (enum, below)
+4   u32   section_flags     (bit0: required-for-load, bit1: compressed
                              payload[zstd], rest rsvd)
+8   u64   offset            (absolute, 16B-aligned)
+16  u64   size              (payload bytes, uncompressed size if compressed)
+24  u32   section_version   (schema version of this section's content)
+28  u32   aux               (per-type auxiliary, e.g. target index)
```

Invariants checked by every reader:

- `header_hash` and `content_hash` verify before any section is used.
- Sections do not overlap, lie within `total_size`, and are sorted by
  `section_type` in the TOC (canonical order).
- Unknown `section_type` with `required-for-load` set → hard reject with
  `TPLC_E_PKG_UNKNOWN_SECTION`.
- `format_version` > reader's max → reject with `TPLC_E_PKG_VERSION`.

### 2.1 Section types

| id | name | required | content |
|---|---|---|---|
| 0x0001 | `manifest` | yes | UTF-8 manifest (§3) |
| 0x0002 | `ir_bitcode` | yes | canonical LLVM bitcode module (§4) |
| 0x0010 | `target_obj` | ≥1 | native shared object for one target (§5); `aux` = target index into manifest `targets[]` |
| 0x0020 | `symmap` | yes | binary symbol map (§6) |
| 0x0021 | `type_catalog` | yes | binary type-descriptor table referenced by symmap (§6.6) |
| 0x0022 | `layout_desc` | yes | state-blob layout descriptor (§7) |
| 0x0023 | `migrate_desc` | no | predecessor layout + migration hints (§8) |
| 0x0030 | `io_map` | no | compiled IO mapping requirements (doc 09 owns semantics; reserved here) |
| 0x0040 | `source_manifest` | no | hashes + names of contributing sources (not the sources) (§9) |
| 0x0041 | `plcopen_xml` | no | the FBD/project PLCopen XML, for editor round-trip (§9) |
| 0x0042 | `st_sources` | no | concatenated ST sources w/ index (editor round-trip) (§9) |
| 0x0050 | `debug_line` | no | compact line/FBD-node program table if stripped from .so (§10) |
| 0x0060 | `signature` | no | signature block (§11) |
| 0x0070 | `toolchain` | no | toolchain identity + versions (§12) |

Rationale notes:

- `ir_bitcode` is *required* even when only native execution is intended —
  G7's sim parity and G1's "one IR" are non-negotiable; stripping is a
  build option (`--strip-ir`) for size-constrained deployments, recorded in
  the manifest (`ir_stripped=true`).
- `target_obj` is plural-capable; at least one must match the loading target.
- `layout_desc` is *required* — every package knows its own layout;
  `migrate_desc` is what makes a package an *online-change candidate*.
- Sources inside the package are optional (IP concern) but strongly
  recommended for debug UX; editor flows regenerate them anyway.

### 2.2 Size budget

Default limits readers must enforce (larger → `TPLC_E_PKG_LIMIT`):

| item | default max |
|---|---|
| file | 64 MiB |
| `ir_bitcode` | 32 MiB |
| `target_obj` each | 32 MiB |
| `symmap`+`type_catalog` | 8 MiB |
| `manifest` | 256 KiB |
| `toc_count` | 64 |

## 3. `manifest` section

UTF-8 JSON (schema-versioned; JSON rather than XML because both Rust and C++
sides already handle it and the file is machine-edited, not IEC-facing —
PLCopen XML remains the *source-level* interchange). Reserved option to
re-issue as PLCopen XML later without changing container.

```json
{
  "schema": "tplc.manifest.v1",
  "name": "fill_station",
  "package_uuid": "…",              // must equal header uuid
  "program_uuid": "…",              // stable identity of the PROGRAM
                                    // (same across recompiles/versions)
  "program_version": "1.4.0+build.231",
  "predecessor": {                  // present iff this is an online change
    "package_uuid": "…",
    "program_version": "1.3.2+build.207",
    "layout_hash": "3f9c…"          // hex
  },
  "layout_hash": "a71d…",           // hash of layout_desc content
  "abi_version": 1,                 // runtime ABI required (doc 02)
  "languages": ["st", "fbd"],
  "created": "2026-11-07T…Z",
  "created_by": "tplcc 0.9.2 (rustc …, llvm 21)",
  "targets": [
    {
      "triple": "x86_64-unknown-linux-gnu",
      "section_toc_index": 4,
      "link": "dynamic-glibc",
      "min_glibc": "2.31",
      "sha256": "…"
    },
    { "triple": "armv6-unknown-linux-gnueabihf", "section_toc_index": 5, ... }
  ],
  "tasks": [                        // duplicated from descriptor for
    {"name":"MAIN","period_ns":1000000,"kind":"cyclic","priority":0}
  ],                                // tooling convenience; .so is authority
  "requires": {                     // runtime-declared needs, verified pre-load
    "image_in_bytes": 128,
    "image_out_bytes": 128,
    "marker_bytes": 4096,
    "retain_bytes": 8192,
    "state_blob_bytes": 262144
  },
  "ir_stripped": false,
  "compile": {"opt_level": 2, "flags": ["fp:strict"]}
}
```

Field rules:

- `program_uuid` is assigned at first successful compile of a project and
  stored in the project file; it is the identity the runtime uses to decide
  "online change of X" vs "different program".
- `predecessor` may appear at most once — chains longer than one hop are
  resolved by the *toolchain* (it can fetch the running program's layout via
  the deploy API and emit a direct migration; doc 08).
- `requires` lets the runtime refuse before `dlopen` when the configured
  image is too small.
- `targets[].section_toc_index` binds a manifest target to its TOC entry.

## 4. `ir_bitcode` section

- Contents: one LLVM module, standard bitcode container (magic `0x0B17C0DE`
  wrapped in bitcode header), produced by the codegen from the merged module:
  program code **+ embedded `tplc-rt` (stdlib) already linked in**.
- Target-data layout/ triple inside the module is the *canonical* one for the
  declared target list; multi-target packages re-codegen target modules from a
  target-independent IR subset — v1 simplification: **the bitcode section is
  produced per declared primary target**, and `targets[]` entries beyond the
  first are pure `.so` payloads codegenned from a copy. (Record `ir_scope` in
  manifest: `"primary"` default.)
- Version pinning: manifest records `llvm_version`; sim hosts whose LLVM
  major differs must fall back to JIT-compiling a *target_obj for the host*
  (toolchain emits host `.so` into the package by default) or reject. Bitcode
  is canonical semantically but **not promised portable across LLVM majors**.
- Bitcode flags: functions `internal` except ABI exports; no inline asm;
  `uwtable` on; `frame-pointer` preserved; no `noinline` stripping of
  `optnone` on `migrate` functions (doc 08 needs reliable stepping there).

## 5. `target_obj` section

- A fully linked `ET_DYN` ELF for the declared triple; `aux` (TOC) and
  `targets[]` (manifest) cross-reference each other.
- Exported dynamic symbols: exactly the ABI set from doc 02
  (`tplc_abi_version`, `tplc_get_descriptor`, …) — version script enforces;
  all other symbols `local`/hidden.
- Link rules: `tplc-rt` statically linked (version-matched, no dependency);
  external deps limited to `libc`, `libm` (and `libgcc_s` for unwinding where
  the target needs it) — recorded in manifest `link`/`min_glibc`. No `NEEDED`
  entries outside that whitelist; the C++ reader verifies with a tiny
  dynamic-section walk (no libelf dependency).
- Built `-fPIC`, `-fvisibility=hidden`, `-z now`-compatible (runtime may
  `dlopen` with `RTLD_NOW|RTLD_LOCAL`), relocations resolvable without
  textrel. CET/IBT/shadow-stack notes recorded in manifest `hardening`.
- `.gnu.hash` present (fast `dlsym`); `.symtab` may be present (debug UX) or
  stripped (then `debug_line` section holds the line table).
- Section-name reservation: generated code places nothing in writable
  ELF sections except `tplc-rt` internals; **the program's mutable state is
  100% in the runtime-allocated state blob** — the reader verifies `.data`
  `.bss` contain only tplc-rt symbols (whitelist) to keep G4's "no hidden
  state" enforceable. This check is `TPLC_E_PKG_HIDDEN_STATE`.

## 6. `symmap` section — binary symbol map

The single most-reused metadata structure (UI watch lists, forcing, tether_io
catalog, migration hints, diagnostics). Binary, not JSON: parsed on the RT
adjacent path and on tiny targets.

### 6.1 Layout

```
header (32 B):
  magic u32        = 'SMAP'
  version u32      = 1
  count u32
  strtab_off u32   (offset to string table, from section start)
  strtab_size u32
  typedesc_base u32  (index base into type_catalog section)
  flags u32        (bit0: has_provenance, bit1: has_init_values)
  reserved u32

entry[count] (64 B fixed + var tail):
  name_off u32        (into strtab)
  kind u8             (§6.2)
  flags u8            (bit0 retain, bit1 forced-capable, bit2 read-only,
                       bit3 volatile-input, bit4 is_fb_instance,
                       bit5 is_task_local)
  region u8           (0=state_blob 1=image_in 2=image_out 3=marker 4=retain)
  access u8           (0=rw 1=ro 2=wo — as seen by debugger/force)
  type_ref u32        (index into type_catalog + typedesc_base)
  offset u64          (byte offset within region)
  size u32            (bytes)
  align u32
  parent u32          (entry index of enclosing FB/struct, or 0xFFFFFFFF)
  first_child u32     (entry index of first child, or 0xFFFFFFFF)
  next_sibling u32    (entry index, or 0xFFFFFFFF)
  provenance u32      (index into provenance table, or 0xFFFFFFFF)
  default_off u32     (into defaults blob, or 0xFFFFFFFF)   [v2]
  reserved_pad 12 B
```

Fixed-size entries + child/sibling indices let readers walk FB instance
hierarchies without recursion and without strings. Strings live in one UTF-8
`strtab`; names are the IEC path (`MAIN.TonDelay.IN`, `GVL.Counter`).

### 6.2 `kind` enum

```
0 reserved
1 VAR               IEC variable (global or FB member, incl. locals lifted to blob)
2 VAR_INPUT/OUTPUT/IN_OUT/EXTERNAL — FB interface vars (kind encodes dir: 2..5)
6 VAR_RETAIN marker (redundant with flag; kept for filtering)
7 FB_INSTANCE
8 PROGRAM_INSTANCE
9 TASK_BINDING      (program instance ↔ task; aux fields reinterpreted)
10 IO_CHANNEL       (%I/%Q mapped address; offset = image offset)
11 ALIAS            (name alias to another entry; parent = target index)
12 CONSTANT         (VAR CONSTANT; read-only, value in defaults blob)
```

### 6.3 Provenance table (when `has_provenance`)

```
count u32
entry[count]:
  source_off u32    (strtab: file name / FBD sheet id)
  line u32  col u32 (ST)  —or—  fbd_node u32, fbd_pin u32 (kind-specific)
  kind u8           (0=st, 1=fbd-node, 2=fbd-wire, 3=generated, 4=sfc-reserved,
                     5=ld-reserved)
  pad 3 B
```

Dual-provenance is a *requirement* (G10): an FBD-authored variable must be
traceable to a diagram node for live-value overlay even though it shares the
ST backend (doc 11 depends on this).

### 6.4 Defaults blob (when `has_init_values`)

Concatenated initial-value images, referenced by `default_off`; used for
cold-init pre-fill (runtime may pre-seed blob before `tplc_init`), for
"reset to defaults" debug op, and by migration for added variables.

### 6.5 Invariants

- Every `region∈{1,2}` entry's `offset+size ≤ image_in_bytes/image_out_bytes`
  from manifest `requires` — loader verifies (else `TPLC_E_SYMMAP_RANGE`).
- Entries sorted by `(region, offset)` for binary-search over image ranges.
- `layout_hash` of the package covers `symmap` + `layout_desc` + `type_catalog`.

### 6.6 `type_catalog` section

Binary type descriptors, indexable:

```
entry:
  tag u8            (0=BOOL 1=SINT 2=INT 3=DINT 4=LINT 5=USINT..ULINT
                     6=REAL 7=LREAL 8=TIME 9=DATE 10=TOD 11=DT
                     12=STRING 13=WSTRING 14=BYTE..LWORD 15=ARRAY
                     16=STRUCT 17=ENUM 18=FB_TYPE 19=POINTER_TO 20=REF)
  name_off u32      (strtab-local)
  elem u32          (ARRAY: element type index; POINTER_TO/REF: target)
  dims u16          (ARRAY: dimension count)
  dim_lo/hi i32[4]  (ARRAY bounds, up to 4 dims)
  member_list u32   (STRUCT/FB: index into member table)
  size u32 align u32
  enum: count u32 + (name_off,i64) pairs
```

Structured types enumerate members (name, offset, type index) so the
debugger can render arbitrary values and forcing can address leaf fields.

## 7. `layout_desc` section — state-blob layout descriptor

The authoritative description of the state blob (doc 02 defines the blob;
doc 07 defines how codegen computes it). Format:

```
header: magic 'LAYD', version u32, region_count u32, total_size u64
region[region_count]: kind u8 (0=global 1=fb_instances 2=task_local
                             3=retain 4=internal),
                      offset u64, size u64, align u32
field-table: count u32 of {sym_entry u32, offset u64, size u32, type_ref u32,
                           gen u16}    // gen = layout generation, for migration
```

Notes:

- `layout_desc` is the *compiler's* contract for `tplc_init`/`tplc_migrate`
  correctness; `symmap` is the *debugger's* view. They overlap deliberately:
  `symmap` may omit internal fields; `layout_desc` is complete.
- `layout_hash` = SHA-256 over the canonical serialization of this section —
  the value `tplc_migrate` and the runtime compare on online change.
- Determinism requirement: identical inputs ⇒ identical `layout_desc` bytes ⇒
  identical hash (reproducible builds are required; see §13).

## 8. `migrate_desc` section — predecessor info

Present only for online-change packages:

```
predecessor_layout_hash 32 B
predecessor_program_version string
rule_count u32
rule[]:
  kind u8     (0=copy-by-name   identical name+type → memcpy
               1=transform      generated transform fn index
               2=default-init   new/renamed var → default
               3=dropped        old var has no home
               4=unsafe-deny    change requires cold start; never migrate)
  old_field u32   (index into predecessor field-table, embedded below)
  new_field u32   (index into layout_desc field-table)
  aux u32         (transform fn index / deny reason code)
embedded_predecessor_layout: (a full layout_desc of the predecessor,
                              verbatim from its package)
```

The compiler embeds the predecessor's `layout_desc` wholesale so the runtime
needs nothing from the old package — the running blob + this descriptor +
the generated `tplc_migrate` code are sufficient. Deny-rules make "this
change is unsafe" a *compile-time* decision with an explicit reason table —
the runtime only ever executes approved migrations (doc 08).

## 9. Source-carrying sections

- `plcopen_xml` — the full PLCopen XML project (FBD source of truth +
  declared ST POUs). Enables exact editor round-trip.
- `st_sources` — `count` + (name_off, len, off) index + concatenated UTF-8
  source. Present for ST-authored or mixed projects.
- `source_manifest` — `count` + (name, sha256, role) covering *all* inputs,
  including ones not embedded (IP option: ship hashes only). Lets the IDE
  prove "the running program was built from these files".
- Both editor sections are optional at load time; absence is a UX downgrade,
  not a failure. Manifest flag `sources_embedded`.

## 10. `debug_line` section

Compact address→provenance program table (symmap provenance indexes),
emitted when the `.so` is built stripped. Format: sorted `(code_offset,
prov_index)` pairs + delta-varint encoding spec (to be filled at
implementation; reserve section now). Only needed for "where am I in
source" diagnostics and future single-step UX (doc 12).

## 11. `signature` section

Optional (manifest `flags.signed`). Format:

```
algo u8        (1 = ed25519)
key_id 16 B    (key identifier — deploy policy maps to trust store)
sig 64 B       over content_hash (header field) → signs everything
meta_len u32 + meta (UTF-8 JSON: signer, timestamp, comment)
```

Verification policy is deployment configuration (doc 15): `open` (warn),
`require` (reject unsigned), `require-trusted` (reject unknown key).
Unsigned-reject error: `TPLC_E_SIG_*`. Signature presence never changes any
other section's semantics.

## 12. `toolchain` section

UTF-8 JSON: producer identity (`tplcc` version, rustc, LLVM major+patch,
tplc-rt version, container format version) + input digests (compiler config
hash, target spec). Pure provenance; no load gating except `abi_version` in
manifest.

## 13. Reproducibility requirements

- `tplcc` must be deterministic: identical source + identical toolchain
  identity ⇒ identical `layout_desc`, `symmap`, `ir_bitcode` bytes and
  identical `content_hash` (modulo `created` timestamp and package_uuid,
  which are excluded from content_hash inputs? — **decision:** package_uuid
  and created timestamp are *inside* the hashed header, so bit-identical
  output is only claimed per `(content, uuid)`; CI verifies determinism by
  comparing everything except `{package_uuid, created, header_hash,
  content_hash}`).
- Rationale: `layout_hash` stability across machines is what makes "same
  program recompiled" detectable — migration can then be skipped entirely
  when old and new hashes match (doc 08 fast path).

## 14. Reader/writer implementation notes

- Rust crate `tplc-pkg`: `Package::build()`, `Package::open()`, section
  iterators, `verify() -> Result<(), PkgError>`; zero-copy reads via `&[u8]`.
- C++ `tether::plc::Package`: same API surface over `std::span<const byte>`;
  mmap-friendly (offsets, not pointers); no allocation except path strings.
- Both sides share a conformance test corpus: `tests/pkg/golden/` fixtures +
  fuzz (structural mutations must never crash the reader; max parse bounds
  enforced).
- Endianness/alignment: readers must not assume host alignment of section
  payloads (memcpy into aligned structs when decoding headers).

## 15. Error codes (owned by this chapter)

`TPLC_E_PKG_MAGIC, _VERSION, _TOC, _HASH_HEADER, _HASH_CONTENT,
_PKG_UNKNOWN_SECTION, _PKG_LIMIT, _SIG_*, _PKG_HIDDEN_STATE,
_SYMMAP_RANGE, _TARGET_MISMATCH, _ABI_MISMATCH, _REQUIRES_UNMET`

## 16. Iteration-2 hooks

- `kind` enum reserves SFC (4) and LD (5) provenance tags.
- `target_obj` may carry MCU object format later (`aux` flag distinguishes
  PIC-blob payloads, doc 18) — container unchanged.
- `io_map` section placeholder for binding compiled IO requirements to a
  station config (doc 09).
- `section_version` allows per-section schema evolution without container v2.

## 17. Acceptance criteria

1. `tplc-pkg` (Rust) writes all sections; `tether::plc::Package` (C++) reads
   them bit-exact; round-trip conformance suite ≥95% branch on the reader's
   validation paths.
2. `tplcc` emits a compliant package for the ST and FBD reference programs;
   `tplcc info` dumps every section; `tplcc verify` checks all hashes and
   signatures.
3. Runtime rejects: wrong magic, truncated file, hash mismatch, unknown
   required section, oversized section, ABI mismatch, missing matching
   target, hidden mutable state — each with its distinct error code.
4. Migration happy path: package B with `migrate_desc` for package A loads
   and supplies everything `tplc_migrate` needs (validated in doc 08 tests).
5. Determinism: two independent builds of the same project differ only in
   `{package_uuid, created}`.
6. GPL-clean: no GPL code or generated text in the package; CI license scan
   on fixtures.

---

## Appendix G — `manifest` JSON Schema (normative)

```jsonc
{ "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://tether.dev/schemas/tplc.manifest.v1.json",
  "type": "object",
  "required": ["schema","name","package_uuid","program_uuid",
               "layout_hash","abi_version","languages","created",
               "targets","requires"],
  "properties": {
    "schema":      {"const":"tplc.manifest.v1"},
    "name":        {"type":"string","pattern":"^[A-Za-z_][A-Za-z0-9_]*$"},
    "package_uuid":{"$ref":"#/$defs/uuid"},
    "program_uuid":{"$ref":"#/$defs/uuid"},
    "program_version":{"type":"string"},
    "predecessor": {"type":"object","additionalProperties":false,
      "required":["package_uuid","layout_hash"],
      "properties":{
        "package_uuid":{"$ref":"#/$defs/uuid"},
        "program_version":{"type":"string"},
        "layout_hash":{"$ref":"#/$defs/hash"}}},
    "layout_hash": {"$ref":"#/$defs/hash"},
    "abi_version": {"type":"integer","minimum":1},
    "languages":   {"type":"array","items":
                    {"enum":["st","fbd","sfc","ld"]},"minItems":1},
    "created":     {"type":"string","format":"date-time"},
    "created_by":  {"type":"string"},
    "targets": {"type":"array","minItems":1,"items":{
      "type":"object","additionalProperties":false,
      "required":["triple","section_toc_index","sha256"],
      "properties":{
        "triple":{"type":"string"},
        "section_toc_index":{"type":"integer","minimum":0},
        "link":{"enum":["self-contained","dynamic-glibc","static-musl",
                        "pic-blob"]},
        "min_glibc":{"type":"string"},
        "hardening":{"type":"object"},
        "sha256":{"$ref":"#/$defs/hash"}}}},
    "tasks": {"type":"array","items":{
      "type":"object","required":["name","kind"],
      "properties":{"name":{"type":"string"},
        "kind":{"enum":["cyclic","freewheeling","event","status"]},
        "period_ns":{"type":"integer"},
        "priority":{"type":"integer","minimum":0,"maximum":255}}}},
    "requires": {"type":"object","additionalProperties":false,
      "properties":{
        "image_in_bytes":{"type":"integer","minimum":0},
        "image_out_bytes":{"type":"integer","minimum":0},
        "marker_bytes":{"type":"integer","minimum":0},
        "retain_bytes":{"type":"integer","minimum":0},
        "state_blob_bytes":{"type":"integer","minimum":1},
        "abi_ext":{"type":"integer","minimum":0}}},
    "ir_stripped":{"type":"boolean","default":false},
    "sources_embedded":{"type":"boolean","default":false},
    "probes_stripped":{"type":"boolean","default":false},
    "compile":{"type":"object","properties":{
        "opt_level":{"enum":[0,1,2,3,"s"]},
        "checks":{"enum":["default","strict","off"]},
        "flags":{"type":"array","items":{"type":"string"}}}}},
  "$defs":{
    "uuid":{"type":"string","pattern":"^[0-9a-f]{32}$|^[0-9a-f-]{36}$"},
    "hash":{"type":"string","pattern":"^[0-9a-f]{64}$"}},
  "additionalProperties": false }
```

## Appendix H — Binary layout quick reference (all sections)

`layout_desc` field-table entry (32 B):

```
+0  u32  sym_entry    (symmap index, or 0xFFFFFFFF internal)
+4  u8   field_kind   (0=var 1=fb_instance 2=task_local 3=internal
                       4=retain_wrapper)
+5  u8   flags        (bit0 retain bit4 fb_instance)
+6  u16  gen          (layout generation — bump on shape change)
+8  u64  offset
+16 u64  size
+24 u32  type_ref     (type_catalog index)
+28 u32  _pad
```

`migrate_desc` rule entry (16 B):

```
+0  u8   rule_kind    (0 copy 1 transform 2 default-init 3 dropped
                       4 deny 5 alias-rename)
+1  u8   severity     (0 info 1 degraded 2 deny)
+2  u16  transform_id (for kind=1; else 0)
+4  u32  old_field    (index into embedded predecessor field-table)
+8  u32  new_field    (index into layout_desc field-table)
+12 u32  aux          (deny reason code | alias string offset)
```

`type_catalog` member-table entry (STRUCT/FB sub-record, 16 B):

```
+0  u32  name_off     (section-local strtab)
+4  u32  type_idx     (member's type index)
+8  u64  offset       (within struct)
+16…   (packed 16 B total: offset is u32 + u32 flags — corrected:
        +8 u32 offset, +12 u32 flags[bit0 retain bit1 const])
```

`debug_line` entry pair (delta-coded): `(code_off_delta u32varint,
prov_idx_delta u32varint)`; table sorted by code_off; varint = LEB128.

`source_manifest` entry:

```
name_off u32  role u8 (0 st-source 1 fbd-model 2 plcopen 3 iomap
                       4 project-file 5 library-dep)
_pad 3 B
sha256 32 B
```
