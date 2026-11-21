#!/usr/bin/env bash
# ha_entity_io.sh — FB_HaEntity e2e: single switch entity on the
# fake MQTT broker; injected HA 'ON' command -> Cmd + AutoWr latch.
# Skips (77) when tplcc/sim/python are missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT:-11889}
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

# inject gated on the entity's command subscription appearing
python3 "$FAKE" "$PORT" \
    --inject 'tether/ent1/pump/set|ON|0.4' \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "broker fake start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name HaBridge \
    libraries/homeassistant/ha_bridge.st -o "$WORK/ha_bridge.tplc" \
    >/dev/null 2>&1 \
    && ok "HaBridge library" \
    || { bad "HaBridge library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name HaEntity \
    libraries/homeassistant/ha_entity.st \
    --lib "$WORK/ha_bridge.tplc" -o "$WORK/ha_entity.tplc" \
    >/dev/null 2>&1 \
    && ok "HaEntity library" \
    || { bad "HaEntity library"; \
         "$TPLCC" build --library --name HaEntity \
             libraries/homeassistant/ha_entity.st \
             --lib "$WORK/ha_bridge.tplc" -o /dev/null; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/homeassistant/ha_entity_case.st \
    --lib "$WORK/ha_entity.tplc" --lib "$WORK/ha_bridge.tplc" \
    -o "$WORK/ha_entity_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 800); do
        t=$((i * 10000000))
        if [ "$i" -eq 799 ]; then
            echo "step $i time=$t in 0=00 expect 0=0f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ha.steps"

IMG=/tplc_haent_$$; CTL="$WORK/ha.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" \
    --package "$WORK/ha_entity_case.tplc" \
    --steps "$WORK/ha.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: HA entity connect/discovery/cmd/AutoWr e2e"
else
    bad "sim replay: HA entity connect/discovery/cmd/AutoWr e2e"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
