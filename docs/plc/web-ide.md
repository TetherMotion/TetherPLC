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
 ├── plcopen.ts    PLCopen XML import/export (DOM port of plcopen-io)
 ├── build.ts      /compile client, source-frame encode, diag mapping
 ├── mon.ts        catalog naming contract + scalar force encoder
 ├── ctl.ts        PlcCtl — ctl wire protocol over POST /ctl + /events
 └── hub.ts        DataHub — binds catalog keys, polls bound values
```

One `<plc-ide>` element hosts a pane switcher (explorer / ST / FBD /
tasks / watch / deploy) over a shared `PlcProject`. Everything compiles
through the proxy (`toolchain.md` §3) — there is no compile or runtime
logic in the browser beyond model validation.

## 2. Project model

The editor's save format is the canonical `fbd.v1` JSON (stable key
order) — the same schema `fbd-model` consumes, so a project round-trips
WebUI ↔ `tplcc` losslessly. `PlcProject` = `{fbd: Project, stSources:
Record<pouName,string>, tasks}`. PLCopen XML is exchange-only
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

- **Editing** — palette per block kind; port-snap wire drop
  (out→in), shift-click pin negation, group drag (one undo step),
  marquee select, clipboard with `nextNodeId` remap (no id collisions),
  align, multi-delete, `Ctrl-A`.
- **Validation (client-side mirror of `fbd-lower/resolve.rs`)** —
  unconnected required inputs, double-driven pins, missing blocks,
  algebraic cycles (FB feedback is legal, matching the lowerer). Runs
  on every change; invalid networks still save (the model is the truth)
  but the canvas flags them.
- **Live overlays** — variable blocks show their polled value under the
  `%at` label; `probe` wires show `plc.<pou>.__probe_<id>` beside the
  source pin; blocks flagged by `fbd{sheet,node}` diags get a red
  outline (the graphical counterpart of ST squiggles).

## 5. Compile + diagnostics

Toolbar `check` → `compileProject(sources, build=false)` →
`POST /compile`. `projectSources` picks the compile surface: non-empty
`stSources` → `&lt;pou&gt;.st` files; else the `.fbd.json` doc. Mixed
projects emit a note diag (the compiler routes by single-file name).

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

## 9. What is deliberately not here

- No compile logic in the browser — `/compile` is always server-side
  (`tplcc serve` or `tplcc --json`).
- No variable writes outside the catalog force path — there is no
  `dbg.*` plane yet (plan 12 keeps suspend/step/trace).
- No SFC/LD editors — deferred per the language decision.
- No per-keystroke compile — `check` is explicit; the `serve` daemon
  keeps it cheap enough for the manual cadence.
