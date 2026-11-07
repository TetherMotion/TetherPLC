# PLC runtime process — as-built

Status of `runtime/tether-plc` + `runtime/host` against
`plan/04-plc-runtime.md`.

## Process

`tether-plc --image <shm> --module <so> | --package <.tplc>
[--ctl <uds>|none] [--retain DIR] [--retain-flush-ms N] [--sim]`

- Attaches to the SHM process image (plc role), validates size.
- `--package` extracts the `target_obj` section to a temp `.so`, then
  the normal `dlopen` path + descriptor validation.
- Single-threaded cooperative scan: `Scheduler::tick` dispatches due
  tasks in priority order inside the main loop; latch/commit hooks go
  through the seqlock image protocol.
- `--ctl none` runs headless (fault injection / tests).

## Control channel (0x0— ops implemented)

`PING`, `RT_STATUS`, `RT_SHUTDOWN`, `PROG_STATUS`, `PROG_LIST`,
`PROG_START`, `PROG_STOP` (schedule gating only), `TASK_STATS`,
`IMAGE_STATS`, `RETAIN_STATUS`, `RETAIN_FLUSH`, `RETAIN_CLEAR`,
`RETAIN_DUMP`, plus `HELLO`/`SET_NOTIFY` session ops.

## Retain store (doc 04 §5)

- `RetainStore` mirrors `retain_region` to
  `<dir>/<program_uuid>.bin` — a second copy, not a blob mmap.
- 64 B header `{magic 'TRET', version, program_uuid,
  retain_layout_hash(lo8), generation, len, payload_crc32,
  header_crc32}` + payload.
- Restore: file magic/version/uuid/layout-hash/len + both CRCs must
  match; payload is copied into the blob before `tplc_init` and
  `ctx_flags.RETAIN_RESTORED` is set — generated init then skips the
  retain-region memset and retain literal inits (verified end-to-end
  across process restart).
- Flush triggers: `svc.retain_dirty` hints drained in the main loop,
  periodic `--retain-flush-ms` (default 1000), stop/shutdown.
  `stage()` memcpys the region into a staging buffer between ticks
  (task boundary); `flush()` writes + `fdatasync`.
- Stats surfaced via `RETAIN_STATUS` (attached/dirty/len/generation/
  flushes/restores/losses).
- v1 simplification: staging and flushing run on the main loop thread,
  not a dedicated flusher thread — bounded since the scan is on the
  same thread; the spec's separate flusher exists for a multi-thread
  scan, which does not exist yet.

## Module-side init (codegen note)

`tplc_init` zeroes the blob in two segments around the retain region;
the retain segment is zeroed + retain literal inits applied only when
`!CTXF_RETAIN_RESTORED`. (An earlier unconditional whole-blob memset
destroyed restored retain data — fixed at P2-01.)

## Watchdog & budget (doc 04 §3.2/§6)

- Emitted tasks carry `ENABLED_AT_START|WATCHDOGGED`; `executeTask`
  arms `ctx.budget_deadline_ns` = now + `--wcet-hard-ns` (or 2x the
  task `wcet_hint_ns`) per dispatch; 0 = unbounded.
- Checkpoints (`__ckpt` on loop back-edges/call boundaries) test
  `abort_flag` (**volatile** load — the monitor thread may write it;
  a plain load is hoisted out of `WHILE TRUE` loops) and, when the
  deadline is armed, call `svc->steady_ns()` and compare. Distinct
  rcs: `-22 TPLC_TASK_E_ABORTED`, `-23 TPLC_TASK_E_OVERRUN`. Verified
  end-to-end: `WHILE TRUE` faults with -22 (abort) and -23 (5 ms
  WCET bound) through the real process.
- Per task: `runs, late, max_late_ns, faults, overruns` (period
  overruns = finished past next release; skip policy — start
  immediately, no catch-up) and `exec_ns min/avg/max` via
  `Program::lastExecNs`.
- Monitor thread (P2-02): progress counter polled every 50 ms; a
  stall beyond `--hang-timeout-ms` (default 2000) sets `plc_status=
  fault` + `abort_flag`; unrecovered beyond 2x → `_exit(2)` for
  supervisor respawn. stall_since anchored to thread start.
- `ctx.last_cycle_overrun_ns` = signed finish-vs-scheduled+period.

## Bridge (`runtime/bridge/`, `tether-plc-bridge`)

The master-side half of doc 03/09 as a reusable library plus a dev
driver process:

- `ImageServer` — creates/owns the shm image (master role), master
  hb/status, `writeInputs` (%I) / `readOutputs` (%Q).
- `MapRunner` — per-cycle `pdo→%I` / `%Q→pdo` over the v1 `sim`
  transport (each channel key is a file under `--simdir`; absent
  input files hold last). Output channels substitute `safe`
  (zero/last/defined) when the PLC is not ok; map is a line format
  with range + single-%Q-writer validation (`parse_map`).
- `Supervisor` — spawn/respawn `tether-plc` child, waitpid liveness
  + plc_hb staleness kill, respawn policy + delay + 3/60s cap (same
  contract as `--supervise`, as a library for the Tether app).
- `tether-plc-bridge --image X --map F --hz N --plc <args>` — the
  standalone driver: image create → spawn plc → cyclic map loop.
  Verified end-to-end: `simdir/btn` byte → %IX0.0 → TON → %QX0.0 →
  `simdir/lamp` at 200 Hz; SIGKILL'd child → stale hb → safe=zero
  on outputs → supervisor respawn → recovery.
- `plc_ok` uses `tplc_image_hb_stale` (a hb newer than `now` counts
  as fresh — unsigned-underflow bug fixed).

## Simulation (doc 14)

- `sim_time_ns` in the image header (carved from reserved) — master
  publishes, `tether-plc --sim` sources `ctx->now_ns` +
  `svc->steady_ns` from it (HostServices override), so all program
  time is virtual while heartbeats stay real.
- `tether-plc-sim` = L2 process-sim: creates the image, spawns the
  real binary with `--sim`, runs a step file
  (`step N time=<ns> [in off=hex..] [expect off=hex..]`), waits for
  the next committed scan per step, diffs %Q — verified with a TON
  vector at 200/600/700 ms crossings; divergence prints step+offset.
- L1 = `tplcc vectors` (interp, same canonical IR). L3 lives in the
  Tether app's simulated-plant fleet — not this repo.

## Not yet

- `PROG_PREPARE/KILL/RESET/UNLOAD` (P2/P3 lifecycle).
- Multi-program scheduling (descriptor supports it; driver is
  single-program).
- ControlProxy curated ctl client in the bridge (ops work via
  `tplc-ctl` meanwhile), real EtherCAT/Modbus transports (sim only),
  overrun-policy config (skip is hardcoded), freewheel guard/slice
  enforcement, sim-parity harness across L1/L2 (same vectors on both
  levels — the formats differ today).
