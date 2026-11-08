# 17 — Milestones: Phased Roadmap and Exit Criteria

> **Status:** work-item document; lives longest of the plan chapters —
> updated (not deleted) as phases land; deleted when v1 ships and
> remaining items are re-planned under iteration 2.
>
> **Satisfies:** scheduling/risk view of all goals. **Depends on:**
> everything; it is the chapter that orders the rest.

---

## 1. Ordering logic

1. **De-risk the runtime before the compiler exists.** The ABI/image/
   runtime loop is provable with a hand-written stub `.so` — build it
   first (P0). Compiler work then lands on proven ground.
2. **Contracts first, tracks second.** Docs 01–03/09/16 freeze the
   interfaces; then compiler (Rust), runtime (C++), and UI (TS)
   tracks can run in parallel without blocking each other.
3. **Online change is not a phase you add — it's a property you
   protect.** Layout machinery ships with the first compiler build;
   the swap itself lands as soon as codegen can emit `tplc_migrate`
   (P3) — early enough that nothing ossifies against it.
4. **User-visible value early:** cold-run (P1), hot-swap (P3),
   browser IDE (P5) are the demo checkpoints.
5. **Everything behind acceptance criteria:** each phase exits only
   when its doc-14 suites are green — no "done except tests".

## 2. Phases

> **Status board (as of the P10 sweep):** P0–P9 **landed** — their
> chapters (01–16) are retired into `docs/plc/` as-builts and the
> acceptance matrix (`docs/plc/acceptance-matrix.md`) is the live
> record. P10 is in flight: this doc survives until v1 ships and the
> remainder re-plans under iteration 2.

### P0 — Foundations & runtime skeleton
*Scope:* `libtplc-pkg` reader (C++/Rust), `tplc_abi.h`, image lib +
`tplc-image` tool, `libtplc-host` skeleton (loader/scheduler/lifecycle
shell), `tether-plc` binary, control channel frame, hand-written stub
`.so` module, conformance harnesses for 01–03.
*Exit:* stub module runs cyclic tasks against a virtual image through
the real ABI + SHM protocol; kill/restart/safe-state paths proven;
CI suites exist.
*Size:* L. *Unlocks:* everything.

### P0.5 — Frontend spikes (DECISIONS.md, runs inside P0)
*Exit:* S1–S6 verdicts in DECISIONS.md; path chosen; 05/07 sections
marked active.
*Size:* S–M. *Hard gate for P1.*

### P1 — Compiler MVP (ST path)
*Scope:* ST frontend (per eval) → `tplc-mir` → sema → layout → LLVM →
`.so`+`.bc`+symmap+layout_desc → package writer; tplc-rt core (logic/
timers/counters/math/convert); `tplcc build/check/info/verify`;
cold deploy via control channel; AT binding incl. wildcards (simple
map); io_bindings path.
*Exit:* ST programs compile→deploy→run against virtual image; oracle
vector suite (initial corpus) green; **DEMO: cold-run end-to-end**.
*Size:* XL. *Unlocks:* P3 (migrate), P4 (FBD lowering), P5 (deploy UI).

### P2 — Runtime completeness
*Scope:* retain store, watchdog+budget enforcement, fault policies,
runbook+respawn, monitor/checkpoint interplay, full control channel,
bridge integration vs emulated slaves (L2/L3), catalog export v1.
*Exit:* doc-04 acceptance; fault-injection suite green; programs
survive restart with retain intact; **DEMO: crash-the-PLC
transparency** (kill -9 → safe-state → respawn → resume).
*Size:* L. *Parallel with P1* (contract: docs–04).

### P3 — Online change
*Scope:* `--change-against` pipeline, migrate codegen, snapshot/
stage/arm/commit in runtime, rollback+quarantine, deploy modes,
migration-preview data feed.
*Exit:* doc-08 acceptance incl. continuity tests; **DEMO: edit ST →
hot-swap a running program with live watch** — the signature demo.
*Size:* L. *Requires:* P1 (codegen), P2 (runtime).

### P4 — FBD language
*Scope:* `fbd-model` (Rust+TS types), `plcopen-io` (strict+roundtrip),
`fbd-lower` → mir, validation rules, probes, gallery corpus.
*Exit:* doc-06 acceptance; FBD compiles and runs identically to
equivalent ST; **DEMO: draw diagram → compile → run**.
*Size:* L. *Requires:* P1 (mir/codegen), parallel with P3.

### P5 — Web IDE
*Scope:* explorer, ST editor (CM6+lezer), FBD canvas (rustylink-style
interactions), tasks/io-map editors, deploy console + migration
preview, watch/force panels, runtime view, sim-target flow.
*Exit:* doc-11 acceptance; **DEMO: zero-terminal commissioning** in
the browser.
*Size:* XL. *Requires:* P4 (canvas), P3 (preview), P2 (ops surface).

### P6 — Debug plane completeness
*Scope:* resolve/watch/force/write, suspend/step/break_when, trace +
flight-recorder, symmap→catalog epoch handling, scope integration.
*Exit:* doc-12 acceptance incl. determinism-under-load proof.
*Size:* M–L. *Requires:* P2; enhances P5.

### P7 — Sim/oracle/CI hardening
*Scope:* L1/L2/L3 sims productized, oracle adapters + vector corpus,
fuzzers nightly, coverage/mutation gates, determinism CI gates, perf
bench trend dashboard.
*Exit:* doc-14 acceptance; sim-parity suite green on full corpus.
*Size:* M (ongoing from P1 — formalized here).
*Continuous from P1, formal gate before v1.*

### P8 — Safety & security
*Scope:* sandbox profile, signing+trust store, post-link scanner,
authZ/role enforcement, audit journaling completeness, safety-boundary
UX affordances.
*Exit:* doc-15 acceptance; threat-model signoff.
*Size:* M. *Requires:* P2; blocks "production" claims.

### P9 — Management surface
*Scope:* full op catalog per doc 16, prog.status, eventing, epoch
discipline, role gating.
*Exit:* doc-16 acceptance.
*Size:* M. *Requires:* P2/P6.

### P10 — v1 hardening & release
*Scope:* acceptance sweep of every remaining chapter, perf-gate
evidence vs DESIGN-GOALS §6, docs/plc as-built specs written (plan
chapters die here), field-trial checklist, release notes + known-
limits page.
*Exit:* **v1 tag.**
*Size:* M (calendar-heavy, code-light).

## 3. Track/dependency map

```
P0 ──┬─► P0.5 (spikes)
     ├─► P1 compiler ──► P3 online-change ─┐
     │        └────────► P4 FBD ───────────┤
     ├─► P2 runtime ────┬──────────────────┤
     │                  ├─► P6 debug ──────┤
     │                  └─► P8 security ───┤
     └─────────────────────────────────► P5 web IDE ─► P10
             P7 hardening (continuous) ────┘  P9 mgmt ┘
```

Parallel-safe after P0: {P1} ∥ {P2} ∥ {P7-drip}; after P1:
{P3, P4, P6-drip} ∥ ; UI track (P5) can scaffold against mock
backends from day 1 — contracts are frozen docs, not code.

## 4. Risk register (top)

| risk | phase | mitigation |
|---|---|---|
| ruSTy boundary unusable → own-frontend cost | P0.5/P1 | fallback is planned (C); spike timeboxed; mir boundary makes the swap local |
| online-change edge-case blowup (FB type change semantics) | P3 | conservative deny table; compile-time gate means users see refusals early |
| LLVM toolchain weight on-device (Mode A) | P1/P10 | static-link; resource caps (toolchain.md); remote mode is the escape |
| debug-plane determinism surprises | P6 | boundary-apply model + load-shed rules; prove early with a stress fixture in P2 |
| seqlock/image protocol bug under adversary timing | P0/P7 | TSan builds + fault-injected adversary suite — cheapest place to find it is P0 |
| FBD editor scope creep | P5 | MVP interaction set fixed in web-ide.md; LD/SFC explicitly deferred |
| scope of "1 program" vs multi-instance UX | P2+ | design is multi-instance; v1 UX may expose one — honest scoping note |

## 5. Iteration 2 (post-v1, recorded not scheduled)

SFC language + editor; LD view/editor; multi-program IDE flows;
CiA-405 OD endpoint (doc 16 §7); MCU loader path (doc 18); shadow
run / compare-and-promote deploys (doc 19 — spec exists, may pull
forward as a v1.x feature); WSTRING/LTIME/REF dialect extensions;
collaborative editing; visual diff; OPC-UA adapter; safety FB
subset (certification program).

## 6. Definition of done (per phase)

All of: chapter acceptance criteria green; as-built doc written; error
codes tested; perf numbers recorded; license/scan gates clean;
DECISIONS.md updated for every deviation from the plan docs.

---

## Appendix K — Work breakdown (task level, per phase)

Effort classes: XS<1w, S=1–2w, M=2–4w, L=1–2mo, XL=2mo+ (single-dev
calibration; parallelize per §3).

### P0 — Foundations & runtime skeleton

| id | task | size | depends |
|---|---|---|---|
| P0-01 | `tplc_abi.h` + `tplc_image.h` published (Appendix A/B → code) | S | — |
| P0-02 | `tplc-pkg` Rust reader/writer + golden fixtures | M | P0-01 |
| P0-03 | C++ `tether::plc::Package` reader + conformance | M | P0-02 |
| P0-04 | image lib (seqlock SHM, both roles) + `tplc-image` tool | M | P0-01 |
| P0-05 | `libtplc-host` skeleton: loader (dlopen+verify), scheduler | L | P0-01..04 |
| P0-06 | control channel framing + session bootstrap | M | — |
| P0-07 | `tether-plc` binary: CLI/config/privdrop/logging | M | P0-05 |
| P0-08 | hand-written stub module (C) + ABI conformance suite | M | P0-05 |
| P0-09 | fault-injection harness (kill, stall, seq-adversary) | M | P0-04 |
| P0-10 | CI wiring: qemu-aarch64 runner + coverage plumbing | S | — |

### P0.5 — spikes (DECISIONS.md §7)

S1–S4 (ruSTy consumption + elimination check) — S/M; S5 (B-path
end-to-end POU) — S; S6 (own-parser prototype) — S.

### P1 — Compiler MVP

| id | task | size | depends |
|---|---|---|---|
| P1-01 | `tplc-mir` crate: types, builders, validation | M | P0.5 |
| P1-02 | ST frontend → mir (chosen path) | XL | P0.5 |
| P1-03 | sema: names/types/conversions/call-shape/AT | L | P1-01 |
| P1-04 | layout engine → layout_desc + symmap + type_catalog | L | P1-03 |
| P1-05 | LLVM lowering (stmts/exprs/calls/checkpoints) | XL | P1-01 |
| P1-06 | tplc-rt core: logic/edge/timer/counter/math/convert/string | L | P1-05 |
| P1-07 | package writer + target codegen (x86_64, aarch64, armv6) | L | P1-05,P0-02 |
| P1-08 | `tplcc` CLI (build/check/info/verify/map-check) | M | P1-07 |
| P1-09 | descriptor/io-binding tables + host integration (prepare→run) | M | P1-04,P0-05 |
| P1-10 | oracle adapter + initial vector corpus | M | P1-08 |
| P1-11 | diagnostics product layer (rendering, explain) | M | P1-02 |

### P2 — Runtime completeness

| id | task | size | depends |
|---|---|---|---|
| P2-01 | retain store (mmap file, dirty hints, flusher) | M | P0-05 |
| P2-02 | watchdog/budget/checkpoint interplay + abort paths | M | P0-05 |
| P2-03 | fault policies + runbook + respawn + supervisor | M | P0-07 |
| P2-04 | control channel full op surface (Appendix I) | L | P0-06 |
| P2-05 | `tether-plc-bridge`: image server, map runner, supervisor | L | P0-04 |
| P2-06 | catalog exporter (symmap→tether_io) v1 | M | P2-05 |
| P2-07 | L2/L3 sim backends + process-sim runner | M | P0-05 |
| P2-08 | map validator + iomap schema impl | M | — |

### P3 — Online change

| id | task | size | depends |
|---|---|---|---|
| P3-01 | `--change-against` + live layout query path | M | P1-08,P2-04 |
| P3-02 | layout diff → rule table + migrate_desc emission | L | P1-04 |
| P3-03 | `tplc_migrate` codegen (copy/transform/deny/alias) | L | P3-02 |
| P3-04 | runtime: snapshot/stage/arm/commit/quarantine/rollback | L | P2-03 |
| P3-05 | deploy modes + deny-report surfacing | M | P2-04 |
| P3-06 | conformance: continuity fixtures + race + rollback | M | P3-04 |

### P4 — FBD language

| id | task | size | depends |
|---|---|---|---|
| P4-01 | `fbd-model` crate + JSON schema + canonical io | M | — |
| P4-02 | `plcopen-io` strict+roundtrip | L | P4-01 |
| P4-03 | `fbd-lower` → mir (topo, cycles, probes, EN/ENO) | L | P1-01 |
| P4-04 | validation rules FBD001–14 + diagnostics | M | P4-03 |
| P4-05 | gallery corpus + equivalence-vs-ST tests | M | P4-03 |
| P4-06 | Beremiz corpus import fixtures | S | P4-02 |

### P5 — Web IDE

| id | task | size | depends |
|---|---|---|---|
| P5-01 | plc/ shell: explorer + project model + stores | M | — |
| P5-02 | ST editor (CM6 + lezer grammar + completion) | L | P5-01 |
| P5-03 | FBD canvas: render + palette + wiring + undo | XL | P5-01,P4-01 |
| P5-04 | live overlay (wire values, badges, force chips) | L | P5-03,P6-* |
| P5-05 | tasks + io-map editors + auto-map wizard | L | P2-08 |
| P5-06 | deploy console + migration preview + sim target | M | P2-04,P3-05 |
| P5-07 | watch/force panels + runtime status views | M | P5-01 |
| P5-08 | PLCopen import/export UX + project zip io | M | P4-02 |
| P5-09 | role gating + authority UX + audit surfaces | M | P8-03 |

### P6 — Debug plane

| id | task | size | depends |
|---|---|---|---|
| P6-01 | resolve + epoch re-resolution + stale handling | M | P1-04 |
| P6-02 | watch add/get/stream + boundary-apply queue | M | P2-04 |
| P6-03 | write/force tables + application points | M | P0-05 |
| P6-04 | suspend/step/break_when + expr evaluator | M | P0-05 |
| P6-05 | trace rings + flight-recorder + scope export | L | P0-05 |
| P6-06 | determinism-under-load proof + watermark shed | M | P6-02 |

### P7 — hardening (continuous; formal gate pre-v1)

suites per doc-14 §4; fuzzers; coverage/mutation gates; perf bench
trending; GPL hygiene scan.

### P8 — Safety & security

| id | task | size |
|---|---|---|
| P8-01 | sandbox profile (user, seccomp, rlimits, ro-root) | M |
| P8-02 | signing: `tplcc sign` + trust store + policies | M |
| P8-03 | authZ roles + step-up + audit journaling | M |
| P8-04 | post-link scanner (syscall/PLT whitelists) | M |
| P8-05 | safety-boundary enforcement tests + UX affordances | S |
| P8-06 | license manifest + CI gate | S |

### P9 — Management surface

op catalog per doc-16; prog.status object; eventing; epoch wiring —
M each ×4.

### P10 — v1 hardening & release

acceptance sweep, as-built docs, perf evidence, field-trial prep —
calendar-heavy.
