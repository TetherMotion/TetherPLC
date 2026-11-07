# 10 — Toolchain: `tplcc`, Compile Service, and Deploy Path

> **Status:** work-item document; deleted when `docs/plc/toolchain.md`
> exists and both deployment modes ship packages end-to-end.
>
> **Satisfies:** D4 (on-device where capable / remote otherwise), D3
> (Rust), G1 (canonical IR pipeline), G12 (clean toolchain).
> **Depends on:** 01, 05–09. **Informs:** 11, 14, 17.

---

## 1. What the toolchain is

`tplcc` is the compiler driver + a thin service wrapper. Two deployment
modes share **one artifact path** (D4):

- **Mode A — on-device**: `tplccd` (compile daemon) runs on the target
  (x86 IPC class or better). The web IDE compiles in place; deploy is a
  local hand-off to `tether-plc`'s control channel. "Appliance" UX: one
  box, browser → run.
- **Mode B — remote/cross**: `tplcc` runs on the engineering host (or
  CI), produces a `.tplc` carrying the *target's* `.so`; the IDE (or
  CLI) deploys over the `tether_io`-proxied control channel. Required
  for Pi1-class and all "too weak for LLVM" targets.

The artifact cannot tell which mode produced it (identical package
spec); the difference is only *where the bytes were made* and *how the
deploy reached the device*.

## 2. `tplcc` — the compiler driver

### 2.1 Interface

```
tplcc build <project>            → pkg.tplc       (all targets configured)
tplcc build --target <triple>    → single-target
tplcc build --change-against <src> → online-change pkg
        src = "live:<host>" | file.tplc | layout_desc.bin
tplcc check <project>            → diagnostics only (no emit)
tplcc fmt / lint                 → source hygiene
tplcc info / verify <pkg.tplc>   → inspect/validate package
tplcc map-check <pkg> <iomap>    → AT-binding + requires validation
tplcc explain <code>             → diagnostic documentation
tplcc serve [--port]             → compile daemon mode (tplccd)
```

### 2.2 Project model

`tplc.toml` (project file — human-authored, editor-maintained):

```toml
[project]   name="fill_station" uuid="…" version="1.4.0"
[sources]   st=["src/*.st"] fbd=["src/*.fbd.json"]
            plcopen="project.plcopen.xml"   # alternative import root
[targets]   triples=["x86_64-unknown-linux-gnu",
                     "armv6-unknown-linux-gnueabihf"]
[compile]   opt_level=2 checks="default" embed_sources=true
[online]    program_uuid="…"            # stable identity for changes
[mapping]   auto_bind="by-label"        # or explicit table file
```

`program_uuid` persistence is the engineer's handle on online-change
identity; generated once per project, lives in VCS.

### 2.3 Compile pipeline (internal stages)

```
load (ST files | FBD json | plcopen import)
  → frontends → tplc-mir merge → sema → layout
  → llvm module (link tplc-rt .bc) → opt pipeline
  → per-target codegen → .so(s)
  → metadata: symmap/type_catalog/layout_desc/migrate_desc?
  → package writer → .tplc  (+ build report JSON)
```

Deterministic (doc 01 §13); `--report` emits layout/mapping/diagnostic
summaries for CI review.

### 2.4 Cross-compilation

- LLVM target triples cover codegen; the harder part is the **target
  sysroot** for linking: `.so` must link against target libc or avoid
  libc entirely.
- Strategy: **minimize libc surface to near-zero** — generated code +
  tplc-rt need only memcpy/memset/compiler intrinsics (`libgcc_s`
  builtins subset) → link `-nostdlib` + vendored `compiler-rt`
  builtins ⇒ **sysroot-free cross .so** for `armv6`, `aarch64`,
  `riscv64`, `i686`… (manifest `link:"self-contained"`). glibc-targeted
  builds stay an option when rt grows libc needs — the design goal is
  that it never does.
- This is a *big* robustness win vs the OpenPLC model (which needs gcc
  + headers + libs on every target/CI matrix row) — one Rust toolchain
  serves all targets.
- On-device (Mode A) is then just "run the same binary natively".

### 2.5 `tplccd` — compile daemon

- Serves the same ops as the CLI over a versioned JSON-RPC/REST surface
  on loopback (+ optionally LAN w/ auth, doc 15): `compile`,
  `compile-change` (auto-fetches live layout via local control
  channel), `diagnose`, `package.get`, `targets.list`.
- Long-compile streaming (progress + diagnostics incremental).
- Resource bounds: concurrent compiles `nproc/2`, per-compile memory
  cap, timeout — an appliance must not OOM its own control process
  (doc 15 resource isolation).
- Auto-mode: `compile-on-deploy` — the IDE sends the *project*; daemon
  compiles + deploys atomically.

## 3. Deploy path

### 3.1 Deploy API (target side)

On the runtime control channel (doc 04 §9), proxied by the bridge for
remote clients (doc 09 §4):

| op | semantics |
|---|---|
| `deploy.begin{size,sha256}` | open staged upload; returns upload_id |
| `deploy.chunk{upload_id,off,data}` | stream package |
| `deploy.commit{upload_id, mode}` | verify + stage per doc 04/08; `mode = cold | online-change | auto` |
| `deploy.status{program_uuid}` | running program identity + layout_desc (the `--change-against` feed) + package hash |
| `deploy.abort{upload_id}` | drop staged |
| `pkg.list / pkg.info` | inventory incl. runbook vs deployed |

Integrity: sha256 checked before parse; signature policy (doc 15)
applied at `commit`, not at upload.

### 3.2 Deploy flows

- **Cold deploy** (new program / stopped): verify → prepare → ready →
  start per operator.
- **Online change**: `mode=online-change` → doc 08 pipeline; mode
  `auto` = runtime picks (layout-hash equal ⇒ fast swap; has migrate
  ⇒ staged; else requires explicit `cold` flag).
- **Rollback**: `prog.rollback` (doc 08 §7).
- Every transition journaled with package hash + operator identity
  (audit trail, G11-adjacent).

### 3.3 Fleet note

Mode B over `tether_io` means the *same* deploy API reaches a fleet
through the master's existing session/auth machinery — fleet
orchestration is a later product chapter, but no deploy-protocol
changes are needed for it (packages are self-contained).

## 4. Build & distribution of the toolchain itself

- `tplcc`/`tplccd` = single Rust workspace; binaries
  `x86_64/aarch64 linux` + musl static variants for appliance images.
- LLVM linkage: `llvm-sys`/inkwell against **a pinned LLVM major**
  (v1: LLVM 21) — statically linked into tplcc (no system-llvm
  dependency on device — the appliance carries its toolchain without
  distro-llvm coupling; binary size ~40–60 MB accepted, noted).
- `tplc-rt` ships inside `tplcc` as embedded bitcode (version-locked
  with the codegen that inlines it).
- The C++ side consumes only the **package format** + control protocol
  — no LLVM on the runtime side ever (G2).

## 5. CI/developer ergonomics

- `tplcc check` on pre-commit (fast path — no emit).
- `tplcc build` in CI → `.tplc` as build artifact + sim tests against
  it (doc 14) — the compiled program itself is the test subject.
- `tplcc verify` + `tplc-image dump` + `deploy.status` give ops a
  complete inspection loop without source access.
- Determinism gate: CI rebuild twice, byte-diff modulo uuid (doc 01).

## 6. Iteration-2 hooks

- `--target` already a list (MCU payloads become another target kind —
  PIC-blob emitter is a codegen backend, not a toolchain change).
- `tplc serve` is where an LLM-assist or headless-IDE compile endpoint
  would attach (open question Q1 — the daemon surface is
  agent-ready by design: ops are project-in, artifact-out).
- SFC/LD add source kinds to `[sources]`; pipeline unchanged.

## 7. Error codes (owned by this chapter)

`TPLC_E_TC_*` (`_TC_PROJECT, _TC_TARGET, _TC_SYSROOT?, _TC_EMIT,
_TC_DEPLOY_*, _TC_LIVE_QUERY, _TC_SIG_REQUIRED`)

## 8. Acceptance criteria

1. `tplcc` builds the full reference corpus on x86_64 and emits
   multi-target packages incl. armv6 — `.so` verifiably
   self-contained (no NEEDED beyond whitelist; readelf check in CI).
2. Mode A: on-device `tplccd` compile→deploy→run on an x86 IPC fixture.
3. Mode B: host compile → deploy over tether_io to a Pi-class target
   (or qemu-aarch64 fixture) → identical traces.
4. `deploy.*` ops cover §3.1 semantics incl. abort and staged integrity.
5. Live-query: `--change-against live:` produces a valid
   online-change package against a running fixture.
6. Determinism: CI double-build byte-diff gate.
7. No LLVM/distro toolchain required on the runtime host (appliance
   image proof: runtime device has only tether-plc + bridge).

---

## Appendix N — `tplccd` service API (`tc.api.v1`, normative)

JSON-RPC 2.0 over HTTP (loopback default; LAN bind opt-in w/ token —
doc 15). Streaming ops use `id`-correlated notifications.

### N.1 Methods

| method | params → result | notes |
|---|---|---|
| `compile` | `{project_ref, targets?, flags?} → job_id` | stream: `compile.progress` + `compile.diagnostic` notifs; result `{package_id, bytes, sha256, diagnostics}` |
| `compileChange` | `{project_ref, against:"live"|package_id|layout_file} → job_id` | result adds `{migration:{preserved,initialized,dropped,denied,rules[]}}` — the IDE preview feed (doc 11 §7) |
| `diagnose` | `{project_ref|sources[]}` | check-only; structured diagnostics `[{code,sev,span,msg,notes[]}]` |
| `package.get` | `{package_id}` → package bytes | stored artifacts (bounded store, LRU evict) |
| `package.list` | — | inventory incl. lineage |
| `targets.list` | — | supported triples + link kinds |
| `layout.fetch` | `{program_uuid}` → `{layout_desc, hash}` | proxies runtime deploy.status live query |
| `deploy.push` | `{package_id, target:"local"|remote_ref, mode}` | performs deploy.* on the control channel; streams staged/committed events |
| `project.create/import/export` | project ops for the IDE | import accepts plcopen/fbd.v1/st zip |
| `catalog.get` | `{flavor:"blocks"|"symbols"}` | tplc-rt block catalog (palette feed) / project symbol index (completion feed) |
| `explain` | `{code}` | diagnostic docs |
| `auth.ping` | token check + role | editor role discovery |

### N.2 Conventions

- Jobs are async: `{state:queued|running|done|error|cancelled}` +
  `job.status`/`job.cancel` methods; `job_id` dedup'd.
- Compile resource limits enforced per doc 10 §2.5; overload →
  `TC_OVERLOAD` with queue position.
- Errors: JSON-RPC `{code,message,data:{tplc_code,subcode}}` — tplc
  codes from the owning chapters.
- `compileChange against:"live"` fails `TC_LIVE_QUERY` if the target
  runtime is unreachable — IDE offers `--offline` path (against a
  package_id).
