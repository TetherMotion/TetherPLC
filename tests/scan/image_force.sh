#!/usr/bin/env bash
# image-channel force path (M8): DBG_FORCE on an AT-bound %I BOOL is
# translated through the io bindings into the latched input image, so
# it reaches the program field every scan (a one-shot DBG_WRITE would
# be overwritten by the next image latch). A force on %Q is stamped
# into the output image before commit. Non-BOOL bindings are denied
# (TPLC_E_FORCE_DENIED) — only BOOL channels are forceable today.
# Skips when tplcc/build tree is missing.
set -uo pipefail
cd "$(dirname "$0")/../.."
ROOT="$PWD"
BUILD="${BUILD:-$ROOT/build}"
TPLCC="${TPLCC:-$ROOT/compiler/target/debug/tplcc}"

for f in "$BUILD/tether-plc" "$BUILD/tplc-image" "$BUILD/tplc-ctl"; do
    [ -f "$f" ] || { echo "SKIP: $f missing"; exit 77; }
done
[ -x "$TPLCC" ] || { echo "SKIP: tplcc not built"; exit 77; }

WORK="$(mktemp -d /tmp/tplc_imgf.XXXXXX)"
IMG=imgf_test_img
CTL="$WORK/imgf.ctl"
PID=""
trap 'rm -rf "$WORK";
      [ -n "$PID" ] && kill "$PID" 2>/dev/null;
      "$BUILD"/tplc-image unlink "$IMG" 2>/dev/null' EXIT

fail=0
pass() { echo "PASS $1"; }
failf(){ echo "FAIL $1"; fail=$((fail+1)); }

# first byte of the %Q image area from tplc-image dump
qbyte() {
    "$BUILD/tplc-image" dump "$IMG" --in 0 --out 16 2>/dev/null \
        | awk '/%Q \(first/{getline; print $2; exit}'
}

# di %IX0.0 -> q %QX0.0; ai %ID16.0 is an analog (non-forceable) channel
cat > "$WORK/prog.st" <<'EOF'
PROGRAM P
VAR
    di AT %IX0.0 : BOOL;
    q  AT %QX0.0 : BOOL;
    ai AT %ID16.0 : INT;
END_VAR
q := di;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/prog.st" -o "$WORK/prog.tplc" >/dev/null \
    || { echo "SKIP: tplcc compile failed"; exit 77; }

"$BUILD/tplc-image" create "$IMG" 64 64 >/dev/null
"$BUILD/tether-plc" --image "$IMG" --package "$WORK/prog.tplc" \
    --ctl "$CTL" >"$WORK/run.log" 2>&1 &
PID=$!
for i in $(seq 1 40); do [ -S "$CTL" ] && break; sleep 0.1; done
[ -S "$CTL" ] || { echo "FAIL runtime did not come up: $(cat "$WORK/run.log")"; exit 1; }
sleep 0.3   # let a few scans run with all inputs zero

# resolve the AT vars to blob offsets ("type N off N size N" line)
DIOFF=$("$BUILD/tplc-ctl" "$CTL" resolve di | sed -n 's/.*off \([0-9]*\).*/\1/p')
QOFF=$("$BUILD/tplc-ctl" "$CTL" resolve q  | sed -n 's/.*off \([0-9]*\).*/\1/p')
AIOFF=$("$BUILD/tplc-ctl" "$CTL" resolve ai | sed -n 's/.*off \([0-9]*\).*/\1/p')
[ -n "$DIOFF" ] && [ -n "$QOFF" ] && [ -n "$AIOFF" ] \
    || { echo "FAIL resolve: di=$DIOFF q=$QOFF ai=$AIOFF"; exit 1; }
pass "resolved di=$DIOFF q=$QOFF ai=$AIOFF"

# baseline: everything zero
"$BUILD/tplc-ctl" "$CTL" dbg-read "$QOFF" 1 | grep -q "00" \
    && pass "baseline q=0" || failf "baseline q"

# --- stimulate the DI: force di=1 -------------------------------------
out=$("$BUILD/tplc-ctl" "$CTL" force "$DIOFF" 01 2>&1)
echo "$out" | grep -q "forced" \
    && pass "force di accepted" || failf "force di: $out"
sleep 0.3
"$BUILD/tplc-ctl" "$CTL" dbg-read "$QOFF" 1 | grep -q "01" \
    && pass "forced di=1 -> q field=1" || failf "q field after di force"
[ "$(qbyte)" = "01" ] \
    && pass "forced di=1 -> %Q image bit0 set" || failf "%Q image after di force (got $(qbyte))"

# --- release: next latch restores the real (zero) input ----------------
"$BUILD/tplc-ctl" "$CTL" force-clear all >/dev/null
sleep 0.3
"$BUILD/tplc-ctl" "$CTL" dbg-read "$QOFF" 1 | grep -q "00" \
    && pass "force cleared -> q back to 0" || failf "q after clear"

# --- preset the DO: force q=1 while di stays 0 -------------------------
out=$("$BUILD/tplc-ctl" "$CTL" force "$QOFF" 01 2>&1)
echo "$out" | grep -q "forced" \
    && pass "force q accepted" || failf "force q: $out"
sleep 0.3
[ "$(qbyte)" = "01" ] \
    && pass "forced q=1 -> %Q image bit0 set (override)" \
    || failf "%Q image after q force (got $(qbyte))"
"$BUILD/tplc-ctl" "$CTL" force-clear all >/dev/null
sleep 0.3
[ "$(qbyte)" = "00" ] \
    && pass "q force cleared -> image restores" || failf "image after q clear (got $(qbyte))"

# --- analog channel is not forceable ------------------------------------
out=$("$BUILD/tplc-ctl" "$CTL" force "$AIOFF" 0000a041 2>&1)
echo "$out" | grep -q "414" \
    && pass "force analog -> FORCE_DENIED" || failf "force ai: $out"

echo "== $([ $fail -eq 0 ] && echo ALL PASS || echo "$fail FAILURES")"
[ "$fail" -eq 0 ]
