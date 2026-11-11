#!/usr/bin/env bash
# tc3_s7comm.sh — Tc3_S7Comm + Tc2_S5S7Com compat (plan 21 C4):
# build compat/tc3/tc3_s7comm.st as a --library package, consume
# it into tests/compat/tc3_s7comm_case.st via --lib, and replay
# an autonomous S7comm exchange through tether-plc-sim against
# the real tplc-s7 RFC1006 transport on TCP 1102.
# tests/compat/s7_test_peer.py speaks TPKT + COTP + S7comm and
# serves DB1 (256B pattern) plus MK area 0 (128B pattern) with
# spec bit semantics; unknown areas return item rc 0x0A.
# The program packs 9 checks into %QD0; the last step expects
# 0x1ff.
# Runs without --scan: the PLC free-runs at its task period and
# each step samples the next committed scan, so 300 steps span
# several seconds of wall time — enough for the connect
# handshake plus six transactions.
set -u
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

python3 tests/compat/s7_test_peer.py 1102 >"$WORK/peer.log" 2>&1 &
PEER=$!
trap 'kill $PEER 2>/dev/null; rm -rf "$WORK"' EXIT
sleep 0.5

"$TPLCC" build --library --name Tc3_S7Comm \
    compat/tc3/tc3_s7comm.st -o "$WORK/tc3_s7comm.tplc" >/dev/null 2>&1 \
    && ok "Tc3_S7Comm library" || bad "Tc3_S7Comm library"

"$TPLCC" build tests/compat/tc3_s7comm_case.st \
    --lib "$WORK/tc3_s7comm.tplc" \
    -o "$WORK/tc3_s7comm_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 299); do
        t=$((i * 10000000))
        if [ "$i" -eq 299 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff010000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/s7.steps"

IMG=/tplc_tc3s7_$$; CTL="$WORK/s7.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_s7comm_case.tplc" \
    --steps "$WORK/s7.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: conn/read/write/rb/bit/s7err/connerr/close"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
    tail -8 "$WORK/peer.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
