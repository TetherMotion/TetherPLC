# TODO — UI Concept: Workflow-first shell

> Status: **concept**. Describes the target information architecture for
> TetherWebUI (PLC side) and how the existing board/tile system fits into
> it. No code yet — this is the agreed direction before refactoring.

## 1. Why the current UI doesn't work

Today everything is a tile. The PLC IDE is a single tile
(`tile-plc-ide`, `src/tiles/plc.ts`) sitting on the `builtin.plc` board,
and *inside* that tile lives a second navigation system — the pane
switcher in `plc-ide.ts`:

```
board tab (Machine | Motion | Analyze | Service | TetherPLC | …)
  └─ tile grid
       └─ plc-ide pane: Explorer | ST | FBD | Curves | Tasks | Watch
                        | Alarms | Events | Deploy | Project
            └─ per-pane sub-structures (POU tree → networks;
               Deploy pane → prog/shadow/dbg/retain/IO-MAP/od/journal cards)
```

Concrete consequences:

- **I/O mapping is 3 levels deep and mis-filed.** The deploy-map editor
  (`ioMapCard`, `mapRowsEditor`) — a pure *configuration* concern — is a
  scroll-down card inside the **Deploy** pane, between run-control and
  object-dictionary cards.
- **Same concept, three homes.** Runtime events exist as the shell's
  rail drawer, the IDE's `events` pane (MQTT journals) *and* the IDE's
  `alarms` pane (event journal + historian). Bus diagnostics live in a
  rail overlay; task control in `tasks`; force in `watch`; run control in
  `deploy`.
- **Discovery doesn't exist.** `tplc-ec-pump` writes `.fleet`/`.topology`
  artifacts; there is no UI surface for them at all (`GET /bus` was
  already noted as planned-but-unbuilt).
- **The workflow is invisible.** A PLC engineer's actual loop —
  *set up hardware & project → write logic → run & verify* — maps onto
  nothing. The UI is organized by implementation module (which file
  renders it), not by what the user is trying to do.
- **Dashboard chrome dominates.** Board tabs + tile galleries are an
  *operator-HMI* paradigm. They are the wrong container for an
  engineering tool, yet they are the top level of the app.

## 2. Design principles

1. **Phases, not features.** Top-level navigation mirrors the
   engineering workflow: **Configure → Program → Debug**. A fourth
   activity, **Operate**, keeps the existing board system for
   operator-facing dashboards.
2. **Two levels of navigation, max.** Activity → section. Anything
   deeper must be *documents* (editor tabs, dashboard tabs, tree
   selection) — not menus.
3. **Objects, not menus.** A discovered terminal, a POU, a task, a
   catalog key, a compile diagnostic are first-class objects that can be
   deep-linked across activities. Clicking a diagnostic opens the
   editor; clicking a discovered terminal opens its I/O-map row;
   clicking a watch row offers "add to board".
4. **Live everywhere.** No separate "online mode". When a ctl/tether-io
   connection is bound, live values flow into whatever the user is
   looking at — FBD pins, watch rows, I/O image, diagnostics tiles.
   (Already true on the canvas; extend it to every section.)
5. **The board system is a feature, not the container.** Tile boards
   remain — demoted into `Operate`, which is where they belong:
   dashboards are for operators, the other three activities are for
   engineers.
6. **Reuse, don't rewrite.** Every existing component keeps working;
   this is a re-homing + a few new surfaces, not a rewrite. `plc-ide`
   panes become activity sections; rail-overlay tiles (diag-*,
   sdo-inspector, …) get a proper place.

## 3. The new shell

```
┌──────────────────────────────────────────────────────────────────┐
│ ● machine   project: oven · main*   ▶ RUNNING  t=4.0ms  ⚡2 forces │
│   connected  (git status)           (program)  (cycle)  ⚠ 1 alarm │
├────┬─────────────────────────────────────────────────────────────┤
│ ⚙  │                                                             │
│ C  │                                                             │
│ ⚙  │                  ACTIVITY CONTENT                           │
│ P  │        (Configure | Program | Debug | Operate)              │
│ ▶  │                                                             │
│ D  │                                                             │
│ ▦  │                                                             │
│ O  ├─────────────────────────────────────────────────────────────┤
│    │ DOCK:  Problems (3)  |  Watch  |  Events         ⌃ collapse  │
├────┴─────────────────────────────────────────────────────────────┤
│ ⌘K jump anywhere · conn · theme · settings                       │
└──────────────────────────────────────────────────────────────────┘
```

- **Activity bar** (left rail, replaces today's tool rail):
  `Configure`, `Program`, `Debug`, `Operate` — icon + word, ordered by
  the workflow. Global entries stay at the bottom (connection, theme,
  settings). Alarm/event drawers remain accessible from the status
  strip — they're operator-global, not workflow steps.
- **Status strip** gains PLC state and becomes the deepest "menu" the
  app ever needs — every item is a deep link:
  `project · branch`, program `RUN/STOP/FAULT` (+ last fault rc), cycle
  time/overruns, active force count, alarm count. Click → jumps to the
  owning section.
- **Dock** (collapsible bottom panel, shared by Program and Debug):
  `Problems` (compile diags — today's strip, promoted), `Watch`
  (bound keys + force), `Events` (merged journal stream). Visible while
  editing *and* while debugging — no pane switch to see values.
- **Command palette** (`⌘K`) graduates from "add signal tile" to real
  jump-anywhere: POUs, sections, catalog keys, commands
  ("deploy", "scan bus", "new POU").

### Workflow progress (orientation, not a wizard)

A subtle pipeline indicator in the status strip:

```
Configure ✓ ──→ Program ●3 errors ──→ Debug ○ not deployed
```

Each phase shows one-line readiness (project open? diags clean?
deployed & running?). Clicking a phase opens that activity; the strip
never blocks navigation. Empty states in each section name the *next
step* ("scan the bus", "compile", "deploy & start") instead of dead
ends.

## 4. Activity: Configure

> Everything that defines the system — before, or independent of,
> running it. Flat section list; each section is one screen.

```
┌─ Configure ──────────────────────────────────────────────┐
│ ▸ Project        open/manifest/files/git                  │
│ ▸ Target         ctl backend · tether-io bridge · version │
│ ▸ Discovery      [scan bus]  EK1100 · EL1014 · EL2024 …   │
│ ▸ I/O Mapping    image in/out · channels · bindings · AT  │
│ ▸ Tasks          task table: POU bindings, period, prio   │
│ ▸ Libraries      installed catalog · resolved sources     │
│ ▸ Services       MQTT brokers · historian · alarms cfg    │
│ ▸ Commissioning  SDO inspector · config txn · baseline    │
└───────────────────────────────────────────────────────────┘
```

Content inventory (where it comes from today):

| Section | Source today |
|---|---|
| Project | IDE `project` pane (manifest, files, git status/diff/commit/log via `project_*`/`git_*` meta-ops) |
| Target | Deploy pane's ctl-URL bar + shell `connect` overlay, unified: ctl socket, `/tether-io` bridge, `/health` |
| Discovery | **NEW.** Bus scan (`bus_scan` meta-op → `tplc-ec-pump --scan`, env-gated), fleet/terminal list + topology from `.fleet`/`.topology` artifacts (needs `GET /bus`) |
| I/O Mapping | `ioMapCard` + `mapRowsEditor` moved out of Deploy. Adds: ST `AT` bindings (currently invisible — MISSING M5), image bar visualization, AT-collision warnings (`atCollisions` already exists), "add channel from discovered terminal" |
| Tasks | `tasks` pane — the *configuration* half (POU↔task bindings, period, watchdog). Runtime controls go to Debug |
| Libraries | Explorer's library section + palette groups — **resolution is automatic**: the IDE scans block refs, pin/interface/global types, and ST bodies for types the project uses but doesn't declare, and pulls the source via `blocks`/`libsrc` (manual `.st` attach remains only as fallback for sources outside the installed catalog) |
| Services | `events` pane's *connection* half (MQTT broker configs); journals → Debug |
| Commissioning | Rail overlay tools: `tile-sdo-inspector`, `tile-config-txn`, `tile-baseline`, `tile-checklist`, `tile-capture` — a first-class home instead of a modal overlay |

The **Discovery → I/O Mapping → Program** chain is the integration
payoff: a scanned EL1014 becomes a `pdo=` channel row and, one click
later, a device block in the program — the loop the device wizard and
deploy map already half-implement (`deploymap.ts` TERMINALS ↔
`fleetInfosFromText` keys stay in lockstep per AGENTS.md).

## 5. Activity: Program

> The editor workbench. Standard IDE anatomy — sidebar, editor tabs,
> inspector, dock — because POUs *are* documents.

```
┌────────────┬──────────────────────────────┬────────────────┐
│ EXPLORER   │ tabs: [Main (FBD)] [PID.st]  │ INSPECTOR      │
│ ▸ Programs │  ┌──────────────────────┐    │ block/channel  │
│  · Main    │  │   FBD canvas — live  │    │ properties     │
│  · HeatUp  │  │   pin values online  │    │ pin params     │
│ ▸ FBs      │  │                      │    │ AT address     │
│ ▸ Functions│  └──────────────────────┘    │                │
│ ▸ Libraries│  networks: ▤1 ▤2 ▤3  +fbd +ld│                │
│ ▸ Files    │                              │                │
└────────────┴──────────────────────────────┴────────────────┘
```

- **Explorer** = today's `explorer` pane (POU tree, libraries, files) —
  becomes a permanent sidebar, not a pane you navigate away from.
- **Editor tabs** = one tab per open POU/network; each tab hosts the
  right editor: `st-editor` (CM6), `fbd-canvas`, `ld-canvas`, or the
  graphical programmers `<temp-curve>` / `<temp-tune>`. Curves/Tune stop
  being global panes — they open *as editor tabs for a POU*, which is
  what they already model (`PlcProject.curves[pou]`).
- **Inspector** (right, collapsible) = the existing per-canvas
  properties panel (pin params, channel table, ST-block pin editor)
  unified into one context panel.
- **Live overlays stay.** FBD pin values, probe wires, diag outlines —
  unchanged. Editing *is* monitoring when a target is bound.
- **Problems** dock below (existing diag strip, promoted): ST diag →
  opens ST tab at span; FBD diag → opens canvas at node. Already
  implemented — just relocates.

## 6. Activity: Debug

> Everything about the running (or simulated) target.

```
┌─ RUN CONTROL ────────────────────────────────────────────────────┐
│ ● RUNNING  epoch 4 · v1.3 · deployed 12:03   [stop][start]       │
│ [build & deploy] [deploy .tplc] [rollback] [online change ▾]     │
│ retain: 1.2kB dirty [flush][dump]      shadow: clean  upload 62% │
├──────────────────┬──────────────────┬────────────────────────────┤
│ Watch & Force    │ Tasks            │ I/O Image                  │
│ plc.Main.PV 42.1 │ cyclic 10ms ✓    │ %I  ▓▓░… live bits/bytes   │
│ plc.Main.SP  50  │ event1 suspended │ %Q  ░▓▓… force overlay     │
│ [force: 1 ]      │ [suspend][step]  │ [preset/force channels]    │
├──────────────────┴──────────────────┴────────────────────────────┤
│ Events & Alarms — merged journal: program EV_* · MQTT rx/tx ·    │
│ task faults · machine events        [filter: severity/program]   │
├──────────────────────────────────────────────────────────────────┤
│ Diagnostics: bus health · topology · cyclic · supervisor · pdo   │
│ Simulate: step-file editor → tether-plc-sim replay → diff view   │
└──────────────────────────────────────────────────────────────────┘
```

Content inventory:

| Section | Source today |
|---|---|
| Run control | `deploy` pane minus io-map/od: prog table (epoch/version/fault/forces/retain-dirty), start/stop/prepare/rollback, shadow/online-change, retain ops, upload progress |
| Watch & Force | `watch` pane (`mon.writeScalar` force path) — also lives in the dock. Adds image-channel force/preset (MISSING M8) |
| Tasks runtime | Deploy pane's `debugCard` (suspend/step/resume) + cycle stats — split from task *config* |
| I/O Image | **NEW-ish**: live %I/%Q/%M view (image sizes already known from map; values via catalog/bind) |
| Events & Alarms | Merge of: `events` pane journals (MQTT rx/tx service journals), `alarms` pane (event journal + historian trend), program EV_* push frames. One stream, filterable |
| Diagnostics | Rail overlay tiles re-homed: `tile-diag-bus`, `tile-diag-chain`, `tile-diag-topology`, `tile-diag-cyclic`, `tile-supervisor`, `tile-pdo-map`, `tile-machine-state` |
| Simulate | **NEW**: step-file editor + `sim_run` meta-op → `tether-plc-sim --steps` → pass/fail + trace. Deterministic tests without touching the live target |
| Explore | `tile-catalog` — full catalog browser, lives here (it's live data) |

Deploy stays in Debug because deploy *is* run control — but its
configuration passengers (io map, od) move out. MQTT/service *journals*
live here; their *configuration* lives in Configure→Services.

## 7. Activity: Operate

> The existing board system, intact — for running the machine, not
> building it.

- Board tabs (builtin Machine/Motion/Analyze/Service, profile boards,
  user boards), tile gallery, templates, import/export — unchanged.
  Board tabs are the legitimate second level here: dashboards are
  documents.
- PLC integration *improves*: "Auto (PLC)" board generation already
  exists (`autoBoardTiles` on the `plc.*` catalog); add "watch row →
  pin to board" and "POU instance → faceplate tile" entry points from
  Debug/Program so the engineering data flows into operator surfaces.
- The old `builtin.plc` board + `tile-plc-ide` disappear — replaced by
  the Program/Debug activities.

## 8. Current → new mapping (complete)

| Today | New home |
|---|---|
| IDE `explorer` pane | Program → sidebar |
| IDE `st`/`fbd` panes | Program → editor tabs |
| IDE `curves` pane | Program → editor tab per POU |
| IDE `tasks` pane | split: config → Configure·Tasks; runtime → Debug·Tasks |
| IDE `watch` pane | Debug·Watch + global dock |
| IDE `alarms` pane | Debug·Events & Alarms |
| IDE `events` pane | split: conn cfg → Configure·Services; journals → Debug·Events |
| IDE `deploy` pane | split: run control → Debug·Run; **io map → Configure·I/O Mapping** |
| IDE `project` pane | Configure·Project |
| IDE problems strip | global dock·Problems |
| Rail: commissioning overlay | Configure·Commissioning |
| Rail: diagnostics overlay | Debug·Diagnostics |
| Rail: explore overlay | Debug·Explore |
| Rail: alarms/events drawers | stay global (operator-facing) |
| Connect overlay | Configure·Target (+ status-strip shortcut) |
| `builtin.plc` board / `tile-plc-ide` | removed — replaced by activities |
| Boards (all others) | Operate — unchanged |

## 9. Backend/API gaps this surfaces

Already-needed endpoints the concept makes explicit:

- **`GET /bus`** (or `bus_topology` meta-op): serve `.topology`/`.fleet`
  from ec-pump `--simdir`. Noted as planned — Discovery depends on it.
- **`bus_scan` meta-op**: run `tplc-ec-pump --scan` server-side; report
  discovered slaves. Environment-gated (BRD discovery doesn't work in
  this sandbox — return a clean "unsupported here" rather than an empty
  list).
- **`sim_run` meta-op**: `tether-plc-sim --image … --package … --steps
  <file>` for the Debug·Simulate section; returns step results.
- **ST `AT` bindings in `projectIoBindings`** (MISSING M5): parse ST
  located vars so the I/O map shows *all* bindings, not just FBD ones.
- **I/O image live view**: needs a read path for raw %I/%Q bytes —
  either a ctl op or mapped through the existing catalog/bind path.
- **`task_config` round-trip**: task bindings must write through to the
  project manifest, not only the UI document.
- Not yet (documented, don't block): `dbg.*` breakpoint/trace plane —
  the Tasks runtime section has a slot for it.

## 10. Rollout (phased, non-breaking)

1. **Phase 1 — reskin, no backend work.** ✅ *Implemented.*
   Activity bar (Configure/Program/Debug/Operate) in `shell/app.ts`;
   the IDE mounts once under engineering activities and gets
   `activity` + `plc-navigate` for cross-activity jumps; panes became
   activity sections (`SECTIONS` in `plc-ide.ts`); `ioMapCard` →
   Configure → I/O Map; rail overlay tiles re-homed into Configure →
   Commissioning and Debug → Diagnostics/Explore via `SECTION_TILES`
   + `ensureTileTag`; per-activity section memory (`lastPane`);
   `builtin.plc` + `tile-plc-ide` deleted (registry, loaders,
   `tiles/plc.ts`, `loadBoards` migration drops stale boards/tiles);
   ⌘K palette and board tabs now only exist under Operate.
2. **Phase 2 — real workbenches.** ✅ *Implemented.*
   Program renders the workbench instead of pane nav: explorer sidebar
   (`.plc-side`), per-document editor tabs (POU FBD/ST/Curve/Tune,
   library sources, globals — `EditorTab`, `openTab`/`closeTab`,
   `programNav` maps deep links to tabs) plus the inspector column;
   the bottom dock (`Problems`/`Watch`/`Events`) is shared by Program
   and Debug and auto-opens on fresh compile errors — diagnostics
   clicks deep-link into the ST editor; Debug's alarms and MQTT
   events merge into one filterable Events & Alarms journal
   (`mergedEvents`) with the historian/trend kept in-section; Debug
   gets an always-on run-control header (`runHeader`) — state chip,
   epoch, deploy/start/stop/rollback — above the section nav, while
   the detail cards stay in Run; the IDE publishes a `plc-state`
   summary consumed by the shell status strip (every item a deep
   link: project → Program, run state → Debug→Run, errors →
   Problems dock, forces → Watch, alarms → Events) and exposes a
   public navigation/dock/editor/command API (`navigate`,
   `openDock`, `openEditor`, `watchKey`, `command`); ⌘K is now
   jump-anywhere — activities, sections, POUs (all editor kinds),
   IDE commands, watch signals, and (under Operate) board tiles.
3. **Phase 3 — new surfaces. DONE.** `tplc serve` grew `GET /bus`
   (`.topology`/`.fleet`/`.bus` from the simdir) and the `bus_scan`
   + `sim_run` meta-ops; `tether-plc` gained `TPLC_OP_IMAGE_READ`
   (raw %I/%Q, seqlock-safe on the PLC side). Configure →
   Discovery now runs a live EtherCAT scan and renders topology,
   fleet, and PDO catalog artifacts; Debug → Simulate compiles the
   project and replays step files through `tether-plc-sim`;
   Debug → I/O Image dumps live %I/%Q bytes labeled with the
   project's AT bindings; the status strip carries a Configure →
   Program → Debug workflow progress indicator that lights from
   real IDE state and deep-links per step. ST `AT` bindings were
   already wired into the map (`projectIoBindingsEx`).

Each phase ships independently; phase 1 alone already kills the
nested-menu problem.

## 11. Explicit non-goals / keep-as-is

- **Tile/board engine**: untouched. It's a good operator-HMI system —
  just not the app shell.
- **`localStorage` document fallback** when no project is open.
- **All ctl ops and meta-ops** (`compile`, `blocks`, `libsrc`,
  `project_*`, `git_*`, binary ctl frames, EV_* push) — reused as-is.
- **Deploy-map text format** and its client-side validator — the editor
  moves, the format stays.
- **No wizard lock-in**: phases are a navigational aid, never a gate.

## 12. Open questions

- Naming: `Configure` vs `Setup`; `Debug` vs `Test & Debug` (the
  activity covers both verification *and* diagnosis).
- Does the dock belong only under Program/Debug, or also in Configure
  (problems visible while editing the I/O map)?
- Breakpoints/trace: when the `dbg.*` plane lands, does it join the
  dock or the Tasks runtime section?
- Should `Operate` boards be creatable straight from a Debug session
  ("pin this view"), or only through the existing editor?
