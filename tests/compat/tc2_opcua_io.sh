#!/usr/bin/env bash
# tc2_opcua_io.sh — OPC UA client e2e: builds compat/tc2/tc2_opcua.st
# as a --library package, starts tether_io_motion_server with --opcua
# (the open62541 endpoint), and replays tests/compat/tc2_opcua_case.st
# through tether-plc-sim. The FB_UA_* wrappers drive a real OPC UA
# session: connect, session-open status, read nc.state, write+read
# nc.scale, namespace-index lookup, disconnect. Skips (77) when the
# server binary or open62541 is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${MOTION_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${MOTION_PORT:-14105}
UAPORT=${UA_PORT:-14199}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "motion server missing: $SRV" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/server.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$SRV" --bind 127.0.0.1 --port "$PORT" --axes 4 --clock sim \
    --opcua "$UAPORT" >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.5
kill -0 "$SRV_PID" 2>/dev/null || { bad "motion server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }
grep -q "opc ua" "$SRV_LOG" || { echo "SKIP: no open62541 in server"; \
    exit 77; }

"$TPLCC" build --library --name Tc2_OPCUA \
    compat/tc2/tc2_opcua.st -o "$WORK/tc2_opcua.tplc" >/dev/null 2>&1 \
    && ok "Tc2_OPCUA library" || { bad "Tc2_OPCUA library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_opcua_case.st \
    --lib "$WORK/tc2_opcua.tplc" \
    -o "$WORK/tc2_opcua_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 300); do
        t=$((i * 10000000))
        if [ "$i" -eq 300 ]; then
            echo "step $i time=$t in 0=00 expect 0=7f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ua.steps"

IMG=/tplc_tc2opcua_$$; CTL="$WORK/ua.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_opcua_case.tplc" \
    --steps "$WORK/ua.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: OPC UA connect/read/write/disconnect"
else
    bad "sim replay: OPC UA connect/read/write/disconnect"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
