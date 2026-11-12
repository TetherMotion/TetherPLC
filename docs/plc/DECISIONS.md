# Implementation decisions & deviations from `docs/plc/plan/`

Running log: every place the implementation deviates from a plan doc, or
resolves an ambiguity the plan left open. Append-only; each entry names the
affected plan doc. Plan docs are corrected in place only when the deviation
is a spec bug (already done for the ones below marked "spec fixed").

## D-IMPL-1 — `TplcImageHeader` atomics (process-image.md)

**Plan:** all header fields `_Atomic uN`.
**Implemented:** plain integer fields; all access through
`tplc_img_load/store_u32/u64` which apply `__atomic_*` builtins with
explicit memory orders. Identical semantics (seqlock protocol unchanged),
portable across C and C++, no `std::atomic`/`stdatomic.h` layout concerns.
Plan doc Appendix B updated to match.

## D-IMPL-2 — `TplcTaskDesc` size: 64 B, not 56 B (runtime-abi.md, spec bug — fixed)

The field list (name[32] + 4×u8 → 36, u64-align → 40, 2×u64 → 56, 2×u32 →
64) totals 64 B; the doc's `/* 56 B */` comment and `_Static_assert(...==56)`
were wrong. Assert now reads 64 in both the header and the plan doc.

## D-IMPL-3 — `TPLC_IMAGE_MAGIC` (runtime-abi.md App B vs process-image.md §3, spec bug — fixed)

Appendix B defined `'TIMG'` u32-with-version-in-high-bits; process-image.md §3's file
layout has an 8-byte magic `TPLCIMAG` at offset 0. Implemented
`0x47414D49434C5054` ('TPLCIMAG' LE) as a plain u64 magic with a separate
`version` u32 field, matching §3's byte offsets. Appendix B updated.

## D-IMPL-4 — Missing error codes added (runtime-abi.md §12 → App A)

`TPLC_E_ABI_MISMATCH`, `TPLC_E_ABI_EXT_UNMET`, `TPLC_PROG_FAULT` are owned by
runtime-abi.md §12 but were absent from the normative listing. Defined as `-64`,
`-65`, and event code `16` respectively; header and plan doc updated.

## D-IMPL-5 — Image-layer error range (process-image.md §12)

`TPLC_E_IMG_*` defined at `-100..-107` (MAGIC/VERSION/TOO_SMALL/
SEQ_EXHAUSTED/EPOCH/RANGE/IO/STATE) — inside the image header, disjoint
from ABI codes. Event codes `TPLC_IMG_PLC_LOST`/`MASTER_LOST` = 1/2
(non-negative; never return values).

## D-IMPL-6 — `task_local` pointer source deferred (runtime-abi.md)

`TplcTaskCtx.task_local` is set to `nullptr` in P0. The ABI descriptor
carries only `task_local_size`; region offsets live in `layout_desc` which
the pure-ABI host doesn't parse yet. Resolution arrives with the package
reader (P1/P2); modules needing task-local storage declare
`task_local_size=0` meanwhile (conformance stubs do).

## D-IMPL-7 — `%Q` commit granularity (process-image.md §5.3)

`Program::executeTask` commits via a caller-supplied hook that receives the
program's `image_out` buffer + declared `image_out_bytes`. Ownership-range
enforcement (per-task disjoint %Q ranges, `TPLC_E_MAP_OWNER`) is a doc-09
map concern — not yet wired; single-program commits are correct today.

## D-IMPL-8 — C dialect: C11, not C23 (runtime-abi.md §2)

Headers are C11 (`_Static_assert`, `__atomic` builtins) with a
`#define _Static_assert static_assert` bridge for C++ — the normative form
from the plan doc is preserved verbatim in source.

## D-IMPL-9 — Host-side error range

Host-only load/lifecycle errors use `-300..` (`TPLC_E_HOST_*` in
`host/Status.hpp`), disjoint from ABI codes (`-1..-65`), image codes
(`-100..`), and module-defined (`-999/-1999`).

## D-IMPL-10 — PROG_STOP/PROG_START gate scheduling only (plc-runtime.md)

P0 control ops: `prog.stop` halts task scheduling, `prog.start` resumes
it — module state and blob are untouched (IEC stop≠reset). A re-initing
`reset` op (cold/warm) is P2. The ctl op surface for `prepare`/`kill`/
`reset`/`unload`/`deploy.*`/`retain.*`/`dbg.*`/`trace.*` is declared in
`tplc_ctl.h` but unimplemented until their phases land.

## D-IMPL-11 — ctl wire details (plc-runtime.md App I)

- Error frames: same `msg_type` as the request, flag bit15
  (`TPLC_CTL_F_ERROR`), payload `{code u32 (positive magnitude),
  subcode u32, str msg}`.
- `hello`: built into the server (pre-dispatch gate); `proto_ver`
  negotiated as `min(client, server)`; `authz` currently always 1
  (full control) — real authz arrives with P8-03.
- Strings on the wire: `u16 len + bytes`, no NUL.

## D-IMPL-12 — ctl is optional at runtime

`tether-plc` keeps running headless if the ctl socket can't be opened
(logs a warning) — matches the spec's "control channel is a service
plane concern; the scan path must not depend on it." Passing
`--ctl none` skips the socket deliberately (used by the fault harness);
an empty `--ctl` value falls back to `<image>.ctl` as before.

## Open items carried forward (not deviations — just not done yet)

- `bad_*` module series (runtime-abi.md §11 fixtures 2): §3.3 prohibition
  enforcement needs the post-link symbol scanner (plan doc 15/P8-04);
  loader-level checks beyond symbol presence aren't implementable at
  dlopen granularity alone.
- Fault-injection harness (P0-09), control channel (P0-06), tether-plc
  binary (P0-07): next in WBS order.
- `_Static_assert` in `TplcImageHeader`: `reserved[112]` + `in_off` field
  alignment verified by the 256 B size assert.

## D-IMPL-13 — `content_hash` covers the TOC (artifact-format.md §2 amended)

Original spec hashed only section payloads; the TOC's `section_type`,
`flags`, `aux`, `section_version` bytes were then integrity-uncovered —
a tamper could flip an optional section to required or redirect
`aux`→`targets[]` undetected. Amended: `content_hash` = SHA-256 over
the raw TOC bytes followed by all payloads in canonical order.
Implemented in both `tplc-pkg` (Rust) and `tplc::Package` (C++) and
locked by `toc_tamper_detected_by_content_hash` tests on both sides.

## D-IMPL-14 — package_uuid is a 64-byte ASCII field

Doc 01 §2 reserves 64 B for `package_uuid`; implementations store the
UUID's ASCII text (36-char hyphenated UUIDv7) NUL-padded — keeps the
field self-describing in hex dumps and forward-compatible with longer
encodings.

## D-IMPL-15 — PkgError block -200..-215, `TPLC_E_PKG_SECTION_FORMAT` added

`tplc_pkg.h` defines the container error block as C macros; -213 is
named `TPLC_E_PKG_ABI_MISMATCH` (abi.h already owns
`TPLC_E_ABI_MISMATCH = -64`). -215 `SECTION_FORMAT` fills an unassigned
slot for malformed section-internal payloads (bad LAYD magic etc.).

## D-IMPL-16 — Frontend spike S1–S4 verdicts: "half-B" confirmed feasible

Spike `compiler/spikes/rusty-parse` against ruSTy `plc-compiler` at
pinned rev `11e8107e` (2026-03 HEAD; LGPL-3.0 — linking evaluation for
the compiler-binary artifact is a separate P8 legal item):

- **S1 PASS** — `parser::parse` consumed as a library; 117 AST nodes in
  the dialect corpus, 100% traversable with `SourceLocation`s. Only
  corpus-side syntax fixes needed (IEC `AT` precedes the type;
  `VAR RETAIN` is a block qualifier).
- **S2 PASS** — every node kind in the dialect corpus classifies
  direct-lowerable or needs-our-sema (0 dialect-excluded at AST level).
  Their AST is a usable lowering source for `tplc-mir`.
- **S3 PASS-with-note** — `TypeAnnotator` writes a side-table
  `AnnotationMap` keyed by node; the AST itself is untouched, so no
  storage-model entanglement at the annotation layer. However their
  `pre_process` DOES desugar: `AT %QX0.0` bindings become generated
  `__PI_*` globals — our lowering must either consume that convention
  or bypass pre_process's hardware-binding rewrite (open detail).
- **S4 CONFIRMED** — Option A eliminated as predicted: POU members
  become `module.add_global` LLVM globals (module-owned .data/.bss);
  FB calls pass `this` pointers to instance structs. Incompatible with
  host-owned relocatable state blob — structural, not patchable.

Recommendation from DECISIONS.md stands: adopt ruSTy **parser only**
(half-B), everything downstream of `tplc-mir` ours. S5 (end-to-end
POU through the B path) and S6 (C-floor RD parser) remain open but are
now refinement, not gate: S1–S3 already show the AST is consumable.

Build note: `spikes/*` are excluded from the compiler workspace — they
pull LLVM-21. Local builds need an LLVM-21 prefix (this machine uses
`~/llvm21` extracted from Ubuntu `llvm-21-dev`/`llvm-21`/
`llvm-21-tools`/`libpolly-21-dev` debs via `dpkg -x`, no sudo):

    PATH=$HOME/llvm21/usr/lib/llvm-21/bin:$PATH \
      LLVM_SYS_211_PREFIX=$HOME/llvm21/usr/lib/llvm-21 \
      cargo build --manifest-path compiler/spikes/rusty-parse/Cargo.toml

## DEC-2026-01 — ST frontend strategy: half-B (ruSTy parser only), CONFIRMED

  Chosen:    **half-B** — adopt ruSTy's parser/AST (crates
             `plc_compiler`, `plc_ast`, `plc_source`, `plc_parser`,
             `plc_lexer`, `plc_index`) at pinned rev `11e8107e` behind a
             `tplc-st` adapter boundary; everything from `tplc-mir`
             downward is ours (DECISIONS.md §8 list unchanged).
  Evidence:  S1–S6 all executed:
    - S1 PASS — library parse, 117/117 nodes located, 0 diags.
    - S2 PASS — 100% of corpus lowerable (0 dialect-excluded at AST).
    - S3 PASS-with-note — annotations side-table (no AST entanglement);
      `pre_process` desugars AT→`__PI_*` globals ⇒ **we lower the RAW
      AST and never call pre_process** (decided in S5: AT is parsed as
      `PointerType{auto_deref: Alias}` we unwrap to shadow+io_binding).
    - S4 CONFIRMED — module-owned LLVM globals ⇒ Option A eliminated.
    - S5 PASS — `compiler/spikes/rusty-mir`: full corpus POU lowered to
      `tplc-mir` (~350 LOC), laid out (148 B blob, retain regioned),
      executed in `tplc-mir`'s interpreter: TON timing, FB calls with
      named args, CASE ranges, FOR/WHILE/REPEAT, typed literals,
      function calls, AT-bound vars, retain init — 5/5 checks, 0 gaps.
    - S6 DONE — `compiler/spikes/rd-floor`: 451 LOC hand-RD parser
      covers ~15% of dialect ⇒ full ST own-parser ≈3-5k LOC + sema
      (weeks) vs ~350 LOC observed for the B-path lowering.
  Rejected:  A (adopt ruSTy backend) — module-owned state, structural.
             C (own parser) — 10× observed B cost, no semantic benefit;
             kept as fallback if ruSTy API churn or license review fails.
  Consequences:
    - `compiler/tplc-mir` crate exists (IR + layout + interp) and is
      the documented producer contract for ST and FBD frontends.
    - Doc 05: our sema/typeck/diagnostics layer is ACTIVE; grammar is
      the normative source (ruSTy parser is an implementation detail).
    - Doc 07: all sections active — `tplc-mir` is real.
    - **License caveat (P8 item):** ruSTy is **LGPL-3.0**. Statically
      linked into `tplccd` it is compliant while TetherPLC source is
      public (users can relink by rebuilding); if a closed binary
      distribution ever appears, split the parse step into a separate
      process or replace the parser. Attribution file required.
      Clean-room FB lib unaffected (ruSTy stdlib not consumed).

## D-IMPL-14 — LLVM binding: inkwell 0.9 (feature-gated), LLVM-21

P1-05 chose **inkwell 0.9 + `llvm21-1`** over textual-IR/`llc` subprocess: (a) ruSTy's dep tree already proves it builds against our LLVM-21 prefix; (b) JIT via `create_execution_engine` gives in-process verification (the e2e test executes `tplc_init`/`tplc_task_run` against a fake ABI ctx — no subprocess, no .so round-trip); (c) bitcode emission for the package comes free. Gated behind `tplc-codegen`'s `llvm` cargo feature — default workspace stays LLVM-free for CI. Dev-env: `LLVM_SYS_211_PREFIX` + `PATH` pointing at an extracted `llvm-21` prefix.

Litmus in `tests/llvm_e2e.rs`: WHILE-loop execution, extern `__tplc_rt_TON` host-mapping (the tplc-rt link seam), TON.Q at 500 ms via `ctx->now_ns`, `abort_flag` → rc −1, and an `inttoptr`-free guarantee (all program state addressed `blob+const_off`).

## D-IMPL-17 — builtin-FB semantics vs matiec oracle: two recorded deltas

The oracle-matiec differential fuzz (`tools/oracle-matiec/diff_fuzz.py`)
found two systematic deltas between our builtin FB semantics and
matiec's (which follows the IEC 61131-3 reference bodies literally):

1. **F_TRIG initial state.** matiec's body is `Q := NOT CLK AND NOT M;
   M := NOT CLK` with `M` init `FALSE` — so the first scan with
   `CLK=FALSE` reports a falling edge (assumed-prior `CLK=TRUE`). Ours
   stores `__clk_last := CLK` (init `FALSE`) and reports no edge.
   Ours is kept: a phantom edge pulse on a quiet input at startup is the
   worse default for machine control. R_TRIG inits agree in both.
2. **Counter CV clamp.** matiec clamps `CV` to `[0, PV]` on CTU/CTD/CTUD
   (increment guarded by `CV<PV`, decrement by `CV>0`, `ELSE`-chained);
   ours applies both edges with `saturating_add/sub` against INT range
   only, so `CV` can exceed `PV` or go below 0. `QU`/`QD` read-outs are
   identical within `[0,PV]`; `CV` itself and `QD`-at-boundary traces
   can differ. Recorded as a known delta; aligning our clamp to the IEC
   reference body is an open option, not done here.

Status: documented known divergences (`known_divergences.txt`), not
defects of record. Everything else in the fuzz subset — bool/int
expressions, IF/ELSIF/ELSE, SR/RS dominance, R_TRIG/F_TRIG steady
state, TON/TOF/TP under injected `__CURRENT_TIME`, located-I/O byte
semantics — agreed on all generated cases (seeds 0–1, 42 cases,
~30 cycles each).

## D-IMPL-18 — Live attach: standalone `tplc-io` TCP provider (doc 16 §7)

**Plan:** iteration-2 bridges the management/debug surface into
`tether_io` (adapter unspecified: in-process vs sidecar vs bridge).
**Implemented:** a separate `tplc-io` process inside TetherPLC that
compiles Tether's `tether_io` server sources (`Registry`, `Session`,
`Server`, `TcpTransportServer`, schema catalog) into a local static
lib and connects to a running `tether-plc` over `ctl.sock` — TCP
listener on `127.0.0.1` by default, SLIP framing, V6 schema slots on
every entry.

Why separate process: the runtime keeps a clean single-threaded ctl
loop and its seccomp profile stays AF_UNIX-only — no network code or
sockets ever execute inside the sandboxed scan process. AuthZ stays
kernel-attested (the provider's own uid is what `SO_PEERCRED` sees),
and a provider crash cannot stall the scan loop.

Consequences recorded in `docs/plc/tether-io-attach.md`: remote
client identity can't reach `SO_PEERCRED` (provider role applies),
and complex replies use JSON strings rather than per-shape schema
structs (keeps the schema catalog small and version-tolerant; typed
structs remain an option).

## D-IMPL-19 — Shadow run: structural output isolation + live-state promote (doc 19)

**Plan:** shadow consumes identical `%I`, writes a "shadow output
region", promotes instantly.
**Implemented:** the shadow region needs no new image machinery —
each `Program` already owns a private `imageOut()` buffer, so
isolation is a no-op commit hook rather than a second image.
Identical inputs come from the shadow latch hook memcpy'ing the
active's `imageIn()` post-latch. Promotion moves the live
`Program`/`Module` into the active slot through the same quarantine
commit as swap, so the candidate's accumulated state carries over
with zero re-initialization and rollback is the same code path.

Consequence recorded in `docs/plc/shadow-run.md`: scheduling is
budget-bounded slack (`--shadow-budget-ns`, default 200 µs) checked
after the active `%Q` commit — a shadow can never delay the scan,
it can only be skipped (`missed`).

## D-IMPL-20 — `TPLC_E_DBG_STALE` (-411) distinguishes vanished vs unknown symbols (retired doc 12)

**Plan:** staleness is marker state (len-0 reads, `n_stale` counts);
`TPLC_E_DBG_STALE` was declared without an emit site.
**Implemented:** `DBG_RESOLVE`/`DBG_WATCH_ADD` now emit -411 when the
requested name misses the live symmap but resolves in the last
committed swap's predecessor map (`sdiff.old_smap`); a never-seen
name keeps -410. The marker behavior for offset-only subscriptions
is unchanged. Covered by `online_change_e2e`.

## D-IMPL-21 — `TPLC_E_MAP_*` moved to -600..-609

**Plan/legacy:** bridge map-validation codes aliased the -500 block
shared with wire-side `TPLC_E_MGMT_*` (different domains, same
numbers — `map: MAP_RANGE (-502)` printed an MGMT code's value).
**Implemented:** MAP codes renumbered to -600..-609 in
`runtime/bridge/include/tplc/bridge/MapRunner.hpp`; they remain
report-domain codes, never wire errors. `error-codes.md` updated.

## D-IMPL-22 — STRING semantics fixed end-to-end (codegen + interp + sema)

**Found:** in the compiled path a STRING literal lowered to a bare
`i8 0` (opaque-pointer store only cleared the len byte), so
`s := 'abc'` silently produced an empty string; `a = b` on strings
panicked the compiler (`ArrayValue` fed to `into_int_value`); the L1
interpreter compared strings as 0 (always equal).

**Implemented:** literals materialize as `[1+cap x i8]`
`{len, bytes, 0…}`; the STRING store pads/truncates to the target cap
(`str_fit`); `=`/`<>` lower to an unrolled length+code-unit compare
(libc-free — the scan path must not call `memcmp`); the interpreter
compares bytes lexicographically; sema rejects ordered or mixed-type
STRING comparisons with `TPLC_E_ST_TYPE` instead of letting codegen
fail. Covered by `str.compare` (oracle vector: L1 + sim parity) and
`string_assign_and_compare_through_jit` (compiled path).

**Still open:** `WSTRING` remains a narrow `STRING(160)`; true UTF-16
(`u16` len + `u16[cap]`) is the iteration-2 dialect item.

## D-IMPL-23 — `WSTRING` is a real UTF-16 type (iteration-2 dialect item)

**Before:** `WSTRING` lowered to a narrow `Str { cap: 160 }`, so wide
literals and code units were silently reinterpreted as bytes.
**Implemented:** `Type::WStr { cap }` — blob = `u16 len` + `u16[cap]`
UTF-16 code units (surrogate pairs preserved), TYPC tag 16; wide
literals (`AstLiteral::String.is_wide`) become `Lit::WStr`; sema types
them `WStr { cap: code-unit count }`, keeps `=`/`<>`-only compares,
and rejects narrow↔wide assignment (`TPLC_E_ST_TYPE`). Codegen
materializes i16 arrays and reuses the shared array-fit/compare paths;
the L1 interpreter stores `Value::WStr(Vec<u16>)` and compares
lexicographically.

Coverage: `wstr.compare` (L1 + sim parity) and
`wide_string_is_utf16_through_jit` (non-ASCII `aä€` encoding, cross-cap
store, equality).

**Deliberately not done:** string functions (`LEN`/`CONCAT`/…) and
`STRING`↔`WSTRING` explicit casts — no builtin string op exists yet in
either engine; the type system is ready for them.

## D-IMPL-24 — string builtins `LEN`/`CONCAT` (IEC 61131-3 §2.5.1.5)

**Before:** sema resolved calls against FUNCTION POUs only, so the
standard string functions were unavailable (`no FUNCTION LEN`).
**Implemented:** `tplc_mir::ty::builtin_fn` is the single dispatch
table; `LEN(IN)` returns `INT` (code units: bytes for STRING, UTF-16
units for WSTRING); `CONCAT(IN1, IN2)` returns the same string kind
with `cap = cap1 + cap2` (saturating), so a concatenation never
truncates unless the target is narrower. Codegen lowers LEN to a len
element extract and CONCAT to a fixed-size copy of the first array
plus a runtime-bound unit loop for the shifted tail; the truncating
store clamps the len element so a narrowed string never claims data
that no longer fits. Mixed narrow/wide or non-string args are
`TPLC_E_ST_TYPE`.

Coverage: `str.builtins` (L1 + sim parity), `string_builtins_through_
jit` (narrow+wide, truncation clamp) and `string_builtins_are_typed`.

**Still open:** the remaining string functions (`LEFT`/`RIGHT`/`MID`/
`INSERT`/`DELETE`/`REPLACE`/`FIND`) and scalar builtins
(`ABS`/`MIN`/`SEL`) — the dispatch table makes each a small addition.

## D-IMPL-25 — scalar builtins `ABS`/`MIN`/`MAX`/`SEL`/`LIMIT`

**Implemented:** added to the same `builtin_fn` dispatch as the string
builtins. Typing follows IEC: `ABS` preserves its argument type
(int/real/time), `MIN`/`MAX`/`LIMIT` unify numerically and coerce
their operands to the result type (so `MIN(3, 7.5)` is `LREAL`),
`SEL(G, IN0, IN1)` requires a `BOOL` selector and branches of one
type. Codegen uses compare+select (signed/unsigned/float predicates
chosen from the result type), so no libc math calls reach the scan
path; `ABS` on the most-negative value wraps (IEC leaves it
undefined).

Coverage: `scalar.builtins` (L1 + sim parity),
`scalar_builtins_through_jit`, `scalar_builtins_are_typed`.

## D-IMPL-26 — string functions LEFT/RIGHT/MID/INSERT/DELETE/REPLACE/FIND

**Implemented** on the shared `builtin_fn` dispatch for narrow and
wide strings. Semantics follow IEC 61131-3 §2.5.1.5 with the clamping
the implementation can guarantee: positions are 1-based, `L <= 0`
yields the empty string, `P` past the end clamps to the end, and
`FIND` returns 0 for an empty needle or no match. Result caps are
`cap(IN1)` for the slicing forms and `cap1 + cap2` for INSERT/REPLACE
(so an insert cannot silently drop text); a narrower target truncates
and clamps the len element, as with every string store.

Codegen lowers each op to a composition of one runtime-bound unit
copy loop over allocas (`copy_units`), so the scan path stays libc
free; FIND is a two-level loop with early exit on the first match.

Coverage: `str.functions` (L1 + sim parity),
`string_functions_through_jit` (narrow + wide + FIND miss),
`string_functions_are_typed`.

## D-IMPL-27 — conversion functions, TRUNC/ROUND, MUX

**Implemented:** `X_TO_Y` names are parsed by `tplc_mir::ty::
conversion_fn` (scalar table shared with the frontend) and rewritten
in sema into a `Conv` node — the call disappears, so interp and codegen
need no new path. The argument is coerced to `X` first, which is what
makes `INT_TO_REAL(dint_var)` lint as narrowing instead of silently
widening. `TRUNC` returns `DINT` via `fptosi` (truncation toward zero);
`ROUND` adds ±0.5 before the same conversion (half away from zero).
`MUX(K, IN0…)` is 0-based, unifies its inputs numerically, and selects
`IN0` when `K` is out of range.

Two latent gaps surfaced and are fixed: `convertible` lacked
`REAL↔LREAL` (so `r : LREAL := DINT_TO_REAL(x)` was a type error) and
codegen's `conv` had no float-widening arm (the value stayed `f32`
inside an 8-byte slot). The sema rewrite also has to set the node's
`ty`, not just its kind, or codegen sees `from = Bool`.

Coverage: `conv.functions` (L1 + sim parity),
`conversions_and_mux_through_jit`, `conversions_round_and_mux_are_
typed`.

## D-IMPL-28 — CiA-405 object dictionary over ctl

**Implemented:** `OD_READ`/`OD_LIST`/`OD_WRITE` (op range `0x0A00`)
expose a 13+-entry index/subindex view of the management state model
(`0x2000` program, `0x2100` tasks incl. per-task rows, `0x2200` image,
`0x2300` lineage, `0x2400` retain). Read returns
`{type, access, len, bytes}`; list returns every row with its name;
write carries `{index, sub, len, data}`.

**Authz shape:** the op floor for `OD_WRITE` is ENGINEER and each
object declares its own minimum role, so an engineer writing the
commissioner-level `0x2400/1` (retain flush) gets `MGMT_AUTHZ` — the
three reserved `MGMT_*` codes (-500 unknown object, -502 object authz,
-503 schema/read-only) are now real. `CtlServer::roleOf(fd)` was added
so handlers can enforce entry-level floors below the op gate.

**Deliberately not done:** the CANopen wire binding (SDO/PDO framing)
— this is the dictionary the adapter binds to; entry values are
computed per request, so nothing is cached.

## D-IMPL-29 — `REFERENCE TO` as a blob offset; refs never survive migration

**Design:** a reference is an 8-byte `u64` blob offset
(`u64::MAX` = null) — pointer-free, so it survives blob *relocation*
(`moveBlob`) and the MCU static-placement story, but not a layout
change. `REF(x)`/`&x` lowers to `ExprKind::RefTo` (constant offset for
vars/fields, runtime for array indices and rebinding through another
ref); `r^` and auto-deref read/write through `Ref::Deref`. Codegen
checks null (`-17` `TPLC_TASK_E_PANIC`) and range (`-18`
`TPLC_TASK_E_BOUNDS`) before the load; the interpreter latches the same
fault so L1 and L2 agree. `ISVALIDREF` reports null.

**Migration — preserved vs reset:** a ref stores a *blob offset*, so
it stays valid exactly when the referenced layout did not move. The
planner discriminates: if every surviving field keeps its offset, size
and type signature and nothing is dropped (`layout_stable`), the ref
field is **copied** like any other value (MIGD `aux` bit2 marks it as a
kept reference); if anything moved or vanished, the ref is
**reset** to null (Dropped rule, `aux` bit1) and the change is marked
degraded. `tplcc build --change-against` says which: per-field
`note: reference field r preserved (referenced layout unchanged)` or
`warning: reference field r will be reset to null — the referenced
layout moved`, plus a reset summary. The runtime swap logs
`online change: N reference field(s) preserved (layout unchanged),
M reset (layout moved)`, so an operator can tell whether re-arming is
needed.

**Also fixed:** the L1 interpreter never applied declaration
initializers (`x : DINT := 42`), so any initialized variable diverged
from the compiled path; `Rt::run_inits` now runs them once at cycle 0
in the vector runner, matching `tplc_init`.

Coverage: `ref.deref` (L1 + sim parity), `reference_semantics_through_
jit`, and `ref_migration` (warning, swap log, program stays running).

## D-IMPL-30 — update authentication is a separate trust domain from boot

**Design:** package signatures authenticate *updates*, not boot.
`--sig-policy`/`--trust-dir` govern the boot package; `--update-auth`
governs every update op (`PROG_PREPARE`, `PROG_SWAP`,
`SHADOW_LOAD`). `open` (default) keeps development boot permissive;
`strict` floors every update channel at `require-trusted` — signed by
a non-revoked key present in the trust store — regardless of
`--sig-policy`, and no flag or config can weaken it (a weaker
per-channel entry is a startup error, not a silent bypass).

**Per-channel policy:** `--update-auth-map key=policy[@trust-dir]`
overrides the update policy per ctl channel. `key` is a role name
(`observer|engineer|commissioner`) or a decimal peer uid; resolution
is uid-entry > role-entry > global. `@dir` selects a different trust
root (e.g. a safety-backed signing authority). Under `strict` every
entry must itself be `require-trusted`.

**Unsigned-module rejection:** a raw module path (`.so`, not
`.tplc`) carries no signature section, so update ops reject it with
`SIG_REQUIRED` unless the resolved policy is `open` — closes the
bypass where an unsigned `.so` skipped package verification
entirely.

**Trust lifecycle:** rotation is additive (several `<keyid>.pub`
files; any trusted key signs). Revocation is `<trust-dir>/revoked`,
a whitespace-separated list of hex key-ids checked before the store
lookup, so a revoked signer is rejected even while its `.pub`
remains.

**Boot vs update, stated:** first-boot package load uses
`--sig-policy` only — trust is an update-time control. A device that
must only ever run vendor-signed images should boot with
`--sig-policy require-trusted`; a device that accepts field updates
uses `--update-auth strict`. The two compose; neither weakens the
other.

Coverage: `update_auth` (ctest) — strict rejection of unsigned and
untrusted packages on swap and shadow-load, trusted acceptance,
rotation, revocation, per-channel trust roots, startup rejection of
weaker maps, and the raw-`.so` rejection.

## D-IMPL-31 — ABI struct offsets computed from the target data layout

**Decision:** generated code no longer hardcodes `TplcCtx`/`TplcTaskCtx`/
`TplcServices`/`TplcMigrateCtx` member offsets. `AbiOffsets` derives them
from the target's LLVM data layout (pointer size + i64 alignment) at
emission time, so the same MIR produces correct GEPs on LP64 hosts and
AAPCS32 Cortex-M targets.

**Why:** the first `--target armv7em-none-eabihf` object ran under
qemu-arm but faulted — the hardcoded offsets assumed 64-bit pointers,
so `ctx.svc`, `ctx.abort_flag`, and the deadline reads landed on the
wrong members. C `offsetof` on both ABIs confirmed the computed values.

**Consequence:** `tplcc` resolves the triple *before* emission
(`emit_program_for_target`), and the manifest records `target_triple`.
A target whose layout the parser cannot read falls back to LP64 —
acceptable because bare-metal ABIs in scope are LP64 and AAPCS32.
Coverage: `abi_offsets_lp64`/`abi_offsets_aapcs32` unit tests pin the
values against `offsetof()` output; `mcu_exec` exercises the AAPCS32
path end to end.

## D-IMPL-32: integer div/mod by zero, saturating default with checked mode opt-in

**Decision:** `tplcc build --checked-arith` sets `Module::checked_arith`,
which both backends honor. The MIR interpreter latches
`TPLC_TASK_E_DIVZERO` (-19) on a zero divisor; LLVM emits a real
branch to a `-19` return block in the POU function. Without the flag,
integer `Div`/`Mod` keep the deterministic saturating-to-0 result.

**Why:** IEC scan semantics want a faultable task for div-by-zero, but
existing packages and the oracle corpus were built on saturating;
a flag keeps both behaviors available per artifact instead of flipping
the default. The manifest records `"checked_arith":true` as
provenance; the mode is baked into bitcode and `.so`, not switched at
load time.

**Consequence:** saturating mode cannot use `select(dz, 0, sdiv)`:
instcombine folds a branch-guarded div back into that shape, and x86
`idiv` still traps on the zero arm. The emitted form clamps the
divisor to a provably nonzero value (`select(dz, 1, r)`), divides,
then selects 0, defined on every input, no trap possible. Checked
mode keeps real control flow (the zero arm exits the function, so no
merge exists for instcombine to fold). `INT_MIN / -1` overflow remains
UB on the hardware path in saturating mode; checked mode guards it
(see D-IMPL-34).
Coverage: `saturating_arith_divmod_jit`, `checked_arith_divzero_jit`,
and the `div.zero.sat`/`div.zero.checked` vectors through L1+L2.

## D-IMPL-34: checked arithmetic emits OVERFLOW (-20) and RANGE (-21)

**Decision:** `--checked-arith` extends past div-by-zero. Int
`Add`/`Sub`/`Mul`/`Neg` results that leave the type's range latch
`TPLC_TASK_E_OVERFLOW` (-20); signed `INT_MIN / -1` joins -20;
conversions whose result cannot fit the target type (narrowing
int, real→int, incl. NaN) latch `TPLC_TASK_E_RANGE` (-21).

**Why:** the three codes were reserved together in the task-error
block for exactly this mode; emitting only -19 left -20/-21 as dead
reservations while real wraparound faults went unreported.

**Consequence:** LLVM checked `bin`/`Un` use `llvm.*.with.overflow`
intrinsics and checked `conv` emits an i128 range compare before the
truncate/`fptosi`; saturating mode keeps wrapping `nsw`-free ops and
truncating converts unchanged. The div path now selects a provably
nonzero divisor *and* guards the `INT_MIN/-1` case in both modes,
so saturating `INT_MIN / -1` yields `INT_MIN` instead of UB.
Real→int checked mode compares against the int range as f64 before
`fptosi`, which also removes the NaN/inf poison case. Coverage:
`checked_arith_overflow_range_jit`/`saturating_arith_overflow_range_jit`
and the `ovf.*`/`rng.*` vectors through L1+L2.

## D-IMPL-33 — multi-shadow: capped candidate slots with uuid/name selectors

**Plan:** doc 19 listed multiple simultaneous shadows as an
iteration-2 candidate next to per-channel promote.
**Implemented:** the single `Shadow` slot became
`std::vector<std::unique_ptr<Shadow>>` capped at `MAX_SHADOWS = 4`.
Every candidate keeps its own `Scheduler`, counters, diff ring,
fault flag, and last-tick timestamp; the scan loop iterates
candidates inside the same post-commit slack gate, so candidates
share the budget check but tick, miss, and fault independently. A
fault drops only the faulting slot; lifecycle ops clear the vector.

Selection is an optional `{u16 slen, sel}` tail on
STATUS/COMPARE/IMAGE/PROMOTE/DROP/RESET where `sel` is the 16-byte
program uuid or the program name. Selector-less ops resolve to the
only loaded candidate and fail `MGMT_STATE` when several are
loaded. `SHADOW_LIST` (`0x0808`) enumerates candidates in load order
— the index space for `SHADOW[<i>].<sym>` resolve/watch paths and
the `DBG_READ`/`DBG_FIELDS` selector byte (`0` active, `N` =
`shadows[N-1]`). Promotion of candidate N moves it into the active
slot through the existing quarantine commit and clears every peer:
their blobs were seeded from the pre-promote program and their
migrate descriptors no longer match. `TplcCtlProgStatus`'s shadow
tail aggregates (state OR, counters summed, newest diff cycle)
because per-candidate detail lives on `SHADOW_LIST`.

**Consequence:** name selectors resolve first-match, so
same-named candidates (the normal versioning case) must be
addressed by uuid — `SHADOW_LOAD`'s reply carries it. The sim and
L1 vectors address candidates by load-order index and resolve to
uuids through `SHADOW_LIST`, keeping the wire selector as the only
stable identity across drops.

## D-IMPL-35 — TPLB loadable blob: emit-time PC-relative fixups, runtime absolute fixups

**Plan:** doc 18's MCU path listed a PIC/loadable-blob variant as
the gap between "relocatable ET_REL for link-time placement" and a
loader that needs no target-side linker.
**Implemented:** `tplc-codegen/src/elf_blob.rs` parses the ARM
ET_REL with a minimal bounds-checked ELF reader (ELF32/ELF64,
REL/RELA) and emits a TPLB v1 image: allocatable PROGBITS/NOBITS
sections at aligned blob offsets (`.ARM.exidx`/`.ARM.attributes`
dropped — blob targets never unwind), exported globals in a symbol
table with `st_value` preserved (Thumb bit included), undefined
symbols as named imports, and a fixup table. The split follows the
relocation semantics verified empirically against `ld`: PC-relative
fields (`REL32`, `THM_CALL` on defined symbols) are patched at emit
time because the blob base cancels (`field = S + A - P` in blob
offsets); absolute fields (`ABS32`, `GLOB_DAT`, `JUMP_SLOT`,
`RELATIVE`, `MOVW/MOVT_ABS_NC`) become runtime fixups
`{place_off, type, value, addend}` where `value` is a blob offset or
`F_IMPORT|index`. `tplcc build --target <foreign> --pic-blob`
stores the blob in `target_obj` and records
`"target_obj_format":"tplb"` in the manifest.

**Consequence:** a ~150-line C loader (`tests/abi/tplb_load.c`)
places the image and applies fixups — no linker, no ELF on target.
Two encoding traps are pinned by tests: Thumb-2 `MOVW/MOVT`
split their imm16 as `imm4:i:imm3:imm8` (not the ARM-state
`imm4:imm12`), and `THM_CALL` fixups to imports must land within
the ±16 MiB BL range of the blob, which the loader range-checks.
NOBITS sections are materialized as zero bytes in the blob for v1
(simplicity over size). `mcu_blob.sh` proves the path end-to-end
under qemu-arm when available; the host-side loader test validates
fixup math on a real blob without executing ARM code.

## D-IMPL-36 — library packages: kind=library manifest + --lib source inclusion

**Plan:** plan 21 §8 phase C0 calls for `tplc` manifest support for
library packages with ST-source and precompiled delivery, as the
foundation for TwinCAT compatibility libraries.
**Implemented:** `tplcc build --library` compiles a module with no
PROGRAM, VAR_GLOBAL, or TASK into a `.tplc` whose manifest carries
`"kind":"library"` and whose sections are Manifest, IrBitcode,
TargetObj (always the relocatable object — `__tplc_pou_*` symbols
stay linkable; TPLB still available via `--pic-blob`), StSources,
Toolchain. No LAYD/SMAP/TYPC/IO_MAP: there is no program state, so
`validate_for_load` rejects library packages by construction and
they can never be deployed. Consumption is `tplcc build app.st
--lib lib.tplc`, which extracts the ST_SOURCES section into the
program's compilation unit — GVL declarations merge into the
consumer's module correctly, which is also the documented escape
hatch for libraries that need globals.
**Consequence:** precompiled consumption is deliberately deferred:
source inclusion gives sema full POU declarations today, while the
relocatable object in TARGET_OBJ preserves the option of true
symbol-level linking later (extern POU decls + link against the
library .o). A discovered pre-existing bug — implicit `GLOBAL.x`
access panics in codegen (`unresolved var GLOBAL`); VAR_EXTERNAL
is the working form — is unrelated to this change and remains open.

## D-IMPL-37 — C1 first tranche: compat/tc2 pure-IEC libraries

**Plan:** plan 21 §8 phase C1 (T0 core): Tc2_Standard, Tc2_Math,
Tc2_Utilities, Tc2_IoFunctions as compatibility libraries.
**Implemented:** `compat/tc2/` holds three library sources built
with `tplcc build --library` and consumed via `--lib`:
`tc2_math.st` (CEIL/FLOOR/FRAC/LTRUNC/LMOD/MODABS/MODTURNS/
F_GetVersionTcMath, all LREAL math in pure ST),
`tc2_util_bcd.st` (DEC_TO_BCD/BCD_TO_DEC FBs with the documented
START/BUSY/ERR handshake, single-scan), and
`tc2_standard_wide.st` (the 9 W* functions as wrappers over the
narrow-string builtins, which already accept WSTRING operands).
Tc2_Standard's narrow functions and timer/counter FBs needed no
source — they are compiler builtins. `tests/compat/tc2.sh`
(ctest `tc2_compat`) packs 21 pass/fail checks into %QD0 and
replays a 4-step tape on the real runtime expecting 0x001FFFFF.
**Tier corrections:** catalog review showed Tc2_IoFunctions is
dominated by EtherCAT device FBs (KL/ASI/DPV1/IO-Link/SCIT/
TcTouch) — re-tiered T0->T3; Tc2_Utilities is mostly host-OS
services — re-tiered T0->T2 with a documented T0 subset (BCD,
hash, format, memory buffers). Plan and per-library docs
regenerated.
**Consequence:** LMOD uses truncation (fmod-style) semantics and
MODABS floor-modulo into [0,|m|); both flagged for Beckhoff
edge-case verification. TRUNC narrows through DINT so |x| >= 2^31
loses precision. LTON/LTOF/LTP (LTIME timers) and the remaining
Tc2_Utilities T0 candidates (FB_FormatString, FB_CalcHashValue,
FB_EnumStringNumbers, FB_MemBuffer*/RingBuffer*) are the next C1
tranche.

## D-IMPL-38 — C1 tranche 2 + FB conditional-output retention bug

**Plan:** plan 21 §8 phase C1 continued: L* timers and the remaining
Tc2_Utilities T0 candidates.
**Implemented:** `tc2_standard_lt.st` wraps TON/TOF/TP as LTON/LTOF/
LTP (LTIME shares the TIME representation); `tc2_util_enum.st`
implements FB_EnumStringNumbers + F_IsDigit with the real enum
ordinals (E_EnumCmdType First=0/Next=1/Abort=2, E_NumGroupTypes
Float=0/Unsigned=1/Signed=2) passed as INT. Tc2_Standard is now
fully covered; Tc2_Math fully covered.
**Bug found:** on the LLVM path, FUNCTION_BLOCK outputs assigned
only inside conditional branches can lose retention between scans
(ERR cleared, ERRID's low byte masked, several scans after the
conditional write; unconditional rewrites every call are stable;
MIR interpreter unaffected). Compat FBs compute into instance
vars and assign outputs unconditionally — tc2_util_bcd.st shows
the pattern. Root cause not yet isolated; documented in
compat/README.md as a known toolchain issue.
**Also fixed:** MID() argument order is (str, len, pos), not
(str, pos, len) — the first enum implementation scanned with
MID(s, i, 1) and never matched a digit; caught by the sim replay.
**Blocked honestly:** FB_CalcHashValue/FB_MemBuffer*/RingBuffer
need PVOID (no pointer type); FB_FormatString needs the T_Arg
union — language gaps, not effort gaps.

## D-IMPL-39 — tplc-net service + FB_Socket builtin (plan 21 C2)

**Plan:** plan 21 §8 phase C2: first runtime service, unblocking
Tc2_TcpIp and every TCP-dependent library (Modbus TCP, SMTP, ADS,
OPC UA transport).
**Implemented:** `include/tplc/tplc_net.h` defines the
`TplcNetService` vtable (ABI v1) returned by
`TplcServices.get_service("tplc-net")`: nonblocking POSIX TCP
connect/listen/accept/send/recv/close plus UDP bind/sendto/
recvfrom/close, all bounded for the scan path. Host side:
`runtime/host/{include,src}/tplc/host/NetService.{hpp,cpp}` keeps a
handle table over O_NONBLOCK fds; `HostServices::getService` is the
single named-service lookup point. PLC side: one op-multiplexed
builtin `FB_Socket` (`builtin_fb_layout` + `__tplc_rt_FB_Socket` in
tplc-rt) resolves the service through `ctx->svc` each poll, arms on
EXEC rising edge, polls until DONE or the TIMEOUT deadline; ERRID
carries the positive service code. `compat/tc2/tc2_tcpip.st`
wraps it in the Beckhoff FB names; `tc2_tcpip` ctest replays a
real loopback exchange through tether-plc-sim.
**Decisions:** (a) single generic FB instead of per-op builtins —
the op enum mirrors `TPLC_SOCK_OP_*` and keeps the builtin count
flat; (b) STRING[256] payload instead of POINTER TO BYTE (no
pointer type); (c) IPv4 literals only — DNS would block the scan;
(d) recv always reads the full DATA capacity so DLEN stays a pure
output and cannot inherit a stale cap.
**Consequence:** UDP ops and the vtable slots exist but only TCP is
e2e-verified; multicast, sSrvNetId/AMS routing, FB_SocketCloseAll,
and handle re-validation across program reload are documented gaps.

## D-IMPL-40 — tplc-serial service + FB_Serial builtin (plan 21 C2)

**Plan:** plan 21 §8 phase C2 continued: second runtime service,
unblocking Tc2_SerialCom.
**Implemented:** `include/tplc/tplc_serial.h` defines the
`TplcSerialService` vtable (ABI v1) returned by
`TplcServices.get_service("tplc-serial")`: open (termios configure:
baud/databits/parity/stopbits/raw/RS485 flag), single nonblocking
write/read, flush, RS485 direction stub, close — all bounded for
the scan path. Host side: `runtime/host/{include,src}/
tplc/host/SerialService.{hpp,cpp}` keeps a handle table over
O_NONBLOCK termios fds. PLC side: builtin `FB_Serial`
(`builtin_fb_layout` + `__tplc_rt_FB_Serial`) mirrors the
FB_Socket state machine. `compat/tc2/tc2_serialcom.st` wraps it as
`FB_SerialOpen/Send/Receive/Close/Flush/Rs485`; `tc2_serial` ctest
replays a real pty-pair loopback (python echo peer) through
tether-plc-sim.
**Decisions:** (a) same op-multiplexed builtin shape as FB_Socket —
consistency over per-op builtins; (b) explicit UDINT port handles
instead of TwinCAT's ComBuffer/SerialLineControl runtime objects —
the ComBuffer model is runtime-internal and cannot be replicated
in ST; (c) STRING[256] payload instead of POINTER TO BYTE;
(d) AUX carries op arguments (flush purge mask, rs485 enable).
**Consequence:** POSIX termios only; MCU UART backend, hardware
ComReset, 3964R/RK512 protocol FBs, and KL6xxx terminal FBs are
documented gaps.

## D-IMPL-41 — tplc-ads service + FB_Ads builtin (plan 21 C2)

**Plan:** plan 21 §8 phase C2 continued: third runtime service, the
ADS/AMS client, unblocking Tc2_System ADS FBs and remote `sSrvNetId`.
**Implemented:** `include/tplc/tplc_ads.h` defines the
`TplcAdsService` vtable (ABI v1) returned by
`TplcServices.get_service("tplc-ads"): open, read, write, read-state,
write-control, close — all nonblocking, one outstanding request per
connection. Host side:
`runtime/host/{include,src}/tplc/host/AdsService.{hpp,cpp}` speaks the
Beckhoff AMS/TCP wire format (6-byte TCP header, 32-byte AMS header,
ADS payload) over nonblocking sockets; connect completes lazily
inside the first request, each exchange is polled per scan. PLC
side: builtin `FB_Ads` (`builtin_fb_layout` + `__tplc_rt_FB_Ads`)
mirrors the FB_Socket/FB_Serial state machine; `RESULT` carries the
ADS-layer error, `ERRID` the transport code. `compat/tc2/
tc2_system.st` wraps it as `FB_AdsConnect/Read/Write/ReadState/
WriteControl/Close`; `tc2_system_ads` ctest replays a real AMS/TCP
exchange against `tests/compat/ads_test_server.py` (python ADS
responder) through tether-plc-sim.
**Decisions:** (a) explicit connect FB — TwinCAT connects implicitly
per (NETID, PORT) but the transport is TCP so a handle is needed;
(b) lazy connect inside the first request keeps `ads_open`
single-scan like the other services; (c) target NetID derived as
`host + ".1.1"` (Beckhoff device convention) with a placeholder
source NetID — real AMS routing needs a configured local NetID;
(d) STRING[256] payload instead of POINTER TO BYTE.
**Consequence:** client only; no AMS router/server, no
Read-Write(0x09)/notification/broadcast commands, Write Control
sends no extended control data. Indication FBs and ADSRDWRT are
documented gaps.

## D-IMPL-42 — tplc-rtmon service + FB_RtMon builtin (plan 21 C3)

**Plan:** plan 21 §8 phase C3: first T1-services-II item,
Tc3_RealtimeMonitoring; siblings Tc3_DynamicMemory (needs PVOID /
REFERENCE TO PVOID) and Tc3_Module (TwinCAT COM object model) are
blocked on missing language/runtime primitives.
**Implemented:** `include/tplc/tplc_rtmon.h` defines the
`TplcRtmonService` vtable (ABI v1) returned by
`TplcServices.get_service("tplc-rtmon"): rtmon_mark, rtmon_count,
rtmon_get, rtmon_clear`. Host side:
`runtime/host/{include,src}/tplc/host/RtmonService.{hpp,cpp}` keeps a
4096-entry ring of `TplcRtmonMark { ts_ns, context, group, event,
markctrl }`, oldest overwritten when full; its clock is injected by
`HostServices` as `steadyNs`, so `--sim` steps timestamp marks
deterministically. PLC side: builtin `FB_RtMon`
(`builtin_fb_layout` + `__tplc_rt_FB_RtMon`) is op-multiplexed
(mark=1/count=2/get=3/clear=4); unlike the socket services the ops
are synchronous O(1), so `DONE` sets on the EXEC edge scan with no
poll state. `compat/tc3/tc3_rtmon.st` wraps it as
`FB_RTMon_LogMarkBase`/`_LogMarkBaseEx` plus the eight
`FB_RTMon_Log*` simple-mark FBs embedding the `TcMarkOption` masks;
`tc3_rtmon` ctest replays a 10-state mark/count/get/clear sequence
through tether-plc-sim (10 packed checks, `0x3ff`).
**Decisions:** (a) synchronous ops — no deadline/poll fields in the
layout; (b) `rtmon_count` returns the retained count so it equals
the valid `rtmon_get` index domain (0 = oldest retained); (c) TwinCAT
FB methods map to one `bExecute`-gated FB per method since the
dialect has no METHOD; (d) `LogMarkEx` takes the `ST_RTMon_MarkDef`
fields flattened because struct types are file-scoped and cannot
cross `--lib`; (e) `RefToCaller`/`bLogCallingTask` records only the
flag bit — no task references exist.
**Consequence:** marks are in-process only; no realtime-monitor
server, trace export, marker-group naming, or server-side recording
trigger (`LogTrigger` stores a plain mark). `hrErrorCode` is 0 or a
negative service code, not a COM HRESULT.

## D-IMPL-43 — tplc-modbus service + FB_Modbus builtin (plan 21 C4)

**Plan:** plan 21 §8 phase C4: first connectivity item, Modbus;
TCP via `tplc-modbus`, RTU later on `tplc-serial`, UDP variants of
Tc2_ModbusSrv deferred.
**Implemented:** `include/tplc/tplc_modbus.h` defines the
`TplcModbusService` vtable (ABI v1) returned by
`TplcServices.get_service("tplc-modbus"): mb_begin, mb_poll,
mb_abort`. Host side:
`runtime/host/{include,src}/tplc/host/ModbusService.{hpp,cpp}`
implements Modbus TCP — MBAP + PDU with big-endian wire fields,
connect-per-transaction over nonblocking sockets, partial
send/receive driven per poll, MBAP-length bounded receive,
tid/proto/unit/func validation, exception responses via
`status`/`exc`; its clock is injected as `HostServices::steadyNs`.
PLC side: builtin `FB_Modbus` (`builtin_fb_layout` +
`__tplc_rt_FB_Modbus`) carries `FUNC` + a 256-byte `DATA` array
(request bytes in via `RLEN`, response out via `DLEN`), `RESULT`
0=ok/1=exception (`EXC`), `ERRID` transport codes. `compat/tc2/
tc2_modbussrv.st` wraps it as the ten documented TCP client FBs
(funcs 1/2/3/4/5/6/15/16/23/8); `tc2_modbussrv` ctest replays a
six-transaction sequence against a python Modbus TCP peer (6
packed checks, `0x3f`).
**Decisions:** (a) `DATA` is `ARRAY[0..255] OF BYTE`, not a string —
PDU payloads are binary and need indexed access; `RLEN`/`DLEN`
carry lengths explicitly; (b) the service copies the request at
`mb_begin`, so caller buffers need only live for that call;
(c) exception responses are normal completions (`status=1`,
`exc`=code), not transport errors — wrappers surface them as
`nErrId = 16#8000 + exc`; (d) connect-per-transaction matches the
TwinCAT FB model and keeps handles stateless across scans.
**Consequence:** IPv4 literals only (no DNS/IPv6); no slave/listen
mode. Two latent compiler/runtime bugs were fixed on the way: the
`fb_call` arg binder stored literal-width values into narrower
FB members (clobbering adjacent inputs such as `bExecute`), now
coerced to the member type; the ELF sandbox x86 length decoder
desynced on `0F 8x` jcc-rel32 and on `0x66`-prefixed imm32
instructions, fixed so generated code stops false-positiving as
kernel-gate opcodes.

**UDP follow-up (same change set):** service ABI bumped to v2 with
`mb_begin_udp` appended to the vtable; `FB_Modbus.UDP` (appended
last in the layout so earlier offsets stay fixed) selects the
transport per transaction. UDP carries the same MBAP+PDU ADU in
one datagram; the response is the first datagram from the request
peer covering the MBAP-declared length. All ten `FB_MBUdp*`
wrappers share the TCP wrapper bodies plus `mb.UDP := TRUE`,
completing Tc2_ModbusSrv at 20/20 catalog entries; `tc2_modbussrv`
ctest covers both transports (TCP `0x3f`, UDP `0x1f`).

## D-IMPL-44 — tplc-modbus RTU transport + FB_ModbusRtu builtin (plan 21 C4)

`include/tplc/tplc_modbus.h` ABI v3 appends `TplcModbusRtuReq` and
`mb_begin_rtu` to `TplcModbusService` (after `mb_begin_udp`,
handles share the TCP/UDP namespace). The RTU codec lives in
`ModbusService`: the serial device is opened per transaction with
termios raw settings derived from baud/parity/databits/stopbits;
the request frame is `unit | func | data | crc16lo | crc16hi`
(CRC-16/Modbus, low byte first); responses are validated on unit
id, CRC, and function code and then parsed by the shared
`finishPdu` used by TCP/UDP. Expected response length is derived
for fixed and byte-count functions; functions without a derivable
length complete on the T3.5 inter-frame gap computed from baud.
Nonblocking `read`/`write`; a 0-byte read under
`VMIN=0`/`VTIME=0` means "no data this scan", not EOF (treating it
as hangup desynced every transaction: each read the previous
transaction's late response and failed func validation).
PLC side: builtin `FB_ModbusRtu` (384-byte instance) mirrors
`FB_Modbus` with `DEV/BAUD/PARITY/DBITS/SBITS/UNIT` replacing
`HOST`/`PORT`. `compat/tc2/tc2_modbusrtu.st` wraps it as the ten
`ModbusRtuMasterV2_*` actions (funcs 1/2/3/4/5/6/15/16/23/8) with
serial settings as explicit inputs; `tc2_modbusrtu` ctest replays
the six-transaction sequence against a python RTU slave on a pty
pair (6 packed checks, `0x3f`).
**Decisions:** (a) per-transaction open keeps the service
stateless across scans like connect-per-transaction on TCP, at
the cost of termios setup per exchange; (b) gap-based completion
uses the last-byte timestamp (`last_rx_ns`) against the injected
`steadyNs` clock, so `--sim` timeouts stay deterministic; (c) the
RTU request reuses `TPLC_MODBUS_MAX_PDU` bounds since RTU ADUs
are smaller than TCP ADUs; (d) Tc2_ModbusRTU stays in the plan
catalog — slave blocks need RTU listen mode, `*_Generic`
ComBuffer variants need the ComBuffer object model, and KL6x*B
variants need EtherCAT terminal integration.
**Consequence:** master-only Modbus RTU over POSIX serial devices
(pty-verified); no RS-485 direction control beyond what the
adapter/driver provides, no broadcast unit id 0 handling beyond
normal framing, no slave mode.

## D-IMPL-45 — tplc-sacn service + FB_SacnSource/FB_SacnReceiver builtins (plan 21 C4)

`include/tplc/tplc_sacn.h` defines `TplcSacnService` (ABI v1):
`src_begin`/`src_poll`/`src_abort` for the ENABLE-lifecycle sender
and `rcv_poll`/`rcv_reset` for the receiver. The host impl
`SacnService` (`runtime/host/src/SacnService.cpp`) owns two
nonblocking UDP sockets: a sender bound to port 5568 that
multicasts per-universe E1.31-2018 data packets
(`239.255.x.y`, universe-derived group) and a receiver joined to
the same group with `SO_REUSEADDR` so source and receiver can
coexist on one host. The codec was verified byte-identical
against two independent Python reference implementations
(sacn-py and the sacn crate): root/framing/DMP vectors, the
`ASC-E1.17` preamble, sender CID, priority/sequence/universe
fields, the static DMP field descriptor `A1 00 00 00 01`, the
property count including the start-code byte, and flags+length
patching.

sACN priority is inverted (lower value wins). `rcv_poll`
replaces the held DMX buffer only on equal-or-better priority;
worse-priority datagrams increment `nDroppedPackets` and are
discarded. The receiver strips the start-code byte before
exposing `nDmxLength`/`DmxData`. Sequence-number tracking is
exposed via `nSequence` for gap detection at the PLC level.

PLC side: builtin `FB_SacnSource` (universe, priority, source
name, sequence, `DmxData[512]`/`nDmxLength`/`nErrId`/`EXEC`/
`BUSY`/`DONE`) and `FB_SacnReceiver` (universe filter,
`bHaveData`/`HAVE_DATA` latch, `sSourceName`, `nPriority`,
`nSequence`, `nDataPackets`, `nDroppedPackets`,
`DmxData[512]`/`nDmxLength`/`nErrId`). `compat/tc3/tc3_sacn.st`
wraps them as `FB_sACN_Source`/`FB_sACN_Receiver` (TwinCAT's
`I_sAcn*` callback model adapted to poll) plus the three
pure-IEC helpers `F_HighLowArr_TO_UINT`,
`F_UNIVERSE_TO_MulticastAdr` (returns the E1.31 multicast
address as packed UDINT), and `FB_UINT_TO_HighLowArr`.
`tc3_sacn` ctest replays a 600-step free-running sim against
`sacn_test_peer.py` — four packed checks (`0x0f`): own-source
receive, priority-10 preemption, priority-200 drop accounting,
helper conversions.

**Decisions:** (a) service-level multicast sockets rather than
per-universe socket pools — one sender/receiver pair covers
the universe the FBs are configured for, matching how the
twinCAT FBs bind one universe per instance; (b) receiver
accepts any source name on the configured universe (no
`F_SACN_DISCOVERY` personality filtering yet); (c) the peer
synchronizes on the DUT's own SRC1 stream before injecting
foreign packets, keeping the priority test deterministic
without wall-clock-dependent phase windows; (d) TwinCAT's
`I_sAcn*` COM-style interfaces stay unimplemented — same
blocker class as Tc3_EventLogger.
**Consequence:** E1.31 data path works end-to-end over real
multicast; discovery/sync packets are codec-complete but not
PLC-exposed; callback-driven TwinCAT sACN code needs porting
to the poll model.
## D-IMPL-46 — tplc-bacnet service + FB_Bacnet builtin (plan 21 C4)

`include/tplc/tplc_bacnet.h` defines `TplcBacnetService` (ABI v1):
`bac_begin`/`bac_poll`/`bac_abort`, one transaction per handle,
mirroring the modbus service shape. The host impl
`BacnetService` (`runtime/host/src/BacnetService.cpp`) owns a
nonblocking UDP socket per transaction (ephemeral port,
`SO_BROADCAST` for limited broadcast) and implements the
BACnet/IP (ASHRAE 135) BVLC + NPDU + APDU codec: BVLC type `0x81`
with unicast `0x0A` / broadcast `0x0B` functions, NPDU version 1
with the expecting-reply control bit for confirmed services,
confirmed-request APDU type 0 matched by invoke ID against
Simple-ACK (2) / Complex-ACK (3) / Error-Reject-Abort (5/6/7),
and unconfirmed type 1 for Who-Is/I-Am. Encodings were verified
against `bacpypes3` wire output.

Three services are exposed: Who-Is (unconfirmed svc 8 with an
optional device-instance range, collecting I-Am responses into a
bounded 8-responder table of object ID + responder IPv4 +
vendor/max-APDU/segmentation until the discovery deadline),
ReadProperty (confirmed svc 12, Complex-ACK echo of
object/property plus the first application-tagged value), and
WriteProperty (confirmed svc 15, Simple-ACK, optional array index
and write priority). Property values cross the ABI as raw
application-tag payload bytes — `val_type` selects the tag and
`val` carries the payload verbatim in wire byte order; the
service frames and deframes tags but never interprets payloads.
`bac_poll` uses a scalar `TplcBacnetRespInfo` (fixed 64-byte
layout) plus caller-owned value/responder buffers so the no_std
runtime bridge never hardcodes aggregate offsets; finished
transactions are dropped on the terminal poll, matching the
modbus model.

PLC side: builtin `FB_Bacnet` (712-byte layout) with FUNC
(1=Who-Is, 2=ReadProperty, 3=WriteProperty), dotted-quad HOST
(`""`/`"255.255.255.255"` = broadcast), PORT, object/property
fields, HAS_IDX/HAS_PRIO/HAS_RANGE qualifiers, raw `VAL[256]`,
and outputs STATUS (0 pending, 1 ok, 2 timeout, 3 BACnet
error, 4 bad response), R_* response echo, R_VAL payload, the
DEV_* responder table, ERRID for service/transport faults, and
EXEC/BUSY/DONE/`__last_exec` handshake. `compat/tc3/
tc3_bacnet.st` wraps it as `FB_BACnet_WhoIs`,
`FB_BACnet_ReadProperty`, `FB_BACnet_WriteProperty`.

TwinCAT's real BACnet integration is device-configuration-driven
(System Manager generates FBs per configured device; the plan
catalog lists only three documentation examples), so the compat
surface is the protocol layer plus poll-model wrappers, not a
mirrored TwinCAT API. `tc3_bacnet` ctest replays a 350-step
free-running sim against `bacnet_test_peer.py` — six packed
checks (`0x3f`): broadcast Who-Is/I-Am with range, ReadProperty
REAL, WriteProperty Simple-ACK, object-name charstring, Error on
unknown object, directed unicast Who-Is.

Two implementation defects were caught by the e2e: `BacnetService`
declared `clock_`/`clock_ctx_` before `svc_`, breaking the
`self()` = `&svc_` convention used by all host services (the vtable
lambdas receive the table pointer, not `this`) — `svc_` must stay
the first data member; and `bac_poll` originally retained finished
transactions (and their sockets) forever — it now drops them on
the terminal result like `ModbusService::mbPoll`.

## D-IMPL-47 — tplc-pjlink service + FB_Pjlink builtin (plan 21 C4)

`include/tplc/tplc_pjlink.h` defines `TplcPjlinkService` (ABI v1):
`pjl_begin`/`pjl_poll`/`pjl_abort`, one transaction per handle,
mirroring the bacnet service shape. The host impl
`PjlinkService` (`runtime/host/src/PjlinkService.cpp`) owns a
nonblocking TCP socket per transaction (connect-per-command,
`TCP_NODELAY`) and drives a Connect → Greet → Send → Recv state
machine against JBMIA PJLink on TCP 4352: ASCII command lines
`%<class><CMD4> <param>\r` (bare CR, no `$`), responses
`%<class><CMD4>=<value>\r`, `?` parameter for queries, and
PJLink-level `ERR1`–`ERR4` values distinguished from transport
status.

Authentication follows the spec: a projector may greet
`PJLINK 0\r` or `PJLINK 1 <8-hex-nonce>\r`; for the latter the
service prepends `MD5(nonce_ascii + password)` (32 lowercase hex
chars, no separator) to the command line via a compact in-tree
MD5 verified against the spec's worked example (`498e4a67` +
`JBMIA` → `8a36d09a…`). Projectors sending no greeting within
400 ms are treated as no-auth (legacy units). `PJLINK ERRA`
surfaces as `STATUS = 5`. `pjl_poll` uses a scalar
`TplcPjlinkRespInfo` (64-byte layout) plus a caller-owned value
buffer; finished transactions close the socket and drop the map
entry on the terminal poll.

PLC side: builtin `FB_Pjlink` (~376-byte layout) with HOST
(dotted-quad), PORT (0 = 4352), CLS (PJLink class digit),
CMD (4-char body), PARAM, PASSWORD, TIMEOUT, and outputs
STATUS (0 pending, 1 ok, 2 timeout, 3 error, 4 badresp,
5 authfail), R_CLS/R_CMD header echo, R_VALUE response text,
R_VLEN, AUTHED, ERRID, EXEC/BUSY/DONE handshake.
`compat/tc3/tc3_pjlink.st` provides the generic `FB_PJLink`
command wrapper plus `FB_PJLink_Power`/`FB_PJLink_Input`/
`FB_PJLink_Mute` method-shaped wrappers for the most-used
TwinCAT `FB_PJLink` operations.

TwinCAT's `FB_PJLink` is an auto-managed connection object —
`bKeepAliveSocket`, `tStatusUpdateTime`, `sServerNetID`,
`eTraceLevel`, `eStep`, `ipResultMessage` (`I_TcMessage`),
`stProjStatus`, `TcEventSeverity`/EventLogger, and `HRESULT`
method returns have no poll-model equivalent and are documented
incompatibilities; every command is a fresh TCP connection
(PJLink's safe pattern) and cyclic status polling is the
caller's job. `tc3_pjlink` ctest replays a 250-step free-running
sim against `pjlink_test_peer.py` — seven packed checks
(`0x7f`): unauthenticated POWR query, Power/Input/Mute
wrappers, MD5 auth accept, wrong-password ERRA, and `ERR1`
command-error surfacing.

## D-IMPL-48 — tplc-s7 service + FB_S7Conn/FB_S7Io builtins (plan 21 C4)

`include/tplc/tplc_s7.h` defines `TplcS7Service` (ABI v1):
`s7_open`/`s7_conn_poll`/`s7_close` for the persistent session
plus `s7_read`/`s7_write`/`s7_poll`/`s7_abort` for transactions,
split into connection and io handle spaces because S7comm is a
persistent COTP session, unlike the connect-per-command
services. The host impl `S7Service`
(`runtime/host/src/S7Service.cpp`) owns one nonblocking TCP
socket per connection and drives TCP connect → COTP CR/CC → S7
Setup Communication → Ready as an explicit stage machine;
transactions serialize one outstanding io per connection
(E_BUSY otherwise). `svc_` stays the first data member per the
BACnet `self()` invariant.

Wire stack verified against the python-snap7 reference server
(byte-exact read/write/bit round-trips): TPKT `03 00 <len16>`,
COTP CR `06 E0` / CC `06 D0` / DT `02 F0 80`, S7 Job header 10B
(`32 01`) vs Ack/Ack_Data header 12B with error class/code.
Read Var func `0x04`, Write Var func `0x05`, Setup Comm
func `0xF0` automatic on connect. COTP called TSAP folds
rack/slot as `0x0100 | (rack<<5) | slot`. Addressing uses
single-item S7-Any specs with the S7WordLen BYTE transport
code for byte arrays (count = byte count); bit access uses
BIT (`0x01`, `byte*8+bit`). Two codec subtleties were caught
by the reference server: the address-spec transport byte is a
type code, not a generic size (0x04 = WORD doubled the byte
count), and data-section lengths are in bits only for
transport sizes 0x04/0x05.

PLC side: builtins `FB_S7Conn` (HOST/PORT/RACK/SLOT/TIMEOUT,
CONN_HANDLE out, STATUS 0 pending/1 ready/2 error, CLOSE edge)
and `FB_S7Io` (CONN_HANDLE/MODE/AREA/DB/BYTE_OFF/BIT_ACCESS/
BIT_NO/LEN, DATA[240] write payload, R_DATA[240] read result,
STATUS 0..5, ITEM_RC, DATA_LEN, ERRID). `compat/tc3/
tc3_s7comm.st` provides `FB_S7_Connect`, `FB_S7_Read`,
`FB_S7_Write`, `FB_S7_ReadBit`, `FB_S7_WriteBit` covering the
`FB_S7CommConnection`/AddRead*/AddWrite* concepts and the
`FB_NLinkOpen`/`Close`/`Fetch`/`Write` set.

TwinCAT's `FB_S7CommConnection` request-queue model
(`AddRequest`, `FB_S7CommCyclicRequest`, `I_S7CommRequest`,
`PVOID`, `HRESULT`, AMS routing) is adapted to explicit
poll-model FBs; `s7_abort` closes the connection rather than
risk a stale response matching the next job's pdu_ref.
S5 protocol (3964/PKW) behind `FB_NLink*` on S5 targets is a
documented gap. `tc3_s7comm` ctest replays a 300-step
free-running sim against `s7_test_peer.py` — nine packed
checks (`0x1ff`): connect, DB read, write+readback, MK bit
read/write, unregistered-area s7err, refused connect, close.

## D-IMPL-49 — tplc-ftp service + FB_FtpConn/FB_FtpCmd/FB_FtpXfer builtins (plan 21 C4)

`include/tplc/tplc_ftp.h` defines `TplcFtpService` (ABI v1):
`ftp_open`/`ftp_conn_poll`/`ftp_close` for the control session,
`ftp_cmd`/`ftp_txn_poll` for one control command per handle, and
`ftp_xfer_begin`/`ftp_xfer_poll`/`ftp_xfer_feed`/`ftp_xfer_finish`/
`ftp_abort` for RETR/STOR/LIST data transfers. The host impl
`FtpService` (`runtime/host/src/FtpService.cpp`) owns one
nonblocking TCP control socket per connection and drives TCP
connect → 220 greeting → USER → PASS → Ready as an explicit
stage machine; the control channel is line-based with
multi-line reply handling (xxx- continuation vs xxx final).
Each xfer serializes TYPE I → PASV (227 tuple parse) → nonblocking
data connect → transfer command → 150 interim → stream → 226
final inside one handle, one outstanding transaction per
connection. `svc_` stays the first data member per the BACnet
`self()` invariant.

Scope is PASV passive data connections only — no PORT/active
mode, EPSV, FTPS/TLS, SFTP/SSH, REST resume, MLSD/MLST, or DNS;
hosts are dotted-quad IPv4. Downloads stream caller-sized chunks
per poll; uploads queue through a bounded 4 KiB pending buffer
fed by `ftp_xfer_feed` and closed by `ftp_xfer_finish`
(SHUT_WR). All deadlines use the injected host clock so `--sim`
stays deterministic.

PLC side: builtins `FB_FtpConn` (HOST/PORT/USER/PASS/TIMEOUT,
CONN_HANDLE out, STATUS 0 pending/1 ready/2 error, CLOSE edge),
`FB_FtpCmd` (CONN_HANDLE/CMD, RESP_CODE + TEXT reply, STATUS
0..5), and `FB_FtpXfer` (OP 0=RETR/1=STOR/2=LIST, PATH, BUF[512]
download chunks with CHUNK/TOTAL, UP_DATA[512]/UP_LEN/UP_OFF
auto-feed for uploads, STATUS adds 6=more-data). `compat/tc3/
tc3_ftp.st` provides `FB_FTP_Connect`, `FB_FTP_Cmd`,
`FB_FTP_Download`, `FB_FTP_Upload`, `FB_FTP_List`, and the
`FB_FTP_MkDir`/`RemoveDir`/`DeleteFile`/`FileExists`/
`DirExists`/`Rename` method wrappers covering the
`FB_FtpConnection`/`FB_FtpExplorer`/`FB_FtpTransfer` objects and
the `FB_FTP_*`/`FB_FTP_HostResolve`-adjacent Tc2 surface;
rename sequences RNFR+RNTO inside the wrapper.

Verified by `tc3_ftp` ctest: 500 free-running sim steps against
`tests/compat/ftp_test_peer.py` (python RFC 959 PASV responder,
TCP 2121), twelve packed checks (`0xfff`). A C++ harness
drove the same service standalone first, which caught a
reply-line scan off-by-one (`\n` at the buffer tail was never
detected) before any PLC layer was built.

## D-IMPL-50 — tplc-smtp service + FB_SmtpSend builtin + GSM AT SMS (plan 21 C4)

`include/tplc/tplc_smtp.h` defines `TplcSmtpService` (ABI v1):
`smtp_begin`/`smtp_poll`/`smtp_abort` drive one RFC 5321
transaction per handle (connect -> 220 -> EHLO -> optional
AUTH LOGIN 334/334/235 -> MAIL FROM -> RCPT TO -> DATA ->
dot-stuffed headers+body -> "." -> 250 -> QUIT -> 221) on a
single nonblocking TCP socket; `TplcSmtpInfo` carries STATUS
(0 pending / 1 ok / 2 smtp-fail / 3 timeout / 4 error /
5 badresp / 6 more-body), RESP_CODE (kept at the DATA-accept
250 rather than overwritten by the QUIT 221), BODY_TOTAL,
and ERRID. The host impl `SmtpService`
(`runtime/host/src/SmtpService.cpp`) owns the socket per
handle and resolves all stages inside `smtp_poll`.

TwinCAT's `FB_SmtpV3`/`FB_SmtpV3_Full` proxy through a TwinCAT
SMTP *server* over ADS (`sNetId`), which has no TetherPLC
equivalent; the compat layer therefore speaks SMTP directly to
the target server. Builtin `FB_SmtpSend` (named to avoid
colliding with the compat `FB_Smtp` alias) exposes HOST/PORT/
USER/PASS/FROM/RCPT/SUBJECT/TIMEOUT/BODY[255] with an EXEC
edge; `compat/tc2/tc2_sms_sms.st` provides `FB_SmtpV3`,
`FB_SmtpV3_Full`, obsolete `FB_Smtp`/`FB_SmtpFull` aliases,
`FB_SmtpAttach` (ERRID 90 stub; attachments unsupported), and
`FB_GetTcPlcSMS_Version`.

SMS reuses `tplc-serial`: `FB_SendSMS` is a pure-ST AT-command
state machine (AT -> AT+CMGF=1 -> AT+CMGS="n" -> ">" -> text +
Ctrl-Z -> +CMGS/OK) over `FB_SerialSend`/`FB_SerialReceive`.
Debugging surfaced two real bugs outside the feature itself:
`BuiltinFn::Concat`, `str_fit`, and the `fb_call` string path
all did capacity/index math in the string element type, so
STRING capacities over 255 truncated constants to zero and
produced empty strings (fixed by doing the math in i32 and
clamping once at the end); and the serial EXEC contract was
documented — `FB_Serial` arms on a 0->1 EXEC edge tracked by
`__last_exec`, so a consumer pulsing EXEC must call the FB
every scan or EXEC freezes high after a fast write and the
next kick is swallowed as a phantom DONE. `FB_SendSMS` drives
both serial FBs unconditionally with `bExecute` gated by phase
for exactly this reason.

Verified by `tc2_sms_sms` ctest: 700 free-running sim steps
against `gsm_modem_peer.py` (pty AT modem answering
AT/AT+CMGF=1/AT+CMGS text mode) and `smtp_test_peer.py` (RFC
5321 responder on TCP 2525 with AUTH LOGIN and a 550 reject
leg), nine packed checks (`0x1ff`). Documented gaps: no TLS,
attachments, receipts, Cc/Bcc, multi-recipient envelopes, or
MIME beyond From/To/Subject/Date; no ADS SMTP-server proxy;
dotted-quad IPv4 only; SMS PDU/UCS2/concatenation/status
reports unimplemented.

## D-IMPL-51 — tplc-iec870 service + FB_Iec870Conn/FB_Iec870Tx/FB_Iec870Rx builtins (plan 21 C4)

`include/tplc/tplc_iec870.h` defines `TplcIec870Service` (ABI v1,
service name `"tplc-iec870"`): `iec870_open`/`iec870_conn_poll`/
`iec870_send`/`iec870_tx_poll`/`iec870_rx_poll`/`iec870_conn_close`
drive an IEC 60870-5-104 telecontrol *client* on TCP (default
port 2404). The host impl `Iec870Service`
(`runtime/host/src/Iec870Service.cpp`) owns the socket per
handle and resolves all stages inside the poll calls, driven by
the injected host clock so deterministic sim replay holds.

Wire format follows IEC 60870-5-104 / lib60870: every APDU is
`0x68` + 1-byte length (bytes after the length octet) + 4-octet
control field. I-format carries 15-bit send/recv sequence
numbers shifted left one; S-format is the fixed 6-byte ack
(`0x01` first control byte); U-format is the fixed 6-byte
control frame (STARTDT ACT/CON `0x07`/`0x0B`, STOPDT
`0x13`/`0x23`, TESTFR `0x43`/`0x83`). Handshake is TCP connect
-> STARTDT ACT -> STARTDT CON -> active transfer; close sends
STOPDT ACT then closes without awaiting confirm; TESTFR
keepalive at the configured interval. ASDU header is type_id +
vsq (bit7 sequence flag, low 7 bits object count) + COT (low
byte cause, upper byte originator when COT size is 2) + common
address + raw information-object payload, with CA/COT widths
configurable 1 or 2 octets (default 2). The service does not
implement per-type information-object codecs; user ST supplies
and interprets raw payload bytes.

Builtins `FB_Iec870Conn` (session), `FB_Iec870Tx` (one ASDU per
EXEC edge, one outstanding transmit per connection, k-window
aware) and `FB_Iec870Rx` (consume-on-read, one queued ASDU per
DONE, bounded queue with drop accounting) bridge through
`TplcServices.get_service`; `svc_` is deliberately the first
data member so callbacks recover the containing object from the
service-table pointer. `compat/tc3/tc3_iec870.st` adds
`FB_IEC870_5_104Master`/`FB_IEC870_SendAsdu`/
`FB_IEC870_RecvAsdu` wrappers plus pure helpers: COT/TYPEID
strings for IEC 101/102/103, `SYSTEMTIME_TO_CP56Time2a` and
`CP56Time2a_TO_SYSTEMTIME` FUNCTION_BLOCKs (multi-output time
conversion, since FUNCTION VAR_OUTPUT params are not callable),
CP56 to CP24/CP32 truncation, IOA composition, SPI/DPI/VTI bit
helpers, and `GetIec870_Version` (`16#0100`).

Debugging surfaced a FUNCTION_BLOCK array-binding subtlety:
VAR_OUTPUT arrays bound via named call args do not write back
to the caller's array (outputs stay on the instance), so the
test copies `tenc.pTime[i]` into a local array before feeding
`tdec`. Verified by `tc3_iec870` ctest: 400 free-running sim
steps against `iec870_test_peer.py`, a python IEC
60870-5-104 slave on TCP 2480, twelve packed checks (`0xfff`)
covering connect/STARTDT, interrogation, M_SP+M_ME receive,
clock sync echo, single-command act-con, refused-port error,
helpers, and STOPDT close.

Documented gaps: no IEC 60870-5-101 serial, 5-102 HDLC, or
5-103 relay-protection transports (104 only); no slave/server
mode; no AO database, AMS NetID routing,
PVOID/pointer-to-array/interface/HRESULT object model,
redundancy groups, file transfer, or TLS; dotted-quad IPv4
only, no DNS.

## D-IMPL-52 — tplc-motion service: self-contained simulated DS402-shaped axis (plan 21 C5)

Plan 21 C5 (Motion core) needs an axis/motion backend for the
PLCopen MC2 function blocks. The plan flagged "whether
`tplc-motion` should live in this repo or in the sibling
Tether motion framework" as an open question; the owner chose
the hybrid: keep TetherPLC self-contained but shape the axis
model on Tether's CiA402/DS402 semantics so a later adapter
maps 1:1 onto `tether/motion` (`Cia402MotionController`,
`DS402MachineAdapter`) without rework.

`include/tplc/tplc_motion.h` defines `TplcMotionService` (ABI
v1, service name `"tplc-motion"`): each axis is a simulated
CiA402 drive exposing the DS402 object subset the MC layer
needs — Controlword (0x6040), Statusword (0x6041),
ModesOfOperation (0x6060)/Display (0x6061), TargetPosition
(0x607A), TargetVelocity (0x60FF), TargetTorque (0x6071),
PositionActualValue (0x6064), VelocityActualValue (0x606C),
TorqueActualValue (0x6077), plus FollowingErrorWindow (0x6065).
The host impl `MotionService`
(`runtime/host/src/MotionService.cpp`) owns a bounded axis pool
and advances deterministic dynamics on the injected host clock
(`HostServices::steadyNs`), so `--sim` steps replay exactly.

The axis runs the CiA402 controlword-command model: the low
nibble (EnableVoltage/QuickStop/SwitchOn/EnableOperation) selects
the target state and bit 7 FaultReset clears a latched fault,
producing the statusword bit pattern (NotReadyToSwitchOn 0x00,
SwitchOnDisabled 0x40, ReadyToSwitchOn 0x21, SwitchedOn 0x23,
OperationEnabled 0x27, QuickStopActive 0x07,
FaultReactionActive 0x0F, Fault 0x08). Operating modes mirror
CiA402Defs: ProfilePosition=1, Velocity=2, ProfileVelocity=3,
ProfileTorque=4, Homing=6, CyclicSyncPosition=8,
CyclicSyncVelocity=9, CyclicSyncTorque=10. Mode handlers
integrate actual position/velocity/torque toward the target on
each poll with configurable velocity/acceleration limits;
profile-position honors NewSetPoint and reports TargetReached
(statusword bit 10).

The ST-facing bridge is the synchronous op-dispatch builtin
`FB_MotionAxis` (`compiler/tplc-mir/src/layout.rs` +
`compiler/tplc-rt/src/lib.rs`): `OP` selects create / destroy /
poll / read / write / info and `STATUS` carries the service
return code (for `OP=6` info it carries the CiA402 state). All
service calls are immediate, so unlike the socket/serial
builtins it needs no EXEC/BUSY/DONE edge machinery; the compat
layer gates create/destroy timing.

The first MC2 tranche lands the axis-management wrappers in
`compat/tc2/tc2_mc2.st` (`MC_Power`, `MC_Reset`, `MC_SetPosition`,
`MC_ReadActualPosition`, `MC_ReadActualVelocity`, `MC_ReadAxisError`,
`MC_ReadStatus`), which drive the axis through `FB_MotionAxis`
object reads/writes. `AXIS_REF` is represented as the UDINT axis
handle and positions/velocities as int32 user units, both
documented simplifications pending an `LREAL` engineering-unit
conversion. `MC_SetPosition` and `MC_Reset` write only on their
execute edge / reset window so they neither fight `MC_Power` over
the controlword nor re-apply a stale `Position` input each scan.
The motion-command blocks (`MC_Move*`…`MC_TorqueControl`), the
full PLCopen state machine, and gearing/phasing/superposition
land in later tranches, still in pure ST on top of the same
object surface. Real-axis binding is a documented
adapter path: a production `tplc-motion` backend can map the
same object dictionary onto Tether's DS402/EtherCAT drives or a
CiA-402-over-ADS link; the simulated backend is the deterministic
test oracle.

## D-IMPL-53 — tplc-motion backed by Tether DS402 over the Tether IO protocol

Supersedes the in-process simulated axis as the primary backend.
TetherPLC motion now targets Tether's real CiA402/DS402 machinery
through the Tether IO binary protocol, with the simulation living
in Tether. The shared surface is the DS402 object dictionary: the
same indices the `FB_MotionAxis`/`motion_obj_*` ABI already uses
(Controlword 0x6040, Statusword 0x6041, ModesOfOperation 0x6060,
TargetPosition 0x607A, TargetVelocity 0x60FF, TargetTorque 0x6071,
Position/Velocity/TorqueActualValue 0x6064/0x606C/0x6077,
FollowingErrorWindow 0x6065, error code 0x603F, home offset 0x607C).

Tether exposes that dictionary over IO. `machine.cia402.v1` keeps
`machine.sdo.list`/`read`/`write` for commissioning and object
discovery behind `IMachineSdoAccess`, but those functions are
authenticated, Technician-role and rate-limited to one write per
50 ms, so they are unsuitable for cyclic controlword/target
updates. Cyclic motion therefore travels as ordinary IO parameters:
`tether_io_motion_server` registers each DS402 object as a
`mc.<slave>.<index>` param (native width: I8 modes, I16 torque,
I32 position/velocity, U16 control/status/error, U32 follow
window) backed by `SimulatedCiA402Drive`, a new
`IMachineSdoAccess` implementation in Tether
(`include/tether/io/SimulatedCiA402Drive.hpp`) that interprets
controlword/mode/target writes, runs the CiA402 state machine and
profile dynamics (PP trapezoid, HM, VL, PV, TQ, CSP/CSV/CST) on an
injected microsecond clock, and reports statusword/actuals on
read. Swapping `SimulatedCiA402Drive` for `EtherCatSdoAccess`
exposes the same param surface over real EtherCAT CoE drives.

Mapping on the TetherPLC side (`MotionIoService`, selected by
`HostServices` when `TPLC_MOTION_IO` names a reachable endpoint,
else the in-process `MotionService`): `motion_obj_read/write`
translate to `getParam`/`setParam` on the axis's `mc.<slave>.
<index>` param after `listParams` discovery; `motion_axis_info`
reads the statusword/mode/actual params; `motion_axis_create/
destroy` map handles to free slave indices. `FB_MotionAxis` and
all `compat/tc2/tc2_mc2.st` wrappers are unchanged (same C ABI).
The service's `svc_` table stays the first member so the rt
bridge's `&svc_` service pointer recovers the object.

Determinism: `tether_io_motion_server --clock sim` advances the
drives on a writable U64 `mc.time` param (microseconds) instead of
the wall clock. `MotionIoService` holds the host steady clock and
writes `mc.time` before each object op (deduplicated per tick), so
`tether-plc-sim --sim` timestamps drive the remote drives exactly
and replay is bit-identical. `--clock realtime` keeps wall-clock
operation for live (non-sim) PLCs. The drive integrates in
bounded <=100 us substeps so sparse client polls cannot overshoot.

Known gaps: `motion_axis_create` validates the config but does not
propagate units/velocity/accel/position-limit values to the remote
drive, which uses its server-side configured dynamics; wire
`writeFn` is void so drive-side write rejections cannot propagate,
so the client pre-validates mode-of-operation and clamps the
follow window to match the local backend. The in-process
`MotionService` remains as the offline fallback.

## D-IMPL-54 — MC command FBs share the DS402 controlword via edge writes

The PLCopen command blocks (`MC_MoveAbsolute`/`MC_MoveRelative`,
`MC_MoveVelocity`, `MC_Stop`, `MC_Halt`, `MC_Home`, `MC_Jog`) all
drive one shared DS402 controlword, so the arbitration rule is
"last writer wins per scan". To keep independent FBs from
corrupting each other's command bits, every writer is
edge-triggered: `MC_Power` writes EnableOperation (0x000F) /
DisableVoltage (0) only on an `Enable` transition, and each command
FB writes the controlword only on its `Execute` rising edge plus —
for ProfilePosition/Homing — one follow-up scan that clears the
NewSetPoint bit (0x001F then 0x000F). The drive holds the controlword
as a level, so a held enable persists without per-scan rewrites.

Mode/target selection per command: PP mode (0x6060=1) +
TargetPosition (0x607A) for point-to-point moves, PV mode (0x6060=3)
+ TargetVelocity (0x60FF) for velocity/halt/jog, Homing (0x6060=6) +
HomeOffset (0x607C) for `MC_Home`, and a cleared QuickStop bit
(0x000B) for `MC_Stop` so the drive decelerates into
QuickStopActive and stays locked until a new command re-asserts the
enable pattern. Because each command edge re-asserts the enable
pattern, a velocity move resumes cleanly after a quick-stop. A
consequence: a command FB implicitly enables the axis (it writes
the full enable pattern), which is a documented simplification over
strict PLCopen power-state checking. `Done`/`InVelocity`/`Busy`
come from `FB_MotionAxis` OP=6 info (TargetReached /
VelocityActualValue); `CommandAborted` is currently always FALSE —
mid-move abort detection is a documented gap.

## D-IMPL-55 — MC coupling/probe FBs are software approximations over object ops

The two-axis and latch blocks (`MC_TouchProbe`, `MC_GearIn`,
`MC_GearOut`) are implemented as pure-ST approximations on top of the
existing `FB_MotionAxis` object ops — no new ABI, and they work
unchanged on the in-process `MotionService` and the Tether IO
`MotionIoService`/`SimulatedCiA402Drive`. Two simplifications are
documented rather than faked:

`MC_TouchProbe` is a cycle-level software latch, not a hardware
touch-probe. On each `Execute` rising edge it arms a per-FB latch that
reads `PositionActualValue` (0x6064) once per scan and captures it on
the `TriggerInput` rising edge (optional `WindowOnly`/`FirstPosition`/
`LastPosition` gating). TwinCAT's real touch-probe latches sub-cycle
against an encoder event in the drive; neither the in-process axis nor
`SimulatedCiA402Drive` exposes a latch object, so capture latency is
one scan and trigger edges are scan-synchronous. Exact hardware-latch
semantics need a DS402 latch object (e.g. 0x60B8/0x60B9/0x60BA) added
to both backends.

`MC_GearIn` is velocity-ratio coupling, not exact position-phase lock.
On `Execute` it enables the slave and selects ProfileVelocity
(0x6060=3), then each scan reads master `VelocityActualValue` (0x606C)
and writes slave `TargetVelocity` (0x60FF) = master_vel *
RatioNumerator / RatioDenominator (zero denominator -> Error). The
slave's velocity tracks the master's scaled, but position phase is not
locked — TwinCAT gearing couples cyclic position setpoints, which
needs a dedicated coupling object or CSP setpoint streaming.
`MC_GearOut` writes slave 0x60FF = 0 on its edge and reports `Done`
when `VelocityActualValue` reaches zero.

`MC_AbortTrigger` is intentionally omitted: aborting an armed probe
requires shared per-axis probe state the FB-local latch model does not
provide — each `MC_TouchProbe` owns its `armed` flag, so a separate
abort FB cannot see it. Supporting it needs the probe state moved into
the axis/backend (a shared armed-slot per axis) — a real backend
change deferred with the latch object above.

## D-IMPL-56 — NC-I channel interpolation as FB-level CSP setpoint streaming

TwinCAT NC-I (TF5100) is a G-code/FIFO interpreter in the NC kernel
that emits per-axis CyclicSyncPosition setpoints every NC cycle. The
compat model does not stand up a full interpreter; it collapses one
in-flight geometric block into each function block so the same
setpoint-streaming mechanism is exercised honestly.

`FB_NciLine` parameterizes a straight line across a 3-axis group
(X/Y/Z axis handles — there is no backend group registry, so a
"group" is just the handles passed to the block, which is how
`CfgBuild3DGroup` maps). On `Execute` it latches each axis's actual
position as the segment start, computes deltas and the Euclidean path
length via `F_NciSqrt` (a fixed-iteration Newton sqrt — the ST layer
has no SQRT builtin), switches all three axes to CSP mode (0x6060=8),
then each scan advances `s` by `Velocity*CycleUs/1e6` and streams each
axis's interpolated `TargetPosition` (0x607A). CSP snaps `pos =
targetPos` per cycle on both the in-process `MotionService` and the
remote `SimulatedCiA402Drive`, so commanded and actual positions
match and the move is a true straight line with simultaneous arrival.

The FB has no host clock, so `CycleUs` supplies the scan period
(time-based path velocity); under `--sim`, `CycleUs` = the step period
keeps the replay deterministic. `FB_NciEStop` writes DisableVoltage to
the group. Honest gaps needing a real backend channel object: the
G-code/DIN-66025 interpreter, FIFO block queue and look-ahead,
circular/helical/spline interpolation (sin/cos), M-functions,
tool/R-params/zero-shift, and multi-block blending.

This tranche also fixed a real `tether_io_motion_server` bug: the
per-object `readFn`/`writeFn` lambdas captured `slave` by reference
(`[&, idx, sz]` lets it fall under the `&` capture-default, leaving
`slave` bound to a dead stack slot holding the last loop index). Every
registered param therefore read/wrote the last-configured slave —
invisible for single-axis use, breaking any multi-axis group. Captures
now bind `slave` by value.

## D-IMPL-57 — G-code interpretation owned by Tether, PLC controls the channel

TwinCAT NC-I's interpreter is a CNC program engine (G-code blocks,
M-functions, look-ahead, R-parameters). Reimplementing that inside
TetherPLC would duplicate Tether's existing state-of-the-art G-code
subsystem, so interpretation was delegated: `tether_io_motion_server`
exposes an `nc.*` parameter channel that compiles program text or
files with `GCode::PlanningSegmentBuilder`, time-parameterizes the
result with `MotionPlanner::MotionPlanBuilder` (corner blending plus
the configured velocity profile over linear, arc and dwell segments;
zero-length moves are dropped and a `planSegToSeg` map preserves
source-line reporting), then streams scaled `0x607A` setpoints to
slaves 0..2 in CSP mode on the `mc.time` drive clock — the same
deterministic model the DS402 axes already use under `--clock sim`.

The PLC surface is deliberately thin: a `tplc-nci` vtable
(`load_text`/`load_file`/`command`/`status`), an `NciIoService` that
maps ops to `nc.*` params (active only when `TPLC_MOTION_IO` names an
endpoint that exposes them; no in-process fallback — honest absence
rather than a second interpreter), the `FB_NciChan` builtin (OP
1..4), and `FB_Itp*` wrappers in `tc2_nci.st` mirroring the TwinCAT
`ItpLoadProg`/`ItpStartStop`/`ItpReset`/`ItpEStop`/`ItpIsEStop`
surface. Channel commands: run/step/pause/resume/stop/reset/verify;
`stop` is permissive (any non-Idle/Error state drops to Stopped, like
TwinCAT's `bStop`) and `step` runs one source block then pauses.
`nc.scale`/`nc.feedovr` are RW config params.

Programs are bounded by the ST string transport (PROG cap 4095,
PATH cap 127); larger programs load by server-side path. `nc.cmd`
values are `GCode::InterpreterState`-aligned for `nc.state`
reporting; `nc.errcode` carries channel errors (1 bad cmd, 2 bad
state, 3 compile, 4 plan build, 5 unsupported segment type).

## D-IMPL-58 — NC FIFO, M-function handshake, and R-params live on the channel

TwinCAT NCI's FIFO model (blocks appended while the channel runs)
and auxiliary surface (M-function handshake, R-parameters, zero-shift)
map onto the `nc.*` channel, not into the PLC:

- `nc.fifo` queues one block per write after a trial compile. The
  channel chains queued entries as continuation plans, each anchored
  at the last commanded position — a FIFO-only program also runs from
  Idle. `nc.fifolen`/`nc.fifoseq` report depth and consumption.
- `nc.mfunc`/`nc.confirm` implement the handshake: the channel scans
  the executing block's source text, pauses on M-codes outside the
  program-control set (M0/M1/M2/M30 stay interpreter-owned) and
  exposes the pending code until `nc.confirm` resumes it.
- `nc.ridx`/`nc.rval` write numbered registers and `nc.zsx/y/z` a
  `G92` offset; both are injected as prefix lines at compile, which
  makes the interpreter genuinely evaluate them (R-params land in
  `VariableSystem`) while being stripped from visible block/line
  numbering. In the builder, `PlanningSegment.blockIndex` now maps
  segments to real source blocks via `mseg.lineNumber` (it was a
  segment ordinal), fixing `nc.line` under multi-segment blocks.

PLC side: `tplc-nci` gains `nci_fifo`/`nci_confirm`/`nci_rparam`/
`nci_zshift` (status struct carves `mfunc`/`fifolen`/`fifoseq` out of
reserved so the 64-byte ABI is unchanged), `FB_NciChan` ops 5-9, and
`FB_ItpFiFoWrite`/`FB_ItpConfirmHsk`/`FB_ItpWriteRParam`/
`FB_ItpReadRParam`/`FB_ItpZeroShift` wrappers. Verified end-to-end:
`tc2_nci_gcode_io` mask `0x7fffff` (FIFO queue, run-from-idle,
chained execution, R-param roundtrip, zero-shift, M8 pause ->
confirm -> resume). Cyclic-channel offset tables remain an honest
gap.

## D-IMPL-59 — OPC UA via libopen62541, opt-in like the other special features

TwinCAT's OPC UA surface is a client (`Tc3_PlcOpenOpcUa`/`TF6100`) over
a real UASC stack. Rather than hand-roll the binary protocol, the
service uses **libopen62541** — the maintained stack — wired opt-in like
Tether's other special features (`-DTPLC_ENABLE_OPCUA`, auto-detected,
silently inert when `libopen62541-dev` is absent; `getService` then
returns nullptr, honest absence rather than a stub).

Split: Tether hosts the *server* (`tether_io_motion_server --opcua`,
open62541 folding the motion Registry into `ns=2;s=<name>` nodes), the
PLC hosts the *client* (`tplc-opcua` = an `OpcUaService` holding up to
8 `UA_Client` sessions; `FB_UaClient` ops 1-6; `FB_UA_*` wrappers in
`tc2_opcua.st`). Node ids travel as text (`ns=2;s=name`, `i=85`) parsed
per call — TwinCAT's `UA_NodeGetHandle` handle-table is collapsed into
per-call resolution (same semantics, no handle round trip). Numeric
values widen into `LREAL` (64-bit nodes lose precision above 2^53 —
documented); strings into `STRING[255]`.

Verified end-to-end with a real open62541 client on both sides:
the server was validated first (connect/read/write/browse, 79 nodes),
then `tc2_opcua_io` drives the PLC client through connect → session →
read `nc.state` → write+read `nc.scale` → ns-index → disconnect.
Gaps kept honest: no browse/node-handle/subscription/method wrappers
yet.

## D-IMPL-60 — EtherNet/IP explicit messaging, bounded nonblocking IO

TwinCAT's EtherNet/IP surface (Tc3_EtherNetIP / TF6281) is a
scanner-side client over the encapsulation layer on TCP 44818. The
service talks RegisterSession/UnRegisterSession, ListIdentity, and
SendRRData carrying unconnected CIP Get/Set_Attribute_Single — the
read/write/status core TwinCAT apps use most; cyclic implicit
(class-1) I/O, ForwardOpen, and other CIP services are a separate
surface kept out of scope.

Every socket op is nonblocking with a `poll` budget (~150 ms) well
under the PLC scan-stall watchdog — a stuck adapter fails the call
(TPLC_EIP_E_IO) instead of stalling the task; a blocked `recv`/`connect`
must never take a scan over the watchdog. CIP object addressing is
class/instance/attribute u16 with automatic 8-bit/16-bit path-segment
encoding; attribute payloads are bounded `STRING[255]` bytes (TwinCAT's
PVOID buffers map to the string carrier). `FB_Eip` op-multiplexes
connect/close/status/identity/get/set; `FB_Eip*` wrappers expose the
TwinCAT shape. Verified at the wire level with `eip_adapter_fake.py`
(RegisterSession, ListIdentity, attribute write+read roundtrip,
unregister — `0x7f`).

## D-IMPL-61 — IEC 61850 via libiec61850, auto-downloaded opt-in

**Decision.**  The `tplc-iec61850` MMS client wraps libiec61850's
`IedConnection_*` API rather than a hand-rolled OSI stack: the full
TPKT/COTP/Session/Presentation/ACSE/MMS path is too large to own.
Like OPC UA it's opt-in — `-DTPLC_ENABLE_IEC61850` (default ON)
auto-downloads and builds libiec61850 via CMake FetchContent (a
system `libiec61850` is preferred when present); when the library is
absent or the option is off the service compiles inert and
`getService("tplc-iec61850")` returns nullptr.

**Consequences.**  The `FB_Iec61850` surface is
connect/close/status/read/write/GetNameList — the synchronous
confirmed-request primitives — addressed by domain+item with an `#FC`
suffix for the functional constraint.  Reporting/RCB, GOOSE/SMV,
control sequences, and the full data model are honest gaps for a
later tranche.  Blocking calls carry 800 ms connect/request timeouts
below the scan-stall watchdog.  The e2e fake is a real libiec61850
`IedServer`, not a protocol stub.

## D-IMPL-62 — MQTT via libpaho-mqtt3c, auto-downloaded opt-in

**Decision.**  The `tplc-mqtt` client wraps Eclipse Paho's
synchronous `MQTTClient` (libpaho-mqtt3c) rather than a hand-rolled
broker wire codec.  Like the other connectivity services it's
opt-in — `-DTPLC_ENABLE_MQTT` (default ON) prefers a system
libpaho-mqtt3c and falls back to a CMake FetchContent build of
paho.mqtt.c; absent or off → `getService` nullptr.

**Consequences.**  The `FB_Mqtt` surface is connect/disconnect/
status/publish/subscribe/unsubscribe/receive/isConnected — the
bounded confirmed-request primitives.  `receive` is a nonblocking
poll (`MQTTClient_receive` with a 0 ms wait) so the cyclic scan
can't stall.  `Retain`/`RETAIN` are ST reserved keywords, so the
wrapper surfaces them as `RetainOn`/`FLG_RETAIN`.  MQTT5
properties, wildcard-matching detail, QoS2 two-phase, retained
messages, and TLS are honest gaps.  The e2e peer is a real
CONNECT/CONNACK/SUBACK/PUBLISH-echo fake, not a stub.

## D-IMPL-63 — OCPP-J via hand-rolled WebSocket + JSON-RPC, always on

**Decision.**  The `tplc-ocpp` charge-point client is TCP +
RFC6455 WebSocket + OCPP JSON-RPC array framing, hand-rolled like
`tplc-eip` — the wire protocol is small enough that an external
library adds dependency without surface value.  Always available
(no opt-in lib).

**Consequences.**  The `FB_Ocpp` surface is connect/disconnect/
status/call/result/callerror/receive — the WebSocket upgrade plus
the three OCPP-J frame types.  Payloads are raw JSON text the
caller composes/parses (bounded `STRING[255]`); per-action schema
validation is honest out-of-scope.  `ACTION` is a reserved keyword
surfaced as `ACT`/`ActName`.  Socket IO is poll-bounded under the
scan-stall watchdog.  The e2e peer is a real WS-upgrade +
OCPP-J echo fake.

## D-IMPL-64 — AES70/OCA via hand-rolled OSCP+OCP codec, always on

**Decision.**  The `tplc-aes70` audio-control client is OCP1 sync
header + OcaMessage PDU framing over TCP, hand-rolled like
`tplc-eip` — the OCA wire protocol is small enough that an
external library adds dependency without surface value.  Always
available (no opt-in lib).

**Consequences.**  The `FB_Aes70` surface is connect/disconnect/
status/call/keepalive/receive — the OSCP session plus method calls
on OCA objects (targetONo + methodID + raw parameter bytes) and
response polling.  Raw parameter payloads are bounded `STRING[255]`
the caller serializes; the full OCA class catalog, notification
subscriptions, and property shorthands are honest gaps.  `METHOD`
is a reserved keyword surfaced as `METH`.  Socket IO is
poll-bounded under the scan-stall watchdog.

## D-IMPL-65 — building buses deferred, serial plumbing already exists

**Decision.**  The building-bus surfaces (DALI, DMX512, EIB/KNX,
LON, M-Bus, EnOcean) are documented as a gap rather than faked.
The serial plumbing already exists (`tplc-serial` + `FB_Serial`,
`tc2_serialcom.st`), and DMX-over-IP is covered by `tplc-sacn`; the
missing piece is each bus's framing/management layer, not hardware
access.  A real per-bus layer is deferred to a serial-media
tranche or a bus-specific service.

**Consequences.**  C7 connectivity wave 2 closes with six real
client surfaces (OPC UA, EtherNet/IP, IEC 61850, MQTT/IoT, OCPP,
AES70) all verified by protocol-level e2e.  The building-bus gap is
documented honestly rather than stubbed.

## D-IMPL-66 — building buses as opt-in serial services

**Decision.**  The building-bus surfaces (DMX first) are opt-in
serial services — each owns its UART and opens it only when its
surface is used, like the IP services own their sockets.  This
satisfies the "serial port access (opt-in) for all the protocols"
requirement: the serial port is per-service and per-open, not a
global always-on resource.

**Consequences.**  `tplc-dmx` emits real DMX512 frames (TIOCSBRK
break + MAB + start code + 512 ch at 250 kbaud 8N2) — a real
serial byte stream a PTY fake reads.  The PLC-side 512-slot image
is a snapshot the scan emits at its own cadence.  RDM and receive
are honest gaps.  DALI/EIB/LON/MBus/EnOcean follow the same
opt-in-serial pattern, each a small service + builtin + wrappers
+ PTY-fake e2e.

## D-IMPL-67 — C8 measurement/control as pure-ST FBs

**Decision.**  The C8 measurement/control libraries are pure-ST
compat FBs — no service, no hardware.  Filter blocks take
`CycleTime` as a VAR_INPUT (TwinCAT's T_PLC_US pattern) rather
than a runtime cycle-time builtin, keeping them honest and
portable.  `FB_PID` uses back-calculated anti-reset-windup on the
integrator.

**Consequences.**  `FB_Limit`/`PT1`/`Integrator`/`Differentiator`/
`Average`/`Ramp`/`PID` are the first tranche; PT2/PT3, dead-time,
statistics and the full `FB_CTRL_*` catalog are honest gaps.

## D-IMPL-68 — PackML as a pure-ST ISA-TR88 state machine

**Decision.**  `FB_PackML` is a pure-ST ISA-TR88 unit-state
machine — the 17 nominal states and the command-driven transition
table, no service or hardware.  *ing wait-states self-advance when
the Busy input clears (modeled as immediate completion); the
caller holds work state in Busy.

**Consequences.**  The unit-state model is real and transition-
correct; the PackML tag/interface layer, unit mode vs unit state,
and the suspend/hold sub-state detail are honest gaps.  No
CONSTANT support in the dialect — state/command values are
documented in the file header.

## D-IMPL-69 — ControllerToolbox as pure-ST primitives

**Decision.**  `tc2_controllertoolbox.st` is a pure-ST controller
primitive library — no service or hardware.  `FB_Statistics`
computes StdDev via Newton's sqrt iteration because the dialect
has no SQRT builtin.

**Consequences.**  `FB_DeadTime`/`Hysteresis`/`TwoPosition`/
`ThreePosition`/`Statistics`/`Pwm` are the tranche; the full
`FB_CTRL_*` catalog and feedforward structures are honest gaps.

## D-IMPL-70 — TempController as pure-ST PID+PWM loop

**Decision.**  `FB_TempController` is a pure-ST temperature loop —
sensor filter, PID, PWM duty output.  The thermal plant lives in
the caller (or a simulated plant in the e2e).

**Consequences.**  The loop is real; auto-tuning, multi-zone and
the full `FB_Temp*` catalog are honest gaps.

## D-IMPL-71 — Weighing as pure-ST load-cell blocks

**Decision.**  `FB_Weigh` is a pure-ST weighing loop — PT1 filter,
tare, net, stability band and dosing feed outputs.  The cell
signal lives in the caller.

**Consequences.**  Tare/net/stable/dosing verified; multi-range,
auto-calibration and belt-weighing are honest gaps.

## D-IMPL-72 — PowerMonitoring as pure-ST metering

**Decision.**  `FB_PowerMonitor` is a pure-ST single-phase meter —
RMS/P/S/Q/PF/energy over a rolling sample window, Newton's sqrt.

**Consequences.**  Single-phase metering verified; three-phase and
harmonic metering are honest gaps.

## D-IMPL-73 — SPA as pure-ST signal analysis

**Decision.**  `tc2_spa.st` is a pure-ST signal-analysis surface —
peak detect, envelope, DFT-bin magnitude, cross-correlation.
Newton's sqrt replaces SQRT; the DFT bin is an honest approximate
because the dialect has no trig builtins.

**Consequences.**  The analysis primitives are real; a true FFT
and Bode analysis are honest gaps (need trig builtins).

## D-IMPL-74 — CM as pure-ST curve/math

**Decision.**  `tc2_cm.st` is a pure-ST curve/math surface —
function generator, piecewise-linear interpolation, linear cam.

**Consequences.**  FuncGen/LinInterp/CamProfile verified; spline
interpolation and cam table management are honest gaps.

## D-IMPL-75 — QRC as pure-ST payload validation

**Decision.**  `tc2_qrc.st` implements the checkable part of a
QR/barcode surface in pure-ST: `FB_QrcValidate` verifies EAN-13
and UPC-A check digits on a decode result that arrives from an
external scanner or camera.

**Consequences.**  EAN-13/UPC-A validation verified (`0x07`).
A true QR decoder needs an image pipeline and Reed-Solomon, and
the Beckhoff Tc3_QRC Q-SYS response frames need the TCP service
of the real library — both are honest gaps, not pure-ST math.

## D-IMPL-76 — tc2_plastic e2e: deep debug findings (open)

Context: `tests/compat/tc2_plastic_case.st` ends at mask 0x17 (expected
0x3f) despite a fully-verified library.

Established facts (all verified by disassembly of the packaged .so and
`expectf` blob reads, 2026 session):

- Pou logic is correct (isolated probe drives 0→4 through all phases).
- Consumer-side `layout.fb_types["FB_PlasticShot"]` dumped: MoldTarget@0,
  Execute@8, StartCycle@9, ScrewPos@16, CavityPressure@24, FillVel@32,
  CoolTime@40, ShotSize@48, VpSwitchPos@56, CycleTime@64, Reset@72,
  Done@73, Error@74, CycleActive@75, CycleFinished@76, ErrorID@80,
  Phase@84, PressureSetpoint@88, VelSetpoint@96, ph@104, fin@108,
  tHold@112, vel@120, pOut@128; size=136. Program blob: cavity@0,
  mask@8, n@12, pSeen@16, screw@24, st@32, shot@40 (total 176).
- Program vars are **name-sorted within regions** (layout_program), not
  declaration order — do not hand-compute offsets from source order.
- Caller .so disasm: every member access at the correct offset
  (verified instruction by instruction). Pou .so disasm: correct too.
- `expectf` DBG_READ data contradicts live execution: StartCycle@49
  reads 1 while the running program provably wrote 0 (st advanced past
  the clearing state). → the DBG blob is a snapshot/stale buffer, not
  the live frame. Treat expectf values with caution.
- In the live program: Phase reaches 3 (+1,+2,+4,+16 land), ScrewPos
  accumulates, but `pSeen := shot.PressureSetpoint` yields ~128
  (ScrewPos-like) and `CycleFinished` never reads TRUE even after the
  `fin` latch fix.

Remaining hypothesis to check next session: two different instance
pointers — the DBG_READ snapshot buffer vs the live frame — would mean
expectf offsets need mapping through the snapshot layout, and the real
+8/+32 failure needs a runtime-side trace (printf in the pou .so, or a
watch on the live frame via TPLC_OP watch/trace ctl ops — see
runtime/tether-plc-sim/main.cpp --scan path).
