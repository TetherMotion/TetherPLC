#!/usr/bin/env bash
# mqtt_journal_ctl.sh — ctl-level e2e for the MQTT journal ops:
# realtime tether-plc + fake broker (periodic injects) + a raw ctl
# client asserting MQTT_STATUS rows and MQTT_JOURNAL entries.
# Skips (77) when binaries/python3 are missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT_JCT:-11996}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc" ] || { echo "tether-plc missing" >&2; exit 77; }
[ -x "$BUILD/tplc-image" ] || { echo "tplc-image missing" >&2; exit 77; }
command -v python3 >/dev/null 2>&1 || { echo "python3 missing" >&2; exit 77; }
[ -f "$FAKE" ] || { echo "mqtt_broker_fake missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/broker.log"
PLC_LOG="$WORK/plc.log"
IMG=/tplc_jctl_$$
CTL="$WORK/plc.ctl"
trap 'kill ${SRV_PID:-0} ${PLC_PID:-0} 2>/dev/null; "$BUILD/tplc-image" unlink "$IMG" >/dev/null 2>&1; rm -rf "$WORK"' EXIT

python3 "$FAKE" "$PORT" \
    --inject 'jrn/a|MSG1|0.3|0.4' \
    --inject 'jrn/b|MSG2|0.5|0.4' \
    --inject 'jrn/c|MSG3|0.7|0.4' \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.6
kill -0 "$SRV_PID" 2>/dev/null || { bad "broker start"; cat "$SRV_LOG"; \
    echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name MqttBlocks \
    libraries/mqtt/mqtt_blocks.st -o "$WORK/mqtt_blocks.tplc" \
    >/dev/null 2>&1 \
    || { bad "MqttBlocks library"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name MqttEvents \
    libraries/mqtt/mqtt_events.st -o "$WORK/mqtt_events.tplc" \
    >/dev/null 2>&1 \
    || { bad "MqttEvents library"; echo "== $pass pass, $fail fail"; exit 1; }

sed 's/11883/'"$PORT"'/' tests/compat/mqtt_journal_case.st \
    > "$WORK/journal_case.st"
"$TPLCC" build "$WORK/journal_case.st" \
    --lib "$WORK/mqtt_blocks.tplc" --lib "$WORK/mqtt_events.tplc" \
    -o "$WORK/journal.tplc" >/dev/null 2>&1 \
    && ok "journal package build" \
    || { bad "journal package build"; \
         "$TPLCC" build "$WORK/journal_case.st" \
             --lib "$WORK/mqtt_blocks.tplc" \
             --lib "$WORK/mqtt_events.tplc" -o /dev/null; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$BUILD/tplc-image" create "$IMG" 64 64 >/dev/null 2>&1 \
    || { bad "tplc-image create"; echo "== $pass pass, $fail fail"; exit 1; }

"$BUILD/tether-plc" --image "$IMG" --package "$WORK/journal.tplc" \
    --ctl "$CTL" >"$PLC_LOG" 2>&1 &
PLC_PID=$!
# wait for connect + a few inject rounds, then query
sleep 5
kill -0 $PLC_PID 2>/dev/null || { bad "tether-plc start"; cat "$PLC_LOG"; \
    echo "== $pass pass, $fail fail"; exit 1; }

if python3 tests/compat/ctl_mqtt_journal.py "$CTL"; then
    ok "ctl MQTT_STATUS + MQTT_JOURNAL report broker + entries"
else
    bad "ctl MQTT_STATUS + MQTT_JOURNAL report broker + entries"
    tail -15 "$PLC_LOG"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
