# Online Change — State-Preserving Program Replacement

How TetherPLC replaces a running program with a recompiled one without
losing variable state. This is the mechanism behind "edit, build, swap"
in the web IDE, and it is what makes `--change-against` meaningful.

## 1. The problem

A PLC program is not just code. Every `VAR`/`VAR RETAIN`/FB instance
lives in a *state blob* (see `runtime-abi.md`): one relocatable region
whose byte layout is chosen by the compiler. Recompiling after an edit
may produce a different layout — a moved field, a grown array, a new
instance, a widened type. The swap must therefore answer, per state
field: *where did this byte live, where does it live now, and what
transformation is legal on it?*

The design keeps three artifacts separate:

- **LAYD** — the layout descriptor. A canonical serialization of every
  state field's name, type, offset, and size, emitted into the package
  by `tplcc build`.
- **MIGD** — the migration descriptor. The compiler's verdict, field by
  field, on how old state maps onto new state, emitted when
  `tplcc build --change-against <old.tplc>` is used.
- **SYMMAP** — name → offset table used by monitoring/forcing and by
  the migrator's field matching (see `io-mapping.md`).

## 2. The diff engine (`tplc_migrate`)

`tplc_migrate` consumes the old LAYD + the new program's layout and
emits three things: a list of migration **rules**, a `layout_hash` for
the new blob, and a deny/allow verdict.

### Rule kinds

| kind | name | meaning |
|---|---|---|
| 0 | `Copy` | same name, same type, compatible size — memcpy |
| 1 | `Transform` | same name, related type — apply `transform_id` (e.g. `StringTrunc`, `ArrayGrow`, numeric widening/narrowing) |
| 2 | `DefaultInit` | new field, no old counterpart — run its initializer |
| 3 | `Dropped` | old field gone — lost; `retain` flag records whether it was retained (audited as a potential data-loss event) |
| 4 | `Deny` | same name, incompatible change — the whole change is rejected |
| 5 | `Alias` | old field's storage is reused under a new name — no copy |

Each rule carries `old_field`/`new_field` indices (u32::MAX = absent)
and a `severity` (0 informational, 1 lossy/dropped, 2 deny).

### Deny rules

Anything that would make migration unsound denies the whole change at
*compile* time: layout-hash collision with different meaning, retain
region shrinkage that would drop retained state, in-out reference type
changes, and type changes the transform table doesn't cover. Denials
surface through `tplcc` as `TPLC_E_MIGRATE_DENY` diags with a
`MIGRATE_ERR_*` subcode, so the IDE shows a normal compile error.

## 3. The migration ABI

The new module's `.so` exports the same descriptor table as any program
(`tplc_get_descriptor`, `runtime-abi.md`), plus one optional entry:
`migrate(TplcMigrateCtx*)`. When the runtime performs a swap:

1. It quiesces the task scheduler (no cycle may observe half-migrated
   state).
2. It calls the *old* module's `state()` to get the source blob pointer.
3. It walks MIGD rules: `Copy`/`Transform`/`Alias` read old bytes and
   write into the new blob (freshly allocated); `DefaultInit` writes
   only the new blob; `Dropped`/`Deny` never reach this point (deny was
   compile-time; dropped is a no-op write).
4. Transforms that need work area use a scratch region in
   `TplcMigrateCtx`; nothing allocates in the scan path — the scratch
   size is declared in MIGD.
5. On success the new blob becomes the program's state; on failure the
   old blob is retained untouched and the swap aborts.

`mig::Transform` ids are stable numbers shared between the compiler
(`tplc-mir::mig`) and `tplc-rt` — the transform code itself is linked
into the artifact, so an old runtime never executes a transform it
doesn't know.

## 4. The swap lifecycle (ctl ops)

| op | name | role |
|---|---|---|
| `0x0301` | `PROG_PREPARE` | load a staged package, run MIGD dry-run, report `out_status`/`out_first_error` |
| `0x030A` | `PROG_SWAP` | quiesce → migrate → swap code+state pointer → resume; payload is `{pkg uuid\|path}`, reply `{out_status, out_first_error}` |
| `0x030B` | `PROG_ROLLBACK` | restore the previous module + state blob (kept until the next successful swap) |
| `0x0307` | `PROG_STATUS` | reflects the post-swap program; `PROG_TRANSITION` event (0x0E01) is emitted on commit |

**Same-layout fast path:** when `layout_hash` matches the running
program's, no MIGD walk is needed — the runtime does a pure
"module-swap": new code, same state bytes. This is the common case for
body-only edits and skips quarantine.

**Quarantine:** a migrated program runs N cycles
(`--quarantine-cycles`, default small) under observation. If it faults
inside the window, the runtime auto-rolls-back (`--auto-rollback`) and
writes the runbook entry as `error`; a clean window commits.

## 5. Retain interaction

Retain vars are a *subset* of the state blob marked `retain` in LAYD.
During migration each retained field's rule is applied like any other,
but rule kind 3 (`Dropped`) on a retained field bumps the retain
store's *losses* counter and records a runbook note — dropping retained
state is allowed (schema changed on purpose) but always audited. A swap
does not flush retain; the store's `dirty` flag simply follows the new
program's writes (see `plc-runtime.md` §retain).

## 6. Runbook

Every swap appends a record to `--runbook DIR` keyed by
`--instance NAME`: `{uuid, package path, layout hash, outcome}`. This
is the audit trail that lets commissioning answer "which binary ran
this state at time T" — the missing half of IEC online-change on most
runtimes.

## 7. IDE surface

Build → deploy → `PROG_SWAP` is exposed in the deploy pane; the proxy
`/events` stream pushes `PROG_TRANSITION` so status refreshes on commit
rather than on poll. Compile-side, `--change-against` runs when the IDE
(or CI) builds a package destined to replace a specific running one —
the deployed package's LAYD is fetched via `PROG_LAYOUT` (0x0309) to
diff against.
