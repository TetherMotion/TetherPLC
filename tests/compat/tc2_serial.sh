#!/usr/bin/env bash
# tc2_serial.sh — Tc2_SerialCom compat (plan 21 C2): build
# compat/tc2/tc2_serialcom.st as a --library package, consume it into
# a generated case program via --lib, and replay an open -> send ->
# receive -> flush -> rs485 -> close sequence through
# tether-plc-sim against the real tplc-serial service. The serial
# endpoint is a POSIX pty pair; a python peer echoes master-side
# bytes so the PLC reads back what it wrote. The case packs 8
# checks into %QD0; the last step expects 0xff.
# Requires: tplcc, tether-plc, tether-plc-sim, python3 (skip 77).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
WORK=$(mktemp -d)
trap 'kill $PEER_PID 2>/dev/null; rm -rf "$WORK"; "'$BUILD'"/tplc-image unlink /tplc_tc2ser_$$ 2>/dev/null' EXIT

[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    && [ -x "$BUILD/tplc-image" ] && command -v python3 >/dev/null \
    || { echo "SKIP: no build tree or python3"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

# pty pair + echo peer: writes the slave path, then echoes every
# master-side byte back so the PLC's receive leg gets its own data.
cat > "$WORK/pty_peer.py" <<'EOF'
import os, pty, select, sys, time
m, s = pty.openpty()
slave = os.ttyname(s)
with open(sys.argv[1], "w") as f:
    f.write(slave)
os.set_blocking(m, False)
while True:
    r, _, _ = select.select([m], [], [], 0.05)
    if r:
        try:
            data = os.read(m, 4096)
        except OSError:
            break
        if not data:
            break
        os.write(m, data)
EOF
python3 "$WORK/pty_peer.py" "$WORK/pty_slave" &
PEER_PID=$!
for i in $(seq 1 50); do [ -s "$WORK/pty_slave" ] && break; sleep 0.05; done
SLAVE=$(cat "$WORK/pty_slave" 2>/dev/null)
[ -n "$SLAVE" ] || { bad "pty peer"; echo "== $pass pass, $fail fail"; exit 1; }

sed "s|__PTY_SLAVE__|$SLAVE|" tests/compat/tc2_serial_case.st \
    > "$WORK/tc2_serial_case.st"

"$TPLCC" build --library --name Tc2_SerialCom \
    compat/tc2/tc2_serialcom.st -o "$WORK/tc2_serialcom.tplc" >/dev/null 2>&1 \
    && ok "Tc2_SerialCom library" || bad "Tc2_SerialCom library"

"$TPLCC" build "$WORK/tc2_serial_case.st" \
    --lib "$WORK/tc2_serialcom.tplc" \
    -o "$WORK/tc2_serial_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

cat > "$WORK/serial.steps" <<'EOF'
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
step 36 time=360000000 in 0=00 expect 0=ff000000
EOF

IMG=/tplc_tc2ser_$$; CTL="$WORK/serial.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_serial_case.tplc" \
    --steps "$WORK/serial.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --scan --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: pty open/send/recv/flush/rs485/close + bad-open"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
