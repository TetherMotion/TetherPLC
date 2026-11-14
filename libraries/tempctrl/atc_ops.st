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
