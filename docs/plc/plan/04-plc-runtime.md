# 04 — `tether-plc`: the PLC Runtime Process

> **Status:** work-item document; deleted when `docs/plc/runtime-process.md`
> exists and the runtime + conformance suite implement this spec.
>
> **Satisfies:** G2, G3, G4 (swap machinery), G6 (fault wall, supervisor),
> G8 (symmap loading, watch plumbing), D1/D2/D11.
> **Depends on:** 01 (package), 02 (ABI), 03 (image).
> **Informs:** 08 (protocol detail), 09, 12, 14, 15, 16.

---

## 1. Role and shape

`tether-plc` is a small realtime process that:

- loads `.tplc` packages (`dlopen` the contained `target_obj`),
- hosts one or more **program instances** (each = descriptor + state blob +
  image bindings + retain),
- schedules their IEC tasks on a single RT scan thread,
- performs the image latch/commit protocol of doc 03,
- executes online change per doc 08 (snapshot → migrate → boundary swap),
- persists retain memory,
- serves the **control channel** (management + debug plumbing),
- dies and restarts without ever taking the fieldbus down.

It is built as **`libtplc-host` (embeddable runtime) + thin `tether-plc`
binary**. Embedders: the standalone binary (production, and simulation when
run as-a-process), the sim harness (in-process for unit tests), and the
conformance hosts of doc 02. All policy below lives in the library; the
binary adds CLI/config, privilege drop, and signal handling only.

```
tether-plc
├─ main/config/privdrop/logging            (binary only)
├─ libtplc-host
│   ├─ PackageLoader    (pkg verify → target select → dlopen → ABI checks)
│   ├─ ProgramInstance  (descriptor + blob + bindings + retain + symmap)
│   ├─ ScanThread       (scheduler: latch → bindings → task → commit)
│   ├─ ImageClient      (doc 03 protocol, latch/commit/heartbeat)
│   ├─ SwapEngine       (online-change staging + boundary commit)
│   ├─ RetainStore      (file-backed mmap + flush policy)
│   ├─ Watchdog/Monitor (budget checks + hang recovery hooks)
│   ├─ DebugAccess      (symmap → blob reads/writes at safe points)
│   └─ ControlServer    (UDS mgmt protocol; docs 12/16 hang services here)
└─ (host side, in Tether app: tether-plc-bridge — doc 03/09 — is separate)
```

## 2. Threading model

| thread | sched | role |
|---|---|---|
| main | OTHER | init → then runs ControlServer accept/dispatch (service plane) |
| scan | **SCHED_FIFO** (prio cfg, default 78 — below master's PDO 80) | all task execution, latch/commit, budget checks, swap commit |
| retain-flush | OTHER (idle prio) | dirty-hint + periodic retain fsync |
| monitor | OTHER | scan-progress supervision, heartbeat-independent hang detection |

Rules:

- Only `scan` is RT. Everything else is service plane.
- `scan` performs no syscalls except `clock_nanosleep(TIMER_ABSTIME)` for
  period waits and (implementation choice) eventfd for wake-on-change —
  image latch/commit are pure shared-memory ops.
- `dlopen`/`dlclose`, allocation, file I/O, package verification happen
  **only** on service threads — never on `scan`.
- All service→scan communication via single-producer atomic queues +
  atomic flags; scan→service via the same + counters. No mutexes taken by
  scan (bounded `try_lock` only where unavoidable — e.g. none by design).
- CPU affinity: scan pinned to configured isolated CPU (default: share
  master's non-PDO CPU); documented with Tether's `CpuIsolationConfig`.

### 2.1 Cooperative checkpoints

Because a hung user loop cannot be preempted safely, **codegen emits
checkpoint calls** (`tplc-rt::__ckpt(ctx)`) inside `WHILE`/`FOR`/`REPEAT`
loop back-edges and at FB call boundaries (doc 07 owns emission rules).
Checkpoint semantics (scan path, ~1–2 ns, predictable branch):

```
__ckpt:
   if (unlikely(ctx->abort_flag || ctx->budget_exceeded_flag))
        return-via-error-path → task returns TPLC_TASK_E_FAULT/OVERRUN
```

This gives: WCET budget enforcement **inside** runaway loops, cooperative
abort for stop/unload requests, and a guaranteed ceiling on "time to
notice a fault". The monitor thread remains the backstop for hangs that
avoid checkpoints (assembly, intrinsic bugs): escalate → process-level
restart (§7).

## 3. Scheduler (scan thread)

### 3.1 Model

- All `CYCLIC` tasks of all loaded programs share one timeline
  (per doc 02: single-threaded cooperative model, `EXT_MT_TASKS` off).
- Dispatch: **earliest-deadline-first within a scheduling round**, computed
  from `next_release` per task. Periods need not be harmonic — the
  scheduler handles arbitrary periods; overload policy below.
- `FREEWHEELING` tasks run in slack time when no cyclic task is due within
  a guard window (`freewheel_guard_ns`, default 50 µs); each invocation is
  checkpoint-bounded and yields after `freewheel_slice_ns` (default 200 µs).
- `STATUS` tasks are enqueued by runtime state transitions (program
  start/stop/fault); `EVENT` tasks arrive via `raise_event` (v1: declared,
  unimplemented — doc 02).
- Jitter is measured per task release and exported as counters (aligned
  with Tether's `RealtimeJitterMonitor` conventions).

### 3.2 Cycle sequence (per due task)

```
 1. ImageClient.latch_inputs()            // seqlock copy → task image_in
    - master-status/heartbeat policy first (doc 03 §6.3)
 2. DebugAccess.apply_input_forces()
 3. copy bound inputs image_in→blob      (io_bindings, §02.5)
 4. ctx.now_ns = clock(); cycle_no++; dt_ns = now - last_release
 5. rc = task_fn(&taskctx)               // checkpoints live inside
 6. if rc==0:
        copy bound outputs shadow→image_out
        DebugAccess.apply_output_forces()
        ImageClient.commit_outputs(owned_ranges)   // seqlock write
    else fault path §6
 7. record exec_time, jitter counters; budget check
 8. service pending SwapEngine commits at this boundary (§8)
```

Overrun policy (per task config):
- `skip` (default): if next release already passed when current finishes,
  count `overrun`, start immediately, do not catch up missed cycles.
- `catchup_limited`: run at most `catchup_max` missed cycles.
- `fault`: any overrun → task fault → program fault policy.
Budget enforcement uses `wcet_hint_ns` as soft alarm (counter+journal)
and `wcet_hard_ns` (runtime config or 2× hint) as checkpoint-abort.

## 4. Program instance lifecycle

States (per instance; reported over control channel; CiA 405-mappable —
doc 16 defines the exact mapping):

```
            deploy         load_ok
 EMPTY ───────────────► LOADING ────────► LOADED
                          │load_fail        │ prepare (dlopen+alloc+init)
                          ▼                 ▼
                        ERROR ◄──────── READY ◄────────────┐
                          ▲              │ start          │ stop/reset
                          │ fault        ▼                │
                          └────────── RUNNING             │
                            ▲          │ │swap-staged     │
                            └──────────┘ └─► SWAPPING ─► RUNNING
                                             (sub-state, commit/abort)
 unload from READY|STOPPED: → UNLOADING → EMPTY
```

- `prepare` = dlopen + ABI verify + blob/markers/task-local alloc +
  retain attach + `tplc_init`. Runs on service thread.
- `start` enqueues task activation → scan thread arms deadlines;
  first run gets `task_flags.first_run`.
- `stop` = graceful: pending-cycle completion → `task_flags.last_run` →
  deschedule → retain flush hint. `kill` = checkpoint-abort + immediate
  deschedule.
- `reset` = stop → re-zero non-retain blob → `tplc_init` again → READY
  (retain semantics per IEC warm/cold — config flag; cold reset also
  clears retain file).
- `SWAPPING` see §8. Any fault enters `ERROR` with `last_fault_code` and
  program-level policy (§6): `hold` (stay error, manual ack),
  `auto_restart` (re-init, bounded retries), `safe_unload`.

Multiple program instances: supported by design (per-instance blob,
bindings, retain file `retain/<program_uuid>.bin`). v1 deployment tooling
targets one; nothing below assumes singleton.

## 5. Retain store

- Region: `retain_region` inside the blob (doc 02) mirrored to
  `retain/<uuid>.bin` via a second mmap'd copy — **not** the blob itself
  mmap'd (the blob must stay RAM-fast and relocatable; the file is the
  persistence image written by the flusher thread).
- Flush triggers: `svc.retain_dirty` hints (batched), periodic
  `retain_flush_ms` (default 1000), on stop/shutdown/fault-transition.
  Flush = memcpy dirty ranges → file → `fdatasync`. Rate-limited;
  flusher thread never touches scan data in-flight — blob→file copies are
  taken at task boundaries via a scan-thread handoff (scan memcpys a
  dirty-marked snapshot into a staging buffer; flusher writes that).
  Bounded scan cost: ≤ a few µs per flush cycle (size of dirty set).
- File header: `{magic 'TRET', version, program_uuid, retain_layout_hash,
  generation, len, crc32}`. On start: if `program_uuid` matches AND
  `retain_layout_hash` matches current layout ⇒ restore +
  `ctx_flags.retain_restored`. Mismatch ⇒ defaults + structured alarm
  `TPLC_RETAIN_LAYOUT_LOST` (never silently mis-restore).
- Power-loss semantics: acknowledged best-effort (dirty window ≤
  `retain_flush_ms`); doc 15 records the honesty statement. True
  power-fail retain needs NVRAM/UPS — noted non-goal v1.

## 6. Fault handling

| fault | detection | action |
|---|---|---|
| task returns `<0` | scan | program fault policy (hold/auto_restart/safe_unload); `plc_status=fault`; no partial output commit |
| WCET hard overrun | checkpoint abort | same |
| checkpoint `abort_flag` | cooperative | graceful for stop/unload paths |
| scan-thread stall | monitor thread (progress counter delta) | `plc_status=fault` + request cooperative abort; if unrecovered in `hang_timeout_ms` (default 2000) → self-terminate for supervisor respawn |
| dlopen/init/migrate fail | service | program stays ERROR; old program (if swapping) unaffected |
| segfault/other signal | process dies | master detects hb stale → safe-state (doc 03); supervisor restarts (§7) |

Error taxonomy: every transition emits `{program_uuid, code, subcode,
task, cycle_no, context}` to journal + control channel.

## 7. Process supervision & restart

- Supervisor = the Tether application (bridge) by default; systemd unit as
  alternative. Contract: supervisor spawns `tether-plc --instance <id>`,
  monitors `plc_hb`, applies respawn policy (`always`/`on-fault`/`never`,
  backoff `restart_delay_ms`, max rate `3/60s`).
- On respawn: reopen image (epoch check), reattach retain files, reload
  program set from `runbook/` (persisted list of deployed packages +
  desired states), resume per policy `auto_resume` (default: programs
  that were RUNNING are re-prepared and started; a program faulted at
  death is restored to ERROR not RUNNING — honesty about state).
- `runbook/` is the recovery source of truth: `{program_uuid →
  {package_path-or-hash, desired_state, config}}`, written atomically on
  every deploy/state change.

## 8. Online change mechanics (runtime side; protocol in doc 08)

Service thread stages; scan thread commits at a task boundary:

```
service: parse pkg → verify → select target → dlopen → descriptor checks
         → alloc new blob (+task_local, markers shadow) → parse symmap
         if program_uuid matches RUNNING/READY program:
             if layout_hash equal: stage "module-swap" (no blob work)
             else: stage "migrate" — request snapshot
scan (boundary): copy running blob → snapshot buffer (pause ≈ memcpy)
                 continue running OLD program (still old code!)
service: call tplc_migrate(snapshot → new_blob)   [off-cycle]
         on OK/OK_DEGRADED → arm swap; on DENY/ERR → abort, report
scan (next boundary): pointer-swap {descriptor, ctx.blob, task table,
         symmap} → run new code with migrated blob; set resume flag
service (later): dlclose old module after quarantine period (see below)
```

- **Pause budget**: only the blob memcpy runs paused — target ≤500 µs at
  256 KiB (measured §12). Everything else off-cycle.
- **dlclose quarantine**: old module is unloaded only after the swap
  survives `quarantine_cycles` (default 10) — enables instant
  **rollback**: keep old module+blob warm; a control `rollback` op or an
  auto-rule (new program faults within N cycles) restores it at a
  boundary. Rollback window bounded (retain divergence note in doc 08).
- Failed swap never leaves zombie code: staging state machine is
  `STAGED→MIGRATING→ARMED→COMMITTED|ABORTED`, all on service thread,
  committed by scan atomically.

## 9. Control channel

- Transport: Unix domain socket `…/tether/plc/<id>/ctl.sock`, framed
  TLV messages `{u16 type, u16 flags, u32 len, u64 req_id, payload}` —
  deliberately NOT the fieldbus SHM (doc 03). Also reachable via the
  master's `tether_io` bridge which proxies a curated subset to the web UI.
- v1 op groups (schemas in doc 16 for mgmt, doc 12 for debug):
  - `pkg.deploy` (stream package), `pkg.list`, `pkg.info`
  - `prog.prepare/start/stop/reset/kill/unload/rollback`
  - `prog.status` (+subscribe), `task.stats`
  - `dbg.watch` ops, `dbg.force` ops (plumbing; semantics doc 12)
  - `image.stats`, `retain.status/flush/clear`
  - `runtime.status/config.get/logs.subscribe`
- AuthZ: socket peer creds (SCM_CREDENTIALS) + role mapping; the bridge
  enforces UI-facing policy (doc 15). Control channel itself is root/
  tether-group only — defense in depth.
- All ops are service-plane: they never block scan; long ops (deploy)
  stream chunks and ack progress.

## 10. Config and CLI

```
tether-plc --instance NAME --config FILE
  --image PATH         (default /dev/shm/tether/plc/<instance>/image)
  --ctl PATH           (default …/ctl.sock)
  --runbook DIR        (default /var/lib/tether/plc/<instance>/runbook)
  --retain DIR         (default /var/lib/tether/plc/<instance>/retain)
  --cpu N  --prio N    (default -1/78)
  --user USER --group GROUP   (privdrop)
  --no-sandbox         (doc 15; sandbox on by default)
```

TOML/INI-equivalent config mirrors every flag; environment overrides for
containers. Startup validation prints effective config (the "no silent
defaults" rule).

## 11. Diagnostics surface

Counters per program/task/runtime (exported over `runtime.status` +
bridge→`tether_io`): task `{runs, overruns, exec_ns min/avg/max, jitter}`,
image `{latches, commits, retries, epochs}`, swap `{staged, committed,
aborted, rollback}`, retain `{flushes, dirty_bytes, restores, losses}`,
runtime `{faults by code, respawns}`. Log: structured, leveled, drain via
control subscription; RT path emits only packed records into a lock-free
ring (svc.log semantics, doc 02).

## 12. Sim parity

- `libtplc-host` takes `ImageBackend` (SHM | virtual) + `ClockSource`
  (monotonic | virtual) at construction. The sim harness embeds the
  library in-process with virtual backends feeding identical semantics —
  including fault injection (master-dead simulation = heartbeat control).
- Standalone binary in "sim attach" mode can also run against a
  sim-published SHM file — the same layout (doc 03) written by a
  simulated master. Same artifact, same runtime, same protocol: G7.

## 13. Iteration-2 hooks

- `STATUS`/`EVENT` task plumbing already in scheduler model.
- SFC: `SwapEngine` generalizes to per-POU hot-swap later; the boundary
  commit mechanism is unchanged.
- Multi-thread tasks: `EXT_MT_TASKS` — scheduler shape (deadline queue)
  survives; dispatch becomes per-thread steal. No data-structure
  commitments leak.
- Runbook/runbook-format versioning reserved for multi-program fleets.

## 14. Error codes (owned by this chapter)

`TPLC_E_RT_*` family: `_RT_PKG (deploy errors reuse doc-01 codes),
_RT_PROGRAM_STATE (illegal transition), _RT_TASK_STATE, _RT_OVERLOAD,
_RT_HANG, _RT_SWAP_*, _RT_RETAIN_*, _RT_CTL_*, _RT_RUNBOOK_*` plus events
`TPLC_RT_RESPAWNED, _RT_PROGRAM_FAULT, _RT_ROLLBACK`.

## 15. Acceptance criteria

1. Runtime loads/runs/unloads N program instances against the doc-03
   image; single-RT-thread scheduling with measured jitter within target.
2. Full lifecycle (deploy→start→stop→reset→unload; kill; fault policies;
   auto_restart) covered by state-machine tests incl. illegal transitions.
3. Online change: staged migrate + boundary swap meets pause budget
   (≤500 µs @256 KiB blob); rollback within window restores old program;
   DENY path leaves running program untouched.
4. Retain: dirty-window flush survives kill -9 with ≤configured loss;
   layout-mismatch refuses restore with the dedicated alarm.
5. Supervisor: kill -9 → master safe-state → respawn → auto-resume with
   correct prior-state semantics (RUNNING→RUNNING, ERROR→ERROR).
6. Control channel: full op surface functional + authZ; no op measurably
   delays scan beyond one boundary (jitter accounting proves).
7. Sim: same `.tplc` runs under virtual backends with identical traces.
8. Checkpoint mechanism: a `WHILE TRUE` program is aborted within
   configured bound, not by luck — verified under test.

---

## Appendix I — Control-channel wire protocol (normative)

Transport: SOCK_STREAM UDS. Frame (all LE):

```
+0  u16   msg_type      (op id, tables below)
+2  u16   flags         (bit0: more-frames-follow [chunked],
                         bit1: is-event [unsolicited],
                         bit2: cancel-requested)
+4  u32   payload_len   (≤ 8 MiB default cap)
+8  u64   req_id        (client-chosen; responses+events echo)
+16 ...   payload       (schema per op; CBOR? NO — compact TLV schema
                         per op, defined below as {field:type} lists;
                         strings = u16 len + utf8, bytes = u32 len + raw,
                         uuid = 16 B, hash = 32 B)
```

Session bootstrap:

```
client → S_HELLO{proto_ver, client_name, caps}
server → S_HELLO_ACK{proto_ver, runtime_ver, caps, session_id,
                     authz_level}
        (peer creds via SCM_CREDENTIALS determine authz_level;
         mismatch → close)
```

### I.1 Op tables (msg_type ranges)

| range | group | ops |
|---|---|---|
| 0x0100 | session | hello, hello_ack, ping, set_notify_mask |
| 0x0200 | deploy | `begin{size,sha256,mode,program_uuid_hint?}` `chunk{upload_id,off,data}` `commit{upload_id}` `abort{upload_id}` `status{program_uuid?}` `pkg.list` `pkg.info{package_uuid}` |
| 0x0300 | program | `prepare{package_uuid,map_epoch}` `start{program_uuid}` `stop{program_uuid}` `kill` `reset{warm|cold}` `unload` `rollback` `status` `list` `subscribe` |
| 0x0400 | task | `stats{program_uuid}` `suspend{name}` `resume` `step{name,n}` |
| 0x0500 | debug | `resolve{paths[]}` `watch.add/get/remove/stream` `write{path,value}` `force{path,mask,value}` `force.clear{_all}` `break_when{task,expr}` `state{program}` `diff{old,new}` `trace.config/start/stop/export` |
| 0x0600 | retain | `status` `flush` `clear` `dump` |
| 0x0700 | image | `stats` `channels` (read-only map view) |
| 0x0800 | map | `get` `verify{hash}` `set{payload}` (map deploy, gated) |
| 0x0900 | runtime | `status` `config.get` `logs.subscribe{level}` `counters` `shutdown{mode}` |
| 0x0A00 | journal | `tail{cursor,n}` `subscribe{filter}` |
| 0x0E00 | events | unsolicited: `prog.transition`, `task.overrun`, `fault`, `swap.stage`, `force.changed`, `retain.flushed`, `image.peer_lost` |

### I.2 Selected payload schemas

`deploy.begin` → `{size:u64, sha256:bytes32, mode:u8(0 cold 1 oc 2 auto),
program_uuid:uuid}` ⇒ `{upload_id:u64, max_chunk:u32}`
`deploy.commit` ⇒ `{result:u8, package_uuid, program_uuid,
staged:u8(0 none 1 module-swap 2 migrate 3 cold),
deny_report?:{rules:[{kind,severity,old,new,aux}]}}`
`prog.status` ⇒ doc-16 §3.3 object encoded as TLV.
`dbg.resolve` ⇒ `{entries:[{path:str, idx:u32, region:u8, off:u64,
size:u32, type:u32, access:u8, stale:u8}], errors:[{path, code}]}`
`watch.stream` (event frames) ⇒ `{handle, seq:u64, stamp_ns:u64,
values:[{idx, len, bytes}]}`
`force` ⇒ `{applied_idx:u32}` (denied → error frame w/ `FORCE_DENIED`)
`trace.export` (chunked) ⇒ `{trace_id, seq, channels:[{idx,type}],
samples:[{cycle:u64, stamp:u64, vals:[bytes]}]}`

### I.3 Semantics

- One outstanding `chunked` op per session per group; cancel via
  flags.cancel.
- Events flow only after `set_notify_mask` subscribes categories.
- Every request → exactly one response frame (or error frame
  `{code:u32, subcode:u32, msg:str}`); streaming ops → response then
  event frames until end/cancel.
- Versioning: `proto_ver` negotiation; unknown msg_type →
  `TPLC_RT_CTL_UNKNOWN_OP` error frame (forward-tolerant).
- Idempotency: `req_id` dedup window (last 4096 per session) —
  retries of commit-type ops are safe.
