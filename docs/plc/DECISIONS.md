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
