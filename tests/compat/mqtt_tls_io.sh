#!/usr/bin/env bash
# mqtt_tls_io.sh — FB_Mqtt TLS e2e: fake broker wrapped in TLS
# (self-signed CA + server cert for localhost/127.0.0.1), client
# connects twice — once tls_insecure, once CA-verified — with
# username/password/client_id on both CONNECTs.  Skips (77) when
# openssl/tplcc/sim are missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT:-11886}
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

# CA + server cert (SAN localhost + 127.0.0.1) at a fixed path the
# ST case references
mkdir -p "$CERTDIR"
openssl req -x509 -newkey rsa:2048 -keyout "$CERTDIR/ca.key" \
    -out "$CERTDIR/ca.pem" -days 2 -nodes -subj '/CN=TetherTestCA' \
    >/dev/null 2>&1
openssl req -newkey rsa:2048 -keyout "$CERTDIR/srv.key" \
    -out "$CERTDIR/srv.csr" -nodes -subj '/CN=localhost' \
    >/dev/null 2>&1
printf 'subjectAltName=DNS:localhost,IP:127.0.0.1\n' > "$CERTDIR/ext.cnf"
openssl x509 -req -in "$CERTDIR/srv.csr" -CA "$CERTDIR/ca.pem" \
    -CAkey "$CERTDIR/ca.key" -CAcreateserial -out "$CERTDIR/srv.pem" \
    -days 2 -extfile "$CERTDIR/ext.cnf" >/dev/null 2>&1
[ -f "$CERTDIR/ca.pem" ] && [ -f "$CERTDIR/srv.pem" ] \
    || { bad "cert generation"; echo "== $pass pass, $fail fail"; exit 1; }

python3 "$FAKE" "$PORT" --tls "$CERTDIR/srv.pem" "$CERTDIR/srv.key" \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "TLS broker start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/mqtt_tls_case.st -o "$WORK/tls.tplc" \
    >/dev/null 2>&1 \
    && ok "mqtt_tls_case build" \
    || { bad "mqtt_tls_case build"; \
         "$TPLCC" build tests/compat/mqtt_tls_case.st -o /dev/null; \
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
} > "$WORK/tls.steps"

IMG=/tplc_mqttls_$$; CTL="$WORK/tls.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tls.tplc" \
    --steps "$WORK/tls.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: TLS insecure + CA-verified connects"
else
    bad "sim replay: TLS insecure + CA-verified connects"
    tail -20 "$WORK/sim.log"
fi

# both CONNECTs must carry credentials on the wire
nconn=$(grep -c "CONNECT cid='tls-e2e" "$SRV_LOG" || true)
if [ "$nconn" -ge 2 ] \
    && grep -q "user='tlsuser' has_pass=True" "$SRV_LOG"; then
    ok "TLS CONNECTs carry username/password/client_id"
else
    bad "TLS CONNECTs carry username/password/client_id"
    grep CONNECT "$SRV_LOG" | tail -5
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
