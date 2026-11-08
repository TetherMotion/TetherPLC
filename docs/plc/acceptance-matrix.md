# Acceptance matrix

Chapter acceptance criteria → covering test(s) → status. The
surviving plan chapters (12, 14, 15, 16) plus DESIGN-GOALS §6 are
the v1 acceptance surface; chapters 00–11 were retired when their
as-built docs landed. Status reflects ctest state on main.

## Doc 12 — Debug & observability

| criterion | covering test | status |
|---|---|---|
| watch/force/write/suspend/step/break ops end-to-end, gated+journaled | `tplc_io_attach` (probe: 26 checks), `authz_audit` (gating+journal), `debug_stress` | green |
| determinism under max-rated debug load | `debug_stress` flood section (cadence + max_late bounds) | green |
| boundary consistency: no torn multi-byte watch at 4 kHz | `debug_stress` torn-read sections (400+200 samples) | green |
| post-swap: re-resolve, alias continuity, vanished→stale | `online_change_e2e` (swap continuity); `DBG_STALE` declared but unemitted — stale-marker path not exercised | gap |
| flight-recorder: pre-fault cycles in trace snapshot | `fault_injection` + trace ops in `tplc_io_attach` | partial — no induced-fault trace check |
| force semantics cycle-accurate; masked-bit no neighbor disturb | `tplc_io_attach` (force/unforce/overlap), `debug_stress` | green |
| sim parity for watch/trace | `debug_stress` sim section, `sim_determinism` | green |

## Doc 14 — Simulation & testing

| criterion | covering test | status |
|---|---|---|
| L1/L2/L3 sims, bit-identical deterministic traces | `sim_vectors`, `sim_determinism` | green |
| oracle-matiec vectors, zero unexplained divergences, GPL clean | `matiec_fuzz`, `gpl_hygiene` | green |
| all §4 suites in CI with green criteria | ctest registry (25 tests), `.github/workflows/ci.yml` | green |
| kill -9 + hang + seq-odd fault injection | `fault_injection`, `watchdog_escalation` | green |
| perf/jitter numbers vs DESIGN-GOALS §6 | `docs/plc/perf-evidence.md` + `tests/perf/evidence.sh` | green |
| coverage gates + error-code coverage zero gaps | `docs/plc/error-code-coverage.md` (29/65 test-hit) | gap — ~40% documented, "zero gaps" not met |

## Doc 15 — Safety & security

| criterion | covering test | status |
|---|---|---|
| sandbox enforced (seccomp, unprivileged, syscall denial) | `sandbox_enforce` | green |
| sign/verify cycle, 3 trust policies, tampered rejected pre-dlopen | `sig_policy` | green |
| safety-channel map/force/write denied at stated layer+code | `safety_boundary` (-414/-415) | green |
| post-link scanner rejects evil .so, accepts corpus | `module_scan` | green |
| audit: mutating ops journaled with identity, durable | `authz_audit` | green |
| license manifest CI gate, no GPL | `license_manifest` | green |
| threat-model §1 controls all green-tested | `docs/plc/threat-model-signoff.md` | green (1 partial: multi-tenant load) |

## Doc 16 — CiA-405 management

| criterion | covering test | status |
|---|---|---|
| §3.2 ops on ctl + tether_io catalog proxy, V6-clean | `mgmt_surface`, `tplc_io_attach` (38 fns) | green |
| lifecycle mapping; illegal transitions → current+requested | `mgmt_surface` (-501 paths) | green |
| full §3.3 prog.status object | `mgmt_surface` (proto v2 tail fields) | green |
| transitions → journal + events; client state machine from events | `mgmt_surface` (events verb), `tplc_io_attach` | green |
| role gating: observer cannot mutate (server-verified) | `authz_audit`, `tplc_io_attach` | green |
| epoch bump invalidates stale clients (V6) | `tplc_io_attach` (schema epoch 2, CatalogChanged) | green |

## DESIGN-GOALS §6 quantitative targets

See `docs/plc/perf-evidence.md` — cadence targets met at 1 kHz/4 kHz,
swap injects no measurable pause, watchdog trip+escalation verified.

## Pending chapters (not yet implemented — spec exists)

| chapter | scope |
|---|---|
| `plan/18-mcu-path` | no-dlopen MCU targets — designed-for material, v1 ships none |
| `plan/19-shadow-run` | shadow program: parallel candidate run, live compare, instant promote — spec written, iteration-2/v1.x candidate |

## Known acceptance gaps (owned, not hidden)

1. `DBG_STALE` path (post-swap stale markers) — code declared, no
   emit site or test. Either implement or drop the reservation.
2. Flight-recorder induced-fault trace content — ops tested, fault-
   triggered capture not asserted.
3. Error-code "zero gaps" (doc 14 §10.6) — report exists
   (`error-code-coverage.md`), coverage ~44%; closing it is test
   work, not feature work.
4. Multi-tenant/cross-instance interference not load-tested
   (threat signoff marks it partial — v1 non-goal).
