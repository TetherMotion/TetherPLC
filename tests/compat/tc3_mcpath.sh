#!/usr/bin/env bash
# tc3_mcpath.sh — Tc3_McCoordinatedMotion path e2e: builds
# compat/tc2/tc2_mc2.st + compat/tc3/tc3_mcgroup.st +
# compat/tc3/tc3_mcpath.st as libraries, consumes them into
# tests/compat/tc3_mcpath_case.st via --lib, and replays through
# tether-plc-sim (in-process tplc-motion, 2 axes). Covers axis
# create/power, group membership, path preparation
# (linear/dwell/linear) and MC_MovePath CSP streaming with final
# position verification. Expects mask 0x3f.
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
"$TPLCC" build --library --name Tc3_McGroup \
    compat/tc3/tc3_mcgroup.st -o "$WORK/tc3_mcgroup.tplc" >/dev/null 2>&1 \
    && ok "Tc3_McGroup library" || { bad "Tc3_McGroup library"; \
         echo "== $pass pass, $fail fail"; exit 1; }
"$TPLCC" build --library --name Tc3_McPath \
    compat/tc3/tc3_mcpath.st -o "$WORK/tc3_mcpath.tplc" >/dev/null 2>&1 \
    && ok "Tc3_McPath library" || { bad "Tc3_McPath library"; \
         echo "== $pass pass, $fail fail"; exit 1; }
"$TPLCC" build tests/compat/tc3_mcpath_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc3_mcgroup.tplc" \
    --lib "$WORK/tc3_mcpath.tplc" \
    -o "$WORK/tc3_mcpath_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" \
    || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 500); do
        t=$((i * 10000000))
        if [ "$i" -eq 400 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mcpath.steps"

IMG=/tplc_tcmp_$$; CTL="$WORK/mcpath.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_mcpath_case.tplc" \
    --steps "$WORK/mcpath.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: coordinated path (2 axes)"
else
    bad "sim replay: coordinated path (2 axes)"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
