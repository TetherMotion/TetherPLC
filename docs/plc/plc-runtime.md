# PLC runtime process — as-built

Status: as-built.

> `doc NN` section references below are provenance to the retired
> plan chapters; this document is authoritative (see
> `architecture.md` §7).

## Process

`tether-plc --image <shm> --module <so> | --package <.tplc>
[--ctl <uds>|none] [--retain DIR] [--retain-flush-ms N] [--sim]
[--sig-policy open|require|require-trusted] [--trust-dir DIR]
[--no-sandbox] [--authz FILE] [--audit-log FILE]`

- Attaches to the SHM process image (plc role), validates size.
- `--package` extracts the `target_obj` section to a temp `.so`, then
  the normal `dlopen` path + descriptor validation.
- Single-threaded cooperative scan: `Scheduler::tick` dispatches due
  tasks in priority order inside the main loop; latch/commit hooks go
  through the seqlock image protocol.
- `--ctl none` runs headless (fault injection / tests).

## Containment, provenance, access control (doc 15)

- **Sandbox (§3)** — on by default: rlimits (NOFILE/FSIZE/MEMLOCK/
  CORE), `PR_SET_NO_NEW_PRIVS`, and a libseccomp filter (~150
  syscalls whitelisted; `socket()` gated to AF_UNIX so the process
  speaks UDS only). `--no-sandbox` is the debug escape — warned on
  stderr and journaled. Verified by `sandbox_enforce` (probe:
  AF_INET → SIGSYS, AF_UNIX → pass) and `Seccomp: 2` in /proc.
- **Signature policy (§4)** — `--sig-policy` + `--trust-dir` at
  every package load path (boot/prepare/swap); see toolchain.md.
- **Post-link scanner (§4.3)** — ELF64 scan inside `Module::open`
  before `dlopen`: kernel-gate opcodes, RWX/exec-stack/PT_INTERP,
  DT_NEEDED + import whitelists.
- **Access control (§5)** — UDS `SO_PEERCRED` binds each session to
  a kernel-attested uid at accept; `--authz FILE` maps
  `<uid> <role>` (`observer`/`engineer`/`commissioner`). Defaults:
  server euid or root → commissioner, other uids → observer.
  observer = read ops only; engineer adds watch-set mutations;
  commissioner owns deploy/lifecycle/suspend/write/force/break/
  trace-arm/retain/shutdown. Denied ops get `TPLC_E_CTL_AUTHZ`
  (401); mutating ops are rate-limited at 20/s (burst 40) per
  **uid** — reconnecting does not reset the budget — excess gets
  `TPLC_E_DBG_RATE` (413). HELLO ack reports the bound role.
- **Audit journal (§5)** — `--audit-log FILE` (default
  `<runbook|retain|cwd>/audit.log`): append-only, fsync per record,
  `ts uid gid pid sess name op args ahash rc` per mutating op —
  including denied ones (`rc=-401/-413`) and the `--no-sandbox`
  boot flag. Verified by `authz_audit`.

## Control channel (0x0— ops implemented)

Session: `HELLO`, `PING`, `SET_NOTIFY`. Deploy: `DEPLOY_BEGIN`,
`DEPLOY_CHUNK`, `DEPLOY_COMMIT`, `DEPLOY_ABORT`, `PKG_LIST`,
`PKG_INFO` (package store under `--packages`; sha256 verified on
commit). Program: `PROG_PREPARE` (by package uuid), `PROG_START`,
`PROG_STOP`, `PROG_KILL` (deschedule, no fault), `PROG_RESET`
(non-faulted only), `PROG_UNLOAD`, `PROG_STATUS`, `PROG_LIST`,
`PROG_LAYOUT`, `PROG_SWAP`, `PROG_ROLLBACK`. `TASK_STATS`,
`IMAGE_STATS`, `RETAIN_STATUS`, `RETAIN_FLUSH`, `RETAIN_CLEAR`,
`RETAIN_DUMP`, `RT_STATUS`, `RT_SHUTDOWN`, plus
`EV_PROG_TRANSITION`/`EV_FAULT` notify events.

## Retain store (plc-runtime.md §5)

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

## Watchdog & budget (plc-runtime.md §3.2/§6)

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

The master-side half of process-image.md/09 as a reusable library plus a dev
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

## EtherCAT fleet integration (`runtime/ecpump`, `tplc-ec-pump`)

`tplc-ec-pump` is a master-side transport for the bridge's `sim`
simdir contract, proven against the emulated Beckhoff fleet in the
sibling Tether checkout (`examples/beckhoff_slave_chain` +
`examples/ethercat_fleet_probe` helpers):

- Connects as EtherCAT master over `UdpEthercatLink` — UDP datagrams
  carry whole Ethernet frames, so no CAP_NET_RAW/kernel interface is
  needed. Runs `FleetMaster::bringUp` (discovery → mailbox → PREOP →
  SM/PDO → SAFEOP → OP) against `fleetInfos()` (2×EK1100 + EL1004/
  EL2004/EL3104/EL4104 per coupler), verifies SII identities, then
  pumps at `--hz`: `<key>.rx` file → RxPDO entry storage →
  `exchangeAll()` → TxPDO entry storage → `<key>.tx` file (atomic
  rename). The bridge's `in`/`out` channels key on the same filenames,
  e.g. `in off=0 size=1 key=el1004.c1.tx`.
- Exposes the master-side PDO views over tether_io (`--io-port`):
  signals `<key>.di<g>`/`.ai<c>` (TxPDO, slave→master) and
  `.do<g>`/`.ao<c>` (RxPDO, master→slave) plus `link`/`cycles`/
  `errors` — the TetherPLC-side observation surface.
- `tplc-io-cli <port> list|get|set` is a generic tether_io client used
  by the test to stimulate slave params and read both endpoints.

`tests/ec/fleet_e2e.sh` (ctest `ec_fleet_e2e`, skips without a Tether
build) runs the whole chain: slave-chain + pump + bridge/PLC, then
drives DI/AI stimuli through the slave's IO params and asserts DO/AO
responses on the slave's IO signals — digital pass-through across
couplers and analog arithmetic (%I→PLC→%Q) on both EK1100 segments.

## Simulation (doc 14)

- `sim_time_ns` in the image header (carved from reserved) — master
  publishes, `tether-plc --sim` sources `ctx->now_ns` +
  `svc->steady_ns` from it (HostServices override), so all program
  time is virtual while heartbeats stay real.
- `tether-plc-sim` = L2 process-sim: creates the image, spawns the
  real binary with `--sim`, runs a step file
  (`step N time=<ns> [in off=hex..] [expect off=hex..]
  [expectf off=hex..]`), waits for the next committed scan per step,
  diffs %Q — verified with a TON vector at 200/600/700 ms crossings;
  divergence prints step+offset. `expect` compares process-image %Q
  bytes, `expectf` reads program-blob bytes through the ctl channel
  (`DBG_READ`, needs `--ctl`). `--scan` suspends the task and runs
  exactly one ctl `TASK_STEP` per step, so step files get the same
  cycle-per-step semantics as the L1 interpreter.
- L1 = `tplcc vectors` (interp, same canonical IR).
  `tests/sim/vector_run.py` replays the oracle corpus
  (`tools/oracle/corpus/vectors.jsonl`) at both levels: L1 direct,
  L2 by compiling each program, resolving `expect_fields` names to
  blob offsets via SMAP/TYPC, and driving `tether-plc-sim --scan`.
  L3 lives in the Tether app's simulated-plant fleet — not this repo.

## Map validator (io-mapping.md §2.2)

- Map schema v1: `image`/`in`/`out` channels + `type=` (bool/u8…f64),
  `safe=` policy, `bind <at> <key|*>` program bindings, and `safety
  <key>` endpoint classification.
- `validate_map` reports every violation with a stable
  `TPLC_E_MAP_*` code: RANGE (channel outside image), OWNER (%Q
  overlap), TYPE (view wider than range), SAFE (missing policy on a
  %Q channel), SAFETY (classified endpoint bound to %Q), AT (fixed
  AT outside any channel), BIND (missing channel key). The bridge
  refuses to start on any violation; parse is permissive so all
  violations surface in one pass.

## Online change (online-change.md)

- `PROG_SWAP {uuid|path}` stages the new module first; live
  `layout_hash` is compared to the package's `MIGD`
  `predecessor_hash` — equal to the *new* hash takes the module-swap
  fast path (no migration), equal to the predecessor hash runs a
  migrated swap, anything else replies `SWAP_RACE` with both hashes
  and leaves the old program untouched.
- Migrated swap: snapshot old blob at a task boundary, run
  `tplc_migrate` off-cycle into the new program's blob, atomic
  program/module/context swap, `tplc_init` skipped (state already
  migrated). Old program + snapshot stay quarantined for
  `--quarantine-cycles` scans; `PROG_ROLLBACK` inside the window
  restores them, outside it reports no quarantined program.
- Module ownership uses `unique_ptr` (program/task ctxs carry
  back-pointers — a moved `optional<Module>` left them dangling).
- Verified live via `tests/online_change/e2e.sh` (CTest
  `online_change_e2e`, exit 77 = skip without tplcc): state copy,
  default-init, module-swap, SWAP_RACE, old-program survival,
  rollback, compile-time deny.

## Not yet

- Multi-program scheduling (descriptor supports it; driver is
  single-program).
- ControlProxy curated ctl client in the bridge (ops work via
  `tplc-ctl` meanwhile), real EtherCAT/Modbus transports (sim only),
  overrun-policy config (skip is hardcoded), freewheel guard/slice
  enforcement.
