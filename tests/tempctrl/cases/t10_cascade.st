// t10_cascade.st — FB_AtcCascadePid: outer product loop (slow, Tau=30)
// drives inner heater loop (fast, Tau=2) setpoint range 20..150;
// inner drives duty 0..100 with gain 1.5 deg/%.
// flags: b0 product PV within band, b1 inner SP clamped to range,
//        b2 Out in range, b3 Done, b4 no Err.
PROGRAM T10
VAR
    flags AT %QB0 : BYTE;
    c : FB_AtcCascadePid;
    pvO : LREAL;
    pvI : LREAL;
    u AT %QD8 : LREAL;
    ispO AT %QD16 : LREAL;
    badIsp : BOOL;
END_VAR
    c(Execute := TRUE, Setpoint := 80.0,
      PvOuter := pvO, PvInner := pvI,
      KpO := 2.0, TnO := 40.0, TvO := 0.0,
      KpI := 4.0, TnI := 4.0, TvI := 0.0,
      InnerSpMin := 20.0, InnerSpMax := 150.0,
      OutMin := 0.0, OutMax := 100.0,
      ManEn := FALSE, ManOut := 0.0,
      CycleTime := 0.5, Reset := FALSE);
    u := c.Out;
    ispO := c.InnerSp;
    IF c.InnerSp < 19.99 OR c.InnerSp > 150.01 THEN
        badIsp := TRUE;
    END_IF;
    // plants: heater lag Tau=2 gain 1.5; product follows heater Tau=30
    pvI := pvI + (1.5 * u - pvI) * (0.5 / (2.0 + 0.5));
    pvO := pvO + (pvI - pvO) * (0.5 / (30.0 + 0.5));
    flags := 0;
    IF pvO > 77.0 AND pvO < 83.0 THEN flags := flags + 1; END_IF;
    IF NOT badIsp THEN flags := flags + 2; END_IF;
    IF u >= -0.01 AND u <= 100.01 THEN flags := flags + 4; END_IF;
    IF c.Done THEN flags := flags + 8; END_IF;
    IF pvI > 0.0 AND pvI <= 150.01 THEN flags := flags + 16; END_IF;
END_PROGRAM
