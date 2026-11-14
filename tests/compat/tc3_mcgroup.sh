#!/usr/bin/env bash
# tc3_mcgroup.sh — Tc3_McCoordinatedMotion group-admin e2e: builds
# compat/tc3/tc3_mcgroup.st as a --library package, consumes it into
# tests/compat/tc3_mcgroup_case.st via --lib, and replays through
# tether-plc-sim (in-process tplc-mcgroup service).  The program
# exercises add/remove/ungroup, enable/disable/reset, status and
# error readback, override, setpoint source, coordinate transform,
# membership readback, and the empty-group enable error.
# Expects mask 0xfff at the final step.
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

"$TPLCC" build --library --name Tc3_McGroup \
    compat/tc3/tc3_mcgroup.st -o "$WORK/tc3_mcgroup.tplc" >/dev/null 2>&1 \
    && ok "Tc3_McGroup library" || { bad "Tc3_McGroup library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_mcgroup_case.st \
    --lib "$WORK/tc3_mcgroup.tplc" \
    -o "$WORK/tc3_mcgroup_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" \
    || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 60); do
        t=$((i * 10000000))
        if [ "$i" -eq 60 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff0f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mg.steps"

IMG=/tplc_tcmg_$$; CTL="$WORK/mg.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_mcgroup_case.tplc" \
    --steps "$WORK/mg.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: group admin/status/override/source/transform"
else
    bad "sim replay: group admin/status/override/source/transform"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
