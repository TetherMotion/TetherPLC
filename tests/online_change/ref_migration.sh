#!/usr/bin/env bash
# ref_migration.sh — REFERENCE TO and online change: a reference stores
# a blob offset, so it cannot survive a layout change. The compiler
# must warn at --change-against time, the migrate plan must reset the
# field (dropped, not copied), and the runtime swap must log it.
# Requires: build tree + tplcc (exit 77 = prerequisites missing).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_refmig
CTL=/tmp/tplc_refmig.ctl
WORK=$(mktemp -d)
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG"' 2>/dev/null; rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tplc-ctl" ] || { echo "SKIP runtime"; exit 77; }
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP no tplcc"; exit 77; }

# v1: a reference bound to cnt, dereferenced every scan
cat > $WORK/r1.st <<'EOF'
PROGRAM Demo
VAR
    cnt     : DINT := 0;
    r       : REFERENCE TO DINT;
    out_cnt AT %QD0 : DINT;
    out_val AT %QD4 : DINT;
END_VAR
cnt := cnt + 1;
r REF= cnt;
out_cnt := cnt;
out_val := r;
END_PROGRAM
EOF
# v2: same reference, plus a field that shifts the layout
cat > $WORK/r2.st <<'EOF'
PROGRAM Demo
VAR
    cnt     : DINT := 0;
    r       : REFERENCE TO DINT;
    extra   : DINT := 7;
    out_cnt AT %QD0 : DINT;
    out_val AT %QD4 : DINT;
END_VAR
cnt := cnt + 1;
r REF= cnt;
out_cnt := cnt;
out_val := r + extra;
END_PROGRAM
EOF

"$TPLCC" build $WORK/r1.st -o $WORK/r1.tplc >/dev/null 2>&1 \
    || { echo "SKIP v1 build"; exit 77; }
"$TPLCC" build $WORK/r2.st -o $WORK/r2.tplc \
    --change-against $WORK/r1.tplc 2>$WORK/warn.txt \
    || bad "v2 change-against compile"
grep -q 'reference field' $WORK/warn.txt \
    && ok "change-against warns about the reference field" \
    || bad "no per-field reference warning: $(cat $WORK/warn.txt)"
grep -q 'referenced layout moved' $WORK/warn.txt \
    && ok "change-against says the referenced layout moved" \
    || bad "no layout-moved reason: $(cat $WORK/warn.txt)"

# the migrate plan must reset the ref, not copy a stale offset
"$BUILD"/tplc-image create "$IMG" 64 64 >/dev/null 2>&1
"$BUILD"/tether-plc --image "$IMG" --package $WORK/r1.tplc \
    --ctl "$CTL" --no-sandbox >"$WORK/rt.log" 2>&1 &
RT_PID=$!
for i in $(seq 1 50); do [ -S "$CTL" ] && break; sleep 0.1; done
[ -S "$CTL" ] || { echo "SKIP ctl socket"; exit 77; }

"$BUILD"/tplc-ctl "$CTL" swap $WORK/r2.tplc >"$WORK/swap.txt" 2>&1 \
    || bad "swap to v2: $(cat $WORK/swap.txt)"
grep -q 'reference field' "$WORK/rt.log" \
    && ok "swap logs the reference reset" \
    || bad "swap log lacks reference reset: $(tail -3 "$WORK/rt.log")"

# the reset ref is null in the new program until re-armed: the swap
# must not leave a stale offset pointing into the old layout
sleep 0.4
state=$("$BUILD"/tplc-ctl "$CTL" prog 2>&1 | head -2)
echo "$state" | grep -q 'state running' \
    && ok "program still running after the ref reset" \
    || bad "program not running: $state"

kill "$RT_PID" 2>/dev/null; RT_PID=
wait 2>/dev/null

# -- preserved: append-only change keeps every existing offset ---------
cat > $WORK/r1p.st <<'EOF'
PROGRAM Demo
VAR
    cnt     : DINT := 0;
    r       : REFERENCE TO DINT;
    out_cnt AT %QD0 : DINT;
    out_val AT %QD4 : DINT;
    pad     : DINT := 0;
END_VAR
cnt := cnt + 1;
r REF= cnt;
out_cnt := cnt;
out_val := r + pad;
END_PROGRAM
EOF
"$TPLCC" build $WORK/r1p.st -o $WORK/r1p.tplc \
    --change-against $WORK/r1.tplc 2>$WORK/warn_p.txt \
    || bad "append-only change-against compile"
grep -q 'preserved (referenced layout unchanged)' $WORK/warn_p.txt \
    && ok "change-against preserves refs when the layout is unchanged" \
    || bad "no preserve note: $(cat $WORK/warn_p.txt)"
grep -q 'reset to null' $WORK/warn_p.txt \
    && bad "append-only change should not reset refs" \
    || ok "append-only change resets nothing"

"$BUILD"/tplc-image create "$IMG" 64 64 >/dev/null 2>&1
"$BUILD"/tether-plc --image "$IMG" --package $WORK/r1.tplc \
    --ctl "$CTL" --no-sandbox >"$WORK/rt_p.log" 2>&1 &
RT_PID=$!
for i in $(seq 1 50); do [ -S "$CTL" ] && break; sleep 0.1; done
"$BUILD"/tplc-ctl "$CTL" swap $WORK/r1p.tplc >"$WORK/swap_p.txt" 2>&1 \
    || bad "append-only swap: $(cat $WORK/swap_p.txt)"
grep -q '1 reference field(s) preserved' "$WORK/rt_p.log" \
    && ok "swap reports the preserved reference" \
    || bad "swap log lacks preserve report: $(tail -3 "$WORK/rt_p.log")"

kill "$RT_PID" 2>/dev/null; RT_PID=
wait 2>/dev/null
echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
