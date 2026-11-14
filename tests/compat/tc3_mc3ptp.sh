#!/usr/bin/env bash
# tc3_mc3ptp.sh — Tc3_Mc3Ptp e2e: builds compat/tc2/tc2_mc2.st and
# compat/tc3/tc3_mc3ptp.st as libraries, consumes them into
# tests/compat/tc3_mc3ptp_case.st via --lib, and replays through
# tether-plc-sim (in-process tplc-motion).  Covers axis create +
# power, torque and hardware-parameter access, wait-for-stable,
# the three accepted no-op FBs, error injection via the 0x200D
# sim-ext object, fault readback and fault-reset clearing.
# Expects mask 0x1ff at the final step.
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
    && ok "Tc2_MC2 library" || { bad "Tc2_MC2 library"; \
         echo "== $pass pass, $fail fail"; exit 1; }
"$TPLCC" build --library --name Tc3_Mc3Ptp \
    compat/tc3/tc3_mc3ptp.st -o "$WORK/tc3_mc3ptp.tplc" >/dev/null 2>&1 \
    && ok "Tc3_Mc3Ptp library" || { bad "Tc3_Mc3Ptp library"; \
         echo "== $pass pass, $fail fail"; exit 1; }
"$TPLCC" build tests/compat/tc3_mc3ptp_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc3_mc3ptp.tplc" \
    -o "$WORK/tc3_mc3ptp_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" \
    || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 60); do
        t=$((i * 10000000))
        if [ "$i" -eq 60 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff010000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ptp.steps"

IMG=/tplc_tcmc3p_$$; CTL="$WORK/ptp.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_mc3ptp_case.tplc" \
    --steps "$WORK/ptp.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: torque/hw-param/stable/no-op/error-inject/reset"
else
    bad "sim replay: torque/hw-param/stable/no-op/error-inject/reset"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
