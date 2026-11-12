// t02_pid.st — FB_AtcPid closed loop on a simulated first-order
// plant (K=2 deg/%, Tau=10 s, Ts=0.5 s): PI must pull PV from 20 to
// SP=80 and hold it there. flags: b0 PV in band, b1 output in range,
// b2 Done, b3 no error, b4 anti-windup: output never exceeded 100.
PROGRAM T02
VAR
    flags AT %QB0 : BYTE;
    c : FB_AtcPid;
    pv : LREAL;
    u AT %QD8 : LREAL;
    pvO AT %QD16 : LREAL;
    satHist : LREAL;
END_VAR
    c(Execute := TRUE, Setpoint := 80.0, Actual := pv,
      Kp := 3.0, Tn := 8.0, Tv := 0.0, TfD := 0.0,
      B := 1.0, C := 0.0, FFwd := 0.0,
      OutMin := 0.0, OutMax := 100.0, RateLim := 0.0,
      Deadband := 0.0, HoldI := FALSE, TrackEn := FALSE,
      TrackVal := 0.0, ManEn := FALSE, ManOut := 0.0,
      CycleTime := 0.5, Reset := FALSE);
    u := c.Out;
    IF u > 100.01 OR u < -0.01 THEN
        satHist := 1.0;
    END_IF;
    // plant lag (start PV at 20 deg)
    pv := pv + (2.0 * u - pv) * (0.5 / (10.0 + 0.5));
    pvO := pv;
    flags := 0;
    IF pv > 78.5 AND pv < 82.5 THEN flags := flags + 1; END_IF;
    IF u >= 0.0 AND u <= 100.0 THEN flags := flags + 2; END_IF;
    IF c.Done THEN flags := flags + 4; END_IF;
    IF NOT c.Err THEN flags := flags + 8; END_IF;
    IF satHist = 0.0 THEN flags := flags + 16; END_IF;
END_PROGRAM
