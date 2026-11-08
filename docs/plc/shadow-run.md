# As-Built: Shadow Run — Parallel Candidate with Instant Promotion

> Status: implemented (runtime + ctl + `tplc-io` + e2e). WebUI flow
> (plan 19 §4) and simulation parity (§6) are not yet built; the
> plan chapter stays until they land.

A **shadow program** runs alongside the **active program** on the
same latched `%I` inputs. The active program alone steers `%Q`; the
shadow's output writes land in a private buffer that never touches
the process image. The user compares live, then promotes the shadow
atomically — its accumulated state (timers, counters, edge
memories) carries over with no cold restart.

## Model

```
        latched %I (active latch, per scan boundary)
          │                 │
    active program      shadow program
          │                 │
   commit → real %Q    private imageOut (never committed)
          │                 │
          └── %Q diff ring (bounded byte runs, ≤32 B) ──┘
```

- One shadow slot. `SHADOW_LOAD` requires a running active program;
  a second load is refused while the slot is occupied.
- Shadow scheduling runs on its own `Scheduler` after the active
  `%Q` commit, only when slack to the next active release exceeds
  `shadow_budget_ns` (default 200 µs, `--shadow-budget-ns`). A due
  tick that does not fit increments `missed` — the active scan is
  structurally never delayed. A faster-period shadow ticks between
  active releases against the same latched `%I` snapshot.
- Identical inputs by construction: the shadow latch hook memcpy's
  the active program's `imageIn()` buffer, not the process image.
- Output isolation by construction: the shadow commit hook is a
  no-op; each `Program` owns its `imageOut()` buffer, so `%Q` can
  never reach the real image — including safety channels.
- A shadow task fault drops only the shadow (deferred until
  `Scheduler::tick` returns); the active program is unaffected.

## Wire surface (ctl, `0x0800` range)

| op | role | payload → reply |
|---|---|---|
| `SHADOW_LOAD` | commissioner | `{u16 len, path}` (same uuid-or-path rule as swap) → program uuid |
| `SHADOW_STATUS` | observer | → `TplcCtlShadowStatus` (uuid, name, state, cycles, missed, diff_count, last_diff_cycle, task runs) |
| `SHADOW_COMPARE` | observer | `{u32 max}` → `{u32 n} + TplcCtlShadowDiff[]` (cycle, img offset, len ≤32, active bytes, shadow bytes) |
| `SHADOW_PROMOTE` | commissioner | → adopted program uuid + new mgmt epoch |
| `SHADOW_DROP` | commissioner | → void |
| `SHADOW_RESET` | commissioner | → migrate result (`out_status`) |

`TplcCtlProgStatus` carries a proto-v3 shadow tail
(`shadow_state/cycles/missed/diff_count/last_diff_cycle`), so
`prog`/`status` consumers see shadow counters without a second op.

`SHADOW_LOAD` reuses the package trust path (`Package::open` →
signature/trust policy identical to deploy) and the online-change
migration machinery: the shadow's initial blob is produced by the
same active→candidate migrate that swap uses, so the candidate
starts where the active is rather than at defaults. `SHADOW_RESET`
re-runs that migration on demand.

Promotion moves the shadow `Module`/`Program` into the active slot
inside the same commit point as swap: the old active is quarantined
(rollback window applies unchanged), mgmt epoch bumps, the journal
records `promote`. Because the shadow blob is already live, no
re-initialization occurs — counters continue from their
accumulated values.

Lifecycle drops: `stop`, `kill`, `swap`, `rollback`, `unload`,
`prepare` (via `unload_program`), and shadow task fault all drop
the shadow and journal the reason.

## Debug surface

- `DBG_READ` takes an optional trailing selector byte (`0` active,
  `1` shadow); `DBG_FIELDS` likewise enumerates the shadow symbol
  map with selector `1`.
- `DBG_RESOLVE` accepts `SHADOW.<sym>` and resolves against the
  candidate symbol map (returns the shadow program uuid).
- Watch items accept `SHADOW.<sym>` paths and sample the shadow
  blob in the same boundary image as active items.
- Mutation ops (`DBG_WRITE`, `FORCE`) are structurally active-only —
  their fixed-size payloads have no selector, so shadow state
  cannot be modified through the debug plane at all.

## `tplc-io` surface

Functions `shadow.load/status/compare/promote/drop/reset` (group
`plc`), live signals `shadow.state`, `shadow.cycles`,
`shadow.missed`, `shadow.diff_count`, optional `sel` on `dbg.read`
and `dbg.fields`. Promote bumps mgmt epoch, which the provider's
epoch watcher already bridges into catalog epoch + `epoch.N`
marker — tether_io clients see the change like any program-set
change.

## Verified

`tests/shadow/e2e.sh` (ctest `shadow_e2e`, 23 checks): stopped-load
refusal (-501), empty status, advancing cycles, `%QD4` divergence
reported at `img+0x04`, active `%Q` ramp unchanged during shadow
(no steering), `SHADOW.` resolve/read/watch, reset, promote
(uuid+epoch), post-promote doubled ramp (live-state adoption),
predecessor quarantined, rollback restoring the original ramp,
swap/drop lifecycle + journal, observer 401 on mutating ops with
status reads allowed, `missed` counting under an impossible budget.

`tests/scan/tplc_io_attach.sh` extends the probe with the shadow
function/signal surface through the real tether_io protocol.

## Open items

- **Bit-identical %Q proof** (plan §7.1): e2e asserts ramp shape;
  a solo-vs-shadowed byte-diff over a deterministic input vector is
  not yet automated.
- **Identical-input proof vs L1** (§7.2) and **sim parity** (§7.10):
  the L1 shadow-pair mode and L2 `tether-plc-sim` directives
  (`shadow_load`, `expect_shadow`, `shadow_promote`) are not
  implemented.
- **WebUI** (§7.11): deploy-as-shadow, diff table, side-by-side
  watch, confirm-with-summary promote — not started.
- Multiple-shadow, per-channel promote, shadow→image preview:
  explicit non-goals, unchanged.
