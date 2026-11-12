// tc3_ba_common.st — Tc3_BA_Common + Tc3_BA2_Common surface
// (TwinCAT_3_PLC_Lib_Tc3_BA*_Common_EN): pure-ST building-
// automation commons — time-of-day helpers, edge/COV triggers,
// ramp limiters, 2-point switches, PID, sequence controller,
// and a bounded dynamic array.  KL32xx terminal config is not
// modeled (blocks take scaled process values).
//
// Differences: the BA_* enum-validity helpers map to plain
// range checks (enums are INT here); the DynamicArray is a
// bounded fixed-capacity variant (no heap growth).

// ---- F_BA_RemMsTp: ms remaining to a target time ----
FUNCTION F_BA_RemMsTp : UDINT
VAR_INPUT
    tNow : UDINT;               // ms
    tTarget : UDINT;
END_VAR
    IF tTarget >= tNow THEN
        F_BA_RemMsTp := tTarget - tNow;
    ELSE
        F_BA_RemMsTp := 0;
    END_IF;
END_FUNCTION

// ---- F_BA_RemSecsTp: s remaining to a target ----
FUNCTION F_BA_RemSecsTp : UDINT
VAR_INPUT
    tNow : UDINT;
    tTarget : UDINT;
END_VAR
    IF tTarget >= tNow THEN
        F_BA_RemSecsTp := tTarget - tNow;
    ELSE
        F_BA_RemSecsTp := 0;
    END_IF;
END_FUNCTION

// ---- F_BA_IsWeekdayValid: 1..7 range check ----
FUNCTION F_BA_IsWeekdayValid : BOOL
VAR_INPUT
    day : INT;
END_VAR
    F_BA_IsWeekdayValid := day >= 1 AND day <= 7;
END_FUNCTION

// ---- F_BA_IsUnitValid: unit-id range check ----
FUNCTION F_BA_IsUnitValid : BOOL
VAR_INPUT
    unit : INT;
END_VAR
    F_BA_IsUnitValid := unit >= 0 AND unit <= 255;
END_FUNCTION

// ---- FB_BA_RFTrig: rising/falling edge detector ----
FUNCTION_BLOCK FB_BA_RFTrig
VAR_INPUT
    bIn : BOOL;
END_VAR
VAR_OUTPUT
    bRise : BOOL;
    bFall : BOOL;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bRise := bIn AND NOT prev;
    bFall := NOT bIn AND prev;
    prev := bIn;
END_FUNCTION_BLOCK

// ---- FB_BA_ATrigCOV: change-of-value trigger ----
// Fires when the input changes by more than fDelta since the
// last reported value.
FUNCTION_BLOCK FB_BA_ATrigCOV
VAR_INPUT
    bEnable : BOOL;
    fIn : LREAL;
    fDelta : LREAL;
END_VAR
VAR_OUTPUT
    bTrig : BOOL;
    fOut : LREAL;
END_VAR
VAR
    fLast : LREAL;
    init : BOOL;
END_VAR
    bTrig := FALSE;
    IF NOT bEnable THEN init := FALSE; RETURN; END_IF;
    IF NOT init THEN
        fLast := fIn;
        init := TRUE;
        bTrig := TRUE;
        fOut := fIn;
        RETURN;
    END_IF;
    IF fIn - fLast >= fDelta OR fLast - fIn >= fDelta THEN
        fLast := fIn;
        fOut := fIn;
        bTrig := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_BA_RampLmt: ramp with output clamp ----
FUNCTION_BLOCK FB_BA_RampLmt
VAR_INPUT
    bEnable : BOOL;
    fIn : LREAL;
    fSlewUp : LREAL;            // unit/s
    fSlewDown : LREAL;
    fMin : LREAL;
    fMax : LREAL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    fOut : LREAL;
    bAtTarget : BOOL;
END_VAR
    IF NOT bEnable THEN bAtTarget := FALSE; RETURN; END_IF;
    IF fOut < fIn THEN
        fOut := fOut + fSlewUp * CycleTime;
        IF fOut > fIn THEN fOut := fIn; END_IF;
    ELSIF fOut > fIn THEN
        fOut := fOut - fSlewDown * CycleTime;
        IF fOut < fIn THEN fOut := fIn; END_IF;
    END_IF;
    IF fOut > fMax THEN fOut := fMax; END_IF;
    IF fOut < fMin THEN fOut := fMin; END_IF;
    bAtTarget := fOut = fIn;
END_FUNCTION_BLOCK

// ---- FB_BA_Swi2P: two-point switch ----
FUNCTION_BLOCK FB_BA_Swi2P
VAR_INPUT
    bEnable : BOOL;
    fIn : LREAL;
    fOn : LREAL;
    fOff : LREAL;
END_VAR
VAR_OUTPUT
    bOut : BOOL;
END_VAR
VAR
    q : BOOL;
END_VAR
    IF NOT bEnable THEN q := FALSE; bOut := FALSE; RETURN; END_IF;
    IF fIn >= fOn THEN q := TRUE; END_IF;
    IF fIn <= fOff THEN q := FALSE; END_IF;
    bOut := q;
END_FUNCTION_BLOCK

// ---- FB_BA_SwiHys2P: two-point switch with hysteresis ----
FUNCTION_BLOCK FB_BA_SwiHys2P
VAR_INPUT
    bEnable : BOOL;
    fIn : LREAL;
    fSp : LREAL;
    fHys : LREAL;
    bInvert : BOOL;
END_VAR
VAR_OUTPUT
    bOut : BOOL;
END_VAR
VAR
    q : BOOL;
END_VAR
    IF NOT bEnable THEN q := FALSE; bOut := FALSE; RETURN; END_IF;
    IF bInvert THEN
        IF fIn >= fSp + fHys THEN q := TRUE; END_IF;
        IF fIn <= fSp THEN q := FALSE; END_IF;
    ELSE
        IF fIn <= fSp - fHys THEN q := TRUE; END_IF;
        IF fIn >= fSp THEN q := FALSE; END_IF;
    END_IF;
    bOut := q;
END_FUNCTION_BLOCK

// ---- FB_BA_PIDCtrl: PI controller with anti-windup ----
FUNCTION_BLOCK FB_BA_PIDCtrl
VAR_INPUT
    bEnable : BOOL;
    fActual : LREAL;
    fSetpoint : LREAL;
    fKp : LREAL;
    fTi : LREAL;                // s; 0 = P only
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
END_VAR
    IF NOT bEnable THEN
        fOut := fOutMin; iTerm := 0.0; fError := 0.0;
        RETURN;
    END_IF;
    fError := fSetpoint - fActual;
    iTerm := iTerm + fError * CycleTime;
    fOut := fKp * fError;
    IF fTi > 0.0 THEN
        fOut := fKp * (fError + iTerm / fTi);
    END_IF;
    IF fOut > fOutMax THEN
        fOut := fOutMax;
        iTerm := iTerm - fError * CycleTime;
    END_IF;
    IF fOut < fOutMin THEN
        fOut := fOutMin;
        iTerm := iTerm - fError * CycleTime;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_BA_SeqCtrl: sequence controller ----
// Distributes a 0..100 demand across up to 4 sequential
// stages (each stage gets a slice of the demand range).
FUNCTION_BLOCK FB_BA_SeqCtrl
VAR_INPUT
    bEnable : BOOL;
    fDemand : LREAL;            // 0..100 %
    nStages : INT;              // 1..4
END_VAR
VAR_OUTPUT
    fStage1 : LREAL;            // each 0..100
    fStage2 : LREAL;
    fStage3 : LREAL;
    fStage4 : LREAL;
END_VAR
VAR
    seg : LREAL;
    rem : LREAL;
    nR : LREAL;
END_VAR
    IF NOT bEnable OR nStages <= 0 THEN
        fStage1 := 0.0; fStage2 := 0.0;
        fStage3 := 0.0; fStage4 := 0.0;
        RETURN;
    END_IF;
    nR := DINT_TO_LREAL(INT_TO_DINT(nStages));
    seg := 100.0 / nR;
    rem := fDemand;
    fStage1 := rem; IF fStage1 > seg THEN fStage1 := seg; END_IF;
    IF fStage1 < 0.0 THEN fStage1 := 0.0; END_IF;
    rem := rem - seg;
    fStage2 := rem; IF fStage2 > seg THEN fStage2 := seg; END_IF;
    IF fStage2 < 0.0 THEN fStage2 := 0.0; END_IF;
    rem := rem - seg;
    fStage3 := rem; IF fStage3 > seg THEN fStage3 := seg; END_IF;
    IF fStage3 < 0.0 THEN fStage3 := 0.0; END_IF;
    rem := rem - seg;
    fStage4 := rem; IF fStage4 > seg THEN fStage4 := seg; END_IF;
    IF fStage4 < 0.0 THEN fStage4 := 0.0; END_IF;
    // normalize each stage to 0..100 of its own slice
    fStage1 := fStage1 * 100.0 / seg;
    fStage2 := fStage2 * 100.0 / seg;
    fStage3 := fStage3 * 100.0 / seg;
    fStage4 := fStage4 * 100.0 / seg;
END_FUNCTION_BLOCK

// ---- FB_BA_DynamicArray: bounded key->value store ----
// Fixed capacity 16; key 0 is empty.
FUNCTION_BLOCK FB_BA_DynamicArray
VAR_INPUT
    bEnable : BOOL;
    Op : INT;                   // 1=set, 2=get, 3=remove, 4=find
    Key : UDINT;
    Value : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Found : BOOL;
    OutValue : LREAL;
    Count : INT;
    Free : INT;
END_VAR
VAR
    keys : ARRAY[0..15] OF UDINT;
    vals : ARRAY[0..15] OF LREAL;
    cnt : INT;
    i : INT;
    slot : INT;
END_VAR
    Done := FALSE;
    Found := FALSE;
    OutValue := 0.0;
    IF NOT bEnable THEN RETURN; END_IF;
    slot := -1;
    FOR i := 0 TO 15 DO
        IF keys[i] = Key THEN slot := i; END_IF;
    END_FOR;
    IF Op = 1 THEN
        IF slot >= 0 THEN
            vals[slot] := Value;
            Found := TRUE;
            Done := TRUE;
        ELSE
            FOR i := 0 TO 15 DO
                IF slot < 0 AND keys[i] = 0 THEN slot := i; END_IF;
            END_FOR;
            IF slot >= 0 THEN
                keys[slot] := Key;
                vals[slot] := Value;
                cnt := cnt + 1;
                Done := TRUE;
            END_IF;
        END_IF;
    ELSIF Op = 2 OR Op = 4 THEN
        IF slot >= 0 THEN
            Found := TRUE;
            Done := TRUE;
            OutValue := vals[slot];
        END_IF;
    ELSIF Op = 3 THEN
        IF slot >= 0 THEN
            keys[slot] := 0;
            vals[slot] := 0.0;
            cnt := cnt - 1;
            Found := TRUE;
            Done := TRUE;
        END_IF;
    END_IF;
    Count := cnt;
    Free := 16 - cnt;
END_FUNCTION_BLOCK
