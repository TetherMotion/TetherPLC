// tc2_babasic.st — Tc2_BABasic surface (TF8010_TC3_BABasic_EN):
// pure-ST building-automation blocks — no host/hardware
// dependency.  Covers the documented categories: conversion,
// energy management, facade, lighting, scenes, signal
// processing, and timers.  Remaining catalogue entries are
// variants of the same patterns (documented gap notes in the
// plan file).

// ---- F_Scale: linear rescale ----
FUNCTION F_Scale : LREAL
VAR_INPUT
    x : LREAL;
    xMin : LREAL;
    xMax : LREAL;
    yMin : LREAL;
    yMax : LREAL;
END_VAR
    IF xMax = xMin THEN
        F_Scale := yMin;
    ELSE
        F_Scale := yMin + (x - xMin) * (yMax - yMin) / (xMax - xMin);
    END_IF;
END_FUNCTION

// ---- FB_ThresholdSwitch: two-point switch with hysteresis ----
FUNCTION_BLOCK FB_ThresholdSwitch
VAR_INPUT
    x : LREAL;
    On : LREAL;                 // switch on when x >= On
    Off : LREAL;                // switch off when x <= Off
END_VAR
VAR_OUTPUT
    Q : BOOL;
END_VAR
    IF x >= On THEN Q := TRUE; END_IF;
    IF x <= Off THEN Q := FALSE; END_IF;
END_FUNCTION_BLOCK

// ---- FB_ShortLongClick: click length discrimination ----
// A release before tLong reports Short; holding past tLong
// reports Long.
FUNCTION_BLOCK FB_ShortLongClick
VAR_INPUT
    In : BOOL;
    tLong : LREAL;              // s
    CycleTime : LREAL;          // s
END_VAR
VAR_OUTPUT
    Short : BOOL;
    Long : BOOL;
END_VAR
VAR
    tAcc : LREAL;
    prev : BOOL;
END_VAR
    Short := FALSE;
    Long := FALSE;
    IF CycleTime <= 0.0 THEN RETURN; END_IF;
    IF In THEN
        tAcc := tAcc + CycleTime;
        IF tAcc >= tLong AND NOT prev THEN
            Long := TRUE;      // edge: just passed threshold
        END_IF;
    ELSE
        IF tAcc > 0.0 AND tAcc < tLong THEN
            Short := TRUE;     // released early
        END_IF;
        tAcc := 0.0;
    END_IF;
    prev := tAcc >= tLong;
END_FUNCTION_BLOCK

// ---- FB_SingleDoubleClick: single vs double click ----
FUNCTION_BLOCK FB_SingleDoubleClick
VAR_INPUT
    In : BOOL;
    tDouble : LREAL;            // s window for second click
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Single : BOOL;
    Double : BOOL;
END_VAR
VAR
    clicks : INT;
    tGap : LREAL;
    prev : BOOL;
END_VAR
    Single := FALSE;
    Double := FALSE;
    IF CycleTime <= 0.0 THEN RETURN; END_IF;
    IF In AND NOT prev THEN
        clicks := clicks + 1;
    END_IF;
    prev := In;
    IF clicks > 0 AND NOT In THEN
        tGap := tGap + CycleTime;
        IF tGap >= tDouble THEN
            IF clicks = 1 THEN
                Single := TRUE;
            ELSE
                Double := TRUE;
            END_IF;
            clicks := 0;
            tGap := 0.0;
        END_IF;
    ELSIF In THEN
        tGap := 0.0;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_SignallingContact: latch until acknowledged ----
FUNCTION_BLOCK FB_SignallingContact
VAR_INPUT
    In : BOOL;
    Ack : BOOL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
END_VAR
    IF In THEN Q := TRUE; END_IF;
    IF Ack THEN Q := FALSE; END_IF;
END_FUNCTION_BLOCK

// ---- FB_Ramp: slew-rate limiter ----
FUNCTION_BLOCK FB_Ramp
VAR_INPUT
    Enable : BOOL;
    Setpoint : LREAL;
    SlewUp : LREAL;             // unit/s
    SlewDown : LREAL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Out : LREAL;
    AtTarget : BOOL;
END_VAR
    IF NOT Enable THEN
        AtTarget := FALSE;
        RETURN;
    END_IF;
    IF Out < Setpoint THEN
        Out := Out + SlewUp * CycleTime;
        IF Out > Setpoint THEN Out := Setpoint; END_IF;
    ELSIF Out > Setpoint THEN
        Out := Out - SlewDown * CycleTime;
        IF Out < Setpoint THEN Out := Setpoint; END_IF;
    END_IF;
    AtTarget := Out = Setpoint;
END_FUNCTION_BLOCK

// ---- FB_Light: light channel with off-delay ----
FUNCTION_BLOCK FB_Light
VAR_INPUT
    On : BOOL;
    OffDelay : LREAL;           // s; 0 = no delay
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    TimeLeft : LREAL;
END_VAR
VAR
    tLeft : LREAL;
END_VAR
    IF On THEN
        tLeft := OffDelay;
        Q := TRUE;
    ELSIF tLeft > 0.0 THEN
        tLeft := tLeft - CycleTime;
        Q := tLeft > 0.0;
    ELSE
        Q := FALSE;
    END_IF;
    TimeLeft := tLeft;
END_FUNCTION_BLOCK

// ---- FB_StairwellLight: retriggable stairwell timer ----
FUNCTION_BLOCK FB_StairwellLight
VAR_INPUT
    Trigger : BOOL;
    Duration : LREAL;           // s
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    TimeLeft : LREAL;
END_VAR
VAR
    tLeft : LREAL;
END_VAR
    IF Trigger THEN tLeft := Duration; END_IF;
    IF tLeft > 0.0 THEN
        tLeft := tLeft - CycleTime;
        Q := TRUE;
    ELSE
        tLeft := 0.0;
        Q := FALSE;
    END_IF;
    TimeLeft := tLeft;
END_FUNCTION_BLOCK

// ---- FB_Dimmer1Switch: one-button dimmer ----
// Short click toggles on/off (memory of last level); long press
// ramps the level between Lo and Hi.
FUNCTION_BLOCK FB_Dimmer1Switch
VAR_INPUT
    Click : FB_ShortLongClick;
    LevelHi : LREAL;            // max level (e.g. 100)
    LevelLo : LREAL;
    SlewRate : LREAL;           // level/s
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Level : LREAL;
    Q : BOOL;
END_VAR
VAR
    mem : LREAL;
    dir : INT;
    ramp : FB_Ramp;
END_VAR
    Click();
    IF Click.Short THEN
        IF Q THEN
            mem := Level;
            Q := FALSE;
        ELSE
            Q := TRUE;
        END_IF;
    END_IF;
    IF Click.Long THEN dir := -dir; END_IF;
    ramp.Enable := TRUE;
    IF Q THEN
        IF dir = 0 THEN dir := 1; END_IF;
        IF Click.In THEN
            IF dir > 0 THEN
                ramp.Setpoint := LevelHi;
            ELSE
                ramp.Setpoint := LevelLo;
            END_IF;
        ELSE
            ramp.Setpoint := LevelHi;
        END_IF;
    ELSE
        ramp.Setpoint := 0.0;
    END_IF;
    ramp.SlewUp := SlewRate;
    ramp.SlewDown := SlewRate;
    ramp.CycleTime := CycleTime;
    ramp();
    Level := ramp.Out;
END_FUNCTION_BLOCK

// ---- FB_Dimmer2Switch: two-button up/down dimmer ----
// bUp/bDown short: on at max / off.  Long: ramp while held.
FUNCTION_BLOCK FB_Dimmer2Switch
VAR_INPUT
    bUp : BOOL;
    bDown : BOOL;
    LevelHi : LREAL;
    LevelLo : LREAL;
    SlewRate : LREAL;
    tLong : LREAL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Level : LREAL;
    Q : BOOL;
END_VAR
VAR
    tUp : LREAL;
    tDown : LREAL;
    upP : BOOL;
    downP : BOOL;
END_VAR
    IF bUp THEN tUp := tUp + CycleTime; ELSE tUp := 0.0; END_IF;
    IF bDown THEN tDown := tDown + CycleTime; ELSE tDown := 0.0; END_IF;
    // short-click edges on release
    IF NOT bUp AND upP AND tUp > 0.0 AND tUp < tLong THEN
        Q := TRUE; Level := LevelHi;
    END_IF;
    IF NOT bDown AND downP AND tDown > 0.0 AND tDown < tLong THEN
        Q := FALSE; Level := 0.0;
    END_IF;
    // long-press ramp
    IF tUp >= tLong THEN
        Level := Level + SlewRate * CycleTime;
        IF Level > LevelHi THEN Level := LevelHi; END_IF;
        Q := TRUE;
    END_IF;
    IF tDown >= tLong THEN
        Level := Level - SlewRate * CycleTime;
        IF Level < 0.0 THEN Level := 0.0; END_IF;
        IF Level <= LevelLo THEN Q := FALSE; END_IF;
    END_IF;
    upP := bUp;
    downP := bDown;
END_FUNCTION_BLOCK

// ---- FB_VenetianBlind: blind with slat-angle control ----
// up/down drive position (0 = open/top, PosMax = closed);
// a short tap nudges the slat angle instead.
FUNCTION_BLOCK FB_VenetianBlind
VAR_INPUT
    bUp : BOOL;
    bDown : BOOL;
    PosMax : LREAL;             // s travel top->bottom
    SlatMax : LREAL;            // s slat full swing
    tSlat : LREAL;              // tap threshold
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Position : LREAL;           // 0..PosMax
    Slat : LREAL;               // -SlatMax..+SlatMax
END_VAR
VAR
    tDir : LREAL;
    upP : BOOL;
    downP : BOOL;
END_VAR
    IF bUp THEN
        tDir := tDir + CycleTime;
        IF tDir >= tSlat THEN
            Position := Position - CycleTime;
            IF Position < 0.0 THEN Position := 0.0; END_IF;
        END_IF;
    END_IF;
    IF bDown THEN
        tDir := tDir + CycleTime;
        IF tDir >= tSlat THEN
            Position := Position + CycleTime;
            IF Position > PosMax THEN Position := PosMax; END_IF;
        END_IF;
    END_IF;
    // released early: slat nudge in that direction
    IF NOT bUp AND upP AND tDir > 0.0 AND tDir < tSlat THEN
        Slat := Slat - CycleTime * 4.0;
        IF Slat < -SlatMax THEN Slat := -SlatMax; END_IF;
    END_IF;
    IF NOT bDown AND downP AND tDir > 0.0 AND tDir < tSlat THEN
        Slat := Slat + CycleTime * 4.0;
        IF Slat > SlatMax THEN Slat := SlatMax; END_IF;
    END_IF;
    IF NOT bUp AND NOT bDown THEN tDir := 0.0; END_IF;
    upP := bUp;
    downP := bDown;
END_FUNCTION_BLOCK

// ---- FB_MaximumDemandController: shed loads over limit ----
// Counts active inputs; when the count exceeds Limit, outputs
// are shed starting from the highest index (shed mask).
FUNCTION_BLOCK FB_MaximumDemandController
VAR_INPUT
    In1 : BOOL;
    In2 : BOOL;
    In3 : BOOL;
    In4 : BOOL;
    Limit : INT;
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    Q1 : BOOL;
    Q2 : BOOL;
    Q3 : BOOL;
    Q4 : BOOL;
    Shedding : BOOL;
    Active : INT;
END_VAR
VAR
    n : INT;
END_VAR
    IF NOT Enable THEN
        Q1 := FALSE; Q2 := FALSE; Q3 := FALSE; Q4 := FALSE;
        Shedding := FALSE;
        Active := 0;
        RETURN;
    END_IF;
    n := 0;
    IF In1 THEN n := n + 1; END_IF;
    IF In2 THEN n := n + 1; END_IF;
    IF In3 THEN n := n + 1; END_IF;
    IF In4 THEN n := n + 1; END_IF;
    Active := n;
    Q1 := In1; Q2 := In2; Q3 := In3; Q4 := In4;
    Shedding := FALSE;
    IF n > Limit THEN
        Shedding := TRUE;
        // shed highest first until under limit
        IF Q4 AND n > Limit THEN Q4 := FALSE; n := n - 1; END_IF;
        IF Q3 AND n > Limit THEN Q3 := FALSE; n := n - 1; END_IF;
        IF Q2 AND n > Limit THEN Q2 := FALSE; n := n - 1; END_IF;
        IF Q1 AND n > Limit THEN Q1 := FALSE; n := n - 1; END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_DailyScheduler: on/off windows within a day ----
// MinuteOfDay 0..1439; on when inside [OnMin, OffMin).
FUNCTION_BLOCK FB_DailyScheduler
VAR_INPUT
    MinuteOfDay : INT;
    OnMin : INT;
    OffMin : INT;
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
END_VAR
    IF NOT Enable THEN Q := FALSE; RETURN; END_IF;
    IF OnMin < OffMin THEN
        Q := MinuteOfDay >= OnMin AND MinuteOfDay < OffMin;
    ELSE
        // window wraps midnight
        Q := MinuteOfDay >= OnMin OR MinuteOfDay < OffMin;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_ScenesLighting: recall one of N light levels ----
FUNCTION_BLOCK FB_ScenesLighting
VAR_INPUT
    Scene : INT;                // 1..4
    L1 : LREAL;
    L2 : LREAL;
    L3 : LREAL;
    L4 : LREAL;
    Enable : BOOL;
END_VAR
VAR_OUTPUT
    Level : LREAL;
    Q : BOOL;
END_VAR
    IF NOT Enable THEN
        Level := 0.0; Q := FALSE;
        RETURN;
    END_IF;
    IF Scene = 1 THEN Level := L1;
    ELSIF Scene = 2 THEN Level := L2;
    ELSIF Scene = 3 THEN Level := L3;
    ELSIF Scene = 4 THEN Level := L4;
    ELSE Level := 0.0;
    END_IF;
    Q := Level > 0.0;
END_FUNCTION_BLOCK
