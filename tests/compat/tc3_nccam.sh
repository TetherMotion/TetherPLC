#!/usr/bin/env bash
# tc3_nccam.sh — Tc3_NC_Camming e2e: build compat/tc3/tc3_nccam.st
# as a --library package, consume it into
# tests/compat/tc3_nccam_case.st via --lib, and replay through
# tether-plc-sim. The program creates two simulated CiA402 axes,
# loads a 3-point cam table (master -> 2x slave), couples them
# while the master runs a velocity move, verifies interpolation
# and periodic wrap, clears the table, and exercises WritePt.
# 6 checks packed into %QD0; the last step expects mask 0x3f.
# In-process tplc-motion, no external peer.
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

"$TPLCC" build --library --name Tc3_NcCam \
    compat/tc3/tc3_nccam.st -o "$WORK/tc3_nccam.tplc" >/dev/null 2>&1 \
    && ok "Tc3_NcCam library" || bad "Tc3_NcCam library"

"$TPLCC" build tests/compat/tc3_nccam_case.st \
    --lib "$WORK/tc3_nccam.tplc" \
    -o "$WORK/tc3_nccam_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 599); do
        t=$((i * 10000000))
        if [ "$i" -eq 599 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/cam.steps"

IMG=/tplc_tc3nccam_$$; CTL="$WORK/cam.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_nccam_case.tplc" \
    --steps "$WORK/cam.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: cam table/interp/wrap/clear/writept"
else
    bad "sim replay"
    tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
