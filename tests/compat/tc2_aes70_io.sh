#!/usr/bin/env bash
# tc2_aes70_io.sh — OCA/AES70 client e2e: builds
# compat/tc2/tc2_aes70.st as a --library package, starts the python
# device fake (aes70_device_fake.py — OCP1 frame peer), and replays
# tc2_aes70_case.st through tether-plc-sim.  Skips (77) when
# python3, the fake, or the toolchain is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${AES70_FAKE:-tests/compat/aes70_device_fake.py}
PORT=${AES70_PORT:-65000}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null 2>&1 || { echo "python3 missing" >&2; exit 77; }
[ -f "$FAKE" ] || { echo "aes70_device_fake missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/dev.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

python3 "$FAKE" "$PORT" >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.7
kill -0 "$SRV_PID" 2>/dev/null || { bad "device fake start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc3_Aes70 \
    compat/tc2/tc2_aes70.st -o "$WORK/tc2_aes70.tplc" >/dev/null 2>&1 \
    && ok "Tc3_Aes70 library" || { bad "Tc3_Aes70 library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_aes70_case.st \
    --lib "$WORK/tc2_aes70.tplc" \
    -o "$WORK/tc2_aes70_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 250); do
        t=$((i * 10000000))
        if [ "$i" -eq 250 ]; then
            echo "step $i time=$t in 0=00 expect 0=1f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/aes70.steps"

IMG=/tplc_tc2a70_$$; CTL="$WORK/aes70.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_aes70_case.tplc" \
    --steps "$WORK/aes70.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: AES70 connect/keepalive/call/response/disconnect"
else
    bad "sim replay: AES70 connect/keepalive/call/response/disconnect"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
