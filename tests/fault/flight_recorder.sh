#!/usr/bin/env bash
# flight recorder (doc 12 §10.5): an induced task fault freezes the
# trace ring and TRACE_READ returns the pre-fault scans — the
# post-mortem channel data. Requires: build tree + tplcc (77 skips).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_flight
SHM=/dev/shm$IMG
CTL=/tmp/tplc_flight.ctl
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
(cd compiler && cargo build -q -p tplcc 2>/dev/null) || true
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
[ -x "$BUILD/tether-plc" ] || { echo "SKIP: no build tree"; exit 77; }

# cnt ticks up, then the task wedges in WHILE — the watchdog aborts
# it cooperatively, which is a task fault (TPLC_TASK_E_ABORTED)
cat > $WORK/hang.st <<'ST'
PROGRAM P
VAR
    cnt : DINT := 0;
END_VAR
cnt := cnt + 1;
IF cnt > 150 THEN
    WHILE TRUE DO
    END_WHILE
END_IF
END_PROGRAM
ST

$TPLCC build $WORK/hang.st -o $WORK/hang.tplc >/dev/null \
    || { echo "hang build failed"; exit 1; }

$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --package $WORK/hang.tplc \
    --ctl $CTL --hang-timeout-ms 400 >$WORK/rt.log 2>&1 &
RT_PID=$!
sleep 1

# arm a channel on cnt before the task wedges itself (~1.5 s in at
# the default 10 ms task period)
off=$($BUILD/tplc-ctl $CTL resolve cnt | awk '{for(i=1;i<NF;i++) if($i=="off") print $(i+1)}')
[ -n "$off" ] || { echo "resolve cnt failed"; exit 1; }
$BUILD/tplc-ctl $CTL trace ${off}:4 >/dev/null || bad "trace arm"

# stall → monitor flags FAULT + aborts → hooks.fault freezes the ring
sleep 1.5

st=$($BUILD/tplc-ctl $CTL trace-status 2>&1)
echo "$st" | grep -q FROZEN \
    && ok "induced fault froze the recorder" \
    || bad "trace-status ($st)"

# post-mortem: frames exist and the cnt channel shows a mid-run
# value (any nonzero hex digit) — pre-fault scans were retained
rd=$($BUILD/tplc-ctl $CTL trace-read 0 16 2>&1)
echo "$rd" | grep -q "frames " \
    && echo "$rd" | grep -E '#[0-9]+ t=[0-9]+ +[0-9a-f]*[1-9a-f]' \
         >/dev/null \
    && ok "frozen ring holds pre-fault channel data" \
    || bad "trace-read ($rd)"

# the program stays faulted (hold policy) and ctl stays serviceable
r=$($BUILD/tplc-ctl $CTL prog 2>&1 || true)
echo "$r" | grep -q "state fault" \
    && ok "program held in FAULT, ctl live" \
    || bad "prog after fault ($r)"

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
