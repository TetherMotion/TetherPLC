#!/usr/bin/env bash
# mcu_shim_arm.sh — ARM variant of the MCU static-link shim: the same
# two stub modules + conformance host cross-compiled with
# arm-none-eabi-gcc and executed under qemu-arm user mode. Proves the
# ABI surface (init/task/migrate/io) works on a Cortex-M instruction
# set with static SRAM buffers, no dlopen/ELF/files/threads.
# Requires: arm-none-eabi-gcc, qemu-arm (QEMU_ARM env or PATH;
# exit 77 = skip).
set -uo pipefail
cd "$(dirname "$0")/../.."

QEMU_ARM=${QEMU_ARM:-$(command -v qemu-arm || true)}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

command -v arm-none-eabi-gcc >/dev/null || { echo "SKIP no arm-none-eabi-gcc"; exit 77; }
[ -n "$QEMU_ARM" ] && [ -x "$QEMU_ARM" ] || { echo "SKIP no qemu-arm"; exit 77; }

# per-version symbol renames, same as the host shim build
SHIM_ABI_SYMS="abi_version get_descriptor init shutdown task_run migrate"
build_stub() {  # $1=ver $2=name
    local defs="-DTPLC_STUB_V=$1"
    for s in $SHIM_ABI_SYMS; do
        defs="$defs -Dtplc_${s}=tplc_$2_${s}"
    done
    arm-none-eabi-gcc -O2 -specs=rdimon.specs \
        $defs -I include -I tests/abi -fno-exceptions \
        -c tests/abi/stub_module.c -o "$WORK/$2.o" 2>"$WORK/$2.err" \
        || { bad "stub $2 cross-compile: $(cat "$WORK/$2.err")"; return 1; }
}
build_stub 1 v1 || exit 1
build_stub 2 v2 || exit 1
ok "ARM stub modules compiled"

arm-none-eabi-gcc -O2 -specs=rdimon.specs \
    -I include -I tests/abi -fno-exceptions \
    -Wl,-Ttext-segment=0x10000 \
    tests/abi/mcu_shim_host.c "$WORK/v1.o" "$WORK/v2.o" \
    -o "$WORK/shim.elf" 2>"$WORK/link.err" \
    || { bad "shim link failed: $(cat "$WORK/link.err")"; \
         echo "== $pass pass, $fail fail"; exit 1; }
ok "ARM shim image linked"

out=$("$QEMU_ARM" "$WORK/shim.elf" 2>&1); rc=$?
echo "$out" | grep -c '^PASS' | xargs -I{} echo "shim checks: {}"
[ "$rc" -eq 0 ] && echo "$out" | grep -q '== PASS (0 fail)' \
    && ok "ARM shim conformance under qemu-arm" \
    || bad "qemu-arm run rc=$rc: $out"

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
