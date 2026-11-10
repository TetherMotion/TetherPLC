# TwinCAT compatibility layer

ST-source compatibility implementations of Beckhoff TwinCAT 3 PLC
libraries, packaged as `kind="library"` `.tplc` artifacts (plan 21,
C0/C1). Each library mirrors the documented API surface closely enough
for TwinCAT ST sources to compile and run unchanged where the
underlying semantics allow.

Per-library API references (parameters, descriptions, recursive
catalog) live in `twincat/` — generated from
`docs/plc/plan/21-twincat-libraries.md` by
`tools/twincat-docs/split_plan.py`.

## Layout

```
compat/tc2/           TwinCAT 2/3-era library sources
  tc2_math.st         Tc2_Math: CEIL FLOOR FRAC LTRUNC LMOD MODABS
                      MODTURNS F_GetVersionTcMath F_TruncL (helper)
  tc2_util_bcd.st     Tc2_Utilities subset: DEC_TO_BCD, BCD_TO_DEC
  tc2_util_enum.st    Tc2_Utilities: FB_EnumStringNumbers + F_IsDigit
  tc2_standard_wide.st Tc2_Standard W* wrappers (WCONCAT WDELETE
                      WFIND WINSERT WLEFT WLEN WMID WREPLACE WRIGHT)
  tc2_standard_lt.st  Tc2_Standard L* timers: LTON LTOF LTP wrapping
                      the TON/TOF/TP builtins (LTIME == TIME repr)
tests/compat/
  tc2_case.st         consumer program (28 packed checks)
  tc2.sh              ctest tc2_compat: build libs, --lib consume,
                      6-step sim replay expecting masks 0x1FFFFF/0xFF
```

## Building and consuming

```sh
tplcc build --library --name Tc2_Math compat/tc2/tc2_math.st \
    -o Tc2_Math.tplc
tplcc build my_program.st \
    --lib Tc2_Math.tplc --lib Tc2_Utilities_BCD.tplc \
    -o my_program.tplc
```

`--lib` extracts the package's `ST_SOURCES` into the compilation
unit, so library POUs resolve as ordinary user functions/FBs.
Binary (precompiled object) consumption is not wired yet; the
library object keeps relocatable `__tplc_pou_*` symbols for it.

## Status vs. plan 21 phases

| Phase | Scope | Status |
|-------|-------|--------|
| C0 | Library package format | done (`f912616`) |
| C1 | T0 core | tranches 1+2 done: Tc2_Math (8/8), Tc2_Standard W* (9/9) + L* timers (3/3), Tc2_Utilities BCD pair + FB_EnumStringNumbers |

### C1 coverage notes

- **Tc2_Standard**: complete — the 16 IEC narrow-string functions,
  9 timer/counter FBs, 9 `W*` wrappers, and 3 `L*` LTIME timers are
  all covered (builtins + wrappers).
- **Tc2_Math**: all 7 math functions implemented in pure ST.
  `TRUNC` narrows through `DINT`, so magnitudes `>= 2^31` lose
  precision (documented in the source header). `LMOD` uses
  truncation semantics (fmod-style); `MODABS` uses floor-modulo so
  the result is always in `[0, |m|)` — both flagged for verification
  against Beckhoff behaviour on edge cases.
- **Tc2_Utilities**: 325 APIs, mostly T2 (file, registry, license,
  AMS router, timezone, `NT_*`). Implemented: BCD pair,
  `FB_EnumStringNumbers` (enum ordinals passed as INT; number-format
  consume sets approximated per `E_NumGroupTypes`). Blocked by
  missing language features, not effort: `FB_CalcHashValue` and
  `FB_MemBuffer*`/`FB_MemRingBuffer*` need `PVOID`/pointers;
  `FB_FormatString` needs the `T_Arg` union type.
- **Tc2_IoFunctions**: re-tiered T0 -> T3 in the plan — the catalog
  is dominated by EtherCAT device FBs (KL terminals, ASI, DPV1,
  IO-Link, SCIT, TcTouch, AX2000), not process-image helpers.

## Known limitations

- Source inclusion, not binary linking: library globals must come
  from the consumer via `VAR_EXTERNAL`; libraries cannot own
  `VAR_GLOBAL` (rejected by `tplcc build --library`).
- `F_GetVersionTcMath` returns a placeholder `16#0100`; Beckhoff's
  exact version-element encoding is not yet replicated.
- `BCD_TO_DEC`/`DEC_TO_BCD` are single-scan FBs: `BUSY` pulses for
  one scan after `START` and `ERR` latches until the next `START`,
  matching the documented handshake shape but not any asynchronous
  timing Beckhoff guarantees.
- **Known toolchain issue**: on the LLVM path, FUNCTION_BLOCK
  outputs written only inside conditional branches can lose
  retention between scans (observed: `ERR`/`ERRID` partially
  cleared several scans after a conditional write; the MIR
  interpreter is unaffected). Compat FBs work around it by
  computing into instance vars and assigning every output
  unconditionally each call — see `tc2_util_bcd.st`. Root cause
  not yet isolated; tracked as a compiler bug.
