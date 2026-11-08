# Process image (as built)

Status: as-built. Deviations from the original plan: `docs/plc/DECISIONS.md`
D-IMPL-1/3/5.

## What exists

- `include/tplc/tplc_image.h` — file layout (256 B `TplcImageHeader`,
  `%I`/`%Q` areas at 64 B-aligned offsets), status enums, image-layer error
  codes, atomic accessors, and the complete API for both roles.
- `runtime/image/tplc_image.c` — `shm_open` create/attach, bounded-wait
  header publish (epoch>0 gates attach), seqlock writers
  (`tplc_image_write_inputs`, `tplc_image_commit_outputs`) and readers
  (`tplc_image_latch_inputs`, `tplc_image_read_outputs`), heartbeats, status,
  fault code, counters, range/role enforcement.
- `tools/tplc_image_tool.c` → `tplc-image create|dump|unlink`; `dump` decodes
  the header, seq parity, heartbeats, counters, and hex-dumps both areas —
  read-only, live-system safe, `--watch <ms>` refresh mode.
- `tests/image/image_conformance.c` — 41 checks: round-trips both directions,
  role/range rejection, stuck-seq bounded reads (`TPLC_E_IMG_SEQ_EXHAUSTED`
  + retry counter), stale-heartbeat helper, creator reclaim.

## Protocol in one paragraph

Single-writer per direction: master writes `%I` under `in_seq`, the PLC
runtime writes `%Q` under `out_seq`; each write is an odd-increment →
release-fence → payload → even-increment bracket. Readers retry a bounded
`MAX_TRIES=8` loop over `seq`+`image_epoch`; exhaustion is an error, never a
spin-forever, so a dead writer can't hang the peer. Liveness is
`master_hb_ns`/`plc_hb_ns` + status words; safe-state policies ride on top
(master substitutes per-channel safe values when `plc_*` goes stale — that
substitution lives in the master-side bridge, P2).

## Not yet

- `%I`/`%Q` region partitioning across multiple programs (map layer, P2).
- Per-program status sub-table (header `reserved[112]` is allocated).
- Input-loss policies (`hold_last`/`zero_inputs`/`stop_tasks`) — the hooks
  are the status+heartbeat reads; the policy engine is runtime P2 scope.
- Two-file opposing-permissions split for hard `%I` write-protection.
