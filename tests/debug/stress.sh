#!/usr/bin/env bash
# debug-plane conformance (doc 12 §10.3): a task writes a 64-bit
# {lo,hi} pair each run; every boundary-consistent read must show
# lo == hi — a torn sample would mix two runs and fail the check.
# Requires: build tree + tplcc + python3 (exit 77 skips).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_dbg_stress
SHM=/dev/shm$IMG
CTL=/tmp/tplc_dbg_stress.ctl
WORK=$(mktemp -d)
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG"' 2>/dev/null; rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

if [ -z "${LLVM_SYS_211_PREFIX:-}" ]; then
    for p in /home/uli/llvm21/usr /opt/llvm21 /usr/lib/llvm-21; do
        if [ -x "$p/bin/llvm-config-21" ] || [ -x "$p/bin/llvm-config" ]; then
            export LLVM_SYS_211_PREFIX=$p
            export PATH="$p/bin:$PATH"
            break
        fi
    done
fi
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] || { echo "SKIP: no build tree"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP: no python3"; exit 77; }

cat > $WORK/pair.st <<'ST'
PROGRAM P
VAR
    i : LINT := 0;
    pair_lo : LINT := 0;
    pair_hi : LINT := 0;
END_VAR
i := i + 1;
pair_lo := i;
pair_hi := i;
END_PROGRAM
ST

$TPLCC build $WORK/pair.st -o $WORK/pair.tplc >/dev/null \
    || { echo "pair build failed"; exit 1; }

$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --package $WORK/pair.tplc \
    --ctl $CTL >$WORK/rt.log 2>&1 &
RT_PID=$!
sleep 1

# named watch over both halves: one GET samples the pair at a single
# task boundary
$BUILD/tplc-ctl $CTL watch pair_lo pair_hi >/dev/null || bad "watch arm"

torn=0; sampled=0
for n in $(seq 1 400); do
    g=$($BUILD/tplc-ctl $CTL watch-get 1 2>/dev/null)
    lo=$(echo "$g" | awk '/\[0\]/{print $4}')
    hi=$(echo "$g" | awk '/\[1\]/{print $4}')
    [ -n "$lo" ] && [ -n "$hi" ] || continue
    sampled=$((sampled+1))
    [ "$lo" = "$hi" ] || torn=$((torn+1))
done
[ "$torn" -eq 0 ] && [ "$sampled" -gt 100 ] \
    && ok "torn-read stress: $sampled boundary samples, 0 torn" \
    || bad "torn-read stress (torn=$torn sampled=$sampled)"

# same check through raw dbg-read on the pair's blob range
lo_off=$($BUILD/tplc-ctl $CTL resolve pair_lo | awk '/^type/{print $4}')
hi_off=$($BUILD/tplc-ctl $CTL resolve pair_hi | awk '/^type/{print $4}')
[ -n "$lo_off" ] && [ -n "$hi_off" ] || bad "resolve pair"
lo0=$((lo_off)); hi0=$((hi_off))
base=$((lo0 < hi0 ? lo0 : hi0))
torn=0; sampled=0
for n in $(seq 1 200); do
    h=$($BUILD/tplc-ctl $CTL dbg-read $base 16 2>/dev/null | awk '{print $2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17}')
    [ -n "$h" ] || continue
    lo=$(echo "$h" | awk '{print $1,$2,$3,$4,$5,$6,$7,$8}' | tr -d ' ')
    hi=$(echo "$h" | awk '{print $9,$10,$11,$12,$13,$14,$15,$16}' | tr -d ' ')
    sampled=$((sampled+1))
    [ "$lo" = "$hi" ] || torn=$((torn+1))
done
[ "$torn" -eq 0 ] && [ "$sampled" -gt 50 ] \
    && ok "dbg-read pair: $sampled samples, 0 torn" \
    || bad "dbg-read pair (torn=$torn sampled=$sampled)"

# jitter under debug load (doc 12 §10.2): flood the ctl channel from
# parallel connections while the cyclic task keeps releasing; cadence
# and lateness must stay bounded.
task_stat() { $BUILD/tplc-ctl $CTL prog | awk '$1=="task"{print $11,$13,$15}'; }

read r0 l0 m0 < <(task_stat)
sleep 1
read r1 l1 m1 < <(task_stat)
base_rate=$((r1 - r0))
[ "$base_rate" -gt 50 ] && ok "baseline cadence $base_rate runs/s" \
    || bad "baseline cadence ($base_rate)"

flood() { local end=$((SECONDS + 4)); while [ $SECONDS -lt $end ]; do
    $BUILD/tplc-ctl $CTL dbg-read 0 64 >/dev/null 2>&1
    $BUILD/tplc-ctl $CTL prog      >/dev/null 2>&1
done; }
fpids=
for w in 1 2 3 4 5 6; do flood & fpids="$fpids $!"; done
read rf0 lf0 _ < <(task_stat)
sleep 4
read rf1 lf1 mf1 < <(task_stat)
wait $fpids   # only the flood workers; RT_PID never exits

druns=$((rf1 - rf0))
# 4 s at 10 ms period: ~400 runs. Cadence must hold within 25% and the
# worst release lateness must stay far under one period (10 ms), so
# no release is ever skipped.
[ "$druns" -ge 300 ] && ok "cadence under flood: $druns runs/4s" \
    || bad "cadence under flood ($druns runs)"
[ "$mf1" -lt 5000000 ] \
    && ok "lateness under flood: max ${mf1} ns (< 5 ms)" \
    || bad "lateness under flood (max ${mf1} ns)"

# sim parity (doc 12 §10.7): an identical release schedule must
# produce identical watch values in --sim and in realtime.
cat > $WORK/par.st <<'ST'
PROGRAM P
VAR
    i : DINT := 0;
    out AT %QB0 : BYTE;
END_VAR
i := i + 1;
out := out + 1;
END_PROGRAM
ST

$TPLCC build $WORK/par.st -o $WORK/par.tplc >/dev/null \
    || bad "parity build"

# realtime: replace with the parity program (stop/unload/prepare/
# start; swap needs a migrate path), suspend exactly at i == 51,
# then read the armed watch
$BUILD/tplc-ctl $CTL stop >/dev/null
$BUILD/tplc-ctl $CTL unload >/dev/null
$BUILD/tplc-ctl $CTL prepare $WORK/par.tplc >/dev/null
$BUILD/tplc-ctl $CTL start >/dev/null
sleep 0.3
rwh=$($BUILD/tplc-ctl $CTL watch i | awk '{print $3}')
$BUILD/tplc-ctl $CTL breakx 0 'i == 51' >/dev/null
for t in $(seq 1 50); do
    $BUILD/tplc-ctl $CTL prog | awk '$1=="task"&&$6=="susp"{f=1}END{exit !f}' \
        && break
    sleep 0.1
done
ri=$($BUILD/tplc-ctl $CTL watch-get $rwh | awk '/\[0\]/{print $4}')
[ "$ri" = "33000000" ] && ok "realtime watch i=51" \
    || bad "realtime watch i ($ri)"

# sim: 50 releases at 10 ms; first_run at t=0 plus 50 steps = 51 runs
{ for k in $(seq 1 50); do
    ex=""; [ $k -eq 50 ] && ex=" expect 0=33"
    printf 'step %d time=%d%s\n' $k $((k * 10000000)) "$ex"
done; } > $WORK/par.steps

SIMCTL=/tmp/tplc_dbg_par.ctl
rm -f $SIMCTL
$BUILD/tether-plc-sim --image /tplc_dbg_par --package $WORK/par.tplc \
    --steps $WORK/par.steps --plc $BUILD/tether-plc \
    --ctl $SIMCTL --settle-ms 2500 --hold-ms 6000 \
    >$WORK/sim.log 2>&1 &
simpid=$!
for t in $(seq 1 50); do [ -S $SIMCTL ] && break; sleep 0.1; done
swh=$($BUILD/tplc-ctl $SIMCTL watch i | awk '{print $3}')
sleep 3.5
si=$($BUILD/tplc-ctl $SIMCTL watch-get $swh | awk '/\[0\]/{print $4}')
wait $simpid
[ "$si" = "33000000" ] && ok "sim watch i=51 (parity)" \
    || bad "sim watch i ($si, want 33000000 = real $ri)"
grep -q 'all matched' $WORK/sim.log && ok "sim expect out=0x33" \
    || bad "sim expect mismatch ($(tail -2 $WORK/sim.log))"

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
