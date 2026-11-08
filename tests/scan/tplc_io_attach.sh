#!/usr/bin/env bash
# tplc_io_attach.sh — live attach over tether_io (TCP):
# tplc-io bridges ctl.sock into a tether_io server; tplc-io-probe
# drives discovery, live signals, typed function calls, debug
# reads/writes/forces/watches/task control and lifecycle
# transitions. Skips when the provider or tplcc is absent.
set -uo pipefail
cd "$(dirname "$0")/../.."
ROOT="$PWD"
BUILD="${BUILD:-$ROOT/build}"
TPLCC="${TPLCC:-$ROOT/compiler/target/debug/tplcc}"

for f in "$BUILD/tether-plc" "$BUILD/tplc-image" "$BUILD/tplc-io" \
         "$BUILD/tplc-io-probe"; do
    [ -f "$f" ] || { echo "SKIP: $f missing"; exit 77; }
done
[ -x "$TPLCC" ] || { echo "SKIP: tplcc not built"; exit 77; }

WORK="$(mktemp -d /tmp/tplc_io.XXXXXX)"
IMG=tplc_io_test_img
CTL="$WORK/io.ctl"
PORT=$((44000 + RANDOM % 5000))
RTPID=""
IOPID=""
trap 'rm -rf "$WORK";
      [ -n "$IOPID" ] && kill "$IOPID" 2>/dev/null;
      [ -n "$RTPID" ] && kill "$RTPID" 2>/dev/null;
      "$BUILD"/tplc-image unlink "$IMG" 2>/dev/null' EXIT

# program with a counter + two IO-bound bools
cat > "$WORK/a.st" <<'EOF'
PROGRAM P
VAR
    n : INT;
    x AT %IX0.0 : BOOL;
    y AT %QX0.0 : BOOL;
END_VAR
n := n + 1;
y := x;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/a.st" -o "$WORK/a.tplc" >/dev/null \
    || { echo "SKIP: build failed"; exit 77; }

"$BUILD/tplc-image" create "$IMG" 64 64 >/dev/null 2>&1
"$BUILD/tether-plc" --image "$IMG" --package "$WORK/a.tplc" \
    --ctl "$CTL" >"$WORK/rt.log" 2>&1 &
RTPID=$!
for i in $(seq 1 40); do [ -S "$CTL" ] && break; sleep 0.1; done
[ -S "$CTL" ] || { echo "FAIL runtime up: $(cat "$WORK/rt.log")"; exit 1; }

"$BUILD/tplc-io" --ctl "$CTL" --bind 127.0.0.1 --port "$PORT" \
    >"$WORK/io.log" 2>&1 &
IOPID=$!
sleep 1
kill -0 "$IOPID" 2>/dev/null \
    || { echo "FAIL provider died: $(cat "$WORK/io.log")"; exit 1; }
grep -q "tether_io on" "$WORK/io.log" \
    || { echo "FAIL provider listen: $(cat "$WORK/io.log")"; exit 1; }

"$BUILD/tplc-io-probe" "$PORT" || { echo "FAIL probe"; exit 1; }

echo "tplc_io_attach: all checks passed"
