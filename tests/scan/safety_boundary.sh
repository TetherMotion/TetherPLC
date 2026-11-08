#!/usr/bin/env bash
# safety-channel boundary (doc 15 §2): channels whose key is listed in a
# `safety` line of the package's DEPLOY_MAP section (0x0031) are read-only
# for user logic — DBG_WRITE → 415, DBG_FORCE → 414; non-safety channels
# and reads are unaffected. Skips when tplcc is not built.
set -uo pipefail
cd "$(dirname "$0")/../.."
ROOT="$PWD"
BUILD="${BUILD:-$ROOT/build}"
TPLCC="${TPLCC:-$ROOT/compiler/target/debug/tplcc}"

for f in "$BUILD/tether-plc" "$BUILD/tplc-image" "$BUILD/tplc-ctl"; do
    [ -f "$f" ] || { echo "SKIP: $f missing"; exit 77; }
done
[ -x "$TPLCC" ] || { echo "SKIP: tplcc not built"; exit 77; }

WORK="$(mktemp -d /tmp/tplc_saf.XXXXXX)"
IMG=saf_test_img
CTL="$WORK/saf.ctl"
PID=""
trap 'rm -rf "$WORK";
      [ -n "$PID" ] && kill "$PID" 2>/dev/null;
      "$BUILD"/tplc-image unlink "$IMG" 2>/dev/null' EXIT

fail=0
pass() { echo "PASS $1"; }
failf(){ echo "FAIL $1"; fail=$((fail+1)); }

# program: %IX0.0 bound to a safety key, %QX0.0 to a normal actuator
cat > "$WORK/prog.st" <<'EOF'
PROGRAM P
VAR
    saf_in  AT %IX0.0 : BOOL;
    motor   AT %QX0.0 : BOOL;
END_VAR
motor := saf_in;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/prog.st" -o "$WORK/prog.tplc" >/dev/null \
    || { echo "SKIP: tplcc compile failed"; exit 77; }

# deploy map (bridge format): saf_in declared safety → section 0x0031
cat > "$WORK/map.txt" <<'EOF'
image in=64 out=64
in off=0 size=1 key=saf_in
out off=0 size=1 key=motor_out safe=last
safety saf_in
EOF
"$TPLCC" pkg add-section "$WORK/prog.tplc" --type 0x0031 \
    --file "$WORK/map.txt" -o "$WORK/prog.map.tplc" >/dev/null \
    || { echo "SKIP: pkg add-section failed"; exit 77; }

"$BUILD/tplc-image" create "$IMG" 64 64 >/dev/null
"$BUILD/tether-plc" --image "$IMG" --package "$WORK/prog.map.tplc" \
    --ctl "$CTL" >"$WORK/run.log" 2>&1 &
PID=$!
for i in $(seq 1 40); do [ -S "$CTL" ] && break; sleep 0.1; done
[ -S "$CTL" ] || { echo "FAIL runtime did not come up: $(cat "$WORK/run.log")"; exit 1; }

# saf_in resolves to blob off 1 (motor is off 0)
out=$("$BUILD/tplc-ctl" "$CTL" resolve saf_in)
echo "$out" | grep -q "off 1" \
    && pass "saf_in resolved" || failf "resolve: $out"

out=$("$BUILD/tplc-ctl" "$CTL" force 1 01 2>&1)
echo "$out" | grep -q "414" \
    && pass "force safety → FORCE_DENIED" || failf "force: $out"

out=$("$BUILD/tplc-ctl" "$CTL" dbg-write 1 01 2>&1)
echo "$out" | grep -q "415" \
    && pass "write safety → DBG_ACCESS" || failf "write: $out"

out=$("$BUILD/tplc-ctl" "$CTL" force 0 01)
echo "$out" | grep -q "forced" \
    && pass "force non-safety ok" || failf "non-safety force: $out"

out=$("$BUILD/tplc-ctl" "$CTL" dbg-read 1 1)
echo "$out" | grep -q "00000000" \
    && pass "read safety allowed" || failf "read: $out"

"$BUILD/tplc-ctl" "$CTL" shutdown >/dev/null 2>&1 || true

echo "== safety_boundary: $fail failures"
[ "$fail" -eq 0 ]
