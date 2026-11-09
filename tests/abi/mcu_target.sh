#!/usr/bin/env bash
# mcu_target.sh — `tplcc build --target <triple>` must emit a
# monolithic object for that triple as target_obj (plan 18 criterion 1):
# the package validates, the manifest records the triple, and the
# embedded ELF is a relocatable ARM object, not a linked .so.
# Requires: tplcc (exit 77 = prerequisites missing).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-compiler/target/debug/tplcc}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

[ -x "$TPLCC" ] || { echo "SKIP no tplcc"; exit 77; }

cat > "$WORK/p.st" <<'EOF'
PROGRAM Demo
VAR
    cnt : DINT := 0;
    out AT %QD0 : DINT;
END_VAR
cnt := cnt + 1;
out := cnt;
END_PROGRAM
EOF

# ARM Cortex-M target: package + ARM relocatable object
"$TPLCC" build "$WORK/p.st" -o "$WORK/arm.tplc" \
    --target armv7em-none-eabihf >/dev/null 2>"$WORK/arm.err" \
    || bad "arm build failed: $(cat "$WORK/arm.err")"
[ -f "$WORK/arm.tplc" ] || { echo "== $pass pass, $fail fail"; exit 1; }

# manifest records the triple
"$TPLCC" info "$WORK/arm.tplc" 2>/dev/null | grep -q \
    '"target_triple":"armv7em-none-eabihf"' \
    && ok "manifest records the target triple" \
    || bad "manifest lacks target_triple"

# the embedded target_obj is a 32-bit ARM relocatable ELF
python3 - "$WORK/arm.tplc" <<'PY' && ok "target_obj is ARM relocatable ELF" \
|| bad "target_obj is not an ARM relocatable"
import struct, sys
d = open(sys.argv[1], 'rb').read()
idx = d.find(b'\x7fELF')
assert idx >= 0, 'no ELF in package'
et = struct.unpack('<H', d[idx+16:idx+18])[0]
mach = struct.unpack('<H', d[idx+18:idx+20])[0]
cls = d[idx+4]
assert cls == 1, f'ELF class {cls} (want 32-bit)'
assert mach == 40, f'ELF machine {mach} (want 40=ARM)'
assert et == 1, f'ELF type {et} (want 1=REL, not a linked .so)'
PY

# host build still produces the deployable .so path
"$TPLCC" build "$WORK/p.st" -o "$WORK/host.tplc" >/dev/null 2>&1 \
    || bad "host build failed"
python3 - "$WORK/host.tplc" <<'PY' && ok "host target_obj stays a .so" \
|| bad "host target_obj changed"
import struct, sys
d = open(sys.argv[1], 'rb').read()
idx = d.find(b'\x7fELF')
et = struct.unpack('<H', d[idx+16:idx+18])[0]
mach = struct.unpack('<H', d[idx+18:idx+20])[0]
assert et == 3, f'host ELF type {et} (want 3=DYN .so)'
assert mach == 62, f'host ELF machine {mach} (want 62=x86-64)'
PY

# an unknown triple is a clean error, not a panic
"$TPLCC" build "$WORK/p.st" -o "$WORK/bad.tplc" \
    --target not-a-real-triple >/dev/null 2>"$WORK/bad.err"
rc=$?
[ "$rc" -ne 0 ] && ok "bad triple fails cleanly (rc=$rc)" \
    || bad "bad triple unexpectedly succeeded"

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
