#!/usr/bin/env bash
# tc3_mll_io.sh — Tc3_MLL e2e: TTS/SR command handshake and
# honest engine-absent error reporting.  Skips (77) when the
# toolchain is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc3_MLL \
    compat/tc3/tc3_mll.st -o "$WORK/tc3_mll.tplc" >/dev/null 2>&1 \
    && ok "Tc3_MLL library" || { bad "Tc3_MLL library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_mll_case.st \
    --lib "$WORK/tc3_mll.tplc" \
    -o "$WORK/tc3_mll_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 300); do
        t=$((i * 10000000))
        if [ "$i" -eq 300 ]; then
            echo "step $i time=$t in 0=00 expect 0=1f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mll.steps"

IMG=/tplc_tc3mll_$$; CTL="$WORK/mll.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_mll_case.tplc" \
    --steps "$WORK/mll.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: mll e2e"
else
    bad "sim replay: mll e2e"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
