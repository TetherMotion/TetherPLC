// tc2_controllertoolbox.st — Tc2_ControllerToolbox surface —
// additional controller primitives, pure-ST, no service or
// hardware: dead-time delay, hysteresis / two-position /
// three-position controllers, PWM duty converter, and
// running statistics (mean + standard deviation).
//
// All blocks take CycleTime as a VAR_INPUT (the caller's scan
// period, TwinCAT's T_PLC_US pattern).  DeadTime holds a ring of
// samples and outputs the N-cycle-delayed value.

// ---- FB_DeadTime: delay a signal N cycles ----
FUNCTION_BLOCK FB_DeadTime
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    DelayCycles : UDINT;   // 0..63
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
END_VAR
VAR
    buf : ARRAY[0..63] OF LREAL;
    idx : UDINT;
END_VAR
    Done := FALSE;
    IF Execute THEN
        buf[idx] := Input;
        Output := buf[(idx + 64 - (DelayCycles MOD 64)) MOD 64];
        idx := (idx + 1) MOD 64;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Hysteresis: relay with on/off hysteresis band ----
FUNCTION_BLOCK FB_Hysteresis
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    OnLevel : LREAL;   // switch ON at/below
    OffLevel : LREAL;  // switch OFF at/above (Off > On)
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Out : BOOL;
END_VAR
VAR
    on : BOOL;
END_VAR
    Done := FALSE;
    IF Execute THEN
        IF Input <= OnLevel THEN
            on := TRUE;
        ELSIF Input >= OffLevel THEN
            on := FALSE;
        END_IF;
        Out := on;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_TwoPosition: two-position (on/off) controller ----
FUNCTION_BLOCK FB_TwoPosition
VAR_INPUT
    Execute : BOOL;
    Setpoint : LREAL;
    Actual : LREAL;
    Hysteresis : LREAL;   // +- band around Setpoint
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Out : BOOL;           // TRUE = drive high
END_VAR
VAR
    e : LREAL;
END_VAR
    Done := FALSE;
    IF Execute THEN
        e := Setpoint - Actual;
        IF e > Hysteresis THEN
            Out := TRUE;      // below band -> drive on
        ELSIF e < -Hysteresis THEN
            Out := FALSE;     // above band -> drive off
        END_IF;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_ThreePosition: three-position controller ----
FUNCTION_BLOCK FB_ThreePosition
VAR_INPUT
    Execute : BOOL;
    Setpoint : LREAL;
    Actual : LREAL;
    Deadband : LREAL;     // +- neutral zone
    Hysteresis : LREAL;   // +- switch band beyond deadband
END_VAR
VAR_OUTPUT
    Done : BOOL;
    OutPos : INT;         // -1 = low, 0 = neutral, +1 = high
END_VAR
VAR
    e : LREAL;
END_VAR
    Done := FALSE;
    IF Execute THEN
        e := Setpoint - Actual;
        IF e > Deadband + Hysteresis THEN
            OutPos := 1;      // well below -> drive high
        ELSIF e < -Deadband - Hysteresis THEN
            OutPos := -1;     // well above -> drive low
        ELSIF e > -Deadband AND e < Deadband THEN
            OutPos := 0;      // inside deadband -> neutral
        END_IF;
        // else keep prior OutPos (hysteresis band)
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Statistics: running mean + standard deviation ----
FUNCTION_BLOCK FB_Statistics
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    N : UDINT;            // 1..64 — window length
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Mean : LREAL;
    StdDev : LREAL;
    Minimum : LREAL;
    Maximum : LREAL;
END_VAR
VAR
    buf : ARRAY[0..63] OF LREAL;
    idx : UDINT;
    cnt : UDINT;
    i : UDINT;
    s : LREAL;
    v : LREAL;
    m : LREAL;
    lo : LREAL;
    hi : LREAL;
    nR : LREAL;
    x : LREAL;     // Newton's sqrt iterate
    it : UDINT;
END_VAR
    Done := FALSE;
    IF Execute AND N > 0 AND N <= 64 THEN
        buf[idx] := Input;
        idx := (idx + 1);
        IF idx >= N THEN idx := 0; END_IF;
        IF cnt < N THEN cnt := cnt + 1; END_IF;
        s := 0.0; lo := buf[0]; hi := buf[0];
        FOR i := 0 TO 63 BY 1 DO
            IF i < cnt THEN
                s := s + buf[i];
                IF buf[i] < lo THEN lo := buf[i]; END_IF;
                IF buf[i] > hi THEN hi := buf[i]; END_IF;
            END_IF;
        END_FOR;
        nR := DINT_TO_LREAL(UDINT_TO_DINT(cnt));
        m := s / nR;
        v := 0.0;
        FOR i := 0 TO 63 BY 1 DO
            IF i < cnt THEN
                v := v + (buf[i] - m) * (buf[i] - m);
            END_IF;
        END_FOR;
        v := v / nR;
        // StdDev = sqrt(v) via Newton's method — no SQRT builtin
        IF v <= 0.0 THEN
            StdDev := 0.0;
        ELSE
            x := v;
            FOR it := 1 TO 12 BY 1 DO
                x := 0.5 * (x + v / x);
            END_FOR;
            StdDev := x;
        END_IF;
        Mean := m;
        Minimum := lo;
        Maximum := hi;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Pwm: duty-cycle converter (0..100% -> pulse width) ----
FUNCTION_BLOCK FB_Pwm
VAR_INPUT
    Execute : BOOL;
    Duty : LREAL;         // 0..100 percent
    Period : LREAL;       // period (s), >0
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Out : BOOL;           // TRUE during on-phase
END_VAR
VAR
    t : LREAL;            // phase accumulator (s)
    d : LREAL;
END_VAR
    Done := FALSE;
    IF Execute AND Period > 0.0 THEN
        t := t + CycleTime;
        IF t >= Period THEN t := t - Period; END_IF;
        d := Duty;
        IF d < 0.0 THEN d := 0.0; END_IF;
        IF d > 100.0 THEN d := 100.0; END_IF;
        Out := (t < Period * d / 100.0);
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK
