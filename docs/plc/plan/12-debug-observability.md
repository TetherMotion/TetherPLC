# 12 — Debug & Observability: Watch, Force, Trace, Suspend

> **Status:** work-item document; deleted when `docs/plc/debugging.md`
> exists and the debug plane passes conformance (incl. determinism
> guarantees under debug load).
>
> **Satisfies:** G8 (the whole chapter), G3 (debug must not break
> determinism), G11 (gated+audited mutating ops), G7 (same debug path
> in sim).
> **Depends on:** 02 (symmap/bindings), 03 (image), 04 (runtime), 09
> (catalog). **Informs:** 11 (UX), 14, 16.

---

## 1. The debug plane

All debug/observability operations flow through one mechanism with one
invariant: **the scan thread applies debug ops at task boundaries**,
between invocations. Service threads enqueue ops; the scan thread
drains them in the inter-task gap. Consequences that define the plane:

- Reads/writes of blob fields always see a **cycle-consistent** value
  (never mid-assignment) — "what the program saw" semantics, which is
  also what PLC engineers expect a watch to show.
- Worst-case op latency ≈ one task cycle; zero RT-jitter impact by
  construction (ops never preempt a task — they run *between* tasks,
  accounted in scheduler slack).
- Debug load is bounded: a queue-watermark policy sheds ops with a
  `DEBUG_OVERLOAD` notice rather than ever delaying a task release.
- Sim host and target implement identical semantics (G7) — including
  deterministic replay: a trace produced in sim is comparable
  bit-for-bit with target traces.

Debug surface = control-channel op groups (plc-runtime.md §9) proxied +
policy-gated by the bridge (io-mapping.md §4) into `tether_io` parameters/
signals/functions for the UI.

## 2. Symbol resolution (`dbg.resolve`)

```
resolve(name | fbd_node_id) →
  symmap entry → {region, offset, type_ref, access, parent chain}
  epoch-checked: entry.layout_epoch == running program's
```

- Names are qualified IEC paths (`PROG.Ton.Q`, `GVLIO.temps[3].raw`).
- FBD overlay resolves by node/wire id via provenance (fbd-model.md).
- After online change: epoch bump → all subscriptions re-resolve by
  name; failures become `stale` (visible state, not silent death);
  re-resolution is automatic on next read. Rename-with-alias entries
  (online-change.md) survive renames transparently.
- Structured values address leaf fields (`struct.field`,
  `array[i].member`); the resolver walks type_catalog — UI never
  parses types itself.

## 3. Watch (`dbg.watch`)

| op | semantics |
|---|---|
| `watch.add{paths[]}` | register; returns handle + resolved descriptors |
| `watch.get{handle}` | one-shot read (boundary-applied) |
| `watch.stream{handle, rate_hz}` | periodic reads → ring → control stream / `tether_io` params |
| `watch.remove` | drop |

- Value encoding: typed binary per type_catalog + JSON mirror for UI.
- Limits: `watch.max_entries` (default 256/program), stream rate
  `≤ 50 Hz` per client, total stream bandwidth capped — values are
  sampled at boundary N and shipped in batches; the *sampling* is
  cycle-aligned, the *transport* is service plane.
- Read of `%I/%Q` shadows = the latched image copies (what the program
  uses), distinct from raw channels (`plc.image.*` signals — both are
  offered, distinction documented).

## 4. Write vs Force — two different operations

PLC debugging needs both and users expect the distinction:

| op | semantics | persistence |
|---|---|---|
| `dbg.write{path, value}` | one-shot assignment to a blob field at next boundary; program may overwrite next cycle (normal for outputs, sticky for internals) | none |
| `dbg.force{path, value}` | **persistent override**: registered in the force table; reapplied every cycle at the documented point | until `force.remove`/`force.clear_all` |

Force application points (process-image.md §5.2):

- `%I`-bound / input-shadow vars: forced **after latch, before task** —
  the program sees the forced value every cycle.
- `%Q`-bound / output-shadow vars: forced **after task, before commit**
  — the fieldbus sees the forced value regardless of program.
- `%M`/internal/GVL fields: forced **before task** (a read-modify
  each cycle — they act as constants to the program).
- Non-forceable entries (symmap `access!=rw`, `forceable=false`
  channel policy, safety-adjacent `%I` reads marked by map) → reject
  `FORCE_DENIED` at resolve time.

Force semantics are **mask + value**: forcing `BOOL` at byte offset
touches only the bit. Force table: `{entry_idx, mask, value}` ≤
`force.max_entries` (64/program default).

## 5. Suspend & step (the PLC-honest breakpoint model)

True mid-cycle breakpoints would freeze outputs — real PLCs solve
this as *task-level* control, and so do we:

| op | semantics |
|---|---|
| `task.suspend{name}` | deschedule task at boundary; %Q commits stop for its ranges (→ master safe-state rules apply — visible, honest) |
| `task.resume` | re-arm schedule |
| `task.step{name, n=1}` | while suspended: run exactly n cycles then re-suspend — **deterministic single-cycle stepping**, cheap because the scheduler is ours |
| `dbg.break_when{task, expr}` | conditional suspend: expression over blob fields (mini expression evaluator in runtime — typed, no codegen) evaluated at each task end → suspend on true. This is the "breakpoint" users get; it cannot stop mid-statement (documented clearly). |
| `dbg.capture_on{name}` | trigger helper: like break_when but captures a trace snapshot instead of suspending |

- Mid-statement source-level breakpoints are a **declared non-goal**
  (requires halting machinery or invasive patching — the pause model
  covers the practical need; DWARF/gdb on a *sim* instance remains the
  escape hatch for deep debugging, doc 14).
- Suspended-task UX makes the safe-state consequence explicit (web-ide.md
  confirmation text shows which channels go safe).

## 6. Trace (cycle-accurate recording)

For "what happened" debugging beyond live rates:

- `trace.config{channels: [paths], task, pre_trig, post_trig, trig_expr}`
  → per-cycle samples of ≤`trace.max_channels` (16) fields into a
  lock-free ring **inside the runtime**, at task boundary (zero-jitter:
  sampled with the same boundary discipline; cost ~stores).
- Triggers: expr-true edge (same evaluator as break_when) or
  task-fault auto-trigger (`capture on fault` — the flight-recorder:
  last N cycles of configured channels around any fault — ON BY
  DEFAULT for every program, because the first question after a fault
  is always "what were the values").
- Export: ring drains to service plane → `tether_io` datalog/scope
  subsystem (existing infra) → UI scope view; binary blob export for
  CI diffing.
- Determinism: trace content is part of sim↔target bit-parity checks.

## 7. Retain & state inspection

`dbg.state{program}`: blob layout browser (fields, regions, sizes,
retain flags), `retain.status/dump` (hex+typed), `state.diff{old,new}`
post-migration report (what the migrate rules actually did — closes
the online-change trust loop with the §11 preview).

## 8. Security & audit (doc 15 cross-ref)

- `dbg.write/force/suspend/step/break` = mutating ops → authority
  `commissioner` role, journal-logged `{who, op, path, old→new}`.
- Force state is **globally visible** (UI banner + `plc.runtime.forces`
  count + per-channel flags) — a hidden force is a safety incident;
  there is no quiet mode.
- `force.clear_all` is one op; "unforce all" also offered on program
  stop (config `force_on_stop: keep|clear`, default clear).

## 9. Error codes (owned by this chapter)

`TPLC_E_DBG_RESOLVE, _DBG_STALE, _DBG_OVERLOAD, _DBG_RATE,
_FORCE_DENIED, _DBG_ACCESS, _DBG_STATE, _TRACE_CONFIG`

## 10. Acceptance criteria

1. Watch/force/write/suspend/step/break_when ops end-to-end on a
   running program; each gated + journaled.
2. Determinism proof: scan jitter with max-rated debug load stays
   inside doc-04 budgets (ops shed before tasks slip).
3. Boundary consistency: a watch never observes a torn multi-byte
   value mid-update (stress test with 64-bit field + 4 kHz task).
4. Post-swap: subscriptions re-resolve; renamed (alias) vars keep
   streaming; vanished vars go `stale` visibly.
5. Flight-recorder: induced fault → trace snapshot contains the
   configured channels' pre-fault cycles.
6. Force semantics: input/output/internal points verified cycle-
   accurate; masked bit force doesn't disturb byte neighbors.
7. Sim parity: identical watch/trace outputs in sim for identical
   input sequences.

---

## Appendix M — Trigger-expression language (`dbg.expr.v1`)

`break_when` and trace triggers evaluate a small typed expression over
blob fields **in the runtime** at task boundaries. It is deliberately
not ST: no calls, no loops, no allocation — a stack machine over
resolved symmap entries, compile-checked at op time.

### M.1 Grammar

```
expr    = or
or      = and { '||' and }
and     = cmp { '&&' cmp }
cmp     = add [ ('=='|'!='|'<'|'<='|'>'|'>=') add ]
add     = mul { ('+'|'-') mul }
mul     = unary { ('*'|'/'|'%') unary }
unary   = [ '!' | '-' ] unary | primary
primary = path | literal | '(' expr ')' | 'changed' '(' path ')'
        | 'rising' '(' path ')' | 'falling' '(' path ')'
path    = qualified symmap path (e.g. "PROG.Ton.Q", "GVL.x[3].v")
literal = int | float | 'true' | 'false' | time-literal
```

### M.2 Semantics

- Operands resolve at `break_when`/`trace.config` time via `dbg.resolve`
  (stale after swap → trigger deactivates with notice).
- `changed(path)`: value differs from previous boundary sample (per-task
  evaluation memory — the evaluator keeps one previous value per path).
- `rising`/`falling`: BOOL edge in consecutive samples.
- Type rules: numeric widening only; BOOL not numeric; comparisons
  need matching families; compile-checked at config → `TRACE_CONFIG`/
  `DBG_RESOLVE` errors carry the offending term.
- Cost bound: bytecode-compiled expr, max 64 ops, evaluated in ≤2 µs
  (budgeted like a service call; exceeding once = disable + alarm).
- `path` may address `%I/%Q` shadow vars, `%M`, any blob field — the
  same name space as watch paths.
