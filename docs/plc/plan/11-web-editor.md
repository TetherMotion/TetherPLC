# 11 — Web Editor: the PLC IDE inside TetherWebUI

> **Status:** work-item document; deleted when `docs/plc/web-ide.md`
> exists and the editor ships the v1 flows end-to-end.
>
> **Satisfies:** D7 (TS reimpl, PLCopen XML), G8 (live overlay via
> symmap/probes), G10 (LD/SFC-prepared model), persona requirements
> (controls engineer in a browser).
> **Depends on:** 06 (model), 09 (mapping), 10 (compile/deploy API),
> 12 (debug semantics). **Informs:** 17.

---

## 1. Product shape

The IDE is a set of views inside TetherWebUI (Lit + TypeScript, existing
stack and transport). It speaks to three backends:

| backend | used for |
|---|---|
| `tplccd` (on-device or remote compile service) | compile, diagnose, project ops |
| `tether-plc` control channel (via bridge proxy) | deploy, program ops, debug ops |
| `tether_io` catalog | live values (watch streams, image channels, counters) |

Persona promise: a controls engineer does everything below **without a
terminal** — write ST, draw FBD, map IO, compile, deploy, online-change,
watch, force — and every operation has a structured error story.

## 2. View inventory (v1)

```
plc/
 ├─ explorer       project tree (POUs, GVLs, networks, sources, maps)
 ├─ st-editor      CodeMirror 6 ST editor + diagnostics gutter
 ├─ fbd-canvas     the FBD editor (§4)
 ├─ tasks          task config editor (name, period, priority, programs)
 ├─ io-map         mapping table editor + auto-map flow
 ├─ deploy         build/deploy/online-change console (§7)
 ├─ watch          watch tables + force panel
 ├─ diagram-live   live overlay mode on fbd-canvas (§6)
 └─ runtime        program instances, state machine, counters, logs
```

## 3. ST editor

- **CodeMirror 6** with our ST grammar for highlighting (Lezer grammar
  `st-lang` — small, own-maintained; tree-sitter is an alternative but
  Lezer is the CM-native path).
- Language services v1: syntax highlight, indent/fold, bracket match,
  identifier completion (from symmap + project index), hover type info
  (from `tplccd diagnose` cached symbols), signature hints on FB calls
  (catalog-driven), snippets (IF/FOR/CASE/FB decl).
- Diagnostics: `tplcc check` (debounced on save, or on-demand) →
  gutter markers + problems panel with code links (`tplcc explain`).
  Inline squiggles keyed by span provenance.
- Refactor-lite: rename local var/FB instance across POU (textual,
  index-driven); **rename feeds the migration alias table** (doc 08
  §3.1) — rename-aware edits are how "rename preserves state" happens.
- Block edit helpers: "insert FB call" from catalog → stub with named
  args; "generate FBD from selection" is a v2 idea (declared non-goal).

## 4. FBD canvas editor

### 4.1 Rendering & interaction

- One `<canvas>` per network (hybrid: canvas for blocks/wires, DOM
  overlays for text editing/values — accessibility + crispness; SVG
  rejected: node count + pan/zoom perf and text-select pain).
- Grid-snapped integer positions (doc 06 §4.2 determinism) — model
  stores `u32` grid units; zoom is render-only.
- Interactions (rustylink-proven set, reimplemented):
  palette drag-drop, port-snap wiring (rubber-band from output pin,
  valid targets highlighted by type), drag-reconnect, multi-select
  move/delete, box-select, pan/zoom, align/distribute, undo/redo
  (command stack on the model, not the pixels), copy/paste within and
  across POUs, comment/label/jump placement, network reorder.
- Live validation on edit: FBD001–FBD014 model rules (doc 06 §7) run
  client-side immediately — type mismatch pins tint red before any
  compile. The compiler remains the authority.
- Block palette = `tplc-rt` catalog JSON (doc 06 §10 — generated from
  stdlib manifest so palette == ship list, always).
- Network list view: collapsible, reorderable, per-network collapse to
  title bar (large diagrams stay navigable).

### 4.2 Model round-trip

Native save = `fbd.v1` JSON (canonical form — doc 06 §6); PLCopen XML
import/export per doc 06 §5 (strict for exchange, roundtrip for own
projects). Editor must never write non-canonical JSON (diff stability).

### 4.3 Undo model

Command log of model ops (`addBlock/moveWire/setParam/…`) with
coalescing; NodeIds never resurrected on undo-delete (new id — doc 06
§6 provenance durability).

## 5. Task config & IO map editors

- `tasks`: table of tasks (name, kind, period µs, priority hint,
  programs bound) + derived view "schedule feasibility" (sum of WCET
  hints vs periods — coarse but honest).
- `io-map`: channel table (label, direction, type, PDO endpoint,
  image offset preview, safe policy, forceable), auto-map wizard
  (scan → propose → diff-approve), binding matrix program↔channels.
  All edits go through map validation (doc 09 §2.2) — the UI cannot
  produce an invalid map (safety bindings are *unselectable* in the
  picker, not just rejected later — G11 by UX).

## 6. Live debugging in the diagram

The feature that sells the product. Overlay mode on the FBD canvas:

- **Wire values**: probed wires + auto-probed block outputs render
  inline value chips, updated from the `tether_io` watch stream
  (rate-limited ~5–10 Hz; scope for faster).
- **FB state badges**: TON shows `ET`, counters `CV`, bistables their
  state — read from symmap (no probe needed for *instance* fields).
- **Force indicators**: forced inputs/outputs glow + tooltip
  `forced: 1 (was 0)`; unforce from the chip.
- **ST live view**: side-panel watch of locals/globals; inline value
  popups are v2 (ST text live values need statement-level provenance —
  noted).
- Implementation: overlay reads via the debug plane (doc 12) — values
  resolve by **node id → symmap → offset**; the diagram does not parse
  code to find values. Swap-safe: after online change the overlay
  re-resolves by name/node (stale entries grayed, reported — doc 12
  epoch handling).

## 7. Build/deploy UX

- `deploy` console: target selector (device/sim), build button →
  streaming diagnostics; then deploy.
- **Online-change preview** — before committing, show the migration
  summary from `migrate_desc`: `preserved 41`, `initialized 3`,
  `dropped 1`, `denied 0` + the field list per class. The engineer
  approves knowing exactly what survives. (Cheap to build, huge trust
  win — this is the IDE's signature safety feature.)
- Deploy states mirror the runtime state machine visibly
  (`staged→migrating→armed→committed`), with abort affordances.
- Version badge: program name + version + package hash + "running
  since" + lineage (`predecessor` chain for audit).
- Sim target: compile + run against a local/CI sim instance (doc 14)
  directly from the IDE — offline development loop.

## 8. Roles, authority, safety UX

- Reuses TetherWebUI's authority/token model: `observer` (read all,
  watch), `engineer` (edit, compile, deploy-to-sim), `commissioner`
  (deploy-to-device, force, online-change commit).
- Force/commit/online-change are **authority-gated + confirm-modal**
  ops; online change additionally shows the §7 preview in the confirm.
- All mutating ops require authority token (doc 15 policy) and write
  to the audit journal — the UI surfaces *who did what when*.

## 9. Project & file management

- Projects live server-side (tplccd workspace per device) with
  import/export as `.zip`/PLCopen XML; browser-local editing is an
  offline-first cache, not the store of record.
- VCS story: export produces canonical files designed for git
  (doc 06 canonical JSON, ST `.st`, project `.toml`, `iomap.v1.json`).
- Concurrent editing: v1 = last-writer-wins with save-conflict
  detection (ETag on files); collaborative editing is out of scope.

## 10. Non-goals for v1

- Graphical SFC/LD editors (iter-2; model-ready).
- Diff/merge tooling for diagrams (VCS handles text; visual diff is v2+).
- Breakpoint UI in FBD (doc 12 decides the mechanism first).
- Custom block-face styling/branding.
- ST→FBD or FBD→ST auto-translation (they share mir but surface
  translation is a research-y feature — declared non-goal).

## 11. Iteration-2 hooks

- Network `kind:"ladder"` → LD canvas = same engine, constrained
  palette + rail layout; spec'd as a rendering mode of fbd-canvas.
- SFC: new top canvas (steps/transitions) whose action bodies open
  the *existing* ST/FBD editors — reuse path designed-in.
- Subsystem drill-in: canvas supports nested network stacks.

## 12. Acceptance criteria

1. Full loop in browser: create project → FBD with ≥2 networks +
   one ST POU → map IO → compile → deploy → run — no terminal.
2. Live overlay: ≥10 probed values updating ≥5 Hz without measurable
   scan jitter impact (accounted test).
3. Force: input force + output force + unforce from diagram; stale
   forces after online change reported.
4. Online-change preview renders the four classes + field lists;
   denied change blocks commit UX-side AND server-side.
5. PLCopen import of a Beremiz demo opens editable diagram; strict
   export validates TC6.
6. Diagnostics: clicking a problem navigates to span/node (both ST
   and FBD provenance).
7. Sim target deploy works end-to-end from the UI.
8. Role gating enforced server-side (UI cannot be talked into
   commissioning ops — verified by API tests).
