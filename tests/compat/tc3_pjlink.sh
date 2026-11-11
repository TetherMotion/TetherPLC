#!/usr/bin/env bash
# tc3_pjlink.sh — Tc3_PJLink compat (plan 21 C4): build
# compat/tc3/tc3_pjlink.st as a --library package, consume it into
# tests/compat/tc3_pjlink_case.st via --lib, and replay an
# autonomous PJLink exchange through tether-plc-sim against the
# real tplc-pjlink TCP transport on port 4352.
# tests/compat/pjlink_test_peer.py picks its greeting by
# connection count: `PJLINK 0` for the unauthenticated phases,
# `PJLINK 1 <nonce>` accepting md5(nonce+"secret") for the auth
# phase, and `PJLINK ERRA` for the wrong-password phase. The
# program packs 7 checks into %QD0; the last step expects 0x7f.
# Runs without --scan: the PLC free-runs at its task period and
# each step samples the next committed scan, so 250 steps span
# ~2.5 s of wall time without hitting the TASK_STEP mut-rate
# limit.
# Requires: tplcc, tether-plc, tether-plc-sim, tplc-image, python3
# (skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
WORK=$(mktemp -d)
trap 'kill $PEER_PID 2>/dev/null; rm -rf "$WORK"; '"$BUILD"'/tplc-image unlink /tplc_tc3pjlink_$$ 2>/dev/null' EXIT

[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    && [ -x "$BUILD/tplc-image" ] && command -v python3 >/dev/null \
    || { echo "SKIP: no build tree or python3"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

python3 tests/compat/pjlink_test_peer.py \
    >"$WORK/peer.log" 2>&1 &
PEER_PID=$!
sleep 0.3   # let the peer bind 4352 before the first connect

"$TPLCC" build --library --name Tc3_PJLink \
    compat/tc3/tc3_pjlink.st -o "$WORK/tc3_pjlink.tplc" >/dev/null 2>&1 \
    && ok "Tc3_PJLink library" || bad "Tc3_PJLink library"

"$TPLCC" build tests/compat/tc3_pjlink_case.st \
    --lib "$WORK/tc3_pjlink.tplc" \
    -o "$WORK/tc3_pjlink_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 249); do
        t=$((i * 10000000))
        if [ "$i" -eq 249 ]; then
            echo "step $i time=$t in 0=00 expect 0=7f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/pjlink.steps"

IMG=/tplc_tc3pjlink_$$; CTL="$WORK/pjlink.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_pjlink_case.tplc" \
    --steps "$WORK/pjlink.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: pwr/inpt/avmt/auth/authfail/err1"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
    tail -8 "$WORK/peer.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
