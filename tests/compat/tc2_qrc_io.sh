#!/usr/bin/env bash
# tc2_qrc_io.sh — Tc2_QRC surface e2e: EAN-13/UPC-A check-digit
# validation via FB_QrcValidate.
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

"$TPLCC" build --library --name Tc2_QRC \
    compat/tc2/tc2_qrc.st -o "$WORK/tc2_qrc.tplc" >/dev/null 2>&1 \
    && ok "Tc2_QRC library" || { bad "Tc2_QRC library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_qrc_case.st \
    --lib "$WORK/tc2_qrc.tplc" \
    -o "$WORK/tc2_qrc_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 100); do
        t=$((i * 10000000))
        if [ "$i" -eq 100 ]; then
            echo "step $i time=$t in 0=00 expect 0=07"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/qrc.steps"

IMG=/tplc_tc2qrc_$$; CTL="$WORK/qrc.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_qrc_case.tplc" \
    --steps "$WORK/qrc.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: ean13/upca check digits"
else
    bad "sim replay: ean13/upca check digits"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
