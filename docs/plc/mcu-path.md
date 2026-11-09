# MCU path

> As-built reference. What exists today for no-`dlopen` targets,
> what is proven, and what remains open. The delivery mechanisms
> below are design space, not commitments — the ABI supports all of
> them because it never assumed ELF, mmap, or a filesystem.

## What exists

### Target-object emission

`tplcc build --target <triple>` selects the LLVM target machine for
the module's `target_obj`. For the host triple the object is linked
into a deployable `.so` (rustc runtime + `cc -shared`); for any
other triple the `target_obj` is the raw relocatable ELF — the
native-only link step is skipped and the manifest records
`target_triple` for tooling.

Example: `tplcc build p.st --target armv7em-none-eabihf` emits a
32-bit ARM `ET_REL` object (machine `EM_ARM`) carrying `.text`,
`.rodata`, `.data`, and `.ARM.exidx`.

The module's ABI context offsets are **computed per target** from
the LLVM data layout (`AbiOffsets` in `tplc-codegen`): pointer size
and i64 alignment determine where `TplcCtx`, `TplcTaskCtx`,
`TplcServices`, and `TplcMigrateCtx` members sit. Verified against
`offsetof()` compiled by gcc (LP64) and arm-none-eabi-gcc
(AAPCS32) — without this, generated GEPs would read the wrong
members on 32-bit targets.

### Bare-metal link

The ARM `target_obj` links into a bare-metal ELF32 EXEC with
`arm-none-eabi-gcc`. The only undefined symbol is
`__aeabi_unwind_cpp_pr0` — LLVM emits `.ARM.exidx` unconditionally
for ARM EHABI; a loader or link shim provides a stub (the generated
code never unwinds through it).

### Execution under qemu-arm

`tests/abi/mcu_exec.sh` (ctest `mcu_exec`) compiles a `tplcc
--target armv7em-none-eabihf` package's `target_obj` together with
`tests/abi/mcu_exec_host.c` into a bare-metal ELF and runs it under
`qemu-arm` user mode (`QEMU_ARM` env or PATH; skips with exit 77
when absent).

The harness stands in for the mini-loader: static SRAM buffers for
state blob, image in/out, and markers; a `TplcServices` function
table; descriptor-driven io-binding copies. It exercises
`tplc_abi_version`, `tplc_get_descriptor`, `tplc_init`,
`tplc_task_run` over multiple scan cycles, and `tplc_shutdown`, and
asserts the `%QD` image output reflects the scan count. This is
real Cortex-M instruction execution — the same object a loader
would place in SRAM.

### Static-link ABI shim

`tests/abi/mcu_shim_host.c` (ctest `mcu_shim`) statically links
two module images into one host binary, resolves the ABI through a
function table, and drives init/task/abort/blob-relocation/migrate.
The gate asserts the binary imports no `dlopen`/`dlsym`/`mmap`/
files/threads symbols — the standing proof that generated code and
the ABI carry no Linux-only assumptions. `tests/abi/mcu_shim_arm.sh`
(ctest `mcu_shim_arm`) cross-compiles the same host and stub modules
with `arm-none-eabi-gcc` and runs the full conformance sequence under
`qemu-arm` — the ABI surface verified on a Cortex-M ISA, not just
the host.

## What this proves

- The canonical IR compiles to Cortex-M machine code that runs.
- The six-function ABI surface (`tplc_abi_version`,
  `tplc_get_descriptor`, `tplc_init`, `tplc_shutdown`,
  `tplc_task_run`, `tplc_migrate`) is sufficient on bare metal —
  no runtime services beyond the `TplcServices` table.
- State blob + image + code fit in static SRAM buffers; pointer-free
  blob addressing survives relocation by construction.
- Online change needs no ELF machinery: `tplc_migrate` is just
  another exported function the loader calls.

## Open items (honest gaps)

- **PIC-blob + fixup table** (§2.1 of the retired chapter): not
  implemented. The current path emits a relocatable `ET_REL` meant
  for link-time placement; a loadable-blob variant with a fixup
  table for RAM placement is future work.
- **Mini-loader**: no production loader exists. `mcu_exec_host.c`
  is a test harness, not firmware. A real loader must validate the
  package, verify the signature (see `crypto-auth.md` — verification
  belongs in the loader, which is the trust boundary), place the
  blob, wire the `svc` table, and call `tplc_init`.
- **On-target hardware**: qemu-arm user mode proves instruction
  execution, not silicon behavior (no MPU, no real SRAM layout, no
  flash retain). No board has been run.
- **Cross-compiled shim**: done — `mcu_shim_arm` runs the same
  conformance host on ARM under qemu-arm. What remains open is a
  real board, not another emulator.
- **MCU runtime crate** (`tplc-mcu`): scheduler, `ImageBackend`
  for SRAM/RPMsg transports, flash-backed retain — not started.

## Coverage

| test | proves |
|---|---|
| `mcu_target` | `--target` emits ARM `ET_REL` `target_obj`, manifest records triple, host `.so` unchanged, bad triple rejected |
| `mcu_link` | ARM object links bare-metal ELF32; six ABI exports resolve; only unwind stub needed |
| `mcu_exec` | ARM module executes init/task/io under qemu-arm (skips without qemu-arm) |
| `mcu_shim` | ABI has no Linux-only imports; static-link conformance on host |
| `mcu_shim_arm` | same shim conformance cross-compiled for ARM, run under qemu-arm |
| `abi_offsets` unit tests | generated struct offsets match `offsetof()` on LP64 and AAPCS32 |
