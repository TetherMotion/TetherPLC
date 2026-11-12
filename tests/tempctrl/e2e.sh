#!/usr/bin/env bash
# e2e.sh — Atc_TempCtrl library: package, consume via --lib, and run
# the semantic suite (self-checking programs; each emits pass/fail
# bits on %QB0 — see vectors.jsonl).
#
# Covers: PID + simulated plant, relay + step-response autotune,
# IMC/SIMC/ZN rule checks, LQR/LQG against scipy-verified DARE
# references (Kleinman iteration in FB_AtcDare3), Kalman gains,
# cascade + Smith predictor loops, and the ramp/soak/loop profile
# programmer.
#
# Requires: tplcc, tplc-pkg (skip 77 otherwise).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-compiler/target/debug/tplcc}
PKG=${PKG:-compiler/target/debug/tplc-pkg}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

[ -x "$TPLCC" ] || { echo "SKIP no tplcc"; exit 77; }
[ -x "$PKG" ]   || { echo "SKIP no tplc-pkg"; exit 77; }

LIB_ST=libraries/tempctrl/atc_tempctrl.st

# --- compile check ---------------------------------------------------
"$TPLCC" check "$LIB_ST" >/dev/null 2>&1 \
    && ok "tplcc check atc_tempctrl.st" \
    || bad "tplcc check atc_tempctrl.st"

# --- library package -------------------------------------------------
"$TPLCC" build --library --name Atc_TempCtrl "$LIB_ST" \
    -o "$WORK/atc_tempctrl.tplc" >/dev/null 2>&1 \
    || { bad "tplcc build --library failed"; \
         echo "== $pass pass, $fail fail"; exit 1; }
ok "tplcc build --library Atc_TempCtrl"

"$PKG" verify "$WORK/atc_tempctrl.tplc" >/dev/null 2>&1 \
    && ok "tplc-pkg verify library" \
    || bad "tplc-pkg verify library"

INFO=$("$TPLCC" info "$WORK/atc_tempctrl.tplc" 2>/dev/null)
echo "$INFO" | grep -q '"kind":"library"' \
    && ok "manifest kind=library" \
    || bad "manifest kind=library"
echo "$INFO" | grep -q '"name":"Atc_TempCtrl"' \
    && ok "manifest name" \
    || bad "manifest name"

# --- consumer program via --lib --------------------------------------
cat > "$WORK/consumer.st" <<'EOF'
PROGRAM C01
VAR
    pid : FB_AtcPid;
    prof : FB_AtcProfile;
    flags AT %QB0 : BYTE;
    pv : LREAL;
    u AT %QD8 : LREAL;
    sp AT %QD16 : LREAL;
END_VAR
    prof.SEG[1].KIND := 2;
    prof.SEG[1].TGT := 80.0;
    prof.SEG[1].P1 := 10.0;
    prof.SEG[2].KIND := 1;
    prof.SEG[2].TGT := 80.0;
    prof.SEG[2].P1 := 30.0;
    prof.SEG[3].KIND := 0;
    prof(NUM_SEG := 3, RUN := TRUE, HOLD := FALSE, RESET := FALSE,
         PvStart := TRUE, PV := pv, CycleTime := 0.5);
    pid(Execute := TRUE, Setpoint := prof.SP, Actual := pv,
        Kp := 2.0, Tn := 20.0, Tv := 0.0,
        OutMin := 0.0, OutMax := 100.0,
        ManEn := FALSE, ManOut := 0.0, CycleTime := 0.5,
        Reset := FALSE);
    u := pid.Out;
    sp := prof.SP;
    pv := pv + (1.5 * u - pv) * (0.5 / (20.0 + 0.5));
    flags := 0;
    IF pv > 78.0 AND pv < 82.0 THEN flags := flags + 1; END_IF;
    IF pid.Done THEN flags := flags + 2; END_IF;
    IF NOT pid.Err THEN flags := flags + 4; END_IF;
    IF prof.Done OR prof.Busy THEN flags := flags + 8; END_IF;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/consumer.st" --lib "$WORK/atc_tempctrl.tplc" \
    -o "$WORK/consumer.tplc" >/dev/null 2>&1 \
    && ok "consumer build --lib" \
    || bad "consumer build --lib"

"$PKG" verify "$WORK/consumer.tplc" >/dev/null 2>&1 \
    && ok "tplc-pkg verify consumer" \
    || bad "tplc-pkg verify consumer"

# --- semantic suite --------------------------------------------------
# each cases/tNN_*.st is concatenated with the library and driven by
# vectors.jsonl; pass/fail lives on %QB0 (self-checking logic).
# NB: vectors resolves programs relative to the .jsonl's directory —
# the raw cases/ legs are not self-contained, so merged sources +
# vectors.jsonl are staged together in the workdir.
for t in tests/tempctrl/cases/t??_*.st; do
    n=$(basename "$t")
    cat "$LIB_ST" "$t" > "$WORK/$n"
done
cp tests/tempctrl/vectors.jsonl "$WORK/vectors.jsonl"
if "$TPLCC" vectors run "$WORK/vectors.jsonl" \
       > "$WORK/vec.log" 2>&1; then
    ok "tplcc vectors: tempctrl suite (12 programs)"
else
    bad "tplcc vectors: tempctrl suite"
    cat "$WORK/vec.log"
fi

# --- real-runtime replay ---------------------------------------------
# the --lib consumer program (profile -> PID -> simulated plant)
# driven through tether-plc-sim: profile ramps SP to 80, PID pulls
# the plant there, flags must read 0x0f once settled (~200 scans).
BUILD=${BUILD:-./build}
if [ -x "$BUILD/tether-plc-sim" ] && [ -x "$BUILD/tether-plc" ]; then
    {
        for i in $(seq 0 249); do
            t=$((i * 10000000))
            if [ "$i" -eq 249 ]; then
                echo "step $i time=$t in 0=00 expect 0=0f"
            else
                echo "step $i time=$t in 0=00"
            fi
        done
    } > "$WORK/consumer.steps"
    IMG=/tplc_tempctrl_$$
    "$BUILD/tether-plc-sim" --image "$IMG" \
        --package "$WORK/consumer.tplc" \
        --steps "$WORK/consumer.steps" \
        --plc "$BUILD/tether-plc" --ctl "$WORK/consumer.ctl" \
        --settle-ms 1500 > "$WORK/sim.log" 2>&1
    rc=$?
    if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
        ok "tether-plc-sim replay: profile->PID->plant"
    else
        bad "tether-plc-sim replay: profile->PID->plant"
        tail -20 "$WORK/sim.log"
    fi
else
    echo "SKIP tether-plc-sim/tether-plc not built (consumer leg)"
fi

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
