# tools/oracle-matiec — quarantined GPL oracle adapter (dev-only)

Per doc 14 §3: matiec (GPL) is a *subprocess oracle only*. It is
downloaded on demand by `ensure_matiec.sh` into `MATIEC_HOME`
(default `~/.cache/tplc-oracle/matiec`, outside the repo), built there,
and invoked as a subprocess. Its emitted C is compiled into a throwaway
stdio runner (`ref_runner.c`) — none of it is vendored, linked, or
copied into shipped artifacts; the produced trace is data.

## Pipeline (diff_fuzz.py)

Per case a random PROGRAM in the common ST subset is generated once and
rendered twice — the engines' surface syntax differs:

- ours: located vars share the VAR block, FB outputs read `inst.PIN`
- matiec: located vars need their own VAR block *after* plain vars,
  `inst.PIN` does not exist (outputs are wired `PIN => tmp`), CTU/CTD
  expose `Q`/`CV` where we expose `QU`/`QD`/`CV`

```
diff_fuzz.py --seed S --cases N --cycles M [--l2]
  gen case → case_m.st / case_ours.st
  iec2c case_m.st → cc ref_runner.c + POUS.c → ./ref
  random cycles (i bytes, t_ns) → ref → %Q trace
  → vector line {expect_q, source:"matiec-<sha>"}
  → tplcc vectors run (L1; --l2 replays via vector_run.py)
```

A FAIL line is a semantic divergence: fingerprint `mfuzz.<seed>.c<k>`,
triaged in `known_divergences.txt` (each with its reason). Unknown ids
fail the run; `--update-known` appends new ids for triage. Cases and
traces stay under `build/matiec-fuzz/` for reproduction.

## Exit codes

- 0 — all agree, or only known divergences
- 1 — new divergence or oracle error
- 77 — skipped (iec2c/tplcc missing; `ensure_matiec.sh` fetches it)

## Status

Working: 12-case default corpus plus `--seed` sweeps pass except the
two documented delta classes in `known_divergences.txt` (F_TRIG init
convention, CV clamp [0,PV] — see DECISIONS.md D-IMPL-17).
