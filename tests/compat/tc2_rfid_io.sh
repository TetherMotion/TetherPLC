#!/usr/bin/env bash
# tc2_rfid_io.sh — wired RFID reader e2e: builds
# compat/tc2/tc2_rfid.st as a --library package, starts a PTY
# reader fake, and replays tc2_rfid_case.st (open -> send READ ->
# receive tag record -> close).  Skips (77) when python3 or the
# toolchain is missing.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${RFID_FAKE:-tests/compat/rfid_fake.py}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null 2>&1 || { echo "python3 missing" >&2; exit 77; }
[ -f "$FAKE" ] || { echo "rfid fake missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'kill $SRV_PID 2>/dev/null; rm -f /tmp/rfid_pty; rm -rf "$WORK"' EXIT

python3 "$FAKE" "$WORK/rx.log" >"$WORK/fake.out" 2>&1 &
SRV_PID=$!
sleep 0.8
kill -0 "$SRV_PID" 2>/dev/null || { bad "rfid fake start"; cat "$WORK/fake.out"; echo "== $pass pass, $fail fail"; exit 1; }
PTY=$(grep '^pty:' "$WORK/rx.log" | cut -d: -f2)
[ -n "$PTY" ] || { bad "rfid fake pty"; cat "$WORK/fake.out"; echo "== $pass pass, $fail fail"; exit 1; }
ln -sf "$PTY" /tmp/rfid_pty

"$TPLCC" build --library --name Tc2_RFID \
    compat/tc2/tc2_rfid.st -o "$WORK/tc2_rfid.tplc" >/dev/null 2>&1 \
    && ok "Tc2_RFID library" || { bad "Tc2_RFID library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_rfid_case.st \
    --lib "$WORK/tc2_rfid.tplc" \
    -o "$WORK/tc2_rfid_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 200); do
        t=$((i * 10000000))
        if [ "$i" -eq 200 ]; then
            echo "step $i time=$t in 0=00 expect 0=1f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/rfid.steps"

IMG=/tplc_tc2rfid_$$; CTL="$WORK/rfid.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_rfid_case.tplc" \
    --steps "$WORK/rfid.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 2000 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: reader open/READ/tag-received/close"
else
    bad "sim replay: reader open/READ/tag-received/close"
    tail -20 "$WORK/sim.log"
    cat "$WORK/rx.log" 2>/dev/null
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
