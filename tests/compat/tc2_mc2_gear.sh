#!/usr/bin/env bash
# tc2_mc2_gear.sh — Tc2_MC2 tranche-5 wrappers e2e (plan 21 C5):
# builds compat/tc2/tc2_mc2.st as a --library package, consumes it
# into tests/compat/tc2_mc2_gear_case.st via --lib, and replays
# through tether-plc-sim. The program creates TWO simulated CiA402
# axes (master + slave), powers both, runs the master in velocity
# mode, captures a touch-probe position latch, couples the slave at
# a 2:1 velocity ratio via MC_GearIn, uncouples with MC_GearOut,
# halts the master and powers both off — packing 9 checks into
# %QD0; the last step expects mask 0x1ff. In-process tplc-motion.
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

"$TPLCC" build --library --name Tc2_MC2 \
    compat/tc2/tc2_mc2.st -o "$WORK/tc2_mc2.tplc" >/dev/null 2>&1 \
    && ok "Tc2_MC2 library" || bad "Tc2_MC2 library"

"$TPLCC" build tests/compat/tc2_mc2_gear_case.st \
    --lib "$WORK/tc2_mc2.tplc" \
    -o "$WORK/tc2_mc2_gear_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 500); do
        t=$((i * 10000000))
        if [ "$i" -eq 500 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff010000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mcgear.steps"

IMG=/tplc_tc2mcgear_$$; CTL="$WORK/mcgear.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_mc2_gear_case.tplc" \
    --steps "$WORK/mcgear.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: 2-axis power/vel/touchprobe/gear/gearout/halt/off"
else
    bad "sim replay"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
