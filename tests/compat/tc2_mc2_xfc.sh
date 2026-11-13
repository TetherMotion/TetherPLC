#!/usr/bin/env bash
# tc2_mc2_xfc.sh — Tc2_MC2_XFC e2e: builds
# compat/tc2/tc2_mc2.st + compat/tc2/tc2_mc2_xfc.st as --library
# packages, consumes them into tests/compat/tc2_mc2_xfc_case.st via
# --lib, and replays through tether-plc-sim (in-process tplc-motion).
# The program creates one simulated CiA402 axis, powers it, presets
# position, ramps a velocity move, then exercises the XFC kinematic
# blocks (PositionAtTime/TimeOfPosition/TimeToPosition/
# TimeToModuloPosition over the TON scan clock), the DC-time helpers,
# the position/time/break cams, MC_DigitalCamSwitch(+MultiEdge) and
# XFC_TouchProbe — packing 16 checks into %QD0; the last step
# expects mask 0xffff.
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

"$TPLCC" build --library --name Tc2_MC2_XFC \
    compat/tc2/tc2_mc2_xfc.st -o "$WORK/tc2_mc2_xfc.tplc" \
    >/dev/null 2>&1 \
    && ok "Tc2_MC2_XFC library" || { bad "Tc2_MC2_XFC library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_mc2_xfc_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc2_mc2_xfc.tplc" \
    -o "$WORK/tc2_mc2_xfc_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" \
    || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 300); do
        t=$((i * 10000000))
        if [ "$i" -eq 300 ]; then
            echo "step $i time=$t in 0=00 expect 0=ffff0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mcxfc.steps"

IMG=/tplc_tc2mcxfc_$$; CTL="$WORK/mcxfc.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_mc2_xfc_case.tplc" \
    --steps "$WORK/mcxfc.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: XFC kinematics/cams/probe"
else
    bad "sim replay: XFC kinematics/cams/probe"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
