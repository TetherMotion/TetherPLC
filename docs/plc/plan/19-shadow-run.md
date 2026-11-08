# 19 — Shadow Run: Parallel Candidate Program with Instant Promotion

> **Status:** work-item document; deleted when `docs/plc/shadow-run.md`
> exists and the shadow/promote path is live with the acceptance
> criteria below green.
>
> **Satisfies:** G4 (online change, extended), G8 (observability —
> the compare view is a debug affordance), G3 (the active program's
> determinism is sacrosanct).
> **Depends on:** retired chapters 03 (image), 04 (runtime), 08
> (online change — migrate machinery, quarantine, rollback), 16
> (management ops/roles/events). **Informs:** 17 (phase P11 or
> iteration 2), TetherWebUI PLC tab.

---

## 1. Requirement

A user must be able to run a **secondary program** alongside the
program that steers the machine:

- The **active program** owns the process image outputs (`%Q`) — it
  is the only program that can steer the machine.
- The **shadow program** consumes the *identical* input data — the
  same latched `%I` sequence at the same task boundaries — and runs
  the same release schedule, but its `%Q` writes go to a shadow
  output region, never the real image.
- The user can **compare** the shadow's would-be outputs and
  intermediate state (blob fields, interfaces) against the active
  program's, live.
- When satisfied, the user **promotes** the shadow: it begins
  steering outputs *instantly* (atomically at a task boundary)
  carrying its already-accumulated live state — timers, counters,
  edge memories — with no re-initialization and no cold restart.
- The pre-promotion active program is quarantined exactly like a
  swap predecessor, so instant rollback is available.

Terminology: *active program* (steers), *shadow program* (candidate,
never steers), *promotion* (shadow becomes active), *drop* (discard
shadow). Alternatives considered: "secondary program" (ambiguous with
multi-instance), "staging" (suggests prepare, which keeps its
meaning), "canary" (implies traffic splitting — wrong model: shadow
sees inputs, never outputs).

## 2. Semantics

1. **Shadow deployment.** `SHADOW_LOAD <pkg>` on a running program:
   the runtime loads the candidate and produces its initial state
   blob through the existing migrate machinery — migrate from the
   *active* blob when the package declares it as predecessor,
   otherwise default-init. The shadow then joins the release
   schedule.
2. **Identical inputs.** At each cycle the shadow reads the same
   latched `%I` snapshot the active program used — by construction,
   not by re-sampling: the boundary latch is taken once and shared.
3. **No output steering, structurally.** Shadow `%Q` writes land in
   a runtime-owned shadow output region sized like `%Q`. Nothing in
   the image write path can route them to real `%Q` — this is a
   structural guarantee, not a policy flag (the bridge never learns
   shadow exists).
4. **Active determinism is sacrosanct.** Shadow tasks run *after*
   the active `%Q` commit inside the same cycle, on a bounded
   budget. If the budget is exceeded, the shadow cycle is skipped
   and counted (`shadow.missed`) — the active scan can never be
   delayed by the shadow. Compare results are only meaningful over
   cycles where both ran.
5. **Compare.** At each co-run boundary the runtime diffs shadow-`%Q`
   vs active-`%Q` (channel granularity via io_bindings → image
   offsets) and keeps: a diff counter/stat block plus a bounded
   ring (default 256 entries) of recent diffs readable via
   `SHADOW_COMPARE`. Blob-field comparison is on demand via the
   existing `DBG_FIELDS`/`DBG_READ` machinery extended with a
   program selector (`active`|`shadow`) — it is not per-cycle.
6. **Promotion.** `SHADOW_PROMOTE` commits at a task boundary:
   shadow becomes active and starts driving `%Q` *with its live
   blob* — no migrate at promote time (migrate happened at
   SHADOW_LOAD). The old active program is quarantined; `rollback`
   restores it. Epoch bumps; the management event carries a
   `promote` reason. Pause budget: same as swap (≤1 task cycle).
7. **Drop/resync.** `SHADOW_DROP` discards the candidate.
   `SHADOW_RESET` re-runs migrate from the *current* active blob —
   the supported "resync shadow state to where the machine is now"
   operation.
8. **Lifecycle interaction.** `stop`/`unload`/`prepare`/`swap` on
   the active program drops the shadow first (explicit, journaled).
   A shadow cannot exist without a running active program.

## 3. Backend surface

### 3.1 ctl ops (management plane)

| op | role | payload → reply |
|---|---|---|
| `SHADOW_LOAD` | commissioner | `{u16 plen,path}` → shadow status |
| `SHADOW_STATUS` | observer | ∅ → `{uuid, name, state, cycles, missed, diff_count, last_diff_cycle}` |
| `SHADOW_COMPARE` | observer | `{u32 max}` → `{u32 n, {u64 img_off,u32 len,active_bytes,shadow_bytes,u64 cycle}*}` ring tail |
| `SHADOW_PROMOTE` | commissioner | ∅ → post-promote status (epoch, lineage) |
| `SHADOW_DROP` | commissioner | ∅ → ∅ |
| `SHADOW_RESET` | commissioner | ∅ → migrate result (rules applied/denied) |

Illegal states return `TPLC_E_MGMT_STATE` (-501) with
current+requested; the state machine gains a *shadow-present*
modifier on RUNNING rather than a new state — `prog.status` gains
`shadow_state` + compare summary fields (proto v3 tail).

### 3.2 Debug-plane extension

`DBG_RESOLVE`/`DBG_READ`/`DBG_FIELDS`/watch items gain a leading
program selector byte (`0`=active default, `1`=shadow). Watch paths
may address `SHADOW.<sym>` — same resolver, shadow symmap. Shadow
vars are read-only through this path: `DBG_WRITE`/`DBG_FORCE` with
selector=1 → `DBG_ACCESS` (-415) — a writeable shadow would invite
users to "fix" a program that might be promoted, bypassing audit.

### 3.3 tether_io surface

`tplc-io` mirrors the ops as `shadow.load/status/compare/promote/
drop/reset` functions plus signals `shadow.state`, `shadow.cycles`,
`shadow.missed`, `shadow.diff_count`; promotion bumps the schema
catalog epoch like swap does (epoch markers continue).

### 3.4 Image format

Unchanged. The shadow output region lives in the runtime, not the
SHM image — masters and bridges see no protocol change. (If a
future bridge wants to *preview* shadow outputs at real terminals,
that is an image-format extension and its own chapter.)

## 4. User interface (TetherWebUI `TetherPLC` tab)

1. **Deploy affordance.** The deploy dialog gains a mode toggle:
   *swap now* | *deploy as shadow*. Shadow-deploy compiles the same
   `.tplc`; no separate artifact type.
2. **Shadow panel** (visible when a shadow is loaded): status line
   (program, cycles, missed count), the live **diff table**
   (channel | active value | shadow value | last-diff cycle),
   auto-refreshing from `shadow.compare`/`dbg.fields`.
3. **Variable compare.** The watch view accepts `SHADOW.` paths; a
   split watch row shows active and shadow values of the same
   symbol side by side. A "diff only" filter lists blob fields
   whose active/shadow values differed in the last N cycles.
4. **Promotion flow.** *Promote* button requires confirmation;
   the dialog shows diff summary (cycles compared, fields still
   diverging) and lineage (predecessor = current active). After
   promote, the panel offers *Rollback* while quarantine lasts.
5. **Empty/hidden states.** No shadow → the tab shows "deploy as
   shadow" affordance only; shadow dropped → panel collapses with a
   journal note.

## 5. Safety & security

- Shadow never writes the image, by construction (§2.3) — including
  safety channels, which need no special casing.
- All mutating ops are commissioner-role and journaled, same as
  swap (uid, op, shadow uuid, result).
- The shadow is *more* privileged information-wise than a user
  (it sees the full `%I` stream) — it is a deploy artifact held to
  the same signature/trust policy as any `.tplc`.
- A promoted shadow is an active program: everything that was true
  about the previous program's state is replaced — the confirm
  dialog must show the unresolved-diff count, and promotion with
  diverging diffs requires an explicit "promote anyway" tick.

## 6. Simulation parity

- L1: shadow-pair mode in the MIR interpreter (two programs, shared
  input tape, compare report) — the cheapest conformance harness.
- L2: `tether-plc-sim` accepts a shadow package + steps file;
  vector format gains `shadow_load`/`expect_shadow`/
  `shadow_promote`/`expect_q` directives.
- Same determinism contract: identical input sequence → identical
  shadow trajectory and identical compare output.

## 7. Acceptance criteria

1. **No steering, proven:** image `%Q` bytes over a shadowed run are
   bit-identical to a solo run of the active program on the same
   inputs (byte-diff test, e.g. 10 s at 4 kHz).
2. **Identical inputs, proven:** a shadow R_TRIG/counter program
   replays a vector's `%I` sequence and its blob fields match the
   L1 interpreter's prediction exactly.
3. **Compare accuracy:** for a known-divergent pair (e.g. gain ×2),
   `SHADOW_COMPARE` reports exactly the channels/cycles that
   differ — count and offsets asserted, no false diffs.
4. **Atomic promote:** promotion commits at a task boundary with
   ≤1-cycle injected pause (same budget as swap); post-promote
   `%Q` is driven by the former shadow *from its live blob* — a
   shadow-side timer/counter continues from its accumulated value,
   no reset observable in `%Q` trace.
5. **Rollback:** `rollback` restores the pre-promotion program
   with its last live blob (same guarantee as swap rollback).
6. **Active determinism under shadow:** active-task cadence and
   `max_late` with a loaded shadow stay within the
   `debug_stress`-class bounds; shadow `missed` counter increments
   under deliberate over-budget load rather than active lateness
   growing.
7. **Epoch/events:** promote bumps mgmt epoch, emits a transition
   event with `promote` reason, and the tether_io catalog epoch
   follows (V6 invalidation).
8. **Gating:** observer → -401 on load/promote/drop/reset; every
   mutating op journaled with identity.
9. **Lifecycle:** stop/swap/unload/prepare drops shadow explicitly
   and journals it; `SHADOW_LOAD` on a stopped program → -501.
10. **Sim parity:** the §6 vectors produce identical compare output
    in L1 and L2.
11. **UI:** shadow-deploy → diff view → confirm → promote is a
    ≤4-click flow; diff table shows channel + field divergence
    live; rollback affordance visible during quarantine.

## 8. Explicit non-goals (v1 of this feature)

- Multiple simultaneous shadows (A/B/C compare) — one shadow slot.
- Shadow-to-image preview outputs (bridging shadow `%Q` to real
  terminals pre-promotion) — image-format extension, own chapter.
- Partial promotion (per-channel adoption).
- Long-running "permanent shadow" telemetry modes (a shadow is a
  deploy candidate, not a monitor).
