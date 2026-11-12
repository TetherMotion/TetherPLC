#!/usr/bin/env bash
# tc3_database_io.sh — Tc2_Database e2e: builds
# compat/tc3/tc3_database.st as a --library package and replays
# tc3_database_case.st through tether-plc-sim (SQLite-backed SQL:
# alloc/write/read/realloc-preserve/range-error/dealloc/counters).
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

"$TPLCC" build --library --name Tc2_Database \
    compat/tc3/tc3_database.st -o "$WORK/tc3_db.tplc" >/dev/null 2>&1 \
    && ok "Tc2_Database library" || { bad "Tc2_Database library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_database_case.st \
    --lib "$WORK/tc3_db.tplc" \
    -o "$WORK/db_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 80); do
        t=$((i * 10000000))
        if [ "$i" -eq 80 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff07"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/db.steps"

IMG=/tplc_tc3db_$$; CTL="$WORK/dm.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/db_case.tplc" \
    --steps "$WORK/db.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: open/create/insert/select/error-text/close"
else
    bad "sim replay: open/create/insert/select/error-text/close"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
