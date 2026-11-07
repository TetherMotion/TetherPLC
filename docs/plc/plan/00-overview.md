# 00 — TetherPLC Plan: Overview, Conventions, Glossary

> **Status:** work-item document. Lives in `docs/plc/plan/` until superseded;
> its durable content (glossary, architecture diagram) migrates to
> `docs/plc/architecture.md` when the system exists. This file is the
> exception to "delete when done" — it shrinks into the permanent doc.
>
> **Satisfies:** all G1–G12 at the level of architecture shape.
> **Dependencies:** `docs/DESIGN-GOALS.md` (read first).

---

## 1. What we are building

A soft-PLC layer on top of Tether:

- A controls engineer writes **IEC 61131-3 structured text (ST)** or draws a
  **function block diagram (FBD)** in the TetherWebUI browser IDE.
- A compiler (`tplcc`, Rust) lowers both to **one LLVM IR**, then to a
  target-specific **ELF shared object** inside an **artifact package**
  (`.tplc`).
- A dedicated realtime process (`tether-plc`, C++) **dlopen**s the `.so`,
  allocates the program's state blob, executes IEC tasks against a
  **shared-memory process image**, supports **online change** (migrate +
  atomic swap), and can crash without taking the fieldbus down.
- The Tether application owns all physical I/O: it cyclically exchanges the
  process image with EtherCAT PDOs (and other transports) and exposes PLC
  variables, control and debug surfaces over `tether_io` to the web UI.

```
engineering host / on-device toolchain            target
┌──────────────────────────────┐         ┌────────────────────────────────────┐
│ TetherWebUI (Lit/TS)         │         │  tether (C++ app)                  │
│  ├─ ST editor                │ deploy  │   owns EtherCAT, FSoE, motion,     │
│  ├─ FBD canvas editor        │ .tplc   │   tether_io server                 │
│  └─ watch/force/trace UI     │ ───────►│        │ SHM image (seqlock)        │
│         │  PLCopen XML/ST    │         │        ▼                           │
│         ▼                    │         │  tether-plc (C++ RT process)       │
│  tplcc (Rust)                │         │   dlopen artifact .so              │
│   ST frontend ─┐             │         │   tasks ↔ state blob ↔ image latch │
│   FBD lowering ┼─► LLVM IR ─►│──► .so  │   online change, watchdog, retain │
│   (iter2: SFC,LD)│           │         └────────────────────────────────────┘
│   tplc-rt FB lib (cleanroom) │
└──────────────────────────────┘
sim host: same canonical IR → host .so → tether-plc against simulated image
```

## 2. The three planes

Everything in the design sits on one of three planes; keeping them separate is
the main structural rule:

| Plane | Content | Latency regime |
|---|---|---|
| **Scan plane** | `tether-plc` executing compiled code against the image | deterministic, sub-cycle; zero allocation, zero syscalls except latch |
| **I/O plane** | `tether` exchanging the image with fieldbus PDOs | deterministic, master-side cyclic task |
| **Service plane** | compile, deploy, online change, watch/force, symbol catalog, journal | best-effort, off the RT path |

Rule: the scan and I/O planes never block on the service plane. All service
operations (including online change staging) are prepared off-cycle and
committed at a task boundary.

## 3. Component inventory

| Component | Lang | Location (planned) | Plan doc |
|---|---|---|---|
| `tplcc` compiler driver | Rust | `compiler/` | 05, 07, 10 |
| ST frontend | Rust | `compiler/tplc-st/` | 05 |
| FBD lowering | Rust | `compiler/tplc-fbd/` | 06 |
| IR codegen + state layout + migrate gen | Rust/LLVM | `compiler/tplc-codegen/` | 07, 08 |
| `tplc-rt` FB/stdlib (clean-room) | Rust→IR, C-callable | `compiler/tplc-rt/` | 07 |
| Artifact packaging (`libtplc-pkg`) | Rust + C++ reader | `shared/pkg/` | 01 |
| `tether-plc` runtime process | C++ | `runtime/` | 02–04, 08 |
| SHM image library | C++ | `runtime/image/` | 03 |
| Tether-side image publisher + IO mapping | C++ | `tether-plc-bridge/` | 03, 09 |
| Web IDE (editors + debug UI) | TS/Lit | `TetherWebUI` | 11, 12 |
| Oracle adapters (GPL quarantine) | any | `tools/oracle-*/` (dev-only) | 14 |
| Sim harness | C++/Rust | `sim/` | 14 |

## 4. Document map and deletion rule

See README-level numbering in the plan directory. Each `NN-*.md` chapter
ends with an **Acceptance criteria** section; a chapter is deleted when its
criteria are met and the corresponding `docs/plc/*.md` as-built spec exists.
Chapters depend on each other as follows:

```
01 ──► 02 ──► 03 ──► 04 ──► 08 ──► 09
 │      │                 ▲
 │      ▼                 │
 10 ──► 07 ◄── 05, 06 ────┘
              ▲    ▲
              │    └── 11 (editor produces FBD docs + ST)
        13 (eval: gates implementation of 05/07 choices)
        14, 15, 16 cross-cut everything; 17 schedules; 18 defers
```

## 5. Shared naming and file conventions

| Name | Meaning |
|---|---|
| `tplc` | universal prefix; artifact file extension `.tplc` |
| `tplcc` | the compiler driver binary |
| `tether-plc` | the runtime process binary |
| `tplc-rt` | clean-room FB/runtime support library (compiled into artifacts) |
| `TplcCtx`, `TplcTaskCtx`, `TplcMigrateCtx` | ABI context structs (doc 02) |
| `TplcProgramDescriptor` | ABI entry table returned by `tplc_get_descriptor()` |
| `%I / %Q / %M / %R` | image areas: inputs, outputs, markers, retain (doc 03) |
| *state blob* | single relocatable region holding all program state (doc 02/07) |
| *symbol map* / `symmap` | compiler-emitted name↔offset table (doc 01/08) |
| *layout hash* | content hash of state-blob layout; drives migration compatibility |
| *image latch* | atomic snapshot of the SHM image taken at task start (doc 03) |
| *online change* | migrate + swap of a running program (doc 08) |
| *oracle* | GPL tool used dev-side only, behind `tools/oracle-*` adapters (G12) |

Formats: all multi-byte fields in file formats are little-endian; all wire
formats follow `tether_io` conventions; all sizes/offsets in file formats are
`u32` unless noted; all timestamps nanoseconds since monotonic epoch unless
noted.

## 6. Cross-cutting requirements (referenced by all chapters)

- **No allocation in scan path.** Generated code and the runtime scanner never
  allocate after `init` completes. (`G2`)
- **No exceptions/RTTI** across the ABI boundary or inside generated code.
- **Position-independent state.** The state blob contains no pointers into
  itself; references are offsets. (`G4`)
- **Determinism.** Same inputs + same initial blob ⇒ bit-identical outputs and
  blob, on target and in sim. (`G7`)
- **Diagnosability.** Every failure path reports a structured error with a
  stable code (per-doc error tables). (`G8`)
- **Iteration-2 hooks.** Where a chapter touches task model, IR, or editor
  model, an *"Iteration-2"* note describes what SFC/LD will need. (`G10`)

## 7. Glossary

**Artifact / `.tplc` package** — self-contained compiled-program container:
manifest, canonical bitcode, one or more target `.so`s, symbol map, migration
descriptor, provenance/signature. (doc 01)

**Function block (FB)** — IEC stateful unit (TON, CTU, …); instance state lives
in the state blob. (doc 07)

**Image** — the shared-memory I/O exchange area, `%I`/`%Q`/`%M`/`%R`. (doc 03)

**Latch** — consistent snapshot of the image taken by the runtime at task
start. (doc 03)

**Migration descriptor** — compiler-emitted description of a *previous*
state-blob layout enabling `tplc_migrate`. (doc 08)

**Online change** — replacing a running program with a new version, preserving
state per migration rules. (doc 08, G4)

**Oracle adapter** — dev-only tool wrapping a GPL program (e.g. matiec) to
emit expected-behavior test vectors; never shipped. (G12, doc 14)

**Process image model** — IEC semantics: inputs frozen for a cycle, program
executes, outputs committed once per cycle.

**Program invocation** — a loaded program instance in the runtime (CiA 405
vocabulary: creation, start, stop, reset, delete). (doc 16)

**Scan plane / I/O plane / Service plane** — see §2.

**State blob** — compiler-defined relocatable struct holding globals, FB
instance state, and internal bookkeeping; allocated and owned by the runtime,
populated by generated code through the context pointer. (doc 02)

**Symbol map (`symmap`)** — binary table: name, kind (var/FB/param/I/O),
state-blob or image offset, type descriptor, retain flag, source provenance
(incl. FBD node ids). Feeds the `tether_io` catalog and the debugger. (doc 01)

**WCET budget** — per-task declared worst-case execution time monitored by the
runtime; violations are structured alarms. (doc 04)

---

## Appendix L — Consolidated error-code registry

Every `TPLC_E_*`/`TPLC_*_*` code is owned by exactly one chapter (its
canonical home); this table is the index. Codes are API — never
renumbered once shipped; new codes append within their owner's range.

| range | owner (doc) | codes |
|---|---|---|
| pkg/format | 01 | `TPLC_E_PKG_MAGIC _PKG_VERSION _PKG_TOC _PKG_HASH_HEADER _PKG_HASH_CONTENT _PKG_UNKNOWN_SECTION _PKG_LIMIT _SIG_REQUIRED _SIG_INVALID _SIG_UNTRUSTED _PKG_HIDDEN_STATE _SYMMAP_RANGE _TARGET_MISMATCH _ABI_MISMATCH _REQUIRES_UNMET` |
| abi | 02 | `TPLC_OK TPLC_E_INIT_{LAYOUT,INVALID,VALUE,RESOURCE} TPLC_TASK_E_{FAULT,PANIC,BOUNDS,DIVZERO,OVERFLOW,RANGE,ABORTED,OVERRUN} TPLC_E_MIGRATE_{LAYOUT,DENY,DESCINVAL,BLOBRANGE} TPLC_MIGRATE_{OK,OK_DEGRADED,DENY,ERR_*} TPLC_E_ABI_{MISMATCH,EXT_UNMET} TPLC_PROG_FAULT` |
| image | 03 | `TPLC_E_IMG_{MAGIC,VERSION,TOO_SMALL,SEQ_EXHAUSTED,EPOCH} TPLC_IMG_{PLC_LOST,MASTER_LOST}` |
| runtime | 04 | `TPLC_E_RT_{PKG,PROGRAM_STATE,TASK_STATE,OVERLOAD,HANG,SWAP_*,RETAIN_*,CTL_*,RUNBOOK_*} TPLC_RT_{RESPAWNED,PROGRAM_FAULT,ROLLBACK}` |
| st dialect | 05 | `TPLC_E_ST_{LEX,PARSE,NAME,TYPE,PROHIBITED,RESERVED,ATMAP,CALLSHAPE,RECURSION,LINT}` |
| fbd model | 06 | `TPLC_E_FBD001..FBD014 TPLC_E_XML_* TPLC_E_MODEL_SCHEMA` |
| codegen | 07 | `TPLC_E_{MIR_INVALID,CG_UNSUPPORTED,CG_LAYOUT,CG_MIGRATE_GEN,CG_EMIT,RT_SEMANTIC}` |
| online change | 08 | (reuses abi/runtime codes; swap-specific `TPLC_RT_SWAP_*` owned by 04) |
| io mapping | 09 | `TPLC_E_MAP_{OWNER,DIR,RANGE,TYPE,PDO,SAFETY,AT,BIND,EPOCH,SIG?}` |
| toolchain | 10 | `TPLC_E_TC_{PROJECT,TARGET,SYSROOT?,EMIT,DEPLOY_*,LIVE_QUERY,SIG_REQUIRED}` |
| debug | 12 | `TPLC_E_DBG_{RESOLVE,STALE,OVERLOAD,RATE,ACCESS,STATE} TPLC_E_FORCE_DENIED TPLC_E_TRACE_CONFIG` |
| sim/testing | 14 | `TPLC_E_SIM_{DIVERGENCE,CLOCK,BACKEND} TPLC_E_ORACLE_*` |
| security | 15 | `TPLC_E_SEC_{SANDBOX,POLICY,AUDIT} TPLC_E_PKG_SYSCALL` |
| management | 16 | `TPLC_E_MGMT_{OP,STATE,AUTHZ,SCHEMA}` |
| module-defined | 02 | `-1000..-1999` reserved for generated/module codes |

Rules: (a) an error code may be *raised* anywhere but is *defined* only
in its owner's range; (b) UI-facing messages come from `tplcc explain
<code>`/the as-built registry doc, never hard-coded per call site;
(c) events vs errors: `TPLC_IMG_*`/`TPLC_RT_*` non-`_E_` forms are
event/alarm codes, not return codes.
