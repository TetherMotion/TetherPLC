// tc2_tempcontroller.st — Tc2_TempController surface — temperature
// control loop, pure-ST, no service or hardware.
//
// FB_TempController wraps a PID heating controller with a PWM duty
// output and a PT1 sensor filter — the classic temperature loop:
// filtered sensor -> PID (SP - PV) -> PWM duty to a heater.  The
// caller supplies the thermal plant (heater -> sensor -> PV) and
// the cycle time; the block handles sensor smoothing, PID and the
// duty output.

// ---- FB_TempController: temperature loop controller ----
FUNCTION_BLOCK FB_TempController
VAR_INPUT
    Execute : BOOL;
    Setpoint : LREAL;        // target temperature
    Actual : LREAL;          // raw sensor reading
    Kp : LREAL;              // proportional gain
    Tn : LREAL;              // integral time (s); 0 = no I
    Tv : LREAL;              // derivative time (s); 0 = no D
    FilterTn : LREAL;        // sensor filter time constant (s)
    PwmPeriod : LREAL;       // PWM period (s), >0
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Duty : LREAL;            // heater duty 0..100 %
    Pwm : BOOL;              // PWM output (TRUE during on-phase)
    Filtered : LREAL;        // filtered sensor reading
    E : LREAL;               // last error SP-Filtered
END_VAR
VAR
    filt : LREAL;
    integ : LREAL;
    prevE : LREAL;
    er : LREAL;
    p : LREAL;
    i : LREAL;
    d : LREAL;
    u : LREAL;
    t : LREAL;
    dd : LREAL;
END_VAR
    Done := FALSE;
    IF Reset THEN
        filt := 0.0;
        integ := 0.0;
        prevE := 0.0;
        t := 0.0;
    END_IF;
    IF Execute AND CycleTime > 0.0 AND PwmPeriod > 0.0 THEN
        // PT1 sensor filter
        IF FilterTn > 0.0 THEN
            filt := filt + (Actual - filt) *
                           (CycleTime / (FilterTn + CycleTime));
        ELSE
            filt := Actual;
        END_IF;
        // PID on the filtered reading
        er := Setpoint - filt;
        p := Kp * er;
        IF Tn > 0.0 THEN
            integ := integ + er * CycleTime / Tn;
        END_IF;
        i := Kp * integ;
        IF Tv > 0.0 THEN
            d := Kp * Tv * (er - prevE) / CycleTime;
        ELSE
            d := 0.0;
        END_IF;
        u := p + i + d;
        // duty 0..100 %
        IF u < 0.0 THEN u := 0.0; END_IF;
        IF u > 100.0 THEN u := 100.0; END_IF;
        Duty := u;
        // PWM phase accumulator
        t := t + CycleTime;
        IF t >= PwmPeriod THEN t := t - PwmPeriod; END_IF;
        dd := Duty;
        IF dd < 0.0 THEN dd := 0.0; END_IF;
        IF dd > 100.0 THEN dd := 100.0; END_IF;
        Pwm := (t < PwmPeriod * dd / 100.0);
        Filtered := filt;
        E := er;
        prevE := er;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK
