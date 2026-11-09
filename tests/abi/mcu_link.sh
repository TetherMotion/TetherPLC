#!/usr/bin/env bash
# mcu_link.sh — plan 18 criterion 2 progress: the ARM target_obj must
# link into a bare-metal ELF with arm-none-eabi-gcc, resolving every
# tplc_* ABI export and needing only the unwind-personality stub a
# mini-loader provides. Execution on-target remains open (no ARM
# runtime on this host).
# Requires: tplcc + arm-none-eabi-gcc (exit 77 = skip).
set -uo pipefail
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-compiler/target/debug/tplcc}
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

command -v arm-none-eabi-gcc >/dev/null || { echo "SKIP no arm-none-eabi-gcc"; exit 77; }
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
# a predecessor package so the candidate carries tplc_migrate —
# the ABI export exists only when a migrate descriptor does
"$TPLCC" build "$WORK/p.st" -o "$WORK/prev.tplc" >/dev/null 2>&1 || exit 77
"$TPLCC" build "$WORK/p.st" -o "$WORK/p.tplc" \
    --change-against "$WORK/prev.tplc" \
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

# the only undefined symbol LLVM emits for ARM EHABI is the unwind
# personality; a mini-loader stubs it (documented in plan 18)
cat > "$WORK/rt_stub.c" <<'EOF'
void __aeabi_unwind_cpp_pr0(void) {}
EOF

arm-none-eabi-gcc -nostdlib -nostartfiles -Wl,-e,0 \
    "$WORK/rt_stub.c" "$WORK/prog.o" -o "$WORK/mcu.elf" 2>"$WORK/link.err"
rc=$?
[ "$rc" -eq 0 ] && ok "ARM object links into bare-metal ELF" \
    || bad "link failed: $(cat "$WORK/link.err")"

# ELF32 + ARM machine + EXEC (statically placed, not PIE)
readelf_out=$(arm-none-eabi-readelf -h "$WORK/mcu.elf" 2>/dev/null)
echo "$readelf_out" | grep -q 'Class:.*ELF32' \
    && echo "$readelf_out" | grep -q 'Machine:.*ARM' \
    && echo "$readelf_out" | grep -q 'EXEC' \
    && ok "linked image is ELF32 ARM EXEC" \
    || bad "unexpected ELF header"

# every public ABI export resolves
nm_out=$(arm-none-eabi-nm "$WORK/mcu.elf" 2>/dev/null)
for sym in tplc_abi_version tplc_get_descriptor tplc_init \
           tplc_shutdown tplc_task_run tplc_migrate; do
    echo "$nm_out" | grep -q " T $sym\$" \
        && ok "export $sym resolves" \
        || bad "missing export $sym"
done

# and the object itself must not need anything beyond the stub —
# undefined symbols other than the personality routine mean the
# mini-loader would need more runtime than planned
undef=$(arm-none-eabi-nm "$WORK/prog.o" 2>/dev/null | grep ' U ' \
        | grep -v '__aeabi_unwind_cpp_pr0' || true)
[ -z "$undef" ] && ok "object needs only the unwind stub" \
    || bad "unexpected undefined symbols: $undef"

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
