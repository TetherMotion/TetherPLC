# 08 — Online Change: Migrate-and-Swap Protocol

> **Status:** work-item document; deleted when `docs/plc/online-change.md`
> exists and the end-to-end swap path passes its conformance suite.
>
> **Satisfies:** G4 (the v1 gating feature), G3 (bounded pause), G6
> (failure isolation during swap), G8 (degraded-change visibility).
> **Depends on:** 01 (`migrate_desc`), 02 (decision matrix, ABI), 04
> (staging machinery), 07 (migrate codegen). **Informs:** 10, 12, 16.

---

## 1. The contract

Online change = replacing a running program's code and layout with a new
version **without stopping its tasks and while preserving state**,
bounded by:

1. **Continuity**: inputs keep latching and outputs keep committing for
   the entire operation except one bounded pause (the blob snapshot,
   target ≤500 µs @256 KiB — DESIGN-GOALS §6).
2. **State fidelity**: every value the migration rules preserve is
   bit-exact across the swap; every loss is an enumerated, reported
   `OK_DEGRADED` item — never silent.
3. **Atomicity**: from any task's perspective the swap is instantaneous
   at a cycle boundary — it never observes half-old/half-new code or
   layout.
4. **Refusability**: unsafe changes are refused *before* anything
   mutates (compile-time deny rules + runtime DENY path), leaving the
   old program untouched.
5. **Reversibility**: a bounded rollback window restores the prior
   program on demand or on auto-fault rules.

What it is NOT: a code patch of the running text, a debugger edit-and-
continue, or a guarantee for arbitrary semantic change. The unit of
change is the whole program artifact; the granularity of preservation
is the named field.

## 2. End-to-end pipeline

```
ENGINEERING SIDE                          TARGET (tether-plc)
───────────────                           ───────────────────
edit project (ST/FBD)
   │
tplcc --change-against <query>:           ┌─ deploy API: returns running
   fetch running layout ────────────────► │  program_uuid + layout_hash +
   produces pkg with migrate_desc         │  layout_desc + package_uuid
   (deny-analysis happens AT COMPILE      └─ (doc 10 channel)
   time — see §4)
   │
deploy pkg ─────────────────────────────► service: verify, dlopen,
                                          descriptor+abi checks,
                                          alloc new blob, parse symmap
                                          → staged: "migrate" | "module-swap"
                                          (layout_hash equal → fast path)
   │
                                          scan boundary: snapshot old blob
                                            (PAUSE = memcpy only)
                                          old program RESUMES (still old code)
   │
                                          service: tplc_migrate(snapshot→new)
                                            OK/DEGRADED → arm
                                            DENY/ERR → abort+report
   │
                                          next boundary: pointer-swap ctx,
                                          descriptor, task table, symmap;
                                          resume flag on first task run
                                          quarantine N cycles →
                                          service dlclose old + free snapshot
                                          (or ROLLBACK, §7)
```

Two invariants worth restating:

- The running program executes **old code until the commit boundary** —
  migration of its frozen snapshot happens off-cycle in parallel; there
  is no "half-migrated running state".
- Compile-time deny analysis means the *common* failure is caught in
  the IDE, not on the machine — runtime DENY exists only for
  "compiled-against-a-different-version" races (§8.2).

## 3. Migration semantics (what preserves)

Rule table semantics (doc 01 §8, generated per §4.6 of doc 07):

| rule | applies when | semantics |
|---|---|---|
| `copy` | same qualified name + identical type | memcpy field range |
| `transform` | same name + type pair in transform table | generated widening/coercion (§3.2) |
| `default-init` | name only in new | init value from new defaults |
| `dropped` | name only in old | value lost — reported; retain-field drop escalates to alarm |
| `unsafe-deny` | §5 catalog | whole change refused |

### 3.1 Name identity

Fields match by **qualified name** (`GVL.Counter`, `PROG.TonDelay`,
`fb_inst.member.path` for FB member sub-fields). Renames are
delete+add (documented UX hint: rename-refactor tooling in the IDE
should emit rename hints into the project so the compiler *can* map
them — `migrate_desc` supports an optional `alias` field per rule:
`old_name → new_name` explicit rename mapping, IDE-generated).

FB instances match by instance name; their member fields migrate
individually (a FB type that gained a member = added field inside a
preserved instance → sub-field rules, not instance-deny — see §5 for
what DOES deny).

### 3.2 Transform table (allowed type changes — v1)

| from → to | rule |
|---|---|
| `SINT→INT→DINT→LINT`, `USINT→…→ULINT` widening | preserve value |
| `REAL→LREAL` | preserve value |
| `TIME` variants (`TIME↔LTIME` iter2) | preserve ns |
| `STRING(n)→STRING(m)` | copy min(n,m) + truncate flag → DEGRADED note |
| `ARRAY[..] OF T` bounds grow | copy prefix, default-init tail → DEGRADED note |
| same-size enum reorder | preserve ordinal + warn |
| narrowing, signed↔unsigned, float↔int, struct shape change | **deny** (§5) |

Transforms are deliberately conservative — "clever" migration is how
state silently corrupts. Anything not in the table is deny-or-default,
explicitly reported.

### 3.3 Retain interaction

- Retain-flagged fields migrate by the same rules, *inside the retain
  region* — the region's own sub-layout is versioned by
  `retain_layout_hash` (doc 04 §5).
- Migration runs **against the snapshot which includes retain values**
  — a migrated retain value is preserved through the swap (it never
  round-trips through the file on this path).
- `retain_layout_hash` compatibility governs the *persistence* path
  (restart), not the swap path — the file needs no update until next
  flush (then writes new layout).
- Dropping a retain field → `OK_DEGRADED` + journal alarm (data loss on
  persistent memory is always loud).

## 4. Compile-time deny analysis (the first gate)

`tplcc --change-against` fetches the running program's `layout_desc`
via the deploy API (or from a cached package/manifest) and performs the
§4.6 diff **at compile time**:

- Deny-cases produce **compile errors** with the offending fields —
  the engineer learns "this change can't hot-apply" before deploying.
- Non-deny changes produce `migrate_desc` + `tplc_migrate` in the
  package.
- `--offline` mode: compile against a specified older `.tplc`/layout
  file instead of a live query.
- If the live program's layout differs from what was compiled against
  (someone else deployed meanwhile) → runtime falls back to §8.2.

## 5. Deny catalog (forces cold restart)

| # | change | why |
|---|---|---|
| DENY1 | FB *type* changed under existing instance (subtype shape differs beyond member-add rules) | instance state can't be reinterpreted |
| DENY2 | field type narrowed / signedness flip / float↔int / struct member removed-or-retyped | lossy or reinterpret |
| DENY3 | task period changed while task running AND program marks `deny_on_task_change`? — actually: task timing changes are ALLOWED (task table swaps wholesale); what denies: task REMOVED while its task_local state existed + referenced? — simplify v1: **task add/remove/retime allowed**, task rename = delete+add (task_local lost, DEGRADED). Removed from deny list; kept: task count change beyond runtime max → deploy reject (not migration deny) |
| DENY4 | AT-binding moved to different image offset type (%I→%Q) | ownership inversion unsafe |
| DENY5 | abi_version / abi_ext requirement increase beyond host | can't run at all (load reject — technically pre-migrate) |
| DENY6 | retain region size shrink below used | data loss on persistent memory — engineer must confirm `--allow-retain-loss` flag (turns deny→DEGRADED deliberately) |
| DENY7 | program kind change (PROGRAM↔FB), instance count semantic breaks (arrayed instance dims change) | identity change |
| DENY8 | anything codegen can't prove safe (fallback rule) | conservative default |

Each deny carries `reason_code + affected fields` into diagnostics and
`out_first_error`.

## 6. Runtime behavior around the swap

- **During snapshot pause**: tasks of *that program* hold at boundary;
  other programs' tasks unaffected; image latching by OTHER programs
  continues; the faulting program's own %Q commits simply skip the pause
  window (master sees ≤1 late commit — inside staleness margins).
- **First cycle after commit** (`resume` flag): `now_ns/dt_ns` correct
  (dt spans the swap); timers see continuous time — a TON mid-countdown
  keeps its `start_ns` and completes on schedule across the swap. Edge
  detectors keep last-input → **no spurious edges**. These are pinned by
  dedicated conformance tests (§11) because they're the classic
  online-change bugs.
- **Probes/forces**: symmap swaps atomically with the program; the
  debug layer re-resolves watch/force entries by name (doc 12) — entries
  whose field vanished become `stale` (reported, not crash); force
  entries on preserved names continue.
- **Task table** swaps wholesale with the program (new periods/priority
  take effect at commit).

## 7. Rollback

- Window: `quarantine_cycles` (default 10, config `1..10_000`) after
  commit — old module + old blob snapshot retained.
- Triggers: explicit `prog.rollback` op; auto-rule `auto_rollback_on_fault`
  (new program faults within window → restore old, program state =
  `ERROR(rolled_back)`, alarm — never silently healthy again).
- Outside window: rollback requires a new deploy of the old package
  (treated as forward migration — which will deny if it must).
- **Retain divergence caveat**: a rollback restores the *snapshot* —
  retain mutations by the new program within the window are discarded.
  Documented; the window exists for "swap was wrong" recovery, not as a
  general undo.

## 8. Failure modes

### 8.1 Mid-pipeline failures

| stage | failure | outcome |
|---|---|---|
| deploy | verify/dlopen/descriptor | program stays as-is; ERROR on the *change attempt*, old program untouched |
| snapshot | (memcpy can't fail; allocation earlier) | n/a — allocs all pre-boundary |
| migrate | `DENY`/`ERR` | abort; old program continued running throughout — **zero downtime consequence** |
| commit | — | commit is a pointer swap; infallible once armed |
| post-commit fault | new code faults | per §7 auto-rollback in window; else program ERROR + safe-state for its %Q |

### 8.2 Layout race (compiled against X, target runs Y)

At arm time, runtime compares `migrate_desc.predecessor_layout_hash` to
the running layout hash:

- equal → proceed;
- mismatch → **abort** `TPLC_RT_SWAP_RACE` + report both hashes (the fix
  is recompile against the live layout — the toolchain can fetch it).
- Never "try anyway" — a wrong-layout migrate is a silent-corruption
  machine; this is a hard rule.

### 8.3 Version-skip chains

`migrate_desc` describes one predecessor hop by design (doc 01 §3) —
the toolchain always compiles against the *live* layout, so chains
never accumulate. Multi-hop history is not modeled (simplicity > a
feature real PLCs themselves handle by re-download).

## 9. Concurrent programs

Online change targets **one program instance** at a time (control op
takes `program_uuid`); other programs run undisturbed. Cross-program
%I/%Q effects during the boundary pause are bounded by each program's
own task timing — no global stop exists (there is deliberately no
"stop the world" in the runtime; even multi-program deploys sequence
per-program).

## 10. User-visible semantics (documentation contract)

The engineer-facing guarantee list (goes in docs verbatim):

1. Logic changes (edited ST code, re-wired FBD): applied live; **all
   variable values preserved**.
2. Added variables/FBs: appear with their initial values.
3. Removed variables: values gone; reported at compile + swap time.
4. Renames: treated as remove+add unless the IDE's rename-tracking was
   used (then preserved via alias).
5. Type changes: widening preserved; narrowing/struct-change ⇒ compile
   error suggesting download-while-stopped instead.
6. Task retimes: applied at the boundary; running timers/counters
   unaffected.
7. Never loses retain values silently; `--allow-retain-loss` exists for
   deliberate schema shrinks.
8. If anything looks unsafe, the *compiler* says so before deploy — the
   machine is never the bearer of bad news.

## 11. Acceptance criteria

1. End-to-end: ST edit → compile-with-predecessor → deploy → swap on a
   running program; IO trace continuity verified (gap ≤ pause budget).
2. Fast path: identical layout → module-only swap, zero blob work.
3. Rule coverage: every §3.2 transform + each deny code exercised by
   fixture pairs; DENY leaves running program 100% intact (blob
   snapshot-compared).
4. Continuity proofs (pinned tests): TON mid-cycle, CTUD mid-count,
   R_TRIG across swap — no spurious output, no lost edge.
5. Race: arm-time hash mismatch aborts cleanly with both hashes
   reported.
6. Rollback: manual + auto-fault paths restore old program; retain
   divergence documented behavior verified.
7. Pause budget measured and enforced: >500 µs at 256 KiB fails perf
   test (warn at 250).
8. Degraded migrations report every loss by name; journal records them.

---

## Appendix O — Transform function table (`migrate_desc.transform_id`)

Generated `tplc_migrate` emits one transform function per distinct
type-pair used; `transform_id` indexes this table in the artifact.

| id | transform | semantics | severity |
|---|---|---|---|
| T01 | int widen (S→I→D→L, U→UI→UD→UL) | sign/zero-extend | info |
| T02 | `REAL→LREAL` | fp widen | info |
| T03 | `TIME` unit-preserving retype | copy ns | info |
| T04 | `STRING(n)→STRING(m)` | copy min(len,m); flag if src_len>m | degraded on truncate |
| T05 | `ARRAY bounds grow` (same elem) | copy common prefix; tail = elem default | degraded (partial) |
| T06 | enum reorder (same members, new ordinals) | map by member name; unknown → default + flag | degraded on unknown |
| T07 | struct member-add (tail members only) | copy existing members; new tail defaults | degraded |
| T08 | BOOL↔BYTE (same storage) | copy | info |
| T09 | alias-rename (identical type, new name) | copy via alias map | info |
| T10 | subrange bounds loosen (superset) | copy; value re-validated | degraded on out-of-new-range |

Anything outside T01–T10 → deny rule (§5). Table is versioned
(`transform_table_version` in migrate_desc header); new transforms
append — never renumber.

## Appendix P — Alias/rename mapping format

IDE rename-tracking emits into the project file:

```toml
[[renames]]
old = "GVL.Counter"
new = "GVL.PartCount"
since = "2026-11-05"           # informational
```

The compiler merges project renames into the rule table as `kind=5
(alias)` rules referencing the old-field index + new-field index +
`aux` = alias string offset. Aliases are consumed once (one rename
hop); a rename chain A→B→C collapses at compile to A→C by the
project's rename log (the log is append-only; pruning is manual).
