#!/usr/bin/env bash
# iolink e2e: tplc-iolink (FB_IoLink) against fake_iolink_device.py —
# an in-process IO-Link device speaking the IEC 61131-9 data-link
# protocol over TCP.  Exercises wakeup/open, DPP1 identification via
# the page channel, cyclic type-2_V process-data exchange with A.1.6
# checksum verification, ISDU parameter write/read, close, and
# invalid-op rejection.
# Skips (77) when tplcc, the sim, or python3 is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
PORT=${IL_PORT:-14411}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null || { echo "python3 missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'kill $DEV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build tests/iolink/iolink_case.st \
    -o "$WORK/iolink_case.tplc" >/dev/null 2>&1 \
    && ok "case build" || { bad "case build"; \
         echo "== $pass pass, $fail fail"; exit 1; }

python3 tests/iolink/fake_iolink_device.py "$PORT" >"$WORK/device.log" 2>&1 &
DEV_PID=$!
sleep 0.4
kill -0 "$DEV_PID" 2>/dev/null || { bad "fake device start"; \
    cat "$WORK/device.log"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 200); do
        # 20 ms/step: image clock must advance for the 10 ms task
        t=$((i * 20000000))
        if [ "$i" -eq 200 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff0f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/iolink.steps"

IMG=/tplc_iolink_$$; CTL="$WORK/iolink.ctl"
TPLC_IOLINK_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/iolink_case.tplc" \
    --steps "$WORK/iolink.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: iolink open/ident/pd/isdu over fake device"
else
    bad "sim replay"; tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
