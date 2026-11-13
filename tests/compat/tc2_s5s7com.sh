#!/usr/bin/env bash
# tc2_s5s7com.sh — Tc2_S5S7Com compat: build
# compat/tc2/tc2_s5s7com.st as a --library package, consume it
# into tests/compat/tc2_s5s7com_case.st via --lib, and replay an
# autonomous RK512/3964 exchange through tether-plc-sim against
# the real tplc-s5 TCP transport on TCP 1103.
# tests/compat/s5_test_peer.py speaks 3964 STX/BCC/ETX framing
# plus the RK512 FETCH/SEND job header and serves DB1 (256B
# pattern (i*7+3)) and DB2 (256B pattern 0x40+(i&0x3F)); unknown
# DBs answer with a nonzero RK512 status byte.
# The program packs 6 checks into %QD0; the last step expects
# 0x3f.
# Runs without --scan: the PLC free-runs at its task period and
# each step samples the next committed scan, so 300 steps span
# several seconds of wall time — enough for the connect plus
# five transactions and the dead-port timeout.
set -u
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

python3 tests/compat/s5_test_peer.py 1103 >"$WORK/peer.log" 2>&1 &
PEER=$!
trap 'kill $PEER 2>/dev/null; rm -rf "$WORK"' EXIT
sleep 0.5

"$TPLCC" build --library --name Tc2_S5S7Com \
    compat/tc2/tc2_s5s7com.st -o "$WORK/tc2_s5s7com.tplc" >/dev/null 2>&1 \
    && ok "Tc2_S5S7Com library" || bad "Tc2_S5S7Com library"

"$TPLCC" build tests/compat/tc2_s5s7com_case.st \
    --lib "$WORK/tc2_s5s7com.tplc" \
    -o "$WORK/tc2_s5s7com_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 299); do
        t=$((i * 10000000))
        if [ "$i" -eq 299 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/s5.steps"

IMG=/tplc_tc2s5_$$; CTL="$WORK/s5.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_s5s7com_case.tplc" \
    --steps "$WORK/s5.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: open/fetch/write/readback/s5err/connerr/close"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
    tail -8 "$WORK/peer.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
