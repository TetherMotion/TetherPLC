#!/usr/bin/env bash
# tc2_iec61850_ds_io.sh — IEC 61850 data-set e2e: fake IED
# (measds over Meas.instMag.f + SetPt.setMag.f) + consumer replay of
# GetDataSetDirectory + ReadDataSet.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${IEC61850_FAKE:-$BUILD/iec61850_ied_fake}
PORT=${IEC61850_PORT:-11022}
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

sed "s/11020/$PORT/g" tests/compat/tc2_iec61850_ds_case.st > "$WORK/ds_case.st"
"$TPLCC" build "$WORK/ds_case.st" \
    --lib "$WORK/tc2_iec61850.tplc" \
    -o "$WORK/tc2_iec61850_ds_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 500); do
        t=$((i*10000000))
        if [ $i -eq 500 ]; then
            echo "step $i time=$t in 0=00 expect 0=b70f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ds.steps"

if ./build/tether-plc-sim --image /tplc_ds \
    --package "$WORK/tc2_iec61850_ds_case.tplc" \
    --steps "$WORK/ds.steps" --plc ./build/tether-plc \
    --ctl "$WORK/tether-plc.ctl" --settle-ms 1500 \
    >"$WORK/sim.log" 2>&1; then
    if grep -q "all matched" "$WORK/sim.log"; then
        ok "sim replay: data-set directory/read/write"
    else
        bad "sim replay: data-set directory/read/write"
        grep -i "divergent\|error" "$WORK/sim.log" | head -5
    fi
else
    bad "sim replay: data-set directory/read/write"
    tail -10 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
