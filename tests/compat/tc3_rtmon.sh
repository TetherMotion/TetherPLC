#!/usr/bin/env bash
# tc3_rtmon.sh — Tc3_RealtimeMonitoring compat (plan 21 C3): build
# compat/tc3/tc3_rtmon.st as a --library package, consume it into
# tests/compat/tc3_rtmon_case.st via --lib, and replay an autonomous
# mark sequence (base/ex/simple marks -> count -> indexed reads ->
# out-of-range -> clear -> recount) through tether-plc-sim against
# the real tplc-rtmon service. No external peer: the service is
# in-process. The program packs 10 checks into %QD0; the last step
# expects 0x3ff.
# Requires: tplcc, tether-plc, tether-plc-sim, tplc-image (skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"; '"$BUILD"'/tplc-image unlink /tplc_tc3rtm_$$ 2>/dev/null' EXIT

[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    && [ -x "$BUILD/tplc-image" ] \
    || { echo "SKIP: no build tree"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

"$TPLCC" build --library --name Tc3_RealtimeMonitoring \
    compat/tc3/tc3_rtmon.st -o "$WORK/tc3_rtmon.tplc" >/dev/null 2>&1 \
    && ok "Tc3_RealtimeMonitoring library" || bad "Tc3_RealtimeMonitoring library"

"$TPLCC" build tests/compat/tc3_rtmon_case.st \
    --lib "$WORK/tc3_rtmon.tplc" \
    -o "$WORK/tc3_rtmon_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

cat > "$WORK/rtm.steps" <<'EOF'
step 0 time=0        in 0=00
step 1 time=10000000 in 0=00
step 2 time=20000000 in 0=00
step 3 time=30000000 in 0=00
step 4 time=40000000 in 0=00
step 5 time=50000000 in 0=00
step 6 time=60000000 in 0=00
step 7 time=70000000 in 0=00
step 8 time=80000000 in 0=00
step 9 time=90000000 in 0=00
step 10 time=100000000 in 0=00
step 11 time=110000000 in 0=00
step 12 time=120000000 in 0=00
step 13 time=130000000 in 0=00 expect 0=ff030000
EOF

IMG=/tplc_tc3rtm_$$; CTL="$WORK/rtm.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_rtmon_case.tplc" \
    --steps "$WORK/rtm.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --scan --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: rtmon mark/count/get/clear sequence"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
