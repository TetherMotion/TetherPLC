# As-Built: Shadow Run — Parallel Candidate with Instant Promotion

> Status: implemented (runtime + ctl + `tplc-io` + WebUI + e2e +
> L1/L2 sim parity). The plan chapter stays as design reference.

A **shadow program** runs alongside the **active program** on the
same latched `%I` inputs. The active program alone steers `%Q`; each
shadow's output writes land in a private buffer that never touches
the process image. The user compares live candidates side by side,
then promotes one atomically — its accumulated state (timers,
counters, edge memories) carries over with no cold restart and every
peer candidate is dropped.

## Model

```
        latched %I (active latch, per scan boundary)
          │                 │            │
    active program      shadow[0]     shadow[1..3]
          │                 │            │
   commit → real %Q    private imageOut buffers (never committed)
          │                 │            │
          └── per-candidate %Q diff rings (bounded byte runs) ──┘
```

- Up to **four** shadow slots (`MAX_SHADOWS`). `SHADOW_LOAD` requires
  a running active program; a fifth load is refused, and re-loading
  an already-loaded program uuid is refused. Candidates are addressed
  by uuid or program name through an optional selector tail — with
  several loaded, selector-less ops fail `MGMT_STATE`; with exactly
  one, the selector may be omitted.
- Each candidate owns a `Scheduler`, counters, diff ring, and fault
  flag. Shadow scheduling runs after the active `%Q` commit, only
  when slack to the next active release exceeds `shadow_budget_ns`
  (default 200 µs, `--shadow-budget-ns`). A due tick that does not
  fit increments that candidate's `missed` — the active scan is
  structurally never delayed, and one candidate's work cannot starve
  its peers beyond the shared phase gate. A faster-period candidate
  ticks between active releases against the same latched `%I`
  snapshot; each candidate ticks at most once per scan instant.
- A candidate's first cycle waits for an active scan so its latch
  copies this scan's `%I`, never stale pre-load inputs.
- Identical inputs by construction: the shadow latch hook memcpy's
  the active program's `imageIn()` buffer, not the process image.
- Output isolation by construction: the shadow commit hook is a
  no-op; each `Program` owns its `imageOut()` buffer, so `%Q` can
  never reach the real image — including safety channels.
- A shadow task fault drops only the faulting candidate (deferred
  until `Scheduler::tick` returns); the active program and peer
  candidates are unaffected.

## Wire surface (ctl, `0x0800` range)

| op | role | payload → reply |
|---|---|---|
| `SHADOW_LOAD` | commissioner | `{u16 len, path}` (same uuid-or-path rule as swap) → program uuid |
| `SHADOW_STATUS` | observer | `[sel]` → `TplcCtlShadowStatus` (uuid, name, state, cycles, missed, diff_count, last_diff_cycle, task runs); no candidate → `state=0` byte |
| `SHADOW_LIST` | observer | `∅` → `{u32 n} + TplcCtlShadowStatus[n]` — every candidate in load (SHADOW_LIST) order |
| `SHADOW_COMPARE` | observer | `{u32 max}[sel]` → `{u32 n} + TplcCtlShadowDiff[]` (cycle, img offset, len ≤32, active bytes, shadow bytes) |
| `SHADOW_IMAGE` | observer | `{u64 off, u32 len}[sel]` → raw bytes of the candidate's image-%Q region (`image_out_bytes` bound; `DBG_ACCESS` outside, `MGMT_STATE` with no shadow) — live preview of what it would drive, without promoting |
| `SHADOW_PROMOTE` | commissioner | `[sel]` → adopted program uuid + new mgmt epoch; drops every peer candidate |
| `SHADOW_DROP` | commissioner | `[sel]` → void |
| `SHADOW_RESET` | commissioner | `[sel]` → migrate result (`out_status`) |

The optional selector is a `{u16 slen, sel}` tail where `sel` is the
16-byte program uuid or the program name. Name selectors resolve to
the first match, so same-named candidates (the normal case — a
program and its candidate versions share a name) must be addressed
by uuid.

`TplcCtlProgStatus` carries a proto-v3 shadow tail
(`shadow_state/cycles/missed/diff_count/last_diff_cycle`). With
several candidates the counters **aggregate**: `shadow_state` is 1
when any candidate runs, `cycles`/`missed`/`diff_count` sum across
candidates, and `last_diff_cycle` is the newest of them. Per-candidate
detail lives on `SHADOW_LIST` / selected `SHADOW_STATUS`.

`SHADOW_LOAD` reuses the package trust path (`Package::open` →
signature/trust policy identical to deploy) and the online-change
migration machinery: each shadow's initial blob is produced by the
same active→candidate migrate that swap uses, so the candidate
starts where the active is rather than at defaults. `SHADOW_RESET`
re-runs that migration on demand for the selected candidate.

Promotion moves the selected candidate's `Module`/`Program` into the
active slot inside the same commit point as swap: the old active is
quarantined (rollback window applies unchanged), mgmt epoch bumps,
the journal records `promote`, and every peer candidate is dropped —
their blobs were seeded from the pre-promote program and their
migrate descriptors no longer match the new active layout. Because
the promoted blob is already live, no re-initialization occurs —
counters continue from their accumulated values.

Lifecycle drops: `stop`, `kill`, `swap`, `rollback`, `unload`,
`prepare` (via `unload_program`), and shadow task fault all drop
every candidate and journal the reason.

## Debug surface

- `DBG_READ` takes an optional trailing selector byte (`0` active,
  `N` = `shadows[N-1]`); `DBG_FIELDS` likewise enumerates the
  selected candidate's symbol map.
- `DBG_RESOLVE` accepts `SHADOW.<sym>` (first candidate) and
  `SHADOW[<i>].<sym>` (i-th candidate, SHADOW_LIST order) and
  resolves against that candidate's symbol map — the reply uuid
  identifies which program.
- Watch items accept `SHADOW.<sym>` / `SHADOW[<i>].<sym>` paths and
  sample the selected candidate's blob in the same boundary image
  as active items; a dropped candidate reads as stale.
- Mutation ops (`DBG_WRITE`, `FORCE`) are structurally active-only —
  their fixed-size payloads have no selector, so shadow state
  cannot be modified through the debug plane at all.

## `tplc-io` surface

Functions `shadow.load/list/status/compare/promote/drop/reset`
(group `plc`); `shadow.list` returns every candidate, and
`status`/`compare`/`promote`/`drop`/`reset` take an optional `sel`
string (uuid hex or name — same selector tail as the wire). Live
signals `shadow.state`, `shadow.cycles`, `shadow.missed`,
`shadow.diff_count` aggregate across candidates; `dbg.read`/
`dbg.fields` take the indexed `sel`. Promote bumps mgmt epoch,
which the provider's epoch watcher already bridges into catalog
epoch + `epoch.N` marker — tether_io clients see the change like
any program-set change.

## WebUI (TetherWebUI)

The deploy pane carries a **shadow run** card (§7.11):

- **Deploy as shadow** — `.tplc` file → `DEPLOY` then
  `SHADOW_LOAD` with the committed package uuid; disabled while
  the active program isn't running.
- **Status line** — candidate name/uuid, state, cycles, missed,
  diff count, last diff cycle.
- **Diff table** — `SHADOW_COMPARE` rows: cycle, `img+off`,
  active vs shadow byte runs.
- **Promote** — confirm dialog summarizing cycles/missed/diffs
  before `SHADOW_PROMOTE`; rollback stays armed.
- **Reset / drop** — `SHADOW_RESET` re-runs active→shadow
  migration, `SHADOW_DROP` discards the candidate.
- **Side-by-side watch/read** — `SHADOW.<sym>` in the debug card
  resolves against the first candidate's symmap (shadow uuid,
  marked `shadow sym`), reads through `dbgRead sel=1`, and
  `+ watch` stages a shadow channel verbatim; write/force controls
  are hidden for shadow symbols since the shadow blob is
  read-only. The wire and `tplc-io` already carry the indexed
  `SHADOW[<i>].<sym>` form and per-candidate selectors — a
  multi-candidate picker is IDE work, not runtime work.
- `progStatus` parses the proto-v3 shadow tail (and still decodes
  v1/v2 layouts for older runtimes).

## Verified

`tests/shadow/e2e.sh` (ctest `shadow_e2e`, 41 checks): stopped-load
refusal (-501), empty status, advancing cycles, `%QD4` divergence
reported at `img+0x04`, active `%Q` ramp unchanged during shadow
(no steering), `SHADOW.` resolve/read/watch, reset, promote
(uuid+epoch), post-promote doubled ramp (live-state adoption),
predecessor quarantined, rollback restoring the original ramp,
swap/drop lifecycle + journal, multi-shadow — four loads at the
slot cap, fifth-load and duplicate-uuid refusal, selector-required
on ambiguous ops, per-candidate status/compare/image through uuid
selectors, `SHADOW[2].` resolve + `dbg-read sel=2` + `fields sel=3`,
drop-by-uuid shrinking the list, promote-by-uuid dropping all
peers — observer 401 on mutating ops with status reads allowed,
`missed` counting under an impossible budget.

`tests/scan/tplc_io_attach.sh` extends the probe with the shadow
function/signal surface through the real tether_io protocol.

`sim_shadow` (ctest): `tools/oracle/corpus/vectors_shadow.jsonl`
drives the same scenario at both legs — the L1 interpreter runs the
`"shadow"`/`"shadows"` vector keys (per-cycle `shadow_load[@N]`,
`shadow_promote[@N]`, `shadow_drop@N`, `expect_shadow_q` string or
per-index object; seed is a name-wise field copy plus declaration
initializers at load, the L1 model of the runtime's tplc_init +
same-layout blob copy), and `tether-plc-sim --scan` replays it on
the real runtime via repeated `--shadow PKG` and the
`shadow_load@N`/`shadow_promote@N`/`shadow_drop@N`/
`expect_shadow@N` step directives (candidate indices resolve
through `SHADOW_LIST` to uuid selectors).
`expect_shadow` reconstructs the candidate's `%Q` from the newest
`SHADOW_COMPARE` entries overlaid on the image.

For deterministic step-level parity the runtime also accepts
`--stopped`: the program loads and registers tasks but does not
start until a ctl `start`. `--scan` uses it to suspend the task
before the first `first_run` can fire, removing the startup
free-run race that made cycle counts nondeterministic.

`shadow_bitidentical` (ctest): the same 10-step input tape replayed
solo and with a divergent candidate shadowed produces byte-identical
`--dump-q` traces — the active program is not perturbed at all.

## Open items

- Per-channel promote (adopting only part of a candidate's `%Q`
  surface) remains a non-goal, as does a multi-candidate picker in
  the WebUI deploy pane — the wire, `tplc-io`, and debug paths
  already carry indexed candidates end to end.
- Shadow→image preview landed as `SHADOW_IMAGE`: observers read the
  selected candidate's live image-%Q region (bounded by
  `image_out_bytes`) without promoting.
