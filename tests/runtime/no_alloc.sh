#!/usr/bin/env bash
# runtime property (doc 14 §5): "no allocation after init" — an
# LD_PRELOAD malloc counter wraps the plc while a 4 kHz task scans;
# after warmup the counter must not move. Scan progress is checked
# via the image's out_seq (reading the shm doesn't allocate in the
# runtime — a ctl query would, legitimately: audit + reply buffers).
# Requires: build tree + tplcc + cc (exit 77 skips).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_noalloc
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
[ -x "$BUILD/tplc-image" ] || { echo "SKIP: no tplc-image"; exit 77; }
command -v cc >/dev/null || { echo "SKIP: no cc"; exit 77; }

cc -shared -fPIC -O2 -o $WORK/malloc_count.so \
    tests/runtime/malloc_count.c -ldl -lpthread \
    || { echo "SKIP: shim build failed"; exit 77; }

cat > $WORK/cnt.st <<'ST'
PROGRAM MAIN
VAR
    i : LINT := 0;
END_VAR
i := i + 1;
END_PROGRAM
ST

$TPLCC build $WORK/cnt.st -o $WORK/cnt.tplc \
    --task-period-ns 250000 >/dev/null \
    || { echo "cnt build failed"; exit 1; }

$BUILD/tplc-image create $IMG 64 64 >/dev/null
TPLC_MC_FILE=$WORK/cnt.txt LD_PRELOAD=$WORK/malloc_count.so \
    $BUILD/tether-plc --image $IMG --package $WORK/cnt.tplc \
    --ctl none >$WORK/rt.log 2>&1 &
RT_PID=$!

# warmup: package load + first-run + spin-margin adaptation settle
sleep 4
c1=$(cat $WORK/cnt.txt 2>/dev/null || echo 0)
s1=$($BUILD/tplc-image dump $IMG 2>/dev/null | awk '/^out_seq/{print $2}')
sleep 4
c2=$(cat $WORK/cnt.txt 2>/dev/null || echo 0)
s2=$($BUILD/tplc-image dump $IMG 2>/dev/null | awk '/^out_seq/{print $2}')

delta=$((c2 - c1))
scans=$((s2 - s1))
[ "$delta" -eq 0 ] && [ "$scans" -gt 5000 ] \
    && ok "no-alloc: $scans scans, 0 heap calls in window" \
    || bad "no-alloc: delta=$delta heap calls over $scans scans"

echo "== no_alloc: $pass pass, $fail fail"
[ "$fail" -eq 0 ]
