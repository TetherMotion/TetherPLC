#!/usr/bin/env bash
# signature policy conformance (doc 15 §4.1, acceptance #2):
#   open            — unsigned loads, signed verified
#   require         — unsigned → TPLC_E_SIG_REQUIRED, signed loads
#   require-trusted — signer key_id must be in the trust store
# Uses a fixture package (no LLVM needed); skips when tplcc is absent.
set -uo pipefail
cd "$(dirname "$0")/../.."
ROOT="$PWD"
BUILD="${BUILD:-$ROOT/build}"
TPLCC="${TPLCC:-$ROOT/compiler/target/debug/tplcc}"
FIX="$ROOT/tests/pkg/fixtures/sample.tplc"

for f in "$BUILD/tether-plc" "$BUILD/tplc-image" "$FIX"; do
    [ -f "$f" ] || { echo "SKIP: $f missing"; exit 77; }
done
[ -x "$TPLCC" ] || { echo "SKIP: tplcc not built"; exit 77; }
ldd "$BUILD/tether-plc" 2>/dev/null | grep -q libsodium \
    || { echo "SKIP: no libsodium backend"; exit 77; }

WORK="$(mktemp -d /tmp/tplc_sig.XXXXXX)"
trap 'rm -rf "$WORK"; "$BUILD"/tplc-image unlink sig_test_img 2>/dev/null' EXIT
cp "$FIX" "$WORK/prog.tplc"

fail=0
run() { timeout 4 "$BUILD/tether-plc" --image sig_test_img \
            --package "$1" --ctl none "${@:2}" 2>&1; }
pass() { echo "PASS $1"; }
failf(){ echo "FAIL $1"; fail=$((fail+1)); }

"$BUILD/tplc-image" create sig_test_img 64 64 >/dev/null
"$TPLCC" sign keygen --dir "$WORK/trust" --name dev >/dev/null
"$TPLCC" sign sign "$WORK/prog.tplc" --key "$WORK/trust/dev.key" \
    -o "$WORK/prog.signed.tplc" >/dev/null

out=$(run "$WORK/prog.tplc" --sig-policy open)
echo "$out" | grep -q "signature policy" \
    && failf "unsigned+open: $out" || pass "unsigned+open loads"

out=$(run "$WORK/prog.tplc" --sig-policy require)
echo "$out" | grep -q -- "-207" \
    && pass "unsigned+require → SIG_REQUIRED" || failf "unsigned+require: $out"

out=$(run "$WORK/prog.tplc" --sig-policy require-trusted \
        --trust-dir "$WORK/trust")
echo "$out" | grep -q -- "-207" \
    && pass "unsigned+req-trusted → SIG_REQUIRED" \
    || failf "unsigned+req-trusted: $out"

out=$(run "$WORK/prog.signed.tplc" --sig-policy require)
echo "$out" | grep -q "signature policy" \
    && failf "signed+require: $out" || pass "signed+require loads"

out=$(run "$WORK/prog.signed.tplc" --sig-policy require-trusted \
        --trust-dir "$WORK/trust")
echo "$out" | grep -q "signature policy" \
    && failf "signed+req-trusted trusted: $out" \
    || pass "signed+req-trusted trusted loads"

out=$(run "$WORK/prog.signed.tplc" --sig-policy require-trusted \
        --trust-dir "$WORK")
echo "$out" | grep -q -- "-209" \
    && pass "signed+untrusted → SIG_UNTRUSTED" \
    || failf "signed+untrusted: $out"

# tampered signature payload: kid/pk/sig corruption → SIG_INVALID
python3 - "$WORK/prog.signed.tplc" "$WORK/prog.tampered.tplc" <<'EOF'
import sys,struct
d=bytearray(open(sys.argv[1],'rb').read())
tc=struct.unpack_from('<I',d,16)[0]
for i in range(tc):
    st,sf,o,s,sv,aux=struct.unpack_from('<IIQQII',d,160+i*32)
    if st==0x60: d[o+40]^=0x40            # pubkey byte
open(sys.argv[2],'wb').write(d)
EOF
out=$(run "$WORK/prog.tampered.tplc" --sig-policy require)
echo "$out" | grep -qE -- "-20[48]" \
    && pass "tampered sig rejected" || failf "tampered sig: $out"

# unsigned fixture stays a fixture: verify reports unsigned
"$TPLCC" sign verify "$WORK/prog.tplc" >/dev/null 2>&1 \
    && { failf "tplcc verify accepted unsigned"; } \
    || pass "tplcc verify rejects unsigned"

echo "== sig_policy: $fail failures"
[ "$fail" -eq 0 ]
