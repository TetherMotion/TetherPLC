# TetherPLC Design Goals

This is the keystone document for TetherPLC. It records **what we are building,
why, and which properties are non-negotiable**. Everything under
`docs/plc/plan/` is a work-item document that exists to satisfy the goals here
and is deleted once implemented; `docs/plc/` itself holds the permanent
as-built specifications. If a plan chapter and this document disagree, this
document wins — change this file first, deliberately, in a commit of its own.

Status: **draft under discussion**. Goals marked *(provisional)* are assumed
defaults pending explicit confirmation.

---

## 1. Vision

TetherPLC turns a Tether-based machine controller into a **programmable
soft-PLC**. A controls engineer — not a C++ developer — writes IEC 61131-3-style
logic (structured text or a function block diagram) in a browser IDE, compiles
it to native machine code via LLVM, and downloads it to a running system that
executes it deterministically while Tether performs all fieldbus I/O in the
background.

The product this is measured against is the Codesys/TwinCAT-shaped mental
model users already carry: scan-cycle execution, online change, forcing,
live watch on the diagram, retain variables, and a task configuration. Tether's
differentiators — a proven EtherCAT stack, FSoE, CiA profiles, a
schema-negotiated IO protocol with an existing web UI, simulation-in-CI — are
the floor this product stands on, not features it re-implements.

Tether's existing code-as-configuration API remains fully supported. The PLC
layer is an additional persona-facing product built on the same core, not a
replacement.

## 2. Personas

| Persona | Expectation |
|---|---|
| Controls / electrical engineer | Codesys-like browser IDE: ST and FBD as interchangeable views, live watch *in the diagram*, forcing, online change, retain vars, task cycle times |
| Machine builder / integrator | Compile-and-deploy pipeline, project files in version control, CI-simulatable programs, fleet rollout |
| Existing Tether user (C++) | PLC logic coexists with code-as-configuration apps on the same image and IO registry |
| Test / QA engineer | Run the *same compiled IR* against simulated slaves in CI with bit-identical semantics |

## 3. Non-negotiable design goals

### G1 — LLVM IR is the canonical program representation

All frontends (ST, FBD, and later SFC/LD) converge onto **one LLVM IR**. There
is exactly one backend/codegen. A shipped artifact package contains the
canonical **bitcode** plus the target-specific **shared object** and metadata —
the same IR is the input to simulation, cross-compilation, and on-target
execution.

*Rationale:* native speed on weak CPUs, retargeting across x86/ARM/RISC-V and
later MCUs from one toolchain, DWARF debug info, and a single semantic truth
for "does this program do what the editor shows".

### G2 — 100% native performance on very weak hardware

No interpreter, bytecode VM, or JIT in the on-target scan path. Generated code
is maximally static: no heap allocation inside the scan cycle, no exceptions or
RTTI in the runtime library, bounded loops, no blocking calls. The dynamic part
of the system lives exclusively in the *load/swap* path, never in the cycle.

The compiler itself may run on-device where the target is capable (x86 IPC
class); on constrained targets the identical artifact is produced by remote or
cross compilation. **Where compilation ran must be invisible to the artifact.**

### G3 — Deterministic scan-cycle execution

IEC task semantics: read inputs → execute programs → write outputs, driven by a
declared task configuration (cyclic tasks at multiple rates first; event and
freewheeling task kinds designed for). Execution rides Tether's cyclic
infrastructure (`CyclicTaskScheduler`-equivalent phasing, jitter monitoring).
Worst-case cycle time is measured, budgeted, and alarmed — never assumed.

### G4 — Online change is a v1 feature

Modify the program **while it runs**, preserving state, swapped at a cycle
boundary. This is the feature that separates a PLC from a firmware updater and
it gates v1.

Consequences that make this a *design* property rather than a feature patch:

- All mutable state (globals, function-block instances, retain memory) lives in
  one compiler-controlled, relocatable, ABI-versioned **state blob** — never in
  scattered `malloc` memory.
- The compiler generates a **`migrate` entry point** knowing both old and new
  layouts; type-unsafe changes are detected at compile/download time and either
  refused with diagnostics or applied under documented semantics.
- Atomic swap happens at a task boundary with a bounded pause budget
  (quantitative target in §6).

### G5 — Process-image decoupling

Compiled code touches **only** the process image (`%I` inputs, `%Q` outputs,
`%M` markers, retain area) — never driver, bus, or network APIs. Tether owns
all physical I/O: fieldbus PDOs are copied into the image before tasks run and
drained after. This yields: zero call overhead, deterministic timing, free
forcing/override semantics, a clean crash-isolation boundary, and an IO
mapping identical in spirit to every real PLC.

### G6 — Crash isolation via sidecar runtime

User logic executes in a dedicated `tether-plc` realtime process; the process
image is exchanged with the Tether application through shared memory. A fault
in compiled code must not take down the fieldbus: the runtime process is
independently watchdogged, restartable, sandboxable, and transitions outputs to
a defined safe state on failure. The ~1 µs image-copy cost per cycle is the
accepted price.

### G7 — Simulation parity

The identical canonical IR runs in hardware-free CI against Tether's simulated
IO (emulated slaves, simulated machine services). Program behavior in
simulation is bit-identical to on-target behavior for all deterministic
semantics; only wall-clock timing differs, and timing-sensitive tests use the
instrumented sim clock. "Your FBD runs in CI" is a core product promise.

### G8 — Debuggability by construction

The compiler emits a **symbol map** (name ↔ image/state offset, type, retain
flags, source provenance — including FBD node provenance, not just text lines).
This feeds the `tether_io` schema catalog so the web UI gets monitoring,
forcing, streaming, and scope traces of PLC variables without bespoke plumbing.
DWARF info is emitted where feasible to enable deeper source-level tooling.

### G9 — IEC interoperability

- **PLCopen XML (TC6)** is *the* exchange format — import/export compatibility
  with Codesys/Beremiz-style projects and a diff-friendly textual form for
  version control. FBD documents serialize to PLCopen XML.
- **CiA 405-style program management** semantics (program invocation,
  start/stop/reset, download state machine) model the runtime's remote control
  surface, consistent with Tether's existing CiA 405 device profile support.
- **IEC 61131-3 semantic conformance** for the implemented subset, verified by
  differential testing against reference implementations used strictly as
  dev-side oracles (see G12).

### G10 — Designed for iteration 2: SFC + LD

v1 ships ST + FBD. The architecture must absorb **SFC** (a sequencing layer
over the task model — steps, transitions, actions as task-bound FB calls) and
**LD** (rung semantics rendered over the same graph model as FBD) without
re-architecting the IR, ABI, task model, or editor model. Plan chapters carry
explicit "iteration-2 preparedness" notes.

### G11 — Safety and security boundary *(provisional)*

v1: safety-related state (FSoE connections, safety terminals) is **read-only**
to compiled logic; safety outputs are driven exclusively by the certified
safety configuration, never by user code. Artifact provenance (signing,
change control, audit trail via the existing event journal) is specified for
integrity and traceability even in v1. User-programmable safety logic is
explicitly out of scope — that is a certification program, not a feature.

### G12 — Clean licensing

Everything shipped is Apache-2.0/MPL-2.0-compatible (Tether's dual license).
The compatibility policy has three hard rules:

1. **PLCopen XML is the interoperability surface.** All import/export
   compatibility with the IEC ecosystem goes through TC6 XML — never through
   linking or embedding third-party code.
2. **GPL-licensed tools (matiec, Beremiz, OpenPLC) are oracles only, and only
   behind a separate adapter.** They are never compiled into, linked with, or
   shipped inside TetherPLC artifacts. All contact happens inside a dedicated,
   clearly quarantined adapter component (e.g. `tools/oracle-*`) that invokes
   GPL binaries as external processes and emits comparison/test data. The
   adapter's *outputs* are test vectors and expected-value traces — never
   source that enters shipped code. This quarantines GPL infectiousness to a
   dev-only subtree.
3. **The function block library is a complete clean-room implementation.**
   External libraries (OSCAT, ruSTy's stdlib, matiec output, vendor docs) may
   be *studied* for semantics and used as oracle expectations — no code, text,
   or generated output is copied. Shipped FBs are implemented from the IEC
   specification and observed behavior only.

MIT/Apache-licensed components (e.g. ruSTy, if adopted per D9) may be embedded
with attribution.

## 4. Locked architectural decisions

These were decided in design discussion and are inputs to every plan chapter.
Changing one requires editing this section first.

| # | Decision | Rationale |
|---|---|---|
| D1 | Runtime in a **sidecar process**, process image via shared memory | Crash isolation, independent watchdog/restart, sandboxing, clean safe-state story (G6) |
| D2 | Deployment artifact is an **ELF shared object**, `dlopen`'d by the runtime; canonical form is **bitcode** inside the artifact package | Simple, robust, no LLVM needed on device at runtime; bitcode stays off-device so LLVM version fragility is confined to toolchain/sim hosts we control |
| D3 | Compiler (frontends + IR codegen) implemented in **Rust**; runtime/loader in **C++** next to Tether | Rust parser ecosystem + ruSTy adjacency; runtime must share Tether's HAL/RT idioms. Boundary is the artifact file + CLI, not FFI |
| D4 | Compilation **on-device where capable**, remote/cross otherwise | "Appliance" feel on x86 IPCs; Pi1-class targets receive deployed artifacts. Same package either way |
| D5 | **Online change required in v1** | Killer PLC feature; v1 cannot ship "download while stopped" only |
| D6 | v1 languages: **ST + FBD**; iteration 2 adds **SFC + LD** | Controls-engineer minimum viable surface; architecture pre-accommodates (G10) |
| D7 | FBD editor: **TypeScript reimplementation in TetherWebUI** (Lit + canvas), rustylink-inspired data model serialized to **PLCopen XML** | Consistent stack and UX; rustylink contributes model/editor concepts, not code |
| D8 | FBD and ST share **one IR**; FBD is lowered to the same backend | Interchangeable views, one semantics, one conformance story |
| D9 | Compiler frontend is an **evaluated decision**: adopt ruSTy wholesale / reuse ruSTy frontend with own codegen / own frontend | Live-update and debugging requirements (state-layout control, FBD-provenance debug info) may disqualify full adoption; the plan contains a dedicated decision chapter |
| D10 | **MCU path designed-for, implemented later**: ABI spec is loader-agnostic; future MCU loaders (PIC blob + ELF loader, OTA-linked module) reuse the same ABI | Linux `.so` is the first loader backend, not the only possible one |
| D11 | **SHM image protocol** is the sole runtime↔master coupling | Testable without EtherCAT; enables crash isolation and forcing |
| D12 | **Compatibility policy**: PLCopen XML as the exchange format; GPL tools as oracle-only inside quarantined adapter code; function block library fully clean-room | Interop without GPL infectiousness (G9, G12) |

## 5. Explicit non-goals

- Full IEC 61131-3 conformance (all five languages, all annexes) — we ship a
  conformant *subset*; conformance claims are per-feature.
- IEC 61499 event-driven function blocks (4diac/FORTE paradigm) — different
  programming model; out of scope.
- Continuous-time simulation / solver semantics (Simulink-style) — the scan
  model is discrete by design; the block diagram compiles to cyclic code.
- Safety-rated user logic (see G11).
- On-device compilation on hardware too weak to host the toolchain — remote
  compile covers these targets.
- LLM-assisted program generation — **unresolved**: README references "LLM";
  assumed LLVM typo. If intentional, this becomes a goal + plan chapter.

## 6. Quantitative draft targets

To be refined during implementation planning; recorded here so goals stay
honest.

| Metric | Draft target |
|---|---|
| Baseline PLC cycle rate | 1 kHz (matches Tether `RealtimeLoop` default); multi-rate tasks 100 Hz–4 kHz |
| Online-change pause injected into scan | ≤ 1 task cycle, preferably < 500 µs |
| Process image size | ≤ 64 KiB per area (%I/%Q/%M) initially |
| Max IEC tasks per runtime | 32 |
| State blob migration failure detection | 100% of type-unsafe changes detected at compile/download time |
| Program fault containment | master unaffected; outputs to safe state ≤ 1 cycle after watchdog trip |
| Minimum Linux target | Pi1-class armv6 (deployed artifacts only); compile-capable floor = x86-64 IPC |
| Sim determinism | bit-identical IO traces vs target for identical input sequences |

## 7. Document map

- `docs/DESIGN-GOALS.md` — this file. Permanent. Changed rarely, deliberately.
- `docs/plc/plan/NN-*.md` — work-item chapters. Each describes **what remains
  to be implemented** for one subsystem, its acceptance criteria, and its
  dependencies. **Deleted when implemented** (moved knowledge lives in the
  as-built spec, not the plan).
- `docs/plc/*.md` — permanent as-built specifications, created as chapters are
  completed (ABI spec, IR codegen spec, FBD model spec, …).
- `docs/plc/DECISIONS.md` — (to be created) append-only log of decisions made
  during implementation that are not yet covered by §4.

## 8. Open questions

| # | Question | Owner |
|---|---|---|
| Q1 | "LLM" in README — typo for LLVM, or LLM-assisted program generation goal? | user |
| Q2 | G11 provisional: safety IO read-only in v1 — confirmed, or is a certified safety FB library in ambition? | user |
| Q3 | MCU: confirmed as design-for-now / implement-later (D10)? | user |
