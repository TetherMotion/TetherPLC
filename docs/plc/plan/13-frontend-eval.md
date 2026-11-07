# 13 — Frontend Strategy Evaluation: ruSTy Adoption Decision

> **Status:** work-item document; deleted when the decision is executed
> (spikes done, path chosen, recorded in `docs/plc/DECISIONS.md` and the
> chosen path's plan chapters updated).
>
> **Satisfies:** D9 (this chapter IS the evaluated decision), G4/G8/G10
> (requirements the eval weighs), D3 (Rust), G12 (licensing).
> **Depends on:** 00–04. **Gates:** implementation choices inside 05, 07.
> **Does not gate:** 01–04, 06, 08 (their specs are inputs to the eval).

---

## 1. The decision

How does structured text get from source to our canonical IR? Three
candidate strategies, evaluated against the hard requirements that are
already locked (not re-opened here).

**Already fixed (inputs to the eval):**

- Compiler implemented in Rust (D3); produced artifact = `.tplc` package
  with bitcode + `.so` (D2); runtime ABI from doc 02.
- Online change in v1 (D5): total control of the **state-blob layout**
  plus generated `tplc_migrate` — the codegen *is* the product's core IP.
- Debug info must support FBD-node provenance (G8, doc 11's live overlay),
  not just ST lines.
- Function-block library is clean-room regardless of source license
  (G12 rule 3) — stdlib reuse is excluded from every option.
- GPL tools are oracle-only behind adapters (G12 rule 2) — matiec is
  therefore never a candidate for embedding; it appears only as §7's
  conformance oracle.

## 2. The options

### Option A — Adopt ruSTy wholesale

Use [ruSTy](https://github.com/PLC-lang/rusty) (MIT/Apache-2.0) as the ST
compiler end-to-end: its parser, semantic analysis, LLVM codegen, and
object emission; we wrap its output in our packaging/ABI glue.

### Option B — Reuse ruSTy's frontend, own the backend

Consume ruSTy as a library: its lexer/parser/AST and (optionally)
semantic-analysis/validation passes, then run **our own codegen** — our
IR emission, our state-blob model, our symmap/layout/migrate generation,
our checkpoints. FBD lowers to the same AST or a shared POU-level IR.

### Option C — Fully own frontend

Write our own ST frontend (lexer → parser → typed AST → sema) plus all
codegen. ruSTy remains a *design reference* only, alongside the IEC spec
and the matiec oracle.

## 3. What ruSTy actually is (facts, not vibes)

Verified against the project at eval time; re-verify versions at spike:

- Parses a large IEC 61131-3 ST subset: programs, function blocks,
  functions, methods/actions, VAR blocks (incl. VAR_GLOBAL/EXTERNAL),
  rich elementary + derived types (structs, enums, arrays, STRING,
  subranges, references/pointers), control flow, calls, initializers,
  properties/interfaces (OOP extensions beyond strict IEC).
- Pipeline: lexer/parser → AST → **annotation/indexing + validation**
  passes → LLVM (inkwell) codegen → object files; links into native
  binaries/libs. Emits **DWARF debug info** (line-level).
- Its compiled state lives in **module-owned storage** (global/section
  variables laid out by its own codegen) — i.e., the classic compiler
  memory model, *not* a host-owned relocatable blob.
- Has an ST-authored standard function/FB library compiled by itself.
- Active development (originated at Bachmann), community traction,
  reasonably clean internal structure — but it is a *compiler binary*
  project first; use-as-library of individual passes is possible yet not
  its primary design contract.
- No online change, no state migration, no PLC-image indirection, no
  FBD/LD/SFC — by design, it's a compiler, not a runtime ecosystem.

### 3.1 Why Option A is (probably) disqualified by G4

The conflict is structural, not a patch-level gap:

- ruSTy lays out program state in **module-owned memory** — exactly what
  our ABI forbids (doc 01 §5 hidden-state check, doc 02 §3). Blob
  relocation, memcpy-migration, retain sub-ranges, per-task shadow
  bindings: all impossible without rewriting its storage model.
- `tplc_migrate` requires codegen that *knows both layouts* — that means
  emitting our own storage addressing everywhere, not a post-pass.
- FBD-node provenance debug info needs custom `DI` emission keyed to
  diagram nodes — its line-based DWARF doesn't cover it.
- Our checkpoint insertion (doc 04 §2.1) needs loop-back-edge
  instrumentation at *our* IR level.
- Verdict: adopting wholesale forces a codegen fork anyway → Option A
  collapses into Option B with extra constraints. **A is eliminated
  unless the spike disproves the structural claim** (recorded as an
  explicit falsification criterion in §7).

## 4. Requirements matrix

| requirement (source) | A: wholesale | B: frontend reuse | C: own frontend |
|---|---|---|---|
| G4 state-blob layout control | ✗ (structural) | ✓ our codegen | ✓ |
| G4 migrate codegen | ✗ | ✓ | ✓ |
| G8 FBD-provenance debug info | ✗ line-only | ✓ we emit DI | ✓ |
| G7/01 canonical-IR packaging | ~ needs wrapping | ✓ native | ✓ native |
| D8 FBD→same IR | via FBD→ST text (workable) | ✓ shared AST/lowering | ✓ shared lowering |
| Doc 04 checkpoints | post-pass hacks | ✓ | ✓ |
| Diagnostics quality (product) | theirs — decent, fixed | theirs for parse/sema; ours in codegen | fully ours, must be *written* |
| IEC conformance breadth day-1 | high (mature subset) | high | grows over time |
| Effort to first compile | weeks | 1–2 quarters est. | 3–4+ quarters est. |
| Maintenance/upgrade coupling | upstream-bound | pinned/forked AST layer | none external |
| AST consumability risk | n/a | **the open question** | n/a |
| Team must deeply understand frontend | no | yes (backend boundary) | yes (everything) |
| License | MIT/Apache OK | MIT/Apache OK | n/a |

## 5. Analysis of the real choice: B vs C

Since A collapses into B, the question is **where the boundary sits**:

**For B:**

- IEC parsing+sema is the long, unglamorous, *already-solved* part:
  grammar edge cases, implicit conversions, overload resolution, IEC's
  baroque VAR initializers — ruSTy already paid that tax. Our
  differentiation is the *backend* (blob/migrate/ABI/checkpoints), so
  effort spent re-deriving their parser is opportunity cost with zero
  product return.
- Fork risk is bounded: AST consumption at a pinned version is a stable
  internal API for us; upstream drift only matters if we want upstream
  fixes — acceptable.
- B preserves optionality: if ruSTy's AST proves unusable, the spike
  tells us cheaply and C is the fallback — nothing built is wasted
  (the backend is identical under both).

**Against B / for C:**

- ruSTy's AST encodes *its* semantics decisions (type resolution, FB
  representation, address-of model) designed for module-owned storage;
  some impedance mismatch is guaranteed at the boundary — e.g. where its
  sema resolves a global to a symbol, we need it resolved to a
  blob-offset declaration. The "seam" may be upstream of where we'd
  like it.
- Their sema/validation may reject or transform constructs in ways that
  fight our provenance tracking (desugared nodes lose source identity).
- Dependency weight: pulling a compiler as a library pulls its whole
  dep tree.
- C's hidden benefit: a frontend written against *our* POU-level IR
  (§6) is the natural convergence point for ST **and** FBD **and** later
  SFC/LD — one typed middle layer designed for our storage model, rather
  than an AST shaped by someone else's.

**Recommended path: B-with-conditions → fallback C.**

1. Spike B (§7). If ruSTy's parser+AST (and optionally sema) can be
   consumed with a clean boundary — adopt it *as the ST parser only*,
   feeding our own **POU-level typed IR** (the layer where sema results
   land). We re-implement sema/type-checking *on* that IR in any case
   where their analysis is entangled with storage decisions — i.e. use
   ruSTy for **front-half** (lex/parse/AST-build), not necessarily for
   analysis. This "half-B" is the expected landing zone: parsing is
   where the effort is; storage-coupled analysis is where their model
   diverges.
2. If the spike shows the AST is not consumable without surgery that
   rivals writing a parser — go C. The ST grammar is big but not exotic;
   a hand-written recursive-descent + own sema is very achievable in
   Rust and gives the best long-term diagnostics control. Documented
   decision checkpoint, not a failure.
3. In **both** outcomes the backend is ours and identical: IR →
   blob-layout → codegen → symmap/layout_desc/migrate. That is the
   design's real invariant; the frontend choice is cost management.

## 6. The piece we own in every option: POU-level IR

Define `tplc-mir` (working name): a typed, source-provenanced
intermediate representation **below** ST/FBD frontends and **above** LLVM
lowering:

- Nodes: POU (program/FB/function) declarations, variables with full IEC
  types, expressions/statements at near-source fidelity (or explicit
  desugar with provenance edges), call/FB-instantiation graph, task
  bindings.
- Provenance: every node carries `{source_span | fbd_node_id | generated}`
  — the same identity space symmap/debug_line use.
- Consumers: our sema (name resolution, type check, IEC conversion rules,
  overload resolution), layout computation (doc 07), migrate generation
  (doc 08), diagnostics.
- Producers: ST parser (option B or C), FBD lowering (doc 06 — FBD
  lowers to `tplc-mir` directly, NOT to ST text — decision recorded here:
  Beremiz's FBD→text trick was for a text-only compiler; we have a real
  middle layer, so FBD→mir is the clean path and preserves node
  provenance exactly).

`tplc-mir` is what makes "B vs C" a localized decision and what makes
G10 (SFC/LD later) cheap: they also produce `tplc-mir`.

## 7. Spike plan (the executable part of this chapter)

Timeboxed spikes, each producing a written verdict in DECISIONS.md:

| # | spike | pass/fail criterion | box |
|---|---|---|---|
| S1 | Consume ruSTy as a library: parse a representative ST corpus, walk its AST programmatically | AST fully traversable with source locations, no private internals needed | 2 wks |
| S2 | Boundary test: can its AST be converted to `tplc-mir` without semantic re-derivation? (types, FB decls, initializers) | Lossless mapping incl. provenance for ≥90% of IEC core constructs | inside S1 |
| S3 | Storage-model interference: does its sema/validation bake in storage assumptions that would *misvalidate* our model? | No blocking entanglement, or isolatable | inside S1 |
| S4 | Falsification check for A: confirm module-owned-state claim (§3.1) by locating its global storage codegen | A formally eliminated (expected) | days |
| S5 | Effort recalibration: implement one non-trivial POU end-to-end through the B path | Confirms B is materially cheaper than C estimate | 1 wk |
| S6 (parallel, small) | C-floor check: prototype recursive-descent ST lexer+parser for expression+assignment+FB-decl subset | Establishes real cost of the fallback | 1 wk |

Oracle hookup (independent of choice): the matiec oracle adapter
(doc 14) provides expected-behavior vectors for whichever frontend ships.

## 8. What is ours regardless of A/B/C

Non-negotiable own-IP list (the backend): `tplc-mir`, IEC type system
realization → layout, blob layout + `layout_desc`/`layout_hash`, IR→LLVM
lowering, `tplc_migrate` generation, symmap/type_catalog/provenance,
checkpoint insertion, `tplc-rt` clean-room stdlib (IR-embeddable),
package writer, diagnostics product layer. Option A was the only path
threatening this list — eliminated for exactly that reason.

## 9. Risk register

| risk | mitigation |
|---|---|
| ruSTy API churn on pin | pin version; wrap consumption behind `tplc-rusty-adapter` crate; own MIR boundary |
| hidden sema/storage entanglement (B degrades toward C) | S3 measures it; fallback is planned, not improvised |
| diagnostics quality below Codesys-bar either way | product layer owns diagnostic UX; oracle tests catch semantic drift |
| underestimating IEC edge cases in C | C chosen only if B fails; matiec oracle + IEC annex test corpus de-risks |
| license hygiene | MIT/Apache attribution file; no code paths to GPL anywhere (G12) |

## 10. Decision record template (for DECISIONS.md when executed)

```
DEC-YYYY-NN  ST frontend strategy
  Chosen:    B / half-B / C   (+ pinned versions)
  Evidence:  spike results S1–S6 summaries
  Rejected:  alternatives + one-line reason each
  Consequences: updates to 05/07 chapters (mark sections active/inactive)
```

## 11. Acceptance criteria

1. S1–S6 executed, verdicts in DECISIONS.md; path chosen.
2. `tplc-mir` spec exists (as-built doc) and is the documented producer
   contract of ST and FBD frontends.
3. Plan chapters 05/07 updated: chosen path's sections marked active.
4. If B: adapter crate isolates ruSTy; if C: own-parser milestones added
   to doc 17.
5. No GPL anywhere in the compile path; license manifest generated.
