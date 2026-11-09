#!/usr/bin/env bash
# mcu_blob.sh — PIC-blob path: tplcc --target ARM --pic-blob emits a
# TPLB loadable image (no target linker); the reference loader
# tplb_load.c places it in SRAM, resolves libgcc imports by name,
# applies the fixup table, and the module runs its ABI surface under
# qemu-arm user mode.
# Requires: tplcc, arm-none-eabi-gcc, qemu-arm (QEMU_ARM env or PATH;
# exit 77 = skip).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-compiler/target/debug/tplcc}
QEMU_ARM=${QEMU_ARM:-$(command -v qemu-arm || true)}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

command -v arm-none-eabi-gcc >/dev/null || { echo "SKIP no arm-none-eabi-gcc"; exit 77; }
[ -n "$QEMU_ARM" ] && [ -x "$QEMU_ARM" ] || { echo "SKIP no qemu-arm"; exit 77; }
[ -x "$TPLCC" ] || { echo "SKIP no tplcc"; exit 77; }

# LREAL math forces __muldf3/__adddf3 imports (Cortex-M4 FPU is
# single-precision only); the counter exercises image IO through
# the ABS32-fixed descriptor pointers.
cat > "$WORK/p.st" <<'EOF'
PROGRAM BlobDemo
VAR
    xin AT %IL0 : LREAL;
    cnt AT %QD8 : DINT;
    y   AT %QL16 : LREAL;
    acc : LREAL := 0.0;
    n   : DINT := 0;
END_VAR
n := n + 1;
acc := acc + xin * 1.5 + 2.0;
y := acc;
cnt := n;
END_PROGRAM
EOF
"$TPLCC" build "$WORK/p.st" -o "$WORK/p.tplc" \
    --target armv7em-none-eabihf --pic-blob >/dev/null 2>&1 \
    || { bad "tplcc --pic-blob build failed"; \
         echo "== $pass pass, $fail fail"; exit 1; }
ok "tplcc --pic-blob package built"

# extract the target_obj payload (TOC: 160B header, 32B entries);
# with --pic-blob it is a TPLB image, not an ELF object
python3 - "$WORK/p.tplc" "$WORK/prog.tplb" <<'PY' || exit 1
import struct, sys
d = open(sys.argv[1], 'rb').read()
toc_n = struct.unpack_from('<I', d, 16)[0]
for i in range(toc_n):
    o = 160 + i*32
    t, fl, off, sz, sv, pd = struct.unpack_from('<IIQQII', d, o)
    if t == 0x10:
        blob = d[off:off+sz]
        assert blob[:4] == b'TPLB', 'target_obj is not a TPLB blob'
        open(sys.argv[2], 'wb').write(blob)
        sys.exit(0)
sys.exit('no target_obj')
PY
ok "target_obj is a TPLB blob"

# manifest must record the format
python3 - "$WORK/p.tplc" <<'PY' || exit 1
import struct, sys, json
d = open(sys.argv[1], 'rb').read()
toc_n = struct.unpack_from('<I', d, 16)[0]
for i in range(toc_n):
    o = 160 + i*32
    t, fl, off, sz, sv, pd = struct.unpack_from('<IIQQII', d, o)
    if t == 0x01:
        m = json.loads(d[off:off+sz])
        assert m.get('target_obj_format') == 'tplb', m
        sys.exit(0)
sys.exit('no manifest')
PY
ok "manifest records target_obj_format=tplb"

# -mcpu=cortex-m4 -mthumb: the blob's THM_CALL fixups emit raw Thumb
# BL, so libgcc soft-float imports must come from the Thumb multilib
arm-none-eabi-gcc -O2 -mcpu=cortex-m4 -mthumb -specs=rdimon.specs \
    -I include \
    -Wl,-Ttext-segment=0x10000 \
    tests/abi/mcu_blob_host.c tests/abi/tplb_load.c \
    -o "$WORK/mcu_blob.elf" 2>"$WORK/cc.err" \
    || { bad "cross-compile failed: $(cat "$WORK/cc.err")"; \
         echo "== $pass pass, $fail fail"; exit 1; }
ok "blob harness cross-compiled"

out=$("$QEMU_ARM" "$WORK/mcu_blob.elf" "$WORK/prog.tplb" 2>&1); rc=$?
echo "$out"
echo "$out" | grep -q '== PASS (0 fail)' && [ "$rc" -eq 0 ] \
    && ok "TPLB blob loads + module runs under qemu-arm" \
    || bad "qemu-arm run rc=$rc"

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
