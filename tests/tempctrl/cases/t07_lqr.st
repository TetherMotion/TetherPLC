// t07_lqr.st — FB_AtcLqr closed loop on the discrete 2-state sim it
// was tuned for (same A/B as t06). K1=6.1884 K2=1.0892 Ki=0.6712,
// Nref=0 (integral does the tracking). SP=80, x1 starts 0.
// flags: b0 x1 in band, b1 Out in range, b2 Done, b3 no Err,
//        b4 xi finite.
PROGRAM T07
VAR
    flags AT %QB0 : BYTE;
    c : FB_AtcLqr;
    x1 : LREAL;
    x2 : LREAL;
    u AT %QD8 : LREAL;
END_VAR
    c(Execute := TRUE, Setpoint := 80.0, Actual := x1, X2 := x2,
      K1 := 6.1884, K2 := 1.0892, Ki := 0.6712,
      Nref := 0.0, FFwd := 0.0,
      OutMin := 0.0, OutMax := 100.0, RateLim := 0.0,
      ManEn := FALSE, ManOut := 0.0,
      CycleTime := 0.5, Reset := FALSE);
    u := c.Out;
    // plant (discrete, Ts=0.5): x1+ = a1 x1 + (1-a1) x2
    //                         x2+ = a2 x2 + K(1-a2) u
    x1 := 0.9753099 * x1 + 0.0246901 * x2;
    x2 := 0.9048374 * x2 + 0.0951626 * u;
    flags := 0;
    IF x1 > 79.0 AND x1 < 81.0 THEN flags := flags + 1; END_IF;
    IF u >= -0.01 AND u <= 100.01 THEN flags := flags + 2; END_IF;
    IF c.Done THEN flags := flags + 4; END_IF;
    IF NOT c.Err THEN flags := flags + 8; END_IF;
    IF c.Xi > -1.0e6 AND c.Xi < 1.0e6 THEN flags := flags + 16; END_IF;
END_PROGRAM
