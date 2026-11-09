# As-Built: CiA-405-style Program Management Surface

> Status: implemented over the ctl.sock control channel, and exposed
> to tether_io clients by the standalone `tplc-io` provider
> (`docs/plc/tether-io-attach.md`). The CiA-405-style object
> dictionary is implemented over ctl (`OD_READ`/`OD_LIST`/
> `OD_WRITE`, see "Object dictionary" below); a CANopen wire binding
> remains a transport adapter over the same entries.
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
| object dictionary | `OD_READ`, `OD_LIST`, `OD_WRITE` (index/subindex view, entry-level authz) |
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

## Object dictionary (`OD_READ`/`OD_LIST`/`OD_WRITE`)

A CiA-405-style index/subindex view of the same state the status ops
expose, so a CANopen adapter is a transport binding rather than a
second implementation.

| index/sub | name | type | access | source |
|---|---|---|---|---|
| `0x2000/0` | state | u8 | ro | `prog_state` |
| `0x2001/0` | program_uuid | bytes16 | ro | descriptor |
| `0x2002/0` | cycles | u64 | ro | `TplcCtx.cycle_no` |
| `0x2003/0` | last_fault | i32 | ro | fault latch |
| `0x2004/0` | epoch | u64 | ro | `mgmt_epoch` |
| `0x2100/0` | task_count | u32 | ro | descriptor |
| `0x2100/1..n` | task_stats | 7 × u64 | ro | runs, late, faults, overruns, max_late_ns, exec_max_ns, exec_avg_ns |
| `0x2200/0`, `0x2201/0` | image_in_size / image_out_size | u32 | ro | image header |
| `0x2300/0` | version | string | ro | manifest version |
| `0x2301/0` | layout_hash | bytes32 | ro | descriptor |
| `0x2400/0` | retain_dirty_bytes | u64 | ro | retain store |
| `0x2400/1` | retain_flush | u8 (wo) | wo | writing flushes the retain store |

Access model: `OD_READ`/`OD_LIST` are observer ops; `OD_WRITE` is an
engineer op and each object declares its own floor, so an engineer
writing `0x2400/1` (commissioner) gets `TPLC_E_MGMT_AUTHZ` (-502).
Unknown index/sub is `TPLC_E_MGMT_OP` (-500); a write to a read-only
object, or a length/type mismatch, is `TPLC_E_MGMT_SCHEMA` (-503).
Writes that act (the flush) land in the audit journal as `od.write`.

Coverage: `tests/scan/od_surface.sh` (ctest `od_surface`).
