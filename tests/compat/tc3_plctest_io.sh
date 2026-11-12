#!/usr/bin/env bash
# tc3_plctest_io.sh — Tc3_PlcTestFramework e2e: builds
# compat/tc3/tc3_plctest.st as a --library package and
# replays tc3_plctest_case.st.  The case addresses its own
# variables by name through the package's exported var table
# (tplc-sym): read/write by name, typed writers, delta writers,
# watchdog, unknown-name error.
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

"$TPLCC" build --library --name Tc3_PlcTestFramework \
    compat/tc3/tc3_plctest.st -o "$WORK/tc3_pt.tplc" >/dev/null 2>&1 \
    && ok "Tc3_PlcTestFramework library" || { bad "Tc3_PlcTestFramework library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_plctest_case.st \
    --lib "$WORK/tc3_pt.tplc" \
    -o "$WORK/pt_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 90); do
        t=$((i * 10000000))
        if [ "$i" -eq 90 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/pt.steps"

IMG=/tplc_tc3pt_$$; CTL="$WORK/de.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/pt_case.tplc" \
    --steps "$WORK/pt.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: assertions/step-control/prep/suite"
else
    bad "sim replay: assertions/step-control/prep/suite"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
