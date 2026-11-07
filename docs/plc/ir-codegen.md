
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
