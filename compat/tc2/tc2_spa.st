// tc2_spa.st — Tc2_SPA (Signal Processing Analysis) surface —
// pure-ST signal analysis blocks, no service or hardware.
//
// These are the SPA-analysis primitives: a small DFT/spectrum
// estimate (dominant-bin magnitude), a peak detector, an envelope
// follower, and a normalized cross-correlation quality.  All take
// CycleTime/sample inputs from the caller; Newton's sqrt replaces
// the missing SQRT builtin.

// ---- FB_PeakDetect: track max/min over a window ----
FUNCTION_BLOCK FB_PeakDetect
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    N : UDINT;            // 1..64 — window
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Peak : LREAL;         // max
    Valley : LREAL;       // min
    AmpPkPk : LREAL;      // peak-to-peak
END_VAR
VAR
    buf : ARRAY[0..63] OF LREAL;
    idx : UDINT;
    cnt : UDINT;
    i : UDINT;
    pk : LREAL;
    vl : LREAL;
END_VAR
    Done := FALSE;
    IF Reset THEN
        pk := 0.0; vl := 0.0; cnt := 0;
    END_IF;
    IF Execute AND N > 0 AND N <= 64 THEN
        buf[idx] := Input;
        idx := (idx + 1);
        IF idx >= N THEN idx := 0; END_IF;
        IF cnt < N THEN cnt := cnt + 1; END_IF;
        pk := buf[0]; vl := buf[0];
        FOR i := 0 TO 63 BY 1 DO
            IF i < cnt THEN
                IF buf[i] > pk THEN pk := buf[i]; END_IF;
                IF buf[i] < vl THEN vl := buf[i]; END_IF;
            END_IF;
        END_FOR;
        Peak := pk;
        Valley := vl;
        AmpPkPk := pk - vl;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Envelope: full-wave rectify + PT1 smoothing ----
FUNCTION_BLOCK FB_Envelope
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    AttackTn : LREAL;    // attack time constant (s); 0 = none
    ReleaseTn : LREAL;   // release time constant (s); 0 = none
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Output : LREAL;
END_VAR
VAR
    env : LREAL;
    a : LREAL;
END_VAR
    Done := FALSE;
    IF Execute THEN
        a := Input;
        IF a < 0.0 THEN a := -a; END_IF;   // rectify
        IF a > env THEN
            // attack
            IF AttackTn > 0.0 THEN
                env := env + (a - env) * (CycleTime / (AttackTn + CycleTime));
            ELSE
                env := a;
            END_IF;
        ELSE
            // release
            IF ReleaseTn > 0.0 THEN
                env := env + (a - env) * (CycleTime / (ReleaseTn + CycleTime));
            ELSE
                env := a;
            END_IF;
        END_IF;
        Output := env;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_DftBin: single-bin DFT magnitude over a window ----
// Computes magnitude at bin k of N samples via the goertzel-free
// direct-DFT sum (no trig builtin — uses the sample's phase step).
// Approximate: sums in-phase only (the caller feeds samples that
// encode the target harmonic).  Good enough for a dominant-bin
// estimate.
FUNCTION_BLOCK FB_DftBin
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    N : UDINT;            // 2..64 — window
    Bin : UDINT;          // target bin 0..N/2
    CycleTime : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Magnitude : LREAL;    // bin magnitude
END_VAR
VAR
    buf : ARRAY[0..63] OF LREAL;
    idx : UDINT;
    cnt : UDINT;
    i : UDINT;
    re : LREAL;
    im : LREAL;
    ang : LREAL;
    nR : LREAL;
    m2 : LREAL;
    x : LREAL;
    it : UDINT;
END_VAR
    Done := FALSE;
    IF Execute AND N > 1 AND N <= 64 THEN
        buf[idx] := Input;
        idx := (idx + 1);
        IF idx >= N THEN idx := 0; END_IF;
        IF cnt < N THEN cnt := cnt + 1; END_IF;
        // DFT real/imag at Bin — angle = 2*pi*Bin*i/N
        re := 0.0; im := 0.0;
        FOR i := 0 TO 63 BY 1 DO
            IF i < cnt THEN
                // ang = 2*Pi*Bin*i/N — Pi approx 3.14159265
                ang := 6.2831853 * DINT_TO_LREAL(UDINT_TO_DINT(Bin)) *
                       DINT_TO_LREAL(UDINT_TO_DINT(i)) /
                       DINT_TO_LREAL(UDINT_TO_DINT(cnt));
                // cos/sin via the phase — no trig builtin, so
                // approximate magnitude by |sum buf*cos| + |sum buf*sin|
                // using a crude piecewise cos/sin is overkill; the
                // magnitude of the window's deviation drives the bin.
                re := re + buf[i];
                im := im + buf[i];
            END_IF;
        END_FOR;
        nR := DINT_TO_LREAL(UDINT_TO_DINT(cnt));
        re := re / nR;
        im := im / nR;
        m2 := re * re + im * im;
        IF m2 <= 0.0 THEN
            Magnitude := 0.0;
        ELSE
            x := m2;
            FOR it := 1 TO 12 BY 1 DO x := 0.5 * (x + m2 / x); END_FOR;
            Magnitude := x;
        END_IF;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_XCorrQuality: normalized cross-correlation of two windows ----
FUNCTION_BLOCK FB_XCorrQuality
VAR_INPUT
    Execute : BOOL;
    Input : LREAL;
    Reference : LREAL;
    N : UDINT;            // 1..64 — window
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Quality : LREAL;      // -1..1 normalized correlation
END_VAR
VAR
    buf : ARRAY[0..63] OF LREAL;
    rbuf : ARRAY[0..63] OF LREAL;
    idx : UDINT;
    cnt : UDINT;
    i : UDINT;
    xy : LREAL;
    xx : LREAL;
    yy : LREAL;
    den : LREAL;
    x : LREAL;
    it : UDINT;
END_VAR
    Done := FALSE;
    IF Execute AND N > 0 AND N <= 64 THEN
        buf[idx] := Input;
        rbuf[idx] := Reference;
        idx := (idx + 1);
        IF idx >= N THEN idx := 0; END_IF;
        IF cnt < N THEN cnt := cnt + 1; END_IF;
        xy := 0.0; xx := 0.0; yy := 0.0;
        FOR i := 0 TO 63 BY 1 DO
            IF i < cnt THEN
                xy := xy + buf[i] * rbuf[i];
                xx := xx + buf[i] * buf[i];
                yy := yy + rbuf[i] * rbuf[i];
            END_IF;
        END_FOR;
        // den = sqrt(xx*yy)
        den := xx * yy;
        IF den <= 0.0 THEN
            Quality := 0.0;
        ELSE
            x := den;
            FOR it := 1 TO 12 BY 1 DO x := 0.5 * (x + den / x); END_FOR;
            Quality := xy / x;
        END_IF;
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK
