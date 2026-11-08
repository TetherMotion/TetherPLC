#!/usr/bin/env bash
# management surface conformance (doc 16 §9): v2 prog.status fields,
# strict-transition -501 errors with current+requested, transition
# events with epoch, journal tail + map get. Skips when tplcc absent.
set -uo pipefail
cd "$(dirname "$0")/../.."
ROOT="$PWD"
BUILD="${BUILD:-$ROOT/build}"
TPLCC="${TPLCC:-$ROOT/compiler/target/debug/tplcc}"

for f in "$BUILD/tether-plc" "$BUILD/tplc-image" "$BUILD/tplc-ctl"; do
    [ -f "$f" ] || { echo "SKIP: $f missing"; exit 77; }
done
[ -x "$TPLCC" ] || { echo "SKIP: tplcc not built"; exit 77; }

WORK="$(mktemp -d /tmp/tplc_mgmt.XXXXXX)"
IMG=mgmt_test_img
CTL="$WORK/m.ctl"
PID=""
trap 'rm -rf "$WORK";
      [ -n "$PID" ] && kill "$PID" 2>/dev/null;
      "$BUILD"/tplc-image unlink "$IMG" 2>/dev/null' EXIT

fail=0
pass() { echo "PASS $1"; }
failf(){ echo "FAIL $1"; fail=$((fail+1)); }

cat > "$WORK/a.st" <<'EOF'
PROGRAM P
VAR
    x AT %IX0.0 : BOOL;
    y AT %QX0.0 : BOOL;
END_VAR
y := x;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/a.st" --prog-version 1.4.2 \
    -o "$WORK/a.tplc" >/dev/null || { echo "SKIP: build failed"; exit 77; }

"$BUILD/tplc-image" create "$IMG" 64 64 >/dev/null
"$BUILD/tether-plc" --image "$IMG" --package "$WORK/a.tplc" \
    --ctl "$CTL" --audit-log "$WORK/audit.log" >"$WORK/run.log" 2>&1 &
PID=$!
for i in $(seq 1 40); do [ -S "$CTL" ] && break; sleep 0.1; done
[ -S "$CTL" ] || { echo "FAIL runtime up: $(cat "$WORK/run.log")"; exit 1; }

# §3.3 status object
out=$("$BUILD/tplc-ctl" "$CTL" prog)
echo "$out" | grep -q "version    1.4.2" \
    && pass "status: version" || failf "version: $out"
echo "$out" | grep -q "epoch 1" \
    && pass "status: epoch 1" || failf "epoch: $out"
echo "$out" | grep -q "pkg [0-9a-f]" \
    && pass "status: package_hash" || failf "pkg hash: $out"
echo "$out" | grep -q "deployed   by uid $(id -u)" \
    && pass "status: deployer uid" || failf "uid: $out"

# §2 strict transitions: current+requested in -501 errors
out=$("$BUILD/tplc-ctl" "$CTL" start 2>&1)
echo "$out" | grep -q "501.*current=1.*requested=start" \
    && pass "start-on-running → -501" || failf "start: $out"
out=$("$BUILD/tplc-ctl" "$CTL" stop 2>&1)
echo "$out" | grep -q "state 1 -> 0" \
    && pass "stop" || failf "stop: $out"
out=$("$BUILD/tplc-ctl" "$CTL" stop 2>&1)
echo "$out" | grep -q "501.*current=0.*requested=stop" \
    && pass "stop-on-stopped → -501" || failf "stop2: $out"
out=$("$BUILD/tplc-ctl" "$CTL" kill 2>&1)
echo "$out" | grep -q "501.*current=0.*requested=kill" \
    && pass "kill-on-stopped → -501" || failf "kill: $out"

# §4 events: a subscriber sees transitions with uuid + epoch
"$BUILD/tplc-ctl" "$CTL" start >/dev/null
timeout 5 "$BUILD/tplc-ctl" "$CTL" events 4 >"$WORK/ev.log" &
EVPID=$!
sleep 0.6
"$BUILD/tplc-ctl" "$CTL" stop >/dev/null
"$BUILD/tplc-ctl" "$CTL" start >/dev/null
wait $EVPID 2>/dev/null
grep -q "TRANSITION.*1->0.*epoch" "$WORK/ev.log" \
    && pass "event: run→stop transition" || failf "events: $(cat "$WORK/ev.log")"
grep -q "TRANSITION.*0->1.*epoch" "$WORK/ev.log" \
    && pass "event: stop→run transition" || failf "events2: $(cat "$WORK/ev.log")"

# journal tail returns durable audit records
"$BUILD/tplc-ctl" "$CTL" stop >/dev/null
sleep 0.2
out=$("$BUILD/tplc-ctl" "$CTL" journal-tail 4)
echo "$out" | grep -q "op=prog.stop" \
    && pass "journal tail has stop" || failf "journal: $out"
echo "$out" | grep -q "uid=$(id -u)" \
    && pass "journal identity" || failf "journal id: $out"

# epoch bumps on prepare (program-set change)
"$TPLCC" build "$WORK/a.st" --prog-version 1.4.3 \
    -o "$WORK/b.tplc" >/dev/null
out=$("$BUILD/tplc-ctl" "$CTL" prepare "$WORK/b.tplc" 2>&1)
echo "$out" | grep -q "state 3 -> 0" \
    && pass "prepare accepted" || failf "prepare: $out"
out=$("$BUILD/tplc-ctl" "$CTL" prog)
echo "$out" | grep -q "epoch 2" \
    && pass "prepare bumps epoch→2" || failf "epoch2: $out"
echo "$out" | grep -q "version    1.4.3" \
    && pass "prepare updates version" || failf "ver2: $out"
echo "$out" | grep -q "pred [0-9a-f][0-9a-f]" \
    && pass "predecessor recorded" || failf "pred: $out"

# 14-char path = 16-byte payload: must decode as {u16 len, path},
# not as a program uuid
"$TPLCC" build "$WORK/a.st" -o "$WORK/c.tplc" >/dev/null
ln -sf "$WORK/c.tplc" /tmp/io_N.tplc
out=$("$BUILD/tplc-ctl" "$CTL" prepare /tmp/io_N.tplc 2>&1)
echo "$out" | grep -q "state 3 -> 0" \
    && pass "14-char path decodes as path" || failf "shortpath: $out"
rm -f /tmp/io_N.tplc

# map.get with no DEPLOY_MAP section reports empty
out=$("$BUILD/tplc-ctl" "$CTL" map-get)
echo "$out" | grep -q "no deploy map" \
    && pass "map-get empty" || failf "map-get: $out"

"$BUILD/tplc-ctl" "$CTL" shutdown >/dev/null 2>&1 || true

echo "== mgmt_surface: $fail failures"
[ "$fail" -eq 0 ]
