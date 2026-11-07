# 06 — FBD Model: Graph Semantics, `tplc-mir` Lowering, PLCopen XML

> **Status:** work-item document; deleted when `docs/plc/fbd-model.md`
> (as-built model + XML mapping spec) exists and the lowering pipeline
> passes conformance.
>
> **Satisfies:** D7/D8 (one IR; PLCopen XML), G9 (TC6 interchange), G10
> (LD/SFC preparedness), G8 (node provenance), rustylink-inspired model.
> **Depends on:** 00–02. **Informs:** 07 (lowering target), 11 (editor),
> 13 (`tplc-mir` boundary).

---

## 1. Scope

Defines the **language-level** semantics of TetherPLC function block
diagrams: what a diagram *means* independent of rendering (editor, doc 11)
and independent of machine code (lowering, doc 07). Three artifacts:

1. `fbd-model` — the in-memory/serialization data model (Rust crate +
   mirrored TS types in TetherWebUI).
2. `fbd-lower` — deterministic translation `fbd-model → tplc-mir`.
3. `plcopen-io` — PLCopen XML (TC6) read/write of `fbd-model`, incl.
   tether-namespaced extensions.

Non-goals: drawing/canvas UX (doc 11), SFC step/transition semantics
(iteration 2, hooks in §11), LD rendering (iteration 2).

## 2. The model

### 2.1 Document hierarchy

```
Project
 └─ POU (kind: program | function_block | function)
     ├─ interface (VAR/VAR_INPUT/VAR_OUTPUT/VAR_IN_OUT/VAR_EXTERNAL decls)
     ├─ body:
     │    └─ Network[]            // ordered; evaluation = sequential
     │         ├─ Block[]         // placed instances + functions/operators
     │         ├─ Variable[]      // standalone in/out variable elements
     │         ├─ Wire[]          // typed connections between pins
     │         ├─ Comment[]       // free text
     │         ├─ Label[] / Jump[]// control flow inside network (§5.4)
     │         └─ ReturnMarker[]  // <return> elements
     └─ (iter2: SFC body alternative)
```

`SubSystem` (hierarchical block containing its own networks — rustylink's
`System`) is **designed-for (flag in schema, implemented iter-2)**; v1
POUs are the encapsulation boundary already, so subsystem is sugar rather
than semantics.

### 2.2 Blocks

```
Block {
  id: NodeId                // stable u64 — THE provenance key (G8)
  type_ref: TypeRef         // FB type | function signature | operator
  kind: fb | function | operator | variable_block | jump_target
  instance_name: Option<Name>   // required for fb; else anonymous
  position: {x, y}          // f64 mm-ish canvas units; layout only
  inputs:  Port[]           // ordered per signature
  outputs: Port[]
  params: ParamAssignment[] // literal/customized parameters
  execution_order: u32      // explicit override; 0 = auto (§4.2)
  negated_inputs: Bitset    // IEC FBD negation markers (visual + semantic)
}
```

- **FB block**: stateful instance — gets a state-blob field named
  `<pou>.<instance_name>` (or synthesized name). Its *call* executes in
  the dataflow position determined by wiring/order.
- **Function/operator block**: stateless — lowers to expression; may be
  fully inlined (ADD, MUL, GT, SEL, MIN, logical ops, casts…).
- **Variable block**: IEC "inVariable/outVariable" — direct access to a
  variable or `%I/%Q` channel; lowers to load/store, not a call.
- Blocks may carry **enable (EN/ENO)** semantics per IEC: first input is
  implicit `EN`, first output `ENO`; config per POU ("EN/ENO shown") —
  EN=false ⇒ outputs hold previous (FBs) or pass-through rule for
  functions (defined: function outputs keep last value — deviation note
  vs IEC's undefined behavior: we *define* hold-last, safer for machines;
  documented in dialect spec).

### 2.3 Ports and wires

```
Port { index, name, dir: in|out|inout, type: TypeExpr, connected_wire }
Wire { id, net_id, type: inferred, src: PortRef, dsts: PortRef[] }
```

- Fan-out: unrestricted. Fan-in: exactly one `src` per wire net; a port
  accepts ≤1 wire (fan-in only via explicit blocks — MUX/SEL) — matches
  IEC and prevents implicit-or ambiguity.
- `inout` connections are reference pairs (both directions to the same
  wire net) — restricted: inout endpoints must resolve to a single
  variable storage (no expressions on inout wires).
- Type inference: net type = src port type; implicit conversion per IEC
  rules applied at dst insertion point (conversion blocks auto-materialize
  as `operator` casts with `generated` provenance — visible in debug).
- Wireless association: PLCopen allows `formalParameter` + `variable`
  association without drawn wire — supported in model (`assoc` edge kind),
  rendered as named stubs by the editor.

### 2.4 Execution semantics

**Inside a network**: dataflow order — a block executes when all inputs
are available; equivalently a topological sort of the DAG. **Ties are
resolved canonically** (§4.2) so order is deterministic and identical
across editor saves, compiles, and platforms (reproducibility, doc 01).

**Across networks**: strict sequential order — network[0] … network[n].
(IEC FBD defines evaluation "top to bottom"; our networks are explicit
and ordered, which is the same thing made structural.)

**Feedback**: a wire feeding an input upstream of its source's evaluation
order creates an algebraic path. Rules:

- Cycle through **no state element** ⇒ **compile error**
  (`TPLC_E_FBD_ALGEBRAIC_LOOP`) with the offending cycle rendered in
  diagnostics (node ids → editor highlights it — doc 11).
- Cycle through an FB instance or an explicit memory block (`DELAY`,
  `PREV`, FB state) ⇒ legal; semantics = previous-cycle value (the FB
  read happens at its graph position; its state update at call position).
- Explicit `execution_order` overrides may reorder independent blocks;
  they can never make an algebraic cycle legal.

This is deliberately stricter than Simulink's solver and deliberately
equal to the IEC mental model: the diagram is a notation for *ordered
evaluation*, not a continuous system.

## 3. Type system mapping

FBD uses the IEC type set verbatim (doc 05 owns the canonical list):
BOOL, SINT…LINT, USINT…ULINT, REAL/LREAL, TIME, DATE/TOD/DT,
BYTE…LWORD, STRING/WSTRING, arrays, structs, enums, FB types,
`ANY_*` families for generic blocks.

- Port typing is checked **at model level** (editor live-validates) and
  re-checked at lowering (compiler authority — the model file is
  untrusted input).
- Generic blocks (`ADD` over numerics etc.) resolve `ANY_*` per connected
  net at infer time; unresolved generic ⇒ error listing candidate pins.
- `BOOL`-edge special case: negated pin marker lowers to `NOT` inline —
  provenance keeps it as pin attribute, not extra node.

## 4. Lowering to `tplc-mir`

### 4.1 Order computation (deterministic)

Per network:

```
1. build dependency graph: block→block via wires (src→dst)
2. stable topo-sort: frontier ordered by (execution_order≠0 → explicit,
   then y, then x, then id)
3. algebraic-check via cycle detection on the same pass
4. emit mir statement sequence: each block → mir::FBCall | mir::Expr
   each wire → mir::Temp {type, provenance(wire_id)}
   variable blocks → mir::Load/Store
```

Networks lower in declared order into the POU's mir body. The result is
indistinguishable in *kind* from ST-authored mir — only provenance tags
differ (`fbd_node`/`fbd_wire` vs `st_span`) — satisfying D8.

### 4.2 Determinism guarantee

Same model ⇒ same mir bytes. The sort key is total (id tiebreak),
position-independent-of-float-precision (positions stored as integers in
model file; canvas floats are render-layer only — editor snaps to grid,
model stores grid units `u32`), and execution_order explicit when set.
Two editors saving the same logical diagram produce identical lowering.

### 4.3 Probe points (debug, doc 12)

Model may mark wires/ports `probe: true` (editor: "watch this wire").
Lowering allocates a blob field per probe (region `internal`, symmap
entry `kind=VAR`, provenance→wire id) and emits a store after value
production. Probes are opt-in (cost = stores + blob bytes) — the watch
overlay reads them like any variable. Default release builds strip
probes; debug builds keep all? — **decision:** probes preserved whenever
requested; a manifest flag `probes_stripped` records intent.

### 4.4 Labels, jumps, returns

IEC FBD permits labels/jumps (structured-goto) and return markers.
Support them at model+lowering level (mir has `Label/Goto/Return`), but
the editor discourages them (lint `fbd-no-jumps` default warn): they are
for imported PLCopen projects' fidelity, not for authored code. SFC will
revisit flow control properly.

### 4.5 EN/ENO semantics

Per POU config `en_eno`:
- `strict-iec`: ENO = EN of executed block; unexecuted FB outputs
  implementation-defined → we define *hold-last* (deviation documented).
- `relaxed` (default): EN/ENO pins exist only where user places them.

## 5. PLCopen XML (TC6) mapping

Serialization target: `TC6_0201` schema, `<POU><body><FBD>` elements.

| model | PLCopen element |
|---|---|
| Network | `<network>`? — TC6 has no explicit network element; networks map to document order within `<FBD>`; we emit `<comment>`-delimited? — **mapping decision:** vendor-ext `tether:network` element wrapping blocks/wires per network; strict readers see flattened FBD (still valid), round-trip preserves networks |
| Block | `<block typeName=".." instanceName=".." localId executionOrderId>` |
| Variable block | `<inVariable>/<outVariable>/<inOutVariable>` |
| Port pin wire | `<connectionPointIn>`/`<connectionPointOut>` + `<connection refLocalId>` |
| Wireless assoc | `<connection>` via `formalParameter`+expression form |
| Negation | `negated="true"` pin attribute |
| Position | `position` x/y attributes |
| Label/Jump/Return | `<label>`/`<jump>`/`<return>` |
| Comment | `<comment>` |
| Probe flag | `tether:probe` vendor attribute |
| Vendor ext | `tether:` namespace, all optional, stripped on strict export |

Two export modes: **strict** (TC6-only, for Codesys/Beremiz import —
extensions dropped, documented losses) and **roundtrip** (extensions in,
lossless for our editor). Import is lossy-tolerant: unknown vendor
namespaces skipped, unsupported constructs reported not guessed.

Import fidelity targets (acceptance §13): Beremiz-exported FBD imports
with ≥95% construct coverage (blocks, wires, vars, comments); gaps
reported as diagnostics, not silent drops.

## 6. The shared TS/Rust schema

`fbd-model` is authored as a **JSON-schema'd JSON document** (canonical
form) — the editor's native save format — with PLCopen XML as
import/export (D12 rule 1: XML is the *exchange* surface; JSON is the
*working* surface — cleaner for git, TS, and Rust).

- Rust: `serde` types + `schemars` derive → JSON Schema artifact.
- TS: generated types from the same schema (`quicktype`/hand bindings)
  so editor and compiler literally share a grammar.
- Stability: schema versioned (`fbd.v1`); migrations in
  `fbd-model::migrate`; forward-compat rule = unknown keys ignored,
  required-keys never removed.
- NodeId allocation: u64, monotonic per POU, never reused after delete
  (provenance durability across edits — symmap/diagnostics/breakpoints
  key on it; an undo-restore gets a NEW id, not resurrection of old).
- canonical form: sorted keys, LF endings, stable field order →
  byte-identical saves of identical diagrams (VC/diff friendly; ties
  into doc 01 determinism chain: model → mir → IR → package).

## 7. Validation rules (model-level, enforced twice: editor + compiler)

```
FBD001 type mismatch on wire endpoints (post-conversion check)
FBD002 unconnected required input (non-optional, no default)
FBD003 fan-in violation (2+ sources)
FBD004 algebraic cycle w/o state element        (hard error)
FBD005 inout non-resolvable storage
FBD006 unknown block type_ref / param name
FBD007 execution_order collision
FBD008 inVariable/outVariable references undeclared name
FBD009 generic ANY_* unresolved
FBD010 network empty / orphan blocks            (warning)
FBD011 jump to nonexistent label
FBD012 instance_name reuse in POU
FBD013 direct recursive FB instantiation        (hard error)
FBD014 inout chain through expression           (hard error)
```

Severity table + lint config in the doc's as-built successor; warnings
never block compile, errors always do.

## 8. Example (normative) — debounced start/stop with motor FB

```
net1: [%IX0.0]──►[TON T_Start(PT:=T#200ms)]──.Q──►[AND]──►(%QX0.0)
       [%IX0.1]─────────────────────────NOT──►[AND]
       T_Start instance state in blob; probe on AND.out
lowering → mir:
  FBCall(T_Start, IN:=load(%I[0.0]), PT:=200ms)
  t1 := AND(load(T_Start.Q), NOT load(%I[0.1]))      // wire t1 provenance→probe
  store(probe_t1, t1); store(%Q[0.0], t1)
```

(The doc's as-built version carries a gallery of 10+ such patterns —
motor, interlock, edge, counter, analog scale — each as model+JSON+XML+mir.)

## 9. Relationship to rustylink

Borrowed (concepts, Apache-2.0 so code *could* be borrowed too — but the
TS rewrite makes it moot): block/port/edge model shape, library-catalog
organization, net-vs-wire naming, editor interaction patterns (rubber-
band wiring, port snapping, subsystem drill-in). Not borrowed: slx/XML
schema (ours is TC6+JSON), egui UI, simulink semantics (sample times,
continuous blocks) — ours is scan-discrete only.

## 10. Block library surface (clean-room, G12 rule 3)

The *model* layer needs the catalog schema: `BlockType {type_ref,
signature(ports+params), category, doc}`. The catalog itself is compiler
property (doc 07 implements FBs; the catalog is generated FROM the
clean-room tplc-rt metadata so editor and compiler never drift — single
source: `tplc-rt` manifest exported as JSON for the editor palette).

Categories for v1 (clean-room scope): logic (AND/OR/XOR/NOT/SR/RS/
R_TRIG/F_TRIG), timers (TON/TOF/TP/TONR), counters (CTU/CTD/CTUD),
math/compare/select (ADD…,GT…,SEL,MUX,MIN,MAX,LIMIT,ABS), moves/casts,
bistable/edge, standard functions on scalars (doc 07 full list).
Iteration-2: SFC elements, PID/control, motion-coupled blocks.

## 11. Iteration-2 hooks

- **LD**: rung = Network with `kind:"ladder"`; contacts/coils/
  power-rail as block kinds over the same wire model; LD rendering is a
  view of constrained FBD. Model needs only: `network.kind` + rail
  pseudo-ports — reserved fields exist.
- **SFC**: alternative POU body `sfc:` steps/transitions/actions where
  action bodies are ST or these same FBD networks — zero changes here.
- **SubSystem**: `kind:"subsystem"` block + `inner: Network[]` field
  (schema field reserved, lowering = flatten with name mangling).

## 12. Error codes (owned by this chapter)

`TPLC_E_FBD_*` (FBD001–FBD014 surfaced as codes), `TPLC_E_XML_*
(import/export)`, `TPLC_E_MODEL_SCHEMA`

## 13. Acceptance criteria

1. `fbd-model` (Rust) + generated TS types round-trip JSON↔model
   lossless; canonical-save byte-stability test.
2. `plcopen-io`: strict + roundtrip modes; Beremiz corpus import ≥95%
   coverage; exported strict XML validates against TC6 schema (xsd).
3. `fbd-lower` produces identical `tplc-mir` for identical models on all
   platforms; topo-order golden tests incl. explicit execution_order.
4. All FBD001–FBD014 verified by fixture cases; algebraic-loop error
   renders node-id cycle.
5. Probe wiring end-to-end: probed wire value observable via symmap on a
   compiled package (integration with 07/12).
6. Equivalence evidence: for the 10-pattern gallery, FBD-lowered and
   hand-written-ST mir are semantically equal (oracle diff on IO traces).

---

## Appendix D — `fbd.v1` JSON schema (normative skeleton)

```jsonc
{ "$schema": "https://tether.dev/schemas/fbd.v1.json",
  "schema": "fbd.v1",
  "project": "fill_station",
  "pous": [{
    "id": "pou_fill", "name": "FILL_CTL", "kind": "program",
    "interface": [
      {"name":"speed","kind":"var_input","type":"INT","init":"100"},
      {"name":"done","kind":"var_output","type":"BOOL"}
    ],
    "networks": [{
      "id": "net_1", "kind": "fbd", "label": "start logic",
      "blocks": [{
        "id": 12, "kind": "fb", "type_ref": "TON",
        "instance": "T_Start", "pos": {"x": 8, "y": 4},
        "params": {"PT": "T#200ms"}, "exec_order": 0,
        "negated": [], "comment": ""
      },{
        "id": 13, "kind": "variable", "role": "in",
        "name": "StartBtn", "at": "%IX0.0", "pos": {"x": 1, "y": 4}
      },{
        "id": 14, "kind": "operator", "type_ref": "AND",
        "pos": {"x": 20, "y": 6}
      },{
        "id": 15, "kind": "variable", "role": "out",
        "name": "MotorRun", "at": "%QX0.0", "pos": {"x": 30, "y": 6}
      }],
      "wires": [
        {"id": 40, "src": {"block": 13, "port": 0},
         "dsts": [{"block": 12, "port": 0}]},
        {"id": 41, "src": {"block": 12, "port": "Q"},
         "dsts": [{"block": 14, "port": 0}], "probe": true},
        {"id": 42, "src": {"block": 14, "port": 0},
         "dsts": [{"block": 15, "port": 0}]},
        {"id": 43, "src": {"block": 99, "port": 0},
         "dsts": [{"block": 14, "port": 1}]}
      ],
      "comments": [{"id":50,"pos":{"x":2,"y":0},"text":"debounce"}],
      "labels":   [{"id":60,"pos":{"x":25,"y":2},"name":"L1"}],
      "jumps":    [{"id":61,"pos":{"x":26,"y":8},"target":"L1"}]
    }]
  }],
  "tasks": [{"name":"MAIN","period_ns":1000000,"programs":["pou_fill"]}]
}
```

Field notes:

- `port` references: `u32` index or `"name"` (FB pin names); indexes
  are signature order.
- `variable` blocks: `role: in|out|inout`, `name` resolves against
  POU interface/GVL; `at` optional direct address.
- `assoc` edges (wireless): `{"assoc": {"src_name": "...", "dst":
  {...}}}` alternative to `wires`.
- `kind:"ladder"` networks reserve: `rails`, `contacts`, `coils`
  fields (iter-2, absent in v1 files — schema ignores unknown keys
  per forward-compat rule).

## Appendix E — PLCopen TC6 mapping table (complete)

| fbd.v1 construct | TC6 XML | strict-mode handling |
|---|---|---|
| pou | `<pou name type>` + `<interface>` + `<body>` | direct |
| network | `tether:network` wrapper; strict = flatten w/ leading `<comment>` `--- net N ---` | degrade w/ marker |
| fb block | `<block typeName instanceName localId executionOrderId>` + `<position>` | direct |
| operator/function block | `<block>` for named std funcs (`ADD`→`typeName="ADD"`) | direct |
| variable in/out/inout | `<inVariable>/<outVariable>/<inOutVariable>` + `<expression>` | direct |
| param assignment | `<inputVariables>/<inVariable>` formalParameter forms on the block | direct |
| wire | `<connectionPointOut>` + `<connection refLocalId formalParameter?>` | direct |
| assoc edge | `<connection>` expression form | direct |
| negated pin | `negated="true"` on `<connectionPointIn>` | direct |
| probe | `tether:probe` attr on connection | dropped in strict (loss note) |
| comment | `<comment>` | direct |
| label/jump | `<label localId>` / `<jump label>` | direct |
| return | `<return/>` | direct |
| EN/ENO | implicit first pins; TC6 blocks list them via signature | direct |
| instance decl | `<interface>` VAR for instance? — FB instances declared in the POU's `<interface><localVars>` as `typeName` vars | direct |
| tasks | TC6 `<tasks>`? — tasks live in our project file, exported as `tether:tasks` vendor block | strict-dropped |
| types/struct decls | `<dataTypes>` section | direct |
| GVL | `<globalVars>` | direct |
| provenance ids | `localId` numeric mapping of NodeId | direct (NodeId→localId bijection in roundtrip mode) |

Import tolerance rules: unknown vendor namespaces skipped with
import-note; `<network>`-less bodies wrapped as one implicit network;
`executionOrderId` honored; ambiguous fan-in detected → FBD003.

## Appendix F — Worked gallery (spec excerpts)

G1. **Debounce + interlock motor** (§8 example): TON → AND(NOT stop)
   → %Q — covers timer, negation, located vars, probe.

G2. **Counter chain**: CTU counting `done` pulses, `CV>=limit` →
   latch via SR, reset on operator input — covers counter + bistable +
   comparison blocks.

G3. **Analog scaling**: `%IW4` raw → `SCALE(raw, lo, hi)` (function)
   → `LIMIT` → internal REAL → comparison window → BOOL out — covers
   numeric pipeline + function blocks + REAL types.

G4. **Two-network sequencing**: net1 sets `stage` var; net2 acts on it —
   covers cross-network communication + evaluation order semantics.

G5. **Feedback through state**: integrator FB fed by its own output via
   a `PREV`-style FB — legal (state element); vs. the same graph
   without the FB → FBD004 rejection — the pair that pins algebraic-
   loop semantics.

G6–G10 reserved in as-built doc: edge-triggered set, MUX-based mode
select, IN_OUT sharing of a ramp FB, struct/array field access across
blocks, EN/ENO gating.
