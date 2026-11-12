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
# shadow candidate: same layout, divergent logic (n doubles)
cat > "$WORK/c.st" <<'EOF'
PROGRAM P
VAR
    n : INT;
    x AT %IX0.0 : BOOL;
    y AT %QX0.0 : BOOL;
END_VAR
n := n + 1;
n := n * 2;
y := x;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/c.st" -o "$WORK/c.tplc" >/dev/null \
    || { echo "SKIP: candidate build failed"; exit 77; }

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

"$BUILD/tplc-io-probe" "$PORT" "$WORK/c.tplc" > "$WORK/probe1.log" \
    || { echo "FAIL probe: $(cat "$WORK/probe1.log")"; exit 1; }
cat "$WORK/probe1.log"
EP1=$(sed -n 's/info schema_epoch=\([0-9]*\)/\1/p' "$WORK/probe1.log")

# epoch bridge: a program-set change must bump the schema epoch and
# add an epoch.N marker — verified through a second probe session
cat > "$WORK/b.st" <<'EOF'
PROGRAM P
VAR
    n : INT;
    x AT %IX0.0 : BOOL;
    y AT %QX0.0 : BOOL;
END_VAR
n := n + 2;
y := x;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/b.st" -o "$WORK/b_long_name.tplc" >/dev/null \
    || { echo "FAIL build b"; exit 1; }
"$BUILD/tplc-ctl" "$CTL" stop >/dev/null 2>&1
"$BUILD/tplc-ctl" "$CTL" prepare "$WORK/b_long_name.tplc" \
    >/dev/null 2>&1 || { echo "FAIL prepare"; exit 1; }
"$BUILD/tplc-ctl" "$CTL" start >/dev/null 2>&1
sleep 1
"$BUILD/tplc-io-probe" "$PORT" > "$WORK/probe2.log" 2>&1 \
    || { echo "FAIL probe2: $(cat "$WORK/probe2.log")"; exit 1; }
# schema epoch = count of program-set changes seen by the watcher.
# probe1 prints the epoch *before* its own shadow.promote, so two
# bumps happen between EP1 and probe2: the promote and the prepare.
want=$(( ${EP1:-1} + 2 ))
grep -q "info schema_epoch=$want" "$WORK/probe2.log" \
    || { echo "FAIL epoch bump ($EP1->$want): $(head -3 "$WORK/probe2.log")"; exit 1; }
grep -q "catalog bumped" "$WORK/io.log" \
    || { echo "FAIL catalog bump log"; exit 1; }

echo "tplc_io_attach: all checks passed"
