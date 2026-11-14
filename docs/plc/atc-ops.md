# Atc_Ops — machine/process operation blocks

`libraries/tempctrl/atc_ops.st` is the operations companion to
`atc_tempctrl.st`: general machine-and-process blocks that sit around
the controllers (debounce, alarms, hour meters, OEE, sensor
plausibility, sequencing, andon, recipe storage). Pure ST, no services
or hardware — deterministic per scan; every timed block takes
`CycleTime` (s).

```bash
tplcc build my_program.st --lib libraries/tempctrl/atc_ops.st \
    --lib libraries/tempctrl/atc_tempctrl.st -o my_program.tplc
# or the prebuilt library package:
tplcc build --library --name Atc_Ops libraries/tempctrl/atc_ops.st \
    -o atc_ops.tplc
tplcc build my_program.st --lib atc_ops.tplc -o my_program.tplc
```

Aggregate inputs cannot be call-bound — write members before calling
(same convention as `FB_AtcProfile.SEG`). BOOL strobes are
level-sensitive unless noted; use `FB_AtcDebounce` or edge-detect in
the caller for pulses.

## Signal conditioning

| Block | Purpose |
|---|---|
| `FB_AtcDebounce` | `D` must hold `OnTime` s to set `Q`, `OffTime` s to reset. Independent on/off times; `ET` = seconds since `Q` last changed. The classic pushbutton/limit-switch conditioner. |

## Alarms & supervision

| Block | Purpose |
|---|---|
| `FB_AtcAlarm` | Latched alarm per condition: `Cond` rising edge sets `Unacked` (seen even after `Cond` clears), `Ack` pulse clears it. `Shelve` pulse suppresses `Active` for `ShelveTime` s (`<=0` = until `Cond` clears). `FirstOut` = active and no higher-priority alarm — alarm-flood first-out. `ShelfET` = remaining shelve time. |
| `FB_AtcSensorCheck` | Analog plausibility on the EU value (after `FB_AtcAiScale`): range violation (`Min`/`Max` — catches 0 mA/4 mA live-zero failures), rate-of-change (`MaxRate` EU/s), stuck value (`StuckBand` deadband frozen for `StuckTime` s). Priority range > rate > stuck; `Bad` latches until `Reset`; `Reason` 0–3, `RateNow`, `StuckET`. |

## Machine data & state

| Block | Purpose |
|---|---|
| `FB_AtcHourMeter` | Run-hour accumulator: `Hours += CycleTime/3600` while `Run`. `Preset` = service interval (h); `Alarm` asserts at `Hours >= Preset` until `Reset`. `Pct` = 0–100+ for HMI bars (`-1` when `Preset<=0`). Compressor/spindle/filter hours. |
| `FB_AtcOee` | Availability/performance/quality: `PlannedH` while `Planned`, `RunH` while `Running AND Planned`, `GoodCount`/`TotalCount` on part pulses. `Availability = RunH/PlannedH`, `Performance = Good*IdealCycle/RunH`, `Quality = Good/Total`, `OEE = A·P·Q/10000`. `Reset` zeroes all. Times in hours. |
| `FB_AtcRecipe` | 16 sets × 16 LREAL parameters, addressed `(WrSet, WrParam)` 0–15. `WrStrobe` rising edge writes `WrVal`; `RdSet`/`RdParam` continuously expose `Val`/`Valid`. `Clear` zeroes the store. Not itself persistent — copy hot params to retained vars for NV. |

## Sequencing & indication

| Block | Purpose |
|---|---|
| `FB_AtcSeq` | Drum sequencer over `STEPS : ARRAY[1..32] OF ST_AtcStep` (`TIME` dwell s — `<=0` waits for `StepDone`; `NEXT` target, `0`/out-of-range ends; `EV` event word). `RUN` rising edge starts at step 1; `HOLD` freezes; `RESET` aborts. Outputs `StepIdx`, `StepTime`, `StepTgt`, `Done`, `Busy`, `EV`, `Err`. Backward `NEXT` makes loops legal — caller owns termination. |
| `FB_AtcAndon` | Stack-light/buzzer mapper, purely combinational: `Red = Fault OR Alarm`, `Amber = Warning OR Maintenance`, `Green = Run AND no higher state`, `Blue = Changeover`, `Buzzer = Red`. Priority Fault > Alarm > Warning > Maintenance > Run. Flashers are downstream logic. |

## Machine control

| Block | Purpose |
|---|---|
| `FB_AtcOnOff` | Bang-bang (thermostat) control: `PV <= SP - Hyst/2` → on, `PV >= SP + Hyst/2` → off, deadband holds. Symmetric — swap PV/SP for inverse action. `Enable` forces off. Stage minimum on/off times downstream if the load can't cycle. |
| `FB_AtcPermissive` | 8-input run-permit AND chain (`P1`–`P8`). `Permit` = all true; `DenyMask` bit i−1 = permissive i denying; `FirstDeny` = lowest denying index. No bypass input by design — bypass upstream with its own guard. |
| `FB_AtcMode` | Mode arbitration, level requests: priority Jog > Maint > Man > Auto → `Mode` 0–4 + one-hot BOOLs + `Changed` pulse on transition. No request → 0 (off). |
| `FB_AtcJog` | Inch/jog: `DirIn` −1/0/+1 follows while deadman `En` held; `MaxOn` timeout latches `TimedOut` until `En` released (anti-run). `ET` = enabled run time. |
| `FB_AtcWatchdog` | Heartbeat supervisor: `Kick` edge resets `ET`; `Err` while `ET > Timeout` (level — auto-recovers when comms resume). `OK` complement, `ET` diagnostic. |
| `FB_AtcBlinker` | Flash generator: `Phase` advances `Rate` Hz while `En`; `Q` true for first `Duty` fraction of cycle. `En` off rewinds phase. |
| `FB_AtcTwoHand` | Two-hand anti-tie-down: both buttons within `MaxGap` → `Out` while both held; either release drops it; both must be released before re-arm. Safety-chain permit — gate through the safety relay layer. |

## Measurement & totals

| Block | Purpose |
|---|---|
| `FB_AtcTotalizer` | `Total += Rate * CycleTime * K` — flow/kWh/parts integration. Signed rates subtract. `Reset` zeroes. |
| `FB_AtcAvg` | Moving average over last `N` scans (clamp 1–32). `Avg` = window mean, `Count` = samples seen. |
| `FB_AtcDemand` | Sliding-window mean of `Power` over `Window` s (ring ≤64 samples — keep `CycleTime` small vs window). `Demand`, `Samples`. |
| `FB_AtcOverride` | Manual override: `ManEn` → `Out = ManIn`, else `AutoIn`. On release edge outputs `TrackIn` one scan (`Tracking`) for bumpless re-entry. `Err` kept for interface symmetry. |

## Tests

`tests/ops/e2e.sh` — library check/build/verify plus a 19-program
self-checking vector suite (`tests/ops/cases/o01..o19`) covering
every block: debounce timing, alarm ack/shelve/first-out, hour-meter
preset alarm + reset, OEE counters + reset, sensor range/rate/stuck
priority, sequencer timed/condition steps + done, andon priority
matrix, recipe write/read/clear/invalid, on/off hysteresis,
permissive deny mask, mode arbitration, jog timeout, watchdog
heartbeat, blinker phase, totalizer reset, moving average, windowed
demand, override release track, two-hand gap/anti-tie-down.
