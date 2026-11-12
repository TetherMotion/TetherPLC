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
