#!/usr/bin/env bash
# eventlog/e2e.sh — tplc-eventlog journal end-to-end:
#   ST FB_EventLog raise/clear/confirm/query/stats  →
#   tether-plc ctl EVTLOG_LIST/QUERY/STATS/HISTORY/CONFIRM  →
#   EV_USER broadcast to notify subscribers.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
CTL=${CTL:-$BUILD/tplc-ctl}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
[ -x "$CTL" ] || { echo "tplc-ctl missing: $CTL" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

"$TPLCC" build tests/eventlog/evlog_case.st \
    -o "$WORK/evlog.tplc" >/dev/null 2>&1 \
    && ok "evlog_case build" \
    || { bad "evlog_case build"; echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 40); do
        echo "step $i time=$((i * 10000000)) in 0=00"
    done
} > "$WORK/ev.steps"

IMG=/tplc_evlog_$$; SOCK="$WORK/dm.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/evlog.tplc" \
    --steps "$WORK/ev.steps" --plc "$BUILD/tether-plc" \
    --ctl "$SOCK" --settle-ms 800 --hold-ms 6000 \
    >"$WORK/sim.log" 2>&1 &
SIM_PID=$!
# EV_USER subscription: ctl_connect retries until the socket
# appears; the 8s window covers the whole replay
"$CTL" "$SOCK" events 8 > "$WORK/events.log" 2>&1 &
EV_PID=$!

# wait for the program to finish its scenario (7 states ≈ 7 scans)
sleep 4

LIST=$("$CTL" "$SOCK" evlog-list 2>&1)
STATS=$("$CTL" "$SOCK" evlog-stats 2>&1)
HIST=$("$CTL" "$SOCK" evlog-history 16 2>&1)

echo "$LIST"  > "$WORK/list.out"
echo "$STATS" > "$WORK/stats.out"
echo "$HIST"  > "$WORK/hist.out"

# -- stats: live 2 (A cleared + INFO), raised 3, cleared 1, confirmed 1
if echo "$STATS" | grep -q "live 2  raised 3  cleared 1  confirmed 1"; then
    ok "evlog-stats counters"
else
    bad "evlog-stats counters"; echo "  got: $STATS"
fi

# -- list: two live entries — cleared 'temp high' + raised 'cycle ok'
if echo "$LIST" | grep -q "live 2" && \
   echo "$LIST" | grep -q "cleared" && \
   echo "$LIST" | grep -q "temp high" && \
   echo "$LIST" | grep -q "cycle ok"; then
    ok "evlog-list live alarms"
else
    bad "evlog-list live alarms"; cat "$WORK/list.out"
fi

# -- history: raise A, raise B, clear A, confirm B, raise INFO
if echo "$HIST" | grep -q "raised" && \
   echo "$HIST" | grep -q "cleared" && \
   echo "$HIST" | grep -q "confirmed" && \
   echo "$HIST" | grep -q "cycle ok"; then
    ok "evlog-history transitions"
else
    bad "evlog-history transitions"; cat "$WORK/hist.out"
fi

# -- query by id + confirm the live alarm via ctl (engineer path)
ID=$(echo "$LIST" | sed -n 's/.*#\([0-9]*\).*/\1/p' | head -1)
if [ -n "$ID" ]; then
    Q=$("$CTL" "$SOCK" evlog-query "$ID" 2>&1)
    if echo "$Q" | grep -q "temp high"; then
        ok "evlog-query by id"
    else
        bad "evlog-query by id"; echo "  got: $Q"
    fi
    C=$("$CTL" "$SOCK" evlog-confirm "$ID" 2>&1)
    STATS2=$("$CTL" "$SOCK" evlog-stats 2>&1)
    if echo "$STATS2" | grep -q "live 1  raised 3  cleared 1  confirmed 2"; then
        ok "evlog-confirm ack"
    else
        bad "evlog-confirm ack"; echo "  got: $STATS2"
    fi
else
    bad "evlog-query by id (no id parsed)"
fi

# -- EV_USER broadcast reached subscribers
sleep 6   # let the events subscription window close
if grep -q "USER" "$WORK/events.log"; then
    ok "EV_USER broadcast"
else
    bad "EV_USER broadcast"; cat "$WORK/events.log"
fi

wait $SIM_PID 2>/dev/null
kill $EV_PID 2>/dev/null
echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
