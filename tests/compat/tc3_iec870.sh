#!/usr/bin/env bash
# tc3_iec870.sh — Tc2_IEC60870_5_10x compat (plan 21 C4):
# build compat/tc3/tc3_iec870.st as a --library package, consume
# it into tests/compat/tc3_iec870_case.st via --lib, and replay
# an autonomous IEC 60870-5-104 exchange through tether-plc-sim
# against the real tplc-iec870 TCP transport on port 2480.
# tests/compat/iec870_test_peer.py is a 104 slave: STARTDT
# handshake, C_IC_NA_1 -> M_SP_NA_1+M_ME_NA_1, C_CS_NA_1 echo,
# C_SC_NA_1 act-con. The program packs 12 checks into %QD0;
# the last step expects mask 0xfff.
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

python3 tests/compat/iec870_test_peer.py 2480 >"$WORK/peer.log" 2>&1 &
PEER=$!
trap 'kill $PEER 2>/dev/null; rm -rf "$WORK"' EXIT
sleep 0.5

"$TPLCC" build --library --name Tc3_Iec870 \
    compat/tc3/tc3_iec870.st -o "$WORK/tc3_iec870.tplc" >/dev/null 2>&1 \
    && ok "Tc3_Iec870 library" || bad "Tc3_Iec870 library"

"$TPLCC" build tests/compat/tc3_iec870_case.st \
    --lib "$WORK/tc3_iec870.tplc" \
    -o "$WORK/tc3_iec870_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 399); do
        t=$((i * 10000000))
        if [ "$i" -eq 399 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff0f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/iec870.steps"

IMG=/tplc_tc3iec870_$$; CTL="$WORK/iec870.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_iec870_case.tplc" \
    --steps "$WORK/iec870.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: conn/interrog/recv/csync/cmd/connerr/helpers/close"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
    tail -15 "$WORK/peer.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
