# TetherPLC architecture

Concepts and implementation map. Read this first; the per-topic
as-built specs (§7) carry the normative detail. For acceptance/test
status see `acceptance-matrix.md`; for rationale see `DECISIONS.md`
and `../DESIGN-GOALS.md`.

---

## 1. What it is

TetherPLC is a soft-PLC layer on top of Tether:

- A controls engineer writes **IEC 61131-3 structured text (ST)** or
  draws a **function block diagram (FBD)** in the TetherWebUI
  browser IDE.
- The compiler (`tplcc`, Rust) lowers both to one LLVM IR, then to a
  target-specific ELF shared object inside an **artifact package**
  (`.tplc`).
- A dedicated realtime process (`tether-plc`, C++) `dlopen`s the
  `.so`, allocates the program's **state blob**, executes IEC tasks
  against a **shared-memory process image**, supports **online
  change** (migrate + atomic swap) and **shadow run** (candidate
  program on identical inputs with instant promotion), and can crash
  without taking the fieldbus down.
- The Tether application owns all physical I/O: it cyclically
  exchanges the process image with EtherCAT PDOs (and other
  transports) and exposes PLC variables, control and debug surfaces
  over `tether_io` to the web UI.

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
│   tplc-rt FB lib (cleanroom) │         └────────────────────────────────────┘
└──────────────────────────────┘
sim: same canonical IR → host .so → tether-plc-sim against a
simulated image (L2); MIR interpreter is the L1 oracle
```

## 2. The three planes

Everything sits on one of three planes; keeping them separate is the
main structural rule:

| Plane | Content | Latency regime |
|---|---|---|
| **Scan plane** | `tether-plc` executing compiled code against the image | deterministic, sub-cycle; zero allocation, zero syscalls except latch |
| **I/O plane** | `tether` exchanging the image with fieldbus PDOs | deterministic, master-side cyclic task |
| **Service plane** | compile, deploy, online change, watch/force, symbol catalog, journal | best-effort, off the RT path |

Rule: the scan and I/O planes never block on the service plane. All
service operations (including online-change staging) are prepared
off-cycle and committed at a task boundary.

## 3. Artifact lifecycle

1. **Author** — ST source (`.st`) or FBD diagram (PLCopen XML /
   `schema/fbd.v1.json`).
2. **Compile** — `tplcc build` runs frontend → `tplc-mir` →
   sema/layout → LLVM → object + canonical bitcode + symbol map +
   layout descriptor → `.tplc` package (`tplc-pkg`).
3. **Sign** (optional) — `tplcc sign` adds an ed25519 signature
   section; deploy-time policy decides whether it is required
   (`security.md`, `artifact-format.md`).
4. **Deploy** — chunked upload over the ctl channel
   (`DEPLOY_BEGIN/CHUNK/COMMIT`); the runtime validates header,
   hashes, target and ABI version *before* `dlopen`, then runs the
   post-link ELF scanner.
5. **Run** — `PROG_PREPARE` allocates the state blob and calls
   `tplc_init`; `PROG_START` puts IEC tasks on the scheduler. Each
   cycle: latch `%I` → run → commit `%Q`.
6. **Change** — `PROG_SWAP` compiles with `--change-against`,
   generates `tplc_migrate` from the layout diff, stages off-cycle,
   swaps at a task boundary; predecessor is quarantined for
   `PROG_ROLLBACK`. Or deploy as **shadow** first and promote later.

## 4. Core concepts

**Artifact package (`.tplc`)** — self-contained container: manifest,
canonical bitcode, one or more target `.so`s, symbol map, migration
descriptor, provenance/signature. Spec: `artifact-format.md`.

**State blob** — single relocatable region holding all program state
(globals, FB instance state, bookkeeping); pointer-free inside, so it
can be memcpy'd, migrated and inspected. Owned by the runtime,
populated by generated code via the context pointer. Spec:
`runtime-abi.md`.

**Process image** — the shared-memory I/O exchange area:
`%I` inputs, `%Q` outputs, `%M` markers, `%R` retain. The runtime
takes an atomic **latch** (seqlock snapshot) at task start; the
program sees inputs frozen for the whole cycle and commits outputs
once. Spec: `process-image.md`.

**Symbol map (`symmap`)** — compiler-emitted name↔offset table
(var/FB/param/I/O kind, blob or image offset, type descriptor,
retain flag, source provenance incl. FBD node ids). Feeds the
`tether_io` catalog and the debugger.

**Layout hash / migration descriptor** — content hash of the
state-blob layout; `tplc_migrate` is generated per layout-diff and
copies/transforms/denies fields. Spec: `online-change.md`.

**Tasks & WCET** — cyclic/interval IEC tasks with declared
worst-case budgets, monitored by the runtime; violations are
structured alarms. Spec: `plc-runtime.md`.

**Retain** — `%R` region persisted through an mmap-backed store with
dirty hints and a flusher; survives process restart.

**Online change** — replace a running program preserving state per
migration rules; pause budget ≤ 1 task cycle. `online-change.md`.

**Shadow run** — a candidate program consuming the *identical*
latched `%I` stream; its `%Q` lands in a runtime-owned shadow region,
never the image. `SHADOW_COMPARE` diffs channels live;
`SHADOW_PROMOTE` swaps it in atomically with its live blob.
`shadow-run.md`.

**Debug plane** — symbol-resolved watch/force/write, suspend/step,
expression breakpoints, trace rings + flight recorder. All
boundary-applied and load-shed so the scan never waits on it.
`debugging.md`.

**Security boundary** — signed artifacts + trust policies, seccomp
sandbox for `tether-plc`, UDS peer-cred roles (observer <
commissioner < admin), durable audit journal, safety-channel
map/force/write denial. `security.md`, `safety.md`.

**Program invocation** — CiA-405-style lifecycle (create, start,
stop, reset, delete) over the ctl surface with epoch-disciplined
events. `management.md`.

## 5. Component map

| Component | Lang | Code | Spec |
|---|---|---|---|
| `tplcc` driver | Rust | `compiler/tplcc/` | `toolchain.md` |
| ST frontend | Rust | `compiler/tplc-st/` | `st-frontend.md` |
| MIR (types, layout, validation) | Rust | `compiler/tplc-mir/` | `ir-codegen.md` |
| FBD model / lowering / PLCopen IO | Rust | `compiler/fbd-model`, `fbd-lower`, `plcopen-io` | `fbd-model.md` |
| LLVM codegen + state layout + migrate gen | Rust | `compiler/tplc-codegen/` | `ir-codegen.md` |
| `tplc-rt` FB/stdlib (clean-room) | Rust | `compiler/tplc-rt/` | `ir-codegen.md` |
| Package writer/reader (Rust) | Rust | `compiler/tplc-pkg/` | `artifact-format.md` |
| Package reader + signature verify | C++ | `runtime/pkg/` | `artifact-format.md` |
| Frozen ABI/image/ctl/pkg headers | C | `include/tplc/` | `runtime-abi.md`, `process-image.md` |
| SHM image library + `tplc-image` tool | C | `runtime/image/`, `tools/tplc_image_tool.c` | `process-image.md` |
| Host runtime (`libtplc-host`) | C++ | `runtime/host/` | `plc-runtime.md` |
| `tether-plc` process binary | C++ | `runtime/tether-plc/` | `plc-runtime.md` |
| Bridge (image server, map runner) | C++ | `runtime/bridge/`, `runtime/tether-plc-bridge/` | `io-mapping.md` |
| `tplc-io` standalone tether_io provider | C++ | `runtime/tplc-io/` | `tether-io-attach.md` |
| L2 sim (`tether-plc-sim`) | C++ | `runtime/tether-plc-sim/` | `simulation.md` |
| ctl client + proxy, oracle adapters | C/C++/py | `tools/` | `testing.md` |
| Web IDE | TS/Lit | TetherWebUI repo | `web-ide.md` |

## 6. Wire & ABI surfaces

- **Module ABI** (`include/tplc/tplc_abi.h`): a 6-function surface —
  `tplc_abi_version`, `tplc_get_descriptor`, `tplc_init`,
  `tplc_shutdown`, `tplc_task_run`, `tplc_migrate` — plus the host
  `svc` table passed in via the context. C ABI, no exceptions/RTTI,
  no allocation after init. Loader-agnostic by design (the same ABI
  underlies the future no-`dlopen` MCU path).
- **Image protocol** (`include/tplc/tplc_image.h`): offset+seq
  seqlock; both roles implemented in `runtime/image/`.
- **ctl channel** (`include/tplc/tplc_ctl.h`, proto v3): binary
  request/reply over UDS. Op ranges: session `0x0100`, deploy
  `0x0200`, program `0x0300`, task `0x0400`, debug `0x0500`, retain
  `0x0600`, image `0x0700`, shadow `0x0800`, runtime `0x0900`, map
  `0x0C00`, journal `0x0D00`, events `0x0E00`.
- **tether_io**: catalog + functions/signals published by `tplc-io`
  or the bridge; schema epoch bumps invalidate stale clients on
  swap/promote.

Conventions: all multi-byte fields little-endian; sizes/offsets
`u32` unless noted; timestamps nanoseconds since monotonic epoch.

## 7. Documentation map

As-built specs (the normative layer): `artifact-format.md`,
`runtime-abi.md`, `process-image.md`, `plc-runtime.md`,
`st-frontend.md`, `fbd-model.md`, `ir-codegen.md`,
`online-change.md`, `io-mapping.md`, `toolchain.md`, `web-ide.md`,
`debugging.md`, `management.md`, `safety.md`, `security.md`,
`crypto-auth.md`, `mcu-path.md`, `simulation.md`, `testing.md`,
`shadow-run.md`, `tether-io-attach.md`, `error-codes.md`.

Reports/evidence: `acceptance-matrix.md` (criterion → test →
status), `perf-evidence.md`, `error-code-coverage.md`,
`threat-model-signoff.md`, `field-trial.md`, `release-notes.md`,
`DECISIONS.md`.

Still-active plan chapters (`plan/`): `17-milestones.md` (until v1
tag).

> **Provenance note.** References of the form `doc NN` / `doc-NN`
> (e.g. "doc 16 §3.2") cite the retired work-item chapters in
> `docs/plc/plan/`. Chapters 01–16 and 18–20 were deleted as their
> as-built specs landed; the section references survive as
> provenance — the authoritative content is the as-built doc
> containing the reference. Historical text is in git history.

## 8. Glossary

- **Active program** — the loaded program steering `%Q`.
- **Artifact / `.tplc`** — see §4.
- **FB (function block)** — IEC stateful unit (TON, CTU, …);
  instance state lives in the state blob.
- **Image latch** — consistent snapshot of the image taken at task
  start.
- **Layout hash** — content hash of the state-blob layout; drives
  migration compatibility.
- **Oracle adapter** — dev-only tool wrapping a GPL program (matiec,
  Beremiz) to emit expected-behavior test vectors; never shipped
  (`tools/oracle-*`).
- **Process image** — `%I`/`%Q`/`%M`/`%R` SHM exchange area.
- **Program invocation** — a loaded program instance (CiA-405:
  creation, start, stop, reset, delete).
- **Shadow program** — candidate consuming identical `%I`, outputs
  to a private region; promotable.
- **State blob** — see §4.
- **Symbol map (`symmap`)** — see §4.
- **WCET budget** — per-task declared worst-case execution time.

## 9. Design invariants (enforced, not aspirational)

- **No allocation in the scan path** after `init` completes.
- **No exceptions/RTTI** across the ABI boundary or in generated
  code.
- **Position-independent state** — the blob contains no pointers
  into itself.
- **Determinism** — same inputs + same initial blob ⇒ bit-identical
  outputs and blob, on target and in sim (tested by
  `sim_determinism`).
- **Diagnosability** — every failure path reports a structured error
  with a stable code (`error-codes.md`).
- **MCU-readiness** — nothing in generated code or the ABI assumes
  mmap/dlopen/files/threads; the MCU-shim conformance host is the
  standing proof.
