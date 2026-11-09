# 17 — Milestones: status board and v1 exit

> **Status:** work-item document; deleted when v1 ships and the
> remainder re-plans under iteration 2. Landed-phase detail was
> pruned — see git history for the full phase/task breakdown.

---

## 1. Status board

P0–P9 and the shadow-run feature **landed**; their chapters (01–16,
19) are retired into `docs/plc/` as-builts and
`docs/plc/acceptance-matrix.md` is the live criterion→test record.

| phase | scope | status |
|---|---|---|
| P0 | foundations: ABI/image/pkg contracts, host skeleton, stub .so | landed |
| P0.5 | frontend spikes (DECISIONS.md §7) | landed |
| P1 | compiler MVP: ST → mir → LLVM → .tplc, tplc-rt, tplcc | landed |
| P2 | runtime: retain, watchdog, fault policies, bridge, catalog | landed |
| P3 | online change: migrate codegen, stage/arm/commit, rollback | landed |
| P4 | FBD: model, plcopen-io, lowering, validation rules | landed |
| P5 | web IDE (TetherWebUI `TetherPLC` tab) | landed |
| P6 | debug plane: watch/force/write, suspend/step, trace | landed |
| P7 | sim/oracle/CI hardening (continuous) | landed |
| P8 | safety & security: sandbox, signing, authZ, scanner | landed |
| P9 | management surface: CiA-405 ops, events, epochs, roles | landed |
| — | shadow run (was planned as doc 19 / P11-class) | landed |
| **P10** | **v1 hardening & release** | **in flight** |

## 2. P10 — v1 hardening & release (remaining work)

Done in the P10 sweep: as-built specs written, perf evidence vs
DESIGN-GOALS §6, error-code coverage report, threat-model signoff,
field-trial checklist, release notes, watchdog escalation fix,
test-serialization fix.

Still open (acceptance-matrix "Known acceptance gaps"):

1. ~~`TPLC_E_DBG_STALE`~~ — closed: `DBG_RESOLVE`/`DBG_WATCH_ADD`
   emit -411 for names removed by the last committed swap;
   `online_change_e2e` asserts -411 vs -410.
2. ~~Flight-recorder induced-fault trace~~ — closed: `flight_recorder`
   induces a hang→abort freeze and reads pre-fault samples.
3. ~~Error-code coverage~~ — closed: `error_paths` induces every
   reachable code; 62/75 test-hit, all emitted codes covered except
   race-only `IMG_EPOCH`; reservations documented.
4. Multi-tenant/cross-instance interference — not load-tested;
   marked v1 non-goal in the threat-model signoff.
5. **v1 tag** — 1–3 closed; owner directive: the v1 tag will not be
   cut explicitly. P10 is complete; further work is iteration-2.

## 3. Definition of done

Chapter acceptance criteria green; as-built doc written; error codes
tested; perf numbers recorded; license/scan gates clean;
DECISIONS.md updated for every deviation.

## 4. Iteration 2 (post-v1, recorded not scheduled)

SFC language + editor; LD view/editor; multi-program IDE flows;
CiA-405 OD endpoint; MCU loader path (`18-mcu-path.md` — the
static-link ABI shim criterion landed as ctest `mcu_shim`; a
cross-compiled Cortex-M loader remains open);
WSTRING/LTIME/REF dialect extensions (WSTRING is a real UTF-16 type
and LTIME rides the i64-ns Time representation; the standard function library landed (string, scalar,
conversions, TRUNC/ROUND/MUX); REF still open); collaborative editing; visual
diff; OPC-UA adapter; safety FB subset (certification program).
Shadow-run extension candidates (all currently non-goals):
multiple simultaneous shadows, per-channel promote, shadow→image
preview.
