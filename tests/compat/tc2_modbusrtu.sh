#!/usr/bin/env bash
# tc2_modbusrtu.sh — Tc2_ModbusRTU compat (plan 21 C4): build
# compat/tc2/tc2_modbusrtu.st as a --library package, consume it into
# tests/compat/tc2_modbusrtu_case.st via --lib, and replay an
# autonomous Modbus RTU sequence (read regs -> write single reg ->
# write regs -> read coils -> exception read -> read/write regs)
# through tether-plc-sim against the real tplc-modbus RTU transport.
# The peer is tests/compat/modbus_rtu_test_server.py, a minimal
# Modbus RTU slave on a pty pair. The program packs 6 checks into
# %QD0; the last step expects 0x3f.
# Requires: tplcc, tether-plc, tether-plc-sim, python3 (skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
WORK=$(mktemp -d)
trap 'kill $PEER_PID 2>/dev/null; rm -rf "$WORK"; "'$BUILD'"/tplc-image unlink /tplc_tc2rtu_$$ 2>/dev/null' EXIT

[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    && [ -x "$BUILD/tplc-image" ] && command -v python3 >/dev/null \
    || { echo "SKIP: no build tree or python3"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

python3 tests/compat/modbus_rtu_test_server.py "$WORK/pty_slave" \
    >"$WORK/rtu_srv.log" 2>&1 &
PEER_PID=$!
for i in $(seq 1 50); do [ -s "$WORK/pty_slave" ] && break; sleep 0.05; done
SLAVE=$(cat "$WORK/pty_slave" 2>/dev/null)
[ -n "$SLAVE" ] || { bad "rtu test server"; \
    cat "$WORK/rtu_srv.log"; echo "== $pass pass, $fail fail"; exit 1; }

sed "s|__PTY_SLAVE__|$SLAVE|" tests/compat/tc2_modbusrtu_case.st \
    > "$WORK/tc2_modbusrtu_case.st"

"$TPLCC" build --library --name Tc2_ModbusRTU \
    compat/tc2/tc2_modbusrtu.st -o "$WORK/tc2_modbusrtu.tplc" >/dev/null 2>&1 \
    && ok "Tc2_ModbusRTU library" || bad "Tc2_ModbusRTU library"

"$TPLCC" build "$WORK/tc2_modbusrtu_case.st" \
    --lib "$WORK/tc2_modbusrtu.tplc" \
    -o "$WORK/tc2_modbusrtu_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

cat > "$WORK/rtu.steps" <<'EOF'
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
step 13 time=130000000 in 0=00
step 14 time=140000000 in 0=00
step 15 time=150000000 in 0=00
step 16 time=160000000 in 0=00
step 17 time=170000000 in 0=00
step 18 time=180000000 in 0=00
step 19 time=190000000 in 0=00
step 20 time=200000000 in 0=00
step 21 time=210000000 in 0=00
step 22 time=220000000 in 0=00
step 23 time=230000000 in 0=00
step 24 time=240000000 in 0=00
step 25 time=250000000 in 0=00
step 26 time=260000000 in 0=00
step 27 time=270000000 in 0=00
step 28 time=280000000 in 0=00
step 29 time=290000000 in 0=00
step 30 time=300000000 in 0=00
step 31 time=310000000 in 0=00
step 32 time=320000000 in 0=00
step 33 time=330000000 in 0=00
step 34 time=340000000 in 0=00
step 35 time=350000000 in 0=00
step 36 time=360000000 in 0=00
step 37 time=370000000 in 0=00 expect 0=3f000000
EOF

IMG=/tplc_tc2rtu_$$; CTL="$WORK/rtu.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_modbusrtu_case.tplc" \
    --steps "$WORK/rtu.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --scan --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: modbus RTU read/write/exception/read-write"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
