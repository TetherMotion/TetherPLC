#!/usr/bin/env bash
# perf evidence (DESIGN-GOALS §6): measures cycle cadence, swap
# pause, and watchdog→safe-state latency on this host, and prints a
# markdown table for docs/plc/perf-evidence.md. Not a ctest gate —
# run manually to refresh the evidence doc.
# Requires: build tree + tplcc + python3 (exit 77 skips).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
IMG=/tplc_perf
SHM=/dev/shm$IMG
CTL=/tmp/tplc_perf.ctl
WORK=$(mktemp -d)
RT_PID=
trap '[ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null; '"$BUILD"'/tplc-image unlink '"$IMG"' 2>/dev/null; rm -rf "$WORK"' EXIT

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
[ -x "$BUILD/hang_spin.so" ] || { echo "SKIP: no hang_spin fixture"; exit 77; }
[ -x "$BUILD/tplc-image" ] || { echo "SKIP: no tplc-image"; exit 77; }
[ -x "$BUILD/tplc-ctl" ] || { echo "SKIP: no tplc-ctl"; exit 77; }

now_ms() { date +%s%3N; }

# task line: $11 runs, $13 late, $15 max_late_ns (see stress.sh)
task_stat() { $BUILD/tplc-ctl $CTL prog 2>/dev/null | awk '$1=="task"{print $11,$13,$15; exit}'; }

# cadence over a window: runs/s and worst release lateness
measure_window() {  # seconds -> "rate max_late_ns exec_max exec_avg"
    local secs=$1 r0 l0 m0 r1 l1 m1
    read r0 l0 m0 < <(task_stat)
    sleep "$secs"
    read r1 l1 m1 < <(task_stat)
    echo "$(( (r1 - r0) / secs )) $m1"
}

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
cat > $WORK/pair2.st <<'ST'
PROGRAM P
VAR
    i : LINT := 0;
    pair_lo : LINT := 0;
    pair_hi : LINT := 0;
END_VAR
i := i + 2;
pair_lo := i;
pair_hi := i;
END_PROGRAM
ST

$TPLCC build $WORK/pair.st  -o $WORK/pair4k.tplc  --task-period-ns 250000  >/dev/null || { echo "build pair4k failed"; exit 1; }
$TPLCC build $WORK/pair2.st -o $WORK/pair4k2.tplc --task-period-ns 250000  >/dev/null || { echo "build pair4k2 failed"; exit 1; }
$TPLCC build $WORK/pair.st  -o $WORK/pair1k.tplc  --task-period-ns 1000000 >/dev/null || { echo "build pair1k failed"; exit 1; }

echo "== tether-plc perf evidence ($(uname -m), $(nproc) CPUs) =="

# ---- 4 kHz cadence ----
$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --package $WORK/pair4k.tplc --ctl $CTL >$WORK/rt4k.log 2>&1 &
RT_PID=$!
sleep 2
read rate4k late4k < <(measure_window 4)
echo "4kHz cadence: ${rate4k} runs/s  max_late ${late4k} ns"

# ---- hot swap at 4 kHz: pause = worst release lateness around swap ----
sw_t0=$(now_ms)
$BUILD/tplc-ctl $CTL swap $WORK/pair4k2.tplc >/dev/null 2>&1
sw_t1=$(now_ms)
sleep 1                      # let post-swap lateness accumulate
read _r _l late_swap < <(task_stat)
swap_ms=$((sw_t1 - sw_t0))
echo "swap: ctl call ${swap_ms} ms  post-swap max_late ${late_swap} ns"

kill $RT_PID 2>/dev/null; wait $RT_PID 2>/dev/null; RT_PID=
$BUILD/tplc-image unlink $IMG 2>/dev/null

# ---- 1 kHz cadence ----
$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --package $WORK/pair1k.tplc --ctl $CTL >$WORK/rt1k.log 2>&1 &
RT_PID=$!
sleep 2
read rate1k late1k < <(measure_window 4)
echo "1kHz cadence: ${rate1k} runs/s  max_late ${late1k} ns"
kill $RT_PID 2>/dev/null; wait $RT_PID 2>/dev/null; RT_PID=
$BUILD/tplc-image unlink $IMG 2>/dev/null

# ---- watchdog -> safe state: uncooperative spin, --hang-timeout-ms ----
HANG_MS=${HANG_MS:-300}
$BUILD/tplc-image create $IMG 64 64 >/dev/null
$BUILD/tether-plc --image $IMG --module $BUILD/hang_spin.so \
    --ctl $CTL --hang-timeout-ms $HANG_MS >$WORK/rtwd.log 2>&1 &
RT_PID=$!
# poll the image flag: while the task spins, the scan loop is blocked
# and ctl does not answer — the image flag is the observable commit.
# Detection latency ground truth is the monitor's own stale_ms in the
# log; the image poll only confirms the flag was set.
t_launch=$(now_ms); t_dead=0; flag_seen=0
for _ in $(seq 1 400); do
    st=$($BUILD/tplc-image dump $IMG 2>/dev/null | awk '/plc_hb/{for(i=1;i<=NF;i++) if($i=="status"){print $(i+1); exit}}')
    [ "$st" = "fault" ] && flag_seen=1
    kill -0 $RT_PID 2>/dev/null || { t_dead=$(now_ms); break; }
    sleep 0.02
done
wait $RT_PID 2>/dev/null; RT_PID=
wd_dead=$(( t_dead > 0 ? t_dead - t_launch : -1 ))
wd_stale=$(awk '/scan stall/{print $4; exit}' $WORK/rtwd.log | tr -dc '0-9')
[ -n "$wd_stale" ] || wd_stale=-1
echo "watchdog: stall->fault flag ${wd_stale} ms (timeout ${HANG_MS} ms, flag_seen=$flag_seen), _exit at ${wd_dead} ms"
grep -h "stall\|unrecovered" $WORK/rtwd.log | sed 's/^/  log: /'

$BUILD/tplc-image unlink $IMG 2>/dev/null

echo
echo "| metric | DESIGN-GOALS §6 target | measured |"
echo "|---|---|---|"
echo "| cycle rate 1 kHz | 1 kHz | ${rate1k} runs/s |"
echo "| cycle rate 4 kHz | up to 4 kHz | ${rate4k} runs/s |"
echo "| max release lateness 1 kHz | (informational) | ${late1k} ns |"
echo "| max release lateness 4 kHz | (informational) | ${late4k} ns |"
echo "| swap duration (ctl call, sync) | (informational) | ${swap_ms} ms |"
echo "| scan pause injected by swap | <= 1 task cycle, < 500 us pref | ${late_swap} ns max_late (${rate4k} Hz task) |"
echo "| watchdog stall->FAULT flag | <= 1 cycle after trip | ${wd_stale} ms (timeout ${HANG_MS} ms) |"
echo "| watchdog unrecovered->_exit | supervisor respawn bound | ${wd_dead} ms |"
