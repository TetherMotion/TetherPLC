// tc2_weighing.st — Tc2_Weighing surface — load-cell weighing
// blocks, pure-ST, no service or hardware.
//
// FB_Weigh wraps the weighing loop: PT1 filter on the raw
// load-cell signal, tare subtraction (gross -> net), stability
// detection (weight within a band for N cycles), and a dosing
// setpoint output (fast/slow dribble feeds for a target weight).
// The caller supplies the raw cell signal and cycle time.

// ---- FB_Weigh: weighing + tare + stability + dosing ----
FUNCTION_BLOCK FB_Weigh
VAR_INPUT
    Execute : BOOL;
    Raw : LREAL;            // raw load-cell signal
    FilterTn : LREAL;       // filter time constant (s); 0 = none
    CycleTime : LREAL;
    DoTare : BOOL;            // capture current gross as tare
    TareValue : LREAL;      // explicit tare (when DoTare is FALSE
                            // and TareValueSet)
    SetTare : BOOL;         // apply TareValue
    StableBand : LREAL;     // +- stability band
    StableCycles : UDINT;   // cycles within band to call stable
    DoseSetpoint : LREAL;   // target weight for dosing
    CoarseLimit : LREAL;    // switch coarse->fine at this fraction
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Gross : LREAL;          // filtered gross weight
    Net : LREAL;            // gross - tare
    TareWeight : LREAL;     // active tare
    Stable : BOOL;          // weight settled
    Coarse : BOOL;          // dosing: coarse feed on
    Fine : BOOL;            // dosing: fine feed on
    DoseDone : BOOL;        // dosing complete (Net >= Setpoint)
END_VAR
VAR
    filt : LREAL;
    tare : LREAL;
    prevW : LREAL;
    stableN : UDINT;
    dW : LREAL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF NOT Execute THEN RETURN; END_IF;

    // PT1 filter on the raw cell signal
    IF FilterTn > 0.0 THEN
        filt := filt + (Raw - filt) * (CycleTime / (FilterTn + CycleTime));
    ELSE
        filt := Raw;
    END_IF;
    Gross := filt;

    // tare handling
    IF DoTare THEN
        tare := filt;
    ELSIF SetTare THEN
        tare := TareValue;
    END_IF;
    TareWeight := tare;
    Net := Gross - tare;

    // stability: weight within StableBand for StableCycles
    dW := Gross - prevW;
    IF dW < 0.0 THEN dW := -dW; END_IF;
    IF dW <= StableBand THEN
        IF stableN < StableCycles THEN
            stableN := stableN + 1;
        END_IF;
    ELSE
        stableN := 0;
    END_IF;
    Stable := (stableN >= StableCycles) AND (StableCycles > 0);
    prevW := Gross;

    // dosing: coarse -> fine -> done
    DoseDone := (Net >= DoseSetpoint) AND (DoseSetpoint > 0.0);
    IF DoseDone THEN
        Coarse := FALSE;
        Fine := FALSE;
    ELSIF Net >= DoseSetpoint * CoarseLimit THEN
        Coarse := FALSE;
        Fine := TRUE;
    ELSE
        Coarse := TRUE;
        Fine := FALSE;
    END_IF;

    Done := TRUE;
END_FUNCTION_BLOCK
