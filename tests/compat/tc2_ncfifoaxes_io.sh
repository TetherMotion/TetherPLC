#!/usr/bin/env bash
# tc2_ncfifoaxes_io.sh — Tc2_NcFifoAxes e2e over Tether IO: builds
# compat/tc2/tc2_mc2.st + compat/tc2/tc2_ncfifoaxes.st as --library
# packages, starts tether_io_motion_server (--clock sim, --axes 2),
# and replays tests/compat/tc2_ncfifoaxes_case.st through
# tether-plc-sim with TPLC_MOTION_IO set. Exercises the fifo.*
# channel params end to end: integrate/dimension/write/override/
# start/drain/overwrite/disintegrate with rows consumed on the
# mc.time drive clock. Skips (77) when the toolchain or Tether
# server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${FIFO_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${FIFO_PORT:-14133}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "fifo server missing: $SRV" >&2; exit 77; }
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

"$TPLCC" build --library --name Tc2_NcFifoAxes \
    compat/tc2/tc2_ncfifoaxes.st -o "$WORK/tc2_ncfifoaxes.tplc" \
    >/dev/null 2>&1 \
    && ok "Tc2_NcFifoAxes library" || { bad "Tc2_NcFifoAxes library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_ncfifoaxes_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc2_ncfifoaxes.tplc" \
    -o "$WORK/tc2_ncfifoaxes_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$SRV" --bind 127.0.0.1 --port "$PORT" --axes 2 --clock sim \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "motion server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 300); do
        t=$((i * 10000000))
        if [ "$i" -eq 300 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff3f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/fifo.steps"

IMG=/tplc_tcfifoio_$$; CTL="$WORK/fifo.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_ncfifoaxes_case.tplc" \
    --steps "$WORK/fifo.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: fifo.* channel"
else
    bad "sim replay over Tether IO: fifo.* channel"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
