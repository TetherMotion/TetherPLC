#!/usr/bin/env bash
# tplc serve /ws: the browser's only backend channel. Compiles project
# sources through the `compile` meta-op (the exact {name,content} shape
# TetherWebUI's build.ts emits), checks the reply envelope: ok/exit +
# a valid package in build mode, the merged blocks catalog, libsrc
# source serving + traversal rejection, and the degraded-mode ctl
# error path (no runtime → F_ERROR frame). Guards the pclose/SIGCHLD
# regression that made compile return exit=-1 with no package.
# Exit 77 = no toolchain.
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLC=${TPLC:-./compiler/target/debug/tplc}
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLC" ] || TPLC=./compiler/target/release/tplc
[ -x "$TPLCC" ] || TPLCC=./compiler/target/release/tplcc
[ -x "$TPLC" ] || { echo "SKIP no tplc"; exit 77; }
[ -x "$TPLCC" ] || { echo "SKIP no tplcc"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP no python3"; exit 77; }

W=$(mktemp -d)
trap 'rm -rf "$W"; [ -n "${SRV_PID:-}" ] && kill "$SRV_PID" 2>/dev/null' EXIT
pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

PORT=$(( (RANDOM % 2000) + 18000 ))
NOPE_CTL=/tmp/tplc-serve-e2e-nope.$$.ctl
"$TPLC" serve --port "$PORT" --ctl "$NOPE_CTL" --tplcc "$PWD/$TPLCC" \
    --tplccd "" --no-webui \
    --libdir "$PWD/libraries/tempctrl" \
    --libdir "$PWD/compat/tc2" --libdir "$PWD/compat/tc3" \
    >"$W/serve.log" 2>&1 &
SRV_PID=$!
for _ in $(seq 50); do
    curl -s -m 1 -o /dev/null "http://127.0.0.1:$PORT/health" && break
    sleep 0.1
done

# old HTTP API must be gone
code=$(curl -s -o /dev/null -w '%{http_code}' -m 5 -X POST \
    --data-binary x "http://127.0.0.1:$PORT/compile")
[ "$code" = "404" ] && ok "POST /compile removed (404)" || bad "POST /compile → $code"
code=$(curl -s -o /dev/null -w '%{http_code}' -m 5 "http://127.0.0.1:$PORT/events")
[ "$code" = "404" ] && ok "GET /events removed (404)" || bad "GET /events → $code"

python3 - "$PORT" "$W" tests/mixed/project.fbd.json tests/mixed/lib.st tests/mixed/stprog.st <<'PY'
import base64, json, sys, os
sys.path.insert(0, os.path.join(os.getcwd(), 'tests/serve'))
from ws_client import WS

port, w = int(sys.argv[1]), sys.argv[2]
ws = WS(port)

# compile?build over the compile meta-op
files = []
for f in sys.argv[3:]:
    files.append({"name": os.path.basename(f), "content": open(f).read()})
d = ws.meta("compile", {"build": True, "files": files})
c = d["compile"]
assert c["ok"] is True, c
assert c["errors"] == 0, c
assert d["exit"] == 0, d
pkg = base64.b64decode(d["package"])
assert len(pkg) > 1024, len(pkg)
open(os.path.join(w, "out.tplc"), "wb").write(pkg)
print("PASS ws compile meta-op: ok + exit 0 + package")

# bad source → ok:false with diags (the IDE renders them)
bad = [{"name": "bad.st",
        "content": "PROGRAM Bad\nVAR R : LREAL; r : DINT; END_VAR\nEND_PROGRAM\n"}]
d = ws.meta("compile", {"build": False, "files": bad})
c = d["compile"]
assert c["ok"] is False, c
assert c["errors"] >= 1, c
assert c["diags"], c
print("PASS ws compile: diagnostics on bad source")

# blocks catalog
d = ws.meta("blocks")
libs = {l["name"]: l["catalog"] for l in d["libs"]}
assert "tempctrl" in libs and "tc2" in libs and "tc3" in libs, libs.keys()
total = sum(len(c["blocks"]) for c in libs.values())
assert total > 200, total
pid = next(b for b in libs["tempctrl"]["blocks"] if b["name"] == "FB_AtcPid")
pins = {p["name"]: p for p in pid["pins"]}
assert pins["Setpoint"]["dir"] == "in" and pins["Out"]["dir"] == "out"
print(f"PASS ws blocks: merged catalog ({total} blocks, typed pins)")

# libsrc: real source + traversal/injection rejection
d = ws.meta("libsrc", {"lib": "tempctrl", "file": "atc_tempctrl.st"})
assert "FB_AtcProfile" in d["text"], d["text"][:200]
for evil in ("../../evil", 'x";touch$(pwd)', "a/b.st"):
    try:
        ws.meta("libsrc", {"lib": "tempctrl", "file": evil})
    except RuntimeError:
        continue
    raise AssertionError(f"libsrc accepted {evil!r}")
print("PASS ws libsrc: serves source, rejects traversal+injection")

# degraded ctl: no runtime → synthesized F_ERROR reply
t, fl, pl = ws.ping_ctl()
assert fl & 0x8000, hex(fl)
code = int.from_bytes(pl[:4], "little", signed=True)
msg = pl[10:10 + int.from_bytes(pl[8:10], "little")].decode(errors="replace")
assert code != 0 and "down" in msg, (code, msg)
print(f"PASS ws ctl degraded: F_ERROR '{msg}'")
PY
[ $? -eq 0 ] && ok "ws meta-ops + degraded ctl" || bad "ws meta-ops"

if [ -s "$W/out.tplc" ] && ./compiler/target/debug/tplc-pkg verify "$W/out.tplc" >"$W/verify.log" 2>&1; then
    ok "package verifies (tplc-pkg)"
else
    bad "package verify"; cat "$W/verify.log" 2>/dev/null | tail -3
fi

# driver-failure path must still emit valid JSON (a malformed envelope
# surfaced as "check failed: Expected double-quoted property name" in
# the IDE — json.load is the regression guard)
kill "$SRV_PID" 2>/dev/null; wait "$SRV_PID" 2>/dev/null
"$TPLC" serve --port "$PORT" --ctl "$NOPE_CTL" --tplcc "$W/no-such-tplcc" \
    --tplccd "" --no-webui >"$W/serve2.log" 2>&1 &
SRV_PID=$!
for _ in $(seq 50); do
    curl -s -m 1 -o /dev/null "http://127.0.0.1:$PORT/health" && break
    sleep 0.1
done
if python3 - "$PORT" "$W" <<'PY'
import json, os, sys
sys.path.insert(0, os.path.join(os.getcwd(), 'tests/serve'))
from ws_client import WS
ws = WS(int(sys.argv[1]))
d = ws.meta("compile", {"build": False,
    "files": [{"name": "bad.st", "content": "PROGRAM Bad\nEND_PROGRAM\n"}]})
c = d["compile"]
assert c["ok"] is False, d
assert c["diags"][0]["code"] == "TPLC_E_DRIVER", d
assert d["exit"] == -1, d
print("PASS ws compile: valid JSON on driver failure")
PY
then ok "ws compile driver-failure JSON"
else bad "ws compile driver-failure JSON"; fi

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
