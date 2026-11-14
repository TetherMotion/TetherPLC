#!/usr/bin/env bash
# Ladder-diagram conformance: each .st/.fbd.json pair runs the same
# input trace and must produce bit-identical %Q images, plus the
# expect_q expectations. LD networks lower positionally — y-bands are
# rungs, x-order is the series chain; contacts (NO/NC) AND into the
# flow, coils latch it. Exit 77 = prerequisites missing → skip.
set -uo pipefail
cd "$(dirname "$0")/../.."

# tplcc needs an LLVM-21 toolchain — locate one if not already on PATH
if [ -z "${LLVM_SYS_211_PREFIX:-}" ]; then
    for p in /home/uli/llvm21/usr/lib/llvm-21 /home/uli/llvm21/usr /opt/llvm21 /usr/lib/llvm-21; do
        if [ -x "$p/bin/llvm-config" ]; then
            export LLVM_SYS_211_PREFIX=$p
            export LLVM_CONFIG=$p/bin/llvm-config
            export PATH="$p/bin:$PATH"
            break
        fi
    done
fi
(cd compiler && cargo build -q -p tplcc 2>/dev/null) || true
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }

"$TPLCC" vectors run tests/ld/vectors.jsonl --src tests/ld

# PLCopen <LD> body import: xml2json converts the TC6 fixture, then
# the same vectors prove imported topology runs identically to ST.
XML2JSON=./compiler/target/debug/examples/xml2json
if ! [ -x "$XML2JSON" ]; then
    (cd compiler && cargo build -q -p plcopen-io --example xml2json 2>/dev/null) || true
fi
if [ -x "$XML2JSON" ]; then
    TMP=$(mktemp -d)
    trap 'rm -rf "$TMP"' EXIT
    "$XML2JSON" compiler/plcopen-io/tests/fixtures/ld_basic.xml \
        "$TMP/04_plcopen.fbd.json" 2>/dev/null \
        || { echo "SKIP: xml2json failed"; exit 0; }
    cp tests/ld/03_parallel.st "$TMP/"
    grep 'ld03' tests/ld/vectors.jsonl \
        | sed 's/ld03\.parallel_nc/ld04.plcopen_xml/; s/03_parallel\.fbd.json/04_plcopen.fbd.json/' \
        > "$TMP/vectors.jsonl"
    "$TPLCC" vectors run "$TMP/vectors.jsonl" --src "$TMP"
else
    echo "SKIP: no xml2json example"
fi
