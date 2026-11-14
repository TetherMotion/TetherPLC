#!/usr/bin/env bash
# canopen e2e: tplc-canopen (FB_CanOpen) against tether_io_ec_server
# --simulate --session-role technician.  The server exposes the CiA 402
# machine stack with machine.sdo.* over an in-memory SDO dictionary;
# the PLC performs CiA 301 SDO upload/download by explicit index —
# the same object-dictionary protocol CANopen uses on CAN, carried
# over EtherCAT CoE.
# Skips (77) when tplcc, the sim, or the Tether server is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${EC_SERVER:-$TETHER_DIR/build/bin/tether_io_ec_server}
PORT=${EC_PORT:-14231}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "ec server missing: $SRV" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/server.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build tests/canopen/canopen_sdo_case.st \
    -o "$WORK/canopen_sdo_case.tplc" >/dev/null 2>&1 \
    && ok "case build" || { bad "case build"; \
         echo "== $pass pass, $fail fail"; exit 1; }

# technician role: sdo.read needs authenticated, sdo.write needs
# technician-or-above — one role covers both.
"$SRV" --bind 127.0.0.1 --port "$PORT" --session-role technician \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "ec server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 8100); do
        # 20 ms/step: machine.sdo.write is rate-limited to one write
        # per 50 ms server-side; the two test writes must be >50 ms apart
        t=$((i * 20000000))
        if [ "$i" -eq 8100 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/canopen.steps"

IMG=/tplc_canopen_$$; CTL="$WORK/canopen.ctl"
TPLC_CANOPEN_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/canopen_sdo_case.tplc" \
    --steps "$WORK/canopen.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: sdo read/write/abort over machine.sdo.*"
else
    bad "sim replay"; tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
