# TetherPLC / TetherWebUI / Tether — User Stories

135 end-to-end user stories describing how engineers, operators,
integrators, and maintenance teams use the TetherPLC ecosystem:
the `tplcc`/`tplc-pkg` toolchain, the `tether-plc` runtime, the
`tether-plc-bridge`/`tplc-ctl` control plane, `tplc-ec-pump` EtherCAT
I/O, the TetherWebUI browser IDE (ST/FBD/LD editors, curve editor,
autotune editor, deploy-map editor, watch/force/debug), the
`atc_tempctrl` temperature library, the Tc2/Tc3 compatibility
libraries, and the Tether EtherCAT master.

Each story lists a persona, the scenario, concrete requirements, and
acceptance criteria. Stories are grouped by domain. Where a story
depends on a capability that does not yet exist, the requirements
describe the *target* behaviour — see MISSING.md for the original
gap analysis and the implementation status notes.

---

## Domain 1 — Temperature & Thermal Processes (US-001 – US-009)

### US-001 — Batch oven with ramp/dwell profile on Beckhoff I/O
**Persona:** Process engineer at a coating line.
**Context:** Parts must follow a 6-segment profile (ramp to 180 °C,
dwell, ramp to 320 °C, dwell, controlled cool, end) driven by an
EL3104 thermocouple input and an EL2004 SSR bank.
**Requirements:**
- Graphical curve editor emits `FB_AtcProfile` + controller +
  `FB_AtcPwm` wiring in FBD, or equivalent ST.
- Profile `Busy`/`Done` land on dedicated EL2004 channels for the
  line controller.
- PV path: `%ID` AI channel → `FB_AtcAiScale` → `FB_AtcProfile.Actual`.
- Start comes from an EL1014 channel; reset from another.
**Acceptance criteria:**
- Door-open interlock pauses the profile and holds outputs safe.
- Profile completes → `Done` energizes finished relay, `Busy`
  de-energizes; re-start requires explicit reset.

### US-002 — Reflow soldering oven, 10 independently controlled zones
**Persona:** Electronics contract manufacturer.
**Context:** Ten convection zones, each needs its own PID with
cascade (zone temp → belt-speed trim) and per-zone setpoint offsets
selected by recipe.
**Requirements:**
- Ten `FB_AtcCascadePid` or `FB_AtcPid` instances, one per zone,
  fed by ten EL3104 channels.
- Recipe block selects per-zone SP offsets and conveyor speed trim.
- Zone over-temperature (>limit) forces that zone's output to 0 and
  raises an alarm — never silently clamps.
**Acceptance criteria:**
- Zone PV deviation > configured band for T seconds raises alarm.
- Belt speed change is bumpless: profile setpoints unchanged, only
  feed-forward trim moves.

### US-003 — Autotune a mystery loop during commissioning
**Persona:** Controls technician, first visit to a customer site.
**Context:** A customer's extrusion barrel heater "oscillates" and
nobody knows the plant. No documentation.
**Requirements:**
- `<temp-tune>` editor offers relay, step-identification, LQR, and
  LQG methods.
- Step-identification mode injects a safe bounded step automatically,
  records PV response, fits FOPDT, proposes PID gains.
- Apply is a single explicit strobe; before/after gains are shown.
**Acceptance criteria:**
- Tuned gains land in the live controller instance (VAR_IN_OUT
  handoff), not just displayed.
- Abort at any time returns outputs to manual/previous setpoint.

### US-004 — Walk-in cooler with defrost cycle and door interlock
**Persona:** Cold-chain integrator.
**Context:** Cooler must hold −18 °C, run a timed defrost every 6 h,
and alarm if door ajar >5 min while compressor runs.
**Requirements:**
- Profile or sequencer runs defrost: disable refrigeration output,
  energize defrost heater for T minutes, resume with anti-short-cycle
  compressor delay.
- Door switch (EL1014) input gates compressor enable via interlock FB.
- High-temp alarm (>−10 °C for >2 min) latches until acknowledged.
**Acceptance criteria:**
- Power fail during defrost resumes safely; no compressor restart
  storm on power-up.
- Door-open event logged with timestamp for HACCP records.

### US-005 — Extruder barrel zone control with split-range cooling
**Persona:** Plastics machine builder.
**Context:** Each barrel zone has electric heaters (0–100 %) and a
water-cooling solenoid. Over-temperature must drive cooling, not
just cut heat.
**Requirements:**
- `FB_AtcHeatCool` split-range output: heat 0–50 %, cool 50–100 %.
- Cooling water valve is an EL2004 channel gated by zone demand.
- Melt-pressure-adjacent zone uses rate-limited setpoint changes
  (`FB_AtcRateLim`) to avoid thermal shock.
**Acceptance criteria:**
- Cooling never energizes while heat demand >0 unless over-temp
  threshold crossed (configurable crossover).
- Zone stable within ±1 °C of setpoint at steady state.

### US-006 — 3D printer heated bed + hotend with profile selection
**Persona:** Printer OEM firmware engineer.
**Context:** Bed uses PID, hotend uses profile ramp with material
library presets; user picks material on the HMI.
**Requirements:**
- Material presets stored as parameter sets (SP, PID gains, fan
  curve, profile segments) selectable by index.
- Hotend profile: ramp at configurable °C/s with dwell before print
  start; bed soak timer before "ready".
- Fan output is duty-cycle PWM at 1 Hz derived from zone state.
**Acceptance criteria:**
- Material change mid-print applies new setpoints bumplessly.
- "Ready" only asserts after soak completes AND hotend within band.

### US-007 — Pasteurizer tunnel with lethality (F0) accumulation
**Persona:** Food safety engineer.
**Context:** Regulatory requirement: every unit must accumulate
F0 lethality; process must divert under-lethality product to rework.
**Requirements:**
- `FB_AtcProfile` tracks zone temps; F0 integrator
  (∫10^((T−Tref)/z) dt) accumulates while product in hold zone.
- Divert gate (EL2004) actuates when accumulated F0 < target at
  tunnel exit sensor.
- Per-batch F0 record exported to historian/event log.
**Acceptance criteria:**
- Any zone sensor failure freezes accumulation and diverts product.
- F0 calculation uses real scan timestamps, not assumed cycle time.

### US-008 — Multi-zone oven master/slave balancing
**Persona:** Paint-line controls engineer.
**Context:** Twelve oven zones share one exhaust fan; zone damper
positions must balance so all zones hold setpoint within tolerance.
**Requirements:**
- One master zone PID cascades trim to slave zone SP offsets.
- Damper position feedback (0–10 V via EL4104/EL4114 AO readback)
  closes a positioning loop per zone.
- Inter-zone air-mixing model optional (cross-coupling compensation).
**Acceptance criteria:**
- Worst-zone deviation < ±2 °C during product loading transients.
- Damper fault (feedback lost) parks damper at safe position and
  alarms — does not run open-loop.

### US-009 — Laboratory freeze-dryer with ramp/soak and endpoint detection
**Persona:** Biotech lab automation engineer.
**Context:** Shelf temperatures follow validated recipe; endpoint
detected by pressure-rise test (valve closes, pressure rise rate
indicates dryness).
**Requirements:**
- Shelf zones: three `FB_AtcPid` cascaded under one profile.
- Pressure-rise test as sequencer state: close isolation valve,
  sample chamber pressure for N min, compute dP/dt, classify dry/wet.
- Recipe parameters (ramp rates, soaks, endpoint thresholds) stored
  per product in NV memory.
**Acceptance criteria:**
- Endpoint decision logged with pressure trace excerpt.
- Recipe download from HMI validated against allowed ranges before
  commit; invalid recipe rejected with reason.

---

## Domain 2 — Motion & Drives (US-010 – US-018)

### US-010 — Conveyor tracking with encoder-following reject
**Persona:** Packaging line engineer.
**Context:** Vision system detects a defect and reports position;
a diverter must fire when the package arrives — timing must track
belt speed changes.
**Requirements:**
- Encoder channel (EL5101-class or sim) feeds position counter.
- Reject logic: store target position + pulse width, arm when
  encoder crosses trigger window accounting for belt slip model.
- Belt speed changes (VFD via Modbus or analog) update tracking
  gain automatically.
**Acceptance criteria:**
- Reject lands within ±5 mm at belt speeds 0.2–2 m/s.
- Encoder loss triggers safe stop and alarm, never silent coast.

### US-011 — Pick-and-place with two servo axes and vacuum gripper
**Persona:** Machine builder for electronics assembly.
**Context:** X/Y gantry moves between pick and place points; vacuum
grate ON at pick, OFF after place dwell; part-present sensor verifies.
**Requirements:**
- Axis position/velocity commands via EtherCAT CiA402 or pulse
  interface; Tether motion planner handles point-to-point moves.
- Vacuum output gated by "at pick position AND vacuum enabled" logic.
- Part-missing at place triggers retry sequence (up to N) then alarm.
**Acceptance criteria:**
- Cycle time < 1.2 s pick-to-place at rated payload.
- E-stop anywhere in sequence leaves axes stopped and vacuum off.

### US-012 — CNC router with tool-change macro and probe cycle
**Persona:** Woodworking/CNC retrofit engineer.
**Context:** Machine has tool rack, spindle, and touch probe; programs
reference tools by number; probe must measure part datum before cut.
**Requirements:**
- Tool-change sequence: retract, move to rack position, clamp/unclamp
  verify switches, spindle warm-up, resume — all in ST/FBD logic.
- Probe cycle: approach at fast rate, slow touch, back-off, repeat
  for 3-point datum; results feed work-offset registers.
- Spindle warm-up timer and overspeed interlock.
**Acceptance criteria:**
- Tool-change completes < 15 s including verification.
- Probe failure (no contact within travel) aborts with clear alarm.

### US-013 — Multi-axis gantry with soft limits and homing
**Persona:** Laser-cutting machine integrator.
**Context:** Two X motors must stay synchronized (gantry), Y crosses;
crash damage is expensive.
**Requirements:**
- Gantry skew monitoring: position difference between X motors >
  threshold triggers controlled stop.
- Soft limits from machine parameters; hard limits are dedicated
  safety inputs that remove all motion authority.
- Homing sequence: slow approach, index pulse, set zero, move to
  safe park — repeatable to ±0.1 mm.
**Acceptance criteria:**
- Homing works after power loss without manual intervention.
- Soft-limit violation decelerates, does not instant-stop (unless
  hard limit also hit).

### US-014 — Rotary indexing table with part-present tracking
**Persona:** Assembly automation engineer.
**Context:** Table indexes 45° per station; each station may reject
parts; downstream must know which stations have good parts.
**Requirements:**
- Index command moves table, waits for in-position + part-present
  sensors before releasing next index.
- Per-station bitmask tracks part presence through the cycle.
- Reject station sets its bit; downstream stations skip or divert.
**Acceptance criteria:**
- Table never indexes with a station still processing (interlock).
- Presence bitmask survives power-cycle for in-flight parts only
  when configured retain; otherwise resets to safe state.

### US-015 — Web-handling dancer arm tension control
**Persona:** Converting plant controls engineer.
**Context:** Film/web line needs constant tension; dancer arm
position is the feedback, line speed is manipulated.
**Requirements:**
- `FB_AtcPid` (or cascade) controls dancer position → line speed
  trim.
- Tension load cell (AI) provides outer loop if available.
- Threading mode: manual inching with tension bypass.
**Acceptance criteria:**
- Tension variation < ±2 % at speed ramps 10–100 %.
- Threading mode disables tension control and alarms if web break
  sensor triggers.

### US-016 — AGV dock handoff with position confirmation
**Persona:** Intralogistics integrator.
**Context:** AGV docks at conveyor infeed; dock clamps, confirms
vehicle present + position, then releases parts onto conveyor.
**Requirements:**
- Dock present sensor + positioning pin verify before transfer
  enable.
- Transfer enable is a time-limited pulse; AGV departure detected by
  dock-clear sensor.
- Interlock: conveyor cannot run unless dock state = CLAMPED and
  vehicle present.
**Acceptance criteria:**
- Dock handshake completes < 3 s; timeout → alarm + safe state.
- Undock before transfer complete aborts with parts held safe.

### US-017 — Spindle tool-life tracking and auto-change prompt
**Persona:** Machining cell supervisor.
**Context:** Cutting tools have rated life (parts or minutes); cell
must prompt change before dull tool scraps parts.
**Requirements:**
- Per-tool counters: parts cut, run minutes, overload events.
- Threshold crossing raises "tool change due" warning at 80 %,
  mandatory stop at 100 %.
- Tool data table in NV memory survives power cycle.
**Acceptance criteria:**
- Warning appears on HMI ≥10 parts before mandatory stop.
- Overload event (torque spike) increments wear counter by
  configurable amount.

### US-018 — Multi-axis cam (fly-cut / label registration)
**Persona:** Packaging machinery engineer.
**Context:** Print head must track moving web; registration mark
provides phase correction each revolution.
**Requirements:**
- Cam table (position vs. master encoder angle) drives follower axis.
- Registration mark sensor triggers phase offset each cycle.
- Web speed change re-scales cam timing automatically.
**Acceptance criteria:**
- Print registration ±0.5 mm at web speeds 10–150 m/min.
- Mark-miss >N consecutive triggers alarm and falls back to open-loop
  cam (no correction) rather than chasing noise.

---

## Domain 3 — Digital/Analog I/O & Fieldbus (US-019 – US-027)

### US-019 — Multi-terminal Beckhoff input rack with start/interlock matrix
**Persona:** Machine builder (the canonical user story).
**Context:** Three EL1014 input terminals provide 12 DI channels:
start, stop, reset, door switches, part-present, E-stop chain, and
spare operator inputs — all must be individually addressable in FBD.
**Requirements:**
- Fleet file lists `EK1100` + `EL1014`×3 + `EL3104` + `EL2004`;
  `tplc-ec-pump` spawns unique PDO keys.
- Device wizard generates one block per terminal with per-channel
  pins; auto base-byte allocation prevents overlap.
- Any channel usable as FBD pin or ST `AT %IX` var.
**Acceptance criteria:**
- All 12 channels readable live in watch pane with correct bit
  positions.
- Deploy-map generated from bindings matches bridge canonical hash.

### US-020 — Analog sensor fusion: redundant temperature probes
**Persona:** Safety-critical process engineer.
**Context:** Two PT100 probes measure the same zone; disagreement
indicates sensor fault — must not silently pick one.
**Requirements:**
- Both probes → `FB_AtcAiScale` → plausibility comparator FB.
- Deviation > threshold for T seconds → sensor-fault alarm +
  fallback to median/last-good with flag.
- Sensor drift tracking: slow divergence logged for predictive
  maintenance.
**Acceptance criteria:**
- Single probe failure detected within configured time and alarmed.
- Control continues on surviving probe; fault flagged in event log.

### US-021 — Pneumatic valve manifold with per-channel diagnostics
**Persona:** Factory automation engineer.
**Context:** 16 solenoid valves on one manifold; need per-valve
command, feedback (if fitted), and cycle counters for maintenance.
**Requirements:**
- Valve FB per channel: command, feedback verify (on/off sensor),
  cycle counter, fault on mismatch >T.
- Manifold air-pressure switch as global interlock input.
- All channels visible as individual FBD pins.
**Acceptance criteria:**
- Valve stuck-on or stuck-off detected and alarmed per channel.
- Cycle counters exported to maintenance report hourly.

### US-022 — CANopen device integration (VFD, encoder, IO module)
**Persona:** Mobile machinery integrator.
**Context:** Vehicle has CANopen VFD, absolute encoder, and CANopen
IO block; need cyclic PDO exchange plus SDO config.
**Requirements:**
- CANopen master service in runtime: NMT, SYNC, PDO mapping.
- SDO client for parameter read/write with timeout + retry.
- Device diagnostics: heartbeat monitoring, EMCY handling, node
  guarding.
**Acceptance criteria:**
- Bus-off detection and auto-recovery without PLC restart.
- EMCY emergency messages surface as alarms with node ID.

### US-023 — IO-Link master for smart sensors (level, pressure, RFID)
**Persona:** Process instrumentation engineer.
**Context:** Smart IO-Link sensors provide process value + diagnostics
+ parameterization; need cyclic I/O plus acyclic SIO/parameter access.
**Requirements:**
- IO-Link master block: cyclic process data per port, device
  parameters via IODD-style mapping.
- Port diagnostics: short circuit, wire break, device missing.
- Sensor replacement: auto-detect new device, restore params from
  stored config.
**Acceptance criteria:**
- Wire-break on port detected < 100 ms and alarmed.
- Device swap restores process value within 1 s without manual
  intervention.

### US-024 — PROFINET IO device (PLC as IO device to SCADA PLC)
**Persona:** System integrator with multi-vendor cell.
**Context:** TetherPLC runs machine logic; a SCADA-level PLC needs
TetherPLC's data as PROFINET IO device (controller ↔ device).
**Requirements:**
- PROFINET device stack: DCP discovery, LLDP, cyclic IO exchange.
- GSDML-based module definition matching EtherCAT device blocks.
- Diagnostics channel (DCP/AL) for device health.
**Acceptance criteria:**
- SCADA PLC sees TetherPLC I/O as configured PROFINET device.
- TetherPLC restart → PROFINET device re-announces and resyncs.

### US-025 — EtherCAT segment diagnostics and frame-error analysis
**Persona:** Maintenance technician troubleshooting intermittent faults.
**Context:** Occasional CRC errors / lost frames on EtherCAT segment;
need to identify which slave/cable/port.
**Requirements:**
- Per-slave frame counters: TX/RX, CRC errors, missed cycles.
- Working-counter mismatch detection per cycle.
- Topology map (from `.bus` file) rendered in WebUI with error
  highlighting.
**Acceptance criteria:**
- Fault localizes to slave index + port within one diagnostic cycle.
- Error counters exportable for trend analysis.

### US-026 — Loopback/self-test mode for I/O validation
**Persona:** Commissioning engineer verifying wiring before production.
**Context:** Before go-live, verify every DI reads correct switch
and every DO drives correct actuator — without manual jumpering.
**Requirements:**
- Loopback mode: DO command reflected to DI internally for wiring
  check.
- Per-channel force/stimulate from WebUI (already shipped — see
  `plc.image.*` stim action).
- Channel health report: all channels tested, pass/fail list.
**Acceptance criteria:**
- Full I/O checkout completed in < 10 min by one technician.
- Report lists every channel with OK/FAIL and timestamp.

### US-027 — Wireless sensor node (BLE/LoRa) integration
**Persona:** Building/facility engineer adding battery sensors.
**Context:** Door/window contacts, occupancy, temperature in remote
areas — battery nodes, no wired infrastructure.
**Requirements:**
- Wireless gateway block: BLE/LoRaWAN receive, node ID → process
  variable mapping.
- Battery-level monitoring with low-battery alarm at 20 %.
- Signal-strength indicator per node.
**Acceptance criteria:**
- Node dropout detected and alarmed within configured timeout.
- Gateway reconnects nodes automatically; no manual pairing per
  power cycle.

---

## Domain 4 — Safety & Security (US-028 – US-036)

### US-028 — Safety-rated E-stop chain with dual-channel monitoring
**Persona:** Machine safety engineer (ISO 13849).
**Context:** Two-channel E-stop must be monitored for short-circuit
and discrepancy; single-channel failure must not defeat the stop.
**Requirements:**
- Dual-channel E-stop inputs with discrepancy detection (<50 ms).
- Output stage: safety relay or safe-PLC output with feedback
  monitoring (EDM).
- Force/stimulate of safety channels blocked at debug level
  (different privilege than process forces).
**Acceptance criteria:**
- Short between E-stop channels detected → safe state + alarm.
- Any safety-channel fault removes motion/power authority
  immediately, independent of program scan.

### US-029 — Light-curtain muting during material feed
**Persona:** Safety integrator for press/robot cell.
**Context:** Light curtain protects operator zone; muting required
when material passes through on conveyor — only while machine in
specific state and material detected.
**Requirements:**
- Muting inputs synchronized (both beams within time window).
- Muting only active when: cycle active + material-present sensor +
  not in manual mode + not in teach mode.
- Muting timeout: if beams don't resync in T ms, unmute and stop.
**Acceptance criteria:**
- Muting never active during manual/teach/jog operations.
- Any muting fault → immediate stop, logged with timestamp.

### US-030 — Two-hand control for hazardous cycle initiation
**Persona:** Safety engineer for press brake.
**Context:** Operator must press both palm buttons simultaneously
to start stroke; release either → stop; anti-repeat prevents
re-trigger.
**Requirements:**
- Two-hand input FB: both pressed within window + held through
  stroke initiation.
- Anti-repeat: both must release before next cycle allowed.
- Time-window enforcement: presses >500 ms apart rejected.
**Acceptance criteria:**
- Single-hand or sequential press cannot initiate cycle.
- Sticking button detected (held >30 s without stroke) → alarm.

### US-031 — Lockout-tagout (LOTO) digital procedure enforcement
**Persona:** EHS manager at manufacturing plant.
**Context:** OSHA requires documented energy isolation before
maintenance; paper LOTO is error-prone.
**Requirements:**
- Digital LOTO: maintenance request → isolate energy sources →
  verify zero-energy → apply personal locks → work permit →
  restore in reverse order.
- Each step logged with operator ID, timestamp, verification value.
- Cannot start work until all energy sources verified isolated.
**Acceptance criteria:**
- Any energy source re-energized during work triggers immediate
  alarm and machine stop.
- Complete LOTO record exportable for audit (who/when/what).

### US-032 — Role-based access: operator/engineer/admin
**Persona:** Plant IT security administrator.
**Context:** Operators need recipe select + start/stop; engineers
need tuning + forcing; admins need firmware + user management.
**Requirements:**
- Three roles with granular permissions: view, operate, tune,
  force, configure, admin.
- Role assignment per user; session timeout; re-auth for
  privileged actions.
- All privileged actions logged (who/what/when/result).
**Acceptance criteria:**
- Operator cannot access force/stimulate or parameter edit.
- Admin can grant/revoke roles; changes logged immutably.

### US-033 — Signed firmware/package updates with rollback
**Persona:** OT security engineer.
**Context:** PLC logic updates must be authenticated; failed update
must not brick the controller.
**Requirements:**
- Package signature verification before load (Ed25519/RSA).
- A/B partition: new version tested before commit; auto-rollback
  on health-check failure.
- Update log: version, signer, timestamp, result.
**Acceptance criteria:**
- Unsigned/tampered package rejected with clear error.
- Failed update → previous version restored automatically within
  watchdog timeout.

### US-034 — Audit trail for all control changes
**Persona:** Quality/regulatory engineer (FDA, ISO 9001).
**Context:** Every parameter change, recipe download, force, and
program change must be attributable and immutable.
**Requirements:**
- Append-only audit log: timestamp, user, action, old/new value,
  result.
- Log covers: parameter writes, recipe changes, forces, program
  downloads, user logins, role changes.
- Export in standard format (CSV/JSON) for QMS integration.
**Acceptance criteria:**
- Audit log cannot be modified or deleted via any interface.
- Retention configurable (e.g., 7 years); auto-archive supported.

### US-035 — Network segmentation and firewall rules
**Persona:** ICS network administrator.
**Context:** PLC on OT network must not be reachable from IT
network except through defined DMZ services.
**Requirements:**
- Service-level access control: which ports/protocols exposed on
  which interfaces.
- TLS/mTLS for remote access; certificate management.
- Unusual-access detection: alert on connections from unexpected
  subnets or at unusual times.
**Acceptance criteria:**
- Only defined services reachable; all others dropped silently.
- Remote access requires valid certificate + logged session.

### US-036 — Secure boot and measured launch
**Persona:** Embedded security engineer.
**Context:** Controller firmware must verify boot chain integrity;
physical access should not allow malicious firmware.
**Requirements:**
- Secure boot: verify bootloader → kernel → runtime signatures.
- Measured boot: log hash of each stage to tamper-evident storage.
- Debug/JTAG disable in production builds.
**Acceptance criteria:**
- Modified firmware rejected at boot; fallback to recovery image.
- Boot measurements exportable for compliance attestation.

---

## Domain 5 — Debugging, Test & Commissioning (US-037 – US-045)

### US-037 — Live force/stimulate with safety expiry
**Persona:** Commissioning technician.
**Context:** Need to force a DI to test logic, but forget to clear
it — machine runs with simulated input overnight.
**Requirements:**
- Force commands accept optional TTL (e.g., 60 s); auto-clear on
  expiry with log entry.
- Force state visible in watch pane: value, source (user/auto),
  remaining time.
- Bulk force-clear command with confirmation.
**Acceptance criteria:**
- Expired forces auto-clear; event logged with original user.
- No force persists across PLC restart (or explicitly configured
  retain with warning).

### US-038 — Watch window with trends and alarms
**Persona:** Process operator monitoring multiple variables.
**Context:** Need to watch 20+ tags simultaneously; spot trends and
alarms without opening trend charts.
**Requirements:**
- Watch list: add/remove variables, pin important ones, reorder.
- Sparkline history per variable (last N samples).
- Color coding: normal/warning/alarm ranges per variable.
- Watch list persists across sessions (saved with project).
**Acceptance criteria:**
- 50+ variables update at scan rate without UI lag.
- Export watch list to CSV for offline analysis.

### US-039 — Breakpoint and single-step debugging
**Persona:** Software engineer debugging complex FBD logic.
**Context:** Logic behaves unexpectedly; need to step through
execution to find the fault.
**Requirements:**
- Breakpoint on FBD block execution, ST line, or LD rung.
- Single-step: next block/statement/instruction.
- Variable inspection at breakpoint: all locals, inputs, outputs.
- Conditional breakpoints (e.g., break when Temp > 100).
**Acceptance criteria:**
- Breakpoint stops scan at exact point; resume continues cleanly.
- Conditional breakpoint evaluates without stopping if false.

### US-040 — Digital twin simulation with simulated plant
**Persona:** Controls engineer validating logic before hardware.
**Context:** No physical machine yet; need to test logic against
simulated process responses.
**Requirements:**
- Plant simulator: transfer functions, time delays, saturation,
  noise injection for temperature/pressure/level loops.
- Simulation mode: PLC runs real logic, I/O connected to simulator
  instead of field devices.
- Scenario playback: record real process data, replay through
  simulator for regression testing.
**Acceptance criteria:**
- Logic validated against simulator before hardware arrival.
- Simulation mode clearly indicated in UI (cannot confuse with
  live mode).

### US-041 — Hardware-in-the-loop (HIL) test bench
**Persona:** Test engineer validating control system.
**Context:** Real PLC connected to simulated plant via real I/O;
need automated test sequences with pass/fail criteria.
**Requirements:**
- HIL test framework: scripted test cases (stimulus → expected
  response → assert).
- Real I/O used for actuation; simulator provides sensor feedback.
- Test report: pass/fail per case, timing measurements, logs.
**Acceptance criteria:**
- Regression suite runs unattended; report generated automatically.
- Failed test captures full I/O trace for analysis.

### US-042 — Online change with minimal disruption
**Persona:** Maintenance engineer updating logic on running machine.
**Context:** Need to fix logic bug without stopping production;
only one program section needs update.
**Requirements:**
- Partial download: update single POU without stopping others.
- State preservation: FB instances retain state across download.
- Validation: syntax/semantic check before commit; rollback on
  failure.
**Acceptance criteria:**
- Update completes < 5 s; machine continues running.
- Failed update → automatic rollback to previous version.

### US-043 — Shadow mode: run new logic alongside old
**Persona:** Process engineer validating new control strategy.
**Context:** Want to test new PID tuning or control algorithm
without affecting production.
**Requirements:**
- Shadow execution: new logic runs on same inputs, outputs
  compared but not actuated.
- Difference monitoring: alarm when shadow vs. live diverge
  beyond threshold.
- Promotion path: after validation period, shadow becomes live.
**Acceptance criteria:**
- Shadow mode has zero effect on physical outputs.
- Divergence report shows where/when strategies differ.

### US-044 — Automated regression test suite
**Persona:** Quality engineer maintaining PLC logic.
**Context:** Logic changes must not break existing functionality;
need automated tests that run on every change.
**Requirements:**
- Test cases: ST/FBD programs with input scenarios and expected
  outputs.
- Test runner: executes all cases, reports pass/fail, timing.
- Integration with CI/CD: tests run on commit, results reported.
**Acceptance criteria:**
- Full regression suite runs in < 5 min.
- Failed test identifies exact scenario and expected vs. actual.

### US-045 — Trace/logging with trigger conditions
**Persona:** Troubleshooting engineer investigating intermittent fault.
**Context:** Fault happens randomly; need to capture all relevant
variables when trigger condition occurs.
**Requirements:**
- Trace buffer: circular buffer of variable values with timestamps.
- Trigger: start trace on condition (e.g., Temp > limit), stop
  after N samples or second condition.
- Pre-trigger buffer: capture data before trigger (e.g., 100
  samples before event).
**Acceptance criteria:**
- Trigger captures pre- and post-event data automatically.
- Trace exportable for analysis in external tools.

---

## Domain 6 — Data, Alarms & Historians (US-046 – US-054)

### US-046 — Central alarm management with shelving and suppression
**Persona:** Operations supervisor managing alarm flood.
**Context:** During startup, hundreds of alarms activate;
operator needs to shelve known-benign alarms temporarily.
**Requirements:**
- Alarm FB per condition: set, acknowledge, clear, first-out
  indication, shelving (time-limited suppression).
- Alarm priority: critical/warning/info with configurable
  escalation.
- Alarm summary: active count by priority, oldest unacked,
  flood detection (>N alarms/min → general alarm).
**Acceptance criteria:**
- Shelved alarm auto-restores after timeout; cannot shelve
  safety-critical alarms.
- Alarm state transitions logged with timestamp and user.

### US-047 — OEE tracking: availability, performance, quality
**Persona:** Plant manager measuring production efficiency.
**Context:** Need daily/shift OEE: uptime %, ideal vs. actual
cycle time, good vs. total parts.
**Requirements:**
- Availability: planned production time vs. actual run time
  (downtime reasons tracked).
- Performance: ideal cycle time / actual cycle time.
- Quality: good parts / total parts (reject counter).
- OEE = A × P × Q; displayed on HMI, logged per shift.
**Acceptance criteria:**
- OEE calculated automatically per shift; exportable to MES.
- Downtime reason codes captured for Pareto analysis.

### US-048 — Shift reports and production counters
**Persona:** Shift supervisor filing daily production report.
**Context:** Need end-of-shift summary: parts produced, rejects,
downtime, top alarms, energy consumed.
**Requirements:**
- Production counters: good parts, rejects, scrap by reason.
- Shift timer: auto-reset at shift change (configurable schedule).
- Report generation: PDF/CSV with all metrics + alarm summary.
- Manual entries: operator comments, downtime reasons.
**Acceptance criteria:**
- Report generated automatically at shift end; available on HMI
  and via file export.
- Historical reports queryable by date range.

### US-049 — Recipe management with versioning and approval
**Persona:** Process engineer managing product recipes.
**Context:** Hundreds of recipes; need version control, approval
workflow, and rollback capability.
**Requirements:**
- Recipe storage: parameter sets with name, version, author,
  timestamp, approval status.
- Download to PLC: validate parameters, checksum, commit with
  backup of previous.
- Rollback: restore any previous version instantly.
- Audit trail: who changed what, when, why.
**Acceptance criteria:**
- Only approved recipes can be downloaded to production.
- Recipe change during production requires confirmation +
  impact assessment display.

### US-050 — Batch record and genealogy tracking
**Persona:** Quality engineer in regulated industry (FDA, GMP).
**Context:** Every batch must have complete record: ingredients,
process parameters, operator actions, deviations.
**Requirements:**
- Batch ID generation and tracking throughout process.
- Automatic parameter logging: all setpoints, actuals, alarms
  with timestamps.
- Electronic signature for critical steps (operator, reviewer,
  approver).
- Deviations: any out-of-spec parameter flagged and recorded.
**Acceptance criteria:**
- Complete batch record exportable for regulatory audit.
- Missing required data prevents batch release.

### US-051 — SPC charting: X-bar, R, p, c charts
**Persona:** Quality engineer monitoring process capability.
**Context:** Need statistical process control charts to detect
trends before parts go out of spec.
**Requirements:**
- Real-time SPC: X-bar/R for variables, p/c for attributes.
- Control limits: calculated from data or specified.
- Western Electric rules: out-of-control detection (1 point
  beyond 3σ, 2 of 3 beyond 2σ, etc.).
- Chart display on HMI; data exportable.
**Acceptance criteria:**
- Out-of-control condition alarms within 1 sample.
- Charts retain last N subgroups; exportable to Minitab/JMP.

### US-052 — Downtime tracking with reason codes and Pareto
**Persona:** Maintenance manager reducing unplanned downtime.
**Context:** Need to know why machine stops, how long, and
which reasons dominate.
**Requirements:**
- Downtime event: start/stop time, reason code, operator,
  category (planned/unplanned).
- Pareto chart: top reasons by frequency and duration.
- Trend analysis: downtime over time, by shift, by product.
- Integration with CMMS for work order generation.
**Acceptance criteria:**
- Every stop reason captured; uncategorized stops flagged.
- Pareto chart auto-updates; exportable for management review.

### US-053 — Energy consumption monitoring and cost allocation
**Persona:** Facility manager tracking energy costs.
**Context:** Need per-machine/per-area energy usage for cost
allocation and efficiency programs.
**Requirements:**
- Energy metering: kWh, kW, power factor per circuit/machine.
- Time-of-use rates: cost calculation based on tariff schedule.
- Allocation: energy cost per product/unit produced.
- Anomaly detection: consumption deviating from baseline.
**Acceptance criteria:**
- Energy cost per unit calculated automatically.
- Unusual consumption (e.g., compressor running idle) alarmed.

### US-054 — Historian with compression and export
**Persona:** Process engineer analyzing long-term trends.
**Context:** Need to store process data for months/years;
full-resolution data is too large.
**Requirements:**
- Time-series database: tag name, timestamp, value, quality.
- Deadband compression: store only on change or threshold.
- Retention policies: raw data 30 days, hourly averages 1 year,
  daily averages forever.
- Query interface: retrieve data by tag + time range.
- Export: CSV, OPC-UA, MQTT for external systems.
**Acceptance criteria:**
- 1 year of data for 1000 tags in < 10 GB.
- Query response < 5 s for typical range.

---

## Domain 7 — Network & Protocol Integration (US-055 – US-063)

### US-055 — MQTT telemetry to cloud platform
**Persona:** IIoT engineer connecting machines to AWS/Azure.
**Context:** Machine data (OEE, alarms, parameters) must publish
to cloud MQTT broker for dashboards and analytics.
**Requirements:**
- MQTT client block: publish tags on change or interval.
- Topic structure: `plant/line/machine/tag` hierarchy.
- QoS 1 for critical data, QoS 0 for telemetry.
- Store-and-forward: buffer during network outage, flush on
  reconnect.
- TLS/mTLS authentication.
**Acceptance criteria:**
- Data published within 1 s of change (configurable).
- Network outage: no data loss; buffer flushes on reconnect.

### US-056 — OPC-UA server exposing PLC tags to SCADA
**Persona:** System integrator connecting to enterprise SCADA.
**Context:** SCADA system (WinCC, Ignition, PI) needs read/write
access to PLC tags via standard OPC-UA.
**Requirements:**
- OPC-UA server: expose tags as nodes with proper data types.
- Access control: read-only vs. read-write per client certificate.
- Subscriptions: client subscribes to tag changes, server pushes
  updates.
- Alarms & conditions: expose alarms as OPC-UA alarm objects.
**Acceptance criteria:**
- SCADA client connects and browses tag tree automatically.
- Write access restricted by certificate; unauthorized writes
  rejected and logged.

### US-057 — Modbus TCP master for VFD/power meter polling
**Persona:** Electrical engineer integrating drives and meters.
**Context:** Multiple VFDs and power meters on Modbus TCP;
need cyclic polling of speed, current, power, energy.
**Requirements:**
- Modbus TCP client block: poll register maps on configurable
  cycle.
- Data mapping: register → process variable with scaling.
- Error handling: timeout, retry, comm-fail flag per device.
- Multiple devices: independent polling, parallel requests.
**Acceptance criteria:**
- 50 devices polled at 1 s cycle without blocking.
- Device failure isolated; others continue polling.

### US-058 — Modbus RTU slave for legacy HMI integration
**Persona:** Integrator connecting to legacy SCADA.
**Context:** Old SCADA only speaks Modbus RTU; need PLC data
exposed as Modbus slave registers.
**Requirements:**
- Modbus RTU slave: holding/input registers, coils, discrete
  inputs mapped to PLC tags.
- Register map configurable; scaling and data type conversion.
- Multiple slave IDs on same bus (RS-485 multidrop).
- Exception handling: illegal address, illegal function.
**Acceptance criteria:**
- Legacy SCADA reads/writes PLC tags transparently.
- Register map changes take effect without PLC restart.

### US-059 — REST/JSON API for custom applications
**Persona:** Software developer building custom HMI or MES.
**Context:** Need programmatic access to PLC tags, alarms, and
commands from web/mobile apps.
**Requirements:**
- REST API: GET/POST tags, alarms, commands.
- JSON format; standard HTTP methods and status codes.
- Authentication: API key or OAuth2.
- Rate limiting and request logging.
- WebSocket endpoint for real-time subscriptions.
**Acceptance criteria:**
- Any tag readable/writable via REST with proper auth.
- Real-time updates via WebSocket at configurable rate.

### US-060 — S7 protocol communication (Siemens PLC peer)
**Persona:** System integrator with mixed Siemens/Beckhoff cell.
**Context:** Need data exchange with Siemens S7-1200/1500 PLCs;
Siemens side may only support S7 protocol.
**Requirements:**
- S7 client: read/write DB blocks, markers, inputs/outputs.
- S7 server: expose TetherPLC data as S7-accessible blocks.
- Data type mapping: S7 types ↔ IEC types.
- Connection management: keep-alive, reconnect on failure.
**Acceptance criteria:**
- Bidirectional data exchange with Siemens PLC verified.
- Connection survives network hiccups; auto-reconnect.

### US-061 — BACnet integration for building systems
**Persona:** Building automation engineer.
**Context:** HVAC, lighting, access control on BACnet; PLC must
read/write BACnet objects (analog values, binary inputs,
schedules).
**Requirements:**
- BACnet client: read/write properties, COV subscriptions.
- BACnet server: expose PLC data as BACnet objects.
- Device instance management; BBMD for routing.
- Schedule/calendar objects for time-based control.
**Acceptance criteria:**
- PLC reads/writes BACnet devices on same subnet.
- COV notifications received within configured interval.

### US-062 — File-based data exchange (CSV, XML, JSON)
**Persona:** IT/OT integration engineer.
**Context:** Need to exchange data with enterprise systems that
only support file interfaces (legacy ERP, reporting tools).
**Requirements:**
- File write: generate CSV/JSON/XML reports on schedule.
- File read: parse incoming files for commands/parameters.
- File transfer: FTP/SFTP to remote systems.
- File watching: trigger actions on file arrival.
- Format templates: configurable field mapping.
**Acceptance criteria:**
- Scheduled reports generated automatically; files transferred
  reliably.
- File errors (malformed, missing fields) logged with details.

### US-063 — Time sync: NTP, PTP, and IEC 61850-9-3
**Persona:** Power/utility automation engineer.
**Context:** Substation automation requires microsecond-accurate
time sync for event correlation and sequence of events.
**Requirements:**
- NTP client: sync to enterprise time server.
- PTP (IEEE 1588): sub-microsecond sync for protection devices.
- IEC 61850-9-3: precision time protocol for substation devices.
- Time quality indication: traceable, holdover, free-running.
- Leap second handling.
**Acceptance criteria:**
- Time sync accuracy < 1 ms for NTP, < 1 µs for PTP.
- Time quality flag indicates sync status; alarms on loss.

---

## Domain 8 — HMI & Operator UX (US-064 – US-072)

### US-064 — Andon stack light with alarm-state mapping
**Persona:** Production supervisor monitoring line status.
**Context:** Stack light tower shows machine state: green=run,
yellow=warning, red=alarm, blue=changeover.
**Requirements:**
- State machine: inputs (running, warning, alarm, fault,
  maintenance) → outputs (5 stack light channels).
- Flash patterns: steady, slow flash, fast flash per state.
- Manual override: operator can force specific light state.
- Integration with alarm system for automatic state.
**Acceptance criteria:**
- Light state always reflects actual machine condition.
- Manual override times out; cannot mask critical alarms.

### US-065 — Recipe selection screen with preview and validation
**Persona:** Operator selecting product recipe on HMI.
**Context:** Operator picks recipe from list, sees parameters,
confirms before download.
**Requirements:**
- Recipe browser: searchable list, filter by product family.
- Preview: key parameters displayed (SP, speeds, temperatures).
- Validation: check against machine limits before allowing
  selection.
- Download: single button, progress indicator, confirmation.
- Current recipe display: always visible on main screen.
**Acceptance criteria:**
- Invalid recipe cannot be downloaded; error message explains
  which parameter is out of range.
- Recipe change logged with operator ID and timestamp.

### US-066 — Mobile-responsive operator interface
**Persona:** Floor supervisor with tablet/phone.
**Context:** Need to check machine status, acknowledge alarms,
start/stop from anywhere on floor.
**Requirements:**
- Responsive web UI: adapts to tablet/phone screen sizes.
- Touch-friendly controls: large buttons, swipe gestures.
- Real-time updates: status changes push to mobile.
- Secure access: authentication, session timeout.
- Offline mode: cached data when network unavailable.
**Acceptance criteria:**
- All critical functions available on mobile.
- UI usable on 7" tablet and phone without pinch-zoom.

### US-067 — Multi-language HMI with operator prompts
**Persona:** Global manufacturer with operators in multiple countries.
**Context:** Operators speak different languages; need UI in
local language with guided workflows.
**Requirements:**
- Language selection: per-user or per-terminal.
- Translation files: external, updatable without code change.
- Guided workflows: step-by-step wizards for common tasks
  (recipe change, maintenance, troubleshooting).
- Text-to-speech for critical alarms (optional).
**Acceptance criteria:**
- All UI strings translatable; new language = new file only.
- Guided workflows reduce training time; measure task completion.

### US-068 — Alarm banner with priority and quick actions
**Persona:** Machine operator responding to alarms.
**Context:** Alarm activates; operator needs to see it immediately,
understand severity, and take action.
**Requirements:**
- Banner at top of screen: alarm message, priority color,
  timestamp, duration.
- Quick actions: acknowledge, silence (temporary), help link.
- Alarm list: all active alarms, sortable by priority/time.
- Alarm history: past alarms with resolution time.
**Acceptance criteria:**
- Critical alarms cannot be silenced; must be acknowledged.
- Alarm banner persists until acknowledged; visual/audio alert.

### US-069 — Kiosk mode for dedicated operator terminals
**Persona:** Production manager deploying fixed operator stations.
**Context:** Dedicated HMI terminal at each machine; operators
should only see machine controls, not engineering tools.
**Requirements:**
- Kiosk mode: lock UI to operator functions only.
- No access to: parameters, forcing, programming, diagnostics.
- Session management: auto-logout after inactivity.
- Remote management: admin can unlock for maintenance.
**Acceptance criteria:**
- Operator cannot access engineering functions without admin
  credentials.
- Kiosk mode survives power cycle; auto-starts on boot.

### US-070 — Dark mode and high-contrast themes
**Persona:** Control room operator working night shifts.
**Context:** Dark control room; bright HMI screens cause eye strain
and reduce night vision.
**Requirements:**
- Dark theme: low-light colors, reduced blue light.
- High-contrast mode: for visually impaired operators.
- Theme selection: per-user preference, saved with profile.
- Automatic: dark mode at night (time-based or ambient light).
**Acceptance criteria:**
- Dark mode reduces eye strain; user preference persists.
- All text readable in both modes; no information lost.

### US-071 — Guided maintenance wizards
**Persona:** Maintenance technician performing routine tasks.
**Context:** Preventive maintenance has many steps; technician
needs checklist with verification.
**Requirements:**
- Maintenance wizard: step-by-step checklist with pass/fail.
- Sensor readings: prompt for measurements, validate ranges.
- Photo capture: attach photos of work performed.
- Signature: technician signs off on completion.
- History: previous maintenance records viewable.
**Acceptance criteria:**
- Cannot skip required steps; must verify each.
- Maintenance record links to asset history for trend analysis.

### US-072 — Operator performance metrics and feedback
**Persona:** Production supervisor coaching operators.
**Context:** Want to track operator efficiency, identify training
needs, recognize top performers.
**Requirements:**
- Metrics: cycle time, rejects, downtime response, alarm
  response time.
- Leaderboard: team comparison (anonymized option).
- Feedback: positive reinforcement for good performance.
- Training flags: identify operators needing additional training.
**Acceptance criteria:**
- Metrics calculated automatically; no manual data entry.
- Privacy respected: individual data only to authorized managers.

---

## Domain 9 — Energy & Power (US-073 – US-081)

### US-073 — Power quality monitoring: THD, sag/swell, flicker
**Persona:** Electrical engineer investigating power issues.
**Context:** Equipment malfunctions; suspect poor power quality
(harmonics, voltage sags, flicker).
**Requirements:**
- Power quality meter: V, I, P, Q, S, PF, THD per phase.
- Event capture: sag/swell/interruption with timestamp and
  magnitude.
- Harmonic analysis: spectrum up to 50th harmonic.
- Trending: PQ parameters over time for analysis.
**Acceptance criteria:**
- Events captured with <1 cycle resolution.
- Data exportable to power quality analysis software.

### US-074 — Demand charge management with load shedding
**Persona:** Facility manager reducing peak demand charges.
**Context:** Utility charges based on peak kW demand; need to
shed non-critical loads during peaks.
**Requirements:**
- Demand monitoring: rolling 15-min average kW.
- Load priorities: critical, important, deferrable.
- Shedding logic: when demand approaches limit, shed deferrable
  loads in sequence.
- Restoration: loads restored when demand drops.
- Notification: alert operators of shedding events.
**Acceptance criteria:**
- Peak demand reduced by configured amount; verify with utility
  bill.
- Critical loads never shed; deferrable loads restored
  automatically.

### US-075 — Peak shaving with battery energy storage
**Persona:** Energy manager integrating BESS.
**Context:** Battery storage charges off-peak, discharges during
peak to reduce demand charges.
**Requirements:**
- BESS control: charge/discharge based on time-of-use rates.
- SOC monitoring: state of charge, health, temperature.
- Dispatch algorithm: optimize charge/discharge for cost
  savings.
- Grid services: frequency regulation, voltage support (if
  enabled).
**Acceptance criteria:**
- Demand charges reduced measurably; ROI tracked.
- Battery protected from over-charge/discharge; thermal
  management active.

### US-076 — Solar PV monitoring and curtailment
**Persona:** Renewable energy engineer.
**Context:** Solar array generates power; need monitoring,
forecasting, and grid-compliant curtailment.
**Requirements:**
- PV monitoring: DC/AC power, voltage, current, efficiency.
- Irradiance sensors: predict output, detect soiling.
- Curtailment: reduce output on grid signal or local control.
- Anti-islanding: detect grid loss, disconnect immediately.
- Revenue tracking: energy produced, exported, self-consumed.
**Acceptance criteria:**
- Anti-islanding tested and certified.
- Curtailment responds to grid signal within required time.

### US-077 — Generator auto-start with load testing
**Persona:** Facilities engineer managing backup power.
**Context:** Generator must start automatically on utility loss,
transfer load, and run periodic tests.
**Requirements:**
- Auto-start: utility loss → start generator → transfer load.
- Load test: weekly/monthly run under load for specified
  duration.
- Fuel management: monitor level, auto-refuel alerts.
- Maintenance: run hours, start count, oil pressure/temp.
- Transfer switch control: utility ↔ generator with proper
  sequencing.
**Acceptance criteria:**
- Transfer completes <10 s; no load interruption.
- Load test runs automatically; failures alarmed.

### US-078 — EV charging station management
**Persona:** Fleet/facility manager with EV charging.
**Context:** Multiple EV chargers; need load management to avoid
demand charges and ensure fair access.
**Requirements:**
- Charger control: enable/disable, set current limit.
- Load management: dynamic current allocation based on total
  demand.
- User authentication: RFID, app, or credit card.
- Billing: per-session cost calculation.
- Scheduling: charge during off-peak hours.
**Acceptance criteria:**
- Total charging load never exceeds configured limit.
- Users charged correctly; revenue tracked.

### US-079 — Power factor correction with switched capacitors
**Persona:** Electrical engineer improving power factor.
**Context:** Low power factor incurs utility penalties; need
automatic capacitor bank switching.
**Requirements:**
- PF monitoring: real-time PF per feeder.
- Capacitor bank control: switch steps in/out to maintain target
  PF.
- Harmonic filtering: detuned reactors or active filters if
  needed.
- Switching transients: zero-crossing or soft-start to minimize
  inrush.
**Acceptance criteria:**
- PF maintained >0.95 lagging; penalties eliminated.
- Switching transients don't trip protection.

### US-080 — Transformer loading and thermal monitoring
**Persona:** Substation engineer.
**Context:** Transformers have thermal limits; need monitoring
to prevent overload and predict failure.
**Requirements:**
- Loading monitor: % of rated capacity, top-oil temp, winding
temp (if available).
- Thermal model: calculate hotspot temperature from load +
  ambient.
- Alarm: overload warning, emergency trip at limit.
- Loss calculation: no-load + load losses for efficiency.
- Dissolved gas analysis integration (if available).
**Acceptance criteria:**
- Hotspot temp calculated accurately; alarms before damage.
- Loading trends identify need for upgrade before failure.

### US-081 — Arc flash labeling and incident energy calculation
**Persona:** Electrical safety engineer.
**Context:** Arc flash hazard analysis requires incident energy
at each panel; labels must be accurate and current.
**Requirements:**
- Arc flash study: calculate incident energy at each location.
- Label generation: print labels with hazard category, PPE
  requirements, approach boundaries.
- Data integration: pull from relay settings, transformer data,
  cable lengths.
- Update workflow: recalculate when system changes.
**Acceptance criteria:**
- Labels match NFPA 70E requirements.
- Incident energy calculations verified against study.

---

## Domain 10 — Water/Wastewater & Environmental (US-082 – US-090)

### US-082 — pH/ORP control for chemical dosing
**Persona:** Water treatment plant operator.
**Context:** Need precise pH control for disinfection; pH probe
feedback, chemical feed pump control.
**Requirements:**
- pH measurement: temperature-compensated, auto-calibration
  reminder.
- Control: PID or cascade (pH → pump speed via VFD).
- Dosing: acid/base feed with day-tank level management.
- Alarms: pH out of range, probe failure, low chemical.
- Recording: pH trend, dosing totals for compliance.
**Acceptance criteria:**
- pH controlled within ±0.2 units of setpoint.
- Dosing pump protects against run-dry; chemical inventory
  tracked.

### US-083 — Dissolved oxygen control in aquaculture
**Persona:** Aquaculture farm manager.
**Context:** Fish health depends on DO levels; need aeration
control based on DO sensors.
**Requirements:**
- DO monitoring: multiple sensors, redundancy.
- Aeration control: paddlewheel/diffuser speed based on DO
  deficit.
- Alarm: low DO (<4 mg/L), sensor failure.
- Feeding integration: pause feeding if DO low.
- Data logging: DO trends for health records.
**Acceptance criteria:**
- DO maintained >5 mg/L 24/7; alarms on deviation.
- Aeration energy optimized; consumption tracked.

### US-084 — Turbidity and water quality monitoring
**Persona:** Environmental compliance engineer.
**Context:** Discharge permit requires turbidity, TSS, pH
monitoring with reporting.
**Requirements:**
- Multi-parameter sonde: turbidity, pH, DO, conductivity,
  temperature.
- Data validation: sensor cleaning cycle, drift detection.
- Reporting: daily averages, exceedance alerts, regulatory
  reports.
- Integration: SCADA, LIMS, regulatory database.
**Acceptance criteria:**
- Data meets regulatory quality requirements.
- Exceedances alarmed immediately; reports generated
  automatically.

### US-085 — Stormwater monitoring and spill prevention
**Persona:** EHS manager at industrial facility.
**Context:** Storm drains must be protected from spills; need
level monitoring, isolation valves, and alarm.
**Requirements:**
- Storm drain level monitoring: ultrasonic/radar sensors.
- Isolation valves: automatic closure on high level or spill
  detection.
- Spill detection: oil/water interface sensor in drain.
- Alarm: high level, valve failure, sensor fault.
- Rain gauge integration: predict storm events.
**Acceptance criteria:**
- Isolation valves close <30 s on alarm.
- Spill events logged with time, duration, volume estimate.

### US-086 — Leak detection for pipelines and tanks
**Persona:** Pipeline integrity engineer.
**Context:** Product pipelines and storage tanks need leak
detection for safety and environmental compliance.
**Requirements:**
- Leak detection: pressure/flow monitoring, acoustic sensors,
  vapor sensors.
- Location estimation: approximate leak location from sensor
  data.
- Automatic shutdown: close valves on confirmed leak.
- Alarm: leak detected, sensor failure, communication loss.
- Integration: SCADA, emergency response system.
**Acceptance criteria:**
- Leak detected and isolated within configured time.
- False alarms minimized; alarm management prevents flood.

### US-087 — Pump station control with duty/assist rotation
**Persona:** Water/wastewater utilities engineer.
**Context:** Multiple pumps share load; need automatic rotation
to equalize wear and handle varying demand.
**Requirements:**
- Pump control: start/stop based on level/pressure setpoints.
- Duty/assist: lead pump handles demand, assist pumps kick in
  for peak.
- Rotation: automatic alternation to equalize run hours.
- Fault handling: pump failure → automatic switch to standby.
- Level control: maintain level/pressure within band.
**Acceptance criteria:**
- Pumps rotate automatically; run hours balanced within 10 %.
- Pump failure detected and backup started <30 s.

### US-088 — Chlorination and disinfection control
**Persona:** Water treatment operator.
**Context:** Need precise chlorine dosing for disinfection;
CT value (concentration × time) must meet permit.
**Requirements:**
- Chlorine residual monitoring: online analyzer.
- Dosing control: adjust feed rate based on flow + residual.
- CT calculation: concentration × contact time.
- Alarm: low residual, high residual, analyzer failure.
- Calibration: automatic buffer verification, manual calibration
  reminder.
**Acceptance criteria:**
- CT value maintained above permit minimum.
- Residual controlled within ±0.1 mg/L.

### US-089 — SBR (Sequencing Batch Reactor) cycle control
**Persona:** Wastewater process engineer.
**Context:** SBR operates in timed phases: fill, react, settle,
draw, idle — each with specific conditions.
**Requirements:**
- Phase sequencer: timed transitions with sensor verification.
- DO control: aerobic/anaerobic phases with blower control.
- Level control: fill/draw based on level sensors.
- Recipe management: different cycles for different flows.
- Data logging: phase times, DO profiles, effluent quality.
**Acceptance criteria:**
- Cycles execute automatically; transitions verified by sensors.
- Effluent quality meets permit; data logged for compliance.

### US-090 — Compressed air system monitoring and leak detection
**Persona:** Plant energy manager.
**Context:** Compressed air is expensive; leaks waste 20–30 %
of production. Need monitoring and leak detection.
**Requirements:**
- Air consumption monitoring: flow meters per department.
- Leak detection: ultrasonic sensors on pipes/valves.
- Pressure monitoring: header pressure, receiver levels.
- Compressor control: load/unload based on demand.
- Alarm: low pressure, high consumption, leak detected.
**Acceptance criteria:**
- Leaks located and quantified; repair prioritized.
- Energy savings verified; consumption reduced measurably.

---

## Domain 11 — Food/Pharma/Life Sciences (US-091 – US-099)

### US-091 — CIP/SIP cycle automation for bioreactor
**Persona:** Bioprocess engineer.
**Context:** Clean-in-place and sterilize-in-place cycles must
meet validated parameters; deviations require investigation.
**Requirements:**
- CIP cycle: pre-rinse, caustic wash, acid rinse, final rinse —
  each with temperature, conductivity, time parameters.
- SIP cycle: steam sterilization with temperature/pressure
  profile.
- Parameter logging: all critical parameters recorded.
- Batch record: cycle ID, operator, parameters, results.
- Validation: cycle parameters within validated ranges.
**Acceptance criteria:**
- CIP/SIP parameters logged for batch record.
- Deviations flagged; cycle cannot complete with out-of-spec
  parameters.

### US-092 — Cleanroom environmental monitoring
**Persona:** Cleanroom qualification engineer.
**Context:** ISO 14644 classification requires particle count,
pressure differential, temperature, humidity monitoring.
**Requirements:**
- Particle counters: 0.5 µm and 5 µm counts.
- Pressure differential: room-to-corridor, room-to-outside.
- Temperature/humidity: continuous monitoring.
- Alarm: excursion from limits, sensor failure.
- Data logging: continuous recording for certification.
**Acceptance criteria:**
- Environmental parameters within ISO limits; excursions alarmed.
- Data supports cleanroom certification audits.

### US-093 — Lyophilizer (freeze dryer) cycle control
**Persona:** Pharmaceutical process engineer.
**Context:** Freeze-drying cycle has precise temperature, pressure,
and time parameters; product quality depends on accuracy.
**Requirements:**
- Shelf temperature control: multi-zone, ramp/soak profiles.
- Chamber pressure control: vacuum levels per phase.
- Condenser temperature: monitoring and control.
- Endpoint detection: pressure-rise or temperature-rise test.
- Recipe management: validated cycles per product.
- Batch record: all parameters logged with timestamps.
**Acceptance criteria:**
- Cycle parameters within validated ranges; deviations alarmed.
- Batch record complete for regulatory submission.

### US-094 — Fill/finish line with weight check and reject
**Persona:** Packaging engineer in pharma/food.
**Context:** Filling line must verify fill weight on every
container; rejects under/over-filled automatically.
**Requirements:**
- Checkweigher integration: weight signal per container.
- Reject mechanism: pneumatic pusher or diverter.
- Statistics: weight distribution, Cpk calculation.
- Alarm: weight out of range, reject mechanism fault.
- Data logging: weight per container, reject reasons.
**Acceptance criteria:**
- Fill weight within ±1 % of target; Cpk >1.33.
- Reject rate <0.5 %; reject mechanism verified each cycle.

### US-095 — Cold chain monitoring with excursion alerts
**Persona:** Supply chain quality manager.
**Context:** Vaccines/biologics require continuous temperature
monitoring; excursions compromise product.
**Requirements:**
- Temperature monitoring: multiple sensors per shipment.
- Excursion detection: out-of-range temperature for >N minutes.
- Data logging: continuous recording with timestamps.
- Alert system: SMS/email on excursion.
- GPS tracking: location + temperature correlation.
- Chain of custody: who handled, when, where.
**Acceptance criteria:**
- Excursions detected and reported within configured time.
- Complete data logger report for product release.

### US-096 — Allergen segregation and changeover control
**Persona:** Food safety manager.
**Context:** Allergen-containing products must be segregated;
changeovers require cleaning verification.
**Requirements:**
- Allergen tracking: which products contain which allergens.
- Changeover sequence: allergen change → cleaning → verification
  → production.
- Cleaning verification: ATP swab or protein test.
- Line clearance: confirm all allergen materials removed.
- Documentation: changeover records for audit.
**Acceptance criteria:**
- Allergen changeovers documented; cleaning verified.
- Cross-contamination risk minimized; audit trail complete.

### US-097 — Tablet press tooling and weight control
**Persona:** Pharmaceutical manufacturing engineer.
**Context:** Tablet weight uniformity critical; press force,
die fill, and compression monitored.
**Requirements:**
- Press monitoring: main compression force, pre-compression,
  ejection force.
- Weight control: individual tablet weight (checkweigher).
- Tooling management: die set ID, punch count, wear tracking.
- Alarm: force out of range, weight out of range, tooling wear.
- Batch record: press parameters, weight data, rejects.
**Acceptance criteria:**
- Tablet weight uniformity RSD <2 %.
- Tooling wear tracked; replacement prompted before failure.

### US-098 — Incubator CO2/O2 control for cell culture
**Persona:** Cell culture lab manager.
**Context:** Cell culture incubators need precise CO2/O2
temperature control for cell growth.
**Requirements:**
- CO2 control: infrared sensor, solenoid valve dosing.
- O2 control: for hypoxia work, precise O2/N2 mixing.
- Temperature control: PID, ±0.1 °C accuracy.
- Humidity: water pan level monitoring.
- Decontamination cycle: high-temp or H2O2 vapor.
- Data logging: continuous recording for GLP.
**Acceptance criteria:**
- CO2/O2/temperature within ±0.1 % / ±0.2 % / ±0.1 °C.
- Data logger supports GLP compliance.

### US-099 — Serial number tracking and genealogy
**Persona:** Medical device manufacturer quality engineer.
**Context:** Each device must have unique serial number;
complete manufacturing history required for traceability.
**Requirements:**
- Serial number generation: unique, sequential, with check digit.
- Station tracking: each work center scans serial, records
  operations.
- Component genealogy: which components went into which device.
- Test results: link test data to serial number.
- Recall capability: identify all devices with suspect component.
**Acceptance criteria:**
- Every device traceable to components and test results.
- Recall can identify affected devices in <1 hour.

---

## Domain 12 — Machine Tools & Discrete Mfg (US-100 – US-108)

### US-100 — Tool changer with broken-tool detection
**Persona:** CNC programmer.
**Context:** Automatic tool changer must verify tool presence
and detect broken tools before machining.
**Requirements:**
- Tool changer sequence: select tool, move to change position,
  unclamp, swap, clamp, verify.
- Tool verification: tool-present sensor, broken-tool detector
  (laser or touch probe).
- Tool data: offset, length, diameter, wear compensation.
- Alarm: tool missing, broken tool detected, changer fault.
- Integration: tool path generation (CAM), tool life tracking.
**Acceptance criteria:**
- Tool change completes <10 s; verification 100 % reliable.
- Broken tool detected before cutting; alarm + stop.

### US-101 — In-process probing and work offset measurement
**Persona:** Machinist setting up new part.
**Context:** Part position varies; probe must locate features
and update work offsets automatically.
**Requirements:**
- Probe cycle: approach, touch, retract, repeat for multiple
  points.
- Feature recognition: plane, cylinder, hole, boss.
- Work offset calculation: translate/rotate part coordinate
  system.
- Manual vs. automatic: probe program or manual jog.
- Data logging: probe results for process control.
**Acceptance criteria:**
- Probe accuracy ±0.005 mm; offsets updated automatically.
- Probe failure (no contact) aborts with clear alarm.

### US-102 — Tool wear compensation and life tracking
**Persona:** Process engineer optimizing tool life.
**Context:** Tools wear; need to track wear and compensate
dimensions before parts go out of spec.
**Requirements:**
- Wear monitoring: cutting force, vibration, surface finish,
  part dimensions.
- Compensation: adjust tool offset based on wear model.
- Life tracking: parts cut, minutes run, wear rate.
- Prediction: estimate remaining life, prompt replacement.
- Data analysis: correlate wear with cutting parameters.
**Acceptance criteria:**
- Tool wear compensated automatically; part quality maintained.
- Tool life prediction within ±10 % of actual.

### US-103 — Palletizing pattern optimization
**Persona:** Packaging engineer.
**Context:** Boxes arrive in random order; robot must palletize
in optimized pattern for shipping.
**Requirements:**
- Pattern library: predefined patterns for box sizes.
- Box detection: vision system identifies box dimensions.
- Pattern selection: choose best pattern for box/pallet.
- Collision avoidance: verify no interference.
- Layer completion: signal when layer full, start new layer.
**Acceptance criteria:**
- Palletization density >95 % of theoretical maximum.
- Pattern changes without manual intervention.

### US-104 — Sortation system with divert control
**Persona:** Distribution center automation engineer.
**Context:** Parcels need sorting to different destinations;
high-speed diverts must be precise.
**Requirements:**
- Parcel tracking: barcode/RFID read, destination lookup.
- Divert control: pneumatic pusher, paddle, or pop-up.
- Timing: divert when parcel at correct position.
- Merge/diverge: induction control for merges.
- Reject handling: unreadable/unknown destination → manual
  lane.
**Acceptance criteria:**
- Sort accuracy >99.5 % at rated speed.
- Divert timing ±10 ms; no parcel damage.

### US-105 — Robotic welding with seam tracking
**Persona:** Welding engineer.
**Context:** Weld seams vary; robot must track seam and
maintain weld parameters.
**Requirements:**
- Seam tracking: laser/vision sensor follows seam.
- Weld control: voltage, current, wire feed, travel speed.
- Adaptive control: adjust parameters based on seam geometry.
- Quality monitoring: penetration, bead width, porosity.
- Program generation: teach pendant or offline programming.
**Acceptance criteria:**
- Weld quality consistent; defect rate <0.1 %.
- Seam tracking accuracy ±0.5 mm.

### US-106 — Machine vision inspection integration
**Persona:** Quality engineer.
**Context:** Vision system inspects parts for defects; PLC
must trigger inspection and handle results.
**Requirements:**
- Inspection trigger: part present → trigger camera.
- Result handling: pass/fail, defect classification.
- Reject mechanism: pneumatic pusher or diverter.
- Statistics: defect rate, Pareto of defect types.
- Recipe management: inspection parameters per product.
**Acceptance criteria:**
- Inspection cycle time <500 ms; false reject <1 %.
- Defect data logged for SPC and traceability.

### US-107 — Spindle load monitoring and tool breakage detection
**Persona:** Machining cell supervisor.
**Context:** Spindle overload or tool breakage must be detected
immediately to prevent damage.
**Requirements:**
- Load monitoring: spindle current, torque, vibration.
- Breakage detection: sudden load drop or vibration spike.
- Adaptive feed: reduce feed on overload, increase on light
  load.
- Alarm: overload, tool breakage, spindle fault.
- Data logging: load profiles for analysis.
**Acceptance criteria:**
- Tool breakage detected within 1 revolution.
- Spindle protected from overload damage.

### US-108 — Automatic door/gate with safety interlocks
**Persona:** Facility engineer.
**Context:** Overhead doors, gates must open/close safely with
person/vehicle detection.
**Requirements:**
- Door control: open/close buttons, auto-close timer.
- Safety: photocells, edge sensors, loop detectors.
- Interlocks: cannot close if obstruction detected.
- Vehicle detection: loop or radar for truck presence.
- Integration: access control, security system.
**Acceptance criteria:**
- Door never closes on obstruction; safety verified.
- Auto-close timer configurable; manual override available.

---

## Domain 13 — Packaging & Materials Handling (US-109 – US-117)

### US-109 — Case erector and packer integration
**Persona:** Packaging line engineer.
**Context:** Flat boxes must be erected, filled, and sealed
automatically at line rate.
**Requirements:**
- Case erector: magazine feed, erect, bottom seal.
- Fill station: product placement, weight check.
- Top seal: tape or glue application.
- Case tracking: barcode/RFID for serialization.
- Reject handling: damaged cases, over/under-filled.
**Acceptance criteria:**
- Line rate achieved; case quality >99 %.
- Reject rate <1 %; reject mechanism reliable.

### US-110 — Label applicator with verification
**Persona:** Packaging technician.
**Context:** Labels must be applied accurately; misapplied
labels cause rejects and compliance issues.
**Requirements:**
- Label applicator: print, peel, apply with precise timing.
- Verification: vision system checks label presence, position,
  print quality.
- Reject: misapplied labels trigger rework or reject.
- Data matrix/barcode: verification scan after application.
- Recipe management: label templates per product.
**Acceptance criteria:**
- Label placement accuracy ±1 mm; verification 100 %.
- Misapplied labels caught; reject rate <0.5 %.

### US-111 — Weigh-check and checkweigher integration
**Persona:** Quality engineer in food/pharma.
**Context:** Every package must be weighed; out-of-spec rejected
automatically.
**Requirements:**
- Checkweigher: high-speed weight measurement.
- Reject mechanism: pneumatic pusher or diverter.
- Statistics: weight distribution, Cpk calculation.
- Tare management: container weight compensation.
- Data logging: weight per package, reject reasons.
**Acceptance criteria:**
- Weight accuracy ±0.5 g; Cpk >1.33.
- Reject rate <0.5 %; mechanism verified each cycle.

### US-112 — Accumulation and buffer management
**Persona:** Line balancing engineer.
**Context:** Different stations run at different speeds;
need accumulation to prevent starvation/blockage.
**Requirements:**
- Accumulation table: variable speed zones, buffer capacity.
- Level control: maintain buffer level within band.
- Merge/diverge: combine multiple lines or split to multiple.
- Jam detection: no movement for >N seconds → alarm.
- Optimization: minimize buffer while preventing blockage.
**Acceptance criteria:**
- Line efficiency >95 %; blockage <1 per shift.
- Buffer level maintained automatically.

### US-113 — AGV fleet management and traffic control
**Persona:** Intralogistics manager.
**Context:** Multiple AGVs share floor space; need traffic
management to prevent collisions and optimize flow.
**Requirements:**
- Fleet manager: assign tasks, optimize routes.
- Traffic control: intersection management, zone control.
- Collision avoidance: proximity sensors, emergency stop.
- Battery management: charge scheduling, low-battery reroute.
- Integration: WMS, MES for task dispatch.
**Acceptance criteria:**
- Zero collisions; traffic flow optimized.
- AGVs charge automatically; no downtime for battery.

### US-114 — ASRS (Automated Storage/Retrieval) control
**Persona:** Warehouse automation engineer.
**Context:** High-bay warehouse with cranes/shuttles; need
precise storage/retrieval control.
**Requirements:**
- Crane control: X/Y/Z positioning, load handling.
- Shuttle system: horizontal transport between aisles.
- Inventory management: location tracking, FIFO/FEFO.
- Safety: collision avoidance, load verification.
- WMS integration: order fulfillment, putaway.
**Acceptance criteria:**
- Retrieval accuracy 100 %; throughput maximized.
- Safety systems prevent collisions; load verified.

### US-115 — Stretch wrapper with film tracking
**Persona:** Packaging line supervisor.
**Context:** Pallets must be wrapped for shipping; film usage
and wrap quality critical.
**Requirements:**
- Wrapper control: turntable rotation, film carriage, cut/seal.
- Film tracking: remaining film roll, splice detection.
- Pattern control: wrap pattern per product.
- Quality check: wrap tension, overlap verification.
- Data logging: pallets wrapped, film usage.
**Acceptance criteria:**
- Wrap quality consistent; film usage optimized.
- Film splice detected; no wrapping interruption.

### US-116 — Vision-based defect detection and reject
**Persona:** Quality engineer in packaging.
**Context:** Products may have defects (missing label, wrong
color, damage); need 100 % inspection.
**Requirements:**
- Vision system: camera, lighting, image processing.
- Defect classification: type, severity, location.
- Reject mechanism: pneumatic pusher or diverter.
- Statistics: defect rate, Pareto analysis.
- Recipe management: inspection parameters per product.
**Acceptance criteria:**
- Defect detection >99.5 %; false reject <1 %.
- Defect data logged for SPC and traceability.

### US-117 — Case packing with pattern optimization
**Persona:** Packaging engineer.
**Context:** Multiple product sizes; need optimal packing
pattern for each case.
**Requirements:**
- Pattern library: predefined patterns for product types.
- Case management: case size, divider inserts.
- Weight distribution: center of gravity optimization.
- Fill level: maximize case utilization.
- Data tracking: case ID, contents, weight.
**Acceptance criteria:**
- Case utilization >95 %; weight within limits.
- Pattern changes automatically per product.

---

## Domain 14 — Building & Facilities (US-118 – US-126)

### US-118 — HVAC zone control with VAV boxes
**Persona:** Building automation engineer.
**Context:** Multiple zones need individual temperature
control; VAV boxes modulate airflow.
**Requirements:**
- Zone temperature: space sensors, averaging.
- VAV control: damper position, airflow measurement.
- AHU control: supply air temp, fan speed.
- Scheduling: occupancy-based setpoints.
- Integration: BACnet, Modbus to BMS.
**Acceptance criteria:**
- Zone temps within ±1 °F of setpoint.
- Energy consumption optimized; comfort maintained.

### US-119 — Lighting control with daylight harvesting
**Persona:** Energy manager.
**Context:** Need to reduce lighting energy while maintaining
comfort; daylight and occupancy sensors available.
**Requirements:**
- Lighting zones: individual or grouped control.
- Daylight harvesting: dim lights based on natural light.
- Occupancy sensing: PIR/microwave sensors.
- Scheduling: time-based on/off, holiday schedules.
- Integration: BACnet, DALI, KNX.
**Acceptance criteria:**
- Energy savings >30 % vs. constant lighting.
- Occupant comfort maintained; no complaints.

### US-120 — Access control integration with door interlocks
**Persona:** Security integrator.
**Context:** Doors must integrate with access control; mantrap
and turnstile sequences required.
**Requirements:**
- Door control: open/close, lock/unlock.
- Access control: badge reader, biometric, PIN.
- Interlocks: mantrap sequence, anti-passback.
- Tailgating detection: door held open too long.
- Integration: security system, video surveillance.
**Acceptance criteria:**
- Unauthorized access prevented; tailgating alarmed.
- Emergency egress always available.

### US-121 — Fire alarm and life safety integration
**Persona:** Fire protection engineer.
**Context:** Fire alarm system must interface with HVAC,
elevators, doors for life safety.
**Requirements:**
- Fire alarm input: zone alarms, device faults.
- HVAC shutdown: stop air handlers on alarm.
- Elevator recall: return to lobby on alarm.
- Door release: unlock fire doors on alarm.
- Integration: fire alarm panel, BMS.
**Acceptance criteria:**
- Life safety functions verified per NFPA 72.
- Integration tested; no interference with fire alarm.

### US-122 — Elevator group control and destination dispatch
**Persona:** Elevator consultant.
**Context:** Multiple elevators serve building; need efficient
dispatch and group control.
**Requirements:**
- Group control: assign calls to optimal elevator.
- Destination dispatch: passenger selects floor, system routes.
- Energy management: idle car parking, regeneration.
- Safety: door interlocks, emergency stop.
- Integration: building management system.
**Acceptance criteria:**
- Average wait time reduced >20 % vs. conventional.
- Energy consumption optimized.

### US-123 — Parking guidance and EV charging
**Persona:** Parking facility manager.
**Context:** Need to guide drivers to available spaces; EV
charging management required.
**Requirements:**
- Space detection: ultrasonic/magnetic sensors per space.
- Guidance system: variable message signs, app integration.
- EV charging: charger control, billing, load management.
- Revenue management: pricing, validation, payment.
- Integration: parking management system.
**Acceptance criteria:**
- Space occupancy accuracy >99 %; guidance effective.
- EV charging revenue tracked; load managed.

### US-124 — Building energy management with demand response
**Persona:** Facility energy manager.
**Context:** Need to reduce energy costs; demand response
programs available.
**Requirements:**
- Energy monitoring: whole-building and per-system.
- Demand response: shed loads on utility signal.
- Peak shaving: battery storage, load scheduling.
- Solar integration: PV generation, net metering.
- Reporting: energy use, cost, carbon footprint.
**Acceptance criteria:**
- Demand charges reduced; DR events automated.
- Energy reporting supports sustainability goals.

### US-125 — Water leak detection and automatic shutoff
**Persona:** Building facilities manager.
**Context:** Water leaks cause major damage; need early
detection and automatic shutoff.
**Requirements:**
- Leak detection: moisture sensors, flow monitoring.
- Automatic shutoff: close main valve on leak detection.
- Location identification: which zone/fixture leaking.
- Alert system: notify maintenance, occupants.
- Integration: building automation system.
**Acceptance criteria:**
- Leaks detected <1 minute; water damage minimized.
- False alarms <1 per month.

### US-126 — Elevator modernisation with PLC-based control
**Persona:** Elevator modernization contractor.
**Context:** Replace legacy relay control with modern PLC;
maintain safety certifications.
**Requirements:**
- Safety circuits: maintain existing safety relays or upgrade
  to safety PLC.
- Motion control: VFD drives, encoder feedback.
- Door control: landing doors, car doors, interlocks.
- Group control: multiple cars, destination dispatch.
- Emergency systems: firefighter recall, emergency power.
- Compliance: ASME A17.1, EN 81-20/50.
**Acceptance criteria:**
- Modernization passes safety inspection.
- Performance equal or better than original.

---

## Domain 15 — Operations, Maintenance & Fleet Management (US-127 – US-135)

### US-127 — Predictive maintenance with vibration analysis
**Persona:** Maintenance planner.
**Context:** Unplanned downtime costs; need to predict failures
before they occur.
**Requirements:**
- Vibration monitoring: accelerometers on rotating equipment.
- FFT analysis: frequency spectrum, bearing fault detection.
- Trend analysis: vibration levels over time.
- Alarm: vibration exceeding thresholds.
- Integration: CMMS for work order generation.
**Acceptance criteria:**
- Failures predicted >2 weeks in advance.
- False alarms <5 %; maintenance optimized.

### US-128 — CMMS integration for work order management
**Persona:** Maintenance coordinator.
**Context:** Maintenance activities must be tracked in CMMS;
need bidirectional integration.
**Requirements:**
- Work order creation: from PLC alarms or manual.
- Status updates: work started, parts used, completed.
- Parts inventory: track usage, reorder points.
- Labor tracking: technician time, skills required.
- Reporting: MTBF, MTTR, maintenance costs.
**Acceptance criteria:**
- Work orders created automatically from alarms.
- Maintenance data flows to CMMS in real-time.

### US-129 — Remote diagnostics and support bundle
**Persona:** OEM support engineer.
**Context:** Customer machine has issue; need remote access
to diagnose without site visit.
**Requirements:**
- Remote access: secure VPN or cloud connection.
- Diagnostics: real-time data, logs, alarms.
- Support bundle: complete system snapshot for analysis.
- Screen sharing: view HMI remotely.
- Security: authenticated access, session logging.
**Acceptance criteria:**
- 80 % of issues resolved remotely.
- Support bundle provides complete diagnostic info.

### US-130 — Fleet health dashboard across multiple sites
**Persona:** Operations director managing multiple plants.
**Context:** Need centralized view of all machines across
sites for performance and issues.
**Requirements:**
- Fleet dashboard: status of all machines/sites.
- KPI aggregation: OEE, production, quality, energy.
- Alert consolidation: critical issues across fleet.
- Benchmarking: compare sites, identify best practices.
- Drill-down: from fleet to individual machine.
**Acceptance criteria:**
- Real-time fleet status; issues identified quickly.
- KPI comparisons drive improvement initiatives.

### US-131 — Over-the-air (OTA) updates for PLC logic
**Persona:** Software deployment engineer.
**Context:** Need to update PLC logic across fleet without
site visits.
**Requirements:**
- OTA update: download, verify, install new version.
- Staged rollout: pilot machines first, then fleet.
- Rollback: automatic if update fails.
- Version management: track versions across fleet.
- Security: signed updates, authenticated download.
**Acceptance criteria:**
- Updates deployed to fleet in <1 hour.
- Failed updates auto-rollback; no downtime.

### US-132 — Training simulator for operator certification
**Persona:** Training manager.
**Context:** New operators need hands-on training without
risking production equipment.
**Requirements:**
- Simulator: virtual machine with realistic behavior.
- Scenarios: normal operation, faults, emergencies.
- Assessment: track operator performance.
- Certification: pass/fail criteria for qualification.
- Refresh training: periodic recertification.
**Acceptance criteria:**
- Operators certified before solo operation.
- Training effectiveness measured; skill gaps identified.

### US-133 — Digital twin for process optimization
**Persona:** Process engineer.
**Context:** Need to test process changes virtually before
implementing on production.
**Requirements:**
- Digital twin: virtual model of process/machine.
- Scenario testing: what-if analysis for changes.
- Optimization: test control strategies, recipes.
- Validation: compare twin predictions to actual.
- Integration: real-time data feeds twin.
**Acceptance criteria:**
- Process changes validated in twin before production.
- Twin accuracy within ±5 % of actual.

### US-134 — Spare parts management and obsolescence tracking
**Persona:** Supply chain manager.
**Context:** Need to manage spare parts inventory; track
obsolete components.
**Requirements:**
- Parts inventory: track stock levels, locations.
- Reorder points: automatic purchase requisitions.
- Obsolescence tracking: EOL/EOS dates, last-time-buy.
- Cross-reference: alternative parts.
- Cost tracking: parts cost, usage, inventory value.
**Acceptance criteria:**
- Stockouts <2 %; obsolete parts identified early.
- Inventory turns optimized; carrying costs reduced.

### US-135 — Knowledge management and documentation system
**Persona:** Engineering manager.
**Context:** Tribal knowledge walks out the door; need
documented procedures, lessons learned.
**Requirements:**
- Procedure library: standard operating procedures.
- Troubleshooting guides: symptom → cause → solution.
- Lessons learned: captured from issues/resolutions.
- Training materials: videos, simulations, documents.
- Search: full-text search across all content.
**Acceptance criteria:**
- Knowledge accessible to all team members.
- New employees productive faster; issues resolved quicker.
