
## Online-change emission (doc 08)

- `tplc-mir/migrate.rs` decodes LAYD/SMAP/TYPC from a predecessor
  package, expands leaf fields (struct/FB members, array elems), and
  classifies rules: `copy` (same name + identical canonical type sig),
  `transform` (signed/unsigned int widening chains, `REAL→LREAL`,
  TIME by ns, `STRING(n)→(m)` prefix, array-bound growth, same-size
  enum reorder), `default-init`, `dropped`, `deny`. Type signatures
  are canonical (structure, not index), so rules survive TYPC index
  reordering between compilations.
- `tplcc build --change-against <pkg>` plans the diff up front; any
  `deny` rule fails the build with `CHANGE-DENY<n>` diagnostics (the
  unsafe change never reaches a runtime). Clean plans emit a MIGD
  section: `predecessor_hash` (sha256 over pred LAYD+SMAP+TYPC),
  rule table, embedded predecessor LAYD.
- `llvm_emit::emit_migrate` emits `tplc_migrate`: old/new layout-hash
  compare → `TPLC_E_MIGRATE_LAYOUT` (-32), deny-rule marker → -33,
  then per-rule copy/memcpy, widening `s/zext`, literal-init stores;
  blob-range checks → -35; `out_status` OK/OK_DEGRADED,
  `out_first_error` = first failed field index. Descriptor `flags`
  bit0 set; `Module::open` requires the symbol when the flag is set.
- JIT-verified in `tplc-codegen/tests/llvm_e2e.rs` (copy, INT→DINT
  widen, hash race, blob-range) and end-to-end via
  `tests/online_change/e2e.sh`.

## Behavior vectors (P1-10)

`tplcc vectors run <file.jsonl|dir>` executes JSONL vectors on the L1
interpreter: `{id, program, cycles:[{i:hex, t_ns, expect_q:hex,
expect_fields:{name:val}}], source}`. `i`/`expect_q` are image bytes
routed through the io_bindings (same addressing as runtime); field
expects accept `T#500ms`, `TRUE`, `16#…`, ints, floats.

Corpus: `tools/oracle/corpus/` (4 spec-derived vectors: TON fire/
clamp/drop, CTU count/edge/reset — all green). `tools/oracle-matiec/`
holds the quarantined GPL adapter skeleton per doc 14 §3 (subprocess
only, outputs = data); matiec isn't installed — `source:"spec"`
vectors stand in until `iec2c` produces real traces.
