// t11_smith.st — FB_AtcSmith on a dead-time plant:
// K=1 deg/%, Tau=8 s, delay=7.5 s (15 scans); model identical.
// The dead-time compensation must hold SP=60 without oscillation.
// flags: b0 PV in band, b1 Out in range, b2 Done, b3 no Err,
//        b4 model delay state nonzero.
PROGRAM T11
VAR
    flags AT %QB0 : BYTE;
    c : FB_AtcSmith;
    pv : LREAL;
    dl : ARRAY[0..15] OF LREAL;
    i : DINT;
    u AT %QD8 : LREAL;
END_VAR
    c(Execute := TRUE, Setpoint := 60.0, Actual := pv,
      Kp := 0.6, Tn := 10.0, Tv := 0.0,
      ModelK := 1.0, ModelT := 8.0, ModelDt := 7.5,
      OutMin := 0.0, OutMax := 100.0,
      ManEn := FALSE, ManOut := 0.0,
      CycleTime := 0.5, Reset := FALSE);
    u := c.Out;
    // plant: 15-scan delay + 8 s lag, gain 1
    pv := pv + (1.0 * dl[15] - pv) * (0.5 / (8.0 + 0.5));
    FOR i := 0 TO 14 DO
        dl[15 - i] := dl[14 - i];
    END_FOR;
    dl[0] := u;
    flags := 0;
    IF pv > 58.0 AND pv < 62.5 THEN flags := flags + 1; END_IF;
    IF u >= -0.01 AND u <= 100.01 THEN flags := flags + 2; END_IF;
    IF c.Done THEN flags := flags + 4; END_IF;
    IF NOT c.Err THEN flags := flags + 8; END_IF;
    IF c.E > -100.0 AND c.E < 100.0 THEN flags := flags + 16; END_IF;
END_PROGRAM
