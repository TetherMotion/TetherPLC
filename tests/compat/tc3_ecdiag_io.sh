#!/usr/bin/env bash
# tc3_ecdiag_io.sh — Tc3_EtherCATDiag e2e: starts
# tether_io_motion_server (--clock sim, exposes ec.coediag.* params)
# and replays tc3_ecdiag_case.st through tether-plc-sim with
# TPLC_EC_IO set, exercising FB_CoE_DiagHistory_Read (read/ack/
# per-slave counts) over the tplc-ecdiag service.  Skips (77) when
# the toolchain or Tether server binary is unavailable.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
TETHER_DIR=${TETHER_DIR:-../Tether}
SRV=${EC_SERVER:-$TETHER_DIR/build/bin/tether_io_motion_server}
PORT=${EC_PORT:-14111}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$SRV" ] || { echo "ecdiag server missing: $SRV" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
SRV_LOG="$WORK/server.log"
trap 'kill $SRV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc3_EcDiag \
    compat/tc3/tc3_ecdiag.st -o "$WORK/tc3_ecdiag.tplc" >/dev/null 2>&1 \
    && ok "Tc3_EcDiag library" || { bad "Tc3_EcDiag library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_ecdiag_case.st \
    --lib "$WORK/tc3_ecdiag.tplc" \
    -o "$WORK/tc3_ecdiag_case.tplc" >/dev/null 2>&1 \
    && ok "case build with --lib" || { bad "case build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$SRV" --bind 127.0.0.1 --port "$PORT" --axes 1 --clock sim \
    >"$SRV_LOG" 2>&1 &
SRV_PID=$!
sleep 0.4
kill -0 "$SRV_PID" 2>/dev/null || { bad "ecdiag server start"; \
    cat "$SRV_LOG"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 119); do
        t=$((i * 10000000))
        if [ "$i" -eq 119 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff010000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ecdiag.steps"

IMG=/tplc_tc3ecd_$$; CTL="$WORK/ecdiag.ctl"
TPLC_EC_IO="tcp://127.0.0.1:$PORT" \
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_ecdiag_case.tplc" \
    --steps "$WORK/ecdiag.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay over Tether IO: read/ack/slave counts"
else
    bad "sim replay over Tether IO"; tail -15 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
