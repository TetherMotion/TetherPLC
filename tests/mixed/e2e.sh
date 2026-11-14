#!/usr/bin/env bash
# Mixed FBD+ST compilation: one source set containing project.fbd.json
# plus .st files compiles to a single module — FBD blocks instantiate
# ST FBs/FUNCTIONs, ST programs call FBD-defined POUs, and inline `st`
# blocks expand to generated FUNCTION_BLOCKs. Exit 77 = no toolchain.
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
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }

D=tests/mixed
W=$(mktemp -d)
trap 'rm -rf "$W"' EXIT
pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

# 1. check: FBD program + ST lib + ST program in one module
if "$TPLCC" check "$D/project.fbd.json" "$D/lib.st" "$D/stprog.st" >"$W/check.log" 2>&1 \
   && grep -q "ok: 6 pou" "$W/check.log"; then
    ok "mixed check (6 pous)"
else
    bad "mixed check"; cat "$W/check.log"
fi

# 2. build: ST program calling the FBD-defined FB
if "$TPLCC" build "$D/fbfb.fbd.json" "$D/stprog.st" -o "$W/st_main.tplc" >"$W/b2.log" 2>&1 \
   && [ -s "$W/st_main.tplc" ]; then
    ok "build: st program + fbd fb"
else
    bad "build: st program + fbd fb"; cat "$W/b2.log"
fi

# 4. multi-program package: two programs (one FBD, one ST) in one image
if "$TPLCC" build "$D/project.fbd.json" "$D/lib.st" "$D/stprog.st" -o "$W/multi.tplc" >"$W/b3.log" 2>&1 \
   && [ -s "$W/multi.tplc" ]; then
    ok "build: multi-program package"
else
    bad "build: multi-program package"; cat "$W/b3.log"
fi

# 5. duplicate implementation across languages → TPLC_E_MIX_DUP_POU
if "$TPLCC" check "$D/dup.fbd.json" "$D/dup.st" >"$W/dup.log" 2>&1; then
    bad "dup impl rejected"
elif grep -q "TPLC_E_MIX_DUP_POU" "$W/dup.log"; then
    ok "dup impl rejected"
else
    bad "dup impl rejected (wrong diag)"; cat "$W/dup.log"
fi

# 6. iface-mismatched decl stub → TPLC_E_MIX_IFACE
python3 - "$D/project.fbd.json" >"$W/mismatch.fbd.json" <<'PY'
import json, sys
p = json.load(open(sys.argv[1]))
for pou in p["pous"]:
    if pou["name"] == "StFb":
        pou["interface"][0]["type"] = "INT"
p["pous"] = [x for x in p["pous"] if x["name"] != "FbdFb"]
json.dump(p, sys.stdout)
PY
if "$TPLCC" check "$W/mismatch.fbd.json" "$D/lib.st" >"$W/mis.log" 2>&1; then
    bad "iface mismatch rejected"
elif grep -q "TPLC_E_MIX_IFACE" "$W/mis.log"; then
    ok "iface mismatch rejected"
else
    bad "iface mismatch rejected (wrong diag)"; cat "$W/mis.log"
fi

# 7. TetherWebUI curve-editor output: FB_AtcProfile + inline `st` config
# block wired through a var_in_out pin + FB_AtcPid, against the shipped
# temperature library. Exercises aggregate inout member writes.
if "$TPLCC" build "$D/ui_profile.fbd.json" libraries/tempctrl/atc_tempctrl.st \
        -o "$W/ui_profile.tplc" >"$W/ui.log" 2>&1 && [ -s "$W/ui_profile.tplc" ]; then
    ok "build: ui curve editor profile"
else
    bad "build: ui curve editor profile"; cat "$W/ui.log"
fi

# 7b. Same editor output plus the status taps (Busy/Done) and the
# FB_AtcPwm heater stage (1 Hz period, duty from the controller).
if "$TPLCC" build "$D/ui_profile_pwm.fbd.json" libraries/tempctrl/atc_tempctrl.st \
        -o "$W/ui_profile_pwm.tplc" >"$W/ui_pwm.log" 2>&1 && [ -s "$W/ui_profile_pwm.tplc" ]; then
    ok "build: ui curve profile+pwm"
else
    bad "build: ui curve profile+pwm"; cat "$W/ui_pwm.log"
fi

# 8. TetherWebUI autotune-editor output: tuner FBs + inline `st`
# handoff (VAR_IN_OUT FB pins, live mirrors, APPLY gain copy) for all
# three tuner shapes — relay, step-identification pair, LQG pair.
for t in ui_tune ui_tune_step ui_tune_lqg; do
    if "$TPLCC" build "$D/$t.fbd.json" libraries/tempctrl/atc_tempctrl.st \
            -o "$W/$t.tplc" >"$W/$t.log" 2>&1 && [ -s "$W/$t.tplc" ]; then
        ok "build: ui autotune $t"
    else
        bad "build: ui autotune $t"; cat "$W/$t.log"
    fi
done

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
