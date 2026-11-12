#!/usr/bin/env bash
# tplc-ctl-proxy: the browser's only compile path. Builds the proxy from
# source, posts a binary src frame (the exact encoding TetherWebUI's
# build.ts emits) and checks the reply envelope: ok/exit + a valid
# package in build mode. Guards the SIGCHLD/pclose regression that made
# /compile?build return exit=-1 with no package. Exit 77 = no toolchain.
set -uo pipefail
cd "$(dirname "$0")/../.."

if [ -z "${LLVM_SYS_211_PREFIX:-}" ]; then
    for p in /home/uli/llvm21/usr/lib/llvm-21 /home/uli/llvm21/usr /opt/llvm21 /usr/lib/llvm-21; do
        if [ -x "$p/bin/llvm-config" ]; then
            export LLVM_SYS_211_PREFIX=$p
            export LLVM_CONFIG=$p/bin/llvm-config
            export PATH="$p/bin:$PATH"
            break
        fi
    done
fi
(cd compiler && cargo build -q -p tplcc 2>/dev/null) || true
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP no tplcc"; exit 77; }
command -v g++ >/dev/null || { echo "SKIP no g++"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP no python3"; exit 77; }

W=$(mktemp -d)
trap 'rm -rf "$W"; [ -n "${PROXY_PID:-}" ] && kill "$PROXY_PID" 2>/dev/null' EXIT
pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

g++ -O1 -std=c++17 tools/tplc_ctl_proxy.cpp -o "$W/proxy" || { echo "SKIP proxy build failed"; exit 77; }

PORT=$(( (RANDOM % 2000) + 18000 ))
"$W/proxy" --port "$PORT" --tplcc "$PWD/$TPLCC" >"$W/proxy.log" 2>&1 &
PROXY_PID=$!
for _ in $(seq 50); do
    curl -s -m 1 -o /dev/null "http://127.0.0.1:$PORT/health" && break
    sleep 0.1
done

# frame: [u16 count][u16 nameLen,name,u32 len,content]xN (LE)
python3 - "$W" tests/mixed/project.fbd.json tests/mixed/lib.st tests/mixed/stprog.st <<'PY'
import sys
w, *files = sys.argv[1:]
chunks = [(f.split('/')[-1], open(f, 'rb').read()) for f in files]
out = bytearray(len(chunks).to_bytes(2, 'little'))
for name, body in chunks:
    n = name.encode()
    out += len(n).to_bytes(2, 'little') + n + len(body).to_bytes(4, 'little') + body
open(w + '/frame.bin', 'wb').write(out)
PY

if curl -s -m 60 -X POST --data-binary @"$W/frame.bin" \
        "http://127.0.0.1:$PORT/compile?build" -o "$W/build.json" \
   && python3 - "$W/build.json" "$W/out.tplc" <<'PY'
import base64, json, sys
d = json.load(open(sys.argv[1]))
assert d["compile"]["ok"] is True, d["compile"]
assert d["compile"]["errors"] == 0, d["compile"]
assert d["exit"] == 0, d
pkg = base64.b64decode(d["package"])
assert len(pkg) > 1024, len(pkg)
open(sys.argv[2], "wb").write(pkg)
PY
then ok "proxy /compile?build: ok + exit 0 + package"
else bad "proxy /compile?build"; cat "$W/build.json" 2>/dev/null | head -c 400; fi

if [ -s "$W/out.tplc" ] && ./compiler/target/debug/tplc-pkg verify "$W/out.tplc" >"$W/verify.log" 2>&1; then
    ok "package verifies (tplc-pkg)"
else
    bad "package verify"; cat "$W/verify.log" 2>/dev/null | tail -3
fi

# a bad source must still answer 200 with ok:false (the IDE renders diags)
printf 'PROGRAM Bad\nVAR R : LREAL; r : DINT; END_VAR\nEND_PROGRAM\n' > "$W/bad.st"
python3 - "$W" "$W/bad.st" <<'PY'
import sys
w, f = sys.argv[1:]
name, body = 'bad.st', open(f, 'rb').read()
n = name.encode()
out = (1).to_bytes(2, 'little') + len(n).to_bytes(2, 'little') + n + len(body).to_bytes(4, 'little') + body
open(w + '/bad.bin', 'wb').write(out)
PY
if curl -s -m 60 -X POST --data-binary @"$W/bad.bin" \
        "http://127.0.0.1:$PORT/compile" -o "$W/bad.json" \
   && python3 - "$W/bad.json" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
assert d["compile"]["ok"] is False, d["compile"]
assert d["compile"]["errors"] >= 1, d["compile"]
assert d["compile"]["diags"], d["compile"]
PY
then ok "proxy /compile: diagnostics on bad source"
else bad "proxy /compile diagnostics"; cat "$W/bad.json" 2>/dev/null | head -c 300; fi

# /blocks + /libsrc: the catalog and source endpoints feed the FBD
# palette's library groups and auto-attach. Relaunch with --libdir.
kill "$PROXY_PID" 2>/dev/null; wait "$PROXY_PID" 2>/dev/null
"$W/proxy" --port "$PORT" --tplcc "$PWD/$TPLCC" --tplccd off \
    --libdir "$PWD/libraries/tempctrl" \
    --libdir "$PWD/compat/tc2" --libdir "$PWD/compat/tc3" \
    >"$W/proxy2.log" 2>&1 &
PROXY_PID=$!
for _ in $(seq 50); do
    curl -s -m 1 -o /dev/null "http://127.0.0.1:$PORT/health" && break
    sleep 0.1
done

if curl -s -m 120 "http://127.0.0.1:$PORT/blocks" -o "$W/blocks.json" \
   && python3 - "$W/blocks.json" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
libs = {l["name"]: l["catalog"] for l in d["libs"]}
assert "tempctrl" in libs and "tc2" in libs and "tc3" in libs, libs.keys()
total = sum(len(c["blocks"]) for c in libs.values())
assert total > 200, total
pid = next(b for b in libs["tempctrl"]["blocks"] if b["name"] == "FB_AtcPid")
pins = {p["name"]: p for p in pid["pins"]}
assert pins["Setpoint"]["dir"] == "in" and pins["Out"]["dir"] == "out"
assert any(b["name"] == "TON" or b["name"].startswith("FB_") for c in libs.values() for b in c["blocks"])
PY
then ok "proxy /blocks: merged catalog (>200 blocks, typed pins)"
else bad "proxy /blocks"; head -c 300 "$W/blocks.json" 2>/dev/null; fi

if curl -s -m 30 "http://127.0.0.1:$PORT/libsrc?lib=tempctrl&file=atc_tempctrl.st" -o "$W/lib.st" \
   && grep -q "FUNCTION_BLOCK FB_AtcProfile" "$W/lib.st" \
   && curl -s -o /dev/null -w '%{http_code}' "http://127.0.0.1:$PORT/libsrc?lib=tc2&file=..%2F..%2Fevil" | grep -q '^4'
then ok "proxy /libsrc: serves source, rejects traversal"
else bad "proxy /libsrc"; head -c 200 "$W/lib.st" 2>/dev/null; fi

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
