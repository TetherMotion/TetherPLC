#!/usr/bin/env bash
# tc2_motion_io.sh — tplc-motion over the Tether IO protocol e2e:
# starts tether_io_motion_server (SimulatedCiA402Drive fleet, --clock
# sim) and replays tests/compat/tc2_motion_case.st through
# tether-plc-sim with TPLC_MOTION_IO set, so FB_MotionAxis object
# ops cross the wire as mc.<slave>.<index> params while mc.time
# carries the deterministic --sim timestamp. Skips (77) when the
# Tether server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${MOTION_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${MOTION_PORT:-14100}
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

"$TPLCC" build tests/compat/tc2_motion_case.st \
    -o "$WORK/tc2_motion_case.tplc" >/dev/null 2>&1 \
    && ok "motion case build" || { bad "motion case build"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 499); do
        t=$((i * 10000000))
        if [ "$i" -eq 499 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff0f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/motion.steps"

IMG=/tplc_tc2motionio_$$; CTL="$WORK/motion.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_motion_case.tplc" \
    --steps "$WORK/motion.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: create/pp-move/vel-mode/errors/destroy"
else
    bad "sim replay over Tether IO"
    tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
