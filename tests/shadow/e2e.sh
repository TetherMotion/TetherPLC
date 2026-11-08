#!/usr/bin/env bash
# shadow-run conformance (doc 19 §7): candidate loads on identical
# inputs, never steers %Q, diffs are reported, promote is atomic and
# state-preserving, lifecycle ops drop the shadow.
# Requires: build tree + tplcc (exit 77 = prerequisites missing -> CTest skips).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_shadow_e2e
SHM=/dev/shm$IMG
CTL=/tmp/tplc_shadow_e2e.ctl
WORK=$(mktemp -d)
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG"' 2>/dev/null; rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

# little-endian u32 at %Q byte offset $1 (header 256 + %I 64 = %Q base)
qd() { python3 -c "import sys;d=open('$SHM','rb').read();o=256+64+$1;print(int.from_bytes(d[o:o+4],'little'))" 2>/dev/null; }

# active: counter drives %QD4
cat > $WORK/active.st <<'EOF'
PROGRAM Demo
VAR
    out_cnt  AT %QD4 : DINT;
    cnt  : DINT := 0;
END_VAR
cnt := cnt + 1;
out_cnt := cnt;
END_PROGRAM
EOF
# shadow candidate: same VAR layout, doubles the output — any %Q
# steering from the shadow is immediately visible as doubled counts
cat > $WORK/cand.st <<'EOF'
PROGRAM Demo
VAR
    out_cnt  AT %QD4 : DINT;
    cnt  : DINT := 0;
END_VAR
cnt := cnt + 1;
out_cnt := 2 * cnt;
END_PROGRAM
EOF

if [ -z "${LLVM_SYS_211_PREFIX:-}" ]; then
    for p in /home/uli/llvm21/usr /opt/llvm21 /usr/lib/llvm-21; do
        if [ -x "$p/bin/llvm-config-21" ] || [ -x "$p/bin/llvm-config" ]; then
            export LLVM_SYS_211_PREFIX=$p
            export PATH="$p/bin:$PATH"
            break
        fi
    done
fi
(cd compiler && cargo build -q -p tplcc 2>/dev/null) || true
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] || { echo "SKIP: no build tree"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP: no python3"; exit 77; }

$TPLCC build $WORK/active.st -o $WORK/active.tplc >/dev/null \
    || { echo "active build failed"; exit 1; }
$TPLCC build $WORK/cand.st -o $WORK/cand.tplc >/dev/null \
    || { echo "candidate build failed"; exit 1; }

$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --package $WORK/active.tplc --ctl $CTL \
    --quarantine-cycles 100000 >$WORK/rt.log 2>&1 &
RT_PID=$!
sleep 1

# §7.9: SHADOW_LOAD on a stopped program is illegal
$BUILD/tplc-ctl $CTL stop >/dev/null
r=$($BUILD/tplc-ctl $CTL shadow-load $WORK/cand.tplc 2>&1)
echo "$r" | grep -q "error 501" \
    && ok "shadow-load on stopped refused (-501)" \
    || bad "shadow-load on stopped ($r)"
$BUILD/tplc-ctl $CTL start >/dev/null
sleep 0.3

# empty-state + load
s=$($BUILD/tplc-ctl $CTL shadow-status 2>&1)
echo "$s" | grep -q "none" && ok "shadow-status empty" \
    || bad "shadow-status empty ($s)"
$BUILD/tplc-ctl $CTL shadow-load $WORK/cand.tplc >/dev/null \
    || bad "shadow-load"
sleep 0.5
s=$($BUILD/tplc-ctl $CTL shadow-status 2>&1)
echo "$s" | grep -q "state 1" && echo "$s" | grep -qE "cycles[[:space:]]+[1-9]" \
    && ok "shadow running (cycles advancing)" \
    || bad "shadow-status ($s)"

# §7.3 compare accuracy: candidate doubles, so %QD4 diverges every
# co-run; img offset 4 is the declared channel
c=$($BUILD/tplc-ctl $CTL shadow-compare 8 2>&1)
echo "$c" | grep -q "img+0x04" \
    && ok "shadow-compare reports %QD4 divergence" \
    || bad "shadow-compare ($c)"

# §7.1 no steering: image %QD4 keeps the active's ramp (cnt), not
# the candidate's doubled value — the two compare streams prove
# which writer is live
a1=$(qd 4); sleep 0.4; a2=$(qd 4); ramp=$((a2 - a1))
[ "$ramp" -gt 0 ] && [ "$ramp" -lt 2000 ] \
    && ok "image output = active ramp ($ramp/0.4s, not doubled)" \
    || bad "image steering ($a1 -> $a2)"

# §3.2 debug selector: resolve + dbg-read target the shadow blob.
# out_cnt diverges deterministically (cnt vs 2*cnt); cnt itself is
# a bad probe — migration copies it and both sides keep it in step
off=$($BUILD/tplc-ctl $CTL resolve SHADOW.out_cnt 2>&1 \
    | grep -oE "off [0-9]+" | awk '{print $2}')
if [ -n "$off" ]; then
    av=$($BUILD/tplc-ctl $CTL dbg-read $off 4 2>&1 | tail -1)
    sv=$($BUILD/tplc-ctl $CTL dbg-read $off 4 1 2>&1 | tail -1)
    [ "$av" != "$sv" ] \
        && ok "dbg-read sel=1 reads shadow blob" \
        || bad "dbg-read sel ($av vs $sv)"
else
    bad "SHADOW.out_cnt resolve"
fi

# watch: SHADOW.<sym> item samples the shadow blob
w=$($BUILD/tplc-ctl $CTL watch out_cnt SHADOW.out_cnt 2>&1)
echo "$w" | grep -qE "handle|h=" || true
g=$($BUILD/tplc-ctl $CTL watch-get 1 2>&1)
echo "$g" | grep -q "4 B" \
    && ok "SHADOW. watch item arms" \
    || bad "SHADOW. watch ($w / $g)"

# shadow-reset: migrate result, then diffs continue (2*cnt diverges)
r=$($BUILD/tplc-ctl $CTL shadow-reset 2>&1)
echo "$r" | grep -q "out_status=0" && ok "shadow-reset migrates" \
    || bad "shadow-reset ($r)"

# §7.4 atomic promote: state preserved — the adopted program's cnt
# keeps counting, and %Q now carries the doubled value
c_before=$(qd 4)
$BUILD/tplc-ctl $CTL shadow-promote >$WORK/promote.out 2>&1 \
    || bad "shadow-promote ($(cat $WORK/promote.out))"
grep -q "promoted" $WORK/promote.out \
    && ok "promote commits (uuid+epoch reply)" \
    || bad "promote reply ($(cat $WORK/promote.out))"
grep -q "shadow promoted" $WORK/rt.log \
    && ok "promote logged" || bad "promote log missing"
sleep 0.3
p1=$(qd 4); sleep 0.4; p2=$(qd 4); ramp_pm=$((p2 - p1))
# adopted program writes 2*cnt: ramp roughly doubles vs the active's
[ "$ramp_pm" -gt "$((ramp * 3 / 2))" ] && \
    [ "$ramp_pm" -lt "$((ramp * 5 / 2 + 500))" ] \
    && ok "post-promote output = shadow's doubled ramp ($ramp_pm)" \
    || bad "post-promote ramp ($ramp vs $ramp_pm)"
p=$($BUILD/tplc-ctl $CTL prog 2>&1)
echo "$p" | grep -q "quarantine" \
    && echo "$p" | grep -qvE "shadow.*running" \
    && ok "post-promote: no shadow, predecessor quarantined" \
    || bad "post-promote status"
s=$($BUILD/tplc-ctl $CTL shadow-status 2>&1)
echo "$s" | grep -q "none" && ok "shadow slot freed post-promote" \
    || bad "shadow-status post-promote ($s)"

# §7.5 rollback inside the quarantine window restores the pre-
# promotion program — output returns to the single-ramp writer
$BUILD/tplc-ctl $CTL rollback >/dev/null \
    && grep -q "rollback restored" $WORK/rt.log \
    && ok "rollback restores pre-promotion program" || bad "rollback"
sleep 0.3
b1=$(qd 4); sleep 0.4; b2=$(qd 4); ramp2=$((b2 - b1))
[ "$ramp2" -gt 0 ] && [ "$ramp2" -lt "$((ramp_pm * 3 / 4))" ] \
    && ok "rollback output = original ramp" \
    || bad "rollback output ($b1 -> $b2)"

# lifecycle: a swap drops the shadow explicitly
$BUILD/tplc-ctl $CTL shadow-load $WORK/cand.tplc >/dev/null \
    || bad "shadow-load for lifecycle"
$BUILD/tplc-ctl $CTL swap $WORK/cand.tplc >/dev/null \
    || bad "swap for lifecycle"
s=$($BUILD/tplc-ctl $CTL shadow-status 2>&1)
echo "$s" | grep -q "none" \
    && ok "swap drops shadow" || bad "swap shadow ($s)"

# drop + audit
$BUILD/tplc-ctl $CTL shadow-load $WORK/cand.tplc >/dev/null \
    || bad "shadow-load for drop"
$BUILD/tplc-ctl $CTL shadow-drop >/dev/null \
    || bad "shadow-drop"
s=$($BUILD/tplc-ctl $CTL shadow-status 2>&1)
echo "$s" | grep -q "none" && ok "shadow-drop frees slot" \
    || bad "shadow-drop ($s)"
grep -q "shadow.drop" $WORK/rt.log && ok "drop journaled" \
    || bad "drop journal"

# §7.8 authorization: an observer can read shadow status but every
# mutating shadow op is commissioner-only
kill "$RT_PID" 2>/dev/null; wait "$RT_PID" 2>/dev/null; RT_PID=
echo "$(id -u) observer" > $WORK/authz
$BUILD/tether-plc --image $IMG --package $WORK/active.tplc --ctl $CTL \
    --authz $WORK/authz >$WORK/rt2.log 2>&1 &
RT_PID=$!
sleep 1
$BUILD/tplc-ctl $CTL start >/dev/null 2>&1; sleep 0.2
r=$($BUILD/tplc-ctl $CTL shadow-status 2>&1)
echo "$r" | grep -q "none\|state" && ok "observer reads shadow status" \
    || bad "observer shadow-status ($r)"
r=$($BUILD/tplc-ctl $CTL shadow-load $WORK/cand.tplc 2>&1)
echo "$r" | grep -q "error 401" && ok "observer shadow-load denied" \
    || bad "observer shadow-load ($r)"
r=$($BUILD/tplc-ctl $CTL shadow-drop 2>&1)
echo "$r" | grep -q "error 401" && ok "observer shadow-drop denied" \
    || bad "observer shadow-drop ($r)"
kill "$RT_PID" 2>/dev/null; wait "$RT_PID" 2>/dev/null; RT_PID=

# §7.6 budget: with a budget far beyond the scan slack every due
# shadow tick is skipped and counted, the active scan is untouched
$BUILD/tplc-image unlink $IMG 2>/dev/null
$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --package $WORK/active.tplc --ctl $CTL \
    --shadow-budget-ns 999999999999 >$WORK/rt3.log 2>&1 &
RT_PID=$!
sleep 1
$BUILD/tplc-ctl $CTL start >/dev/null 2>&1
$BUILD/tplc-ctl $CTL shadow-load $WORK/cand.tplc >/dev/null 2>&1 \
    || bad "shadow-load for missed"
sleep 0.5
s=$($BUILD/tplc-ctl $CTL shadow-status 2>&1)
echo "$s" | grep -qE "missed[[:space:]]+[1-9]" \
    && ok "missed cycles counted under impossible budget" \
    || bad "missed counter ($s)"
echo "$s" | grep -qE "cycles[[:space:]]+0([^0-9]|$)" \
    && ok "shadow never ran over budget" \
    || bad "cycles under impossible budget ($s)"

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
