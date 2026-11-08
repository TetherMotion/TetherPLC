# Testing strategy (as-built)

The test taxonomy, gates, and coverage policy. The ctest registry is
the source of truth (`ctest -N`); this page explains the shape.

## Layers

| layer | what it proves | tests |
|---|---|---|
| unit | host/image/pkg/ABI correctness in-process | `image_conformance`, `abi_conformance`, `pkg_conformance`, `retain_store`, `ctl_smoke` |
| runtime gates | lifecycle, online change, fault injection, watchdog | `fault_injection`, `online_change_e2e`, `watchdog_escalation` |
| security envelope | sandbox, signing, scanner, authZ, safety boundary, licenses | `sandbox_enforce`, `sig_policy`, `module_scan`, `authz_audit`, `safety_boundary`, `license_manifest`, `gpl_hygiene` |
| management/attach | ctl surface + tether_io bridge | `mgmt_surface`, `tplc_io_attach` |
| determinism/debug | torn reads, cadence under flood, sim parity | `debug_stress`, `sim_determinism`, `sim_vectors` |
| compiler e2e | ST→package, FBD gallery, oracle diff-fuzz | `fbd_gallery_e2e`, `matiec_fuzz`, `st_fuzz` |
| hygiene | no-allocs, sanitizers, GPL quarantine | `no_alloc`, `sanitizers` |

## Gates policy

- Everything above runs in CI (`.github/workflows/ci.yml`).
- Tests that need optional tooling (tplcc/LLVM, matiec, cargo
  metadata) exit 77 = ctest SKIP rather than fail on a thin box.
- `debug_stress` flood-lateness is a known flake on loaded hosts —
  cadence is the real gate; the 5 ms lateness bound assumes an
  unloaded machine.
- Perf numbers are evidence, not gates: `tests/perf/evidence.sh`
  regenerates `perf-evidence.md` manually.
- Error-code coverage is reported, not enforced:
  `tools/error_code_report.py` → `error-code-coverage.md`.

## Fuzz/property coverage

- `st_fuzz` — ST frontend robustness on generated programs.
- `matiec_fuzz` — differential: our L1 interpreter vs the matiec
  oracle on the IEC-subset corpus (subprocess only, GPL-quarantined).
- `known_divergences.txt` — the divergence ledger (see
  simulation.md); unexplained entries are release blockers.

## Coverage targets (release criteria)

- Zero ctest regressions on main.
- Oracle trace-diff: zero *unexplained* divergences on the corpus.
- Error-code coverage: report published; the "zero gaps" aspiration
  is documented-honest at ~44% (error-code-coverage.md) — closing
  it is test work.
- perf-evidence.md refreshed per release on a stated host.
