#!/usr/bin/env bash
# tc2_nci_gcode_io.sh — Tc2_NCI G-code channel e2e over the Tether IO
# protocol: builds compat/tc2/tc2_nci.st as a --library package,
# starts tether_io_motion_server (--clock sim, --axes 4), and replays
# tests/compat/tc2_nci_gcode_case.st through tether-plc-sim with
# TPLC_MOTION_IO set. The FB_ItpLoadText/StartStop/Reset/Status
# wrappers drive the server's nc.* channel: program text upload,
# compile, run, status polling, stop, reset — G-code interpretation
# and CSP streaming stay on the Tether side. Skips (77) when the
# Tether server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${MOTION_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${MOTION_PORT:-14105}
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

"$TPLCC" build --library --name Tc2_NCI \
    compat/tc2/tc2_nci.st -o "$WORK/tc2_nci.tplc" >/dev/null 2>&1 \
    && ok "Tc2_NCI library" || { bad "Tc2_NCI library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_nci_gcode_case.st \
    --lib "$WORK/tc2_nci.tplc" \
    -o "$WORK/tc2_nci_gcode_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

# the NC program file loaded through FB_ItpLoadProg (nc.prog_path);
# the case loads it by this fixed path
cat >/tmp/tplc_nci_case.g <<'G'
G21 G90
G0 X0 Y0 Z0
G1 X10 Y5 Z2 F600
G2 X15 Y10 I5 J0 F600
G1 Z0 F300
G1 X0 Y0 F1200
M30
G

{
    for i in $(seq 0 900); do
        t=$((i * 10000000))
        if [ "$i" -eq 900 ]; then
            echo "step $i time=$t in 0=00 expect 0=ffff7f00"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/gcode.steps"

IMG=/tplc_tc2ncigc_$$; CTL="$WORK/gcode.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_nci_gcode_case.tplc" \
    --steps "$WORK/gcode.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: gcode load/run/status/stop/reset"
else
    bad "sim replay over Tether IO"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
