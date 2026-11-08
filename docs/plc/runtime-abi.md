# Runtime ABI & host library (as built)
Status: as-built. Deviations from the original plan: `docs/plc/DECISIONS.md`.

> `doc NN` section references below are provenance to the retired
> plan chapters; this document is authoritative (see
> `architecture.md` §7).

## What exists

- `include/tplc/tplc_abi.h` — the ABI itself, verbatim from the normative
  listing: 6 exported symbols, all structs `_Static_assert`-locked
  (`TplcIoBinding` 20 B, `TplcTaskDesc` 64 B, `TplcServices` host-offset
  pinned), full return-code space, ctx/task/migrate contexts, `abort_flag` +
  `budget_deadline_ns` for cooperative checkpoints.
- `runtime/host/` — C++17 host library:
  - `Module` — post-link scan (`ElfScan`, doc 15 §4.3) then
    `dlopen(RTLD_NOW|RTLD_LOCAL)`. The scanner rejects
    (`TPLC_E_PKG_SYSCALL`, -216) any module with kernel-gate opcodes
    (syscall/sysenter/sysret/sysexit/int/into/int1 on x86-64,
    svc on aarch64) at instruction boundaries inside executable
    sections, RWX segments, an executable stack, PT_INTERP, a
    DT_NEEDED outside the libc/libm/rt whitelist, or an undefined
    dynsym import outside the libc/libm/abort surface. x86-64 code is
    decoded at STT_FUNC symbol boundaries with a minimal length
    decoder so forbidden byte patterns inside immediates do not
    false-positive; bytes outside symbols are scanned literally.
    Then `dlopen`, resolves the symbol table,
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

## Conformance status vs runtime-abi.md §11

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
  ranges per plan plc-runtime.md App I, session bootstrap (`hello`/`hello_ack`),
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
  `last_fault_code` per runtime-abi.md §4.6.
- `tools/tplc-ctl` + `tools/ctl_client.h` — client CLI and shared
  client helper (hello, framed send/recv).
- `tests/runtime/ctl_smoke.cpp` — 27 checks across a real process
  boundary: the test plays master (image creator), forks `tether-plc`,
  and verifies %I→%Q data flow, ctl ops, stop/start gating (outputs
  freeze and resume), stats, and clean `rt.shutdown`.

## Fault-injection harness (P0-09, `tests/fault/fault_injection.cpp`)

37 checks across three scenarios:

- **Cooperative abort / hang path** — `Program::requestAbort()` while the
  stub task spins in its checkpoint loop: task returns
  `TPLC_TASK_E_ABORTED`, the %Q commit is skipped for that cycle
  (`executeTask` early-returns before `copyOutputsOut`), and clearing
  the flag resumes cleanly. This is the pattern codegen emits at every
  loop back-edge (plc-runtime.md §5).
- **kill -9** — real `tether-plc` child is SIGKILLed mid-run: plc
  heartbeat freezes, `tplc_image_hb_stale` detects loss inside the
  bound, image stays consistent for master reads/writes, and a fresh
  `tether-plc` reattaches and resumes data flow.
- **seq-odd adversary** — a child process holds `out_seq` odd (a writer
  dead mid-commit) after scribbling the payload: `read_outputs` exhausts
  retries bounded (`TPLC_E_IMG_SEQ_EXHAUSTED`, retries counted), the
  destination never sees torn bytes, kill -9 leaves it consistent, and
  seq repair restores reads.

## Not yet

- Force tables (input/output overlay application points are inside
  `executeTask` where the hooks sit — the force table itself is P6/doc 12).
- Task-local regions (D-IMPL-6).
- ctl ops beyond the implemented set (dbg.*, trace.*, map.*) —
  declared, pending phases.
- Non-cooperative task hang mid-scan (loop with no checkpoints): the
  monitor thread sets `abort_flag` + `plc_status=fault` and escalates
  to `_exit(2)` for supervisor respawn, but the task itself keeps
  running until it returns — there is no async preemption.
