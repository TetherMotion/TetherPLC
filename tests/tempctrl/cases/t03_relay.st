// t03_relay.st — FB_AtcRelayTune on a simulated first-order plant
// (K=1, Tau=10 s, Ts=0.5 s): relay hysteresis 2 deg around SP=50,
// MeanOut=50, amp=20. Must finish with Done, plausible Ku/Tu, and
// ZN-classic gains Kp=0.6Ku Tn=0.5Tu Tv=0.125Tu.
// flags: b0 Done, b1 Ku in [4,20], b2 Tu in [4,60], b3 Kp>0,
//        b4 Tn=0.5*Tu, b5 Tv=0.125*Tu, b6 Busy went low, b7 no Err.
PROGRAM T03
VAR
    flags AT %QB0 : BYTE;
    c : FB_AtcRelayTune;
    pv : LREAL;
    u AT %QD8 : LREAL;
    kuO AT %QD16 : LREAL;
    tuO AT %QD24 : LREAL;
END_VAR
    c(Execute := TRUE, Setpoint := 50.0, Actual := pv,
      RelayAmp := 20.0, MeanOut := 50.0, Hyst := 2.0,
      OutMin := 0.0, OutMax := 100.0, TuneRule := 0,
      MinCyc := 3, CycleTime := 0.5,
      Abort := FALSE, Reset := FALSE);
    u := c.Out;
    pv := pv + (1.0 * u - pv) * (0.5 / (10.0 + 0.5));
    kuO := c.Ku;
    tuO := c.Tu;
    flags := 0;
    IF c.Done THEN flags := flags + 1; END_IF;
    IF c.Ku > 4.0 AND c.Ku < 30.0 THEN flags := flags + 2; END_IF;
    IF c.Tu > 4.0 AND c.Tu < 40.0 THEN flags := flags + 4; END_IF;
    IF c.Kp > 0.0 THEN flags := flags + 8; END_IF;
    IF ABS(c.Tn - 0.5 * c.Tu) < 0.01 THEN flags := flags + 16; END_IF;
    IF ABS(c.Tv - 0.125 * c.Tu) < 0.01 THEN flags := flags + 32; END_IF;
    IF NOT c.Busy THEN flags := flags + 64; END_IF;
    IF NOT c.Err THEN flags := flags + 128; END_IF;
END_PROGRAM
