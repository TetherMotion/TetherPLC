# 14 — Simulation & Testing: Sim Host, Oracle Harness, CI Strategy

> **Status:** work-item document; deleted when `docs/plc/simulation.md`
> and `docs/plc/testing.md` exist and the sim/oracle/CI machinery is
> green on the reference corpus.
>
> **Satisfies:** G7 (sim parity — the headline goal of this chapter),
> G12 r2 (GPL oracles quarantined here), G2/G3 (perf+jitter evidence).
> **Depends on:** all implementation chapters. **Informs:** 17.

---

## 1. What "simulation" means here

Three nested levels, all using **the same canonical IR** (G1/G7):

| level | what runs | host |
|---|---|---|
| L1 unit-sim | a single `.tplc` in-process with virtual image/clock | `libtplc-host` embedded in test binary (`ImageBackend::Virtual`, `ClockSource::Virtual`) |
| L2 process-sim | `tether-plc` binary against a **sim-published SHM file** | real process, simulated master side — exercises the real doc-03 protocol incl. failure modes |
| L3 system-sim | the artifact inside Tether's existing simulated-plant fleet (emulated slaves, `SimulatedMachineService`) | end-to-end: map → bridge → image → runtime → virtual IO → simulated machine physics |

The `.so` under test is byte-identical to the deployed one (the
package's `target_obj` for the sim host's triple — emitted into every
package by default, doc 01 §4). Bitcode JIT on host is a fallback path
only; **the preferred sim target is the host .so** — what runs in CI is
what ships.

## 2. Determinism contract (what G7 promises)

For an identical initial state blob and an identical **input sequence**
(latched `%I` values + control ops + forces), a program produces
**bit-identical**: `%Q` commit sequences, blob contents at each
boundary, watch/trace samples, and fault events — across target, L1,
L2, L3 hosts.

Mechanisms making this true (already spec'd, tested here):

- all time comes through `ctx->now_ns`/`svc->steady_ns` — sim hosts
  inject a virtual clock driving the *sequence* deterministically;
- all IO comes through the latched image — sim injects sequences;
- no nondeterminism sources inside generated code (doc 02 §8, doc 05
  §10) — tested by the "same-input-twice" property suite;
- scheduling differences between hosts may change *when* tasks run but
  never *what* they compute — the trace comparator therefore works on
  value sequences per task-cycle-index, not wall time. Timing-sensitive
  tests drive the virtual clock through the same release schedule.

Divergence = a defect class of its own (`TPLC_E_SIM_DIVERGENCE`
reported with first-divergent cycle + field).

## 3. The oracle harness (`tools/oracle-*`, G12 quarantine)

GPL tools (matiec first; Beremiz corpus for FBD cases) are used
**only** as dev-side oracles inside quarantined adapters:

```
tools/oracle-matiec/           (dev-only; never shipped, never linked)
 ├─ adapter: ST source → matiec (GPL binary, subprocess) → C → gcc
 │          → reference harness → expected IO traces (JSONL vectors)
 ├─ corpus: IEC conformance cases + dialect §14 corpus
 └─ compare: run our artifact on same vectors → trace diff
```

Rules (enforcing G12):

- The adapter **executes GPL binaries as subprocesses** — no code from
  them is compiled into, linked with, or copied into anything shipped;
  its outputs are *data* (expected traces), not code.
- `tools/oracle-*` lives behind a feature flag excluded from release
  builds; CI license-scan asserts no GPL artifacts in shipped paths.
- Where matiec can't run a construct (e.g., our extensions), the vector
  is marked `ours-only` and expected behavior comes from the spec —
  never silently assumed.
- The FBD corpus uses Beremiz-exported PLCopen XML as *test inputs*
  (import fidelity) — again inputs, not code.

### 3.1 Vector format

```
{ "id": "iec.ton.basic", "program": "ton_case.st",
  "cycles": [{"i": "0001…hex", "t_ns": 1000000,
              "expect_q": "…hex", "expect_fields": {"Ton.ET": "T#150ms"}}],
  "source": "matiec-12.3 | spec | manual" }
```

Vectors drive: L1 sim, L3 sim, *and* (where hardware exists) target
runs — one vector set, three evidence levels.

## 4. Test taxonomy

| suite | scope | where |
|---|---|---|
| `pkg-conformance` | doc-01 reader/writer/fuzz | CI |
| `abi-conformance` | doc-02 §11 suite incl. moving-host, panic, bad_* | CI (+qemu-aarch64) |
| `image-conformance` | doc-03 protocol, crash-mid-write, staleness, safe-state | CI (L2) |
| `dialect-conformance` | doc-05 positive/negative ≥400 cases | CI |
| `fbd-conformance` | doc-06 rules + lowering goldens + XML roundtrips | CI |
| `rt-semantics` | tplc-rt units + oracle diffs + timer waveforms | CI |
| `migrate-conformance` | doc-08 fixtures: all rules, denies, races, rollback | CI (L1+L2) |
| `io-trace-equivalence` | our-vs-oracle trace diffs on shared vectors | CI (nightly, heavier) |
| `sim-parity` | same vectors on L1/L2/L3/target → bit-identical | CI (+HW lab) |
| `fault-injection` | kill -9 paths, hang (WHILE TRUE), seq-odd, runbook restore | CI+lab |
| `perf-jitter` | cycle-time, pause budgets, image copy cost, debug-load jitter | nightly + lab |
| `fuzz` | package reader, mir, image seqlock adversary, ST parser, FBD model, debug ops | nightly |
| `webui-e2e` | doc-11 flows via Playwright | CI |

Every plan chapter's acceptance criteria map onto these suites — the
deletion rule for a chapter is "its suite exists and is green".

## 5. Fuzz/property strategy (beyond the table)

- **Differential fuzzing** on the frontend: random/grammar-mutated ST →
  our compiler vs oracle compiler; divergence in *accept/reject* is
  investigated (both may differ legitimately — triage taxonomy, not
  auto-bug).
- **Blob-migration fuzz**: random field-add/remove/retype pairs →
  migrate must produce `OK|DEGRADED|DENY` per the rule table — never
  corrupt (property: post-migrate blob validates against new
  layout_desc).
- **Seqlock adversary**: readers/writers on fault-injected timings
  (SIGSTOP mid-write etc.) → no torn read, bounded retries.
- **Runtime property**: "no allocation after init" verified under
  `LD_PRELOAD` malloc-interpose counter + ASan/UBSan/TSan builds of
  `libtplc-host` (TSan on the SHM protocol is meaningful — lock-free
  code is exactly where TSan earns its keep).

## 6. Hardware-in-the-loop story

- CI does L1/L2/L3 without hardware (the entire point).
- A small **hardware lab rack** (one x86 IPC + one arm target + one
  EtherCAT IO slice + one drive) runs the same vector suites nightly —
  same vectors, same comparators, different transport.
- Fault-injection cases that need real power (retain loss windows,
  restart storms) live here; results journaled into the same
  `io-trace` format so lab-vs-sim diffs are the same tool.

## 7. Coverage & gates (release criteria, doc 17 consumes)

- Line/branch coverage targets per component: runtime ≥90% branch on
  ABI/protocol code; codegen ≥85%; frontend parser ≥90% (grammars are
  cheap to cover); oracle adapters best-effort (dev-only).
- Mutation score sampling on migrate+image code (high-value targets).
- Every `TPLC_E_*` code has ≥1 test producing it (the "no untested
  error path" rule — error codes are API).
- Perf gates as in DESIGN-GOALS §6 (measured, recorded, trended —
  regressions block release, not just warn).

## 8. Iteration-2 hooks

- SFC/LD slots reserved in the vector schema (`language` field) and
  corpus layout.
- 61499-style distributed tests are out; multi-program L2/L3 sims
  already in scope.
- Deterministic-replay tooling (record input sequences → replay in
  debugger at L1) is the natural extension once traces stabilize —
  noted, not committed.

## 9. Error codes (owned by this chapter)

`TPLC_E_SIM_DIVERGENCE, _SIM_CLOCK, _SIM_BACKEND, _ORACLE_*`

## 10. Acceptance criteria

1. L1/L2/L3 sims run the reference `.tplc` set; sim-parity suite proves
   bit-identical traces for the deterministic subset.
2. `tools/oracle-matiec` produces vectors end-to-end; trace-diff finds
   zero unexplained divergences on the IEC-subset corpus; GPL hygiene
   scan clean.
3. All §4 suites exist in CI with their green criteria; fuzzers run
   nightly with crashers triaged.
4. Kill -9 + hang + seq-odd fault-injection cases prove doc-03/04
   promises in L2.
5. Perf/jitter numbers recorded against DESIGN-GOALS §6 targets.
6. Coverage gates enforced; error-code coverage report has zero gaps.
