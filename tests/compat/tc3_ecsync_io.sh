#!/usr/bin/env bash
# tc3_ecsync_io.sh — Tc3_EtherCATExtSync e2e: starts
# tether_io_motion_server (--clock sim, exposes ec.dc.* params) and
# replays tc3_ecsync_case.st through tether-plc-sim with TPLC_EC_IO
# set, exercising FB_EcExtSyncExtTimes/FB_EcExtSyncIsSynchronized over
# the tplc-ecsync service.  A second replay without the endpoint
# verifies the not-connected path.  Skips (77) when the toolchain or
# Tether server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${EC_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${EC_PORT:-14110}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "ecsync server missing: $SRV" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/server.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc3_EcSync \
    compat/tc3/tc3_ecsync.st -o "$WORK/tc3_ecsync.tplc" >/dev/null 2>&1 \
    && ok "Tc3_EcSync library" || { bad "Tc3_EcSync library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_ecsync_case.st \
    --lib "$WORK/tc3_ecsync.tplc" \
    -o "$WORK/tc3_ecsync_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_ecsync_nosvc_case.st \
    --lib "$WORK/tc3_ecsync.tplc" \
    -o "$WORK/tc3_ecsync_nosvc.tplc" >/dev/null 2>&1 \
    && ok "nosvc case build" || { bad "nosvc case build"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$SRV" --bind 127.0.0.1 --port "$PORT" --axes 1 --clock sim \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "ecsync server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 199); do
        t=$((i * 10000000))
        if [ "$i" -eq 199 ]; then
            echo "step $i time=$t in 0=00 expect 0=0f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ecsync.steps"

IMG=/tplc_tc3ec_$$; CTL="$WORK/ecsync.ctl"
TPLC_EC_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_ecsync_case.tplc" \
    --steps "$WORK/ecsync.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: enable/window/disable/re-enable"
else
    bad "sim replay over Tether IO"; tail -15 "$WORK/sim.log"
fi

# no-endpoint run: service inactive -> bNotConnected + bError
{
    for i in $(seq 0 20); do
        t=$((i * 10000000))
        if [ "$i" -eq 20 ]; then
            echo "step $i time=$t in 0=00 expect 0=03000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/nosvc.steps"
IMG=/tplc_tc3ecn_$$; CTL="$WORK/nosvc.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_ecsync_nosvc.tplc" \
    --steps "$WORK/nosvc.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 500 >"$WORK/nosvc.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/nosvc.log"; then
    ok "sim replay without endpoint: not-connected path"
else
    bad "sim replay without endpoint"; tail -15 "$WORK/nosvc.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
