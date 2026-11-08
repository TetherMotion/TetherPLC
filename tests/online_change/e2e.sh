#!/usr/bin/env bash
# online-change conformance (doc 08 §11): migrated swap preserves
# state, defaults apply, module-swap fast path, rollback restores,
# SWAP_RACE refuses, compile-time DENY gates unsafe changes.
# Requires: build tree + tplcc (exit 77 = prerequisites missing → CTest skips).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_oc_e2e
SHM=/dev/shm$IMG
CTL=/tmp/tplc_oc_e2e.ctl
WORK=$(mktemp -d)
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG"' 2>/dev/null; rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

# little-endian u32 at %Q byte offset $1 (header 256 + %I 64 = %Q base)
qd() { python3 -c "import sys;d=open('$SHM','rb').read();o=256+64+$1;print(int.from_bytes(d[o:o+4],'little'))" 2>/dev/null; }

cat > $WORK/v1.st <<'EOF'
PROGRAM Demo
VAR
    in_seq   AT %IX0.0 : BOOL;
    out_lamp AT %QX0.0 : BOOL;
    out_cnt  AT %QD4   : DINT;
    ton1 : TON;
    cnt  : DINT := 0;
END_VAR
cnt := cnt + 1;
ton1(IN := in_seq, PT := T#500ms);
out_lamp := ton1.Q;
out_cnt  := cnt;
END_PROGRAM
EOF
cat > $WORK/v2.st <<'EOF'
PROGRAM Demo
VAR
    in_seq   AT %IX0.0 : BOOL;
    out_lamp AT %QX0.0 : BOOL;
    out_cnt  AT %QD4   : DINT;
    out_x    AT %QD8   : DINT;
    ton1 : TON;
    cnt  : DINT := 0;
    extra : DINT := 42;
END_VAR
cnt := cnt + 1;
ton1(IN := in_seq, PT := T#500ms);
out_lamp := ton1.Q;
out_cnt  := cnt;
out_x    := extra;
END_PROGRAM
EOF
# v4: same vars as v2 plus `added` — compiled against v1, so applying
# it while v2 is live must hit the SWAP_RACE gate
cat > $WORK/v4.st <<'EOF'
PROGRAM Demo
VAR
    in_seq   AT %IX0.0 : BOOL;
    out_lamp AT %QX0.0 : BOOL;
    out_cnt  AT %QD4   : DINT;
    out_x    AT %QD8   : DINT;
    ton1 : TON;
    cnt  : DINT := 0;
    extra : DINT := 42;
    added : DINT := 99;
END_VAR
cnt := cnt + 1;
ton1(IN := in_seq, PT := T#500ms);
out_lamp := ton1.Q;
out_cnt  := cnt;
out_x    := added;
END_PROGRAM
EOF
cat > $WORK/v3_deny.st <<'EOF'
PROGRAM Demo
VAR
    out_cnt  AT %QD4   : DINT;
    ton1 : TON;
    cnt  : INT := 0;
END_VAR
cnt := cnt + 1;
ton1(IN := FALSE, PT := T#1s);
out_cnt  := cnt;
END_PROGRAM
EOF

# tplcc needs an LLVM-21 toolchain — locate one if not already on PATH
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

$TPLCC build $WORK/v1.st -o $WORK/v1.tplc >/dev/null || { echo "v1 build failed"; exit 1; }
$TPLCC build $WORK/v2.st -o $WORK/v2.tplc --change-against $WORK/v1.tplc >/dev/null \
    || bad "v2 change-against compile"
$TPLCC build $WORK/v4.st -o $WORK/v4.tplc --change-against $WORK/v1.tplc >/dev/null \
    || bad "v4 change-against compile"

# deny gate: INT←DINT narrowing must refuse at compile time
$TPLCC build $WORK/v3_deny.st -o $WORK/v3.tplc --change-against $WORK/v1.tplc 2>/dev/null \
    && bad "deny: narrowing compiled anyway" || ok "compile-time deny"

$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --package $WORK/v1.tplc --ctl $CTL \
    --quarantine-cycles 100000 >$WORK/rt.log 2>&1 &
RT_PID=$!
sleep 1

before=$(qd 4)
sleep 0.4
# named + offset watch armed pre-swap: the name re-resolves through
# the new symmap, the raw range goes stale
$BUILD/tplc-ctl $CTL watch cnt 0:4 >/dev/null || bad "watch arm"
$BUILD/tplc-ctl $CTL swap $WORK/v2.tplc >/dev/null || bad "swap op"
sleep 0.5
after=$(qd 4)
[ "${after:-0}" -gt "${before:-0}" ] \
    && ok "migrated swap: state preserved ($before→$after)" \
    || bad "migrated swap: cnt reset ($before→$after)"
[ "$(qd 8)" = "42" ] && ok "default-init applied (out_x=42)" \
    || bad "default-init (got $(qd 8))"

# state.diff: the committed migrate reports its rules with names
d=$($BUILD/tplc-ctl $CTL state-diff 2>&1)
echo "$d" | grep -q "migrated" && echo "$d" | grep -q "init " \
    && ok "state.diff reports migrate rules" \
    || bad "state.diff ($d)"

# post-swap subscriptions: named item still reads, offset item stale
g=$($BUILD/tplc-ctl $CTL watch-get 1 2>&1)
echo "$g" | grep -q "(stale)" && echo "$g" | grep -q "4 B" \
    && ok "watch re-resolution + stale marking" \
    || bad "watch re-resolution ($g)"

# module-swap fast path: v2→v2 identical layout
$BUILD/tplc-ctl $CTL swap $WORK/v2.tplc >/dev/null \
    && grep -q module-swap $WORK/rt.log \
    && ok "module-swap fast path" || bad "module-swap fast path"

# SWAP_RACE: v4 was compiled against v1 but live layout is v2
r=$($BUILD/tplc-ctl $CTL swap $WORK/v4.tplc 2>&1)
echo "$r" | grep -q SWAP_RACE \
    && ok "SWAP_RACE refusal on stale predecessor" || bad "SWAP_RACE (got: $r)"
sleep 0.3
[ "$(qd 4)" -gt 0 ] && ok "old program survived refusal" \
    || bad "program dead after refusal"

# rollback inside the window
$BUILD/tplc-ctl $CTL rollback >/dev/null \
    && grep -q "rollback restored" $WORK/rt.log \
    && ok "rollback restores prior program" || bad "rollback"

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
