#!/usr/bin/env bash
# mqtt_journal_io.sh — ST event-journal e2e: FB_MqttEventLog fed by
# FB_MqttRx over a plain tcp:// fake broker with three timed injects.
# Skips (77) when tplcc/sim/python3 are missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT_JRN:-11883}
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

# periodic injects (0.3 s) — sim replay is faster than wall-clock,
# one-shot delays raced the run; repetition guarantees coverage
python3 "$FAKE" "$PORT" \
    --inject 'jrn/a|MSG1|0.2|0.3' \
    --inject 'jrn/b|MSG2|0.4|0.3' \
    --inject 'jrn/c|MSG3|0.6|0.3' \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "broker start"; cat "$SRV_LOG"; \
    echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name MqttBlocks \
    libraries/mqtt/mqtt_blocks.st -o "$WORK/mqtt_blocks.tplc" \
    >/dev/null 2>&1 \
    && ok "MqttBlocks library" \
    || { bad "MqttBlocks library"; \
         "$TPLCC" build --library --name MqttBlocks \
             libraries/mqtt/mqtt_blocks.st -o /dev/null; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name MqttEvents \
    libraries/mqtt/mqtt_events.st -o "$WORK/mqtt_events.tplc" \
    >/dev/null 2>&1 \
    && ok "MqttEvents library" \
    || { bad "MqttEvents library"; \
         "$TPLCC" build --library --name MqttEvents \
             libraries/mqtt/mqtt_events.st -o /dev/null; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/mqtt_journal_case.st \
    --lib "$WORK/mqtt_blocks.tplc" --lib "$WORK/mqtt_events.tplc" \
    -o "$WORK/journal.tplc" >/dev/null 2>&1 \
    && ok "mqtt_journal_case build" \
    || { bad "mqtt_journal_case build"; \
         "$TPLCC" build tests/compat/mqtt_journal_case.st \
             --lib "$WORK/mqtt_blocks.tplc" \
             --lib "$WORK/mqtt_events.tplc" -o /dev/null; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 1200); do
        t=$((i * 10000000))
        if [ "$i" -eq 1199 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/journal.steps"

IMG=/tplc_mjrn_$$; CTL="$WORK/journal.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/journal.tplc" \
    --steps "$WORK/journal.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: link/sub/rx + FB_MqttEventLog ring (count, order, head)"
else
    bad "sim replay: link/sub/rx + FB_MqttEventLog ring (count, order, head)"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
