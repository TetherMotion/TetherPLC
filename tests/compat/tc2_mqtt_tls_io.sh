#!/usr/bin/env bash
# tc2_mqtt_tls_io.sh — MQTT over TLS e2e: generates a self-signed
# broker cert, runs mqtt_broker_fake.py --tls, and replays
# tc2_mqtt_tls_case.st through tether-plc-sim.  conn1 connects with
# TlsInsecure (skip verify); conn2 verifies against the cert via
# TlsCaFile.  Skips (77) when the toolchain or openssl is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${MQTT_FAKE:-tests/compat/mqtt_broker_fake.py}
PORT=${MQTT_PORT:-11884}
CERTDIR=${MQTT_TLS_CERTDIR:-/tmp/tplc_mqtt_tls}
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
openssl req -x509 -newkey rsa:2048 -nodes -days 1 \
    -keyout "$CERTDIR/key.pem" -out "$CERTDIR/cert.pem" \
    -subj "/CN=localhost" \
    -addext "subjectAltName=DNS:localhost,IP:127.0.0.1" \
    >/dev/null 2>&1 || { echo "cert generation failed" >&2; exit 77; }

python3 "$FAKE" "$PORT" --tls "$CERTDIR/cert.pem" "$CERTDIR/key.pem" \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "broker fake start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc3_Mqtt \
    compat/tc2/tc2_mqtt.st -o "$WORK/tc2_mqtt.tplc" >/dev/null 2>&1 \
    && ok "Tc3_Mqtt library" || { bad "Tc3_Mqtt library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_mqtt_tls_case.st \
    --lib "$WORK/tc2_mqtt.tplc" \
    -o "$WORK/tc2_mqtt_tls_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 800); do
        t=$((i * 10000000))
        if [ "$i" -eq 790 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff07"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/tls.steps"

IMG=/tplc_tc2mqtttls_$$; CTL="$WORK/tls.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_mqtt_tls_case.tplc" \
    --steps "$WORK/tls.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: MQTT TLS insecure + CA-verified e2e"
else
    bad "sim replay: MQTT TLS insecure + CA-verified e2e"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
