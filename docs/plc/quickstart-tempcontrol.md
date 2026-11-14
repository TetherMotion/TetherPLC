# Quickstart — EtherCAT temperature cell (EL1014 → profile → EL2004 PWM)

End-to-end recipe for the typical machine-cell story: a bank of Beckhoff
EL1014 digital inputs (one channel = cycle start), an EL3104 analog
input feeding a temperature profile controller, an EL2004 output
PWM-driving the heater at 1 Hz, and two EL2004/EL1014-visible status
lines (running / finished) for a downstream system.

Works identically from **ST** and from the **FBD editor** — the
profile, controller, scalers and PWM are all `Atc_TempCtrl` function
blocks, callable from either language.

## 1. Install

```bash
sudo apt install ./tetherplc_*_amd64.deb
# or from source: cargo build --release -p tplcc && cmake --build build -j8
```

Binaries land in `/usr/bin`: `tplcc`, `tplc-pkg`, `tether-plc`,
`tether-plc-sim`, `tether-plc-bridge`, `tplc-ec-pump`, `tplc-proxy`.

## 2. Fleet description

`tplc-ec-pump` (emulated fleet) or a real Tether master takes a line-based
topology file — one node per line, `EK1100` opens a coupler segment.
Repeat a module type to get "a bunch"; PDO keys auto-suffix
(`el1014.c1`, `el1014.c1.2`, …):

```text
# cell.fleet — chain order
EK1100
EL1014        # c1: 4 DI  — ch1 = cycle start
EL1014        # c1.2: 4 DI — spares / interlocks
EL2004        # c1: 4 DO  — ch1 = running, ch2 = finished, ch3 = heater PWM
EL3104        # c1: 4 AI  — ch1 = thermocouple (int16 counts)
EL2004        # c1.2: 4 DO — heater PWM on ch1 (alternate)
```

```bash
tplc-ec-pump --simdir /tmp/tplc_sim --fleet cell.fleet --hz 100
```

On real hardware, drop `--simdir` and point Tether's EtherCAT master at
the NIC (`--iface`); the same fleet file drives the emulated path so
logic is testable without the bus.

## 3. The program

Two equivalent authoring paths — pick either; both compile to the same
package.

### 3a. ST

```st
PROGRAM Cell
VAR
    di    AT %IX0.0 : BOOL;    // EL1014.c1 ch1 — cycle start
    ai    AT %ID16.0 : INT;    // EL3104.c1 ch1 — raw counts
    run   AT %QX0.0 : BOOL;    // EL2004.c1 ch1 — running status
    fin   AT %QX0.1 : BOOL;    // EL2004.c1 ch2 — finished status
    heat  AT %QX0.2 : BOOL;    // EL2004.c1 ch3 — heater PWM (1 Hz)

    prof  : FB_AtcProfile;
    cfg   : FB_cell_cfg;       // generated curve FB (see below)
    pid   : FB_AtcPid;
    ais   : FB_AtcAiScale;
    pwm   : FB_AtcPwm;
    pv    : LREAL;
    eu    : LREAL;
    ct    : LREAL := 0.1;
END_VAR
    // raw counts -> engineering units (e.g. 0.01 deg/count + offset)
    ais(Execute := TRUE, Raw := ai, Scale := 0.01, Offset := -50.0,
        FiltT := 0.5, CycleTime := ct, Reset := FALSE);
    pv := ais.Out;

    prof.SEG[1].KIND := 3; prof.SEG[1].TGT := 200.0; prof.SEG[1].P1 := 300.0;
    prof.SEG[2].KIND := 1; prof.SEG[2].TGT := 200.0; prof.SEG[2].P1 := 600.0; prof.SEG[2].P2 := 2.0;
    prof.SEG[3].KIND := 0;
    prof(NUM_SEG := 3, RUN := di, HOLD := FALSE, RESET := FALSE,
         PvStart := TRUE, PV := pv, CycleTime := ct);

    pid(Execute := prof.Busy, Setpoint := prof.SP, Actual := pv,
        Kp := 2.0, Tn := 60.0, Tv := 0.0, OutMin := 0.0, OutMax := 100.0,
        CycleTime := ct);

    pwm(Execute := prof.Busy, Duty := pid.Out, Period := 1.0,
        MinOn := 0.1, CycleTime := ct, Reset := FALSE);   // 1 Hz base

    run := prof.Busy;   // running status -> downstream
    fin := prof.Done;   // finished status -> downstream
    heat := pwm.Q;      // PWM pulse -> heater
END_PROGRAM
```

The curve table can equally come from a generated
`FUNCTION_BLOCK FB_cell_cfg` (the WebUI **export .st** path) whose
`PROF : FB_AtcProfile` inout pin is called once per scan.

### 3b. FBD (TetherWebUI)

1. **Devices**: add `EL1014` (4×DI), `EL3104` (4×AI int16), `EL2004`
   (4×DO) blocks; the wizard auto-allocates non-overlapping image
   bases and suggests `el1014.c1`-style PDO endpoints. Wire the pins
   to interface vars (`di`, `ai`, `run`, `fin`, `heat`).
2. **Curves pane**: draw the profile; set `run=di`, `pv=pv`,
   `busy=run`, `done=fin`; enable **heater pwm** (period 1.0 s,
   min on 0.1 s, out `heat`). Insert — the editor drops
   `FB_AtcProfile` + config + `FB_AtcPid` + `FB_AtcPwm` and wires
   everything, including `ctl.Out → pwm.Duty → Q → heat`.
3. Analog scaling: drop `FB_AtcAiScale` between the `ai` pin and `pv`.

## 4. Deploy map (image ↔ PDO)

The `.tplc` carries an `IO_MAP` section; the bridge needs a
`DEPLOY_MAP` binding image channels to bus PDO keys:

```bash
# generate from the package's IO_MAP + the live bus catalog
# (<simdir>/.bus is written by tplc-ec-pump at startup)
tether-plc-bridge --gen-map cell.tplc --bus /tmp/tplc_sim/.bus -o cell.map
# or validate an existing/edited map
tether-plc-bridge --validate cell.map --bus /tmp/tplc_sim/.bus
```

The WebUI **io-map** card does the same graphically: *generate from
bindings*, *validate*, *save for deploy* — the saved map is injected
as a `DEPLOY_MAP` section during build/deploy (`/pkgmap` on the
proxy). `safe=zero` on output channels defines the fault-state policy.

## 5. Run

```bash
# 1. EtherCAT pump (emulated fleet or real master)
tplc-ec-pump --simdir /tmp/tplc_sim --fleet cell.fleet --hz 100 &

# 2. I/O bridge: process image <-> PDO files, spawns the runtime
tether-plc-bridge --image /dev/shm/tplc_cell --map cell.map \
    --simdir /tmp/tplc_sim --hz 500 \
    --plc tether-plc --image /dev/shm/tplc_cell \
         --package cell.tplc --ctl /tmp/tplc_cell.ctl

# 3. (WebUI path) tplc-proxy serves /compile /blocks /pkgmap for the IDE
```

`plc.image.*` catalog signals expose every channel live; `dbg-write`
on the ctl socket can force `di`/`run` for bench tests.

## 6. ST ↔ FBD parity notes

- `AT %IX/%ID/%QX/%QD/%MW` located vars work in ST and produce the same
  `IO_MAP` entries the deploy-map generator consumes.
- FBD `device` blocks and ST `AT` declarations describe the same
  channels — don't bind both to overlapping addresses (the validator
  rejects overlaps).
- `FB_AtcAiScale`/`FB_AtcAoScale` are plain FBs: callable from ST,
  insertable as FBD blocks, both against the same `atc_tempctrl.st`.
