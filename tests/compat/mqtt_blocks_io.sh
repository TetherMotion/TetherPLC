#!/usr/bin/env bash
# mqtt_blocks_io.sh — easy FBD block e2e: FB_MqttLink/Tx/Rx/Sub
# loopback over wss:// (fake broker --ws --tls, self-signed).
# Skips (77) when openssl/tplcc/sim are missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT_WSS:-11888}
CERTDIR=${MQTT_TLS_CERTDIR:-/tmp/tether_mqtt_tls}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null 2>&1 || { echo "python3 missing" >&2; exit 77; }
command -v openssl >/dev/null 2>&1 || { echo "openssl missing" >&2; exit 77; }
[ -f "$FAKE" ] || { echo "mqtt_broker_fake missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/broker.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

mkdir -p "$CERTDIR"
if [ ! -f "$CERTDIR/srv.pem" ]; then
    openssl req -x509 -newkey rsa:2048 -keyout "$CERTDIR/ca.key" \
        -out "$CERTDIR/ca.pem" -days 2 -nodes -subj '/CN=TetherTestCA' \
        >/dev/null 2>&1
    openssl req -newkey rsa:2048 -keyout "$CERTDIR/srv.key" \
        -out "$CERTDIR/srv.csr" -nodes -subj '/CN=localhost' \
        >/dev/null 2>&1
    printf 'subjectAltName=DNS:localhost,IP:127.0.0.1\n' \
        > "$CERTDIR/ext.cnf"
    openssl x509 -req -in "$CERTDIR/srv.csr" -CA "$CERTDIR/ca.pem" \
        -CAkey "$CERTDIR/ca.key" -CAcreateserial \
        -out "$CERTDIR/srv.pem" -days 2 -extfile "$CERTDIR/ext.cnf" \
        >/dev/null 2>&1
fi

"$TPLCC" build --library --name MqttBlocks \
    libraries/mqtt/mqtt_blocks.st -o "$WORK/mqtt_blocks.tplc" \
    >/dev/null 2>&1 \
    && ok "MqttBlocks library" \
    || { bad "MqttBlocks library"; \
         "$TPLCC" build --library --name MqttBlocks \
             libraries/mqtt/mqtt_blocks.st -o /dev/null; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name HaBridge \
    libraries/homeassistant/ha_bridge.st -o "$WORK/ha_bridge.tplc" \
    >/dev/null 2>&1 \
    || { bad "HaBridge library"; echo "== $pass pass, $fail fail"; exit 1; }

python3 "$FAKE" "$PORT" --tls "$CERTDIR/srv.pem" "$CERTDIR/srv.key" \
    --ws >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "wss broker start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/mqtt_blocks_case.st \
    --lib "$WORK/mqtt_blocks.tplc" \
    -o "$WORK/blocks.tplc" >/dev/null 2>&1 \
    && ok "mqtt_blocks_case build with --lib" \
    || { bad "mqtt_blocks_case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 500); do
        t=$((i * 10000000))
        if [ "$i" -eq 499 ]; then
            echo "step $i time=$t in 0=00 expect 0=1f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/blocks.steps"

IMG=/tplc_mblocks_$$; CTL="$WORK/blocks.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/blocks.tplc" \
    --steps "$WORK/blocks.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: wss link/sub/pub/rx loopback via easy blocks"
else
    bad "sim replay: wss link/sub/pub/rx loopback via easy blocks"
    tail -20 "$WORK/sim.log"
fi

if grep -q "CONNECT cid='blocks-e2e' user='wsuser' has_pass=True" \
        "$SRV_LOG"; then
    ok "block CONNECT carries username/password/client_id"
else
    bad "block CONNECT carries username/password/client_id"
    grep CONNECT "$SRV_LOG" | tail -5
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
