#!/usr/bin/env bash
# tc2_pfwlib_io.sh — PfwLib_Processing TcTvA alarm-queue e2e:
# builds compat/tc2/tc2_pfwlib.st as a --library package and replays
# tc2_pfwlib_case.st through tether-plc-sim.  Skips (77) when the
# toolchain is missing.
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

"$TPLCC" build --library --name Tc3_PfwLib \
    compat/tc2/tc2_pfwlib.st -o "$WORK/tc3_pfwlib.tplc" >/dev/null 2>&1 \
    && ok "Tc3_PfwLib library" || { bad "Tc3_PfwLib library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_pfwlib_case.st \
    --lib "$WORK/tc3_pfwlib.tplc" \
    -o "$WORK/tc2_pfwlib_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 60); do
        t=$((i * 10000000))
        if [ "$i" -eq 60 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/pfwlib.steps"

IMG=/tplc_tc2pfw_$$; CTL="$WORK/pfwlib.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_pfwlib_case.tplc" \
    --steps "$WORK/pfwlib.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: alarm append/query/deactivate/gc/clearsignal/clearpending/updatetime"
else
    bad "sim replay: alarm queue"; tail -5 "$WORK/sim.log"
fi
echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
