#!/usr/bin/env bash
# tc3_ba_common_io.sh — Tc3_BA_Common/Tc3_BA2_Common pure-ST e2e:
# builds compat/tc3/tc3_ba_common.st as a --library package and
# replays tc3_ba_common_case.st through tether-plc-sim.
# NOTE: `expect <off>=<hex>` compares raw image bytes — a UDINT mask
# at %QD0 is stored little-endian, so 0x7fff is written "ff7f".
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

"$TPLCC" build --library --name Tc3_BA_Common \
    compat/tc3/tc3_ba_common.st -o "$WORK/tc3_ba.tplc" >/dev/null 2>&1 \
    && ok "Tc3_BA_Common library" || { bad "Tc3_BA_Common library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_ba_common_case.st \
    --lib "$WORK/tc3_ba.tplc" \
    -o "$WORK/bac_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 120); do
        t=$((i * 10000000))
        if [ "$i" -eq 120 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff7f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ba.steps"

IMG=/tplc_tc3ba_$$; CTL="$WORK/ba.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/bac_case.tplc" \
    --steps "$WORK/ba.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: time/cov/ramplmt/swi2p/swihys/pid/seq/dynarray/rftrig"
else
    bad "sim replay: time/cov/ramplmt/swi2p/swihys/pid/seq/dynarray/rftrig"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
