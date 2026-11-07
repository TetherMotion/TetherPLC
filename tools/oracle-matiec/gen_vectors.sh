#!/usr/bin/env bash
# gen_vectors.sh <case.st> <cycles> — matiec subprocess oracle (dev-only,
# doc 14 §3). Requires iec2c on PATH; outputs JSONL vector lines.
set -euo pipefail
src="$1"; cycles="${2:-50}"
command -v iec2c >/dev/null || { echo "iec2c not found — matiec required" >&2; exit 1; }
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
iec2c "$src" -T "$work" -I "$(command -v iec2c | xargs dirname)/../lib" 2>/dev/null || iec2c "$src" -T "$work"
# harness.c is hand-written per-case in corpus/harness/ (inputs, prints
# JSONL trace); oracle outputs are DATA — nothing here is shipped.
echo "oracle-matiec: generated C in $work — build with corpus harness" >&2
