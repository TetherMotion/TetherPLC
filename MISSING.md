# Missing pieces for the "temperature profile on Beckhoff I/O" user story

Scope: TetherPLC + TetherWebUI (+ the Tether EtherCAT side where the
story physically depends on it).

## The user story

A typical machine-builder workflow:

1. Attach **a bunch of Beckhoff EL1014** (4-ch DI) EtherCAT terminals —
   one channel is the "start profile" input, the rest are operator
   inputs / interlock signals.
2. Feed the loop PV from **a Beckhoff analog input** terminal.
3. Drive the heater through **EL2004** (4-ch DO) with
   **FB_AtcPwm at 1 Hz base frequency** (Period = 1.0 s).
4. One EL2004 channel reports **"process finished"** downstream,
   another reports **"process running"**.
5. All of this must be buildable **in FBD and in ST**.

## What already works (verified in-tree)

- `device` FBD blocks: free-form `type_ref`, channel pins with `at`
  process-image bindings (`%IX/%QX/%ID…`); DI→logic→DO is real image
  traffic (`tests/device/dev_term.fbd.json`, `ek_pid.fbd.json`).
- WebUI device wizard: name/channel/count/type/base-byte → generates
  pins; per-channel editor for name/dir/type/`at`
  (`fbd-canvas.ts` `deviceWizard`/`devicePins`/`deviceEditor`).
- `FB_AtcProfile` exposes `SP`, **`Done`**, **`Busy`**, `SegIdx`,
  `SegTime`, `SegTgt`, `Err` — the running/finished signals exist.
- `FB_AtcPwm`: `Duty` 0–100 %, `Period` s, `MinOn`, `Q` BOOL out —
  1 Hz = `Period := 1.0`; `ek_pid.fbd.json` proves
  DI→Execute, AI→Actual, `pid.Out`→`pwm.Duty`, `pwm.Q`→DO end-to-end
  (interpreter + sim replay).
- ST located vars: `x AT %IX0.0 : BOOL` (and `%QX/%MX`) compile and
  run (`tests/ld/02`, `05`); mixed FBD+ST projects compile into one
  package; multiple PROGRAMs + editable task table exist.
- Live channel monitoring: `plc.image.<I|Q|M>.ch.<pou>.__dev<id>_<pin>`
  catalog signals; IO-map card in the Deploy pane.
- Map validation is strict and complete (`validate_map`,
  `TPLC_E_MAP_*`): range/owner/type/safe/at/bind/dir/pdo/epoch.

## Missing — ordered by how hard each blocks the story

### M1. Deploy-map authoring is entirely manual (critical)

The bridge between the PLC image and the physical bus is a hand-written
text file (`image in=N out=M`, `in|out off= size= key=|pdo=`,
`bind <at> <key>`, `safety`, `hash`). Nothing generates it:

- `tplcc build` emits `IO_MAP` (0x30) — a JSON array of
  `{name, region, byte, bit}` — but **no tool consumes it** to produce
  the deploy map. `DEPLOY_MAP` (0x31) is only injected by hand via
  `tplcc pkg add-section --type 0x31`, or passed as `--map <file>` to
  `tether-plc-bridge`.
- The WebUI compile/deploy flow uploads the raw `.tplc`; there is no
  map-file step, no section injection, no map editor. The io-map card
  (`plc-ide.ts` `ioMapCard`) is **read-only**.
- `projectIoBindings()` already lists every `at` binding in the project
  — the exact input a generator needs — but nothing turns it into
  `in|out|bind` lines, and nothing cross-checks it against the bus
  `.bus` PDO catalog.
- `validate_map` exists only inside `tether-plc-bridge` startup; there
  is no `tplcc`/`tplc-*` CLI to validate a draft map against a package
  + bus catalog before deploy.
- The user must know image offsets, channel sizes, PDO endpoint names
  (`el2004.c1`-style keys from the pump), `safe=` policy, and the
  `--map-hash` stamping ritual (`tether-plc-bridge --map-hash`).

For "a bunch of EL1014s" this means hand-computing ~20+ bind/channel
lines with zero tooling feedback.

### M2. EtherCAT fleet/pump cannot express the topology (critical, Tether side)

- `examples/common/BeckhoffFleet.hpp` `fleetInfos()` is a **fixed**
  topology: 2× EK1100 couplers, each with exactly **one** EL1004,
  one EL2004, one EL3104, one EL4104. There is **no EL1014 module at
  all**, and no way to attach N modules of one type — "a bunch of
  EL1014s" is impossible even in emulation.
- The EL1014 *driver* exists (`src/Beckhoff/EL1014.cpp`,
  `MultiEL1014.hpp`) and EL2004 likewise, but they are not in the
  fleet table; the pump (`tplc-ec-pump`) iterates `fleetInfos()` only.
- There is no production/configurable master: fleet + pump are
  examples. A real machine needs a fleet-description format
  (slave list: vendor/product/PO2SM/PDO mapping per coupler) driving
  `FleetMaster::bringUp`; only the hardcoded path exists.
- PDO endpoints are whole-slave TxPDO/RxPDO images; deploy-map channel
  `size=` must match exactly or `TPLC_E_MAP_PDO` fires — fine for
  EL1014 (1-byte TxPDO) but the user has no catalog telling them the
  sizes per part.

### M3. Analog input channel reality (high)

- The wizard's AI channels emit `%ID<base+4i>.0` typed `REAL` — a
  32-bit float image slot. Real Beckhoff AI terminals carry **raw
  counts** (EL3104 in the fleet = 4ch × 16-bit = 8-byte TxPDO;
  EL3114/EL3164/EL3174 variants differ). The deploy-map `type=` and
  channel `size=` must match the PDO bytes; nothing guides the user
  to the right type.
- **No scaling path**: `atc_tempctrl` has no raw→engineering-units
  block; terminal-side scaling is mailbox/SDO device configuration —
  the "device parameters" gap (`Block.params` exists but device blocks
  never consume it; no SDO write path from PLC or IDE).
- The wizard type list offers `REAL`/`LREAL` but not the `INT`/`UINT`
  raw-count workflow prominently; a thermocouple/RTD terminal's real
  behavior (cold-junction, linearization) is entirely terminal config.

### M4. "A bunch of modules" ergonomics (high)

- Device wizard `base` defaults to **0 every time** — no auto-allocation
  of the next free `%I`/`%Q`/`%ID` range across existing device blocks
  and located vars. Eight EL1014s = eight hand-tracked base bytes.
- **Silent channel aliasing**: the compiler deliberately shares storage
  for identical `at` addresses (`at_vars` dedup in `fbd-lower`). Two
  EL1014 blocks left at `base=0` silently share DI0–DI3 — no warning.
  `projectIoBindings` is display-only; `deployMapOverlaps` only checks
  deploy-map channels, not project `at` bindings.
- No terminal catalog: `EL1014`/`EL2004`/`EL3xxx` are free-typed names;
  no part-number suggestions, no per-part channel-count/layout presets,
  no datasheet-derived PDO size hints.
- No device-instance concept beyond `type_ref` + block id — naming two
  blocks "EL1014" is allowed (live keys disambiguate via `__dev<id>`),
  but nothing helps the user label coupler/position.

### M5. ST path parity (high for the "also in ST" requirement)

- ST located vars work (`AT %IX0.0`), but there is **no device-block
  equivalent**: the user must hand-declare every channel
  (`di0 AT %IX0.0 : BOOL; …`) with hand-computed bit/byte offsets —
  for 8× EL1014 that is ~32 error-prone addresses and no wizard.
- ST `AT` bindings are **invisible to the WebUI**:
  `projectIoBindings()` scans only the FBD JSON model; ST sources are
  opaque text in `stSources`. The compiler does emit them into
  SMAP/IO_MAP, but the IDE's "project bindings" table misses them.
- `%ID` (analog, D-width) `AT` in ST is untested in-tree (only
  `%IX/%QX/%MX` vectors exist) — likely works via the same `parse_at`,
  but should be verified.
- No ST-side terminal/device abstraction or generator; mixed
  FBD+ST works, so a user *can* put device blocks in FBD and control
  logic in ST — but a pure-ST project has no assisted I/O path.

### M6. Control-logic composition gaps (medium/low)

- `FB_AtcProfile.Done`/`Busy` exist but nothing auto-wires them —
  the user manually routes them to DO pins (acceptable, but a
  "status outs" convenience on the profile/curve insertion would
  match the story's finished/running requirement directly).
- No heater-channel helper composing `controller → FB_AtcPwm → DO`;
  `ek_pid.fbd.json` proves the manual 4-wire pattern works in FBD;
  the same is expressible in ST but with no guided path.
- PWM granularity: `FB_AtcPwm` steps at `CycleTime`; 1 Hz period at a
  0.1 s task = 10 steps → **10 % duty resolution**. The task table is
  editable, but nothing warns that heater duty resolution is
  scan-rate-bound.
- `Done` is a level that persists until the next RUN/RESET cycle —
  downstream "finished" signaling may want a pulse; no pulse/ack
  output exists (user can edge-detect in logic — minor).

### M7. Packaging & docs (medium)

- `scripts/make-deb.sh` ships `tplcc`, `tether-plc`, `tplc-io`,
  `tplc-ctl`, `tplc-ctl-proxy`, `tplc-image`, probes — but **not**
  `tether-plc-bridge` or `tplc-ec-pump`, the two binaries that actually
  move image↔PDO data. `apt install` alone cannot run the I/O path.
- `tplc-ec-pump` builds only when `TETHER_DIR` points at a Tether
  checkout (examples headers) — not packaged at all.
- No single quickstart tying the story together: device blocks →
  deploy map → pump `.bus` catalog → bridge → live `plc.image.*`
  monitoring. The pieces are documented separately
  (`tether-io-attach.md`, `io-mapping.md`, `plc-runtime.md`) but no
  end-to-end "attach EL1014s, run a profile" guide exists.

### M8. Force/ stimulate image channels from the IDE (low-medium)

- `plc.image.*.ch.*` catalog entries are `kind:"signal"`,
  non-writable — DI states can be watched but not forced, DO states
  can't be preset from the UI. Bench-testing the "start" button or a
  stuck DI requires the sim file transport or physical I/O.
- Located vars also publish as writable `plc.<prog>.<name>` params —
  whether `DBG_WRITE` on an AT-bound var reaches the image byte (vs a
  shadow) needs verification; if it does, forcing may already work
  through that path and only the UX is missing.

## Suggested order

1. **M1** — deploy-map generator + validator CLI + WebUI editor/
   "generate from bindings" (unblocks everything physical).
2. **M2** — fleet description format + EL1014 in `fleetInfos()` (or N
   modules per type) so the story is even *testable* end-to-end.
3. **M4/M5** — terminal catalog + base-address allocator + collision
   warning; ST AT-var generator / bindings visibility.
4. **M3** — analog channel types + scaling block/device params.
5. **M7/M8** — package the bridge/pump, write the quickstart, image
   force path.
