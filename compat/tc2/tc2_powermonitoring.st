// tc2_powermonitoring.st — Tc2_PowerMonitoring surface — AC power
// metering, pure-ST, no service or hardware.
//
// FB_PowerMonitor wraps single-phase AC metering over a rolling
// sample window: RMS voltage and current, real (P) / apparent (S) /
// reactive (Q) power, power factor, and accumulated energy
// (kWh-style, integrated from the sample rate).  The caller feeds
// instantaneous V/I samples and the cycle time; the block
// integrates and computes over the window.
//
// Energy accumulates in watt-hours per hour of sim time (dt is in
// seconds -> Ws -> Wh at 3600 divisor on the displayed value).

// ---- FB_PowerMonitor: single-phase AC metering ----
FUNCTION_BLOCK FB_PowerMonitor
VAR_INPUT
    Execute : BOOL;
    Voltage : LREAL;       // instantaneous V sample
    Current : LREAL;       // instantaneous I sample
    N : UDINT;             // 1..64 — RMS window length
    CycleTime : LREAL;     // sample period (s)
    Reset : BOOL;          // reset energy accumulator
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    VRms : LREAL;          // RMS voltage
    IRms : LREAL;          // RMS current
    PReal : LREAL;         // real power (W)
    SApparent : LREAL;     // apparent power (VA)
    QReactive : LREAL;     // reactive power (var)
    PF : LREAL;            // power factor 0..1
    Energy : LREAL;        // accumulated energy (Wh)
END_VAR
VAR
    vbuf : ARRAY[0..63] OF LREAL;
    ibuf : ARRAY[0..63] OF LREAL;
    idx : UDINT;
    cnt : UDINT;
    i : UDINT;
    vs : LREAL;
    isq : LREAL;
    pp : LREAL;
    nR : LREAL;
    q : LREAL;
    x : LREAL;
    it : UDINT;
    e : LREAL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Reset THEN e := 0.0; END_IF;
    IF Execute AND N > 0 AND N <= 64 THEN
        vbuf[idx] := Voltage;
        ibuf[idx] := Current;
        idx := (idx + 1);
        IF idx >= N THEN idx := 0; END_IF;
        IF cnt < N THEN cnt := cnt + 1; END_IF;

        // RMS + real power over the window
        vs := 0.0; isq := 0.0; pp := 0.0;
        FOR i := 0 TO 63 BY 1 DO
            IF i < cnt THEN
                vs := vs + vbuf[i] * vbuf[i];
                isq := isq + ibuf[i] * ibuf[i];
                pp := pp + vbuf[i] * ibuf[i];
            END_IF;
        END_FOR;
        nR := DINT_TO_LREAL(UDINT_TO_DINT(cnt));
        vs := vs / nR;
        isq := isq / nR;
        pp := pp / nR;

        // VRms = sqrt(vs), IRms = sqrt(isq) — Newton's sqrt
        IF vs <= 0.0 THEN VRms := 0.0;
        ELSE
            x := vs;
            FOR it := 1 TO 12 BY 1 DO x := 0.5 * (x + vs / x); END_FOR;
            VRms := x;
        END_IF;
        IF isq <= 0.0 THEN IRms := 0.0;
        ELSE
            x := isq;
            FOR it := 1 TO 12 BY 1 DO x := 0.5 * (x + isq / x); END_FOR;
            IRms := x;
        END_IF;

        PReal := pp;
        SApparent := VRms * IRms;
        q := SApparent * SApparent - PReal * PReal;
        IF q <= 0.0 THEN
            QReactive := 0.0;
        ELSE
            x := q;
            FOR it := 1 TO 12 BY 1 DO x := 0.5 * (x + q / x); END_FOR;
            QReactive := x;
        END_IF;
        IF SApparent > 0.0 THEN
            PF := PReal / SApparent;
            IF PF < 0.0 THEN PF := -PF; END_IF;
            IF PF > 1.0 THEN PF := 1.0; END_IF;
        ELSE
            PF := 0.0;
        END_IF;

        // energy accumulate: W * s = Ws -> Wh at 3600
        e := e + PReal * CycleTime / 3600.0;
        Energy := e;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK
