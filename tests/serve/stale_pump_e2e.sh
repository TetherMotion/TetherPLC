#!/usr/bin/env bash
# bus_scan stale-pump recovery: the resolved tplc-ec-pump can itself be
# an old build that rejects --scan (cargo/cmake rebuild failures leave
# the stale file sitting at build/tplc-ec-pump — the path is right, the
# binary is old). The scan must (a) retry the spawn after re-resolving
# — including the SAME path, since the file may have been replaced —
# and (b) when the pump still rejects --scan, report a stale build with
# the rebuild command instead of surfacing a bare "unknown arg" error
# that looks like a scan bug. Pinned with fake pumps via TPLC_BINDIR.
# Exit 77 = no toolchain.

set -uo pipefail
cd "$(dirname "$0")/../.."

TPLC=${TPLC:-./compiler/target/debug/tplc}
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLC" ] || TPLC=./compiler/target/release/tplc
[ -x "$TPLC" ] || { echo "SKIP no tplc"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP no python3"; exit 77; }

W=$(mktemp -d)
trap 'rm -rf "$W"; [ -n "${SRV_PID:-}" ] && kill "$SRV_PID" 2>/dev/null' EXIT
pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

mkfakebin() {
    # $1 = dir; writes a fake tplc-ec-pump whose behaviour is driven by
    # $FAKE_MODE + $FAKE_COUNT counter file (read at exec time, so the
    # "binary" can change behaviour between spawns — simulating a
    # rebuild landing between resolution and retry).
    mkdir -p "$1"
    cat > "$1/tplc-ec-pump" <<'EOS'
#!/usr/bin/env bash
mode="$(cat "$FAKE_MODE" 2>/dev/null || echo ok)"
n=$(cat "$FAKE_COUNT" 2>/dev/null || echo 0); n=$((n+1)); echo "$n" > "$FAKE_COUNT"
if [ "$mode" = "stale" ] || { [ "$mode" = "rebuilt" ] && [ "$n" -le 1 ]; }; then
    for a in "$@"; do
        case "$a" in --scan) echo "unknown arg --scan" >&2; exit 2;; esac
    done
fi
# "current" pump: parse --fleet-out, write a fleet line, exit 0
out=""
while [ $# -gt 0 ]; do
    case "$1" in --fleet-out) out="$2"; shift 2;; *) shift;; esac
done
[ -n "$out" ] && printf 'slave0 EK1100 coupler\n' > "$out"
echo "fake pump scanned"
exit 0
EOS
    chmod +x "$1/tplc-ec-pump"
}

FAKE_MODE="$W/mode" FAKE_COUNT="$W/count"
export FAKE_MODE FAKE_COUNT
mkfakebin "$W/fakebin"
echo stale > "$FAKE_MODE"

PORT=$(( (RANDOM % 2000 ) + 19000 ))
TPLC_BINDIR="$W/fakebin" "$TPLC" serve --port "$PORT" --ctl /tmp/tplc-stale-pump.$$.ctl \
    --tplccd "" --no-webui --simdir "$W/sim" \
    >"$W/serve.log" 2>&1 &
SRV_PID=$!
for _ in $(seq 50); do
    curl -s -m 1 -o /dev/null "http://127.0.0.1:$PORT/health" && break
    sleep 0.1
done

python3 - "$PORT" <<'PY'
import json, sys, os
sys.path.insert(0, os.path.join(os.getcwd(), 'tests/serve'))
from ws_client import WS

port = int(sys.argv[1])
ws = WS(port)
fails = []

def check(name, cond, extra=""):
    print(("PASS " if cond else "FAIL ") + name + (" — " + extra if extra and not cond else ""))
    if not cond: fails.append(name)

# A) stale pump → meta ERROR explaining the build is stale + how to fix.
#    (UDP scan: no CAP_NET_RAW gating on this path.)
try:
    ws.meta("bus_scan", {"peer": "127.0.0.1", "port": 34980})
    check("stale pump → error", False, "meta call unexpectedly succeeded")
except RuntimeError as e:
    msg = str(e)
    check("stale pump → error", True)
    check("error says stale build", "stale" in msg.lower(), msg[:200])
    check("error has rebuild cmd", "tplc-ec-pump" in msg and "cmake --build" in msg, msg[:200])

# B) pump "rebuilt" between resolution and retry: first spawn sees the
#    stale binary, the re-resolved SAME path sees the fresh one. The old
#    code skipped the retry when fresh==pump and surfaced exit 2.
open(os.environ["FAKE_MODE"], "w").write("rebuilt")
open(os.environ["FAKE_COUNT"], "w").write("0")
r = ws.meta("bus_scan", {"peer": "127.0.0.1", "port": 34980})
check("rebuilt pump → exit 0", r.get("exit") == 0, str(r)[:200])
check("rebuilt pump → fleet written", "EK1100" in r.get("fleet", ""), r.get("fleet", "")[:100])

# C) healthy pump → normal result, no false-positive stale diagnosis.
open(os.environ["FAKE_MODE"], "w").write("ok")
open(os.environ["FAKE_COUNT"], "w").write("0")
r = ws.meta("bus_scan", {"peer": "127.0.0.1", "port": 34980})
check("healthy pump → exit 0", r.get("exit") == 0, str(r)[:200])
check("healthy pump → pump path reported", "fakebin" in r.get("pump", ""), r.get("pump", ""))

sys.exit(1 if fails else 0)
PY
rc=$?
[ $rc -eq 0 ] || fail "python assertions"

echo "== serve log tail"; tail -5 "$W/serve.log" 2>/dev/null
if [ $fail -eq 0 ] && [ $rc -eq 0 ]; then echo "ALL PASS"; exit 0; fi
exit 1
