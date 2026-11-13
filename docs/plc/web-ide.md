# Web IDE — TetherWebUI's `TetherPLC` Board

The browser-side engineering surface: project model, ST editor, FBD
canvas, task/watch/deploy consoles, live monitoring/force, compile
diagnostics, and event push. Implemented as a separate `TetherPLC`
board in `TetherWebUI` (`src/plc/`) so existing Machine/Motion/Analyze/
Service boards are untouched.

## 1. Architecture

```
PlcIde (Lit element, src/plc/plc-ide.ts)
 ├── store.ts      project (fbd.v1 doc + stSources + tasks) in localStorage
 ├── model.ts      TS mirror of compiler/fbd-model (fbd.v1 schema)
 ├── st-editor.ts  CodeMirror 6 + StreamLanguage ST mode + lint diags
 ├── fbd-canvas.ts SVG canvas: blocks, wires, marquee, undo, validation
 ├── sigs.ts       block pin signatures (mirror of fbd-lower::resolve)
 ├── types.ts      FBD type system: parse, coerce, infer, typecheck
 ├── palette.ts    full block catalog + live pin-value keys
 ├── stiface.ts    ST interface parser (library FB decls → pins)
 ├── curve.ts      temperature-profile model + FBD/ST codegen
 ├── tempcurve.ts  <temp-curve> graphical profile editor
 ├── plcopen.ts    PLCopen XML import/export (DOM port of plcopen-io)
 ├── build.ts      /compile client, source-frame encode, diag mapping
 ├── mon.ts        catalog naming contract + scalar force encoder
 ├── ctl.ts        PlcCtl — ctl wire protocol over POST /ctl + /events
 └── hub.ts        DataHub — binds catalog keys, polls bound values
```

One `<plc-ide>` element hosts a pane switcher (explorer / ST / FBD /
curves / tasks / watch / deploy) over a shared `PlcProject`. Everything
compiles through the proxy (`toolchain.md` §3) — there is no compile or
runtime logic in the browser beyond model validation.

## 2. Project model

The editor's save format is the canonical `fbd.v1` JSON (stable key
order) — the same schema `fbd-model` consumes, so a project round-trips
WebUI ↔ `tplcc` losslessly. `PlcProject` = `{fbd: Project, stSources:
Record<pouName,string>, curves?: Record<pouName, CurveProfile>}` (the
`curves` map holds the Curves-pane editor model per POU). PLCopen XML is exchange-only
(import/export, never the save format). ST sources are stored per-POU
name; FBD POUs live in `fbd.pous[].networks[]`.

## 3. ST editing

`st-editor.ts` wraps CM6 (`@codemirror/{state,view,language,commands}`):
line numbers, gutter folding, bracket matching, indent-on-input,
history, drawSelection, tab-indent. `st-lang.ts` is a `StreamLanguage`
tokenizer aligned with the ruSTy lexer set — POU decls, control flow,
elementary types, `16#`/`T#`/`2#` literals, `(* *)`+`//` comments. It's
a tokenizer, not a grammar: highlighting only, no grammar-driven
autocomplete (the design reserves that for a richer `tplccd`).

Diagnostics arrive via `@codemirror/lint`: `diagsForFile` filters the
last check's `CompileDiag`s to the visible file, `byteToOffset` maps
`tplcc`'s UTF-8 byte spans onto CM6's UTF-16 code-unit positions (they
diverge on non-ASCII), and the IDE applies them in `updated()` so
positions survive edits. Problem-strip clicks jump the caret to the
exact span via `st-editor.jumpTo`.

## 4. FBD canvas

SVG, one `<fbd-canvas>` per shown network. State: `Set<blockId>`
multi-select + optional `selWire`, `pending` (wire being dragged),
`marquee`, `vb` (viewBox pan/zoom), undo/redo snapshot stack.

- **Block palette** — grouped and searchable (`find block…`): bit logic,
  arithmetic, comparison, selection, shifts, conversions (`X_TO_Y` for
  every scalar pair), timers, edge/bistable, counters, **Devices**,
  **Project** POUs, **Libraries** (every `FUNCTION_BLOCK` found in an
  attached ST source) and one group per installed library file
  (tempctrl / TC2 / TC3 compat packs, `.tplc` packages). The
  operator/builtin set mirrors exactly what `fbd-lower` accepts
  (`op_kind` + `builtin_sig`), so any palette entry compiles. Library
  pins come from `stiface.ts` (attached sources) or `catalog.ts`
  (`GET /blocks` on the ctl proxy — `tplcc blocks --json` over every
  `--libdir`); placing a catalog block fires `need-libsrc`, the IDE
  fetches `GET /libsrc` and stores the source in `stSources`, so the
  project stays self-contained.
- **Device blocks (`kind: 'device'`)** — fieldbus terminals such as
  Beckhoff EK modules. The palette's Devices group offers templates
  (EK1108 8×DI, EK1123 8×DO, EK1122 2×AI+2×AO, EK1110 4×DI+4×DO) plus a
  wizard (terminal name, DI/DO/AI/AO, channel count, type, base byte)
  that generates channel pins; mixed templates emit both channel
  groups into one block. The properties panel's channel table edits
  every pin's name / direction (`var_output` = device→logic,
  `var_input` = logic→device, `var_in_out` = both) / type / `AT`
  process-image address. Each channel compiles to a located variable —
  inputs alias `%I`/`%Q`/`%M` reads, outputs generate assignments into
  them — so DI→logic→DO wiring is real image traffic, and pins show
  live values via `plc.image.<I|Q|M>.ch.<pou>.__dev<id>_<pin>`. A device
  may wire its own pins (AI→AO on one terminal) without tripping the
  cycle check.
- **Pin parameters** — the properties panel renders a typed row per
  input pin (wired pins show `wired`): BOOL → checkbox, everything else
  → a literal input pre-filled from `params`; edits write
  `params[<pin>]` (the lowerer's literal fallback), so unconnected FB/
  device/ST inputs are configurable without leaving the canvas.
- **Typechecking (client-side, live)** — `types.ts` mirrors
  `tplc-mir::ty`: `parseTy` (scalars, `ARRAY[lo..hi] OF T`, `STRING(n)`,
  user POUs), `compatible`/`convertible`/`narrows`/`unifyNum`, operator
  result typing and builtin FB pin types. `inferNetwork` seeds pin types
  from declarations, propagates them through wires and operators to a
  fixpoint, and reports: type mismatch = **error** (same matrix the
  compiler uses — e.g. `BOOL → REAL` is rejected, `LREAL → REAL` is a
  narrowing **warning**), operator class violations (BOOL-only,
  numeric-only, int-only operands). Offending wires render red/amber,
  pins get a coloured ring, the status strip counts errors vs warnings,
  and every pin exposes `<title>name : TYPE</title>`.
- **Live sync** — when the ctl connection is bound, every block pin
  shows its polled value. Variable blocks read `plc.<pou>.<var>`;
  FB/function pins read `plc.<pou>.<instance>.<member>`. `DBG_FIELDS`
  only exports top-level program fields, so `tplc-io` decodes the
  package TYPC table and recursively expands STRUCT/FUNCTION_BLOCK
  members into those dotted catalog entries (see
  `tether-io-attach.md`); the browser reaches them through the
  proxy's `/tether-io` WebSocket→SLIP bridge. Operator outputs are
  scan temporaries (not exported) and stay blank. The IDE binds
  **all** `plc.*` params/signals, so member keys are polled alongside
  program vars.
- **Editing** — port-snap wire drop (out→in, one source may fan out),
  shift-click pin negation, group drag (one undo step), marquee select,
  clipboard with `nextNodeId` remap (no id collisions), align,
  multi-delete, `Ctrl-A`, `P` on a selected wire (or the ⌁ toolbar
  button) toggles its live probe — the lowerer emits a `__probe_<id>`
  field whose value renders beside the wire.
- **Validation (client-side mirror of `fbd-lower/resolve.rs`)** —
  unconnected required inputs, double-driven pins, missing blocks,
  algebraic cycles (FB feedback is legal, matching the lowerer). Runs
  on every change; invalid networks still save (the model is the truth)
  but the canvas flags them. Required means operator inputs, builtin FB
  pins (TON/TOF/TP/triggers/counters/SR/RS) and inline-ST pins; a
  user/library FB input left open is legal — the lowerer keeps its blob
  value — so `FB_AtcProfile`/`FB_AtcPid` don't drown the strip in
  warnings for optional pins.
- **Live overlays** — variable blocks show their polled value under the
  `%at` label; `probe` wires show `plc.<pou>.__probe_<id>` beside the
  source pin; blocks flagged by `fbd{sheet,node}` diags get a red
  outline (the graphical counterpart of ST squiggles).
- **Inline `ST` blocks** — palette `ST` adds a `kind: 'st'` block with
  `pins: IfaceVar[]` and `params.CODE`. The properties panel edits the
  pin table (name / in · out · in_out · var / type / initializer) and
  the ST body in a multiline editor; the block's signature (and thus its
  pins on the canvas) is derived from `pins`, with `var_in_out` visible
  on both sides. `tplcc` expands the block into a generated
  `FUNCTION_BLOCK`, so FBD keeps 100% interop with ST in both
  directions: FBD calls any ST POU, ST calls any FBD POU.

## 4b. Temperature curve editor

`<temp-curve>` (pane **Curves**) is a graphical programmer for the
`Atc_TempCtrl` library's `FB_AtcProfile`:

- A segment table (`dwell`/`ramp °/s`/`ramp time`/`step`/`loop`/`end`)
  with target °C, timing, guaranteed-soak band, event word, plus
  `PvStart`, `CycleTime`, and an optional controller
  (`FB_AtcPid`/`CascadePid`/`Smith`/`Lqr`/`Lqg`/`RelayTune`, seeded with
  limits so it runs before tuning).
- An SP-vs-time preview; dragging a point retunes its target and
  duration.
- The profile is **persisted per POU** (`PlcProject.curves`), so pane
  switches and reloads keep the table. With no saved state the editor
  **recovers the profile from the POU's own blocks** (`profileFromPou`:
  instance/controller from the FB blocks, the SEG table parsed back out
  of the inline `st` body, wiring names from the var blocks), so an
  imported `.fbd.json` opens with its real curve instead of defaults.
- **run/pv/hold/reset/out** fields auto-wire the generated blocks to
  interface vars (creating them if missing: `BOOL` for run/hold/reset,
  `LREAL` otherwise) — RUN → `prof.RUN` + controller enable, PV →
  `prof.PV` + controller PV, HOLD/RESET → the profile, `prof.SP` →
  controller setpoint, controller output → out var. Wiring is
  idempotent: an already-driven pin keeps its source.
- **Re-inserting updates in place** — same config block, same instance,
  same wires; only the SEG table, cycle time and controller type change.
  Appending a second profile would be an `INST_REUSE` error.
- **Insert into POU** emits real blocks, not a mockup: an inline `st`
  config block whose body writes `PROF.SEG[i].*` / `NUM_SEG` /
  `PvStart` / `CycleTime` one-shot, wired to the `FB_AtcProfile`
  instance through a `var_in_out` pin (aggregate inputs cannot be
  call-bound), plus the controller FB. Execution order is handled by
  the lowerer's dependency sort, so the profile holds its run request
  until the table is loaded.
- **Library dependency**: the pane checks whether any project source
  declares `FB_AtcProfile` + `ST_AtcSeg`; if not, it offers a one-click
  *attach `.st`* (`atc_tempctrl.st`) that stores the source in
  `stSources` (emitted as a file on every compile, so the project stays
  self-contained). Attached sources that are not POUs are listed under
  **Libraries** in the explorer with a detach button. Attaching a
  packaged library (`tplcc build --library` → `--lib`) is the
  service-plane equivalent for deployed images.
- **Export .st** instead emits a standalone `FUNCTION_BLOCK` source
  (same body, `PROF` as `VAR_IN_OUT`) into `stSources` and registers the
  POU in the tree, so it is editable as ST and callable from both
  languages.

The checked-in `tests/mixed/ui_profile.fbd.json` is this editor's
output for a complete project (profile + PID + RUN/PV/SP/OUT wiring);
`tests/mixed/e2e.sh` builds it and `tplcc`'s `ui_curve_profile_runs`
unit test runs it for 90 s of simulated time, asserting the ramp and
soak.

## 5. Compile + diagnostics

Toolbar `check` → `compileProject(sources, build=false)` →
`POST /compile`. `projectSources` emits **both** surfaces for a mixed
project — `project.fbd.json` plus every non-empty `<pou>.st` — because
`tplcc` compiles them as one module (see `fbd-model.md` § "Mixed FBD +
ST"). FBD-authored POUs whose body lives in an `stSource` (empty
networks) are dropped from the emitted JSON: they are editor
bookkeeping, and the ST decl is the implementation; a POU with both
networks and an ST body stays in and produces the duplicate-impl diag.
The proxy writes each file into a temp dir and invokes `tplcc check` /
`tplcc build` on all of them.

The problems strip renders diags severity-colored and clickable: St
diags open that POU's editor; Fbd diags locate the POU whose network at
`sheet` index contains the `node` id and jump the canvas there.
`build & deploy` chains `compile?build` → `PlcCtl.deploy` → status
refresh; manual `.tplc` upload remains alongside.

## 6. Monitor + force

`hub.ts` binds all `plc.*` params + signals; `pollBoundValues` resolves
bound keys over both entry kinds (params were added for PLC — signals
win collisions). The watch pane lists every bound key with its live
value and, when `entryWritable` (flag `0x02`), a force input + set
button → `mon.writeScalar` (alias-resolved schema, scalar-only —
`io-mapping.md` §5). Canvas overlays read the same `values` map.

## 7. Runtime events (`/events` SSE)

`GET /events` on the proxy forks a child holding its own greeted UDS
session with `SET_NOTIFY(0xFFFFFFFF)`, pumping each ctl frame as
`data:{type,flags,req,payload:b64}`. `PlcCtl.events(cb)` filters to
`0x0E00`-range types and decodes payloads. The IDE opens it once the
ctl channel is healthy and coalesces refreshes (150 ms) so a restarting
fault loop can't storm the poll path:

- `EV_PROG_TRANSITION` (0x0E01) → swap committed → refresh status
- `EV_FAULT` (0x0E02) → flash `plc fault rc=N` → refresh status

## 8. Deploy console

`PlcCtl` speaks the ctl wire protocol (16B header + LE payload) through
`POST /ctl`: program/rt/retain status parsers, chunked `.tplc` deploy
(SHA-256 + progress), start/stop/rollback, retain flush/clear/dump
(dump downloads the raw region as `.bin`). The deploy pane shows
prog + rt + retain cards plus the event-driven refresh; the ctl URL is
`localStorage`-persisted (`tether.plc.ctlUrl`, default
`http://127.0.0.1:8089`).

`DEPLOY_COMMIT` only *stores* the package under `packages_dir` — it does
not run it. Both deploy paths (`build & deploy`, `deploy .tplc…`) then
chain `PROG_STOP` (prepare rejects while RUNNING) → `PROG_PREPARE
<uuid>` → `PROG_START`, so a successful deploy leaves the new program
running. State-preserving online change stays on the shadow path
(deploy → `SHADOW_LOAD` → compare → `SHADOW_PROMOTE`). The program
card shows the proto-v2+ status fields: program-set `epoch`, package
`version`, `deployed_at` timestamp, last fault rc/cycle, active force
count, retain dirty bytes, and swap/quarantine state.

### Live-runtime E2E

`TetherWebUI/e2e/live-stack.sh` boots a real `tether-plc` (shm image +
pre-deployed `LiveDemo` package + ctl socket) plus `tplc-io` (dynamic
`plc.*` catalog over TCP) behind `tplc-ctl-proxy` on `:8090`
(`--socket` = the runtime ctl.sock for `/health`, `/ctl`, `/events`;
`--tplc-io` bridges `/tether-io` WebSocket→SLIP so the browser's
`DataHub` reaches live values). `e2e/plc-live.spec.ts` then drives the
real scan loop from the browser: cycle counters advance, `stop`/`start`
freeze/resume them, task suspend/step/resume hit the scheduler,
`build & deploy` ships a project package that ends up RUNNING, the
debug plane resolves/reads/forces live memory, `plc.*` values stream
into the watch pane across catalog rebuilds, and a deployed TON's
member pins (`plc.<prog>.i1.ET`/`Q`) show live values on the FBD
canvas. The stack spawns automatically when `tether-plc`, `tplc-image`,
`tplc-io` and `tplcc` are built; the spec skips otherwise.

## 9. What is deliberately not here

- No compile logic in the browser — `/compile` is always server-side
  (`tplcc serve` or `tplcc --json`).
- No variable writes outside the catalog force path — there is no
  `dbg.*` plane yet (plan 12 keeps suspend/step/trace).
- No SFC/LD editors — deferred per the language decision.
- No per-keystroke compile — `check` is explicit; the `serve` daemon
  keeps it cheap enough for the manual cadence.
