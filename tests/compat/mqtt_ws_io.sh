#!/usr/bin/env bash
# mqtt_ws_io.sh — FB_Mqtt WebSocket e2e: fake broker in --ws mode
# (plain ws:// on 11887) and --ws --tls mode (wss:// on 11888,
# self-signed CA + server cert).  The case connects twice with
# username/password/client_id and publishes on each.  Skips (77)
# when openssl/tplcc/sim are missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT_WS=${MQTT_PORT_WS:-11887}
PORT_WSS=${MQTT_PORT_WSS:-11888}
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
WS_LOG="$WORK/ws.log"; WSS_LOG="$WORK/wss.log"
trap 'kill $WS_PID $WSS_PID 2>/dev/null; rm -rf "$WORK"' EXIT

# reuse the TLS cert dir (generate if absent — same layout as
# mqtt_tls_io.sh)
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

python3 "$FAKE" "$PORT_WS" --ws >"$WS_LOG" 2>&1 &
WS_PID=$!
python3 "$FAKE" "$PORT_WSS" --tls "$CERTDIR/srv.pem" "$CERTDIR/srv.key" \
    --ws >"$WSS_LOG" 2>&1 &
WSS_PID=$!
sleep 0.7
kill -0 $WS_PID 2>/dev/null || { bad "ws broker start"; \
    cat "$WS_LOG"; echo "== $pass pass, $fail fail"; exit 1; }
kill -0 $WSS_PID 2>/dev/null || { bad "wss broker start"; \
    cat "$WSS_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/mqtt_ws_case.st -o "$WORK/ws.tplc" \
    >/dev/null 2>&1 \
    && ok "mqtt_ws_case build" \
    || { bad "mqtt_ws_case build"; \
         "$TPLCC" build tests/compat/mqtt_ws_case.st -o /dev/null; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 400); do
        t=$((i * 10000000))
        if [ "$i" -eq 399 ]; then
            echo "step $i time=$t in 0=00 expect 0=0f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ws.steps"

IMG=/tplc_mqttws_$$; CTL="$WORK/ws.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/ws.tplc" \
    --steps "$WORK/ws.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: ws:// + wss:// connects/publish"
else
    bad "sim replay: ws:// + wss:// connects/publish"
    tail -20 "$WORK/sim.log"
fi

# both WS CONNECTs must carry credentials on the wire
if grep -q "CONNECT cid='ws-e2e-1' user='wsuser' has_pass=True" \
        "$WS_LOG" \
    && grep -q "CONNECT cid='ws-e2e-2' user='wsuser' has_pass=True" \
        "$WSS_LOG"; then
    ok "WS CONNECTs carry username/password/client_id"
else
    bad "WS CONNECTs carry username/password/client_id"
    grep CONNECT "$WS_LOG" "$WSS_LOG" | tail -5
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
