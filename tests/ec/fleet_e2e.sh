#!/usr/bin/env bash
# EtherCAT fleet e2e: TetherPLC as EtherCAT master against the emulated
# Beckhoff chain (Tether beckhoff_slave_chain over the UDP virtual
# interface).  Validates stimuli on both Tether IO endpoints:
#
#   slave params (fleet.<key>.di/.ai) -> TxPDO -> master -> %I -> PLC
#   -> %Q -> RxPDO -> slave signals (fleet.<key>.do/.ao)
#
# Processes: beckhoff_slave_chain (Tether), tplc-ec-pump (master+pump
# +IO), tether-plc-bridge + tether-plc (PLC).  Skips (77) when the
# sibling Tether checkout/build, tplcc, or build tree is missing.
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
TETHER_DIR=${TETHER_DIR:-../Tether}
TETHER_BIN=${TETHER_BIN:-$TETHER_DIR/build/bin/beckhoff_slave_chain}
IMG=/tplc_ec_fleet_e2e
SHM=/dev/shm$IMG
WORK=$(mktemp -d)
CTL=$WORK/ctl.sock
SIM=$WORK/sim
SLAVE_PID= PUMP_PID= BRIDGE_PID= PLC_PID=

cleanup() {
    [ -n "$BRIDGE_PID" ] && kill "$BRIDGE_PID" 2>/dev/null
    [ -n "$PUMP_PID" ]   && kill "$PUMP_PID"   2>/dev/null
    [ -n "$SLAVE_PID" ]  && kill "$SLAVE_PID"  2>/dev/null
    [ -n "$PLC_PID" ]    && kill "$PLC_PID"    2>/dev/null
    sleep 0.3
    pkill -f "tplc-ec-pump.*$SIM" 2>/dev/null
    pkill -f "beckhoff_slave_chain.*--port $UDP_PORT" 2>/dev/null
    "$BUILD"/tplc-image unlink "$IMG" 2>/dev/null
    rm -rf "$WORK"
}
trap cleanup EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

# ---- prerequisites -------------------------------------------------------
[ -x "$TETHER_BIN" ] || { echo "SKIP: no $TETHER_BIN (build Tether)"; exit 77; }
[ -x "$BUILD/tplc-ec-pump" ] || { echo "SKIP: no tplc-ec-pump"; exit 77; }
[ -x "$BUILD/tplc-io-cli" ]   || { echo "SKIP: no tplc-io-cli"; exit 77; }
[ -x "$BUILD/tether-plc-bridge" ] || { echo "SKIP: no build tree"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP: no python3"; exit 77; }

# tplcc needs an LLVM-21 toolchain — locate one if not already on PATH
if [ -z "${LLVM_SYS_211_PREFIX:-}" ]; then
    for p in /home/uli/llvm21/usr /opt/llvm21 /usr/lib/llvm-21; do
        if [ -x "$p/bin/llvm-config-21" ] || [ -x "$p/bin/llvm-config" ]; then
            export LLVM_SYS_211_PREFIX=$p
            export PATH="$p/bin:$PATH"
            break
        fi
    done
fi
(cd compiler && cargo build -q -p tplcc 2>/dev/null) || true
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }

# ---- ports ---------------------------------------------------------------
read -r UDP_PORT SLAVE_IO PUMP_IO <<<"$(python3 - <<'PY'
import socket
ports = []
for _ in range(3):
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.bind(("127.0.0.1", 0))
    ports.append(s.getsockname()[1])
    s.close()
print(*ports)
PY
)"

# ---- PLC program + map ----------------------------------------------------
# TetherPLC AT addresses are byte offsets (%I<byte>.<bit>).
cat > $WORK/ecfleet.st <<'EOF'
PROGRAM EcFleet
VAR
    di_c1   AT %IB0   : BYTE;
    di_c2   AT %IB1   : BYTE;
    ai_c1_0 AT %IW2   : INT;
    ai_c1_1 AT %IW4   : INT;
    ai_c1_2 AT %IW6   : INT;
    ai_c1_3 AT %IW8   : INT;
    ai_c2_0 AT %IW10  : INT;
    ai_c2_1 AT %IW12  : INT;
    ai_c2_2 AT %IW14  : INT;
    ai_c2_3 AT %IW16  : INT;
    do_c1   AT %QB0   : BYTE;
    do_c2   AT %QB1   : BYTE;
    ao_c1_0 AT %QW2   : INT;
    ao_c1_1 AT %QW4   : INT;
    ao_c1_2 AT %QW6   : INT;
    ao_c1_3 AT %QW8   : INT;
    ao_c2_0 AT %QW10  : INT;
    ao_c2_1 AT %QW12  : INT;
    ao_c2_2 AT %QW14  : INT;
    ao_c2_3 AT %QW16  : INT;
END_VAR
do_c1 := di_c2;
do_c2 := di_c1;
ao_c1_0 := ai_c1_0 * 2;
ao_c1_1 := ai_c1_1 + 100;
ao_c1_2 := ai_c1_2 - ai_c1_3;
ao_c1_3 := 0 - ai_c1_0;
ao_c2_0 := ai_c2_0;
ao_c2_1 := ai_c2_1 + ai_c2_2;
ao_c2_2 := ai_c2_3 * 3;
ao_c2_3 := 7;
END_PROGRAM
EOF
cat > $WORK/map.txt <<'EOF'
# fleet TxPDO (.tx) -> %I, %Q -> fleet RxPDO (.rx)
image in 18 out 18
in  off=0  size=1 key=el1004.c1.tx
in  off=1  size=1 key=el1004.c2.tx
in  off=2  size=8 key=el3104.c1.tx
in  off=10 size=8 key=el3104.c2.tx
out off=0  size=1 key=el2004.c1.rx safe=zero
out off=1  size=1 key=el2004.c2.rx safe=zero
out off=2  size=8 key=el4104.c1.rx safe=zero
out off=10 size=8 key=el4104.c2.rx safe=zero
EOF

$TPLCC build $WORK/ecfleet.st -o $WORK/ecfleet.tplc >/dev/null \
    || { echo "tplcc build failed"; exit 1; }

# ---- IO helpers ------------------------------------------------------------
IOCLI=$BUILD/tplc-io-cli
io_get() { $IOCLI "$1" get "$2" 2>/dev/null; }
io_set() { $IOCLI "$1" set "$2" "$3" >/dev/null 2>&1; }

# wait_io <io-port> <name> <expected> <timeout_s> — poll until match
wait_io() {
    local port=$1 name=$2 want=$3 tmo=${4:-15} i got
    for ((i = 0; i < tmo * 10; i++)); do
        got=$(io_get "$port" "$name")
        [ "$got" = "$want" ] && return 0
        sleep 0.1
    done
    echo "  timeout: $name = '${got:-?}' want $want"
    return 1
}

# ---- start the fleet -------------------------------------------------------
"$TETHER_BIN" --port "$UDP_PORT" --io-port "$SLAVE_IO" --hz 1000 \
    >$WORK/slave.log 2>&1 &
SLAVE_PID=$!

"$BUILD/tplc-ec-pump" --peer 127.0.0.1 --port "$UDP_PORT" \
    --simdir "$SIM" --io-port "$PUMP_IO" --hz 500 \
    >$WORK/pump.log 2>&1 &
PUMP_PID=$!

# pump brings the bus up (discovery -> PREOP -> SAFEOP -> OP)
if ! wait_io "$PUMP_IO" link 1 30; then
    echo "FAIL: pump never reached OP"; tail -20 $WORK/pump.log; exit 1
fi
ok "fleet bring-up: 10 slaves in OP"

# ---- start the PLC ---------------------------------------------------------
"$BUILD/tether-plc-bridge" --image "$IMG" --map $WORK/map.txt \
    --simdir "$SIM" --hz 500 \
    --plc "$BUILD/tether-plc" --image "$IMG" \
         --package $WORK/ecfleet.tplc --ctl "$CTL" \
    >$WORK/bridge.log 2>&1 &
BRIDGE_PID=$!

# ---- stimulus round 1 ------------------------------------------------------
# slave-side inputs (Tether IO params on the slave instance)
io_set "$SLAVE_IO" fleet.el1004.c1.di0 165 || true
io_set "$SLAVE_IO" fleet.el1004.c2.di0 60  || true
io_set "$SLAVE_IO" fleet.el3104.c1.ai0 100 || true
io_set "$SLAVE_IO" fleet.el3104.c1.ai1 50  || true
io_set "$SLAVE_IO" fleet.el3104.c1.ai2 10  || true
io_set "$SLAVE_IO" fleet.el3104.c1.ai3 4   || true
io_set "$SLAVE_IO" fleet.el3104.c2.ai0 5   || true
io_set "$SLAVE_IO" fleet.el3104.c2.ai1 7   || true
io_set "$SLAVE_IO" fleet.el3104.c2.ai2 8   || true
io_set "$SLAVE_IO" fleet.el3104.c2.ai3 9   || true

# TetherPLC-side check: stimuli must be visible in the master's TxPDO
# view (proves the EtherCAT leg into the PLC's process image)
wait_io "$PUMP_IO" el1004.c1.di0 165 15 && ok "DI c1.0 -> master TxPDO" \
    || bad "DI c1.0 -> master TxPDO"
wait_io "$PUMP_IO" el3104.c1.ai0 100 15 && ok "AI c1.0 -> master TxPDO" \
    || bad "AI c1.0 -> master TxPDO"

# full chain: slave IO -> EtherCAT -> PLC -> EtherCAT -> slave IO
check_out() { # <slave-io-name> <expected>
    if wait_io "$SLAVE_IO" "$1" "$2" 20; then ok "$1 == $2"; else bad "$1 == $2"; fi
}
check_out el2004.c1.do0 60    # do_c1 := di_c2
check_out el2004.c2.do0 165   # do_c2 := di_c1
check_out el4104.c1.ao0 200   # ai0*2
check_out el4104.c1.ao1 150   # ai1+100
check_out el4104.c1.ao2 6     # ai2-ai3
check_out el4104.c1.ao3 -100  # -ai0
check_out el4104.c2.ao0 5     # ai0
check_out el4104.c2.ao1 15    # ai1+ai2
check_out el4104.c2.ao2 27    # ai3*3
check_out el4104.c2.ao3 7     # const

# master-side mirror: PLC outputs must sit in the master's RxPDO view
check_mout() {
    if wait_io "$PUMP_IO" "$1" "$2" 10; then ok "master $1 == $2"; else bad "master $1 == $2"; fi
}
check_mout el2004.c1.do0 60
check_mout el4104.c1.ao0 200
check_mout el4104.c2.ao3 7

# ---- stimulus round 2 (reactivity) ----------------------------------------
io_set "$SLAVE_IO" fleet.el1004.c1.di0 0x3c || true   # 60
io_set "$SLAVE_IO" fleet.el1004.c2.di0 0xa5 || true   # 165
io_set "$SLAVE_IO" fleet.el3104.c1.ai0 -50  || true
check_out el2004.c1.do0 165   # swapped
check_out el2004.c2.do0 60
check_out el4104.c1.ao0 -100  # ai0*2
check_out el4104.c1.ao3 50    # -ai0

echo "----"
echo "ec_fleet_e2e: $pass passed, $fail failed"
[ "$fail" -eq 0 ]
