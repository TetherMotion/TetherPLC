#!/usr/bin/env bash
# tc2_mc2.sh — Tc2_MC2 axis-management wrappers e2e (plan 21 C5):
# build compat/tc2/tc2_mc2.st as a --library package, consume it
# into tests/compat/tc2_mc2_case.st via --lib, and replay through
# tether-plc-sim. The program creates a simulated CiA402 axis and
# drives MC_Power / MC_SetPosition (abs+rel) / MC_Read* / MC_Reset
# / MC_Power-disable, packing 10 checks into %QD0; the last step
# expects mask 0x3ff. In-process tplc-motion, no external peer.
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

"$TPLCC" build tests/compat/tc2_mc2_case.st \
    --lib "$WORK/tc2_mc2.tplc" \
    -o "$WORK/tc2_mc2_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 299); do
        t=$((i * 10000000))
        if [ "$i" -eq 299 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff030000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mc2.steps"

IMG=/tplc_tc2mc2_$$; CTL="$WORK/mc2.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_mc2_case.tplc" \
    --steps "$WORK/mc2.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: power/setpos/read/reset/disable"
else
    bad "sim replay"
    tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
