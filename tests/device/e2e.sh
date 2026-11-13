#!/usr/bin/env bash
# Device-block conformance: `device` FBD blocks alias located vars —
# DI channels feed logic, DO channels take logic outputs, analog
# channels pass REAL through the image. The vector trace drives %I and
# checks %Q cycle-for-cycle.
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }
command -v python3 >/dev/null || { echo "SKIP no python3"; exit 77; }
W=$(mktemp -d); trap 'rm -rf "$W"' EXIT

"$TPLCC" vectors run tests/device/vectors.jsonl --src tests/device

# EK → temperature controller → EK: the exact scenario the web editor
# produces — DI enable → FB_AtcPid → FB_AtcPwm → DO channel, plus an
# analog PV channel. The Rust test ek_device_pid_pwm_chain executes it;
# here we prove it checks and builds natively with the library.
"$TPLCC" check tests/device/ek_pid.fbd.json libraries/tempctrl/atc_tempctrl.st
"$TPLCC" build tests/device/ek_pid.fbd.json libraries/tempctrl/atc_tempctrl.st \
    -o "$W/ek_pid.tplc" --json | python3 -c "import json,sys; d=json.load(sys.stdin); assert d.get('ok'), d"
[ -s "$W/ek_pid.tplc" ] || { echo "FAIL: no package"; exit 1; }
echo "PASS ek_pid check+build"

# --- real-runtime replay ---------------------------------------------
# the EK chain through tether-plc + tether-plc-sim --scan: DI0/AI0 are
# written into the process image, DO0 is expected high inside the PWM
# on-window and low after it. PWM Period is shrunk to 0.5 s so the whole
# scenario fits the ctl TASK_STEP rate limit (20/s, burst 40); the
# 10 s period semantics are covered by the interpreter test.
BUILD=${BUILD:-./build}
if [ -x "$BUILD/tether-plc-sim" ] && [ -x "$BUILD/tether-plc" ]; then
    python3 - "$W" <<'PY'
import json, sys
w = sys.argv[1]
d = json.load(open('tests/device/ek_pid.fbd.json'))
for p in d['pous']:
    for n in p.get('networks', []):
        for b in n.get('blocks', []):
            if b.get('type_ref') == 'FB_AtcPwm':
                b['params']['Period'] = '0.5'   # Ton=0.1375 s @ duty 27.5
json.dump(d, open(w + '/ek_pid_fast.fbd.json', 'w'))
# step k -> scan k+1 -> t=(k+1)*10 ms. PWM on for t < 0.1375 s (scans
# 1-13), off until the 0.5 s wrap (scan 50). Probe inside each window.
lines = []
for i in range(30):
    t = i * 10000000
    exp = ''
    if i == 5:  exp = ' expect 0=01'   # t=0.06 s: DO0 high
    if i == 20: exp = ' expect 0=00'   # t=0.21 s: DO0 low
    lines.append(f'step {i} time={t} in 0=01 in 4=0000a041{exp}')
open(w + '/ek.steps', 'w').write('\n'.join(lines) + '\n')
PY
    "$TPLCC" build "$W/ek_pid_fast.fbd.json" libraries/tempctrl/atc_tempctrl.st \
        -o "$W/ek_pid_sim.tplc" --json | python3 -c "import json,sys; d=json.load(sys.stdin); assert d.get('ok'), d"
    "$BUILD/tether-plc-sim" --image "/tplc_ekdev_e2e_$$" \
        --package "$W/ek_pid_sim.tplc" --steps "$W/ek.steps" --scan \
        --ctl "$W/ek.ctl" --in 64 --out 64 --settle-ms 1500 \
        > "$W/sim.log" 2>&1
    if [ $? -eq 0 ] && grep -q "all matched" "$W/sim.log"; then
        echo "PASS ek_pid tether-plc-sim replay"
    else
        echo "FAIL ek_pid tether-plc-sim replay"; tail -15 "$W/sim.log"
        exit 1
    fi
else
    echo "SKIP tether-plc-sim/tether-plc not built (ek_pid replay leg)"
fi
