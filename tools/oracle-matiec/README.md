# tools/oracle-matiec — quarantined GPL oracle adapter (dev-only)

Per doc 14 §3 / G12: matiec (GPL) is a *subprocess oracle only* — its
outputs are data (expected IO traces), never compiled into, linked
with, or copied into shipped artifacts. This directory is excluded
from release builds and license-scanned in CI.

## Pipeline (requires matiec `iec2c` on PATH — not vendored)

```
gen_vectors.sh case.st out.jsonl
  iec2c case.st -o build/        → matiec-generated C sources
  cc harness.c build/*.c -o ref  → reference executable
  ./ref --cycles N --trace       → JSONL of {i,t_ns,%Q,fields}
  → emit vector lines with "source":"matiec-<ver>"
```

The same vectors then run through `tplcc vectors run` (L1 interp now;
L2/L3 sim + target later — one vector set, three evidence levels).

## Status

Skeleton: no matiec installed in the dev image. Initial corpus
(`../oracle/corpus/*.jsonl`) is `source:"spec"` — hand-derived from
doc 07 App C semantics, replaced by matiec traces when available.
Vectors marked `ours-only` where matiec can't express our dialect.
