#!/usr/bin/env bash
# tc2_iec61850_brcb_io.sh — IEC 61850 buffered RCB e2e: builds
# compat/tc2/tc2_iec61850.st as a --library package, starts the
# libiec61850-based fake IED (measds data set + measurcb URCB +
# bufrcb buffered RCB), and replays tc2_iec61850_brcb_case.st through
# tether-plc-sim: GetRCBValues on the $BR$ ref, SetRCBValues enable,
# dataset write, dchg InformationReport via report poll, PurgeBuf,
# disable, disconnect.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${IEC61850_FAKE:-$BUILD/iec61850_ied_fake}
PORT=${IEC61850_PORT:-11020}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$FAKE" ] || { echo "iec61850_ied_fake missing (libiec61850 off)" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/ied.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$FAKE" "$PORT" >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 1.0
kill -0 "$SRV_PID" 2>/dev/null || { bad "ied fake start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc2_IEC61850 \
    compat/tc2/tc2_iec61850.st -o "$WORK/tc2_iec61850.tplc" >/dev/null 2>&1 \
    && ok "Tc2_IEC61850 library" || { bad "Tc2_IEC61850 library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_iec61850_brcb_case.st \
    --lib "$WORK/tc2_iec61850.tplc" \
    -o "$WORK/tc2_iec61850_brcb_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 400); do
        t=$((i * 10000000))
        if [ "$i" -eq 400 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff01"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/iec.steps"

IMG=/tplc_tc2iecbrcb_$$; CTL="$WORK/iec.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_iec61850_brcb_case.tplc" \
    --steps "$WORK/iec.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: BRCB get/set/enable/dchg-report/purge/disable"
else
    bad "sim replay: buffered RCB get/set/report/integrity"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
