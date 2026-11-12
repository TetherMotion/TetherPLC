#!/usr/bin/env bash
# tc2_filter_io.sh — Tc2_Filter surface e2e: builds
# compat/tc2/tc2_filter.st as a --library package and replays
# tc2_filter_case.st through tether-plc-sim.  Skips (77) when the
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

"$TPLCC" build --library --name Tc2_Filter \
    compat/tc2/tc2_filter.st -o "$WORK/tc2_filter.tplc" >/dev/null 2>&1 \
    && ok "Tc2_Filter library" || { bad "Tc2_Filter library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_filter_case.st \
    --lib "$WORK/tc2_filter.tplc" \
    -o "$WORK/tc2_filter_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 100); do
        t=$((i * 10000000))
        if [ "$i" -eq 100 ]; then
            echo "step $i time=$t in 0=00 expect 0=1f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/filter.steps"

IMG=/tplc_tc2filter_$$; CTL="$WORK/filter.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_filter_case.tplc" \
    --steps "$WORK/filter.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: limit/pt1/pid/avg/ramp"
else
    bad "sim replay: limit/pt1/pid/avg/ramp"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
