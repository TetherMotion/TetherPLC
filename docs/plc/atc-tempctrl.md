# Atc_TempCtrl — advanced temperature-control library

`libraries/tempctrl/atc_tempctrl.st` is a pure-ST IEC 61131-3 library
packaged as `kind="library"` `.tplc` and consumed by programs via
`tplcc build <prog>.st --lib atc_tempctrl.tplc`. Everything runs as
PLC logic on the TetherPLC scan task: no services, no allocation, no
native libm — even `EXP`/`LN`/`SQRT` are implemented in ST below.

```bash
compiler/target/debug/tplcc build --library --name Atc_TempCtrl \
    libraries/tempctrl/atc_tempctrl.st -o atc_tempctrl.tplc

compiler/target/debug/tplcc build my_program.st \
    --lib atc_tempctrl.tplc -o my_program.tplc
```

With the `.deb` installed (`toolchain.md` §6) the library package is
already on disk — skip the `--library` build and use
`--lib /usr/share/tetherplc/lib/atc_tempctrl.tplc`.

Conventions: `Execute` enables work for one scan; `Done`/`Err` report
status (latched on the "finishing" blocks until re-armed or `Reset`);
`CycleTime`/`Ts` inputs are seconds and must equal the task interval;
temperatures and outputs are LREAL in whatever engineering units the
plant uses. Function-block state lives in the PLC state blob, so
online change / shadow run treat it like any other instance data.

## Classic controllers

| Block | Purpose |
|---|---|
| `FB_AtcPid` | Positional PID: SP weighting, filtered D (N), back-calculation anti-windup on saturation and slew limit, `Track`/`ManEn` manual modes, `HoldI` for cascade coupling, `AtLimit`/`E`/`Out`/`Done`/`Err`. |
| `FB_AtcCascadePid` | Master/slave: outer process PID emits an inner-loop setpoint clamped to `InnerSpMin/Max`; inner `FB_AtcPid` gets `HoldI` when the inner SP is clamped (anti-windup). Outputs `InnerSp`, inner `Out`. |
| `FB_AtcSmith` | Smith predictor for dead-time-dominant loops: internal PID acts on `SP + (x1 - xd) - PV`, where `x1` is the undelayed model output and `xd` the model output delayed `ModelDt`; model = `ModelK/(ModelT*s+1)` discretized per `CycleTime`. |

## Signal / output conditioning

| Block | Purpose |
|---|---|
| `FB_AtcPt1` | First-order sensor filter, `Tf` time constant. |
| `FB_AtcRateLim` | Slew limiter: `Out` approaches `In` at `Rate` units/s. |
| `FB_AtcPwm` | Time-proportioning output: `Duty` % → BOOL pulse over `Period`, `MinOn` drops too-short pulses. |
| `FB_AtcHeatCool` | Split-range: `U ∈ [-100,100]` → `Heat = max(U-DZ,0)`, `Cool = CoolGain*max(-U-DZ,0)`. |

## State-space control (LQR / LQG)

The plant is modeled as a discrete 2-state system
`x+ = A x + B u, y = x1` produced by `FB_AtcModel2s` (or supplied by
hand). `x1` is the measured temperature; `x2` the heater/first-lag
state.

| Block | Purpose |
|---|---|
| `FB_AtcModel2s` | FOPDT `(K, Tau, Theta[, TauH])` → discrete `A`/`B` at step `Ts`. Dead time is folded into the heater lag (accurate for `Theta <~ Tau`; use `FB_AtcSmith` for delay-dominant plants). |
| `FB_AtcLqr` | Servo LQR: `u = Nref*SP - K1*x1 - K2*x2 + Ki*xi + FFwd`, `xi+ = xi + (SP-PV)*Ts`, with output clamp back-calculation on `Xi`, `RateLim`, `ManEn`. Gains from `FB_AtcLqrTune` (`Ki > 0` for self-regulating heating). |
| `FB_AtcLqg` | LQG = steady-state Kalman filter + the same LQR law on estimates `xh`: predict `xp = A*xh + B*u`, update `xh = xp + L*(PV - xp1)`, control `u = -K*xh + Ki*xi`. Exposes `X1`,`X2` (estimates) and `Xi`. |

### Riccati solvers

`FB_AtcDare3` (internal, also usable directly) solves the 3×3 DARE
`P = A'PA - A'PB(R+B'PB)⁻¹B'PA + Q` by **Kleinman iteration**: each
step solves `(A-BK)'P(A-BK) - P = -(Q+K'RK)` via matrix squaring
(`G ← G + W'GW; W ← W²`, ~32 squarings). Chosen over plain fixed-point
iteration because the servo plant's integrator adds a pole at exactly
1 — fixed point converges linearly at the open-loop spectral radius
(needs ~10⁵ iterations), Kleinman converges quadratically in ~10.
Bounded worst case ~200 kflop on one scan; runs on the `Execute`
rising edge, `Conv`/`Iter` report convergence.

| Block | Purpose |
|---|---|
| `FB_AtcLqrTune` | Builds the augmented servo system `Aa = [[A,0],[-Ts,0,1]]`, `Ba = [B;0]`, `Q = diag(Q1,Q2,Qi)` and emits `K1,K2` plus `Ki` already negated to the `+Ki*xi` convention. |
| `FB_AtcKalmanTune` | Steady-state Kalman gains via the dual DARE on `(A', C')`, `C=[1 0]`, `Q = diag(Q1,Q2)` process noise, `R` measurement noise → `L1,L2` filter-form gains for `FB_AtcLqg`. |

## Autotuning

| Block | Purpose |
|---|---|
| `FB_AtcRelayTune` | Åström-Hägglund relay feedback: oscillates `Actual` about `Setpoint` (`MeanOut ± RelayAmp`, hysteresis `Hyst`), measures oscillation amplitude/period over `MinCyc` cycles → `Ku, Tu` → PID gains by `TuneRule` (0 ZN, 1 ZN-some-overshoot, 2 ZN-no-overshoot, 3 ZN-P, 4 Tyreus-Luyben). `Abort`/`Reset`/`Busy`/`Done`/`Err`. |
| `FB_AtcStepIdent` | Open-loop step-response identification: settles at `U0` (`SettleT` + `TolBand`/`SettleN`), steps to `U1`, records the rise in a 512-sample trace (`StoreEvery` decimation), fits FOPDT `K, Tau, Theta` by the t35/t85 method. `Progress`, `Pv0`, `PvFinal`, `ErrId` diagnostics. |
| `FB_AtcTuneRules` | FOPDT `(K,Tau,Theta)` → `Kp,Tn,Tv` by rule: 0 IMC/Rivera, 1 SIMC, 2 ZN reaction curve, 3 Cohen-Coon, 4 AMIGO. `Lambda` overrides the closed-loop target for rules 0/1. |

## Profile programming

`ST_AtcSeg` is the segment record (`KIND, TGT, P1, P2, EV`); a profile
is `SEG : ARRAY[1..32] OF ST_AtcSeg` on `FB_AtcProfile`. Aggregate
inputs cannot be call-bound — write members before calling:
`prof.SEG[1].KIND := 2; prof.SEG[1].TGT := 100.0; ...` then
`prof(NUM_SEG := n, RUN := ..., PV := pv, CycleTime := 0.5)`.

| `KIND` | Meaning |
|---|---|
| 0 END | finish (`Done`, `Busy` off) |
| 1 DWELL | hold `TGT` for `P1` s; with `P2 > 0` the timer only advances while `|PV - TGT| <= P2` (guaranteed soak) |
| 2 RAMP | ramp to `TGT` at `|P1|` deg/s (`P1 <= 0` behaves as STEP) |
| 3 RAMPT | ramp to `TGT` in exactly `P1` s |
| 4 STEP | jump `SP := TGT`, advance next scan |
| 5 LOOP | jump to segment `TRUNC(TGT)`, `P1` times total |

`RUN` rising edge starts at segment 1 (`SP := PV` when `PvStart` —
bumpless PV-start), `HOLD` freezes ramp progress and dwell timers,
`RESET` aborts with `SP := PV` (no thermal jump). A run request that
arrives before a segment table exists (`NUM_SEG < 1`, e.g. a config
block that the FBD lowerer orders after the profile call) is **held**,
not started: `Err` is set and the next scan retries once the table
loads — a boot-time `RUN` never starts a zero-segment profile that
would end immediately. Outputs: `SP`,
`SegIdx`, `SegTime`, `SegTgt`, `RateNow`, `RemTime` (−1 unbounded),
`EV` (active segment's event word), `Done`/`Busy`/`Err`.

### Graphical programming (TetherWebUI)

The Web IDE's **Curves** pane (`src/plc/tempcurve.ts` + `curve.ts`)
edits this table graphically — segment rows, SP-vs-time preview with
draggable points, `PvStart`/`CycleTime`/controller selection — and
**insert**s it into an FBD network as an inline `st` block that writes
`PROF.SEG[i].*` one-shot through a `var_in_out` pin, or **export**s it
as a standalone `FUNCTION_BLOCK` ST source. Both forms compile to the
same logic; see `web-ide.md` §4b. `tests/mixed/ui_profile.fbd.json` is
that editor's output and is executed by `tplcc`'s
`ui_curve_profile_runs` test.

## Math helpers (internal, exported for reuse)

`F_AtcExp` (range-reduced, 10th-order Taylor), `F_AtcLn` (2^k
reduction + atanh series), `F_AtcSqrt` (Newton, ~9 iterations),
`F_AtcPow` (`exp(E*ln B)`, negative bases for bounded integer
exponents). These exist because the ST builtin set has no
transcendentals; they are LREAL-accurate to <1e-9 relative on the
ranges the library exercises.

## Relationship to `compat/tc2`

`compat/tc2/tc2_tempcontroller.st` (`FB_TempController`) and
`compat/tc2/tc2_filter.st` (`FB_PID` etc.) stay: they are TwinCAT API
compatibility shims for ported sources, kept byte-compatible with the
Tc2 semantics they emulate. `Atc_TempCtrl` is the native advanced
library — new code should prefer it (cascade/Smith/LQR/LQG,
autotuning, profiles, richer diagnostics); the two can coexist in one
program without interference since instance state is per-FB.

## Tests

`tests/tempctrl/e2e.sh` (ctest `tempctrl_lib`): packages the library,
verifies the `.tplc`, builds a `--lib` consumer, replays twelve
self-checking programs through `tplcc vectors` (`tests/tempctrl/
t01..t12*.st` + `vectors.jsonl`) — PID/cascade/Smith closed loops on
simulated plants, relay and step-response autotune, tuning-rule
constants, LQR/LQG gains checked against scipy `solve_discrete_are`
references, profile sequencing with guaranteed-soak and LOOP — and,
when `tether-plc` + `tether-plc-sim` are built, replays a
profile→PID→plant program through the real runtime.
