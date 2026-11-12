// tc2_cm.st — Tc2_CM (curve/math) surface — curve and math
// primitives, pure-ST, no service or hardware.
//
// These are the curve/trajectory helpers: a function generator
// (sawtooth/triangle/square), a piecewise-linear interpolator over
// a breakpoint table, and a cam-style master->slave curve
// (linear segment lookup between master positions).  All take
// phase/cycle inputs from the caller.

// ---- FB_FuncGen: periodic function generator ----
// Shape: 0=sawtooth, 1=triangle, 2=square
FUNCTION_BLOCK FB_FuncGen
VAR_INPUT
    Execute : BOOL;
    Shape : UDINT;          // 0=saw 1=tri 2=square
    Period : LREAL;         // period (s), >0
    Amplitude : LREAL;
    Offset : LREAL;
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
    Phase : LREAL;          // 0..1
END_VAR
VAR
    ph : LREAL;
    y : LREAL;
END_VAR
    Done := FALSE;
    IF Execute AND Period > 0.0 THEN
        ph := ph + CycleTime / Period;
        IF ph >= 1.0 THEN ph := ph - 1.0; END_IF;
        IF Shape = 0 THEN
            y := ph;                                   // sawtooth
        ELSIF Shape = 1 THEN
            IF ph < 0.5 THEN
                y := 2.0 * ph;                          // triangle up
            ELSE
                y := 2.0 * (1.0 - ph);                  // triangle down
            END_IF;
        ELSE
            IF ph < 0.5 THEN
                y := 1.0;                               // square high
            ELSE
                y := 0.0;                               // square low
            END_IF;
        END_IF;
        Output := Offset + Amplitude * y;
        Phase := ph;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_LinInterp: piecewise-linear interpolation over a table ----
// PointsX[i] -> PointsY[i], clamped at the ends.  Caller fills the
// 8-point table.
FUNCTION_BLOCK FB_LinInterp
VAR_INPUT
    Execute : BOOL;
    X : LREAL;
    X0 : LREAL; Y0 : LREAL;
    X1 : LREAL; Y1 : LREAL;
    X2 : LREAL; Y2 : LREAL;
    X3 : LREAL; Y3 : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Y : LREAL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF NOT Execute THEN RETURN; END_IF;
    // 4-point piecewise-linear — X0..X3 ascending
    IF X <= X1 THEN
        IF X1 = X0 THEN Error := TRUE; ErrorID := 1; END_IF;
        Y := Y0 + (Y1 - Y0) * (X - X0) / (X1 - X0);
    ELSIF X <= X2 THEN
        IF X2 = X1 THEN Error := TRUE; ErrorID := 1; END_IF;
        Y := Y1 + (Y2 - Y1) * (X - X1) / (X2 - X1);
    ELSIF X <= X3 THEN
        IF X3 = X2 THEN Error := TRUE; ErrorID := 1; END_IF;
        Y := Y2 + (Y3 - Y2) * (X - X2) / (X3 - X2);
    ELSE
        Y := Y3;   // clamp above
    END_IF;
    IF NOT Error THEN Done := TRUE; END_IF;
END_FUNCTION_BLOCK

// ---- FB_CamProfile: master-position -> slave-position cam ----
// A simple linear cam: slave ramps 0..SlaveAmp as master goes
// 0..MasterEnd, then holds.  Model for a cam table lookup.
FUNCTION_BLOCK FB_CamProfile
VAR_INPUT
    Execute : BOOL;
    Master : LREAL;         // master position
    MasterEnd : LREAL;      // master end (cam period), >0
    SlaveAmp : LREAL;       // slave amplitude
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Slave : LREAL;
    InSync : BOOL;
END_VAR
VAR
    k : LREAL;
END_VAR
    Done := FALSE;
    IF Execute AND MasterEnd > 0.0 THEN
        k := Master;
        IF k < 0.0 THEN k := 0.0; END_IF;
        IF k > MasterEnd THEN k := MasterEnd; END_IF;
        Slave := SlaveAmp * k / MasterEnd;
        InSync := (Master >= 0.0) AND (Master <= MasterEnd);
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK
