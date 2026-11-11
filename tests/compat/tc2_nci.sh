#!/usr/bin/env bash
# tc2_nci.sh — Tc2_NCI channel-interpolation e2e (plan 21 C6):
# builds compat/tc2/tc2_mc2.st + compat/tc2/tc2_nci.st as --library
# packages, consumes them into tests/compat/tc2_nci_case.st via
# --lib, and replays through tether-plc-sim. The program creates a
# 3-axis group, powers it, runs a coordinated FB_NciLine move to
# (1000,600,300) streaming CSP setpoints, verifies the Euclidean
# path length and that every axis reached its coordinate, runs a
# return move to the origin, and E-stops the group — packing 8
# checks into %QD0; the last step expects mask 0xff. In-process
# tplc-motion.
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
"$TPLCC" build --library --name Tc2_NCI \
    compat/tc2/tc2_nci.st -o "$WORK/tc2_nci.tplc" >/dev/null 2>&1 \
    && ok "Tc2_NCI library" || bad "Tc2_NCI library"

"$TPLCC" build tests/compat/tc2_nci_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc2_nci.tplc" \
    -o "$WORK/tc2_nci_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 500); do
        t=$((i * 10000000))
        if [ "$i" -eq 500 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/nci.steps"

IMG=/tplc_tc2nci_$$; CTL="$WORK/nci.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_nci_case.tplc" \
    --steps "$WORK/nci.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: 3-axis group power/line move/return/estop"
else
    bad "sim replay"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
