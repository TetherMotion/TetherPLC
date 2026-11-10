#!/usr/bin/env bash
# tc3_sacn.sh — Tc3_sACN compat (plan 21 C4): build
# compat/tc3/tc3_sacn.st as a --library package, consume it into
# tests/compat/tc3_sacn_case.st via --lib, and replay an autonomous
# source+receiver exchange through tether-plc-sim against the real
# tplc-sacn UDP multicast transport. The in-process FB_sACN_Source
# streams universe-1 DMX at priority 100; tests/compat/
# sacn_test_peer.py injects priority-10 'HIPRI' and priority-200
# 'LOWPR' packets on a wall-clock schedule (gated on seeing the
# DUT's own SRC1 stream first) to exercise preemption and drop
# accounting. The program packs 4 checks into %QD0; the last step
# expects 0x0f. Runs without --scan: the PLC free-runs at its task
# period and each step samples the next committed scan, so 600
# steps span several seconds of wall time for the peer schedule
# without hitting the TASK_STEP mut-rate limit.
# Requires: tplcc, tether-plc, tether-plc-sim, tplc-image, python3
# (skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
WORK=$(mktemp -d)
trap 'kill $PEER_PID 2>/dev/null; rm -rf "$WORK"; '"$BUILD"'/tplc-image unlink /tplc_tc3sacn_$$ 2>/dev/null' EXIT

[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    && [ -x "$BUILD/tplc-image" ] && command -v python3 >/dev/null \
    || { echo "SKIP: no build tree or python3"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

python3 tests/compat/sacn_test_peer.py \
    >"$WORK/peer.log" 2>&1 &
PEER_PID=$!

"$TPLCC" build --library --name Tc3_sACN \
    compat/tc3/tc3_sacn.st -o "$WORK/tc3_sacn.tplc" >/dev/null 2>&1 \
    && ok "Tc3_sACN library" || bad "Tc3_sACN library"

"$TPLCC" build tests/compat/tc3_sacn_case.st \
    --lib "$WORK/tc3_sacn.tplc" \
    -o "$WORK/tc3_sacn_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 599); do
        t=$((i * 10000000))
        if [ "$i" -eq 599 ]; then
            echo "step $i time=$t in 0=00 expect 0=0f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/sacn.steps"

IMG=/tplc_tc3sacn_$$; CTL="$WORK/sacn.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_sacn_case.tplc" \
    --steps "$WORK/sacn.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: sacn source/receiver/priority/helpers"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
    tail -5 "$WORK/peer.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
