// atc_ops.st — Atc_Ops: general machine/process operation blocks.
// Pure-ST, no services or hardware: deterministic per scan, every
// timed block takes CycleTime (s). Lives next to atc_tempctrl.st so
// the shipped --libdir picks it up into the IDE block catalog:
//
//   tplcc build --library --name Atc_Ops \
//       libraries/tempctrl/atc_ops.st -o atc_ops.tplc
//
// Blocks (each maps to many USERSTORIES.md entries):
//   FB_AtcDebounce   input debounce with independent on/off times
//   FB_AtcAlarm      latched alarm: ack, timed shelve, first-out
//   FB_AtcHourMeter  run-hour accumulator + service preset alarm
//   FB_AtcOee        availability/performance/quality counters
//   FB_AtcSensorCheck analog plausibility: range, rate, stuck
//   FB_AtcSeq        drum sequencer (timed or condition-held steps)
//   FB_AtcAndon      stack-light/buzzer state mapper
//   FB_AtcRecipe     16x16 LREAL parameter-set store + readback
//   FB_AtcOnOff      bang-bang control with hysteresis
//   FB_AtcPermissive 8-input run-permit chain + deny mask/first-deny
//   FB_AtcMode       operating-mode arbitration (auto/man/jog/maint)
//   FB_AtcJog        inch/jog with deadman + anti-run timeout
//   FB_AtcWatchdog   heartbeat/communication supervisor
//   FB_AtcBlinker    flash pattern generator (Hz + duty)
//   FB_AtcTotalizer  rate->total integrator (kWh, flow, parts)
//   FB_AtcAvg        moving-average filter (window <=32 scans)
//   FB_AtcDemand     sliding-window demand average (<=64 samples)
//   FB_AtcOverride   manual override with bumpless release track
//   FB_AtcTwoHand    two-hand anti-tie-down safety control
//
// Conventions match atc_tempctrl.st: unbound inputs keep their last
// value; timed blocks accumulate CycleTime per scan; BOOL strobes
// are level-sensitive unless noted (use FB_AtcDebounce or edge
// detect in the caller for pulses).

// ============================ debounce ============================

// FB_AtcDebounce — condition D must hold for OnTime seconds before Q
// sets and for OffTime seconds before Q resets. OnTime=OffTime=Ct is
// the classic single-time debounce. ET counts seconds since Q last
// changed (diagnostic / HMI "debounced for" display).
FUNCTION_BLOCK FB_AtcDebounce
VAR_INPUT
    D : BOOL;
    OnTime : LREAL;      // s D must stay TRUE  to set Q
    OffTime : LREAL;     // s D must stay FALSE to reset Q
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : LREAL;          // s since Q last changed
END_VAR
VAR
    t : LREAL;
END_VAR
    ET := ET + CycleTime;
    IF D <> Q THEN
        t := t + CycleTime;
        IF (D AND t >= OnTime) OR (NOT D AND t >= OffTime) THEN
            Q := D;
            t := 0.0;
            ET := 0.0;
        END_IF;
    ELSE
        t := 0.0;
    END_IF;
END_FUNCTION_BLOCK

// ============================ alarm ================================

// FB_AtcAlarm — latched alarm for one condition.
//   Cond rising edge sets Unacked (the event must be seen and acked
//   even after Cond clears). Ack pulse clears Unacked.
//   Shelve pulse starts a ShelveTime-second suppression of Active;
//   Active := Cond AND NOT Shelved. Shelved also clears when Cond
//   clears or when the timer expires. ShelveTime<=0 shelves until
//   Cond clears (operator "mute while investigating").
//   HigherPriActive feeds FirstOut: TRUE only when this alarm is
//   active and no higher-priority alarm in the group is — the
//   classic alarm-flood first-out indication.
//   Priority is documentation only (0 info, 1 warn, 2 alarm,
//   3 critical); ordering is the caller's job via FirstOut wiring.
FUNCTION_BLOCK FB_AtcAlarm
VAR_INPUT
    Cond : BOOL;
    Ack : BOOL;          // pulse: acknowledge Unacked
    Shelve : BOOL;       // pulse: start shelve timer
    ShelveTime : LREAL;  // s; <=0 = shelf until Cond clears
    HigherPriActive : BOOL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Active : BOOL;       // Cond AND NOT Shelved
    Unacked : BOOL;      // latched until Ack
    Shelved : BOOL;
    FirstOut : BOOL;
    ShelfET : LREAL;     // s remaining in shelve (0 when not shelved)
END_VAR
VAR
    condLast : BOOL;
    shelfT : LREAL;      // >0 while shelved
END_VAR
    IF Cond AND NOT condLast THEN
        Unacked := TRUE;
    END_IF;
    condLast := Cond;
    IF Ack THEN
        Unacked := FALSE;
    END_IF;
    IF Shelve AND Cond THEN
        IF ShelveTime > 0.0 THEN
            shelfT := ShelveTime;
        ELSE
            shelfT := -1.0;   // indefinite until Cond clears
        END_IF;
    END_IF;
    IF shelfT > 0.0 THEN
        shelfT := shelfT - CycleTime;
        IF shelfT < 0.0 THEN shelfT := 0.0; END_IF;
    END_IF;
    IF NOT Cond THEN
        shelfT := 0.0;       // nothing to suppress once clear
    END_IF;
    Shelved := shelfT <> 0.0;
    ShelfET := shelfT;
    Active := Cond AND NOT Shelved;
    FirstOut := Active AND NOT HigherPriActive;
END_FUNCTION_BLOCK

// ============================ hour meter ============================

// FB_AtcHourMeter — maintenance hour meter. Hours accumulates
// CycleTime/3600 while Run is TRUE. Preset is the service interval
// in hours; Alarm asserts when Hours >= Preset and stays asserted
// until Reset (or Preset raised above Hours). Pct gives 0..100+
// for an HMI bar. This is the classic "compressor hours",
// "spindle hours", "filter hours" block behind US-017/US-077.
FUNCTION_BLOCK FB_AtcHourMeter
VAR_INPUT
    Run : BOOL;
    CycleTime : LREAL;
    Preset : LREAL;       // service interval, hours
    Reset : BOOL;         // clears Hours and Alarm (service done)
END_VAR
VAR_OUTPUT
    Hours : LREAL;
    Alarm : BOOL;
    Pct : LREAL;          // Hours/Preset*100, -1 when Preset<=0
END_VAR
VAR
    over : BOOL;
END_VAR
    IF Reset THEN
        Hours := 0.0;
        over := FALSE;
    ELSE
        IF Run THEN
            Hours := Hours + CycleTime / 3600.0;
        END_IF;
        IF Preset > 0.0 AND Hours >= Preset THEN
            over := TRUE;
        END_IF;
    END_IF;
    Pct := -1.0;
    IF Preset > 0.0 THEN
        Pct := Hours / Preset * 100.0;
    END_IF;
    Alarm := over;
END_FUNCTION_BLOCK

// ============================ OEE ===================================

// FB_AtcOee — overall equipment effectiveness counters.
//   PlannedTime  accumulates while Planned (scheduled production)
//   RunTime      accumulates while Running AND Planned (OEE
//                availability counts only planned windows; a lunch
//                break should clear Planned, not Running)
//   GoodCount/TotalCount increment on GoodPart/TotalPart pulses
//   (level inputs — a one-scan TRUE counts once per scan it is held,
//   so drive them with a pulse/edge in the caller if parts complete
//   faster than a scan).
//   Availability  = RunTime/PlannedTime*100   (0 when no plan)
//   Performance   = GoodCount*IdealCycle/RunTime*100
//                  (ideal seconds per good part over actual run
//                  seconds — >100 possible on over-performing)
//   Quality       = GoodCount/TotalCount*100  (0 when no parts)
//   OEE           = A*P*Q/10000
// Reset zeroes all counters. Times accumulate in HOURS.
FUNCTION_BLOCK FB_AtcOee
VAR_INPUT
    Planned : BOOL;
    Running : BOOL;
    IdealCycle : LREAL;   // s per good part at design rate
    GoodPart : BOOL;
    TotalPart : BOOL;
    Reset : BOOL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Availability : LREAL; // %
    Performance : LREAL;  // %
    Quality : LREAL;      // %
    OEE : LREAL;          // %
    PlannedH : LREAL;     // planned hours
    RunH : LREAL;         // run hours
    GoodCount : DINT;
    TotalCount : DINT;
END_VAR
VAR
    dt : LREAL;
END_VAR
    dt := CycleTime / 3600.0;
    IF Reset THEN
        PlannedH := 0.0;
        RunH := 0.0;
        GoodCount := 0;
        TotalCount := 0;
    ELSE
        IF Planned THEN
            PlannedH := PlannedH + dt;
            IF Running THEN
                RunH := RunH + dt;
            END_IF;
        END_IF;
        IF TotalPart THEN
            TotalCount := TotalCount + 1;
        END_IF;
        IF GoodPart THEN
            GoodCount := GoodCount + 1;
        END_IF;
    END_IF;
    Availability := 0.0;
    IF PlannedH > 0.0 THEN
        Availability := RunH / PlannedH * 100.0;
    END_IF;
    Performance := 0.0;
    IF RunH > 0.0 AND IdealCycle > 0.0 THEN
        Performance :=
            DINT_TO_LREAL(GoodCount) * IdealCycle /
            (RunH * 3600.0) * 100.0;
    END_IF;
    Quality := 0.0;
    IF TotalCount > 0 THEN
        Quality := DINT_TO_LREAL(GoodCount) /
                   DINT_TO_LREAL(TotalCount) * 100.0;
    END_IF;
    OEE := Availability * Performance * Quality / 10000.0;
END_FUNCTION_BLOCK

// ======================== sensor plausibility ======================

// FB_AtcSensorCheck — plausibility supervision for one analog
// channel (EU value after FB_AtcAiScale). Detects, in priority
// order: range violation, rate-of-change violation, stuck value.
//   Reason  0 OK
//           1 below Min / above Max (includes sensor 0 mA / 4 mA
//             live-zero failures when Min/Max bracket them)
//           2 |dv/dt| > MaxRate (EU/s; spike / wiring glitch)
//           3 value frozen inside StuckBand for StuckTime s
//             (frozen transmitter / stuck ADC / comm dropout)
// Bad latches until Reset (or Cond clears when AutoClear). Ok is the
// complement of Bad. Dev shows the last violation reason even while
// Bad is acknowledged — use Reason for the live code.
// Deadband: values differing by less than StuckBand count as
// "unchanged" for stuck detection; 0 disables band (exact compare).
FUNCTION_BLOCK FB_AtcSensorCheck
VAR_INPUT
    In : LREAL;
    Min : LREAL;
    Max : LREAL;
    MaxRate : LREAL;      // EU/s; <=0 disables rate check
    StuckBand : LREAL;    // EU deadband; <=0 disables stuck check
    StuckTime : LREAL;    // s frozen before stuck trips
    Reset : BOOL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Ok : BOOL;
    Bad : BOOL;
    Reason : DINT;
    RateNow : LREAL;      // last computed EU/s
    StuckET : LREAL;      // s the value has been frozen
END_VAR
VAR
    last : LREAL;
    rate : LREAL;
    frozenT : LREAL;
    badLat : BOOL;
    reasonLat : DINT;
    init : BOOL;
END_VAR
    rate := 0.0;
    IF init THEN
        rate := (In - last) / CycleTime;
    END_IF;
    RateNow := rate;
    // frozen detection: value moved beyond deadband -> reset timer,
    // otherwise accumulate. First scan after init counts as moved.
    IF init THEN
        IF StuckBand > 0.0 THEN
            IF In - last >= StuckBand OR last - In >= StuckBand THEN
                frozenT := 0.0;
            ELSE
                frozenT := frozenT + CycleTime;
            END_IF;
        ELSE
            IF In <> last THEN
                frozenT := 0.0;
            ELSE
                frozenT := frozenT + CycleTime;
            END_IF;
        END_IF;
    END_IF;
    // reason evaluation (priority: range > rate > stuck)
    reason := 0;
    IF In < Min OR In > Max THEN
        reason := 1;
    ELSIF MaxRate > 0.0 THEN
        IF rate > MaxRate OR rate < -MaxRate THEN
            reason := 2;
        END_IF;
    END_IF;
    IF reason = 0 AND StuckTime > 0.0 AND frozenT >= StuckTime THEN
        reason := 3;
    END_IF;
    IF Reset THEN
        badLat := FALSE;
        reasonLat := 0;
    ELSE
        IF reason <> 0 THEN
            badLat := TRUE;
            reasonLat := reason;
        END_IF;
    END_IF;
    Bad := badLat;
    Ok := NOT badLat;
    // live reason while violating, else the latched cause while Bad
    IF reason <> 0 THEN
        Reason := reason;
    ELSIF badLat THEN
        Reason := reasonLat;
    ELSE
        Reason := 0;
    END_IF;
    StuckET := frozenT;
    last := In;
    init := TRUE;
END_FUNCTION_BLOCK

// ============================ drum sequencer ========================

// ST_AtcStep — one drum-sequencer step.
//   TIME   dwell seconds; <=0 means "wait for StepDone input"
//   NEXT   step index to advance to (0 or >NUM ends the sequence)
//   EV     event word exposed on EV while the step is active
// Same record style as ST_AtcSeg so the profile editor patterns
// (member-wise writes, kind constants) carry over.
TYPE ST_AtcStep : STRUCT
    TIME : LREAL;
    NEXT : DINT;
    EV : DWORD;
END_STRUCT
END_TYPE

// FB_AtcSeq — drum sequencer: NUM_STEPS entries of ST_AtcStep.
//   RUN rising edge starts at step 1. Each scan StepTime advances
//   by CycleTime while the step is timed; when StepTime>=TIME the
//   block advances to STEP.NEXT. Timed-out or NEXT<=0/>NUM ends the
//   sequence: Done:=TRUE, Busy:=FALSE.
//   Steps with TIME<=0 hold until StepDone is TRUE (one scan) —
//   the caller wires the interlock/condition into StepDone.
//   HOLD freezes timers and waits; RESET aborts (StepIdx:=0).
//   A NEXT that points backwards makes loops legal — the caller
//   owns termination; there is deliberately no step counter cap.
//   StepTime/StepTgt/EV mirror FB_AtcProfile outputs for HMI reuse.
FUNCTION_BLOCK FB_AtcSeq
VAR_INPUT
    STEPS : ARRAY[1..32] OF ST_AtcStep;
    NUM_STEPS : DINT;
    RUN : BOOL;
    HOLD : BOOL;
    RESET : BOOL;
    StepDone : BOOL;      // advance signal for TIME<=0 steps
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    StepIdx : DINT;       // 0 idle, 1..NUM active
    StepTime : LREAL;     // s inside active step
    StepTgt : DINT;       // NEXT of active step
    Done : BOOL;
    Busy : BOOL;
    EV : DWORD;
    Err : BOOL;
END_VAR
VAR
    runLast : BOOL;
    idx : DINT;
END_VAR
    IF RESET THEN
        Busy := FALSE;
        Done := FALSE;
        StepIdx := 0;
        StepTime := 0.0;
        StepTgt := 0;
        EV := 0;
        Err := FALSE;
        runLast := FALSE;
        RETURN;
    END_IF;
    IF RUN AND NOT runLast THEN
        IF NUM_STEPS < 1 THEN
            Err := TRUE;
        ELSE
            idx := 1;
            Busy := TRUE;
            Done := FALSE;
            StepTime := 0.0;
            Err := FALSE;
        END_IF;
    END_IF;
    runLast := RUN;
    IF NOT Busy THEN
        RETURN;
    END_IF;
    IF idx < 1 OR idx > NUM_STEPS THEN
        Busy := FALSE;
        Done := TRUE;
        StepIdx := 0;
        EV := 0;
        RETURN;
    END_IF;
    StepIdx := idx;
    StepTgt := STEPS[idx].NEXT;
    EV := STEPS[idx].EV;
    IF HOLD THEN
        RETURN;
    END_IF;
    StepTime := StepTime + CycleTime;
    IF STEPS[idx].TIME <= 0.0 THEN
        // untimed step: advance on StepDone (level, one scan is enough
        // — the caller edge-detects if a multi-scan hold is needed)
        IF StepDone THEN
            StepTime := 0.0;
            idx := STEPS[idx].NEXT;
        END_IF;
    ELSIF StepTime >= STEPS[idx].TIME THEN
        StepTime := 0.0;
        idx := STEPS[idx].NEXT;
    END_IF;
END_FUNCTION_BLOCK

// ============================ andon =================================

// FB_AtcAndon — stack-light / tower state mapper. Pure combinational
// (no CycleTime): priority Fault > Alarm > Warning > Maintenance >
// Run > idle. Changeover is orthogonal — while set it forces Blue
// and suppresses Green (setup mode), but never masks Red/Amber.
//   Red    := Fault OR Alarm
//   Amber  := Warning OR Maintenance
//   Green  := Run AND no higher state
//   Blue   := Changeover
//   Buzzer := Fault OR Alarm
// Flasher patterns are downstream logic (FB_AtcPwm or a blink FB);
// this block reports steady state only — predictable for FBD.
FUNCTION_BLOCK FB_AtcAndon
VAR_INPUT
    Run : BOOL;
    Warning : BOOL;
    Alarm : BOOL;
    Fault : BOOL;
    Maintenance : BOOL;
    Changeover : BOOL;
END_VAR
VAR_OUTPUT
    Red : BOOL;
    Amber : BOOL;
    Green : BOOL;
    Blue : BOOL;
    Buzzer : BOOL;
END_VAR
VAR
    hi : BOOL;
END_VAR
    hi := Alarm OR Fault;
    Red := hi;
    Amber := Warning OR Maintenance;
    Green := Run AND NOT hi AND NOT Amber AND NOT Changeover;
    Blue := Changeover;
    Buzzer := hi;
END_FUNCTION_BLOCK

// ============================ recipe store ==========================

// FB_AtcRecipe — small non-volatile-style parameter store: 16 sets
// x 16 LREAL parameters addressed flat as mem[set*16 + param]
// (indices 0..255 internally). Not itself persistent — RETAIN is a
// program-level feature; the caller copies hot parameters to
// retained vars or NV storage on shutdown if needed.
//   WrStrobe rising edge writes WrVal into (WrSet, WrParam);
//   RdSet/RdParam continuously expose Val := mem[..] and Valid.
//   Clear zeroes the whole store. Typical use: operator downloads a
//   recipe -> write all params -> select set -> read params into
//   controller inputs each scan (or copy on set change).
FUNCTION_BLOCK FB_AtcRecipe
VAR_INPUT
    WrSet : DINT;        // 0..15
    WrParam : DINT;      // 0..15
    WrVal : LREAL;
    WrStrobe : BOOL;
    RdSet : DINT;
    RdParam : DINT;
    Clear : BOOL;
END_VAR
VAR_OUTPUT
    Val : LREAL;         // mem[RdSet,RdParam] (0 when invalid)
    Valid : BOOL;        // Rd indices in range
    Busy : BOOL;         // FALSE (kept for interface symmetry)
END_VAR
VAR
    mem : ARRAY[0..255] OF LREAL;
    wrLast : BOOL;
    i : DINT;
    off : DINT;
END_VAR
    IF Clear THEN
        FOR i := 0 TO 255 DO
            mem[i] := 0.0;
        END_FOR;
    END_IF;
    IF WrStrobe AND NOT wrLast THEN
        IF WrSet >= 0 AND WrSet <= 15 AND
           WrParam >= 0 AND WrParam <= 15 THEN
            off := WrSet * 16 + WrParam;
            mem[off] := WrVal;
        END_IF;
    END_IF;
    wrLast := WrStrobe;
    Valid := RdSet >= 0 AND RdSet <= 15 AND
             RdParam >= 0 AND RdParam <= 15;
    Val := 0.0;
    IF Valid THEN
        off := RdSet * 16 + RdParam;
        Val := mem[off];
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ==================== on/off with hysteresis ======================

// FB_AtcOnOff — bang-bang (thermostat) control with hysteresis.
//   PV <= SP - Hyst/2  -> Out := TRUE   (call it the "on" side)
//   PV >= SP + Hyst/2  -> Out := FALSE
//   inside the deadband Out holds its last state.
// Symmetric deadband works for heating and cooling — swap PV/SP
// roles for the inverse action. Enable=FALSE forces Out off.
// No CycleTime needed: evaluated once per scan; put a minimum
// on/off-time downstream (FB_AtcDebounce on Out, or the actuator's
// own staging) if the load can't cycle fast.
FUNCTION_BLOCK FB_AtcOnOff
VAR_INPUT
    PV : LREAL;
    SP : LREAL;
    Hyst : LREAL;        // total deadband width, EU
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    Out : BOOL;
END_VAR
VAR
    lo : LREAL;
    hi : LREAL;
END_VAR
    lo := SP - Hyst / 2.0;
    hi := SP + Hyst / 2.0;
    IF NOT Enable THEN
        Out := FALSE;
    ELSIF PV <= lo THEN
        Out := TRUE;
    ELSIF PV >= hi THEN
        Out := FALSE;
    END_IF;
END_FUNCTION_BLOCK

// ========================= permissive chain =======================

// FB_AtcPermissive — run-permit / interlock chain. All eight
// permissive inputs must be TRUE for Permit. DenyMask bit i is set
// while permissive i+1 is FALSE (bit0 = P1); FirstDeny is the lowest
// denying index (0 = none denying). Wiring guidance for safety
// chains: feed NOT-estop, NOT-guard, air-pressure-ok etc. as the P
// inputs; a bypass is deliberately NOT an input — bypass a permissive
// upstream (with its own guard) so this block stays a pure AND-tree
// whose DenyMask always shows the true first cause.
FUNCTION_BLOCK FB_AtcPermissive
VAR_INPUT
    P1 : BOOL; P2 : BOOL; P3 : BOOL; P4 : BOOL;
    P5 : BOOL; P6 : BOOL; P7 : BOOL; P8 : BOOL;
END_VAR
VAR_OUTPUT
    Permit : BOOL;
    DenyMask : DWORD;    // bit i-1 = permissive i denying
    FirstDeny : DINT;    // 1..8 lowest denying index, 0 = clear
END_VAR
VAR
    d : DWORD;
END_VAR
    d := 0;
    // bitwise OR isn't in the dialect — each position is added at
    // most once so arithmetic OR is exact here
    IF NOT P1 THEN d := d + 1; END_IF;
    IF NOT P2 THEN d := d + 2; END_IF;
    IF NOT P3 THEN d := d + 4; END_IF;
    IF NOT P4 THEN d := d + 8; END_IF;
    IF NOT P5 THEN d := d + 16; END_IF;
    IF NOT P6 THEN d := d + 32; END_IF;
    IF NOT P7 THEN d := d + 64; END_IF;
    IF NOT P8 THEN d := d + 128; END_IF;
    DenyMask := d;
    FirstDeny := 0;
    IF NOT P1 THEN FirstDeny := 1; END_IF;
    IF FirstDeny = 0 AND NOT P2 THEN FirstDeny := 2; END_IF;
    IF FirstDeny = 0 AND NOT P3 THEN FirstDeny := 3; END_IF;
    IF FirstDeny = 0 AND NOT P4 THEN FirstDeny := 4; END_IF;
    IF FirstDeny = 0 AND NOT P5 THEN FirstDeny := 5; END_IF;
    IF FirstDeny = 0 AND NOT P6 THEN FirstDeny := 6; END_IF;
    IF FirstDeny = 0 AND NOT P7 THEN FirstDeny := 7; END_IF;
    IF FirstDeny = 0 AND NOT P8 THEN FirstDeny := 8; END_IF;
    Permit := d = 0;
END_FUNCTION_BLOCK

// ============================ mode arbiter ========================

// FB_AtcMode — machine operating-mode arbitration. Requests are
// level inputs (key switch / HMI button held or latched upstream).
// Fixed priority Jog > Maint > Man > Auto; Mode reports the winning
// request as 0=off 1=auto 2=man 3=jog 4=maint with one-hot BOOL
// outputs. Changed pulses TRUE for one scan on every transition —
// wire it to logging or bumpless-transfer logic. No request -> Mode
// 0 (safe off state). The caller decides which outputs each mode
// gates; this block only arbitrates.
FUNCTION_BLOCK FB_AtcMode
VAR_INPUT
    ReqAuto : BOOL;
    ReqMan : BOOL;
    ReqJog : BOOL;
    ReqMaint : BOOL;
END_VAR
VAR_OUTPUT
    Mode : DINT;         // 0 off, 1 auto, 2 man, 3 jog, 4 maint
    Auto : BOOL;
    Man : BOOL;
    Jog : BOOL;
    Maint : BOOL;
    Changed : BOOL;
END_VAR
VAR
    m : DINT;
    last : DINT;
END_VAR
    m := 0;
    IF ReqAuto THEN m := 1; END_IF;
    IF ReqMan THEN m := 2; END_IF;
    IF ReqMaint THEN m := 4; END_IF;
    IF ReqJog THEN m := 3; END_IF;
    // explicit priority: jog > maint > man > auto — write highest
    // last so it wins
    Auto := m = 1;
    Man := m = 2;
    Jog := m = 3;
    Maint := m = 4;
    Changed := m <> last;
    last := m;
    Mode := m;
END_FUNCTION_BLOCK

// =============================== jog ===============================

// FB_AtcJog — inch/jog control with deadman and anti-run timeout.
//   DirIn: -1 reverse, 0 stop, +1 forward (any other value = stop).
//   Out follows DirIn while En (deadman/key) is held. Holding En
//   longer than MaxOn seconds drops Out and latches TimedOut until
//   En is released — the classic "jog must be re-pressed" rule that
//   prevents a stuck input running the axis. ET counts the current
//   enabled run time. CycleTime is required for the timeout.
FUNCTION_BLOCK FB_AtcJog
VAR_INPUT
    DirIn : DINT;        // -1 / 0 / +1
    En : BOOL;           // deadman / jog enable
    MaxOn : LREAL;       // s; <=0 disables timeout
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Out : BOOL;          // drive request this scan
    Dir : DINT;          // -1/0/+1 actually commanded
    TimedOut : BOOL;     // latched until En released
    ET : LREAL;          // s En has been continuously held
END_VAR
VAR
    tmo : BOOL;
    enLast : BOOL;
END_VAR
    IF NOT En THEN
        tmo := FALSE;
        ET := 0.0;
    ELSE
        IF NOT enLast THEN
            ET := 0.0;
        END_IF;
        ET := ET + CycleTime;
        IF MaxOn > 0.0 AND ET >= MaxOn THEN
            tmo := TRUE;
        END_IF;
    END_IF;
    enLast := En;
    TimedOut := tmo;
    IF En AND NOT tmo AND (DirIn = 1 OR DirIn = -1) THEN
        Out := TRUE;
        Dir := DirIn;
    ELSE
        Out := FALSE;
        Dir := 0;
    END_IF;
END_FUNCTION_BLOCK

// ============================ watchdog =============================

// FB_AtcWatchdog — heartbeat/communication supervisor. Kick must
// toggle (or stay retriggered) at least every Timeout seconds;
// each scan ET accumulates CycleTime since the last Kick edge and
// resets on a new edge. Err asserts while ET > Timeout — level, so
// OK returns automatically when comms resume (link-recovered
// semantics). For a latched fault use FB_AtcAlarm downstream.
// Typical wiring: Kick := comm-alive flag from the fieldbus/
// protocol layer; Err -> machine stop or degraded-mode request.
FUNCTION_BLOCK FB_AtcWatchdog
VAR_INPUT
    Kick : BOOL;
    Timeout : LREAL;     // s allowed between edges
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    OK : BOOL;
    Err : BOOL;
    ET : LREAL;          // s since last Kick edge
END_VAR
VAR
    last : BOOL;
END_VAR
    IF Reset THEN
        ET := 0.0;
        last := Kick;
    ELSE
        IF Kick AND NOT last THEN
            ET := 0.0;
        ELSE
            ET := ET + CycleTime;
        END_IF;
        last := Kick;
    END_IF;
    Err := Timeout > 0.0 AND ET > Timeout;
    OK := NOT Err;
END_FUNCTION_BLOCK

// ============================= blinker =============================

// FB_AtcBlinker — flash pattern generator for tower lights, HMI
// beacons, alarm lamps. Phase advances Rate cycles per second while
// En; Q is TRUE during the first Duty fraction of each cycle
// (Duty 0.5 = symmetric blink). En=FALSE forces Q off and rewinds
// Phase so re-enable starts at a known point. Rate<=0 or Duty<=0
// holds Q off; Duty>=1 holds Q on while En.
FUNCTION_BLOCK FB_AtcBlinker
VAR_INPUT
    En : BOOL;
    Rate : LREAL;        // Hz
    Duty : LREAL;        // 0..1 fraction of cycle Q is TRUE
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    Phase : LREAL;       // 0..1 position inside current cycle
END_VAR
VAR
    ph : LREAL;
    frac : LREAL;
END_VAR
    IF NOT En OR Rate <= 0.0 THEN
        ph := 0.0;
        Q := FALSE;
    ELSE
        ph := ph + Rate * CycleTime;
        frac := ph - DINT_TO_LREAL(TRUNC(ph));
        ph := frac;
        IF Duty >= 1.0 THEN
            Q := TRUE;
        ELSIF Duty <= 0.0 THEN
            Q := FALSE;
        ELSE
            Q := frac < Duty;
        END_IF;
        Phase := frac;
    END_IF;
END_FUNCTION_BLOCK

// =========================== totalizer =============================

// FB_AtcTotalizer — rate-to-total integrator: Total += Rate*
// CycleTime*K each scan. The workhorse behind pulse/flow/kWh/
// parts-per-hour totals: scale the raw rate (counts/s, kW, L/min)
// into engineering units with K. Preset alarms via FB_AtcHourMeter
// pattern or wire Total to a compare. Reset zeroes. Negative rates
// subtract (reversing flow / regeneration) — clamp Rate upstream if
// the process can't reverse.
FUNCTION_BLOCK FB_AtcTotalizer
VAR_INPUT
    Rate : LREAL;        // units per second (signed)
    K : LREAL;           // unit scale, default 1.0
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Total : LREAL;       // accumulated units
END_VAR
    IF Reset THEN
        Total := 0.0;
    ELSE
        Total := Total + Rate * CycleTime * K;
    END_IF;
END_FUNCTION_BLOCK

// ======================== moving average ==========================

// FB_AtcAvg — moving-average filter over the last N scans
// (N clamped 1..32). Ring buffer of LREAL samples; Avg is the mean
// of the filled window — during the first N-1 scans it averages
// what has arrived (Count reports samples seen so far). Use for
// noisy level/pressure/temperature signals where a Pt1 lag is too
// slow or you want a true windowed mean. N<=1 passes In through.
FUNCTION_BLOCK FB_AtcAvg
VAR_INPUT
    In : LREAL;
    N : DINT;            // window length, clamped 1..32
END_VAR
VAR_OUTPUT
    Avg : LREAL;
    Count : DINT;        // samples accumulated (<= N)
END_VAR
VAR
    buf : ARRAY[1..32] OF LREAL;
    idx : DINT;
    wn : DINT;
    i : DINT;
    sum : LREAL;
END_VAR
    wn := N;
    IF wn < 1 THEN wn := 1; END_IF;
    IF wn > 32 THEN wn := 32; END_IF;
    idx := idx + 1;
    IF idx > wn THEN idx := 1; END_IF;
    buf[idx] := In;
    IF Count < wn THEN Count := Count + 1; END_IF;
    sum := 0.0;
    FOR i := 1 TO Count DO
        sum := sum + buf[i];
    END_FOR;
    IF Count > 0 THEN
        Avg := sum / DINT_TO_LREAL(Count);
    ELSE
        Avg := 0.0;
    END_IF;
END_FUNCTION_BLOCK

// =========================== demand ===============================

// FB_AtcDemand — sliding-window average (demand) over the last
// Window seconds, sampled once per scan. Power (or any rate) goes
// in kW; Demand is the mean over the window — the classic
// kW-demand / load-profile block behind demand-charge and
// peak-load stories. Ring buffer holds up to 64 samples so the
// effective window is min(Window, 64*CycleTime); keep CycleTime
// small relative to Window. First scans average what has arrived.
FUNCTION_BLOCK FB_AtcDemand
VAR_INPUT
    Power : LREAL;       // kW (or any rate unit)
    Window : LREAL;      // s
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Demand : LREAL;      // windowed mean
    Samples : DINT;      // samples in window so far
END_VAR
VAR
    buf : ARRAY[1..64] OF LREAL;
    idx : DINT;
    n : DINT;
    i : DINT;
    sum : LREAL;
END_VAR
    n := 1;
    IF CycleTime > 0.0 THEN
        n := TRUNC(Window / CycleTime);
    END_IF;
    IF n < 1 THEN n := 1; END_IF;
    IF n > 64 THEN n := 64; END_IF;
    IF Reset THEN
        FOR i := 1 TO 64 DO buf[i] := 0.0; END_FOR;
        idx := 0;
        Samples := 0;
        Demand := 0.0;
        RETURN;
    END_IF;
    idx := idx + 1;
    IF idx > n THEN idx := 1; END_IF;
    buf[idx] := Power;
    IF Samples < n THEN Samples := Samples + 1; END_IF;
    sum := 0.0;
    FOR i := 1 TO Samples DO
        sum := sum + buf[i];
    END_FOR;
    IF Samples > 0 THEN
        Demand := sum / DINT_TO_LREAL(Samples);
    ELSE
        Demand := 0.0;
    END_IF;
END_FUNCTION_BLOCK

// =========================== override =============================

// FB_AtcOverride — manual override / release for any final output.
//   ManEn TRUE  -> Out := ManIn  (operator has the loop)
//   ManEn FALSE -> Out := AutoIn (auto value passes through)
// On the ManEn falling edge the block latches TrackIn for one scan
// — feed TrackIn from the controller's Track/PV so the loop
// re-syncs bumplessly before ManEn is released upstream. Err stays
// FALSE (kept for interface symmetry with the controller blocks).
// Typical use: HMI "hand/auto" switch on heater, valve, drive
// enable — independent of which controller produced AutoIn.
FUNCTION_BLOCK FB_AtcOverride
VAR_INPUT
    AutoIn : LREAL;
    ManIn : LREAL;
    ManEn : BOOL;
    TrackIn : LREAL;     // value to hold during release sync
END_VAR
VAR_OUTPUT
    Out : LREAL;
    Tracking : BOOL;     // TRUE on the release-sync scan
    Err : BOOL;
END_VAR
VAR
    last : BOOL;
END_VAR
    Tracking := FALSE;
    IF ManEn THEN
        Out := ManIn;
    ELSE
        IF last THEN
            // just released: hold TrackIn this scan for bumpless
            // re-entry (caller re-enables auto next scan)
            Out := TrackIn;
            Tracking := TRUE;
        ELSE
            Out := AutoIn;
        END_IF;
    END_IF;
    last := ManEn;
    Err := FALSE;
END_FUNCTION_BLOCK

// =========================== two-hand =============================

// FB_AtcTwoHand — two-hand anti-tie-down control (safety). Both
// buttons must be pressed within MaxGap seconds of each other;
// once armed, Out stays TRUE until EITHER button releases. A button
// held while the other is pressed does NOT re-arm (anti-tie-down:
// releasing one and re-pressing requires both to be released first
// — wait, classic anti-tie-down means you can't keep one pressed
// and tap the other: both must be released before a new cycle).
// CycleTime drives the gap timer. Out is a one-scan-level permit —
// gate it through the machine's safety chain, never wire it
// straight to a hazard without the safety relay layer.
FUNCTION_BLOCK FB_AtcTwoHand
VAR_INPUT
    Btn1 : BOOL;
    Btn2 : BOOL;
    MaxGap : LREAL;      // s allowed between the two presses
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Out : BOOL;          // permit while both held after valid press
    Armed : BOOL;        // first button pressed, waiting for second
    GapET : LREAL;       // s since first press while armed
END_VAR
VAR
    b1Last : BOOL;
    b2Last : BOOL;
    gapT : LREAL;
    bothRel : BOOL;
END_VAR
    bothRel := NOT Btn1 AND NOT Btn2;
    IF Reset OR bothRel THEN
        gapT := 0.0;
        Armed := FALSE;
    END_IF;
    // first press starts the gap timer
    IF Btn1 AND NOT b1Last AND NOT Btn2 THEN
        gapT := 0.0;
        Armed := TRUE;
    END_IF;
    IF Btn2 AND NOT b2Last AND NOT Btn1 THEN
        gapT := 0.0;
        Armed := TRUE;
    END_IF;
    IF Armed THEN
        gapT := gapT + CycleTime;
        IF gapT > MaxGap THEN
            Armed := FALSE;   // second press too late
        END_IF;
    END_IF;
    // both pressed inside the gap -> permit while both held
    IF Btn1 AND Btn2 AND (gapT <= MaxGap) THEN
        Out := TRUE;
    ELSE
        Out := FALSE;
    END_IF;
    GapET := gapT;
    b1Last := Btn1;
    b2Last := Btn2;
END_FUNCTION_BLOCK

// ---- FB_AtcShift: schedule-based shift timer --------------------------
// Three shifts defined by start/end times-of-day in seconds [0,86400).
// END <= START wraps midnight (night shift). TOD is supplied by the
// caller (HMI, clock FB, or accumulated CycleTime) — the block is
// wall-clock agnostic and fully deterministic.
// Outputs: Shift (0=off,1..3), InShift, ShiftStart/ShiftEnd pulses,
// Elapsed s into shift, Remaining s to shift end.
// ShiftStart is the hook for per-shift counter resets / OEE rollovers.
FUNCTION_BLOCK FB_AtcShift
VAR_INPUT
    TOD : LREAL;          // s since local midnight [0,86400)
    Enable : BOOL;
    S1Start : LREAL; S1End : LREAL;
    S2Start : LREAL; S2End : LREAL;
    S3Start : LREAL; S3End : LREAL;
END_VAR
VAR_OUTPUT
    Shift : INT;          // 0 = between shifts, 1..3
    InShift : BOOL;
    ShiftStart : BOOL;    // pulse: entered a shift this scan
    ShiftEnd : BOOL;      // pulse: left a shift this scan
    Elapsed : LREAL;      // s since shift start (TOD domain)
    Remaining : LREAL;    // s until shift end
END_VAR
VAR
    lastShift : INT;
    wrk : LREAL;
    shStart : LREAL;
    shEnd : LREAL;
END_VAR
    ShiftStart := FALSE;
    ShiftEnd := FALSE;
    IF NOT Enable THEN
        Shift := 0; InShift := FALSE; Elapsed := 0.0; Remaining := 0.0;
        lastShift := 0;
        RETURN;
    END_IF;
    // normalize TOD into [0,86400)
    wrk := TOD;
    WHILE wrk < 0.0 DO wrk := wrk + 86400.0; END_WHILE
    WHILE wrk >= 86400.0 DO wrk := wrk - 86400.0; END_WHILE
    // find active shift: END < START wraps midnight; END = START disables
    Shift := 0; shStart := 0.0; shEnd := 0.0;
    // shift 1
    IF S1End > S1Start THEN
        IF wrk >= S1Start AND wrk < S1End THEN
            Shift := 1; shStart := S1Start; shEnd := S1End;
        END_IF;
    ELSIF S1End < S1Start THEN
        IF wrk >= S1Start OR wrk < S1End THEN
            Shift := 1; shStart := S1Start; shEnd := S1End;
        END_IF;
    END_IF;
    // shift 2
    IF Shift = 0 AND S2End > S2Start THEN
        IF wrk >= S2Start AND wrk < S2End THEN
            Shift := 2; shStart := S2Start; shEnd := S2End;
        END_IF;
    ELSIF Shift = 0 AND S2End < S2Start THEN
        IF wrk >= S2Start OR wrk < S2End THEN
            Shift := 2; shStart := S2Start; shEnd := S2End;
        END_IF;
    END_IF;
    // shift 3
    IF Shift = 0 AND S3End > S3Start THEN
        IF wrk >= S3Start AND wrk < S3End THEN
            Shift := 3; shStart := S3Start; shEnd := S3End;
        END_IF;
    ELSIF Shift = 0 AND S3End < S3Start THEN
        IF wrk >= S3Start OR wrk < S3End THEN
            Shift := 3; shStart := S3Start; shEnd := S3End;
        END_IF;
    END_IF;
    InShift := Shift > 0;
    // elapsed/remaining in shift-local time (handles wrap)
    IF InShift THEN
        Elapsed := wrk - shStart;
        IF Elapsed < 0.0 THEN Elapsed := Elapsed + 86400.0; END_IF;
        Remaining := shEnd - wrk;
        IF Remaining <= 0.0 THEN Remaining := Remaining + 86400.0; END_IF;
    ELSE
        Elapsed := 0.0; Remaining := 0.0;
    END_IF;
    // edge detect vs last scan — also fires on TOD jumps
    IF Shift <> lastShift THEN
        IF Shift > 0 THEN ShiftStart := TRUE;
        ELSE ShiftEnd := TRUE; END_IF;
    END_IF;
    lastShift := Shift;
END_FUNCTION_BLOCK

// ---- FB_AtcDefrost: cooler defrost sequencer (US-004) ----------------
// Timed defrost every DefrostEvery s of compressor runtime: kills the
// fridge output, energizes the defrost heater for DefrostTime s, then
// holds the compressor off for MinOffTime s (anti-short-cycle).
// DoorOpen/Warm are pre-conditioned inputs (switch / temp compare).
// DoorAlarm latches after DoorAlarmTime s ajar while fridge runs;
// TempAlarm latches after WarmTime s warm; Ack clears both.
FUNCTION_BLOCK FB_AtcDefrost
VAR_INPUT
    Enable : BOOL;
    DoorOpen : BOOL;
    Warm : BOOL;          // cooler temp above alarm threshold
    Ack : BOOL;           // alarm acknowledge
    DefrostEvery : LREAL; // s between defrost starts (compressor time)
    DefrostTime : LREAL;  // s heater on
    MinOff : LREAL;       // s compressor off after defrost
    DoorAlarmTime : LREAL;// s door ajar while fridge runs
    WarmTime : LREAL;     // s warm before TempAlarm
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Fridge : BOOL;        // compressor enable
    DefrostHeat : BOOL;
    InDefrost : BOOL;
    DoorAlarm : BOOL;
    TempAlarm : BOOL;
    DefrostCount : DINT;
END_VAR
VAR
    coolT : LREAL;
    defT : LREAL;
    offT : LREAL;
    doorT : LREAL;
    warmT : LREAL;
    st : INT;             // 0 cool-run, 1 defrost, 2 off-delay
    fridgeOn : BOOL;      // mirrors Fridge for same-scan reads
    enLast : BOOL;
END_VAR
    IF NOT Enable THEN
        Fridge := FALSE; DefrostHeat := FALSE; InDefrost := FALSE;
        DoorAlarm := FALSE; TempAlarm := FALSE;
        coolT := 0.0; defT := 0.0; offT := 0.0;
        doorT := 0.0; warmT := 0.0; st := 0;
        fridgeOn := FALSE;
        enLast := FALSE;
        RETURN;
    END_IF;
    // alarm timers + ack — read fridgeOn (internal), not the Fridge
    // output, so the value is valid before this scan's assignment
    IF DoorOpen AND fridgeOn THEN
        doorT := doorT + CycleTime;
        IF doorT >= DoorAlarmTime THEN DoorAlarm := TRUE; END_IF;
    ELSE
        doorT := 0.0;
    END_IF;
    IF Warm THEN
        warmT := warmT + CycleTime;
        IF warmT >= WarmTime THEN TempAlarm := TRUE; END_IF;
    ELSE
        warmT := 0.0;
    END_IF;
    IF Ack THEN DoorAlarm := FALSE; TempAlarm := FALSE; END_IF;
    // state machine
    IF st = 0 THEN
        Fridge := TRUE; fridgeOn := TRUE;
        DefrostHeat := FALSE; InDefrost := FALSE;
        coolT := coolT + CycleTime;
        IF coolT >= DefrostEvery AND NOT DoorOpen THEN
            st := 1; defT := 0.0; Fridge := FALSE; fridgeOn := FALSE;
            DefrostHeat := TRUE; InDefrost := TRUE;
            DefrostCount := DefrostCount + 1;
        END_IF;
    ELSIF st = 1 THEN
        Fridge := FALSE; fridgeOn := FALSE;
        DefrostHeat := TRUE; InDefrost := TRUE;
        defT := defT + CycleTime;
        IF defT >= DefrostTime OR DoorOpen THEN
            // door opened mid-defrost: abort early, still off-delay
            st := 2; offT := 0.0; DefrostHeat := FALSE; InDefrost := FALSE;
        END_IF;
    ELSE
        Fridge := FALSE; fridgeOn := FALSE;
        DefrostHeat := FALSE; InDefrost := FALSE;
        offT := offT + CycleTime;
        coolT := 0.0;
        IF offT >= MinOff THEN st := 0; END_IF;
    END_IF;
    enLast := Enable;
END_FUNCTION_BLOCK

// ---- FB_AtcZoneBalance: master/slave trim (US-008) --------------------
// Master zone error (PV - SP) produces a bounded, rate-limited trim
// added to slave zone setpoints. Trim saturates at TrimMax and flags
// Sat so the HMI can show windup; Enable rising starts bumpless at 0.
FUNCTION_BLOCK FB_AtcZoneBalance
VAR_INPUT
    MasterErr : LREAL;   // master PV - SP, degC
    Enable : BOOL;
    Gain : LREAL;       // trim per degC of master error
    TrimMax : LREAL;    // |trim| limit, degC
    RateMax : LREAL;    // max |dTrim/dt|, degC/s (0 = unlimited)
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    TrimOut : LREAL;    // add to slave SPs
    Sat : BOOL;         // trim at a limit
END_VAR
VAR
    tr : LREAL;
    dt : LREAL;
    want : LREAL;
    g : LREAL;
    tm : LREAL;
    enLast : BOOL;
END_VAR
    g := Gain;
    tm := TrimMax;
    IF tm <= 0.0 THEN tm := 10.0; END_IF;
    IF NOT Enable THEN
        tr := 0.0; Sat := FALSE; enLast := FALSE;
        TrimOut := 0.0;
        RETURN;
    END_IF;
    IF NOT enLast THEN tr := 0.0; END_IF;
    enLast := TRUE;
    want := g * MasterErr;
    IF want > tm THEN want := tm; END_IF;
    IF want < -tm THEN want := -tm; END_IF;
    dt := want - tr;
    IF RateMax > 0.0 THEN
        IF dt > RateMax * CycleTime THEN
            dt := RateMax * CycleTime;
        END_IF;
        IF dt < -RateMax * CycleTime THEN
            dt := -RateMax * CycleTime;
        END_IF;
    END_IF;
    tr := tr + dt;
    IF tr > tm THEN tr := tm; END_IF;
    IF tr < -tm THEN tr := -tm; END_IF;
    Sat := (tr >= tm - 0.0001) OR (tr <= -tm + 0.0001);
    TrimOut := tr;
END_FUNCTION_BLOCK

// ---- FB_AtcEndpoint: pressure-rise dryness test (US-009) --------------
// Freeze-dryer endpoint: close the isolation valve, let pressure
// settle, then sample chamber pressure for SampleTime. A fast rise
// means residual moisture (wet); slow rise means dry. Result:
// 0 none, 1 dry, 2 wet. ValveCmd stays FALSE for the whole test.
FUNCTION_BLOCK FB_AtcEndpoint
VAR_INPUT
    Execute : BOOL;     // start test (rising edge)
    Pressure : LREAL;   // chamber pressure, any unit
    SettleTime : LREAL; // s valve-closed settle before sampling
    SampleTime : LREAL; // s pressure-rise sampling window
    MaxRise : LREAL;    // max dP/dt for 'dry' (pressure/s)
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    ValveCmd : BOOL;    // TRUE = open; FALSE = isolate
    Testing : BOOL;
    Result : INT;       // 0 none, 1 dry, 2 wet
    RiseRate : LREAL;   // measured dP/dt over the window
END_VAR
VAR
    st : INT;           // 0 idle, 1 settle, 2 sample, 3 done
    t : LREAL;
    pRef : LREAL;
    exLast : BOOL;
END_VAR
    IF Reset THEN
        st := 0; t := 0.0; pRef := 0.0;
        ValveCmd := TRUE; Testing := FALSE;
        Result := 0; RiseRate := 0.0;
        exLast := Execute;
        RETURN;
    END_IF;
    IF Execute AND NOT exLast AND st = 0 THEN
        st := 1; t := 0.0; Result := 0; RiseRate := 0.0;
    END_IF;
    exLast := Execute;
    IF st = 0 THEN
        ValveCmd := TRUE; Testing := FALSE;
    ELSIF st = 1 THEN
        ValveCmd := FALSE; Testing := TRUE;
        t := t + CycleTime;
        IF t >= SettleTime THEN
            st := 2; t := 0.0; pRef := Pressure;
        END_IF;
    ELSIF st = 2 THEN
        ValveCmd := FALSE; Testing := TRUE;
        t := t + CycleTime;
        IF t >= SampleTime THEN
            st := 3;
            IF SampleTime > 0.0 THEN
                RiseRate := (Pressure - pRef) / SampleTime;
            ELSE
                RiseRate := 0.0;
            END_IF;
            IF RiseRate <= MaxRise THEN Result := 1;
            ELSE Result := 2; END_IF;
        END_IF;
    ELSE
        // done: hold result, valve open again
        ValveCmd := TRUE; Testing := FALSE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_AtcLeadLag: pump/compressor staging --------------------------
// Duty or level demand stages identical units lead/lag with runtime
// rotation: the least-used unit leads, lag units start as demand
// rises and stop in reverse order as it falls. StageDelay seconds
// must elapse between stage changes (motor start settle, inrush).
// Demand is in whole capacity units per stage (TRUNC). Hours
// accumulate per stage so rotation stays fair over weeks.
FUNCTION_BLOCK FB_AtcLeadLag
VAR_INPUT
    Demand : LREAL;      // required capacity, units of one stage
    NumUnits : INT;      // identical staged units, 1..8
    StageDelay : LREAL;  // s between stage changes
    Enable : BOOL;
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Stage : ARRAY[1..8] OF BOOL; // stage i commanded on
    NumOn : INT;                 // stages currently on
    LeadIdx : INT;               // 1-based index of lead stage
END_VAR
VAR
    req : INT;
    onT : INT;
    onTLast : INT;
    pendT : LREAL;
    hrs : ARRAY[1..8] OF LREAL;
    sel : ARRAY[1..8] OF BOOL;
    nu : INT;
    i : INT;
    k : INT;
    best : INT;
    bestH : LREAL;
END_VAR
    nu := NumUnits;
    IF nu < 1 THEN nu := 1; END_IF;
    IF nu > 8 THEN nu := 8; END_IF;
    IF Reset OR NOT Enable THEN
        FOR i := 1 TO 8 DO
            Stage[i] := FALSE; hrs[i] := 0.0;
        END_FOR;
        onT := 0; onTLast := -1; pendT := 0.0; LeadIdx := 1; NumOn := 0;
        FOR i := 1 TO 8 DO sel[i] := FALSE; END_FOR;
        IF NOT Enable THEN RETURN; END_IF;
    END_IF;
    req := DINT_TO_INT(TRUNC(Demand));
    IF req < 0 THEN req := 0; END_IF;
    IF req > nu THEN req := nu; END_IF;
    pendT := pendT + CycleTime;
    IF pendT >= StageDelay THEN
        IF onT < req THEN
            onT := onT + 1; pendT := 0.0;
        ELSIF onT > req THEN
            onT := onT - 1; pendT := 0.0;
        END_IF;
    END_IF;
    // re-select only when the stage count changes: pick the onT
    // units with the least accumulated hours (ties -> low index).
    // Selection then holds until the next change so units do not
    // swap every scan — rotation equalizes across demand changes.
    IF onT <> onTLast THEN
        FOR i := 1 TO 8 DO sel[i] := FALSE; END_FOR;
        FOR k := 1 TO onT DO
            bestH := 1.0e30; best := 0;
            FOR i := 1 TO nu DO
                IF NOT sel[i] AND hrs[i] < bestH THEN
                    bestH := hrs[i]; best := i;
                END_IF;
            END_FOR;
            IF best > 0 THEN sel[best] := TRUE; END_IF;
        END_FOR;
        onTLast := onT;
    END_IF;
    FOR i := 1 TO 8 DO
        IF i <= nu THEN Stage[i] := sel[i]; ELSE Stage[i] := FALSE; END_IF;
    END_FOR;
    NumOn := onT;
    LeadIdx := 0;
    FOR i := 1 TO nu DO
        IF Stage[i] AND LeadIdx = 0 THEN LeadIdx := i; END_IF;
    END_FOR;
    IF LeadIdx = 0 THEN LeadIdx := 1; END_IF;
    FOR i := 1 TO nu DO
        IF Stage[i] THEN hrs[i] := hrs[i] + CycleTime / 3600.0; END_IF;
    END_FOR;
END_FUNCTION_BLOCK

// ---- FB_AtcToolLife: tool/cycle life tracking ------------------------
// Counts machine cycles (Cycle rising edge) and run hours while
// Enable. PctUsed = Cycles/LifeLimit*100 for HMI bars; Warn asserts
// at WarnPct % used; Due latches at the limit until Reset (tool
// change). Pair with FB_AtcHourMeter for calendar-based service.
FUNCTION_BLOCK FB_AtcToolLife
VAR_INPUT
    Cycle : BOOL;       // cycle-complete signal (edge detected)
    Enable : BOOL;      // machine running (hours accumulate)
    LifeLimit : DINT;   // rated cycles before change due (<=0 = off)
    WarnPct : LREAL;    // % used where Warn asserts (e.g. 80)
    CycleTime : LREAL;
    Reset : BOOL;       // tool changed -> clear counters
END_VAR
VAR_OUTPUT
    Cycles : DINT;
    Hours : LREAL;
    PctUsed : LREAL;    // 0..100+, -1 when LifeLimit<=0
    Warn : BOOL;
    Due : BOOL;
END_VAR
VAR
    cycLast : BOOL;
END_VAR
    IF Reset THEN
        Cycles := 0; Hours := 0.0; Due := FALSE;
    END_IF;
    IF Enable THEN
        Hours := Hours + CycleTime / 3600.0;
        IF Cycle AND NOT cycLast THEN Cycles := Cycles + 1; END_IF;
    END_IF;
    cycLast := Cycle;
    IF LifeLimit > 0 THEN
        PctUsed := DINT_TO_LREAL(Cycles) * 100.0 / DINT_TO_LREAL(LifeLimit);
        Warn := PctUsed >= WarnPct;
        IF Cycles >= LifeLimit THEN Due := TRUE; END_IF;
    ELSE
        PctUsed := -1.0; Warn := FALSE; Due := FALSE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_AtcSoftLimit: position window + homing supervision -----------
// Software limit monitoring for servo/stepper axes: PosValid only
// after Homed (position meaningless before homing); InWindow /
// OverPos / UnderPos classify Pos against [MinPos,MaxPos]; Fault
// latches the first violation while Enable; NeedHome prompts the
// operator/HMI to home after power-up or fault.
FUNCTION_BLOCK FB_AtcSoftLimit
VAR_INPUT
    Pos : LREAL;
    MinPos : LREAL;
    MaxPos : LREAL;
    Homed : BOOL;       // homing complete (from drive/PLC logic)
    Enable : BOOL;      // axis enabled / motion permitted
    Reset : BOOL;       // clear latched Fault
END_VAR
VAR_OUTPUT
    PosValid : BOOL;    // position trusted (Homed)
    InWindow : BOOL;
    OverPos : BOOL;
    UnderPos : BOOL;
    Fault : BOOL;       // latched window violation
    NeedHome : BOOL;    // enabled but not homed
END_VAR
VAR
    lo : LREAL;
    hi : LREAL;
    tmp : LREAL;
END_VAR
    lo := MinPos; hi := MaxPos;
    IF hi < lo THEN
        tmp := lo; lo := hi; hi := tmp;
    END_IF;
    PosValid := Homed;
    OverPos := Homed AND Pos > hi;
    UnderPos := Homed AND Pos < lo;
    InWindow := Homed AND Pos >= lo AND Pos <= hi;
    NeedHome := Enable AND NOT Homed;
    IF Reset THEN Fault := FALSE; END_IF;
    IF Enable AND (OverPos OR UnderPos) THEN Fault := TRUE; END_IF;
END_FUNCTION_BLOCK

// ---- FB_AtcDeviation: PV-SP band alarm with on/off delay -------------
// Dev = |PV - SP| > Band (instantaneous, for HMI). Alarm latches
// after the deviation persists OnDelay s and clears only after the
// loop is back inside Band for OffDelay s (OffDelay<=0 clears
// immediately) — the standard temp-deviation alarm for batch records.
FUNCTION_BLOCK FB_AtcDeviation
VAR_INPUT
    PV : LREAL;
    SP : LREAL;
    Band : LREAL;       // allowed |PV-SP|, EU
    OnDelay : LREAL;    // s in deviation before Alarm
    OffDelay : LREAL;   // s in band before clear (<=0 immediate)
    Enable : BOOL;
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Dev : BOOL;         // instantaneous out-of-band
    Alarm : BOOL;       // latched deviation alarm
    ET : LREAL;         // s continuously out of band
END_VAR
VAR
    inT : LREAL;
    outT : LREAL;
    bnd : LREAL;
END_VAR
    bnd := Band;
    IF bnd < 0.0 THEN bnd := 0.0; END_IF;
    IF Reset THEN Alarm := FALSE; ET := 0.0; inT := 0.0; outT := 0.0; END_IF;
    IF NOT Enable THEN
        Dev := FALSE; ET := 0.0; inT := 0.0; outT := 0.0;
        RETURN;
    END_IF;
    IF PV - SP > bnd OR SP - PV > bnd THEN
        Dev := TRUE;
        ET := ET + CycleTime;
        inT := 0.0;
        outT := outT + CycleTime;
        IF ET >= OnDelay THEN Alarm := TRUE; END_IF;
    ELSE
        Dev := FALSE;
        ET := 0.0;
        outT := 0.0;
        inT := inT + CycleTime;
        IF Alarm AND inT >= OffDelay THEN Alarm := FALSE; END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_AtcDualChannel: two-channel input agreement ------------------
// Safety input redundancy (E-stop, safety door, light curtain OSSD):
// both channels must agree. A mismatch shorter than MismatchTime is
// contact bounce; longer is a fault — Fault latches until Reset.
// Safe = both channels in the safe state AND no latched fault.
// No bypass by design — wire safety contacts, not logic tricks.
FUNCTION_BLOCK FB_AtcDualChannel
VAR_INPUT
    In1 : BOOL;
    In2 : BOOL;
    MismatchTime : LREAL; // s of disagreement before Fault
    Enable : BOOL;
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Agree : BOOL;      // channels currently equal
    Safe : BOOL;       // both TRUE and no fault
    Fault : BOOL;      // latched disagreement
    ET : LREAL;        // s of current disagreement
END_VAR
VAR
    mm : LREAL;
END_VAR
    mm := MismatchTime;
    IF mm < 0.0 THEN mm := 0.0; END_IF;
    IF Reset THEN Fault := FALSE; ET := 0.0; END_IF;
    IF NOT Enable THEN
        Agree := FALSE; Safe := FALSE; ET := 0.0;
        RETURN;
    END_IF;
    IF In1 = In2 THEN
        Agree := TRUE; ET := 0.0;
    ELSE
        Agree := FALSE;
        ET := ET + CycleTime;
        IF ET >= mm THEN Fault := TRUE; END_IF;
    END_IF;
    Safe := In1 AND In2 AND NOT Fault;
END_FUNCTION_BLOCK

// ---- FB_AtcMuting: safety-device muting with restart inhibit ---------
// Light-curtain/safety-mat muting: MutingActive (material present /
// vacuum / guard open authorisation) mutes the protective outputs so
// the machine can run while material passes. SafeRun = SafeIn OR
// (MutingActive AND NOT MutingBad). When MutingActive drops, SafeRun
// stays FALSE until SafeIn is clear — no restart into the zone.
// MutingBad flags muting requested while the device is already clear
// (wiring fault or misuse).
FUNCTION_BLOCK FB_AtcMuting
VAR_INPUT
    SafeIn : BOOL;       // protective device clear (TRUE = clear)
    MutingActive : BOOL; // material/vacuum authorising the mute
    Enable : BOOL;
    CycleTime : LREAL;   // unused, kept for interface symmetry
END_VAR
VAR_OUTPUT
    MuteOut : BOOL;      // mute the protective outputs
    SafeRun : BOOL;      // machine may run
    MutingBad : BOOL;    // mute requested while device clear
END_VAR
    MuteOut := Enable AND MutingActive;
    MutingBad := Enable AND MutingActive AND SafeIn;
    // SafeRun while the device is clear, or while legitimately
    // muted. When MutingActive drops, SafeIn alone governs again —
    // a still-blocked device keeps SafeRun FALSE (restart inhibit).
    SafeRun := Enable AND (SafeIn OR MutingActive);
END_FUNCTION_BLOCK

// ---- FB_AtcValveFb: valve/pneumatic feedback verification ------------
// Compares commanded vs actual feedback; a mismatch persisting
// FbDelay s latches Fault (until Reset). Stuck-open and stuck-closed
// both detected — direction is irrelevant for interlock purposes.
// OK = enabled, no fault, command and feedback agree.
FUNCTION_BLOCK FB_AtcValveFb
VAR_INPUT
    Cmd : BOOL;          // valve command (TRUE = open/energise)
    Fb : BOOL;           // valve feedback (TRUE = open/energised)
    FbDelay : LREAL;     // s allowed for feedback to follow
    Enable : BOOL;
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Fault : BOOL;        // latched Cmd/Fb mismatch
    OK : BOOL;           // command verified
    ET : LREAL;          // s of current mismatch
END_VAR
VAR
    dl : LREAL;
END_VAR
    dl := FbDelay;
    IF dl < 0.0 THEN dl := 0.0; END_IF;
    IF Reset THEN Fault := FALSE; ET := 0.0; END_IF;
    IF NOT Enable THEN
        Fault := FALSE; OK := FALSE; ET := 0.0;
        RETURN;
    END_IF;
    IF Cmd = Fb THEN
        ET := 0.0;
    ELSE
        ET := ET + CycleTime;
        IF ET >= dl THEN Fault := TRUE; END_IF;
    END_IF;
    OK := Cmd = Fb AND NOT Fault;
END_FUNCTION_BLOCK

// ---- FB_AtcVote: redundant analog input voting -----------------------
// Median-of-3 for redundant transmitters (US-020): Vout is the
// middle value so a single drifting/frozen probe cannot move the
// process value. Spread = max-min; Bad asserts while Spread > Band
// (two probes disagree — maintenance needed). BadVote optionally
// substitutes a caller default when Bad.
FUNCTION_BLOCK FB_AtcVote
VAR_INPUT
    V1 : LREAL;
    V2 : LREAL;
    V3 : LREAL;
    Band : LREAL;        // max allowed spread, EU
    UseDefault : BOOL;   // substitute DefV when Bad
    DefV : LREAL;        // fallback value
END_VAR
VAR_OUTPUT
    Vout : LREAL;        // median (or DefV when UseDefault AND Bad)
    Spread : LREAL;      // max - min of the three inputs
    Bad : BOOL;          // Spread > Band
END_VAR
VAR
    a : LREAL;
    b : LREAL;
    c : LREAL;
    tmp : LREAL;
END_VAR
    a := V1; b := V2; c := V3;
    // order a<=b<=c with three compare-exchanges
    IF a > b THEN
        tmp := a; a := b; b := tmp;
    END_IF;
    IF b > c THEN
        tmp := b; b := c; c := tmp;
    END_IF;
    IF a > b THEN
        tmp := a; a := b; b := tmp;
    END_IF;
    Vout := b;
    Spread := c - a;
    Bad := Spread > Band;
    IF Bad AND UseDefault THEN Vout := DefV; END_IF;
END_FUNCTION_BLOCK

// ---- FB_AtcLoto: lockout-tagout permit enforcement -------------------
// L1..L8 are per-lock "locked" signals (TRUE = lock applied). Permit
// asserts only when EVERY lock is released — no bypass input by
// design (bypass belongs upstream in a guarded procedure step).
// LockedCount / FirstLock feed the HMI "which lock is still on".
FUNCTION_BLOCK FB_AtcLoto
VAR_INPUT
    L1 : BOOL; L2 : BOOL; L3 : BOOL; L4 : BOOL;
    L5 : BOOL; L6 : BOOL; L7 : BOOL; L8 : BOOL;
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    Permit : BOOL;        // all locks released
    LockedCount : INT;    // how many locks still applied
    FirstLock : INT;      // lowest locked index, 0 = none
END_VAR
    LockedCount := 0; FirstLock := 0;
    IF L1 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 1; END_IF; END_IF;
    IF L2 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 2; END_IF; END_IF;
    IF L3 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 3; END_IF; END_IF;
    IF L4 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 4; END_IF; END_IF;
    IF L5 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 5; END_IF; END_IF;
    IF L6 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 6; END_IF; END_IF;
    IF L7 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 7; END_IF; END_IF;
    IF L8 THEN LockedCount := LockedCount + 1; IF FirstLock = 0 THEN FirstLock := 8; END_IF; END_IF;
    Permit := Enable AND LockedCount = 0;
END_FUNCTION_BLOCK

// ---- FB_AtcCounter: preset up/down counter ---------------------------
// Case/parts/bin counts: Up/Down are level inputs edge-detected inside.
// Count clamps to [0, Preset]; AtPreset = Count >= Preset ( Preset<=0
// disables the clamp and the flag). Zero-detect for empty-bin logic.
FUNCTION_BLOCK FB_AtcCounter
VAR_INPUT
    Up : BOOL;
    Down : BOOL;
    Preset : DINT;      // count target (<=0 = no preset)
    Enable : BOOL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Count : DINT;
    AtPreset : BOOL;
    Zero : BOOL;
END_VAR
VAR
    upL : BOOL;
    dnL : BOOL;
END_VAR
    IF Reset THEN Count := 0; END_IF;
    IF Enable THEN
        IF Up AND NOT upL THEN
            Count := Count + 1;
        END_IF;
        IF Down AND NOT dnL THEN
            Count := Count - 1;
        END_IF;
        IF Preset > 0 THEN
            IF Count > Preset THEN Count := Preset; END_IF;
            IF Count < 0 THEN Count := 0; END_IF;
        END_IF;
    END_IF;
    upL := Up; dnL := Down;
    AtPreset := Preset > 0 AND Count >= Preset;
    Zero := Count <= 0;
END_FUNCTION_BLOCK

// ---- FB_AtcRateAlarm: PV rate-of-change alarm ------------------------
// Exotherm/runaway/coil-burn detection: Alarm latches after |dPV/dt|
// exceeds MaxRate for OnDelay s and clears after RateNow stays inside
// for OffDelay s. RateNow is EU/s (signed). First scan RateNow = 0.
FUNCTION_BLOCK FB_AtcRateAlarm
VAR_INPUT
    PV : LREAL;
    MaxRate : LREAL;    // |EU/s| limit (<=0 disables alarming)
    OnDelay : LREAL;    // s over-rate before Alarm
    OffDelay : LREAL;   // s in-rate before clear (<=0 immediate)
    Enable : BOOL;
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    RateNow : LREAL;    // signed dPV/dt, EU/s
    Alarm : BOOL;
    ET : LREAL;         // s continuously over-rate
END_VAR
VAR
    pvL : LREAL;
    haveL : BOOL;
    inT : LREAL;
    lim : LREAL;
END_VAR
    lim := MaxRate;
    IF lim < 0.0 THEN lim := -lim; END_IF;
    IF Reset THEN Alarm := FALSE; ET := 0.0; inT := 0.0; haveL := FALSE;
    END_IF;
    IF NOT Enable THEN
        RateNow := 0.0; ET := 0.0; inT := 0.0;
        RETURN;
    END_IF;
    IF haveL AND CycleTime > 0.0 THEN
        RateNow := (PV - pvL) / CycleTime;
    ELSE
        RateNow := 0.0;
    END_IF;
    pvL := PV; haveL := TRUE;
    IF RateNow > lim OR RateNow < -lim THEN
        ET := ET + CycleTime;
        inT := 0.0;
        IF ET >= OnDelay THEN Alarm := TRUE; END_IF;
    ELSE
        ET := 0.0;
        inT := inT + CycleTime;
        IF Alarm AND inT >= OffDelay THEN Alarm := FALSE; END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_AtcDelay: transport delay line --------------------------------
// Pure-delay element for pipeline/conveyor transport lag: Out follows
// In by Delay seconds. Implemented as a 512-sample ring; effective
// delay clamps to the filled depth (Out ramps from passthrough to full
// delay over the first Delay seconds). Delay > 512*CycleTime clamps.
FUNCTION_BLOCK FB_AtcDelay
VAR_INPUT
    In : LREAL;
    Delay : LREAL;      // s
    CycleTime : LREAL;
    Enable : BOOL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Out : LREAL;
    Filled : BOOL;      // ring holds >= requested delay
END_VAR
VAR
    buf : ARRAY[1..512] OF LREAL;
    wr : INT;
    fill : INT;
    n : INT;
    eff : INT;
    rd : INT;
    i : INT;
END_VAR
    IF Reset THEN
        wr := 1; fill := 0; Out := 0.0; Filled := FALSE;
        FOR i := 1 TO 512 DO buf[i] := 0.0; END_FOR;
    END_IF;
    IF NOT Enable THEN
        Out := In; Filled := FALSE;
        wr := 1; fill := 0;
        RETURN;
    END_IF;
    n := DINT_TO_INT(TRUNC(Delay / CycleTime));
    IF n < 1 THEN n := 1; END_IF;
    IF n > 512 THEN n := 512; END_IF;
    IF fill = 0 THEN
        Out := In;
    ELSE
        eff := n;
        IF eff > fill THEN eff := fill; END_IF;
        rd := wr - eff;
        WHILE rd < 1 DO rd := rd + 512; END_WHILE;
        Out := buf[rd];
        Filled := fill >= n;
    END_IF;
    buf[wr] := In;
    wr := wr + 1;
    IF wr > 512 THEN wr := 1; END_IF;
    IF fill < 512 THEN fill := fill + 1; END_IF;
END_FUNCTION_BLOCK

// ---- FB_AtcRamp: trapezoidal position ramp ----------------------------
// Move Pos toward Target with velocity/acceleration limits: cruise at
// MaxVel, brake via the v^2/2a stopping-distance curve (triangular
// profile for short moves), clamp overshoot, Done at rest on target.
// Enable=FALSE or Reset holds position and zeroes velocity.
FUNCTION_BLOCK FB_AtcRamp
VAR_INPUT
    Target : LREAL;
    MaxVel : LREAL;     // units/s (<=0 -> treated as 1)
    MaxAcc : LREAL;     // units/s^2 (<=0 -> treated as 1)
    Enable : BOOL;
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Pos : LREAL;
    Vel : LREAL;
    Moving : BOOL;
    Done : BOOL;
END_VAR
VAR
    dist : LREAL;
    s : LREAL;
    ad : LREAL;
    sd : LREAL;
    nv : LREAL;
    newPos : LREAL;
    mv : LREAL;
    ac : LREAL;
    brk : BOOL;
END_VAR
    mv := MaxVel;
    IF mv <= 0.0 THEN mv := 1.0; END_IF;
    ac := MaxAcc;
    IF ac <= 0.0 THEN ac := 1.0; END_IF;
    IF Reset THEN
        Vel := 0.0; Done := FALSE; Moving := FALSE; brk := FALSE;
    END_IF;
    IF NOT Enable THEN
        Vel := 0.0; Moving := FALSE; brk := FALSE;
        Done := (Pos = Target);
        RETURN;
    END_IF;
    dist := Target - Pos;
    ad := dist;
    IF ad < 0.0 THEN ad := -ad; END_IF;
    IF ad <= 0.0000001 THEN
        // on target: hold, never drift away
        Vel := 0.0; brk := FALSE;
    ELSE
        IF dist > 0.0 THEN s := 1.0; ELSE s := -1.0; END_IF;
        sd := Vel * Vel / (2.0 * ac);
        IF (s > 0.0 AND Vel >= 0.0) OR (s < 0.0 AND Vel <= 0.0) THEN
            // moving toward target (or starting from rest)
            IF NOT brk AND ad <= sd THEN brk := TRUE; END_IF;
            IF brk THEN
                // latched braking: shed speed monotonically so the
                // move cannot oscillate between accel/brake branches
                nv := Vel;
                IF nv < 0.0 THEN nv := -nv; END_IF;
                nv := nv - ac * CycleTime;
                IF nv < 0.0 THEN nv := 0.0; END_IF;
                IF Vel < 0.0 THEN Vel := -nv; ELSE Vel := nv; END_IF;
                IF Vel = 0.0 THEN brk := FALSE; END_IF;
            ELSIF ABS(Vel) < mv THEN
                Vel := Vel + s * ac * CycleTime;
                IF Vel > mv THEN Vel := mv; END_IF;
                IF Vel < -mv THEN Vel := -mv; END_IF;
            END_IF;
        ELSE
            // moving away: bleed speed, re-arm braking check
            nv := Vel;
            IF nv < 0.0 THEN nv := -nv; END_IF;
            nv := nv - ac * CycleTime;
            IF nv < 0.0 THEN nv := 0.0; END_IF;
            IF Vel < 0.0 THEN Vel := -nv; ELSE Vel := nv; END_IF;
            brk := FALSE;
        END_IF;
        // arrival: remaining distance within this scan's travel
        IF ad <= ABS(Vel) * CycleTime THEN
            Pos := Target; Vel := 0.0; brk := FALSE;
        ELSE
            newPos := Pos + Vel * CycleTime;
            IF (newPos - Target) * (Pos - Target) < 0.0 THEN
                Pos := Target; Vel := 0.0; brk := FALSE;
            ELSE
                Pos := newPos;
            END_IF;
        END_IF;
    END_IF;
    Moving := Vel <> 0.0;
    Done := (Pos = Target) AND (Vel = 0.0);
END_FUNCTION_BLOCK

// ---- FB_AtcRatio: ratio/blend feedforward -----------------------------
// FlowA = FlowB * Ratio + Bias — two-component blending (phosphate/
// polymer, ingredient dosing). Ratio dimensionless (e.g. 0.02 = 2 %),
// Bias in flow units/s trims the zero offset. Enable=FALSE outputs 0.
FUNCTION_BLOCK FB_AtcRatio
VAR_INPUT
    FlowB : LREAL;      // master flow (units/s)
    Ratio : LREAL;      // A per unit B
    Bias : LREAL;       // constant trim, same units as FlowA
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    FlowA : LREAL;
    Total : LREAL;
END_VAR
    IF Enable THEN
        FlowA := FlowB * Ratio + Bias;
    ELSE
        FlowA := 0.0;
    END_IF;
    Total := FlowA + FlowB;
END_FUNCTION_BLOCK

// ---- FB_AtcEdge: edge detector ---------------------------------------
// Q = rising edge of I, QN = falling edge — the R_TRIG/FD_TRIG pair
// every counter/latch needs. Level I is tracked even while disabled so
// re-enable does not re-fire on a held input.
FUNCTION_BLOCK FB_AtcEdge
VAR_INPUT
    I : BOOL;
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    Q : BOOL;    // rising edge pulse (one scan)
    QN : BOOL;   // falling edge pulse (one scan)
END_VAR
VAR
    iL : BOOL;
END_VAR
    Q := FALSE; QN := FALSE;
    IF Enable THEN
        IF I AND NOT iL THEN Q := TRUE; END_IF;
        IF NOT I AND iL THEN QN := TRUE; END_IF;
    END_IF;
    iL := I;
END_FUNCTION_BLOCK

// ---- FB_AtcWindow: time-of-day enable window --------------------------
// Q = In AND inside [StartT,EndT) in caller TOD units. Overnight
// windows (StartT > EndT) wrap midnight; equal bounds never open.
// Purely combinational — feed a TOD source (FB_AtcShift domain,
// wall-clock, or a counter scaled to the day).
FUNCTION_BLOCK FB_AtcWindow
VAR_INPUT
    In : BOOL;
    TOD : LREAL;       // current time-of-day, same units as bounds
    StartT : LREAL;
    EndT : LREAL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    Active : BOOL;     // inside the window regardless of In
END_VAR
VAR
    s : LREAL;
    e : LREAL;
END_VAR
    s := StartT; e := EndT;
    IF s = e THEN
        Active := FALSE;
    ELSIF s < e THEN
        Active := TOD >= s AND TOD < e;
    ELSE
        Active := TOD >= s OR TOD < e;
    END_IF;
    Q := In AND Active;
END_FUNCTION_BLOCK

// ---- ST_AtcCamPt: one cam/curve breakpoint ---------------------------
TYPE ST_AtcCamPt : STRUCT
    X : LREAL;   // input abscissa (e.g. position, temperature)
    Y : LREAL;   // output ordinate (e.g. follower position, gain)
END_STRUCT
END_TYPE

// ---- FB_AtcCam: piecewise-linear cam / characteristic curve ---------
// NPTS sorted breakpoints (ascending X); Y linearly interpolated
// between neighbours, clamped to the end values outside the table.
// Sat flags inputs outside [X1,XN]. Write cam.PTS[i].X/.Y members
// before calling (aggregate inputs are member-bound, like SEG tables).
FUNCTION_BLOCK FB_AtcCam
VAR_INPUT
    X : LREAL;
    NPTS : INT;                 // active points, 2..8
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    Y : LREAL;
    Sat : BOOL;                 // X outside table range
END_VAR
VAR_INPUT
    PTS : ARRAY[1..8] OF ST_AtcCamPt;
END_VAR
VAR
    i : INT;
    np : INT;
    x0 : LREAL;
    x1 : LREAL;
    y0 : LREAL;
    y1 : LREAL;
    f : LREAL;
END_VAR
    np := NPTS;
    IF np < 2 THEN np := 2; END_IF;
    IF np > 8 THEN np := 8; END_IF;
    IF NOT Enable THEN
        Y := 0.0; Sat := FALSE;
        RETURN;
    END_IF;
    Sat := X < PTS[1].X OR X > PTS[np].X;
    IF X <= PTS[1].X THEN
        Y := PTS[1].Y;
    ELSIF X >= PTS[np].X THEN
        Y := PTS[np].Y;
    ELSE
        // find segment i with PTS[i].X <= X < PTS[i+1].X
        i := 1;
        WHILE i < np - 1 AND X >= PTS[i + 1].X DO
            i := i + 1;
        END_WHILE;
        x0 := PTS[i].X; x1 := PTS[i + 1].X;
        y0 := PTS[i].Y; y1 := PTS[i + 1].Y;
        IF x1 > x0 THEN
            f := (X - x0) / (x1 - x0);
        ELSE
            f := 0.0;
        END_IF;
        Y := y0 + f * (y1 - y0);
    END_IF;
END_FUNCTION_BLOCK
