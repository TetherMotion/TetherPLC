#!/usr/bin/env bash
# tc2_mc2_drive_io.sh — Tc2_MC2_Drive e2e: builds
# compat/tc2/tc2_mc2.st + compat/tc2/tc2_mc2_drive.st as --library
# packages, starts tether_io_motion_server (--clock sim, --axes 2),
# and replays tests/compat/tc2_mc2_drive_case.st through
# tether-plc-sim with TPLC_MOTION_IO set.  Exercises the Beckhoff
# general OD blocks (position offset/brake/park via 0x2002..0x2004),
# CoE read/write/command (0x2005), AX5000/AX8000/EL72xx wrappers, the
# SoE surface over tplc-soe, flying-saw characteristics
# (0x2006..0x2009) and the MC_GearInVelo/MC_GearInPos coupling over
# the mc.* axis params.  Skips (77) when the toolchain or Tether
# server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${DRIVE_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${DRIVE_PORT:-14122}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "drive server missing: $SRV" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/server.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc2_MC2 \
    compat/tc2/tc2_mc2.st -o "$WORK/tc2_mc2.tplc" >/dev/null 2>&1 \
    && ok "Tc2_MC2 library" || { bad "Tc2_MC2 library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc2_MC2_Drive \
    compat/tc2/tc2_mc2_drive.st -o "$WORK/tc2_mc2_drive.tplc" \
    >/dev/null 2>&1 \
    && ok "Tc2_MC2_Drive library" || { bad "Tc2_MC2_Drive library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_mc2_drive_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc2_mc2_drive.tplc" \
    -o "$WORK/tc2_mc2_drive_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$SRV" --bind 127.0.0.1 --port "$PORT" --axes 2 --clock sim \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "motion server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 399); do
        t=$((i * 10000000))
        if [ "$i" -eq 399 ]; then
            echo "step $i time=$t in 0=00 expect 0=ffffff0f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mc2drive.steps"

IMG=/tplc_tc2mc2drive_$$; CTL="$WORK/mc2drive.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_mc2_drive_case.tplc" \
    --steps "$WORK/mc2drive.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: OD/SoE/AX*/gear blocks"
else
    bad "sim replay over Tether IO"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
