// t13_handoff.st — the WebUI autotune handoff shape, runtime-verified.
// Mirrors what fbd-lower's expand_st_blocks generates for an inline
// `st` block: VAR_IN_OUT pins aliasing tuner/controller FB instances,
// VAR_OUTPUT live mirrors, and a one-shot APPLY strobe copying gains
// into the controller's input members (CTL.K1 := TUNE.K1 …).
// Same model params as t06 → K1≈6.188 K2≈1.089 Ki≈0.671.
FUNCTION_BLOCK FB_TuneHandoff
VAR_INPUT
    APPLY : BOOL;
    RESET : BOOL;
END_VAR
VAR_OUTPUT
    lqr_k1 : LREAL;
    lqr_k2 : LREAL;
    lqr_ki : LREAL;
    lqr_done : BOOL;
END_VAR
VAR_IN_OUT
    TUNE : FB_AtcLqrTune;
    CTL : FB_AtcLqr;
END_VAR
VAR
    applied : BOOL;
END_VAR
    lqr_k1 := TUNE.K1;
    lqr_k2 := TUNE.K2;
    lqr_ki := TUNE.Ki;
    lqr_done := TUNE.Done;
    IF APPLY AND NOT applied THEN
        CTL.K1 := TUNE.K1;
        CTL.K2 := TUNE.K2;
        CTL.Ki := TUNE.Ki;
        applied := TRUE;
    END_IF;
    IF RESET THEN applied := FALSE; END_IF;
END_FUNCTION_BLOCK

PROGRAM T13
VAR
    flags AT %QB0 : BYTE;
    m : FB_AtcModel2s;
    t : FB_AtcLqrTune;
    c : FB_AtcLqr;
    ho : FB_TuneHandoff;
END_VAR
    m(Execute := TRUE, K := 1.0, Tau := 20.0, Theta := 0.0,
      TauH := 5.0, Ts := 0.5);
    t(Execute := TRUE, A11 := m.A11, A12 := m.A12,
      A21 := m.A21, A22 := m.A22, B1 := m.B1, B2 := m.B2,
      Q1 := 1.0, Q2 := 0.1, Qi := 0.5, R := 1.0, Ts := 0.5,
      MaxIter := 0, Tol := 0.0);
    // K1/K2/Ki intentionally omitted — the handoff owns them; binding
    // them here would clobber the tuned gains every scan (same reason
    // applyTunerToPou strips those params from the controller block).
    c(Execute := FALSE, Setpoint := 0.0, Actual := 0.0,
      X2 := 0.0,
      Nref := 0.0, FFwd := 0.0, OutMin := -100.0, OutMax := 100.0,
      RateLim := 0.0, ManEn := FALSE, ManOut := 0.0,
      CycleTime := 0.1, Reset := FALSE);
    ho(TUNE := t, CTL := c, APPLY := TRUE, RESET := FALSE);
    flags := 0;
    IF t.Done AND t.Conv THEN flags := flags + 1; END_IF;
    IF ABS(ho.lqr_k1 - t.K1) < 0.001 THEN flags := flags + 2; END_IF;
    IF c.K1 > 6.0 AND c.K1 < 6.4 THEN flags := flags + 4; END_IF;
    IF c.K2 > 1.0 AND c.K2 < 1.2 THEN flags := flags + 8; END_IF;
    IF c.Ki > 0.6 AND c.Ki < 0.75 THEN flags := flags + 16; END_IF;
END_PROGRAM
