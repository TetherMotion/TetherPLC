# 07 — IR Codegen: `tplc-mir` → LLVM IR, State Layout, `tplc-rt`

> **Status:** work-item document; deleted when `docs/plc/ir-codegen.md` +
> `docs/plc/stdlib.md` exist and the codegen produces conforming
> artifacts for the full reference corpus.
>
> **Satisfies:** G1 (one backend), G2 (static code), G4 (layout+migrate
> is codegen), G8 (symmap/DI emission), G12 r3 (clean-room `tplc-rt`).
> **Depends on:** 01, 02, 05, 06, 13 (all inputs land in `tplc-mir`).
> **Informs:** 08 (migrate mechanics), 10, 14.

---

## 1. Position in the pipeline

```
ST frontend ─┐
             ├─► tplc-mir ─► [sema/typeck] ─► [layout] ─► [llvm-lower]
FBD lower ───┘                                   │            │
                                                 ├─► layout_desc/symmap
                                                 ├─► migrate codegen
                                                 └─► module.bc ─► target .so
                          tplc-rt (stdlib .bc) ──┘ (linked in pre-codegen)
```

Everything below `tplc-mir` is unconditionally ours (doc 13 §8). This
chapter specifies: the mir contract, layout algorithm, LLVM emission
rules, migrate generation, the clean-room runtime library, and
reproducibility.

## 2. `tplc-mir` contract (input)

`tplc-mir` (Rust crate `tplc-mir`) is a typed, provenance-complete IR:

- `Module { pous: POU[], gvl: GvlDecl[], tasks: TaskDecl[], io_at: AtDecl[] }`
- `POU { kind, name, iface: VarDecl[], body: Stmt[] (program/fb) | Expr (fn) }`
- `Stmt`: Assign | If | Case | For | While | Repeat | Call(FBInst,
  args) | Return | Exit | Continue | Label/Goto (from FBD) — each with
  `prov: Prov {st span | fbd node | generated}`.
- `Expr`: typed tree; literals resolved; conversions explicit nodes
  (sema inserts them — codegen never guesses a conversion).
- `VarDecl {name, varkind, type, init, retain, at: Option<At>,
  probe, prov}` — the single field source for layout.
- Call graph: explicit edges; verified acyclic (05 §10).
- Invariants checked on entry: names resolved, types complete,
  conversions explicit, io_bindings derivable, no recursion.

## 3. State-blob layout algorithm

### 3.1 Field collection

Per program instance, gather into ordered field sets:

| set | contents | destination region |
|---|---|---|
| `globals` | GVL var fields (per program's used GVLs — flattened `GVL.var`) | global |
| `instances` | FB instance sub-objects (each = layout of FB type) | fb_instances |
| `progvars` | program-level VARs incl. AT shadows | global |
| `retains` | every field with `retain` flag (moved) | retain |
| `tasklocs` | per-task local storage (task-persistent scratch vars — rare; mostly empty) | task_local |
| `internals` | probes, bookkeeping (per-FB bookkeeping fields live inside their instance sub-object), inout-binding slots | internal |

AT-bound variables create **shadow fields** (in `progvars`/`instances`
where declared) + an `io_binding` descriptor (doc 02 §2.4) — the binding
is the only thing that touches the image.

### 3.2 FB type layout (recursive)

Each FB *type* gets a canonical struct layout:

```
struct Fb_<T> {
   iface fields in declaration order (VAR_INPUT, VAR_OUTPUT, VAR_IN_OUT
            is NOT stored — call-scoped binding slot in caller)
   internal VARs in declaration order
   rt bookkeeping fields (e.g., TON: last_in:bool, start_ns:i64)
   nested FB instance sub-objects (declared VAR of FB type)
}
```

### 3.3 Ordering and packing rules

- Region order fixed: `[global | fb_instances | task_local | retain |
  internal]` (retain last-but-one so its range is stable against
  non-retain churn; internal last).
- Within a region: **fields ordered by fully-qualified name** (not
  declaration order!) — the canonical sort makes layout identical for
  semantically identical inputs regardless of source formatting or
  decl-order wobble, maximizing the layout-hash fast-path hit rate (doc
  02 §7). Wait — reconsider: declaration order is what IEC users reason
  about and what makes "add a var at end" predictable; name-sort can
  reshuffle on rename (rename = migration anyway). **Decision:
  canonical order = (region, qualified-name)**. Rationale: migration
  matches fields *by name*; sorted layout makes "did anything move" a
  pure function of content, and padding differences identical across
  builds. Decl-order would create layout diffs from harmless reordering.
- Alignment: natural per type, ≤8 B; arrays pack elements; STRING(n) =
  `u8 len + u8 data[n]` (n+1 rounded to 2).
- Padding: explicit zeroed, included in hash.
- `layout_desc` emission + `layout_hash` per doc 01 §7 — the hash covers
  the resolved descriptor bytes.
- Total size → manifest `requires.state_blob_bytes` + descriptor.

### 3.4 Addressing in generated code

- No absolute pointers anywhere: code computes `base = ctx->state_blob`
  per entry; field access = `base + CONST_OFFSET` — the offsets are
  baked-in constants (this is what "compiled" means here: no symbol
  lookups at run time).
- `GVL.var` access = same mechanism (GVLs live in the program's blob —
  v1: each program instance carries its *own copy*? — **critical
  decision:** IEC GVLs are resource-global, shared between POUs/tasks of
  a resource. With multi-program instances, a shared GVL between two
  PROGRAMS is shared state across program instances. v1 rule: **GVLs
  are per-program-instance copies** — declared global per program
  namespace. True cross-program shared data comes later via
  `VAR_ACCESS`-like mechanism or shared marker area; documented
  deviation (inter-program sharing needs a real design — the honest
  answer is: don't fake it). Programs of the same task are still
  independent instances.
- FB instance call: `this_ptr = blob_base + inst_offset` passed to the
  FB's generated function.

## 4. LLVM lowering rules

### 4.1 Module shape

- One LLVM module per program package: `tplc_init`, `tplc_shutdown`,
  `tplc_task_run` (dispatch), per-task entry fns `__tplc_task_<name>`,
  per-POU fns `__tplc_pou_<qual>`, `tplc_migrate`, `tplc_get_descriptor`
  + `tplc_abi_version` const-returning fns, descriptor data in
  custom-named rodata sections (`tplc.desc`, `tplc.symref`? — symmap
  itself ships as package section, not ELF — but descriptor needs
  io_bindings etc. as compiled tables: those DO live in `.rodata`).
- Symbols: only ABI exports `default` visibility; everything else
  `internal`/`private` — enforced by emission + verified post-link
  (doc 01 §5).
- No `llvm.global_ctors`, no init/fini sections, no TLS, no comdat-needed
  constructs; tplc-rt must comply equally.

### 4.2 Statements/expressions

- Locals: SSA temporaries; `alloca` allowed **only in entry block, fixed
  size** (mem2reg-friendly; dynamic alloca = bug).
- Loops: structured CFG per source; **back-edge checkpoint**: at each
  loop latch emit `call void @__tplc_ckpt(ptr %ctx)` — the intrinsic
  lives in tplc-rt, reads abort/budget flags in ctx, on trip emits a
  synthesized early-return path (error sentinel unwind — implemented as
  a per-function `ckpt_failed` flag checked by callers? — **decision:**
  `__tplc_ckpt` returns `i1 abort`; callers test + propagate a
  `TPLC_TASK_E_*` return; codegen wraps: `if ckpt() ret CODE`. Adds one
  predictable branch per back-edge, ~1 cycle amortized).
- Call-boundary checkpoint optional via `checks.call_ckpt` (cost note).
- Expressions: strict ordering per §05; no UB-producing ops: `sdiv`→
  checked helper `__tplc_div_*` (returns fault), `urem/srem` same;
  float: `fdiv` fine (IEEE); shifts masked semantics per IEC (`SHL` via
  rt fn, not raw shl — IEC shift counts ≥ width are defined by stdlib,
  not LLVM poison).
- Strings: rt functions on (len,cap,data) layout; bounds-checked;
  overflow → fault in checked mode, truncate+flag in default? — per 05
  §11: string overflow → fault (checked), default mode truncates with
  `string_overflow` diag counter (documented deviation note).
- Enums → their base int with validity check in checked mode.
- FB call sequence: evaluate arg exprs (source order) → copy into
  iface-input fields → bind inout offsets (call-frame table, not blob)
  → `call @__tplc_pou_<T>(ctx, this)` → outputs read via this→fields.

### 4.3 Checkpoints & faults

- `__tplc_ckpt(ctx) -> i1` checks: `abort_flag` (atomic), elapsed vs
  `budget_deadline` (ctx field set by host before call — reads
  `svc->steady_ns` only in slow path? — **decision:** host stamps
  `deadline_ns` in taskctx; ckpt compares `ctx->now_ns`-refreshed? now_ns
  is per-entry stale; ckpt needs real clock → calls `svc->steady_ns()`
  (host fn, ~20ns) only when the cheap flag path doesn't fire — i.e.
  budget checks read clock at every Nth checkpoint or when flag set.
  Keep simple v1: ckpt checks abort flag + calls steady_ns always?
  steady_ns every back-edge at 4kHz loops could be hot; make it every
  64th invocation via per-task counter → bounded latency 64 iterations.
  Spec: `ckpt` = abort-flag check every call; clock read every 64 calls.)
- Fault returns propagate up: every call site checks POU return for
  `<0`? — overhead concern: make faults **rare-path**: POU fns return
  `i32` where only nonzero on fault; caller branches to cold block.
  Predictable cost.

### 4.4 Debug info / provenance

- Emit `DICompileUnit` per module; `DISubprogram` per POU with source
  file/line from provenance (ST spans direct; FBD nodes map to a
  synthesized "file" `fbd://<pou>/node/<id>` — the symmap is the real
  resolver; DI gives gdb-level utility).
- Variables: `dbg.value`/`dbg.declare` against blob addresses — DWARF
  expressions `blob_base + off` via `DIExpression` on the ctx pointer.
  Practical limit acknowledged: blob fields aren't real allocas; DI
  completeness is best-effort — **symmap is the authoritative debug
  surface**; DI serves stack traces + future gdb flows (doc 12).
- `debug_line` package section mirrors the same provenance compactly.

### 4.5 Probes and non-eliminable state

- Probe stores (doc 06 §4.3) are ordinary blob stores — naturally
  non-eliminated (address escapes via ctx). Same mechanism means **all
  blob writes survive optimization** — observable state is safe by
  construction (the blob is externally visible memory to LLVM).
- io_binding shadows likewise — copies happen in host code anyway.

### 4.6 `tplc_migrate` generation

When compiling with a predecessor `layout_desc` (doc 01 §8):

1. Diff old vs new field tables by qualified name:
   - name+type identical → `copy` (memcpy of field range);
   - name match, type changed → `transform` if a defined transform
     exists (INT→DINT widening etc., table-driven) else `deny`;
   - new only → `default-init` (emit init code);
   - old only → `dropped` (report; retain-loss alarm if retain field).
   - FB instance type changed under same name → deep-compare sub-type;
     changed ⇒ deny (state can't be reinterpreted) unless trivially
     compatible rule defined.
   - **moved offsets don't matter** — migration is by name, not offset:
     fields may reorder freely between versions and still migrate.
2. Emit `tplc_migrate`: iterates embedded rule table (also the
   `migrate_desc` section) — generated code performs copies via
   old-layout offsets (read from embedded old layout) → new-layout
   offsets (compiled constants). Deny-rules → `out_status=DENY`.
3. Post-pass: run `default-init` for all `internal` bookkeeping and any
   untouched fields → blob exactly as init+migrated (doc 02 §4.4).
4. Rule table + old layout embedded both as `migrate_desc` section AND
   as module rodata (self-contained execution; section serves tooling).

### 4.7 Optimization policy

- `-O2` pipeline curated: passes that are semantics-safe only;
  `fast-math`/`reassoc` OFF; `strictfp` default; vectorization allowed
  (deterministic); `mergefunc` OFF (breaks per-POU identity in DI);
  `globalopt` restricted from touching exported/ctx-linked data.
- LTO between program module and tplc-rt module: yes (stdlib inlines
  into calls — big perf win for TON/edge ops).
- Reproducibility: deterministic LLVM emission (stable iteration
  order everywhere in codegen — sorted BTreeMaps internally), `-frandom-seed`
  equivalent control via explicit naming (no hash-order naming);
  verified by doc 01 §13 bit-diff CI.

## 5. `tplc-rt` — the clean-room runtime library

### 5.1 What it is

- Rust crate compiled to bitcode, `no_std`-style (no allocator, no panic
  beyond handler, no std io), `extern "C"` internal API, linked into
  every program module pre-optimization (LTO).
- Contains: IEC standard functions + FBs, intrinsic helpers
  (`__tplc_ckpt`, `__tplc_div_*`, string ops, shifts, casts, TIME
  arithmetic, math funcs), the panic/fault path, and descriptor/diag
  glue.
- Clean-room rules (G12 r3): written from IEC semantics + oracle
  observations only; OSCAT/ruSTy-stdlib/matiec-output may inform
  *expected behavior* but no code is copied or transliterated — a
  `CLEANROOM.md` log records sources consulted per function (audit
  trail for licensing hygiene).

### 5.2 v1 surface (conformance list — each with semantics table in
as-built doc)

| group | entries |
|---|---|
| edge | `R_TRIG F_TRIG` |
| bistable | `SR RS` `SEMA` (basic) |
| timers | `TON TOF TP` (TIME inputs; state fields incl. running/start) — waveforms pinned by oracle tests |
| counters | `CTU CTD CTUD` |
| math | `ABS SQRT LN LOG EXP SIN COS TAN ASIN ACOS ATAN ATAN2 EXPT` `MIN MAX LIMIT MUX SEL` `MOD SHL SHR ROL ROR` `TRUNC ROUND FLOOR CEIL` |
| convert | `*_TO_*` full matrix elementary↔elementary + string parse/format |
| string | `LEN LEFT RIGHT MID CONCAT INSERT DELETE REPLACE FIND` (bounded, fault-on-overflow per §4.2) |
| time | `ADD/SUB/MUL/DIV` on TIME; `NOW`?? — clock access goes via ctx (`svc->steady_ns`); FBs needing time read `ctx->now_ns` — no syscall |
| compare | `GE NE EQ LE LT GT` (overloads incl. STRING/TIME) |
| sel/move | `MOVE` semantics as assignment |
| numeric | derived `MIN3?`—no: strict list |

Each FB's **blob-state struct** is part of its spec (it's user-visible
state — ET outputs, accumulated counts — and migration-visible).

### 5.3 Testing

- Unit semantics in Rust tests (pure logic).
- End-to-end: compiled ST calling each entry vs oracle vectors (matiec)
  and vs hand-derived IEC waveforms (timers esp. — the classic
  off-by-one domain).
- Property tests: monotonicity, edge invariants, reset idempotence.

## 6. Emitted artifacts per package

| output | doc | notes |
|---|---|---|
| `module.bc` | 01 §4 | merged program+rt module |
| `target .so` | 01 §5 | per triple; abi exports only |
| `symmap`/`type_catalog` | 01 §6 | from layout+provenance |
| `layout_desc`+hash | 01 §7 | the migrate contract |
| `migrate_desc`+code | §4.6 | online-change only |
| `io_bindings` table | 02 §2.4 | in descriptor rodata |
| `debug_line` | 01 §10 | if .so stripped |

## 7. Reproducibility

- All emission orderings sorted; no hashmap iteration into output.
- `now_ns`-independent: bitcode contains no timestamps (build-id off or
  content-derived).
- CI: two clean builds diff `.bc`/`.so` modulo package_uuid (doc 01 §13).

## 8. Iteration-2 hooks

- SFC: steps/transitions lower to mir like FB calls → same codegen;
  SFC chart state = blob fields (active step bitmap etc.).
- LD: no codegen change (FBD lowering covers).
- Multi-thread tasks: codegen already thread-agnostic (ctx-carried
  state); `EXT_MT_TASKS` only needs scheduler proof.
- MCU: lowering already PIC-clean; MCU codegen = different object
  emission + no dynamic relocs (doc 18).

## 9. Error codes (owned by this chapter)

`TPLC_E_MIR_INVALID, _CG_UNSUPPORTED, _CG_LAYOUT, _CG_MIGRATE_GEN,
_CG_EMIT, _RT_SEMANTIC` (+ frontend-owned upstream codes pass through)

## 10. Acceptance criteria

1. Corpus: every conforming ST + FBD reference program compiles to a
   doc-01-valid package; negative cases yield assigned codes.
2. Layout: golden `layout_desc` fixtures; hash-stability across
   machines/platforms; name-order insensitivity test (reordered decls →
   same hash when same fields).
3. Migrate: synthetic old/new pairs cover copy/transform/deny/default
   paths; DENY on FB-type-change.
4. Checkpoints: instrumented loops abort within bound (doc 04 test);
   non-instrumented straight-line adds ≤1% overhead measured.
5. tplc-rt: oracle-diff suite green; waveform golden tests for timers;
   fuzz on string ops (no UB under ASan/UBSan/Miri-on-rt).
6. DI: objdump --dwarf on package .so shows POUs/lines; symmap
   cross-checks every blob field.
7. Bit-identical double-build on x86_64 + aarch64 (CI gate).
8. No `global_ctors`/TLS/hidden writable state in emitted `.so`
   (loader checks pass).

---

## Appendix C — `tplc-rt` semantics tables (clean-room spec)

Normative semantics for the v1 function-block/function surface. These
tables *are* the spec — implemented clean-room per G12 r3 and verified
against oracle vectors (doc 14). Time is `i64` ns unless noted. All FB
state below lists the **blob-resident fields** (visible to symmap,
debug, and migration).

### C.1 Timers

#### TON — on-delay

```
interface: IN:BOOL PT:TIME → Q:BOOL ET:TIME
state:     { in_prev:BOOL, running:BOOL, start_ns:TIME }
```

Semantics per cycle:

```
if IN && !running:              running:=true; start_ns:=now
if !IN:                         running:=false; ET:=0; Q:=false
if running:                     ET:=min(now-start_ns, PT); Q:=(ET>=PT)
```

- Falling IN before PT: elapsed lost, Q never sets. Rising IN after
  Q: stays (running holds; ET clamps at PT — documented: IEC leaves ET
  behavior past PT impl-defined; we clamp).
- PT change mid-run: evaluated against *current* start (Q flips
  accordingly — shrinking PT can set Q immediately).
- `ET` reads `now-start_ns` even after Q (no, clamp per above — pick
  ONE rule: **clamp at PT**, consistent with Codesys).
- Waveform test vectors: {IN=0→1, ET≥PT, IN=1→0 mid, IN toggles,
  PT=0 → Q follows IN}.

#### TOF — off-delay

```
state: { out_prev:BOOL, off_running:BOOL, off_start_ns:TIME }
if IN:                          Q:=true; off_running:=false; ET:=0
if !IN && !off_running && Q:    off_running:=true; off_start_ns:=now
if off_running:                 ET:=min(now-off_start,PT);
                                Q:=(ET<PT); if ET>=PT: off_running:=false
```

#### TP — pulse

```
if IN rising && !running: running:=true; start_ns:=now; Q:=true
if running:               ET:=min(now-start,PT); if ET>=PT: Q:=false;running:=false
                          (IN during run: ignored — TP is non-retriggerable;
                           retriggerable variant TPR is an extension, v1.5)
```

#### TONR (retentive on-delay) — v1.5 placeholder, reserved signature.

### C.2 Edge detectors

```
R_TRIG: CLK:BOOL → Q:BOOL ; state{clk_prev}
   Q := CLK && !clk_prev ; clk_prev := CLK     (Q true exactly one cycle)
F_TRIG: mirror on falling edge.
```

Q is computed-then-updated (no output carry between cycles).

### C.3 Bistables

```
SR:  S1:BOOL R:BOOL → Q1:BOOL   (set-dominant)
     if S1: Q1:=true elsif R: Q1:=false     — set wins when both
RS:  S:BOOL R1:BOOL → Q1:BOOL   (reset-dominant)
     if R1: Q1:=false elsif S: Q1:=true
SEMA: X:BOOL → Q:BOOL,CLAIMER:… — v1.5 (multitask semaphores need MT model)
```

### C.4 Counters

```
CTU: CU:BOOL R:BOOL PV:INT → Q:BOOL CV:INT
  state{cu_prev}  if R: CV:=0; Q:=false
                  elsif CU rising: CV:=min(CV+1, PV_MAX? — INT max)
                  Q := CV >= PV      (CV keeps counting past PV)
CTD: CD:BOOL LD:BOOL PV:INT → Q:BOOL CV:INT
                  if LD: CV:=PV; Q:=(CV<=0)
                  elsif CD rising: CV:=max(CV-1, INT_MIN)
                  Q := CV <= 0
CTUD: CU,CD,R,LD,PV → QU,QD,CV   (both behaviors merged)
```

- `PV` change: takes effect next edge (no re-eval of Q until edge or
  R/LD). Overflow clamps (saturating, not wrapping — documented choice
  vs IEC-undefined; checked-mode can fault instead).

### C.5 Selection/comparison/math functions

`MUX(K, in0..inN)`, `SEL(G, in0, in1)`, `MIN/MAX(ANY_NUM list)`,
`LIMIT(MN, IN, MX)` = `MIN(MAX(IN,MN),MX)`; `ABS`, `SQRT`(x<0 → fault
RANGE), `LN/LOG`(x≤0 → fault), `EXPT`, trig `SIN COS TAN ASIN ACOS
ATAN ATAN2`; comparisons over all ordered types incl. STRING
(lexicographic byte order — documented: locale-free) and TIME.

### C.6 Shift/rotate (on bitstring family)

`SHL(IN, N)`/`SHR`: logical, vacated bits 0; `N ≥ width` → 0
(IEC-defined, no poison); `N<0` → fault RANGE. `ROL/ROR`: circular,
`N mod width`.

### C.7 String functions (bounded, layout `u8 len; u8 s[cap]`)

`LEN`, `LEFT(IN,L)`, `RIGHT`, `MID(IN,L,P)` (1-based P; P>len→empty),
`CONCAT`, `INSERT(IN1,IN2,P)`, `DELETE(IN,L,P)`, `REPLACE(IN1,IN2,L,P)`,
`FIND(IN1,IN2)` (0 = not found). Overflow of destination capacity:
checked-mode → `RANGE` fault; default → truncate + `string_overflow`
diag counter (doc 05 §11). Parameters out of range (L<0, P<1) →
`RANGE` in all modes.

### C.8 Conversion functions `X_TO_Y`

Per Appendix-B matrix cells marked `C`/`✓` (✓ = implicit, no call
needed; C = explicit function exists). String↔numeric uses strict
parse (fail → `RANGE` fault or 0+flag? — **decision:** strict, fault —
silent-0 parsing corrupts machine state invisibly) and IEC-ish format
(e.g. `INT_TO_STRING` → no leading spaces — deviation vs some impls;
documented). `REAL→int` = round-half-away (`ROUND` semantics;
`TRUNC` for truncation), range-checked.

### C.9 TIME arithmetic

`ADD/SUB` (TIME±TIME→TIME; TIME+negative clamps at
`INT64_MIN/MAX`? — clamp+diag), `MUL/DIV` (TIME×int|real → TIME),
`DIV` (TIME/TIME → REAL). `DATE/TOD/DT` ops via stdlib (calendar math
in rt, proleptic Gregorian, no TZ — machine-time is monotonic ns since
epoch; display formatting is a service-side concern).

### C.10 Internal helpers (not user-visible)

`__tplc_ckpt` (§4.3), `__tplc_div_s*`/`_u*`/`_mod_*` (fault-on-zero),
`__tplc_memcpy_checked`, `__tplc_str_*` ops, panic handler →
`TPLC_TASK_E_PANIC`, `__tplc_saturate_*` for checked casts.
