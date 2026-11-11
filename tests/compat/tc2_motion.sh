#!/usr/bin/env bash
# tc2_motion.sh — tplc-motion / FB_MotionAxis e2e (plan 21 C5):
# build tests/compat/tc2_motion_case.st and replay it through
# tether-plc-sim. The program drives the simulated CiA402 axis
# through create, PP-position move, velocity mode, error paths and
# destroy, packing 12 checks into %QD0; the last step expects
# mask 0xfff. No external peer: tplc-motion is fully in-process
# and deterministic under the sim clock.
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

"$TPLCC" build tests/compat/tc2_motion_case.st \
    -o "$WORK/tc2_motion_case.tplc" >/dev/null 2>&1 \
    && ok "motion case build" || { bad "motion case build"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 499); do
        t=$((i * 10000000))
        if [ "$i" -eq 499 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff0f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/motion.steps"

IMG=/tplc_tc2motion_$$; CTL="$WORK/motion.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_motion_case.tplc" \
    --steps "$WORK/motion.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: create/pp-move/vel-mode/errors/destroy"
else
    bad "sim replay"
    tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
