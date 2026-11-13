#!/usr/bin/env bash
# tc2_drive_io.sh — Tc2_Drive e2e: starts tether_io_motion_server
# (--clock sim, exposes ec.soe.* params plus the mc.* axis params)
# and replays tc2_drive_case.st through tether-plc-sim with
# TPLC_MOTION_IO set, exercising the SoE wrappers (state read, diag
# fields, password, command control/exec/state, error+reset, AX5000
# blocks) over tplc-soe and the SimplePlcMotion enable/velocity
# blocks over tplc-motion.  Skips (77) when the toolchain or Tether
# server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${DRIVE_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${DRIVE_PORT:-14121}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "drive server missing: $SRV" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/server.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc2_Drive \
    compat/tc2/tc2_drive.st -o "$WORK/tc2_drive.tplc" >/dev/null 2>&1 \
    && ok "Tc2_Drive library" || { bad "Tc2_Drive library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_drive_case.st \
    --lib "$WORK/tc2_drive.tplc" \
    -o "$WORK/tc2_drive_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$SRV" --bind 127.0.0.1 --port "$PORT" --axes 2 --clock sim \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "drive server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 799); do
        t=$((i * 10000000))
        if [ "$i" -eq 799 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff3f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/drive.steps"

IMG=/tplc_tc2drive_$$; CTL="$WORK/drive.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_drive_case.tplc" \
    --steps "$WORK/drive.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: soe state/cmd/diag/ax5000 + plcmotion"
else
    bad "sim replay over Tether IO"; tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
