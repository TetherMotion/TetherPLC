#!/usr/bin/env bash
# e2e.sh — Atc_Ops library: package, consume via --lib, vector suite.
# Mirrors tests/tempctrl/e2e.sh structure.
set -u
cd "$(dirname "$0")/../.."
ROOT=$PWD
TPLCC=${TPLCC:-$ROOT/compiler/target/release/tplcc}
PKG=${PKG:-$ROOT/compiler/target/release/tplc-pkg}
LIB_ST=libraries/tempctrl/atc_ops.st
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
pass=0; fail=0
ok()  { echo "PASS $*"; pass=$((pass+1)); }
bad() { echo "FAIL $*"; fail=$((fail+1)); }

[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC"; exit 1; }

"$TPLCC" check "$LIB_ST" >/dev/null 2>&1 \
    && ok "tplcc check atc_ops.st" \
    || bad "tplcc check atc_ops.st"

"$TPLCC" build --library --name Atc_Ops "$LIB_ST" \
    -o "$WORK/atc_ops.tplc" >/dev/null 2>&1 \
    && ok "tplcc build --library Atc_Ops" \
    || bad "tplcc build --library failed"

"$PKG" verify "$WORK/atc_ops.tplc" >/dev/null 2>&1 \
    && ok "tplc-pkg verify atc_ops.tplc" \
    || bad "tplc-pkg verify atc_ops.tplc"

INFO=$("$TPLCC" info "$WORK/atc_ops.tplc" 2>/dev/null)
echo "$INFO" | grep -q '"name":"Atc_Ops"' \
    && ok "manifest name" \
    || bad "manifest name"

# vector suite: each case concatenated with the library, driven by
# vectors.jsonl; pass/fail lives on %QB0 (self-checking logic).
for t in tests/ops/cases/o??_*.st; do
    n=$(basename "$t")
    cat "$LIB_ST" "$t" > "$WORK/$n"
done
cp tests/ops/vectors.jsonl "$WORK/vectors.jsonl"
if "$TPLCC" vectors run "$WORK/vectors.jsonl" > "$WORK/vec.log" 2>&1; then
    ok "tplcc vectors: ops suite (19 programs)"
else
    bad "tplcc vectors: ops suite"
    cat "$WORK/vec.log"
fi

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
