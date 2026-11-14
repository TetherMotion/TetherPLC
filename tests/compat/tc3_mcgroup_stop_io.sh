#!/usr/bin/env bash
# tc3_mcgroup_stop_io.sh — Tc3_McCoordinatedMotion group stop/halt e2e over Tether IO:
# builds compat/tc2/tc2_mc2.st + compat/tc3/tc3_mcgroup.st as
# libraries, consumes them into tests/compat/
# tc3_mcgroup_stop_case.st via --lib, and starts tether_io_motion_server (--clock sim, --axes 2) and
# replays through tether-plc-sim with TPLC_MOTION_IO set. Covers axis
# create/power, group membership, PV moves, MC_GroupStop and
# MC_GroupHalt on the IO backend. Skips (77) when the
# toolchain or Tether server binary is unavailable. Expects mask 0x3f.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${MCGS_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${MCGS_PORT:-14141}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "mcgs server missing: $SRV" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc2_MC2 \
    compat/tc2/tc2_mc2.st -o "$WORK/tc2_mc2.tplc" >/dev/null 2>&1 \
    && ok "Tc2_MC2 library" || { bad "Tc2_MC2 library"; \
         echo "== $pass pass, $fail fail"; exit 1; }
"$TPLCC" build --library --name Tc3_McGroup \
    compat/tc3/tc3_mcgroup.st -o "$WORK/tc3_mcgroup.tplc" >/dev/null 2>&1 \
    && ok "Tc3_McGroup library" || { bad "Tc3_McGroup library"; \
         echo "== $pass pass, $fail fail"; exit 1; }
"$TPLCC" build tests/compat/tc3_mcgroup_stop_case.st \
    --lib "$WORK/tc2_mc2.tplc" --lib "$WORK/tc3_mcgroup.tplc" \
    -o "$WORK/tc3_mcgroup_stop_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" \
    || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$SRV" --bind 127.0.0.1 --port "$PORT" --axes 2 --clock sim \
    >"$WORK/server.log" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "motion server start"; \
    cat "$WORK/server.log"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 200); do
        t=$((i * 10000000))
        if [ "$i" -eq 200 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f000000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/mcgs.steps"

IMG=/tplc_tcmcgsio_$$; CTL="$WORK/mcgsio.ctl"
TPLC_MOTION_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_mcgroup_stop_case.tplc" \
    --steps "$WORK/mcgs.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: group stop + halt"
else
    bad "sim replay over Tether IO: group stop + halt"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
