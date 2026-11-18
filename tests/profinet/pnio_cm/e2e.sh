#!/usr/bin/env bash
# pnio_cm e2e: PNIO-CM AR content layer (PnioCm) against
# fake_pnio_cm_device.py — an in-process PROFINET IO device speaking
# connectionless DCE/RPC on UDP/34964.  Exercises the full AR
# establishment sequence: Connect -> Write(F-Parameters) -> PrmEnd ->
# device ApplicationReady -> Release, plus an I&M0 record read.
# Skips (77) when g++ or python3 is missing.
set -u
cd "$(dirname "$0")/../../.."
PORT=${PNIO_CM_PORT:-34964}
command -v g++ >/dev/null || { echo "g++ missing" >&2; exit 77; }
command -v python3 >/dev/null || { echo "python3 missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'kill $DEV_PID 2>/dev/null; rm -rf "$WORK"' EXIT

g++ -std=c++23 -I runtime/host/include \
    tests/profinet/pnio_cm/pnio_cm_smoke.cpp \
    runtime/host/src/PnioCm.cpp runtime/host/src/Dcerpc.cpp \
    -o "$WORK/pnio_cm_smoke" 2>"$WORK/build.log" \
    && ok "smoke build" || { bad "smoke build"; cat "$WORK/build.log"; \
         echo "== $pass pass, $fail fail"; exit 1; }

g++ -std=c++23 -I runtime/host/include \
    tests/profinet/pnio_cm/pnio_cm_golden.cpp \
    runtime/host/src/PnioCm.cpp runtime/host/src/Dcerpc.cpp \
    -o "$WORK/pnio_cm_golden" 2>>"$WORK/build.log" \
    && ok "golden build" || { bad "golden build"; cat "$WORK/build.log"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$WORK/pnio_cm_golden" tests/profinet/pnio_cm/golden \
    >"$WORK/golden.log" 2>&1 \
    && ok "golden IODRead/I&M decode (real p-net capture)" \
    || { bad "golden decode"; cat "$WORK/golden.log"; }

python3 tests/profinet/pnio_cm/fake_pnio_cm_device.py "$PORT" \
    >"$WORK/device.log" 2>&1 &
DEV_PID=$!
sleep 0.4
kill -0 "$DEV_PID" 2>/dev/null || { bad "fake CM device start"; \
    cat "$WORK/device.log"; echo "== $pass pass, $fail fail"; exit 1; }

"$WORK/pnio_cm_smoke" "$PORT" >"$WORK/smoke.log" 2>&1
rc=$?
cat "$WORK/smoke.log"
if [ $rc -eq 0 ] && grep -q "final state=3" "$WORK/smoke.log" \
    && grep -q "release OK" "$WORK/smoke.log"; then
    ok "AR sequence: connect/write/prmend/appready/release + im0 read"
else
    bad "AR sequence"; tail -15 "$WORK/device.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
