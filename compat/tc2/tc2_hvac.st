// tc2_hvac.st — Tc2_HVAC surface (TF8000_TC3_HVAC_EN): pure-ST
// HVAC control blocks — no host/hardware dependency.  Covers
// the documented categories: actuators, motors/pumps, mux,
// redundancy, analog scaling, controllers (2-point, step, PID),
// sequence controllers, heating curve / outside-temp damping,
// alarm/anti-blocking/blink helpers, and BAR room-automation
// primitives.  Terminal bindings (KL32xx) are not modeled —
// the blocks operate on scaled process values.

// ---- FB_HVACAnalogInput: raw counts -> engineering value ----
FUNCTION_BLOCK FB_HVACAnalogInput
VAR_INPUT
    Raw : LREAL;                // raw counts
    RawMin : LREAL;
    RawMax : LREAL;
    EngMin : LREAL;
    EngMax : LREAL;
END_VAR
VAR_OUTPUT
    Value : LREAL;
END_VAR
    IF RawMax = RawMin THEN
        Value := EngMin;
    ELSE
        Value := EngMin + (Raw - RawMin) * (EngMax - EngMin) /
                 (RawMax - RawMin);
        IF Value < EngMin THEN Value := EngMin; END_IF;
        IF Value > EngMax THEN Value := EngMax; END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_HVACAnalogOutput: engineering value -> raw ----
FUNCTION_BLOCK FB_HVACAnalogOutput
VAR_INPUT
    Value : LREAL;
    EngMin : LREAL;
    EngMax : LREAL;
    RawMin : LREAL;
    RawMax : LREAL;
END_VAR
VAR_OUTPUT
    Raw : LREAL;
END_VAR
    IF EngMax = EngMin THEN
        Raw := RawMin;
    ELSE
        Raw := RawMin + (Value - EngMin) * (RawMax - RawMin) /
               (EngMax - EngMin);
        IF Raw < RawMin THEN Raw := RawMin; END_IF;
        IF Raw > RawMax THEN Raw := RawMax; END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_HVACScale: generic piecewise-linear scale ----
FUNCTION_BLOCK FB_HVACScale
VAR_INPUT
    x : LREAL;
    xMin : LREAL;
    xMax : LREAL;
    yMin : LREAL;
    yMax : LREAL;
END_VAR
VAR_OUTPUT
    y : LREAL;
END_VAR
    IF xMax = xMin THEN
        y := yMin;
    ELSE
        y := yMin + (x - xMin) * (yMax - yMin) / (xMax - xMin);
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_HVAC2PointActuator: two-point damper/valve ----
// bCtrlVoltage TRUE -> run open; plus manual override and end-
// switch feedback monitoring.
FUNCTION_BLOCK FB_HVAC2PointActuator
VAR_INPUT
    bEnable : BOOL;
    bIn : BOOL;                 // demand: TRUE = open
    bManSwitch : BOOL;          // manual open override
    bLimitSwitchOpen : BOOL;
    bLimitSwitchClose : BOOL;
    tRun : LREAL;               // travel time s
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    bOut : BOOL;                // drive command
    bFault : BOOL;              // end-switch disagreement
    Position : LREAL;           // 0 closed .. 100 open
END_VAR
VAR
    pos : LREAL;
END_VAR
    IF NOT bEnable THEN
        bOut := FALSE;
        bFault := FALSE;
        RETURN;
    END_IF;
    bOut := bIn OR bManSwitch;
    IF tRun > 0.0 THEN
        IF bOut THEN
            pos := pos + 100.0 * CycleTime / tRun;
            IF pos > 100.0 THEN pos := 100.0; END_IF;
        ELSE
            pos := pos - 100.0 * CycleTime / tRun;
            IF pos < 0.0 THEN pos := 0.0; END_IF;
        END_IF;
    END_IF;
    Position := pos;
    // fault: fully driven but feedback contradicts
    bFault := (pos >= 100.0 AND NOT bLimitSwitchOpen) OR
             (pos <= 0.0 AND NOT bLimitSwitchClose);
END_FUNCTION_BLOCK

// ---- FB_HVAC3PointActuator: three-point (open/close) ----
FUNCTION_BLOCK FB_HVAC3PointActuator
VAR_INPUT
    bEnable : BOOL;
    bOpen : BOOL;
    bClose : BOOL;
    tRun : LREAL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    bOutOpen : BOOL;
    bOutClose : BOOL;
    Position : LREAL;
END_VAR
VAR
    pos : LREAL;
END_VAR
    IF NOT bEnable THEN
        bOutOpen := FALSE;
        bOutClose := FALSE;
        RETURN;
    END_IF;
    bOutOpen := bOpen AND NOT bClose;
    bOutClose := bClose AND NOT bOpen;
    IF tRun > 0.0 THEN
        IF bOutOpen THEN
            pos := pos + 100.0 * CycleTime / tRun;
            IF pos > 100.0 THEN pos := 100.0; END_IF;
        END_IF;
        IF bOutClose THEN
            pos := pos - 100.0 * CycleTime / tRun;
            IF pos < 0.0 THEN pos := 0.0; END_IF;
        END_IF;
    END_IF;
    Position := pos;
END_FUNCTION_BLOCK

// ---- FB_HVACCirculationPump: pump with standby lead swap ----
FUNCTION_BLOCK FB_HVACCirculationPump
VAR_INPUT
    bEnable : BOOL;
    bDemand : BOOL;
    bFault1 : BOOL;
    bFault2 : BOOL;
    bSwap : BOOL;               // lead-rotation request
END_VAR
VAR_OUTPUT
    bPump1 : BOOL;
    bPump2 : BOOL;
    Lead : INT;                 // 1 or 2
    bAlarm : BOOL;
END_VAR
VAR
    leadN : INT;
END_VAR
    IF leadN = 0 THEN leadN := 1; END_IF;
    IF bSwap THEN
        IF leadN = 1 THEN leadN := 2; ELSE leadN := 1; END_IF;
    END_IF;
    Lead := leadN;
    IF NOT bEnable OR NOT bDemand THEN
        bPump1 := FALSE;
        bPump2 := FALSE;
        bAlarm := bFault1 AND bFault2;
        RETURN;
    END_IF;
    bPump1 := leadN = 1 AND NOT bFault1;
    bPump2 := (leadN = 2 OR bFault1) AND NOT bFault2;
    bAlarm := bFault1 AND bFault2;
END_FUNCTION_BLOCK

// ---- FB_HVACMotor1Speed / Motor2Speed / Motor3Speed ----
FUNCTION_BLOCK FB_HVACMotor1Speed
VAR_INPUT
    bEnable : BOOL;
    Speed : INT;                // 0 off, 1 run
END_VAR
VAR_OUTPUT
    bOut1 : BOOL;
END_VAR
    bOut1 := bEnable AND Speed = 1;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_HVACMotor2Speed
VAR_INPUT
    bEnable : BOOL;
    Speed : INT;                // 0..2
END_VAR
VAR_OUTPUT
    bOut1 : BOOL;
    bOut2 : BOOL;
END_VAR
    bOut1 := bEnable AND Speed = 1;
    bOut2 := bEnable AND Speed = 2;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_HVACMotor3Speed
VAR_INPUT
    bEnable : BOOL;
    Speed : INT;                // 0..3
END_VAR
VAR_OUTPUT
    bOut1 : BOOL;
    bOut2 : BOOL;
    bOut3 : BOOL;
END_VAR
    bOut1 := bEnable AND Speed = 1;
    bOut2 := bEnable AND Speed = 2;
    bOut3 := bEnable AND Speed = 3;
END_FUNCTION_BLOCK

// ---- FB_HVACMux8: 8 -> 1 analog mux by selector ----
FUNCTION_BLOCK FB_HVACMux8
VAR_INPUT
    Sel : INT;                  // 1..8
    In1 : LREAL; In2 : LREAL; In3 : LREAL; In4 : LREAL;
    In5 : LREAL; In6 : LREAL; In7 : LREAL; In8 : LREAL;
END_VAR
VAR_OUTPUT
    Out : LREAL;
END_VAR
    IF Sel = 1 THEN Out := In1;
    ELSIF Sel = 2 THEN Out := In2;
    ELSIF Sel = 3 THEN Out := In3;
    ELSIF Sel = 4 THEN Out := In4;
    ELSIF Sel = 5 THEN Out := In5;
    ELSIF Sel = 6 THEN Out := In6;
    ELSIF Sel = 7 THEN Out := In7;
    ELSIF Sel = 8 THEN Out := In8;
    ELSE Out := 0.0;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_HVACMux8_BOOL: 8 -> 1 bool mux ----
FUNCTION_BLOCK FB_HVACMux8_BOOL
VAR_INPUT
    Sel : INT;
    In1 : BOOL; In2 : BOOL; In3 : BOOL; In4 : BOOL;
    In5 : BOOL; In6 : BOOL; In7 : BOOL; In8 : BOOL;
END_VAR
VAR_OUTPUT
    Out : BOOL;
END_VAR
    IF Sel = 1 THEN Out := In1;
    ELSIF Sel = 2 THEN Out := In2;
    ELSIF Sel = 3 THEN Out := In3;
    ELSIF Sel = 4 THEN Out := In4;
    ELSIF Sel = 5 THEN Out := In5;
    ELSIF Sel = 6 THEN Out := In6;
    ELSIF Sel = 7 THEN Out := In7;
    ELSIF Sel = 8 THEN Out := In8;
    ELSE Out := FALSE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_HVACRedundancyCtrl: n+1 pump/fan redundancy ----
// N units, need Required running; rotates the lead and covers
// faults in order.
FUNCTION_BLOCK FB_HVACRedundancyCtrl
VAR_INPUT
    bEnable : BOOL;
    Required : INT;             // units demanded 0..3
    bFault1 : BOOL;
    bFault2 : BOOL;
    bFault3 : BOOL;
    bRotate : BOOL;
END_VAR
VAR_OUTPUT
    bOut1 : BOOL;
    bOut2 : BOOL;
    bOut3 : BOOL;
    Lead : INT;
    bAlarm : BOOL;
END_VAR
VAR
    leadN : INT;
    want : INT;
    ok1 : BOOL;
    ok2 : BOOL;
    ok3 : BOOL;
END_VAR
    IF leadN = 0 THEN leadN := 1; END_IF;
    IF bRotate THEN
        leadN := leadN + 1;
        IF leadN > 3 THEN leadN := 1; END_IF;
    END_IF;
    Lead := leadN;
    ok1 := NOT bFault1;
    ok2 := NOT bFault2;
    ok3 := NOT bFault3;
    want := Required;
    bOut1 := FALSE; bOut2 := FALSE; bOut3 := FALSE;
    IF bEnable THEN
        // start at leadN, wrap, pick healthy units
        IF want > 0 THEN
            IF leadN = 1 THEN
                IF ok1 THEN bOut1 := TRUE; want := want - 1; END_IF;
                IF want > 0 AND ok2 THEN bOut2 := TRUE; want := want - 1; END_IF;
                IF want > 0 AND ok3 THEN bOut3 := TRUE; want := want - 1; END_IF;
            ELSIF leadN = 2 THEN
                IF ok2 THEN bOut2 := TRUE; want := want - 1; END_IF;
                IF want > 0 AND ok3 THEN bOut3 := TRUE; want := want - 1; END_IF;
                IF want > 0 AND ok1 THEN bOut1 := TRUE; want := want - 1; END_IF;
            ELSE
                IF ok3 THEN bOut3 := TRUE; want := want - 1; END_IF;
                IF want > 0 AND ok1 THEN bOut1 := TRUE; want := want - 1; END_IF;
                IF want > 0 AND ok2 THEN bOut2 := TRUE; want := want - 1; END_IF;
            END_IF;
        END_IF;
    END_IF;
    bAlarm := want > 0;         // couldn't satisfy demand
END_FUNCTION_BLOCK

// ---- FB_HVAC2PointCtrl: on/off controller with hysteresis ----
FUNCTION_BLOCK FB_HVAC2PointCtrl
VAR_INPUT
    bEnable : BOOL;
    fActual : LREAL;
    fSetpoint : LREAL;
    fHysteresis : LREAL;
    bInvert : BOOL;             // TRUE = cooling (on above SP)
END_VAR
VAR_OUTPUT
    bOut : BOOL;
END_VAR
VAR
    q : BOOL;
END_VAR
    IF NOT bEnable THEN q := FALSE; bOut := FALSE; RETURN; END_IF;
    IF bInvert THEN
        IF fActual >= fSetpoint + fHysteresis THEN q := TRUE; END_IF;
        IF fActual <= fSetpoint THEN q := FALSE; END_IF;
    ELSE
        IF fActual <= fSetpoint - fHysteresis THEN q := TRUE; END_IF;
        IF fActual >= fSetpoint THEN q := FALSE; END_IF;
    END_IF;
    bOut := q;
END_FUNCTION_BLOCK

// ---- FB_HVACI_CtrlStep: staged step controller ----
// Splits the control error into up to 4 equal stages.
FUNCTION_BLOCK FB_HVACI_CtrlStep
VAR_INPUT
    bEnable : BOOL;
    fActual : LREAL;
    fSetpoint : LREAL;
    fStepRange : LREAL;         // error span for all 4 stages
    nStages : INT;              // 1..4
END_VAR
VAR_OUTPUT
    bStage1 : BOOL;
    bStage2 : BOOL;
    bStage3 : BOOL;
    bStage4 : BOOL;
END_VAR
VAR
    e : LREAL;
    step : LREAL;
END_VAR
    IF NOT bEnable OR nStages <= 0 OR fStepRange <= 0.0 THEN
        bStage1 := FALSE; bStage2 := FALSE;
        bStage3 := FALSE; bStage4 := FALSE;
        RETURN;
    END_IF;
    e := fSetpoint - fActual;
    step := fStepRange / DINT_TO_LREAL(INT_TO_DINT(nStages));
    bStage1 := e >= step;
    bStage2 := nStages >= 2 AND e >= 2.0 * step;
    bStage3 := nStages >= 3 AND e >= 3.0 * step;
    bStage4 := nStages >= 4 AND e >= 4.0 * step;
END_FUNCTION_BLOCK

// ---- FB_HVACPIDCtrl: PI(D) controller ----
// Incremental PID with anti-windup clamp on the output.
FUNCTION_BLOCK FB_HVACPIDCtrl
VAR_INPUT
    bEnable : BOOL;
    fActual : LREAL;
    fSetpoint : LREAL;
    fKp : LREAL;
    fTi : LREAL;                // s; 0 = P only
    fTd : LREAL;                // s
    fOutMin : LREAL;
    fOutMax : LREAL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    fOut : LREAL;
    fError : LREAL;
END_VAR
VAR
    iTerm : LREAL;
    ePrev : LREAL;
END_VAR
    IF NOT bEnable THEN
        fOut := fOutMin;
        iTerm := 0.0;
        ePrev := 0.0;
        fError := 0.0;
        RETURN;
    END_IF;
    fError := fSetpoint - fActual;
    iTerm := iTerm + fError * CycleTime;
    fOut := fKp * (fError + ePrev);
    IF fTi > 0.0 THEN
        fOut := fKp * (fError + iTerm / fTi);
    END_IF;
    IF fTd > 0.0 AND CycleTime > 0.0 THEN
        fOut := fOut + fKp * fTd * (fError - ePrev) / CycleTime;
    END_IF;
    IF fOut > fOutMax THEN
        fOut := fOutMax;
        iTerm := iTerm - fError * CycleTime;   // anti-windup
    END_IF;
    IF fOut < fOutMin THEN
        fOut := fOutMin;
        iTerm := iTerm - fError * CycleTime;
    END_IF;
    ePrev := fError;
END_FUNCTION_BLOCK

// ---- FB_HVACHeatingCurve: outdoor-reset flow setpoint ----
// Flow SP = fSpMin + (fSpMax-fSpMin) * (tRoom - tOut) /
// (tRoom - tOutMin) when outside is cold, clamped.
FUNCTION_BLOCK FB_HVACHeatingCurve
VAR_INPUT
    bEnable : BOOL;
    fOutsideTemp : LREAL;
    fOutsideMin : LREAL;        // design outside temp
    fRoomTemp : LREAL;
    fSpMin : LREAL;             // flow SP at warm extreme
    fSpMax : LREAL;             // flow SP at design outside
END_VAR
VAR_OUTPUT
    fFlowSp : LREAL;
END_VAR
    IF NOT bEnable OR fRoomTemp = fOutsideMin THEN
        fFlowSp := fSpMin;
        RETURN;
    END_IF;
    fFlowSp := fSpMin + (fSpMax - fSpMin) *
               (fRoomTemp - fOutsideTemp) / (fRoomTemp - fOutsideMin);
    IF fFlowSp > fSpMax THEN fFlowSp := fSpMax; END_IF;
    IF fFlowSp < fSpMin THEN fFlowSp := fSpMin; END_IF;
END_FUNCTION_BLOCK

// ---- FB_HVACOutsideTempDamped: damped outdoor temperature ----
// First-order lag on the measured outside temperature.
FUNCTION_BLOCK FB_HVACOutsideTempDamped
VAR_INPUT
    bEnable : BOOL;
    fOutsideTemp : LREAL;
    fTau : LREAL;               // damping time constant s
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    fDamped : LREAL;
END_VAR
VAR
    y : LREAL;
    init : BOOL;
END_VAR
    IF NOT bEnable THEN
        init := FALSE;
        RETURN;
    END_IF;
    IF NOT init OR fTau <= 0.0 THEN
        y := fOutsideTemp;
        init := TRUE;
    ELSE
        y := y + (fOutsideTemp - y) * CycleTime / fTau;
    END_IF;
    fDamped := y;
END_FUNCTION_BLOCK

// ---- FB_HVACSetpointRamp: ramp a setpoint to target ----
FUNCTION_BLOCK FB_HVACSetpointRamp
VAR_INPUT
    bEnable : BOOL;
    fTarget : LREAL;
    fSlew : LREAL;              // unit/s
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    fOut : LREAL;
    bDone : BOOL;
END_VAR
    IF NOT bEnable THEN bDone := FALSE; RETURN; END_IF;
    IF fOut < fTarget THEN
        fOut := fOut + fSlew * CycleTime;
        IF fOut > fTarget THEN fOut := fTarget; END_IF;
    ELSIF fOut > fTarget THEN
        fOut := fOut - fSlew * CycleTime;
        IF fOut < fTarget THEN fOut := fTarget; END_IF;
    END_IF;
    bDone := fOut = fTarget;
END_FUNCTION_BLOCK

// ---- FB_HVACAlarm: limit alarm with hysteresis + delay ----
FUNCTION_BLOCK FB_HVACAlarm
VAR_INPUT
    bEnable : BOOL;
    fValue : LREAL;
    fLimit : LREAL;
    fHysteresis : LREAL;
    tDelay : LREAL;             // s before alarm latches
    bAck : BOOL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    bAlarm : BOOL;
    bActive : BOOL;             // condition currently true
END_VAR
VAR
    tAcc : LREAL;
    latch : BOOL;
END_VAR
    IF NOT bEnable THEN
        bAlarm := FALSE; bActive := FALSE; latch := FALSE;
        tAcc := 0.0;
        RETURN;
    END_IF;
    bActive := fValue >= fLimit;
    IF bActive THEN
        tAcc := tAcc + CycleTime;
        IF tAcc >= tDelay THEN latch := TRUE; END_IF;
    END_IF;
    IF fValue <= fLimit - fHysteresis THEN tAcc := 0.0; END_IF;
    IF bAck THEN latch := FALSE; END_IF;
    bAlarm := latch;
END_FUNCTION_BLOCK

// ---- FB_HVACAntiBlockingPump: periodic exercise run ----
// If the pump has been idle longer than tIdle, run it for
// tRunTime to prevent seizing.
FUNCTION_BLOCK FB_HVACAntiBlockingPump
VAR_INPUT
    bEnable : BOOL;
    bDemand : BOOL;
    tIdle : LREAL;              // s idle before exercise
    tRunTime : LREAL;           // s exercise duration
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    bPump : BOOL;
    bExercise : BOOL;           // exercising (not demand)
END_VAR
VAR
    tIdleAcc : LREAL;
    tRunAcc : LREAL;
END_VAR
    IF NOT bEnable THEN
        bPump := FALSE; bExercise := FALSE;
        tIdleAcc := 0.0; tRunAcc := 0.0;
        RETURN;
    END_IF;
    IF bDemand THEN
        tIdleAcc := 0.0;
        tRunAcc := 0.0;
        bPump := TRUE;
        bExercise := FALSE;
    ELSIF tRunAcc > 0.0 THEN
        tRunAcc := tRunAcc - CycleTime;
        bPump := tRunAcc > 0.0;
        bExercise := bPump;
        IF NOT bExercise THEN tIdleAcc := 0.0; END_IF;
    ELSE
        tIdleAcc := tIdleAcc + CycleTime;
        IF tIdleAcc >= tIdle THEN
            tRunAcc := tRunTime;
            tIdleAcc := 0.0;
        END_IF;
        bPump := FALSE;
        bExercise := FALSE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_HVACBlink: square-wave blinker ----
FUNCTION_BLOCK FB_HVACBlink
VAR_INPUT
    bEnable : BOOL;
    tOn : LREAL;                // s
    tOff : LREAL;               // s
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    bOut : BOOL;
END_VAR
VAR
    tAcc : LREAL;
    q : BOOL;
END_VAR
    IF NOT bEnable THEN bOut := FALSE; q := FALSE; tAcc := 0.0; RETURN; END_IF;
    tAcc := tAcc + CycleTime;
    IF q THEN
        IF tAcc >= tOn THEN q := FALSE; tAcc := 0.0; END_IF;
    ELSE
        IF tAcc >= tOff THEN q := TRUE; tAcc := 0.0; END_IF;
    END_IF;
    bOut := q;
END_FUNCTION_BLOCK

// ---- FB_HVACPIDHeating / Cooling convenience wrappers ----
// Sequence controllers in the catalogue are PID + mode glue;
// the core PID is FB_HVACPIDCtrl above.

// ---- FB_BARSetpointRoom: room setpoint with shift ----
FUNCTION_BLOCK FB_BARSetpointRoom
VAR_INPUT
    bEnable : BOOL;
    fBaseSp : LREAL;            // base comfort setpoint
    fShift : LREAL;             // user shift +-K
    fShiftMax : LREAL;
END_VAR
VAR_OUTPUT
    fSetpoint : LREAL;
END_VAR
    IF NOT bEnable THEN fSetpoint := fBaseSp; RETURN; END_IF;
    fSetpoint := fBaseSp + fShift;
    IF fSetpoint > fBaseSp + fShiftMax THEN
        fSetpoint := fBaseSp + fShiftMax;
    END_IF;
    IF fSetpoint < fBaseSp - fShiftMax THEN
        fSetpoint := fBaseSp - fShiftMax;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_BARDelayedHysteresis: on/off delay on a level ----
FUNCTION_BLOCK FB_BARDelayedHysteresis
VAR_INPUT
    bEnable : BOOL;
    fValue : LREAL;
    fOn : LREAL;
    fOff : LREAL;
    tOnDelay : LREAL;
    tOffDelay : LREAL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    bOut : BOOL;
END_VAR
VAR
    tAcc : LREAL;
    q : BOOL;
END_VAR
    IF NOT bEnable THEN bOut := FALSE; q := FALSE; tAcc := 0.0; RETURN; END_IF;
    IF fValue >= fOn AND NOT q THEN
        tAcc := tAcc + CycleTime;
        IF tAcc >= tOnDelay THEN q := TRUE; tAcc := 0.0; END_IF;
    ELSIF fValue <= fOff AND q THEN
        tAcc := tAcc + CycleTime;
        IF tAcc >= tOffDelay THEN q := FALSE; tAcc := 0.0; END_IF;
    ELSE
        tAcc := 0.0;
    END_IF;
    bOut := q;
END_FUNCTION_BLOCK
