#!/usr/bin/env bash
# tc2_dataexchange_io.sh — Tc2_DataExchange e2e: builds
# compat/tc2/tc2_dataexchange.st as a --library package and
# replays tc2_dataexchange_case.st.  The case addresses its own
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

"$TPLCC" build --library --name Tc2_DataExchange \
    compat/tc2/tc2_dataexchange.st -o "$WORK/tc2_de.tplc" >/dev/null 2>&1 \
    && ok "Tc2_DataExchange library" || { bad "Tc2_DataExchange library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_dataexchange_case.st \
    --lib "$WORK/tc2_de.tplc" \
    -o "$WORK/de_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 90); do
        t=$((i * 10000000))
        if [ "$i" -eq 90 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff01"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/de.steps"

IMG=/tplc_tc2de_$$; CTL="$WORK/de.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/de_case.tplc" \
    --steps "$WORK/de.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: read/write by name, typed, delta, watchdog, unknown-name"
else
    bad "sim replay: read/write by name, typed, delta, watchdog, unknown-name"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
