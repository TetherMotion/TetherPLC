// t08_lqg.st — FB_AtcLqg closed loop: same plant as t07 but PV is
// corrupted by a deterministic +/- noise square wave (amplitude 0.5);
// the Kalman filter (L1=0.0777 L2=0.00609) must still track.
// flags: b0 x1 plant state in band, b1 estimated X1 near PV mean,
//        b2 Out bounded, b3 Done, b4 no Err, b5 X2 estimate finite.
PROGRAM T08
VAR
    flags AT %QB0 : BYTE;
    c : FB_AtcLqg;
    x1 : LREAL;
    x2 : LREAL;
    nz : LREAL;
    ymeas : LREAL;
    u AT %QD8 : LREAL;
    xe AT %QD16 : LREAL;
END_VAR
    // deterministic measurement noise: +-0.5 alternating every 7 scans
    nz := nz + 0.5;
    IF nz >= 7.0 THEN nz := -7.0; END_IF;
    ymeas := x1 + nz / 7.0 * 0.5 - 0.25;
    c(Execute := TRUE, Setpoint := 80.0, Actual := ymeas,
      A11 := 0.9753099, A12 := 0.0246901,
      A21 := 0.0, A22 := 0.9048374,
      B1 := 0.0, B2 := 0.0951626,
      L1 := 0.0777, L2 := 0.00609,
      K1 := 6.1884, K2 := 1.0892, Ki := 0.6712,
      Nref := 0.0, FFwd := 0.0,
      OutMin := 0.0, OutMax := 100.0,
      ManEn := FALSE, ManOut := 0.0,
      CycleTime := 0.5, Reset := FALSE);
    u := c.Out;
    x1 := 0.9753099 * x1 + 0.0246901 * x2;
    x2 := 0.9048374 * x2 + 0.0951626 * u;
    xe := c.X1;
    flags := 0;
    IF x1 > 78.0 AND x1 < 82.0 THEN flags := flags + 1; END_IF;
    IF c.X1 > 78.0 AND c.X1 < 82.0 THEN flags := flags + 2; END_IF;
    IF u >= -0.01 AND u <= 100.01 THEN flags := flags + 4; END_IF;
    IF c.Done THEN flags := flags + 8; END_IF;
    IF NOT c.Err THEN flags := flags + 16; END_IF;
    IF c.X2 > -1.0e6 AND c.X2 < 1.0e6 THEN flags := flags + 32; END_IF;
END_PROGRAM
