#!/usr/bin/env bash
# Raw-NIC serve e2e: the complete Discovery-card flow over a real
# virtual interface, end to end:
#
#   ip veth pair                    (CAP_NET_ADMIN)
#   beckhoff_slave_chain --iface    (raw AF_PACKET slave, CAP_NET_RAW)
#   tplc serve (privileged)         meta-ops over /ws
#     iface_list -> veth ends enumerated, raw_capable=true
#     bus_scan --iface -> exit 0, scan table, <simdir>/.fleet written
#     bus_scan bad iface -> clean "no such interface" error
#     GET /bus -> .fleet artifact served
#   fleet_e2e EC_IFACE phase        full cyclic path: raw slave chain
#                                   -> pump --fleet auto -> .topology/.bus
#                                   -> bridge -> PLC %I/%Q round trip
#
# Privileges: CAP_NET_ADMIN for the veth pair (root, runec, 'sudo -n',
# or cap_net_admin+ep on ip(8)) and CAP_NET_RAW for the AF_PACKET
# binaries (wrapper, or cap_net_raw+ep on tplc-ec-pump and
# beckhoff_slave_chain). Otherwise SKIP 77.
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
TETHER_DIR=${TETHER_DIR:-../Tether}
TETHER_BIN=${TETHER_BIN:-$TETHER_DIR/build/bin/beckhoff_slave_chain}
WORK=$(mktemp -d)
CTL=$WORK/ctl.sock
SIM=$WORK/sim
VETH_S=tplcsv0          # slave end
VETH_M=tplcsv1          # master end (the NIC the master scans on)
PRIV=()
SLAVE_PID= SERVE_PID=

cleanup() {
    [ -n "$SERVE_PID" ] && kill "$SERVE_PID" 2>/dev/null
    [ -n "$SLAVE_PID" ] && kill "$SLAVE_PID" 2>/dev/null
    sleep 0.3
    pkill -f "beckhoff_slave_chain.*--iface $VETH_S" 2>/dev/null
    pkill -f "tplc-ec-pump.*$SIM" 2>/dev/null
    "${PRIV[@]:-env}" "$(command -v ip || echo ip)" link del "$VETH_S" 2>/dev/null
    rm -rf "$WORK"
}
trap cleanup EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

# ---- prerequisites ---------------------------------------------------------
[ -x "$TETHER_BIN" ] || { echo "SKIP: no $TETHER_BIN (build Tether)"; exit 77; }
[ -x "$BUILD/tplc-ec-pump" ] || { echo "SKIP: no tplc-ec-pump"; exit 77; }
[ -f tests/serve/ws_client.py ] || { echo "SKIP: no ws_client.py"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP: no python3"; exit 77; }
IPBIN=$(command -v ip || true)
[ -n "$IPBIN" ] || { echo "SKIP: no ip(8)"; exit 77; }

# tplc binary for `serve`
TPLC=${TPLC:-./compiler/target/debug/tplc}
[ -x "$TPLC" ] || TPLC=$(command -v tplc || true)
[ -n "$TPLC" ] || { echo "SKIP: no tplc binary"; exit 77; }

# ---- privilege wrapper -------------------------------------------------------
# Three ways in: root/setcap'd ip directly, runec, or NOPASSWD sudo.
# File caps on the raw binaries cover the no-wrapper case too.
PRIV=()
if ! "$IPBIN" link add "$VETH_S" type veth peer name "$VETH_M" 2>/dev/null; then
    if command -v runec >/dev/null; then
        PRIV=(runec)
    elif sudo -n true 2>/dev/null; then
        PRIV=(sudo -n)
    else
        echo "SKIP: cannot create veth (need CAP_NET_ADMIN: root, runec,"
        echo "  'sudo -n', or 'sudo setcap cap_net_admin+ep $IPBIN')"
        exit 77
    fi
    "${PRIV[@]:-env}" "$IPBIN" link del "$VETH_S" 2>/dev/null
    "${PRIV[@]:-env}" "$IPBIN" link del "$VETH_M" 2>/dev/null
    if ! "${PRIV[@]}" "$IPBIN" link add "$VETH_S" type veth peer name "$VETH_M"; then
        echo "SKIP: ${PRIV[*]} ip link add failed"
        exit 77
    fi
fi
if ! "${PRIV[@]:-env}" "$IPBIN" link set "$VETH_S" up \
   || ! "${PRIV[@]:-env}" "$IPBIN" link set "$VETH_M" up; then
    echo "SKIP: cannot bring veth pair up"
    exit 77
fi
ok "veth pair $VETH_S <-> $VETH_M up (${PRIV[*]:-direct})"

# AF_PACKET binaries need CAP_NET_RAW — inherited via PRIV/root, or the
# file cap must sit on the binaries themselves (children of an
# unprivileged serve get nothing from the parent).
if [ "$(id -u)" != 0 ] && [ ${#PRIV[@]} -eq 0 ]; then
    for b in "$TETHER_BIN" "$BUILD/tplc-ec-pump"; do
        getcap "$b" 2>/dev/null | grep -q cap_net_raw || {
            echo "SKIP: $b lacks cap_net_raw — 'sudo setcap cap_net_raw+ep $b',"
            echo "  or run this test via runec/root/sudo"
            exit 77
        }
    done
fi

# ---- ports ------------------------------------------------------------------
read -r PORT SLAVE_IO <<<"$(python3 - <<'PY'
import socket
ports = []
for _ in range(2):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind(("127.0.0.1", 0)); ports.append(s.getsockname()[1]); s.close()
print(*ports)
PY
)"

# ---- raw slave chain on the slave end ---------------------------------------
"${PRIV[@]}" "$TETHER_BIN" --iface "$VETH_S" --io-port "$SLAVE_IO" --hz 1000 \
    >$WORK/slave.log 2>&1 &
SLAVE_PID=$!
sleep 1
kill -0 "$SLAVE_PID" 2>/dev/null || { echo "FAIL: slave chain died"; tail -20 $WORK/slave.log; exit 1; }
ok "beckhoff_slave_chain raw on $VETH_S"

# ---- tplc serve (privileged so the spawned pump gets AF_PACKET) --------------
"${PRIV[@]}" "$TPLC" serve --ctl "$CTL" --port "$PORT" --simdir "$SIM" \
    --no-webui >$WORK/serve.log 2>&1 &
SERVE_PID=$!
for i in $(seq 1 50); do
    ss -tln 2>/dev/null | grep -q ":$PORT " && break
    kill -0 "$SERVE_PID" 2>/dev/null || { echo "FAIL: tplc serve died"; tail -20 $WORK/serve.log; exit 1; }
    sleep 0.2
done
ss -tln 2>/dev/null | grep -q ":$PORT " || { echo "FAIL: serve not listening"; tail -20 $WORK/serve.log; exit 1; }
ok "tplc serve on :$PORT (ctl $CTL, simdir $SIM)"

# ---- meta ops ---------------------------------------------------------------
python3 - "$PORT" "$VETH_S" "$VETH_M" "$SIM" <<'PY'
import json, os, sys, time
sys.path.insert(0, 'tests/serve')
from ws_client import WS

port, veth_s, veth_m, sim = int(sys.argv[1]), sys.argv[2], sys.argv[3], sys.argv[4]
ws = WS(port)

def ok(msg): print(f"PASS {msg}")
def bad(msg): print(f"FAIL {msg}"); sys.exit(1)

# iface_list: both veth ends enumerated, raw_capable under privilege
d = ws.meta("iface_list")
names = {i["name"]: i for i in d["ifaces"]}
assert veth_s in names and veth_m in names, f"veth pair missing from {sorted(names)[:8]}…"
ok(f"iface_list sees {veth_s}/{veth_m} (kind={names[veth_s]['kind']}, {len(d['ifaces'])} ifaces)")
assert d.get("raw_capable") is True, "raw_capable should be true under privilege"
ok("raw_capable=true under privileged serve")

# bus_scan over the raw master end — discovery must find the fleet
d = ws.meta("bus_scan", {"iface": veth_m})
assert d["exit"] == 0, f"scan exit {d['exit']}: {d['stderr'][:300]}"
assert "slave(s)" in d["stderr"], d["stderr"][:300]
ok(f"bus_scan --iface {veth_m}: {d['stderr'].splitlines()[0]}")
fleet = open(os.path.join(sim, ".fleet")).read() \
        if os.path.exists(os.path.join(sim, ".fleet")) else ""
assert "EK" in fleet or "EL" in fleet, f"fleet artifact missing/empty: {fleet!r}"
ok(f"<simdir>/.fleet written ({len(fleet.splitlines())} lines)")

# error paths stay clean: no such iface / no link at all
d = ws.meta("bus_scan", {"iface": "nosuchnic9"})
assert d["exit"] != 0 and "no such interface" in d["stderr"], d["stderr"]
ok("bus_scan bad iface → 'no such interface'")
try:
    ws.meta("bus_scan", {})
    bad("bus_scan with no link should error")
except RuntimeError as e:
    assert "needs a link" in str(e), e
    ok("bus_scan with no link → meta error")
PY
[ $? -eq 0 ] || { echo "FAIL: meta-op phase"; exit 1; }

# ---- GET /bus artifact dump --------------------------------------------------
if command -v curl >/dev/null; then
    body=$(curl -s -m 5 "http://127.0.0.1:$PORT/bus")
    echo "$body" | python3 -c "
import json,sys
d = json.load(sys.stdin)
assert d.get('fleet'), 'GET /bus: fleet artifact missing'
assert 'EK' in d['fleet'] or 'EL' in d['fleet'], d['fleet'][:200]
print('PASS GET /bus serves the .fleet artifact')
" || bad "GET /bus artifact"
fi

# ---- full cyclic path --------------------------------------------------------
# The serve-level scan only writes .fleet; the complete process (discovery
# -> bring-up -> cyclic PDO -> PLC %I/%Q) is fleet_e2e's veth mode — run it
# as the second phase with its own pair.
echo "---- full cyclic e2e (fleet_e2e EC_IFACE=veth-tplc-ec) ----"
EC_IFACE=veth-tplc-ec BUILD="$BUILD" TETHER_BIN="$TETHER_BIN" \
    bash tests/ec/fleet_e2e.sh
rc=$?
if [ $rc -eq 77 ]; then
    echo "cyclic phase skipped (77)"
elif [ $rc -eq 0 ]; then
    ok "cyclic e2e over veth (fleet_e2e raw mode)"
else
    bad "cyclic e2e over veth (rc=$rc)"
fi

echo "---- $pass passed, $fail failed ----"
[ $fail -eq 0 ] || exit 1
