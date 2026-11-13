#!/usr/bin/env bash
# tc2_mqtt5_io.sh — MQTT 5.0 client e2e: builds compat/tc2/tc2_mqtt.st
# as a --library package, starts the python fake broker (v5 framing),
# and replays tc2_mqtt5_req_case.st through tether-plc-sim.  Skips (77)
# when python3, the fake, or the toolchain is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT:-11884}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null 2>&1 || { echo "python3 missing" >&2; exit 77; }
[ -f "$FAKE" ] || { echo "mqtt_broker_fake missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/broker.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

python3 "$FAKE" "$PORT" --responder tplc/v5/req >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "broker fake start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc3_Mqtt \
    compat/tc2/tc2_mqtt.st -o "$WORK/tc2_mqtt.tplc" >/dev/null 2>&1 \
    && ok "Tc3_Mqtt library" || { bad "Tc3_Mqtt library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_mqtt5_req_case.st \
    --lib "$WORK/tc2_mqtt.tplc" \
    -o "$WORK/tc2_mqtt5_req_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 800); do
        t=$((i * 10000000))
        if [ "$i" -eq 790 ]; then
            echo "step $i time=$t in 0=00 expect 0=0700"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mqtt5req.steps"

IMG=/tplc_tc2mqtt5req_$$; CTL="$WORK/mqtt5req.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_mqtt5_req_case.tplc" \
    --steps "$WORK/mqtt5req.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log" \
    && grep -q "RESPONDER reply -> tplc/v5/resp/in" "$SRV_LOG"; then
    ok "sim replay: MQTT5 request-response"
else
    bad "sim replay: MQTT5 request-response"
    tail -20 "$WORK/sim.log"; tail -10 "$SRV_LOG"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
