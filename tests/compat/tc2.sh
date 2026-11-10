#!/usr/bin/env bash
# tc2.sh — TwinCAT T0 compat libraries (plan 21 C1): build the
# compat/tc2/*.st sources as --library packages, consume them into
# tests/compat/tc2_case.st via --lib, and replay a 4-step tape
# through tether-plc-sim. The program packs 21 pass/fail checks
# into %QD0; the final steps expect 0x001FFFFF.
# Requires: tplcc, tether-plc, tether-plc-sim (skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"; "'$BUILD'"/tplc-image unlink /tplc_tc2_$$ 2>/dev/null' EXIT

[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    || { echo "SKIP: no build tree"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

"$TPLCC" build --library --name Tc2_Math \
    compat/tc2/tc2_math.st -o "$WORK/tc2_math.tplc" >/dev/null 2>&1 \
    && ok "Tc2_Math library" || bad "Tc2_Math library"
"$TPLCC" build --library --name Tc2_Utilities_BCD \
    compat/tc2/tc2_util_bcd.st -o "$WORK/tc2_bcd.tplc" >/dev/null 2>&1 \
    && ok "Tc2_Utilities_BCD library" || bad "Tc2_Utilities_BCD library"
"$TPLCC" build --library --name Tc2_Standard_Wide \
    compat/tc2/tc2_standard_wide.st -o "$WORK/tc2_wide.tplc" >/dev/null 2>&1 \
    && ok "Tc2_Standard_Wide library" || bad "Tc2_Standard_Wide library"

"$TPLCC" build tests/compat/tc2_case.st \
    --lib "$WORK/tc2_math.tplc" --lib "$WORK/tc2_bcd.tplc" \
    --lib "$WORK/tc2_wide.tplc" \
    -o "$WORK/tc2_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib x3" \
    || { bad "consumer build with --lib x3"; \
         echo "== $pass pass, $fail fail"; exit 1; }

cat > "$WORK/tc2.steps" <<'EOF'
step 0 time=0        in 0=00
step 1 time=10000000 in 0=01
step 2 time=20000000 in 0=01 expect 0=ffff1f00
step 3 time=30000000 in 0=01 expect 0=ffff1f00
EOF

IMG=/tplc_tc2_$$; CTL="$WORK/tc2.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_case.tplc" \
    --steps "$WORK/tc2.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --scan --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: 21 checks (math + W* + BCD)"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
