#!/usr/bin/env bash
# watchdog escalation gate: an uncooperative (spinning) task must be
# flagged FAULT in the image at ~hang-timeout and the process must
# _exit for supervisor respawn at ~2x hang-timeout. Regression test —
# the first version of the monitor reset stale_since once PS_FAULT was
# set, so the 2x backstop could never fire.
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_wd_gate
CTL=/tmp/tplc_wd_gate.ctl
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG"' 2>/dev/null' EXIT

[ -x "$BUILD/tether-plc" ] || { echo "SKIP: no build tree"; exit 77; }
[ -x "$BUILD/hang_spin.so" ] || { echo "SKIP: no hang_spin fixture"; exit 77; }
[ -x "$BUILD/tplc-image" ] || { echo "SKIP: no tplc-image"; exit 77; }

$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --module $BUILD/hang_spin.so \
    --ctl $CTL --hang-timeout-ms 300 >/tmp/tplc_wd_gate.log 2>&1 &
RT_PID=$!

deadline=$((SECONDS + 6))
fault=0
while [ $SECONDS -lt $deadline ]; do
    st=$($BUILD/tplc-image dump $IMG 2>/dev/null | awk '/plc_hb/{for(i=1;i<=NF;i++) if($i=="status"){print $(i+1); exit}}')
    [ "$st" = "fault" ] && fault=1
    kill -0 $RT_PID 2>/dev/null || break
    sleep 0.05
done
wait $RT_PID 2>/dev/null; RT_PID=

grep -q "scan stall" /tmp/tplc_wd_gate.log || { echo "FAIL: no stall detection"; exit 1; }
grep -q "unrecovered stall — exiting" /tmp/tplc_wd_gate.log \
    || { echo "FAIL: no _exit escalation"; exit 1; }
[ "$fault" -eq 1 ] || { echo "FAIL: image fault flag never set"; exit 1; }
echo "PASS: watchdog flagged fault and _exit escalated"
