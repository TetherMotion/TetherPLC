# Runtime ABI & host library (as built)

Status: implemented for the P0 slice — see `docs/plc/plan/02-runtime-abi.md`
for the normative contract. Deviations: `docs/plc/DECISIONS.md`.

## What exists

- `include/tplc/tplc_abi.h` — the ABI itself, verbatim from the normative
  listing: 6 exported symbols, all structs `_Static_assert`-locked
  (`TplcIoBinding` 20 B, `TplcTaskDesc` 64 B, `TplcServices` host-offset
  pinned), full return-code space, ctx/task/migrate contexts, `abort_flag` +
  `budget_deadline_ns` for cooperative checkpoints.
- `runtime/host/` — C++17 host library:
  - `Module` — `dlopen(RTLD_NOW|RTLD_LOCAL)`, resolves the symbol table,
    validates descriptor (size, abi_version, caps `abi_ext ⊆ host_caps`,
    task-table sanity, retain range, name termination), `has_migrate` flag
    ↔ symbol consistency.
  - `Program` — 64 B-aligned host-owned regions (state blob, image_in,
    image_out, markers), `TplcCtx` construction, `start()` → `tplc_init`,
    `stop()` → `tplc_shutdown`, `executeTask` implementing the §5 sequence
    (latch hook → `copyInputsIn` → ctx refresh → `task_fns[entry_index]` →
    `copyOutputsOut` → commit hook), binding copies incl. BOOL bit
    addressing (`unit` = bit index), `moveBlob`/`snapshotBlob` conformance
    hooks, `migrateFrom` → `tplc_migrate`.
  - `HostServices` — the `TplcServices` table: bounded log sink, diag
    counters, `steady_ns`, retain-dirty range tracking, unimplemented
    `raise_event`/`get_service` (v1).
  - `Scheduler` — single-thread CYCLIC/FREEWHEELING dispatch, priority
    order within a tick, phase-stable `next_due` accumulation, per-task
    runs/late/faults stats.
- `tests/abi/stub_module.c` — the hand-written module (built as `stub_v1`,
  `stub_v2`, `stub_bad_abi`): two cyclic tasks, five io_bindings
  (BOOL + byte + %M direct map), cooperative abort checkpoint, retain-aware
  init, and a real `migrate_desc` walker (LAYD parse + copy/dropped/
  default/deny rules).
- `tests/abi/abi_conformance.cpp` — 68 checks end-to-end: SHM image →
  dlopen → init → latched scan cycles → committed outputs, `tplc_task_run`
  dispatch, scheduler, abort flag, blob relocation mid-life, retain-restore
  skip, migrate OK/DEGRADED/DENY/malformed.

## Conformance status vs doc 02 §11

| fixture | status |
|---|---|
| `stub_module` loads/runs | done |
| `bad_abi` rejected | done (version mismatch path) |
| `moving_host` | done (`moveBlob`) |
| `panic_module` | partial — abort-flag path done; panic-handler is a codegen/`tplc-rt` feature (P1) |
| `migrate_sim` | done for copy/dropped/default/deny/desc-invalid |
| armv6 qemu module | not yet (needs cross toolchain in CI) |
| MCU static-link shim | not yet |
| `bad_*` prohibition series | blocked on the post-link scanner (P8-04) |

## Control channel + binary (added)

- `include/tplc/tplc_ctl.h` — wire protocol: 16 B LE frame header, op
  ranges per plan doc 04 App I, session bootstrap (`hello`/`hello_ack`),
  packed payload structs for the implemented ops, ctl-local error range
  (-400..) + `TPLC_CTL_F_ERROR` flag-bit-15 error frames.
- `runtime/host/CtlServer` — nonblocking UDS server: framed codec,
  per-client read buffers, hello gate, op dispatch table, event fan-out
  by notify mask, bounded `poll()`. The smoke test caught and fixed a
  snapshot-vs-live-vector bug in `poll()` (clients accepted mid-poll
  must not index into the pre-poll fd array).
- `runtime/tether-plc` — the runtime process: `--image`/`--module`/
  `--ctl`/`--wait-ms`/`--poll-ns`/`--sim`; attaches image, validates
  image size ≥ descriptor, loads module, schedules tasks, heartbeats,
  implements ping/status/prog.{status,list,start,stop}/task.stats/
  image.stats/rt.{status,shutdown}; fault → `plc_status=fault` +
  `last_fault_code` per doc 02 §4.6.
- `tools/tplc-ctl` + `tools/ctl_client.h` — client CLI and shared
  client helper (hello, framed send/recv).
- `tests/runtime/ctl_smoke.cpp` — 27 checks across a real process
  boundary: the test plays master (image creator), forks `tether-plc`,
  and verifies %I→%Q data flow, ctl ops, stop/start gating (outputs
  freeze and resume), stats, and clean `rt.shutdown`.

## Not yet

- Force tables (input/output overlay application points are inside
  `executeTask` where the hooks sit — the force table itself is P6/doc 12).
- Task-local regions (D-IMPL-6), WCET budget accounting, watchdog kill path,
  online-change staging (the `migrateFrom` primitive exists; the
  snapshot/arm/commit dance is P3/doc 08).
- ctl ops beyond the P0 subset (deploy.*, retain.*, dbg.*, trace.*,
  map.*, prog.prepare/kill/reset/unload) — declared, pending phases.
