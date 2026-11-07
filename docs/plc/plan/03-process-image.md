# 03 — Process Image: the SHM Exchange Protocol

> **Status:** work-item document; deleted when `docs/plc/process-image.md`
> exists and `tether-plc-bridge` (master side) + `runtime/image` (PLC side)
> implement it with the conformance suite green.
>
> **Satisfies:** G5 (image decoupling), G6 (crash isolation — seqlock is
> crash-safe, no locks), G2 (bounded ~1 µs copies), D1/D11 (sole coupling).
> **Depends on:** 02 (context/io_bindings). **Informs:** 04, 09, 12, 15.

---

## 1. Purpose

The SHM process image is the **only** coupling between the Tether application
(which owns the fieldbus) and the `tether-plc` runtime process (which owns
compiled user logic). Requirements:

1. Wait-free for both writers; bounded-retry for readers; **no locks** — a
   crash on either side must never block or corrupt the other.
2. Single-writer per direction (SPSC): master writes `%I`, PLC runtime
   writes `%Q`. Seqlock guards each direction independently.
3. Bounded cost: full image copy ≤ a few µs at ≤64 KiB.
4. Mutual liveness detection (heartbeats) + a defined **safe-state** path.
5. Multi-program capable: several program instances share one `%I`/`%Q`
   space under non-overlapping write ownership.
6. Debuggable: header carries exchange counters; the whole file is
   inspectable (`hexdump`, `tplc-image dump`).

Non-goal: `%M` markers and retain are **not** in the SHM (§3.3) — only the
two physical directions `%I`/`%Q` cross the process boundary.

## 2. Channel map (what lives where)

| data | transport | owner |
|---|---|---|
| `%I` inputs (bus→program) | SHM seqlock area | master writes, PLC latches |
| `%Q` outputs (program→bus) | SHM seqlock area | PLC writes, master latches |
| `%M` markers | runtime-private memory (per program instance) | PLC; watch/force via service plane |
| retain | file-backed mmap inside runtime (doc 04) | PLC |
| program variables / blob | runtime-private | PLC; watch via service plane |
| forces, watchlist, control, download | **control channel** — separate UDS `tether-plc` socket speaking the management protocol (docs 04/12/16); *never* SHM | service plane |
| io mapping table | config file + control channel distribution (doc 09) | service plane |

SHM is **data-only**: no control, no pointers, no locks. Control channel
design is doc 04/16 scope; this chapter owns the data plane.

## 3. SHM file layout

Path: `/dev/shm/tether/plc/<machine_id>/image` (Linux `shm_open`; created by
the master, mode `0660` group `tether`). One file per machine instance;
partitions for multiple programs are *ranges*, not files.

```
offset  size      field
0       8         magic 'TPLCIMAG'
8       u32       version (=1)
12      u32       header_size (=256)
16      u32       layout_flags   (bit0: BE? must be 0 on v1 hosts)
20      u32       _rsvd
24      u64       image_epoch    (incremented on each master re-init)
32      u64       in_seq         seqlock for %I (master-side writer)
40      u64       out_seq        seqlock for %Q (PLC-side writer)
48      u64       master_hb_ns   master heartbeat (monotonic)
56      u64       plc_hb_ns      runtime heartbeat
64      u32       master_status  (0=init 1=run 2=degraded 3=stopping)
68      u32       plc_status     (0=open 1=run 2=fault 3=stopped 4=safe)
72      u64       in_off         (absolute offset of %I area)
80      u64       in_size
88      u64       out_off        (absolute offset of %Q area)
96      u64       out_size
104     u64       in_writes      master write counter
112     u64       out_writes     plc write counter
120     u64       in_retries     latch retry counter (plc side)
128     u64       out_retries    latch retry counter (master side)
136     u64       last_fault_code (plc writes; master reads)
144     112       reserved (expansion: per-program status table, etc.)
256     ...       %I area  [in_size]    (aligned 64)
...     ...       %Q area  [out_size]   (aligned 64)
```

All fields `_Atomic`/accessed with explicit memory order per §4.
`image_epoch` lets the runtime detect a master restart mid-operation:
readers compare epoch before/after a latch; mismatch = resync (§7.3).

### 3.1 Sizes

- `in_size`/`out_size` are master-chosen (from IO config + slack, doc 09);
  runtime verifies `≥ descriptor.image_in/out_bytes` of every loaded program,
  else refuses start (`TPLC_E_IMG_TOO_SMALL`).
- v1 caps: 64 KiB each. Alignment: areas 64 B-aligned; counters/seq fields on
  separate cache lines where feasible (header is r/o-heavy vs areas
  write-heavy — acceptable v1; note NUMA/false-sharing review TODO in §12).

### 3.2 Why `%M`/retain are excluded

`%M` is *program-internal* memory: putting it in SHM buys nothing (the master
never reads it; watch/force already travel the control channel), costs shared
surface, and would couple crash domains. Retain likewise is a runtime-local
persistence concern (doc 04). SHM = physical-IO plane only. (DESIGN-GOALS
mentions %M inside the "process image" *memory model*; this chapter is the
physical realization — the distinction is deliberate.)

## 4. Seqlock protocol (normative)

Writer = exactly one thread on each seq. Reader = one thread (bounded spin).

```
writer (master on in_seq; plc on out_seq):
  seq.fetch_add(1, relaxed)                    // now odd = "write in progress"
  atomic_thread_fence(release-ish → acq_rel is fine via seq_cst ops)
  <write payload bytes>
  seq.fetch_add(1, release)                    // even = stable

reader:
  do:
    s1 = seq.load(acquire); if (s1 & 1) continue/spin
    <copy payload>
    s2 = seq.load(acquire)                     // or seq_cst for rigor
    tries++
  while (s1 != s2 && tries < MAX_TRIES)
  success iff s1==s2 && !(s1&1)
```

- `MAX_TRIES`: reader-bound — copy duration vs writer rate guarantees ≤2
  retries in practice; `MAX_TRIES = 8`, counter `*_retries` records; exceeding
  = `TPLC_E_IMG_SEQ_EXHAUSTED` (design bug or writer stalled; alarm).
- Payload copies use `memcpy` on naturally-aligned buffers; no partial-field
  atomic requirements — the seqlock *is* the consistency primitive.
- Memory ordering: seq ops at least acq_rel as written; payload itself is
  relaxed — seq acquire/release pairs order it.
- Writers never wait → both RT paths stay wait-free.
- Crash semantics: a writer dying mid-write leaves seq **odd** permanently →
  readers see liveness gone via heartbeat (§6) and treat data as
  last-good/unavailable per policy — they do *not* spin forever (spin
  bounded by MAX_TRIES then "writer-dead" path).

## 5. Cyclic sequencing

### 5.1 Master side (in its existing PDO cycle, doc 09 owns scheduling detail)

```
per bus cycle:
 1. PDO inputs → staging
 2. seqlock-write staging → %I area      (in_writes++)
 3. refresh master_hb_ns
 4. seqlock-read  %Q area → PDO-out staging
    if plc_status∈{fault,safe} or plc_hb stale:
        substitute safe_values per channel map (§6.2)
 5. PDO-out staging → slaves
```

Order matters: %I publish precedes %Q read in the same cycle, so a
1-cycle-old output is the worst case (PLC may run on a slightly different
phase — accepted; phase alignment knobs in doc 09).

### 5.2 PLC runtime side (per program task due; doc 04 owns scheduling)

```
per task cycle:
 1. seqlock-read %I → task's private image_in   (latch; in_retries += n)
    if master_status∉{run,degraded} or master_hb stale:
        apply input-loss policy (§6.3)
 2. apply input force overlays                 (control-channel table)
 3. copy bound inputs: image_in→blob shadows   (io_bindings)
 4. tplc_task_run
 5. on ok: shadow→image_out (bound outputs);
           apply output force overlays;
           seqlock-write owned %Q ranges → SHM   (out_writes++)
    on fault: §6.4
 6. refresh plc_hb_ns
```

### 5.3 Multi-task / multi-program rules

- **Latching is per-task at task start** — each task sees a consistent
  frozen input set (IEC process-image semantics at task granularity).
- **Output commit is per-task at task end**, restricted to the ranges that
  task *owns*. The IO mapping (doc 09) enforces: every `%Q` byte has exactly
  one writer task across all loaded programs; `%I` bytes may be read by
  many. Violation = config error `TPLC_E_MAP_OWNER` at load.
- Two program instances in one runtime share the SHM file and partition
  `%Q`; each latches the full `%I` (cheap) or its mapped subset
  (implementation choice; spec'd as "full latch" for v1 simplicity —
  revisit if image grows past 64 KiB).
- `%Q` bytes with **no owner** are written by the master as its own
  pass-through/default values — ownership table is total.

## 6. Failure and safe-state model

### 6.1 Liveness

- Each side stamps its heartbeat every cycle (monotonic ns).
- `stale_after` = 3× the counterpart's declared cycle (configured per side,
  in mapping config): master computes staleness of `plc_hb`, PLC of
  `master_hb`. Both also read the other's `*_status` for explicit states —
  heartbeat covers silent death, status covers announced degradation.

### 6.2 PLC fault → master safe-state

On `plc_status∈{fault}` OR stale `plc_hb` OR odd `out_seq` persisting:

- Master substitutes, per %Q channel: `safe_value | last_value | zero`
  (per-channel config in the IO map, doc 09) into PDO staging.
- Sets `master_status=degraded`, raises `TPLC_IMG_PLC_LOST` event.
- Continues publishing %I (so a recovered/rel restarted runtime resumes
  cleanly — recovery protocol doc 04).
- FSoE channels are **never** owned by %Q writes from user logic (G11):
  safety outputs bypass this path entirely under the certified config;
  safe-state config cannot make a safety channel "safe" — that belongs to
  the safety layer. Mapping validation forbids %Q↔safety-PDO binding.

### 6.3 Master fault/stop → PLC input-loss policy

On stale `master_hb` or `master_status==stopping`, per program config:
- `hold_last` (default): keep running on frozen `%I` — bounded: after
  `hold_max` cycles (config, default 1000) transition own status to
  `fault` (self-safe-state).
- `zero_inputs`: latch zeroed image.
- `stop_tasks`: orderly program stop.
Rationale: physical outputs driven by a dead master aren't leaving the
bus anyway; the policy governs program behavior for restart smoothness.

### 6.4 Task fault mid-cycle

Program fault protocol (doc 02 §4.6): runtime sets `plc_status=fault`,
writes `last_fault_code`, commits *no* partial outputs (shadows are
private until commit — torn output frames are impossible by construction).
Faulted program stops committing; its %Q ranges go stale → master
safe-state per §6.2. Other programs keep running (per-program status
sub-table expansion noted in header reserved space — v1: one `plc_status`
covers "worst of" all programs + per-program detail over control channel).

## 7. Lifecycle

### 7.1 Bring-up

1. Master creates file, sizes areas, writes header (epoch++, status=init),
   publishes path via control/config.
2. `tether-plc` opens (retries while absent — master may start later),
   validates magic/version/sizes, maps `PROT_READ|PROT_WRITE`, sets
   `plc_status=open`.
3. Master goes `status=run` when slaves reach OP and first exchange done.
4. Runtime starts scheduling only while `master_status==run|degraded` and
   program state permits (doc 04 state machine).

### 7.2 Teardown

- PLC orderly exit: `plc_status=stopped`, flush plc_hb, munmap. Master
  detects next cycle → safe-state for owned ranges.
- Master shutdown: `master_status=stopping`, %I stops; PLC policy §6.3.
- File unlink: master only, after PLC gone (or tolerant on stale fd —
  reopened next start).

### 7.3 Resync / epoch

Any observed `image_epoch` change mid-read ⇒ reader discards the attempt
and restarts; a program start validates epoch once at attach and treats a
later change as master-restart (input policy §6.3 + revalidate mapping).

## 8. Debuggability

- `tplc-image dump <path>` (tool): decode header, hex areas, show seq
  parity, heartbeats, counters — usable on a live system (read-only).
- Counters (`*_writes`, `*_retries`, `last_fault_code`) are exported into
  the `tether_io` catalog by the bridge (doc 09) → UI dashboards free.
- Watch-the-image: the service plane may `mmap` read-only the same file
  for raw channel monitors — readers obey the same seqlock (read path is
  already multi-reader safe; writers unchanged).
- Recording: image latched snapshots are the boundary where the
  scope/trace subsystem hooks (doc 12) — reproducible because latched.

## 9. Security notes

- `/dev/shm` file `0660 root:tether` (or configured user/group); the PLC
  runtime runs unprivileged in that group (doc 15 sandboxing).
- No pointers/offsets beyond fixed header fields are shared — no pointer
  confusion. Sizes are validated on open and on every epoch change.
- A compromised PLC process can corrupt `%Q` and heartbeats — accepted
  (it owns outputs anyway); it *cannot* write `%I` (read-only side is
  convention only — same file ⇒ same permissions; **limitation noted**:
  integrity of %I vs a hostile PLC relies on seqlock semantics + master
  sanity checks, not memory protection. If hard write-protection for %I is
  later required: split into two shm files with opposing permissions —
  designed-for, not v1).

## 10. Alternatives considered (record)

| option | verdict |
|---|---|
| mq/ring buffer per byte-range | adds queuing semantics we don't want (image is sampled, not queued) |
| double-buffer swap index | viable; seqlock simpler for single-writer and equal boundedness; kept as abi/internal alternative |
| eventfd/futex wakeups | coupling + syscall on RT path; polling at cycle rate is free |
| memfd + SCM_RIGHTS | nicer lifecycle; plain shm_open chosen for inspectability (`dump` tool, debug by hand) — memfd noted as alternate transport of the same layout |
| two files opposing perms | deferred per §9 |

## 11. Iteration-2 hooks

- Reserved header space for per-program status table (multi-invocation
  health at a glance).
- `STATUS`/`EVENT` task wakeups may use a lightweight event word in the
  reserved header range rather than a new transport (SFC needs cheap
  transition triggers; eventfd stays the documented fallback).
- LD/SFC introduce no new image requirements — they consume the same
  latch/commit path.
- MCU path: the *latch/commit semantics* of this chapter are what the MCU
  model reproduces over shared SRAM/dual-port memory (doc 18) — the spec
  is written transport-agnostic deliberately (offsets+seq, not shm_open).

## 12. Error codes (owned by this chapter)

`TPLC_E_IMG_MAGIC, _IMG_VERSION, _IMG_TOO_SMALL, _IMG_SEQ_EXHAUSTED,
TPLC_IMG_PLC_LOST (event), TPLC_IMG_MASTER_LOST (event),
TPLC_E_IMG_EPOCH, TPLC_E_MAP_OWNER (with doc 09)`

## 13. Acceptance criteria

1. `tether-plc-bridge` (in-process master lib) + `runtime/image` implement
   §3–§7; shared header `tplc_image.h` is the single source of truth.
2. Conformance: writer-crash-mid-write leaves readers bounded (no hang);
   seq-odd persistence detected; recovery documented behavior.
3. Fault-injection: kill -9 the PLC → master applies safe values ≤1 cycle
   after staleness bound; kill master → PLC runs input policy; restart
   both → clean resync via epoch.
4. Perf: full 64 KiB latch+commit ≤5 µs on reference x86, ≤50 µs on
   Pi1-class (draft targets — measure, don't guess; record in as-built).
5. Multi-program: two instances with disjoint %Q ownership run
   concurrently; overlap config is rejected `TPLC_E_MAP_OWNER`.
6. `tplc-image dump` works on a live system.
