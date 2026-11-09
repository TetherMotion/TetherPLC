#!/usr/bin/env bash
# mcu_exec.sh — plan 18 criterion 2/4 progress: compile a tplcc
# --target ARM object together with the exec harness into a bare-metal
# ELF and RUN it under qemu-arm user mode. Proves the emitted module
# executes the ABI surface (init/task_run/io bindings) on a Cortex-M
# instruction set with static SRAM buffers — the mini-loader's job,
# minus the loader itself.
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

# blob counter + %QD0 image output: after N scans image_out == N
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
"$TPLCC" build "$WORK/p.st" -o "$WORK/p.tplc" \
    --target armv7em-none-eabihf >/dev/null 2>&1 || exit 77

# extract the target_obj payload (TOC: 160B header, 32B entries)
python3 - "$WORK/p.tplc" "$WORK/prog.o" <<'PY' || exit 1
import struct, sys
d = open(sys.argv[1], 'rb').read()
toc_n = struct.unpack_from('<I', d, 16)[0]
for i in range(toc_n):
    o = 160 + i*32
    t, fl, off, sz, sv, pd = struct.unpack_from('<IIQQII', d, o)
    if t == 0x10:
        open(sys.argv[2], 'wb').write(d[off:off+sz])
        sys.exit(0)
sys.exit('no target_obj')
PY

# LLVM emits an ARM EHABI unwind reference; the mini-loader stubs it
cat > "$WORK/rt_stub.c" <<'EOF'
void __aeabi_unwind_cpp_pr0(void) {}
EOF

# nonzero text segment: qemu-arm user mode cannot map vaddr 0
arm-none-eabi-gcc -O2 -specs=rdimon.specs -I include \
    -Wl,-Ttext-segment=0x10000 \
    tests/abi/mcu_exec_host.c "$WORK/rt_stub.c" "$WORK/prog.o" \
    -o "$WORK/mcu_exec.elf" 2>"$WORK/cc.err" \
    || { bad "cross-compile failed: $(cat "$WORK/cc.err")"; \
         echo "== $pass pass, $fail fail"; exit 1; }
ok "ARM exec image linked"

out=$("$QEMU_ARM" "$WORK/mcu_exec.elf" 2>&1); rc=$?
echo "$out" | grep -q '== PASS (0 fail)' && [ "$rc" -eq 0 ] \
    && ok "ARM module runs init/task/io under qemu-arm" \
    || bad "qemu-arm run rc=$rc: $out"

echo "$out" | grep -c '^PASS' | xargs -I{} echo "exec checks: {}"
echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
