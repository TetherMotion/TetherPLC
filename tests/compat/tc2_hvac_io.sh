#!/usr/bin/env bash
# tc2_hvac_io.sh — Tc2_HVAC pure-ST e2e: builds
# compat/tc2/tc2_hvac.st as a --library package and replays
# tc2_hvac_case.st through tether-plc-sim.
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

"$TPLCC" build --library --name Tc2_HVAC \
    compat/tc2/tc2_hvac.st -o "$WORK/tc2_hvac.tplc" >/dev/null 2>&1 \
    && ok "Tc2_HVAC library" || { bad "Tc2_HVAC library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_hvac_case.st \
    --lib "$WORK/tc2_hvac.tplc" \
    -o "$WORK/tc2_hvac_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 200); do
        t=$((i * 10000000))
        if [ "$i" -eq 80 ]; then
            echo "step $i time=$t in 0=00 expect 0=ffff"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ba.steps"

IMG=/tplc_tc2ba_$$; CTL="$WORK/ba.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_hvac_case.tplc" \
    --steps "$WORK/ba.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: analog/actuators/pumps/mux/redundancy/ctrl/pid/curve/alarm/anti-block/blink/bar"
else
    bad "sim replay: analog/actuators/pumps/mux/redundancy/ctrl/pid/curve/alarm/anti-block/blink/bar"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
