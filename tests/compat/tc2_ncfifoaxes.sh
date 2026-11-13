#!/usr/bin/env bash
# tc2_ncfifoaxes.sh — Tc2_NcFifoAxes e2e: builds
# compat/tc2/tc2_mc2.st + compat/tc2/tc2_ncfifoaxes.st as --library
# packages, consumes them into tests/compat/tc2_ncfifoaxes_case.st
# via --lib, and replays through tether-plc-sim (in-process
# tplc-motion + tplc-fifo). The program creates two simulated
# CiA402 axes, powers them, integrates both into FIFO channel 0,
# checks FiFoGetDimension, streams two LREAL rows through FiFoWrite,
# sets an override, runs FiFoStart to drain, verifies the axes
# tracked the last row, then FiFoOverwrite + rerun + FiFoGroupDisintegrate.
# Expects mask 0x3fff at the final step.
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

"$TPLCC" build --library --name Tc2_NcFifoAxes \
    compat/tc2/tc2_ncfifoaxes.st -o "$WORK/tc2_ncfifoaxes.tplc" \
    >/dev/null 2>&1 \
    && ok "Tc2_NcFifoAxes library" || { bad "Tc2_NcFifoAxes library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_ncfifoaxes_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc2_ncfifoaxes.tplc" \
    -o "$WORK/tc2_ncfifoaxes_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" \
    || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 300); do
        t=$((i * 10000000))
        if [ "$i" -eq 300 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff3f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/fifo.steps"

IMG=/tplc_tcfifo_$$; CTL="$WORK/fifo.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_ncfifoaxes_case.tplc" \
    --steps "$WORK/fifo.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: FIFO integrate/write/start/overwrite/disintegrate"
else
    bad "sim replay: FIFO integrate/write/start/overwrite/disintegrate"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
