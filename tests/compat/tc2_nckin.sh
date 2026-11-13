#!/usr/bin/env bash
# tc2_nckin.sh — Tc2_NcKinematicTransformation e2e: builds
# compat/tc2/tc2_nckin.st as a --library package, consumes it into
# tests/compat/tc2_nckin_case.st via --lib, and replays through
# tether-plc-sim (in-process tplc-kin Tether kinematics engine).
# The program configures a planar2d and a serial6 UR5 object, binds
# ACS/MCS axes + channel, runs forward and IK transforms (incl. a
# serial6 IK roundtrip), exercises the parameter lock, rotation
# save/restore, group reset, status readback, and error paths.
# Expects mask 0x1fff at the final step.
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

"$TPLCC" build --library --name Tc2_NcKin \
    compat/tc2/tc2_nckin.st -o "$WORK/tc2_nckin.tplc" >/dev/null 2>&1 \
    && ok "Tc2_NcKin library" || { bad "Tc2_NcKin library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_nckin_case.st \
    --lib "$WORK/tc2_nckin.tplc" \
    -o "$WORK/tc2_nckin_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" \
    || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 200); do
        t=$((i * 10000000))
        if [ "$i" -eq 200 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff1f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/kin.steps"

IMG=/tplc_tckin_$$; CTL="$WORK/kin.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_nckin_case.tplc" \
    --steps "$WORK/kin.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: configure/group/fwd/ik/lock/rotation/reset"
else
    bad "sim replay: configure/group/fwd/ik/lock/rotation/reset"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
