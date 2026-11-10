#!/usr/bin/env bash
# tc3_bacnet.sh — Tc3_BACnet compat (plan 21 C4): build
# compat/tc3/tc3_bacnet.st as a --library package, consume it into
# tests/compat/tc3_bacnet_case.st via --lib, and replay an
# autonomous BACnet/IP exchange through tether-plc-sim against
# the real tplc-bacnet UDP transport. tests/compat/
# bacnet_test_peer.py answers broadcast Who-Is (I-Am for
# device(8).1234 inside the requested 1000..2000 range),
# ReadProperty Complex-ACKs (REAL/UNSIGNED/characterString) and
# WriteProperty Simple-ACKs, plus a BACnet Error for the unknown
# object and a directed unicast Who-Is. The program packs 6
# checks into %QD0; the last step expects 0x3f. Runs without
# --scan: the PLC free-runs at its task
# period and each step samples the next committed scan, so 200
# steps span ~4 s of wall time without hitting the TASK_STEP
# mut-rate limit.
# Requires: tplcc, tether-plc, tether-plc-sim, tplc-image, python3
# (skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
WORK=$(mktemp -d)
trap 'kill $PEER_PID 2>/dev/null; rm -rf "$WORK"; '"$BUILD"'/tplc-image unlink /tplc_tc3bacnet_$$ 2>/dev/null' EXIT

[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    && [ -x "$BUILD/tplc-image" ] && command -v python3 >/dev/null \
    || { echo "SKIP: no build tree or python3"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

python3 tests/compat/bacnet_test_peer.py \
    >"$WORK/peer.log" 2>&1 &
PEER_PID=$!
sleep 0.3   # let the peer bind 47808 before the first Who-Is

"$TPLCC" build --library --name Tc3_BACnet \
    compat/tc3/tc3_bacnet.st -o "$WORK/tc3_bacnet.tplc" >/dev/null 2>&1 \
    && ok "Tc3_BACnet library" || bad "Tc3_BACnet library"

"$TPLCC" build tests/compat/tc3_bacnet_case.st \
    --lib "$WORK/tc3_bacnet.tplc" \
    -o "$WORK/tc3_bacnet_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 349); do
        t=$((i * 10000000))
        if [ "$i" -eq 349 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/bacnet.steps"

IMG=/tplc_tc3bacnet_$$; CTL="$WORK/bacnet.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_bacnet_case.tplc" \
    --steps "$WORK/bacnet.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: who-is/read/write/error"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
    tail -5 "$WORK/peer.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
