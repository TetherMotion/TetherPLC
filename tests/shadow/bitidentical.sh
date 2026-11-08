#!/usr/bin/env bash
# shadow-run §7.1: the shadow must not perturb the active program.
# Replay the same input tape through tether-plc-sim --scan twice —
# once solo, once with a divergent-logic candidate shadowed — and
# require byte-identical %Q dumps at every step.
# Requires: build tree + tplcc (exit 77 = prerequisites missing -> CTest skips).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

# active steers several %Q regions from inputs + state so a stray
# shadow write, latch leak or commit leak shows up in the diff
cat > $WORK/active.st <<'EOF'
PROGRAM BiProg
VAR
    in1  AT %IB0 : BYTE;
    in2  AT %IW2 : INT;
    cnt  : DINT := 0;
    q0   AT %QB0 : BYTE;
    q4   AT %QD4 : DINT;
    q8   AT %QD8 : DINT;
END_VAR
cnt := cnt + 1;
q0 := in1;
q4 := cnt + in2;
q8 := cnt * 16;
END_PROGRAM
EOF
# candidate: same program name + VAR layout (same-layout seed path),
# different output math — divergent by construction
cat > $WORK/cand.st <<'EOF'
PROGRAM BiProg
VAR
    in1  AT %IB0 : BYTE;
    in2  AT %IW2 : INT;
    cnt  : DINT := 0;
    q0   AT %QB0 : BYTE;
    q4   AT %QD4 : DINT;
    q8   AT %QD8 : DINT;
END_VAR
cnt := cnt + 1;
q0 := in1 + 16#40;
q4 := 0 - cnt;
q8 := 16#7777;
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
[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tether-plc-sim" ] \
    || { echo "SKIP: no build tree"; exit 77; }

$TPLCC build $WORK/active.st -o $WORK/active.tplc >/dev/null \
    || { echo "active build failed"; exit 1; }
$TPLCC build $WORK/cand.st -o $WORK/cand.tplc >/dev/null \
    || { echo "candidate build failed"; exit 1; }

# 10-step tape, task period 10 ms; inputs move between steps
cat > $WORK/solo.steps <<'EOF'
step 0 time=0         in 0=05 in 2=0300
step 1 time=10000000  in 0=07 in 2=0500
step 2 time=20000000  in 0=09 in 2=fdff
step 3 time=30000000  in 0=00 in 2=0000
step 4 time=40000000  in 0=40 in 2=1000
step 5 time=50000000  in 0=41 in 2=0100
step 6 time=60000000  in 0=7f in 2=ff7f
step 7 time=70000000  in 0=00 in 2=0100
step 8 time=80000000  in 0=12 in 2=3400
step 9 time=90000000  in 0=ff in 2=ffff
EOF
# shadowed tape: identical inputs/times, plus a load at step 1
sed 's/^step 1 \(.*\)/step 1 \1 shadow_load/' $WORK/solo.steps \
    > $WORK/shadowed.steps

IMG_A=/tplc_bi_a_$$; IMG_B=/tplc_bi_b_$$
CTL_A=/tmp/tplc_bi_a_$$.ctl; CTL_B=/tmp/tplc_bi_b_$$.ctl
trap 'rm -rf "$WORK"; '"$BUILD"'/tplc-image unlink '"$IMG_A"' 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG_B"' 2>/dev/null' EXIT

$BUILD/tether-plc-sim --image $IMG_A --package $WORK/active.tplc \
    --steps $WORK/solo.steps --plc $BUILD/tether-plc \
    --ctl $CTL_A --scan --settle-ms 800 \
    --dump-q $WORK/solo.q >$WORK/solo.log 2>&1 \
    || { sed -n '1,8p' $WORK/solo.log; bad "solo sim run"; }
$BUILD/tether-plc-sim --image $IMG_B --package $WORK/active.tplc \
    --shadow $WORK/cand.tplc \
    --steps $WORK/shadowed.steps --plc $BUILD/tether-plc \
    --ctl $CTL_B --scan --settle-ms 800 \
    --dump-q $WORK/shadowed.q >$WORK/shadowed.log 2>&1 \
    || { sed -n '1,8p' $WORK/shadowed.log; bad "shadowed sim run"; }

[ -s $WORK/solo.q ] && [ -s $WORK/shadowed.q ] \
    || { bad "dump files missing"; echo "$fail fail"; exit 1; }

if diff -u $WORK/solo.q $WORK/shadowed.q >$WORK/diff.txt; then
    ok "active %Q byte-identical solo vs shadowed ($(wc -l <$WORK/solo.q) steps)"
else
    bad "active %Q diverged under shadow"; cat $WORK/diff.txt | head -20
fi

echo "== bitidentical: $pass pass, $fail fail"
[ $fail -eq 0 ]
