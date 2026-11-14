#!/usr/bin/env bash
# historian/e2e.sh — HIST_CONF/HIST_LIST + TRACE_READ/CSV export:
#   named symmap channels -> flight-recorder ring -> ctl read.
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_hist_$$
SHM=/dev/shm$IMG
CTL=/tmp/tplc_hist_$$.ctl
WORK=$(mktemp -d)
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink "'"$IMG"'" 2>/dev/null; rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] || { echo "SKIP: no build tree"; exit 77; }
[ -x "$BUILD/tplc-ctl" ] || { echo "SKIP: no tplc-ctl"; exit 77; }

"$TPLCC" build tests/historian/hist_case.st -o "$WORK/hist.tplc" \
    >/dev/null || { echo "hist_case build failed"; exit 1; }

"$BUILD/tplc-image" create "$IMG" 64 64 >/dev/null
"$BUILD/tether-plc" --image "$IMG" --package "$WORK/hist.tplc" \
    --ctl "$CTL" >"$WORK/rt.log" 2>&1 &
RT_PID=$!
sleep 1

# arm three named channels: DINT(4) + LREAL(8) + INT(2) = 14 B frame
A=$("$BUILD/tplc-ctl" "$CTL" hist n ramp stage 2>&1)
echo "$A" | grep -q "hist armed: 14 B frame" \
    && ok "hist conf by symbol name" \
    || bad "hist conf ($A)"

L=$("$BUILD/tplc-ctl" "$CTL" hist-list 2>&1)
echo "$L" | grep -q "channels 3" \
    && echo "$L" | grep -q "ramp" \
    && ok "hist-list names" \
    || bad "hist-list ($L)"

# let ~1 s of scans accumulate (~100 frames at 10 ms task period)
sleep 1

R=$("$BUILD/tplc-ctl" "$CTL" trace-read 0 8 2>&1)
echo "$R" | grep -q "frames " \
    && echo "$R" | grep -Eq "#[0-9]+ t=[0-9]+ +[0-9a-f]{28}" \
    && ok "trace-read returns named-channel frames" \
    || bad "trace-read ($R)"

C=$("$BUILD/tplc-ctl" "$CTL" trace-csv 0 4 2>&1)
echo "$C" | grep -q "^seq,ts_ns" \
    && [ "$(echo "$C" | grep -c '^[0-9]')" -ge 2 ] \
    && ok "trace-csv export" \
    || bad "trace-csv ($C)"

# re-arm resets the ring (seq restarts)
"$BUILD/tplc-ctl" "$CTL" hist n >/dev/null 2>&1
sleep 0.3
R2=$("$BUILD/tplc-ctl" "$CTL" trace-read 0 64 2>&1)
echo "$R2" | grep -q "first seq 0" \
    && ok "hist re-arm resets ring" \
    || bad "hist re-arm ($R2)"

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
