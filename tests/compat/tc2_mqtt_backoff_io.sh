#!/usr/bin/env bash
# tc2_mqtt_backoff_io.sh — exponential-backoff reconnect e2e:
# starts the python fake broker, replays tc2_mqtt_backoff_case.st,
# kills the broker mid-run (connection drops), restarts it, and
# expects the host watchdog to reconnect with doubling delay.
# Skips (77) when the toolchain or python3 is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT:-11885}
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

# the fake broker self-bounces after broadcasting the trigger
# publish: it drops every client + the listener, rebinds after 0.5 s,
# and the host watchdog must bridge the gap
python3 "$FAKE" "$PORT" --bounce tplc/bo/a 0.5 >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "broker fake start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc3_Mqtt \
    compat/tc2/tc2_mqtt.st -o "$WORK/tc2_mqtt.tplc" >/dev/null 2>&1 \
    && ok "Tc3_Mqtt library" || { bad "Tc3_Mqtt library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_mqtt_backoff_case.st \
    --lib "$WORK/tc2_mqtt.tplc" \
    -o "$WORK/tc2_mqtt_backoff_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 3000); do
        t=$((i * 10000000))
        if [ "$i" -eq 2990 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff03"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/bo.steps"

IMG=/tplc_tc2mqttbo_$$; CTL="$WORK/bo.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" \
    --package "$WORK/tc2_mqtt_backoff_case.tplc" \
    --steps "$WORK/bo.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: backoff drop -> doubling-delay reconnect -> resub/echo"
else
    bad "sim replay: backoff drop -> doubling-delay reconnect -> resub/echo"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
