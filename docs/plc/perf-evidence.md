# Performance evidence — DESIGN-GOALS §6

Measured numbers backing the §6 quantitative draft targets. Regenerate
with `tests/perf/evidence.sh` (not a ctest gate; run manually and
refresh this page).

## Method

- `tests/perf/evidence.sh` builds a `LINT` pair program via `tplcc`,
  runs it under `tether-plc` on a SHM image, and samples the
  `tplc-ctl prog` task stats (`runs`, `late`, `max_late`).
- Watchdog latency uses `tests/perf/hang_spin.c`, a valid ABI module
  whose task spins without cooperative checkpoints. The monitor's
  log line reports its own measured `stale_ms`; the script also polls
  the image `plc_status` flag and the process lifetime.
- Host numbers below are from a loaded x86-64 desktop (16 CPUs, GUI
  load). `max_late` is a cumulative worst-case over the whole run and
  is dominated by host jitter, not scheduler cost.

## Results (2026-xx-xx run)

| metric | DESIGN-GOALS §6 target | measured |
|---|---|---|
| cycle rate 1 kHz | 1 kHz | 1002 runs/s |
| cycle rate 4 kHz | up to 4 kHz | 3974 runs/s |
| max release lateness 1 kHz | (informational) | 629 us |
| max release lateness 4 kHz | (informational) | 1093 us |
| swap duration (ctl call, sync) | (informational) | 12 ms |
| scan pause injected by swap | <= 1 task cycle, < 500 us pref | no increase in max_late (same 1093 us before/after swap on 4 kHz task) |
| watchdog stall->FAULT flag | <= 1 cycle after trip | 350 ms (timeout 300 ms) |
| watchdog unrecovered->_exit | supervisor respawn bound | 724 ms (~2x timeout + boot) |

Second run on the same host showed max_late 92 us at 4 kHz and 3 us at
1 kHz — the variance is host load, so only cadence and watchdog timing
are treated as acceptance evidence.

## Interpretation

- **Cycle rate.** Both 1 kHz and 4 kHz cadences hold within 0.6% of
  nominal under a 4 s window. `stress.sh` additionally gates cadence
  under a 6-way ctl flood (>= 50% of nominal retained).
- **Swap pause.** The swap commits at a task boundary; worst-case
  release lateness did not increase across the swap on the 4 kHz task.
  The synchronous ctl call (~12 ms) covers module open + load +
  migrate + commit wait and is not the scan pause.
- **Watchdog.** Detection latency is `hang-timeout-ms` + <= 50 ms
  monitor poll (observed 350 ms at a 300 ms budget). The fault flag +
  `TPLC_IMG_PLC_FAULT` commit happen in the same monitor iteration —
  that is the master's "outputs to safe state" signal. If the stall
  does not recover (uncooperative task), the process `_exit(2)`s at
  ~2x timeout so a supervisor can respawn.

## Bug found and fixed by this measurement

The first watchdog run exposed dead escalation code: once the monitor
set `PS_FAULT`, its `!= PS_RUNNING` branch reset `stale_since` every
pass, so `stale_ms` could never reach the 2x-timeout `_exit` threshold —
a permanently wedged task would spin at 100% CPU forever. Fixed by
tracking `fault_since` separately; verified the process now exits at
~2x timeout (`tests/perf/hang_spin.c` + the evidence script).

## Structural targets (not measured here)

| target | where covered |
|---|---|
| image <= 64 KiB per area | `tplc-image` enforces; image spec |
| <= 32 tasks | `Scheduler` capacity bound |
| migration failure detection 100% | compile-time MIGD + `state_diff`; `online_change` tests |
| sim determinism | `stress.sh` sim-parity section; sim docs |
