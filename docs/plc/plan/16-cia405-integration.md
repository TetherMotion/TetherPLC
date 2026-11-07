# 16 — CiA 405-style Program Management & Management Protocol

> **Status:** work-item document; deleted when `docs/plc/management.md`
> exists and the management surface is live over `tether_io` (and,
> where scoped, over CANopen/EtherCAT-facing profile semantics).
>
> **Satisfies:** G9 (CiA 405 management semantics), G8 (discoverable
> surface), persona (familiar program-control model).
> **Depends on:** 04 (lifecycle), 09 (catalog), 10 (deploy), 12 (debug
> ops), 15 (authZ). **Informs:** 11, 17.

---

## 1. Why CiA 405 as the management model

CiA 405 is the CANopen device profile for **IEC 61131-3 programmable
devices** — it standardizes how a PLC's programs are *managed* over a
network: program download, start/stop/reset, state reporting, and IO
interface control. Tether already implements CiA 405 **device-side
profile objects** (the profile exists in the codebase for controlling
*external* CiA-405 devices). Adopting its semantics for **our own
runtime's** management surface gives:

- a proven, standard vocabulary for program lifecycle (avoids
  inventing bespoke state names that confuse PLC people),
- alignment with the profile code that already exists in Tether
  (documentation/diagnostic tooling reuse),
- a credible path to expose TetherPLC as a **CiA-405-manageable
  device** to third-party tools later (differentiation: our soft-PLC
  is *itself* manageable like a real one).

Scope honesty: v1 adopts the *program-management semantics and state
model* over `tether_io` + the control channel; exposing a full
CANopen/CiA-405 object dictionary endpoint to external managers is
declared a later option (§8), not a v1 deliverable.

## 2. The program-management model

CiA 405's core: **program invocations** in states with transition
commands. Mapping onto doc 04's lifecycle (the runtime's states are
authoritative; the management model is a view):

| CiA-405-ish concept | TetherPLC state | transition ops |
|---|---|---|
| program downloaded | `LOADED`/`READY` | `deploy.*` (doc 10) |
| program created/idle | `READY` | `prog.prepare`, `prog.unload` |
| running | `RUNNING` (+`SWAPPING`) | `prog.start` |
| stopped | `READY` (stopped is not a separate state — READY means "prepared, not scheduled") | `prog.stop`, `prog.kill` |
| halted | `ERROR` | (implicit on fault) |
| resume semantics | `task.resume` | debug ops |
| reset | `READY` after re-init | `prog.reset` (warm/cold flag) |
| delete | `EMPTY` | `prog.unload` |

- **Program invocation = program instance** (doc 04); multiple
  invocations map 1:1 (the profile's multi-program model fits our
  multi-instance runtime).
- Commands are **acknowledged + journaled**; illegal transitions
  return `PROGRAM_STATE` with current+requested (no silent no-ops —
  CiA 405 itself reports state explicitly, we mirror that honesty).

## 3. The management surface

### 3.1 Transport split (same split as everything else)

| surface | transport | contents |
|---|---|---|
| data plane | SHM image (doc 03) | %I/%Q — never management |
| management plane | `tether-plc` ctl.sock (local), `tether_io` (remote via bridge) | all ops below |

The management plane is schema-exposed: each op group becomes
`tether_io` catalog functions/parameters per doc 09 §6 — the web UI
(and any `tether_io` client) sees the full surface with no bespoke
protocol.

### 3.2 Op catalog (v1, consolidated view — schemas in as-built doc)

| group | ops |
|---|---|
| program | `prepare start stop reset kill unload rollback status list` |
| deploy | `begin chunk commit abort status` + `pkg.list/info` |
| task | `stats suspend resume step` + task table view |
| image | `stats channels force? (channels ro)` |
| debug | `resolve watch.* write force.* break_when state diff trace.*` |
| retain | `status flush clear dump` |
| map | `get verify` |
| runtime | `status config.get logs.subscribe counters` |
| journal | `tail subscribe` (event journal access) |

Each op: request/response schema + `req_id` correlation + idempotent-
safe retry where meaningful + authZ role requirement + journal
emission. Structured errors from the owning chapter's code space.

### 3.3 Program status object (the "405 answer")

`prog.status` returns, per instance:

```
{ program_uuid, name, version, package_hash,
  state, sub_state?, fault: {code, subcode, task, cycle}?,
  tasks: [{name, kind, period_ns, runs, overruns, exec_us_max,
           state}],
  layout_hash, retain: {restored, dirty_bytes, last_flush},
  lineage: {predecessor_version?, deployed_at, deployed_by},
  swap?: {stage, quarantine_left} }
```

— the complete answer to "what is running and how did it get here".

## 4. State-change events

`prog.status` subscriptions + journal events on every transition
(`loaded/prepared/started/stopped/error/swapping/committed/aborted/
rolled_back/fault`) — UI state machines and external monitors react to
*events*, not polling (the journal is the durable log; subscriptions
are the live feed).

## 5. Relationship to tether_io schema negotiation

- PLC program variables export as parameters/signals (doc 09 §6);
  management ops export as catalog functions; status objects as
  schema-typed values — all under the V6 bootstrap (client sees them
  after schema commit like everything else).
- **Epoch discipline**: program swap bumps the catalog epoch; V6's
  stale-epoch rejection handles the UI's re-resolution naturally —
  the management surface rides the proven versioning machinery.

## 6. Multiple machines/instances

Ops take `instance`/`program_uuid` selectors; the bridge multiplexes
per machine instance (doc 04 config). Fleet-scale orchestration
(N devices) is a later product layer — the op surface already carries
the selectors it needs.

## 7. Iteration-2 hooks

- CiA 405 **object-dictionary endpoint**: serving an actual OD
  (program download via SDO segmented transfer, control via
  PDOs/NMT-like commands) over Tether's existing CiA405 slave-profile
  machinery — makes TetherPLC manageable by generic CANopen tools.
  Big product idea, modest lift once v1 exists (the state model
  already matches). Planned, not committed.
- SFC: step-level state additions to prog.status (active steps) —
  field reserved.
- OPC-UA exposure: another adapter on the same op surface (no
  protocol invention needed — the catalog is already typed).

## 8. Error codes (owned by this chapter)

`TPLC_E_MGMT_OP, _MGMT_STATE, _MGMT_AUTHZ, _MGMT_SCHEMA`

## 9. Acceptance criteria

1. All §3.2 ops live on the control channel + proxied via bridge into
   `tether_io` catalog (discoverable post-commit, V6-clean).
2. Lifecycle command/state mapping matches §2 table; illegal
   transitions error with current+requested.
3. `prog.status` returns the full §3.3 object for multi-instance
   fixtures.
4. Transitions emit journal events + live notifications; a client can
   run a state machine purely from events.
5. Role gating: observer cannot invoke mutating ops (server-verified).
6. Epoch bump on swap invalidates stale clients per V6 rules.
