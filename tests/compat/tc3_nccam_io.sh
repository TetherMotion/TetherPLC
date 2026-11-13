#!/usr/bin/env bash
# tc3_nccam_io.sh — Tc3_NC_Camming over the Tether IO protocol e2e:
# builds compat/tc3/tc3_nccam.st as a --library package, starts
# tether_io_motion_server (SimulatedCiA402Drive fleet, --clock sim),
# and replays tests/compat/tc3_nccam_case.st through tether-plc-sim
# with TPLC_MOTION_IO set. The cam coupling FB drives the remote
# simulated DS402 drives; FB_MotionAxis object ops cross the wire
# as mc.<slave>.<index> params. Skips (77) when the Tether server
# is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${MOTION_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${MOTION_PORT:-14111}
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
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "motion server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc3_NcCam \
    compat/tc3/tc3_nccam.st -o "$WORK/tc3_nccam.tplc" >/dev/null 2>&1 \
    && ok "Tc3_NcCam library" || bad "Tc3_NcCam library"

"$TPLCC" build tests/compat/tc3_nccam_case.st \
    --lib "$WORK/tc3_nccam.tplc" \
    -o "$WORK/tc3_nccam_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 999); do
        t=$((i * 10000000))
        if [ "$i" -eq 999 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/cam.steps"

IMG=/tplc_tc3nccamio_$$; CTL="$WORK/cam.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_nccam_case.tplc" \
    --steps "$WORK/cam.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: cam interp/wrap/clear/writept"
else
    bad "sim replay over Tether IO"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
