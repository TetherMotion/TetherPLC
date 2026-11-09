#!/usr/bin/env bash
# update_auth.sh — plan 20 crypto authentication: --update-auth strict
# makes every update op (prepare/swap/shadow-load) require a trusted
# signature regardless of --sig-policy; the trust store supports key
# rotation (multiple keys) and revocation (<trust-dir>/revoked).
# Requires: build tree + tplcc (exit 77 = prerequisites missing).
set -uo pipefail
cd "$(dirname "$0")/../.."

BUILD=${BUILD:-build}
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
IMG=/tplc_upauth
CTL=/tmp/tplc_upauth.ctl
WORK=$(mktemp -d)
RT_PID=
cleanup() {
    [ -n "$RT_PID" ] && kill "$RT_PID" 2>/dev/null
    "$BUILD"/tplc-image unlink "$IMG" 2>/dev/null
    rm -rf "$WORK"
}
trap cleanup EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

[ -x "$BUILD/tether-plc" ] && [ -x "$BUILD/tplc-ctl" ] && [ -x "$TPLCC" ] \
    || { echo "SKIP prerequisites"; exit 77; }

# two programs, identical layout (swap needs no migrate path)
cat > "$WORK/a.st" <<'EOF'
PROGRAM Demo
VAR
    cnt AT %QD0 : DINT;
END_VAR
cnt := cnt + 1;
END_PROGRAM
EOF
cat > "$WORK/b.st" <<'EOF'
PROGRAM Demo
VAR
    cnt AT %QD0 : DINT;
END_VAR
cnt := cnt + 10;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/a.st" -o "$WORK/a.tplc" >/dev/null 2>&1 || exit 77
"$TPLCC" build "$WORK/b.st" -o "$WORK/b.tplc" >/dev/null 2>&1 || exit 77

# keys: dev stays OUT of the trust dir; ops + rot are trusted
"$TPLCC" sign keygen --dir "$WORK/devkeys" --name dev >/dev/null 2>&1 || exit 77
"$TPLCC" sign keygen --dir "$WORK/trust" --name ops >/dev/null 2>&1 || exit 77
"$TPLCC" sign keygen --dir "$WORK/trust" --name rot >/dev/null 2>&1 || exit 77

"$TPLCC" sign sign "$WORK/a.tplc" --key "$WORK/devkeys/dev.key" \
    -o "$WORK/a_dev.tplc" >/dev/null 2>&1 || exit 77
"$TPLCC" sign sign "$WORK/a.tplc" --key "$WORK/trust/ops.key" \
    -o "$WORK/a_ops.tplc" >/dev/null 2>&1 || exit 77
"$TPLCC" sign sign "$WORK/b.tplc" --key "$WORK/trust/rot.key" \
    -o "$WORK/b_rot.tplc" >"$WORK/rot.id" 2>/dev/null || exit 77
ROT_KID=$(sed -n 's/.*key_id \([0-9a-f]*\).*/\1/p' "$WORK/rot.id")

"$BUILD"/tplc-image create "$IMG" 64 64 >/dev/null 2>&1
# boot unsigned under open policy; updates are strict
"$BUILD"/tether-plc --image "$IMG" --package "$WORK/a.tplc" \
    --ctl "$CTL" --no-sandbox --sig-policy open \
    --update-auth strict --trust-dir "$WORK/trust" \
    >"$WORK/rt.log" 2>&1 &
RT_PID=$!
for i in $(seq 1 50); do [ -S "$CTL" ] && break; sleep 0.1; done
[ -S "$CTL" ] || { echo "SKIP ctl socket"; exit 77; }

swap() { "$BUILD"/tplc-ctl "$CTL" swap "$1" 2>&1; }

out=$(swap "$WORK/a.tplc")
echo "$out" | grep -q 'error 207' \
    && ok "strict: unsigned swap → SIG_REQUIRED" \
    || bad "unsigned swap under strict: $out"

out=$(swap "$WORK/a_dev.tplc")
echo "$out" | grep -q 'error 209' \
    && ok "strict: untrusted signer → SIG_UNTRUSTED" \
    || bad "untrusted swap under strict: $out"

out=$(swap "$WORK/a_ops.tplc")
echo "$out" | grep -q 'out_status=0' \
    && ok "strict: trusted signer accepted (rotation key 1/2)" \
    || bad "trusted swap under strict: $out"

# rotation: rot was already trusted at boot — a second trusted key
out=$(swap "$WORK/b_rot.tplc")
echo "$out" | grep -q 'out_status=0' \
    && ok "rotation: second trusted key accepted" \
    || bad "rotation swap: $out"

# revocation: ban rot's key id; its .pub stays in trust dir
echo "$ROT_KID" > "$WORK/trust/revoked"
out=$(swap "$WORK/a_ops.tplc")
echo "$out" | grep -q 'out_status=0' \
    && ok "revocation: ops key still works after rot revoked" \
    || bad "post-revoke trusted swap: $out"

# sign a third package with the revoked key → must now fail
cat > "$WORK/c.st" <<'EOF'
PROGRAM Demo
VAR
    cnt AT %QD0 : DINT;
END_VAR
cnt := cnt + 100;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/c.st" -o "$WORK/c.tplc" >/dev/null 2>&1
"$TPLCC" sign sign "$WORK/c.tplc" --key "$WORK/trust/rot.key" \
    -o "$WORK/c_rot.tplc" >/dev/null 2>&1
out=$(swap "$WORK/c_rot.tplc")
echo "$out" | grep -q 'error 209' \
    && ok "revocation: revoked key rejected despite .pub present" \
    || bad "revoked-key swap: $out"

kill "$RT_PID" 2>/dev/null; RT_PID=
wait 2>/dev/null

# control: without --update-auth, unsigned updates still load under open
echo "" > "$WORK/trust/revoked"
"$BUILD"/tether-plc --image "$IMG" --package "$WORK/a.tplc" \
    --ctl "$CTL" --no-sandbox --sig-policy open \
    --trust-dir "$WORK/trust" >"$WORK/rt2.log" 2>&1 &
RT_PID=$!
for i in $(seq 1 50); do [ -S "$CTL" ] && break; sleep 0.1; done
out=$(swap "$WORK/b.tplc")
echo "$out" | grep -q 'out_status=0' \
    && ok "control: open policy still accepts unsigned updates" \
    || bad "open-policy unsigned swap: $out"
kill "$RT_PID" 2>/dev/null; RT_PID=
wait 2>/dev/null

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
