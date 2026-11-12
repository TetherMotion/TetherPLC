#!/usr/bin/env bash
# tc2_eip_io.sh — EtherNet/IP client e2e: builds compat/tc2/tc2_eip.st
# as a --library package, starts the Python EIP adapter fake
# (eip_adapter_fake.py — encapsulation + CIP Get/Set_Attribute_Single),
# and replays tests/compat/tc2_eip_case.st through tether-plc-sim.
# The FB_Eip* wrappers drive a real session: RegisterSession,
# ListIdentity, get/set attribute, unregister. Skips (77) when the
# toolchain or Python is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
PORT=${EIP_PORT:-14481}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null || { echo "python3 missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/eip.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

python3 tests/compat/eip_adapter_fake.py "$PORT" >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.5
kill -0 "$SRV_PID" 2>/dev/null || { bad "eip fake start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build --library --name Tc2_EIP \
    compat/tc2/tc2_eip.st -o "$WORK/tc2_eip.tplc" >/dev/null 2>&1 \
    && ok "Tc2_EIP library" || { bad "Tc2_EIP library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_eip_case.st \
    --lib "$WORK/tc2_eip.tplc" \
    -o "$WORK/tc2_eip_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 200); do
        t=$((i * 10000000))
        if [ "$i" -eq 200 ]; then
            echo "step $i time=$t in 0=00 expect 0=7f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/eip.steps"

IMG=/tplc_tc2eip_$$; CTL="$WORK/eip.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_eip_case.tplc" \
    --steps "$WORK/eip.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1200 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: EIP register/identity/getattr/setattr/close"
else
    bad "sim replay: EIP register/identity/getattr/setattr/close"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
