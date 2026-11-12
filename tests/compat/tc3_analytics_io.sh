#!/usr/bin/env bash
# tc3_analytics_io.sh — Tc3_Analytics e2e: TTS/SR command handshake and
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

"$TPLCC" build --library --name Tc3_Analytics \
    compat/tc3/tc3_analytics.st -o "$WORK/tc3_analytics.tplc" >/dev/null 2>&1 \
    && ok "Tc3_Analytics library" || { bad "Tc3_Analytics library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_analytics_case.st \
    --lib "$WORK/tc3_analytics.tplc" \
    -o "$WORK/tc3_analytics_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 300); do
        t=$((i * 10000000))
        if [ "$i" -eq 300 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/analytics.steps"

IMG=/tplc_tc3analytics_$$; CTL="$WORK/analytics.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_analytics_case.tplc" \
    --steps "$WORK/analytics.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: analytics e2e"
else
    bad "sim replay: analytics e2e"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
