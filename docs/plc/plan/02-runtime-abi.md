# 02 — Runtime ABI: `tplc_*` Contract Between Runtime and Compiled Program

> **Status:** work-item document; deleted when `docs/plc/runtime-abi.md` exists
> and `tplc_abi.h` + the runtime loader implement it with the conformance
> suite passing.
>
> **Satisfies:** G1 (ABI is IR-level), G2 (zero-cost scan), G4 (migrate is
> ABI), G6 (loader owns memory — crash isolation), G10 (task kind
> extensibility), D10 (loader-agnostic: same ABI for dlopen and MCU loaders).
> **Depends on:** 01. **Informs:** 03, 04, 07, 08, 18.

---

## 1. Scope and principles

This chapter defines the **complete binary contract** between the `tether-plc`
runtime (the "host") and a compiled program (the "module", delivered as a
`.so` inside a `.tplc` package). The ABI is:

- **C, not C++** — no mangling, no exceptions, stable across compilers.
- **Data-in, code-out**: the host owns ALL memory; the module receives
  pointers. The module owns no mutable state outside the state blob.
- **Loader-agnostic**: identical whether reached via `dlopen` (Linux), a PIC
  loader, or static linking (MCU). Nothing in the ABI assumes ELF exists —
  ELF is a transport for the symbols below.
- **Versioned**: `TPLC_ABI_VERSION` gates compatibility at load time.
- **Minimal**: every entry point earns its place. v1 exports exactly
  6 symbols.

### 1.1 Exported symbol table (v1)

| symbol | signature | required |
|---|---|---|
| `tplc_abi_version` | `uint32_t (void)` | yes |
| `tplc_get_descriptor` | `const TplcProgramDescriptor* (void)` | yes |
| `tplc_init` | `int32_t (TplcCtx*)` | yes |
| `tplc_shutdown` | `void (TplcCtx*)` | yes |
| `tplc_task_run` | `int32_t (TplcTaskCtx*)` | yes |
| `tplc_migrate` | `int32_t (TplcMigrateCtx*)` | conditional — required iff manifest has `migrate_desc` |

Nothing else may be exported (doc 01 §5 enforces via version script). The
host resolves by name (`dlsym`) on Linux, by linker/JIT symbol table on MCU —
same names either way.

### 1.2 Versioning contract

- `tplc_abi_version()` returns the ABI version the module was compiled
  against. Host loads iff `module == host` for v1 (exact match; a compat
  range may be defined in a later ABI version, encoded `(major<<16|minor)`).
- Mismatch → `TPLC_E_ABI_MISMATCH` before any other call.
- `TplcProgramDescriptor.abi_ext` carries optional capability bits so
  additive features need not bump the version.

## 2. Type definitions

C23/C11-compatible. Fixed-width integers; `_Static_assert`s on sizes; all
structs versioned via leading `size`/`version` fields where they're
host-allocated, or fixed-layout where module-compiled. Alignment: all
structs are naturally aligned; host provides ≥16 B-aligned regions.

```c
#define TPLC_ABI_VERSION 1u
#define TPLC_TASK_NAME_MAX 32
#define TPLC_PROG_NAME_MAX 64
#define TPLC_SERVICE_NAME_MAX 32
```

### 2.1 `TplcTaskKind`

```c
typedef enum TplcTaskKind : uint8_t {
    TPLC_TASK_CYCLIC       = 0, /* period_ns-scheduled              */
    TPLC_TASK_FREEWHEELING = 1, /* loops whenever CPU free (bg)     */
    TPLC_TASK_EVENT        = 2, /* triggered by host event bitmask  */
    TPLC_TASK_RESERVED_SFC = 3, /* iteration-2: SFC sequencing layer */
    TPLC_TASK_STATUS       = 4, /* triggered on runtime state changes*/
} TplcTaskKind;
```

v1 runtime implements `CYCLIC` (multi-rate) and may implement
`FREEWHEELING`/`STATUS`; `EVENT` and `RESERVED_SFC` are declared for the
module/host negotiation but unimplemented in v1 (load-time warning, not error,
when unused).

### 2.2 `TplcTaskDesc` — per-task descriptor (module-compiled, const)

```c
typedef struct TplcTaskDesc {
    uint8_t  name[TPLC_TASK_NAME_MAX]; /* NUL-padded IEC identifier   */
    uint8_t  kind;                     /* TplcTaskKind                */
    uint8_t  flags;                    /* bit0: enabled at start
                                          bit1: watchdoggled (wcet enforced)
                                          bit2: runs during online-change pause? (must be 0) */
    uint8_t  priority;                 /* 0..255, lower = earlier     */
    uint32_t _pad0;
    uint64_t period_ns;                /* CYCLIC only, else 0         */
    uint64_t wcet_hint_ns;             /* budget for jitter monitor; 0 = unspecified */
    uint32_t entry_index;              /* index into task fn table     */
    uint32_t task_local_size;          /* bytes of task-local blob region */
} TplcTaskDesc; /* 64 B */
```

### 2.3 `TplcProgramDescriptor` — the module's identity

```c
typedef struct TplcProgramDescriptor {
    uint32_t struct_size;          /* sizeof(TplcProgramDescriptor)     */
    uint32_t abi_version;          /* == tplc_abi_version()             */
    uint32_t abi_ext;              /* capability bits (§9)               */
    uint32_t flags;                /* bit0: has_migrate, bit1: has_task_table_v2(rsvd) */
    uint8_t  program_uuid[16];     /* == manifest program_uuid           */
    uint8_t  layout_hash[32];      /* == manifest layout_hash            */
    uint8_t  name[TPLC_PROG_NAME_MAX];

    /* sizes the host must provide */
    uint64_t state_blob_size;
    uint64_t retain_size;          /* subset of blob persisted           */
    uint64_t task_scratch_size;    /* per-task scratch (optional, 0)     */
    uint32_t image_in_bytes;       /* %I region consumed                 */
    uint32_t image_out_bytes;      /* %Q region produced                 */
    uint32_t marker_bytes;         /* %M consumed                        */

    /* task table */
    uint32_t task_count;
    uint32_t _pad1;
    const TplcTaskDesc* tasks;

    /* task entry table: entry_index indexes this array */
    const int32_t (*const* task_fns)(TplcTaskCtx*);
    uint32_t task_fn_count;

    /* io binding table: how generated code reaches the image (§5.2) */
    const TplcIoBinding* io_bindings;
    uint32_t io_binding_count;
    uint32_t _pad2;
} TplcProgramDescriptor;
```

All pointer fields are module-const (`.rodata`) — valid for module lifetime,
host must not retain past unload. `layout_hash` and `program_uuid` are the
fields the runtime uses for the online-change decision matrix (§7, doc 08).

### 2.4 `TplcIoBinding` — image access indirection

```c
typedef struct TplcIoBinding {
    uint32_t blob_offset;   /* location in state blob of the shadow var  */
    uint8_t  region;        /* 0=%I read 1=%Q write 2=%M rw               */
    uint8_t  type_code;     /* primitive type code (copy semantics)       */
    uint8_t  flags;         /* bit0: forceable                            */
    uint8_t  _pad;
    uint32_t image_offset;  /* offset into the region's image             */
    uint32_t size;          /* bytes                                      */
    uint32_t _pad2;
} TplcIoBinding; /* 24 B */
```

Generated code **never touches the image directly** — it reads/writes shadow
variables inside the state blob; the host copies `image→shadow` before task
execution and `shadow→image` after (this is what makes forcing, latching,
and migration uniform, see §5.2 and doc 03).

### 2.5 `TplcCtx` — host→module context (per program instance)

```c
typedef struct TplcCtx {
    uint32_t struct_size;
    uint32_t ctx_flags;          /* bit0: sim-mode, bit1: debug-services,
                                    bit2: retain-dirty-notify supported   */

    /* program instance memory (host-allocated, module-read/written) */
    void*    state_blob;         /* size = descriptor.state_blob_size   */
    void*    retain_region;      /* == state_blob + retain_off (inside blob) */
    void*    image_in;           /* latched %I snapshot (host-owned)    */
    void*    image_out;          /* working %Q (host-owned)             */
    void*    markers;            /* %M region (host-owned)              */

    /* time + invocation info (host-updated per entry) */
    uint64_t now_ns;             /* monotonic                            */
    uint64_t cycle_no;           /* invocation counter                   */
    int64_t  last_cycle_overrun_ns; /* signed; >0 if previous overran   */

    /* services — host-provided, callable from scan path (nonblocking) */
    const TplcServices* svc;

    /* host-private */
    void*    host_private;       /* module must not touch               */
} TplcCtx;
```

`now_ns`/`cycle_no` are refreshed by the host *before each entry call*, never
asynchronously — generated code reads a stable value. `image_in`/`image_out`
point at the **runtime's latched working copies**, not the SHM exchange buffer
(see doc 03 — the latch is the runtime's job).

### 2.6 `TplcTaskCtx` — per-invocation context

```c
typedef struct TplcTaskCtx {
    TplcCtx*  prog;              /* program context                      */
    uint32_t  task_index;        /* index into descriptor.tasks          */
    uint32_t  task_flags;        /* bit0: first-run-after-start
                                    bit1: last-run-before-stop
                                    bit2: resume-after-online-change      */
    void*     task_local;        /* task-local blob region               */
    uint64_t  scheduled_ns;      /* when this cycle was due              */
    uint64_t  deadline_ns;       /* scheduled + period (cyclic)          */
    uint64_t  dt_ns;             /* actual elapsed since last run        */
} TplcTaskCtx;
```

### 2.7 `TplcMigrateCtx` — online-change context

```c
typedef struct TplcMigrateCtx {
    TplcCtx*  new_prog;          /* fully allocated new instance         */

    const void* old_blob;        /* old program's state blob (frozen)    */
    uint64_t    old_blob_size;
    uint8_t     old_layout_hash[32];
    uint32_t    old_abi_version;

    const void* migrate_desc;    /* package's migrate_desc payload        */
    uint64_t    migrate_desc_size;

    /* out-params (module writes) */
    uint32_t    out_status;      /* TplcMigrateStatus                    */
    uint32_t    out_first_error; /* index into migrate_desc rule table    */
} TplcMigrateCtx;
```

### 2.8 `TplcServices` — host function table

```c
typedef struct TplcServices {
    uint32_t struct_size;
    uint32_t caps;               /* host capability bits (§9)            */

    /* diagnostics: nonblocking, bounded, may drop under load */
    void (*log)(void* host, uint8_t level, uint32_t code,
                const char* fmt_args_packed, uint32_t packed_len);
    void (*diag_counter)(void* host, uint32_t id, int64_t delta);

    /* runtime queries */
    uint64_t (*steady_ns)(void* host);   /* == ctx->now_ns normally        */

    /* retain persistence hint: region [off,off+len) of retain changed;
       host may schedule background flush. Never blocks. */
    void (*retain_dirty)(void* host, uint64_t off, uint64_t len);

    /* event task trigger (EVENT tasks, iter2/v2) */
    int32_t (*raise_event)(void* host, uint32_t event_mask);

    /* extension point: named service lookup → fn ptr or NULL.
       Names are stable strings ("trace.write", "string.utf16", ...).
       This is the ONLY growth path for services — table stays fixed. */
    void* (*get_service)(void* host, const char* name, uint32_t name_len);

    void* host;                  /* cookie passed as first arg           */
} TplcServices;
```

**Rules for service calls from generated code:** nonblocking, bounded-time
(`< 2 µs` contract, enforced by jitter accounting), never take locks the scan
path holds, never call back into the module. `log`/`diag_counter` write into
lock-free rings the host drains on the service plane. Anything not callable
under these rules does not exist in the scan path (e.g. string formatting
happens in `tplc-rt` against packed args, host formats later).

## 3. Memory model

### 3.1 Ownership

| region | allocated by | init by | lifetime |
|---|---|---|---|
| `state_blob` | host (aligned ≥64 B) | host zeroes → `tplc_init` fills | program instance |
| `retain_region` | inside blob (offset in descriptor-derived layout) | host restores persisted data | program instance + persisted |
| `image_in/out`, `markers` | host | host | program instance |
| `task_local` | inside blob | host zeroes per task | program instance |
| scratch | host (optional) | none (uninit) | per invocation |
| module `.data/.bss` | dlopen | ELF loader | module lifetime — **must contain no program state** (doc 01 §5 check) |

The host may allocate `state_blob` anywhere (heap, hugepage, dedicated arena)
and may **move it between calls** (defrag/migration staging): generated code
must never retain pointers to the blob or its subregions across an ABI call.
This is the core enabler of blob relocation and is tested by the
conformance suite's "moving host" mode.

### 3.2 State blob rules (the contract G4 depends on)

1. All mutable program state (globals, FB instances, task locals, internal
   bookkeeping) lives inside the blob at `layout_desc`-described offsets.
2. The blob contains **no pointers** — internal references are offsets.
   Enforced by codegen (doc 07); spot-checked by the "blob scrub" test
   (relocate + run; pointer-in-blob would crash).
3. The blob is `memcpy`-able at task boundaries (snapshot, migrate source).
4. `retain_region` is a contiguous sub-range; its sub-layout is described in
   `layout_desc`; persistence format is host's choice (doc 04 — file mmap).
5. Read-only constants may live in `.rodata` — not part of the blob, not
   migrated (identical across versions for same constants; changed constants
   are handled as "new value, old identity" by migration rules).

### 3.3 What generated code may NOT do

Hard prohibitions (verified by codegen + loader checks + fuzz review):

- No `malloc`/`free`/`realloc`/TLS/`thread_local`/`pthread_*`/`syscall`/
  `fork`/`exec`/file or socket I/O from any ABI entry point.
- No C++ exceptions across the boundary; module compiled `-fno-exceptions`
  `-fno-rtti`; a C++ `throw`/Rust `panic` inside module code must not escape —
  `tplc-rt` installs a module-local panic handler that longjmp-free
  terminates the call with `TPLC_TASK_E_PANIC` (see §6.3).
- No `setjmp/longjmp` into/out of the host; no signal handler installation.
- No floating-point mode changes (rounding/exceptions masks); codegen fixes
  `fp:strict` semantics (manifest `compile.flags`).
- No calling host pointers other than `svc` entries; no retaining host
  pointers; no writing outside provided regions.
- `init`/`shutdown` may allocate **only if** the ABI stays silent about
  internals? — **Decision: no.** Even `init` gets no allocator; any needed
  memory is declared in descriptor sizes so the host sees true footprint up
  front. (Keeps MCU parity and makes `requires` honest.)

## 4. Call semantics

### 4.1 `tplc_abi_version` / `tplc_get_descriptor`

Idempotent, pure, callable any number of times before `init`. Descriptor
pointer must remain valid until module unload (const rodata).

### 4.2 `tplc_init(ctx) -> int32`

- Called **exactly once** per program instance, on the service plane (never
  RT), before first task run.
- Precondition: host zeroed `state_blob`, `markers`, `task_local` regions;
  `image_in`/`image_out` contain current latched values; `retain_region`
  contains either zeroes (cold) or persisted data (warm restart).
- The module initializes variables to IEC defaults + declared initial
  values, marks FB instances, and **must not** overwrite retain data flagged
  `retain` in symmap when `ctx_flags` advertises warm restore? —
  **Clarification:** warm/cold is decided by the HOST (it restores retain
  into the region before init and tells the module via
  `ctx_flags bit3: retain_restored`). On `retain_restored`, init skips
  initializing retain-flagged fields (it walks layout_desc, which it knows
  internally).
- Returns `0` ok; `<0` = `TPLC_E_INIT_*` — host unloads, reports.
- Time contract: bounded but not RT-critical (declared `init_time_hint`
  not in v1 — runtime times it and reports).

### 4.3 `tplc_task_run(taskctx) -> int32`

- The hot path. Called by the host scheduler per task period.
- Preconditions: host has (a) latched `%I` into `image_in`, (b) applied
  active forces, (c) copied bound inputs `image→shadow` per `io_bindings`,
  (d) refreshed `now_ns/cycle_no/dt_ns`.
- Postconditions on return `0`: host copies bound outputs `shadow→image`,
  commits `image_out` (doc 03 sequencing).
- Return `<0` = task fault: host invokes per-task fault policy (stop task /
  stop program / safe-state; doc 04). `>0` reserved.
- Reentrancy: never concurrent — host guarantees one outstanding call per
  task; different tasks of one program *may* run on different threads ⇒
  generated code must be reentrant across tasks **only for task-disjoint
  state** (task_local + binding shadows are per-task; shared globals are
  codegen-marshalled: multi-task globals are legal only if the program
  declares them `VAR_ACCESS`-shared? — **v1 simplification: all tasks of a
  program run on the same RT thread**, dispatched by the host scheduler
  (matches CyclicTaskScheduler model). Concurrency model therefore =
  cooperative; true multi-threaded tasks are an abi_ext capability bit for
  later (§9 EXT_MT_TASKS) and are forbidden unless advertised.

### 4.4 `tplc_migrate(mctx) -> int32`

- Called on the service plane during online change, with all tasks of the
  program paused at a cycle boundary (doc 08 sequencing).
- Preconditions: `new_prog` fully allocated + host pre-zeroed; `old_blob`
  is a frozen copy (host snapshotted it — the running program keeps its
  original until swap commit, so migrate must not write to it);
  `migrate_desc` is the section bytes from the package.
- Module walks `migrate_desc` rules (which codegen emitted — it *knows* both
  layouts), copies/transforms state into `new_prog->state_blob`, sets
  `out_status`:
  - `MIGRATE_OK` — complete; host may swap.
  - `MIGRATE_OK_DEGRADED` — completed with losses listed in rules
    (dropped vars, defaulted fields); host swaps + raises structured alarm.
  - `MIGRATE_DENY` — a deny-rule hit; `out_first_error` identifies the rule;
    host aborts change, keeps old program, reports.
- Must be alloc-free, pure with respect to outside state, and idempotent
  (host may retry once after transient error).
- Post-migrate the host calls `tplc_init`-lite? — **No.** Post-migrate
  consistency: generated `tplc_migrate` is responsible for producing a blob
  exactly as `init` would have left it plus migrated values (it must run
  default-init for new fields, fix internal bookkeeping). The host then runs
  one "resume" pass: `task_flags.resume-after-online-change` on first cycle.
  Rationale: a second init pass would risk double-initializing migrated
  fields; putting complete responsibility in `migrate` keeps semantics in
  one place.

### 4.5 `tplc_shutdown(ctx)`

- Service plane, before unload/stop. May run FB "destructor" semantics
  (IEC has none standard — used for internal hygiene + retain_dirty flush
  hints). Not RT; must not fail meaningfully (void). After shutdown the host
  persists retain + frees regions; module code must not be called again.
- Distinct from dlclose: host calls `shutdown` → retain flush → `dlclose`.

### 4.6 Fault containment protocol

When `tplc_task_run` faults or the watchdog trips (doc 04), the host:
1. stops scheduling the program (all its tasks),
2. commits `%Q` safe-values per mapping config,
3. calls `tplc_shutdown` *only if* the fault was a clean return-code —
   after a trap/segv-equivalent (process survives in in-proc-sim mode) the
   host must not re-enter the module,
4. reports `TPLC_PROG_FAULT` + structured code to the service plane.

Module-side trap safety: on Linux the module runs in the sidecar process;
a segfault kills `tether-plc`, not the host contract — the process boundary
IS the fault wall (G6). In sim/in-proc test hosts, generated code is still
required to be UB-free; conformance runs under ASan/UBSan.

## 5. Data flow per cycle (normative sequence)

```
host (per task due):
 1. seqlock-latch %I SHM→image_in                [doc 03]
 2. apply force table to image_in/out overlays   [doc 12]
 3. for each io_binding read:  image→blob shadow
 4. refresh ctx.now_ns/cycle_no/dt_ns
 5. rc = task_fn(&taskctx)
 6. if rc==0: for each io_binding write: shadow→image_out
             publish image_out per doc 03 (per-cycle commit)
    else: fault protocol (§4.6)
```

`%M` is read-write directly (markers are program-internal scratch shared
across tasks and exposed to watch/force via symmap region 3).

## 6. Status and error spaces

### 6.1 Return codes

```
 0                       TPLC_OK
-1                       TPLC_E_INIT_LAYOUT       (blob size/layout mismatch)
-2                       TPLC_E_INIT_INVALID      (bad ctx / null field)
-3                       TPLC_E_INIT_VALUE        (init value out of range)
-16                      TPLC_TASK_E_FAULT        (generic task fault)
-17                      TPLC_TASK_E_PANIC        (module panic handler fired)
-18                      TPLC_TASK_E_BOUNDS       (internal bounds check)
-32                      TPLC_E_MIGRATE_LAYOUT    (desc/blob inconsistency)
-33                      TPLC_E_MIGRATE_DENY      (deny rule; see out_status)
```

Module-defined codes `-1000..-1999` pass through to diagnostics with
`module-defined` flag.

### 6.2 `TplcMigrateStatus` (out_status)

```
0 OK   1 OK_DEGRADED   2 DENY   3 ERR_DESC_INVALID   4 ERR_BLOB_RANGE
```

## 7. Online-change decision matrix (host side; full protocol in doc 08)

```
new.program_uuid != running.program_uuid → cold load (stop old, init new)
new.layout_hash == running.layout_hash   → FAST PATH: swap module only,
                                           blob untouched (recompile-identical)
else                                      → staged migrate: alloc new blob,
                                           tplc_migrate, commit or abort
```

ABI guarantee making the fast path legal: `layout_hash` covers *all* of
{layout_desc, symmap types, io_bindings, retain ranges}. Same hash ⇒
byte-compatible semantics even if code differs (pure logic change).

## 8. Sim parity hooks

- `ctx_flags bit0 (sim)` informs the module it runs against a simulated
  host — generated code MUST NOT branch behavior on it (determinism G7);
  it's for tplc-rt diagnostics only.
- `svc->steady_ns` on sim host returns the virtual sim clock — the ONLY
  sanctioned divergence point; code reading time goes through ctx/svc.
- All nondeterminism sources are enumerated and owned by the host: clock,
  image content, service responses. Module has no other inputs.

## 9. `abi_ext` capability bits

```
bit0 EXT_MT_TASKS        host may schedule program's tasks on >1 thread
bit1 EXT_EVENT_TASKS     host implements EVENT kind
bit2 EXT_SVC_TRACE       host implements "trace.*" named services
bit3 EXT_MOVABLE_BETWEEN_CALLS  (always on in v1; reserved)
bit4 EXT_RETAIN_ASYNC    host flushes retain asynchronously w/ dirty hints
bit5 EXT_FORCE_ATOMIC64  force/commit are 64-bit-atomic capable
```

Module loads iff `required_caps ⊆ host.caps` (module requirements recorded
in manifest `requires.abi_ext`; default 0 — conservative).

## 10. Iteration-2 hooks

- `TplcTaskKind::RESERVED_SFC` + `raise_event` + `STATUS` kind give SFC its
  sequencing substrate without ABI change.
- `get_service` named-service space is how LD/SFC tooling services
  (e.g. `sfc.stepinfo`, `ld.powerflow`) attach without ABI churn.
- `task_fn_count`/`entry_index` indirection lets SFC compile each
  step/transition to separately addressable functions.
- `abi_ext` space covers multi-threaded task scheduling if a later
  performance case demands it.

## 11. Conformance suite (owned by this chapter)

`tests/abi/` fixtures:

1. `stub_module.c` — hand-written minimal module exercising every ABI rule
   (must load & run).
2. `bad_*` series — modules violating each §3.3 prohibition (must be
   rejected at load or contained at run).
3. `moving_host` — host moves blob between calls; module must be correct.
4. `panic_module` — asserts fault protocol + no host corruption.
5. `migrate_sim` — scripted migrate: OK / DEGRADED / DENY paths.
6. Cross-compile check: module built for armv6 loads under qemu-user
   runtime (CI smoke).
7. MCU-shim check: module linked statically into a host-shim binary runs
   the same conformance sequence (proves loader-agnosticism, doc 18 seed).

## 12. Error codes (owned by this chapter)

`TPLC_E_ABI_MISMATCH, TPLC_E_ABI_EXT_UNMET, TPLC_E_INIT_*,
TPLC_TASK_E_*, TPLC_E_MIGRATE_*, TPLC_PROG_FAULT`

## 13. Acceptance criteria

1. `tplc_abi.h` published, C11-clean, `_Static_assert` layout-locked.
2. Conformance suite §11 green on x86_64 and qemu-aarch64.
3. `tplcc`-generated module passes the same suite unmodified.
4. Moving-host and panic-containment tests prove §3.2/§4.6 guarantees.
5. The same `.tplc` runs in sim host and Linux target with identical IO
   traces on identical input sequences (G7 evidence).
6. `EXT_*` negotiation behaves: module requiring unimplemented caps is
   refused with `TPLC_E_ABI_EXT_UNMET`.

---

## Appendix A — `tplc_abi.h` (normative listing)

The header below is the ABI itself; prose above explains semantics.
It is C11 (`#ifdef __cplusplus` guarded), freestanding-clean (no libc
includes beyond `<stdint.h>`/`<stddef.h>`), and `_Static_assert`-locked
so layout drift fails at compile time on both sides.

```c
/* tplc_abi.h — TetherPLC runtime ABI, version 1
 * Contract between host runtime (tether-plc) and a compiled program
 * module. C, not C++; no exceptions; stable across compilers.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */
#ifndef TPLC_ABI_H
#define TPLC_ABI_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ------------------------------------------------------------------ */
/* Versioning                                                          */
/* ------------------------------------------------------------------ */

#define TPLC_ABI_VERSION        1u
#define TPLC_ABI_VERSION_MAJOR(v) ((v) >> 16)
#define TPLC_ABI_VERSION_MINOR(v) ((v) & 0xFFFFu)
#define TPLC_ABI_MAKE(maj,min)    (((maj) << 16) | (min))

#define TPLC_TASK_NAME_MAX      32u
#define TPLC_PROG_NAME_MAX      64u
#define TPLC_SERVICE_NAME_MAX   32u

/* ------------------------------------------------------------------ */
/* Return codes (ABI entries return int32_t; <0 = error)               */
/* ------------------------------------------------------------------ */

enum {
    TPLC_OK                  = 0,

    /* tplc_init */
    TPLC_E_INIT_LAYOUT       = -1,   /* blob size/layout mismatch      */
    TPLC_E_INIT_INVALID      = -2,   /* bad ctx / null required field  */
    TPLC_E_INIT_VALUE        = -3,   /* init value out of range        */
    TPLC_E_INIT_RESOURCE     = -4,   /* declared resource unsatisfiable*/

    /* tplc_task_run */
    TPLC_TASK_E_FAULT        = -16,  /* generic task fault             */
    TPLC_TASK_E_PANIC        = -17,  /* module panic handler fired     */
    TPLC_TASK_E_BOUNDS       = -18,  /* internal bounds check          */
    TPLC_TASK_E_DIVZERO      = -19,  /* checked int division by zero   */
    TPLC_TASK_E_OVERFLOW     = -20,  /* checked-mode int overflow      */
    TPLC_TASK_E_RANGE        = -21,  /* subrange/enum/string range     */
    TPLC_TASK_E_ABORTED      = -22,  /* cooperative abort via ckpt     */
    TPLC_TASK_E_OVERRUN      = -23,  /* WCET budget hard-exceeded      */

    /* tplc_migrate */
    TPLC_E_MIGRATE_LAYOUT    = -32,  /* desc/blob inconsistency        */
    TPLC_E_MIGRATE_DENY      = -33,  /* deny rule; see out_status      */
    TPLC_E_MIGRATE_DESCINVAL = -34,  /* migrate_desc malformed         */
    TPLC_E_MIGRATE_BLOBRANGE = -35,  /* blob access outside declared   */

    /* loader/host surface (never returned by module code itself) */
    TPLC_E_ABI_MISMATCH      = -64,  /* abi_version != host's          */
    TPLC_E_ABI_EXT_UNMET     = -65,  /* required caps not in host caps */

    /* reserved */
    TPLC_E_MODULE_MIN        = -999,
    TPLC_E_MODULE_MAX        = -1999 /* module-defined range           */
};

/* Event/alarm codes (non-negative; never a return value) */
enum {
    TPLC_PROG_FAULT = 16             /* program faulted; §4.6          */
};

/* TplcMigrateStatus — written into TplcMigrateCtx.out_status */
enum TplcMigrateStatus {
    TPLC_MIGRATE_OK             = 0,
    TPLC_MIGRATE_OK_DEGRADED    = 1,
    TPLC_MIGRATE_DENY           = 2,
    TPLC_MIGRATE_ERR_DESC       = 3,
    TPLC_MIGRATE_ERR_BLOB_RANGE = 4
};

/* ------------------------------------------------------------------ */
/* Flags and enums                                                     */
/* ------------------------------------------------------------------ */

/* TplcCtx.ctx_flags */
enum {
    TPLC_CTXF_SIM             = 1u << 0, /* sim host (diagnostics only) */
    TPLC_CTXF_DEBUG_SVC       = 1u << 1, /* debug named services up    */
    TPLC_CTXF_RETAIN_HINT     = 1u << 2, /* retain_dirty supported     */
    TPLC_CTXF_RETAIN_RESTORED = 1u << 3  /* retain region warm-loaded  */
};

/* TplcTaskDesc.flags */
enum {
    TPLC_TASKF_ENABLED_AT_START = 1u << 0,
    TPLC_TASKF_WATCHDOGGED      = 1u << 1
};

/* TplcTaskCtx.task_flags */
enum {
    TPLC_TASKF_FIRST_RUN  = 1u << 0,
    TPLC_TASKF_LAST_RUN   = 1u << 1,
    TPLC_TASKF_RESUME_OC  = 1u << 2  /* first run after online change  */
};

/* TplcTaskDesc.kind — TplcTaskKind */
enum TplcTaskKind {
    TPLC_TASK_CYCLIC       = 0,
    TPLC_TASK_FREEWHEELING = 1,
    TPLC_TASK_EVENT        = 2,   /* v1: declared, unimplemented      */
    TPLC_TASK_RESERVED_SFC = 3,   /* iteration-2 sequencing layer     */
    TPLC_TASK_STATUS       = 4
};

/* TplcProgramDescriptor.abi_ext capability bits */
enum TplcAbiExt {
    TPLC_EXT_MT_TASKS            = 1u << 0,
    TPLC_EXT_EVENT_TASKS         = 1u << 1,
    TPLC_EXT_SVC_TRACE           = 1u << 2,
    TPLC_EXT_MOVABLE_BETWEEN     = 1u << 3, /* always on in v1         */
    TPLC_EXT_RETAIN_ASYNC        = 1u << 4,
    TPLC_EXT_FORCE_ATOMIC64      = 1u << 5
};

/* TplcIoBinding.region */
enum TplcIoRegion {
    TPLC_REGION_IMAGE_IN   = 0,  /* %I: image→shadow before task       */
    TPLC_REGION_IMAGE_OUT  = 1,  /* %Q: shadow→image after task        */
    TPLC_REGION_MARKER     = 2   /* %M: direct map, no copy needed     */
};

/* TplcIoBinding.flags */
enum { TPLC_IOB_FORCEABLE = 1u << 0 };

/* TplcIoBinding.type_code — primitive codes for host copy semantics */
enum TplcIoType {
    TPLC_IOT_BOOL  = 0,   /* bit copy: image_offset in bits          */
    TPLC_IOT_U8  = 1, TPLC_IOT_S8  = 2,
    TPLC_IOT_U16 = 3, TPLC_IOT_S16 = 4,
    TPLC_IOT_U32 = 5, TPLC_IOT_S32 = 6,
    TPLC_IOT_U64 = 7, TPLC_IOT_S64 = 8,
    TPLC_IOT_F32 = 9, TPLC_IOT_F64 = 10,
    TPLC_IOT_RAW = 15   /* size-byte copy (structs/arrays/strings)   */
};

/* ------------------------------------------------------------------ */
/* Tables (module-const)                                               */
/* ------------------------------------------------------------------ */

typedef struct TplcIoBinding {
    uint32_t blob_offset;   /* shadow var location in state blob       */
    uint8_t  region;        /* TplcIoRegion                            */
    uint8_t  type_code;     /* TplcIoType                              */
    uint8_t  flags;
    uint8_t  unit;          /* bits: type_code==TPLC_IOT_BOOL uses     */
    uint32_t image_offset;  /* offset in region's image (bytes; bits   */
                            /* for BOOL: byte offset, unit = bit idx)  */
    uint32_t size;          /* bytes (BOOL: 1)                         */
    uint32_t _pad;
} TplcIoBinding;
_Static_assert(sizeof(TplcIoBinding) == 20, "TplcIoBinding layout");

typedef struct TplcTaskDesc {
    uint8_t  name[TPLC_TASK_NAME_MAX];
    uint8_t  kind;          /* TplcTaskKind                            */
    uint8_t  flags;
    uint8_t  priority;      /* 0..255, lower = earlier                 */
    uint8_t  _pad0;
    uint64_t period_ns;     /* CYCLIC only                             */
    uint64_t wcet_hint_ns;  /* 0 = unspecified                         */
    uint32_t entry_index;   /* into task_fns[]                         */
    uint32_t task_local_size;
} TplcTaskDesc;
_Static_assert(sizeof(TplcTaskDesc) == 64, "TplcTaskDesc layout");

/* forward decls */
typedef struct TplcCtx       TplcCtx;
typedef struct TplcTaskCtx   TplcTaskCtx;
typedef struct TplcMigrateCtx TplcMigrateCtx;
typedef int32_t (*TplcTaskFn)(TplcTaskCtx*);

typedef struct TplcProgramDescriptor {
    uint32_t struct_size;         /* sizeof(TplcProgramDescriptor)     */
    uint32_t abi_version;
    uint32_t abi_ext;             /* required caps (host must superset)*/
    uint32_t flags;               /* bit0: has_migrate                 */
    uint8_t  program_uuid[16];
    uint8_t  layout_hash[32];
    uint8_t  name[TPLC_PROG_NAME_MAX];

    uint64_t state_blob_size;
    uint64_t retain_size;
    uint64_t retain_offset;       /* within blob                       */
    uint64_t task_scratch_size;
    uint32_t image_in_bytes;
    uint32_t image_out_bytes;
    uint32_t marker_bytes;
    uint32_t _pad0;

    uint32_t task_count;
    uint32_t _pad1;
    const TplcTaskDesc* tasks;

    const TplcTaskFn* task_fns;
    uint32_t task_fn_count;
    uint32_t _pad2;

    const TplcIoBinding* io_bindings;
    uint32_t io_binding_count;
    uint32_t _pad3;
} TplcProgramDescriptor;

/* ------------------------------------------------------------------ */
/* Services (host-provided; callable from scan path, nonblocking)      */
/* ------------------------------------------------------------------ */

typedef struct TplcServices {
    uint32_t struct_size;
    uint32_t caps;                /* TplcAbiExt host supports          */

    void     (*log)(void* host, uint8_t level, uint32_t code,
                    const void* packed_args, uint32_t packed_len);
    void     (*diag_counter)(void* host, uint32_t id, int64_t delta);
    uint64_t (*steady_ns)(void* host);
    void     (*retain_dirty)(void* host, uint64_t off, uint64_t len);
    int32_t  (*raise_event)(void* host, uint32_t event_mask);
    void*    (*get_service)(void* host, const char* name, uint32_t len);

    void*    host;                /* cookie — first arg of the above   */
} TplcServices;
_Static_assert(offsetof(TplcServices, host) == 7 * sizeof(void*),
               "TplcServices layout");

/* ------------------------------------------------------------------ */
/* Contexts (host-allocated; pointers valid for call duration only)    */
/* ------------------------------------------------------------------ */

struct TplcCtx {
    uint32_t struct_size;
    uint32_t ctx_flags;           /* TPLC_CTXF_*                       */

    void*    state_blob;
    void*    retain_region;       /* == state_blob + retain_offset     */
    void*    image_in;
    void*    image_out;
    void*    markers;

    uint64_t now_ns;
    uint64_t cycle_no;
    int64_t  last_cycle_overrun_ns;

    const TplcServices* svc;
    void*    host_private;

    /* module-visible control fields (host-written, codegen-read) */
    volatile uint32_t abort_flag; /* cooperative-abort request         */
    uint32_t  _pad0;
    uint64_t  budget_deadline_ns; /* hard WCET bound for ckpt          */
};

struct TplcTaskCtx {
    TplcCtx* prog;
    uint32_t task_index;
    uint32_t task_flags;          /* TPLC_TASKF_*                      */
    void*    task_local;
    uint64_t scheduled_ns;
    uint64_t deadline_ns;
    uint64_t dt_ns;
};

struct TplcMigrateCtx {
    TplcCtx*    new_prog;

    const void* old_blob;
    uint64_t    old_blob_size;
    uint8_t     old_layout_hash[32];
    uint32_t    old_abi_version;
    uint32_t    _pad0;

    const void* migrate_desc;
    uint64_t    migrate_desc_size;

    uint32_t    out_status;       /* TplcMigrateStatus                 */
    uint32_t    out_first_error;
};

/* ------------------------------------------------------------------ */
/* Exported symbol table (the complete ABI surface)                    */
/* ------------------------------------------------------------------ */

uint32_t                      tplc_abi_version(void);
const TplcProgramDescriptor*  tplc_get_descriptor(void);
int32_t                       tplc_init(TplcCtx* ctx);
void                          tplc_shutdown(TplcCtx* ctx);
int32_t                       tplc_task_run(TplcTaskCtx* tctx);
int32_t                       tplc_migrate(TplcMigrateCtx* mctx);

/* symbol names as strings for loader tables */
#define TPLC_SYM_ABI_VERSION   "tplc_abi_version"
#define TPLC_SYM_DESCRIPTOR    "tplc_get_descriptor"
#define TPLC_SYM_INIT          "tplc_init"
#define TPLC_SYM_SHUTDOWN      "tplc_shutdown"
#define TPLC_SYM_TASK_RUN      "tplc_task_run"
#define TPLC_SYM_MIGRATE       "tplc_migrate"

#ifdef __cplusplus
} /* extern "C" */
#endif
#endif /* TPLC_ABI_H */
```

Notes on the listing vs the earlier prose:

- `TplcTaskCtx.task_fns` indirection (descriptor `task_fns[]` +
  `entry_index`) is retained — the public `tplc_task_run` remains for
  loaders that want a single entry; the table is the scheduler's fast
  dispatch path (task_fn[i] == the per-task entry). Both are spec'd
  so MCU loaders and the Linux scheduler share one dispatch model.
- `abort_flag` + `budget_deadline_ns` are **in `TplcCtx`** — that is
  what makes checkpoints ~1–2 ns (memory flag, no service call); the
  clock-read-every-64th rule from doc 04/07 uses `svc->steady_ns` only
  on the slow path.
- `retain_offset` added to the descriptor so the host derives
  `retain_region` mechanically.

## Appendix B — `tplc_image.h` (SHM header, normative)

```c
/* tplc_image.h — SHM process-image header (doc 03 §3 layout).
 * Fields are plain integers; all access goes through the tplc_img_*
 * accessors which apply __atomic_* builtins with explicit memory
 * orders — portable across C and C++ with identical semantics to
 * _Atomic fields (recorded in docs/plc/DECISIONS.md). */
#define TPLC_IMAGE_MAGIC   0x47414D49434C5054ull /* 'TPLCIMAG' LE     */
#define TPLC_IMAGE_VERSION 1u
#define TPLC_IMAGE_HDRSIZE 256u

typedef struct TplcImageHeader {
    uint64_t magic;                     /* TPLC_IMAGE_MAGIC           */
    uint32_t version;
    uint32_t header_size;
    uint32_t layout_flags;
    uint32_t _rsvd;
    uint64_t image_epoch;
    uint64_t in_seq;                    /* seqlock: %I (master writes)*/
    uint64_t out_seq;                   /* seqlock: %Q (plc writes)   */
    uint64_t master_hb_ns;
    uint64_t plc_hb_ns;
    uint32_t master_status;             /* 0 init 1 run 2 deg 3 stop  */
    uint32_t plc_status;                /* 0 open 1 run 2 fault 3 stop*/
    uint64_t in_off, in_size;
    uint64_t out_off, out_size;
    uint64_t in_writes, out_writes;
    uint64_t in_retries, out_retries;
    uint64_t last_fault_code;
    uint8_t  reserved[112];
} TplcImageHeader;
_Static_assert(sizeof(TplcImageHeader) == 256, "image header size");
```
