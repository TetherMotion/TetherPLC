#!/usr/bin/env bash
# profinet e2e: tplc-profinet (FB_Profinet) against
# fake_profinet_device.py — an in-process PROFINET device speaking
# DCP (Discovery and Configuration Protocol) PDUs over UDP plus the
# cyclic IO record primitive.  Exercises open, service state, DCP
# Identify (all device properties), DCP Get/Set round-trip, cyclic
# record exchange, close, and invalid-op rejection.
# Skips (77) when tplcc, the sim, or python3 is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
PORT=${PN_PORT:-14511}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null || { echo "python3 missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'kill $DEV_PID $CM_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build tests/profinet/profinet_case.st \
    -o "$WORK/profinet_case.tplc" >/dev/null 2>&1 \
    && ok "case build" || { bad "case build"; \
         echo "== $pass pass, $fail fail"; exit 1; }

python3 tests/profinet/fake_profinet_device.py "$PORT" >"$WORK/device.log" 2>&1 &
DEV_PID=$!
# PNIO-CM fake device on UDP/34964 (AR connect/write/control/read)
python3 tests/profinet/pnio_cm/fake_pnio_cm_device.py 34964     >"$WORK/cm_device.log" 2>&1 &
CM_PID=$!
sleep 0.4
kill -0 "$DEV_PID" 2>/dev/null || { bad "fake device start"; \
    cat "$WORK/device.log"; echo "== $pass pass, $fail fail"; exit 1; }
kill -0 "$CM_PID" 2>/dev/null || { bad "fake CM device start"; \
    cat "$WORK/cm_device.log"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 250); do
        # 20 ms/step: image clock must advance for the 10 ms task
        t=$((i * 20000000))
        if [ "$i" -eq 250 ]; then
            echo "step $i time=$t in 0=00 expect 0=ffff0100"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/profinet.steps"

IMG=/tplc_profinet_$$; CTL="$WORK/profinet.ctl"
TPLC_PROFINET_IO="127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/profinet_case.tplc" \
    --steps "$WORK/profinet.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: profinet dcp identify/get/set/io over fake device"
else
    bad "sim replay"; tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
