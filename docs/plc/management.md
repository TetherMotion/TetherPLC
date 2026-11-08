# As-Built: CiA-405-style Program Management Surface

> Status: implemented over the ctl.sock control channel, and exposed
> to tether_io clients by the standalone `tplc-io` provider
> (`docs/plc/tether-io-attach.md`). A CANopen object-dictionary
> endpoint remains declared iteration-2 (doc 16 §7) — the state model
> and op surface below are what those adapters must expose.
>
> `doc 16`/`doc 15` section references below are provenance to the
> retired plan chapters; this document is authoritative (see
> `architecture.md` §7).

## State model (doc 16 §2 mapping)

Authoritative runtime states (`ProgState` in `runtime/tether-plc/main.cpp`):

| management state | runtime | entered by |
|---|---|---|
| downloaded/created | `STOPPED` (READY) | `prepare`, `deploy.commit`, swap |
| running | `RUNNING` | `start` |
| stopped | `STOPPED` | `stop`, `kill`, `reset` |
| halted | `FAULT` | task fault, watchdog stall |
| deleted | `EMPTY` | `unload` |

**Illegal transitions are errors**: `start` on RUNNING, `stop` on
non-RUNNING, `kill/reset` outside their required states, `unload`/
`prepare`/`swap` while RUNNING all return `TPLC_E_MGMT_STATE` (-501)
with `current=<n> requested=<op>` in the message — no silent no-ops.

## Op catalog (doc 16 §3.2 → ctl ops)

| group | ops |
|---|---|
| program | `PROG_PREPARE/START/STOP/RESET/KILL/UNLOAD/STATUS/LIST/LAYOUT/SWAP/ROLLBACK` |
| deploy | `DEPLOY_BEGIN/CHUNK/COMMIT/ABORT`, `PKG_LIST/INFO` |
| task | `TASK_STATS/SUSPEND/RESUME/STEP` |
| image | `IMAGE_STATS`; channels via `DBG_FIELDS` |
| debug | `DBG_RESOLVE/READ/WRITE/STATE`, `DBG_FORCE*`, `DBG_BREAK*`, `DBG_CAPTURE*`, `DBG_WATCH_*`, `DBG_STATE_DIFF`, `DBG_FIELDS`, `TRACE_*` |
| retain | `RETAIN_STATUS/FLUSH/CLEAR/DUMP` |
| map | `MAP_GET` (deploy-map section bytes, observer role) |
| runtime | `RT_STATUS`, `RT_SHUTDOWN` |
| journal | `JOURNAL_TAIL` (durable audit tail), `SET_NOTIFY` + `TPLC_EV_*` (live feed) |

## prog.status object (`TplcCtlProgStatus`, proto v2)

`program_uuid[16] name[64] state task_count cycles last_fault_code
force_count` then the v2 tail: `package_hash[32]` (pkg content_hash),
`layout_hash[32]`, `version[32]` (manifest `version`, set by
`tplcc build --prog-version`), `fault_task/fault_cycle`,
`retain_restored/retain_dirty_bytes/retain_last_flush_ns`,
`deployed_at_ns` (realtime), `deployed_by_uid` (SO_PEERCRED of the
prepare/swap requester; runtime's own uid for a boot deploy),
`predecessor_uuid[16]`,
`swap_stage` (0 none / 1 quarantine), `quarantine_left`, `epoch`.
Per-task rows: `name kind state period_ns runs late faults last_rc
last_late_ns overruns exec_max_ns exec_avg_ns`.

## Events (doc 16 §4)

`TPLC_EV_PROG_TRANSITION` frames carry `TplcCtlEvTransition`:
`program_uuid from_state to_state reason(0 op,1 fault,2 rollback)
ts_ns epoch` — a client can run the §2 state machine purely from
events. Emitted on start/stop/kill/unload/prepare/swap/rollback and
the fault path. `TPLC_EV_FAULT` carries the fault rc. `tplc-ctl
events [SEC]` subscribes to all categories and dumps frames.

## Epoch discipline (doc 16 §5)

`mgmt_epoch` bumps on every program-set change: boot load, prepare,
swap commit, unload, rollback (including auto-rollback). Exposed in
`prog.status.epoch` and in every transition event — the bridge maps
it onto the `tether_io` catalog epoch so V6 stale-epoch rejection
invalidates cached program views.

## Lineage (doc 16 §3.3)

On every package load the runtime records `content_hash`, manifest
`version`, realtime timestamp, and the requesting connection's
kernel-attested uid. The replaced program's uuid becomes
`predecessor_uuid`; the quarantined side also keeps the prior package
identity so rollback restores lineage, not just code.

## Role gating (doc 15 §5 + doc 16 §9.5)

Minimum roles per op in `op_min_role()`: read-only status/map/
journal/debug-read ops at OBSERVER; watch arm/stream and debug writes
at ENGINEER; everything else mutating (lifecycle, deploy, force,
suspend, clear, retain, shutdown) at COMMISSIONER. Role resolves from
`SO_PEERCRED` uid via `--authz <file>` map, defaulting unmapped
non-server users to OBSERVER.

## Error codes (doc 16 §8)

`TPLC_E_MGMT_OP=-500`, `_MGMT_STATE=-501`, `_MGMT_AUTHZ=-502`,
`_MGMT_SCHEMA=-503` in `include/tplc/tplc_ctl.h`.

## Multi-instance readiness

Ops are written against the single loaded program of a `tether-plc`
instance; instance selection is the bridge's multiplexing concern
(one ctl.sock per runtime). `program_uuid` selectors ride in status
and event payloads already.

## Verification

`tests/scan/mgmt_surface.sh` (ctest `mgmt_surface`): v2 status fields
populated, strict-transition `-501` errors with current+requested,
transition events observable via subscription with epoch bumps,
journal tail + map get. Role gating is covered by `authz_audit`.
