// tc2_filter.st — Tc2_Filter measurement/control surface
// (TwinCAT filter library) — pure-ST filter, controller and
// signal-processing blocks.
//
// These are pure IEC 61131-3 FBs — no service, no hardware: PID /
// PI controllers, PT1/PT2 lag elements, integrator, differentiator,
// limiter, ramp, and a running-average filter.  The caller supplies
// fCycleTime (the scan period in seconds, like TwinCAT's
// T_PLC_US-based timing); the blocks integrate/derive against it.
// Anti-reset-windup is clamped to the output limits on the PID.

// ---- FB_Limit: clamp a signal to [fLow, fHigh] ----
FUNCTION_BLOCK FB_Limit
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    Low : LREAL;
    High : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
END_VAR
    Done := FALSE;
    IF Execute THEN
        IF Input < Low THEN
            Output := Low;
        ELSIF Input > High THEN
            Output := High;
        ELSE
            Output := Input;
        END_IF;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_PT1: first-order lag element ----
FUNCTION_BLOCK FB_PT1
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    Kp : LREAL;        // static gain
    Tn : LREAL;        // time constant (s), >0 required
    CycleTime : LREAL; // scan period (s)
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
END_VAR
VAR
    y : LREAL;
END_VAR
    Done := FALSE;
    IF Execute AND Tn > 0.0 THEN
        // y += (Kp*in - y) * dt/(Tn+dt) — discrete PT1
        y := y + (Kp * Input - y) * (CycleTime / (Tn + CycleTime));
        Output := y;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Integrator: I element, y += in * dt / Tn ----
FUNCTION_BLOCK FB_Integrator
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    Tn : LREAL;        // integration time (s), >0 required
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
END_VAR
VAR
    y : LREAL;
END_VAR
    Done := FALSE;
    IF Reset THEN
        y := 0.0;
    END_IF;
    IF Execute AND Tn > 0.0 THEN
        y := y + Input * CycleTime / Tn;
        Output := y;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Differentiator: D element, y = (in - inPrev) * Tv / dt ----
FUNCTION_BLOCK FB_Differentiator
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    Tv : LREAL;        // derivative time (s)
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
END_VAR
VAR
    prev : LREAL;
END_VAR
    Done := FALSE;
    IF Execute AND CycleTime > 0.0 THEN
        Output := (Input - prev) * Tv / CycleTime;
        prev := Input;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Average: running average over the last N samples ----
FUNCTION_BLOCK FB_Average
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    N : UDINT;         // 1..64 — window length
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
END_VAR
VAR
    buf : ARRAY[0..63] OF LREAL;
    idx : UDINT;
    cnt : UDINT;
    sum : LREAL;
    i : UDINT;
    nR : LREAL;
END_VAR
    Done := FALSE;
    IF Execute AND N > 0 AND N <= 64 THEN
        buf[idx] := Input;
        idx := (idx + 1);
        IF idx >= N THEN idx := 0; END_IF;
        IF cnt < N THEN cnt := cnt + 1; END_IF;
        sum := 0.0;
        FOR i := 0 TO 63 BY 1 DO
            IF i < cnt THEN sum := sum + buf[i]; END_IF;
        END_FOR;
        nR := DINT_TO_LREAL(UDINT_TO_DINT(cnt));
        Output := sum / nR;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Ramp: rate-limited approach to a setpoint ----
FUNCTION_BLOCK FB_Ramp
VAR_INPUT
    Execute : BOOL;
    Setpoint : LREAL;
    SlewRate : LREAL;  // units per second, >0
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
    InRange : BOOL;
END_VAR
VAR
    y : LREAL;
    delta : LREAL;
END_VAR
    Done := FALSE;
    InRange := FALSE;
    IF Execute AND SlewRate > 0.0 THEN
        delta := Setpoint - y;
        IF delta > SlewRate * CycleTime THEN
            y := y + SlewRate * CycleTime;
        ELSIF delta < -SlewRate * CycleTime THEN
            y := y - SlewRate * CycleTime;
        ELSE
            y := Setpoint;
            InRange := TRUE;
        END_IF;
        Output := y;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_PID: PID controller with anti-reset-windup ----
FUNCTION_BLOCK FB_PID
VAR_INPUT
    Execute : BOOL;
    Setpoint : LREAL;
    Actual : LREAL;
    Kp : LREAL;        // proportional gain
    Tn : LREAL;        // integral time (s); 0 = no I
    Tv : LREAL;        // derivative time (s); 0 = no D
    OutMin : LREAL;    // output clamp low
    OutMax : LREAL;    // output clamp high
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
    E : LREAL;         // last error SP-Actual
END_VAR
VAR
    integ : LREAL;
    prevE : LREAL;
    e : LREAL;
    p : LREAL;
    i : LREAL;
    d : LREAL;
    u : LREAL;
END_VAR
    Done := FALSE;
    IF Reset THEN
        integ := 0.0;
        prevE := 0.0;
    END_IF;
    IF Execute AND CycleTime > 0.0 THEN
        e := Setpoint - Actual;
        p := Kp * e;
        // integrator with Tn; clamped anti-reset-windup
        IF Tn > 0.0 THEN
            integ := integ + e * CycleTime / Tn;
        END_IF;
        i := Kp * integ;
        IF Tv > 0.0 THEN
            d := Kp * Tv * (e - prevE) / CycleTime;
        ELSE
            d := 0.0;
        END_IF;
        u := p + i + d;
        // clamp + back-calculate anti-reset-windup on I
        IF u > OutMax THEN
            u := OutMax;
            integ := (u - p - d) / Kp;
        ELSIF u < OutMin THEN
            u := OutMin;
            integ := (u - p - d) / Kp;
        END_IF;
        Output := u;
        E := e;
        prevE := e;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK
