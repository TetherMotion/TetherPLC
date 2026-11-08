# Simulation (as-built)

Three nested levels, all consuming the same canonical representation:

| level | what runs | host |
|---|---|---|
| L1 unit-sim | a single program interpreted in-process | `tplc-mir` interpreter (`interp.rs`) — virtual image + virtual clock |
| L2 process-sim | `tether-plc` binary against a SHM image file | real process + real doc-03 image protocol, scripted `%I` sequences via `tests/sim/vector_run.py` |
| L3 system-sim | the artifact inside Tether's simulated-plant fleet | map → bridge → image → runtime → virtual IO → simulated machine physics |

The `.so` under test is byte-identical to the deployed one (the
package `target_obj` for the sim host's triple). "What runs in CI is
what ships" — JIT/bitcode is a fallback, not the preferred path.

## Determinism contract (G7)

For an identical initial state blob and an identical **input
sequence** (latched `%I` + control ops + forces), a program produces
bit-identical `%Q` commit sequences, boundary blob contents,
watch/trace samples, and fault events — across target, L1, L2, L3.

What makes it true:

- all time flows through `ctx->now_ns`/`svc->steady_ns` — sim hosts
  inject a virtual clock driving the release *sequence*;
- all IO flows through the latched image — sim injects sequences;
- no nondeterminism sources inside generated code (runtime-abi.md);
- scheduling differences change *when* tasks run, never *what* they
  compute — trace comparison works on value sequences per
  task-cycle-index, not wall time.

`sim_determinism` gates same-input-twice parity; `debug_stress` adds
a sim-parity section for the watch path; `sim_vectors` runs the
oracle corpus through the L2 vector runner.

## Oracle harness (`tools/oracle-*`, GPL quarantine)

- `tools/oracle-matiec/ensure_matiec.sh` downloads matiec **on
  demand** into `~/.cache/tplc-oracle/` — outside the repo, never
  linked, invoked only as a subprocess producing reference vectors.
  `gpl_hygiene` scans for contamination.
- `tools/oracle-matiec/ref_runner.c` + `diff_fuzz.py`: differential
  fuzz of our L1 interpreter vs matiec's `iec2c` output;
  `known_divergences.txt` is the triage ledger — every divergence
  is either explained there or a bug.
- `tools/oracle/corpus/` + `tests/sim/vector_run.py`: checked-in
  reference vectors replayed through L2 (`sim_vectors` ctest).
- `tools/oracle-beremiz/` holds an additional corpus (Beremiz
  dialect coverage).

## Divergence handling

A sim/target divergence is a defect class of its own: reported with
first-divergent cycle + field, triaged into `known_divergences.txt`
with a reason, or fixed. The corpus is the contract — a green
`sim_vectors` + clean `known_divergences.txt` is the parity gate.
