#!/usr/bin/env bash
# tc2_mc2_modulo_io.sh — Tc2_MC2 tranche-4 wrappers over the Tether
# IO protocol e2e (plan 21 C5): builds compat/tc2/tc2_mc2.st as a
# --library package, starts tether_io_motion_server
# (SimulatedCiA402Drive fleet, --clock sim), and replays
# tests/compat/tc2_mc2_modulo_case.st through tether-plc-sim with
# TPLC_MOTION_IO set — the same modulo/override/param wrappers that
# run in-process drive the remote simulated DS402 drive. Skips (77)
# when the Tether server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${MOTION_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${MOTION_PORT:-14102}
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

"$TPLCC" build --library --name Tc2_MC2 \
    compat/tc2/tc2_mc2.st -o "$WORK/tc2_mc2.tplc" >/dev/null 2>&1 \
    && ok "Tc2_MC2 library" || bad "Tc2_MC2 library"

"$TPLCC" build tests/compat/tc2_mc2_modulo_case.st \
    --lib "$WORK/tc2_mc2.tplc" \
    -o "$WORK/tc2_mc2_modulo_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 400); do
        t=$((i * 10000000))
        if [ "$i" -eq 400 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff0f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mcmod.steps"

IMG=/tplc_tc2mcmodio_$$; CTL="$WORK/mcmod.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_mc2_modulo_case.tplc" \
    --steps "$WORK/mcmod.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: modulo/params/override/state/halt"
else
    bad "sim replay over Tether IO"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
