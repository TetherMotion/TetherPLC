// t17_plant.st — FB_AtcPlant: FOPDT+deadtime plant simulator.
// Phase 1 (cycles 0..39, U=50 open loop): PV must stay near Pv0
// early (Theta=2 s dead time at 0.5 s scan) then rise toward
// K*U = 100.  Phase 2 (cycles 40+, PI control SP=80): PV settles
// in band.  flags: b0 settled, b1 no err, b2 plant Done,
// b3 output in range, b4 heated above 60, b5 dead time held PV<30
// at cycle 4.
PROGRAM T17
VAR
    flags AT %QB0 : BYTE;
    plant : FB_AtcPlant;
    pid : FB_AtcPid;
    cyc : DINT;
    u : LREAL;
    pvEarly : LREAL;
    maxPv : LREAL;
    closed : BOOL;
END_VAR
    cyc := cyc + 1;
    closed := cyc > 40;
    IF closed THEN
        pid(Execute := TRUE, Setpoint := 80.0, Actual := plant.PV,
            Kp := 3.0, Tn := 8.0, Tv := 0.0, TfD := 0.0,
            B := 1.0, C := 0.0, FFwd := 0.0,
            OutMin := 0.0, OutMax := 100.0, RateLim := 0.0,
            Deadband := 0.0, HoldI := FALSE, TrackEn := FALSE,
            TrackVal := 0.0, ManEn := FALSE, ManOut := 0.0,
            CycleTime := 0.5, Reset := FALSE);
        u := pid.Out;
    ELSE
        u := 50.0;
    END_IF;
    plant(Execute := TRUE, U := u, K := 2.0, Tau1 := 10.0,
          Tau2 := 0.0, Theta := 2.0, Pv0 := 20.0, DistU := 0.0,
          Taamb := 20.0, TauAmb := 0.0, NoiseAmp := 0.0,
          CycleTime := 0.5, Reset := cyc = 1);
    IF cyc = 4 THEN pvEarly := plant.PV; END_IF;
    IF plant.PV > maxPv THEN maxPv := plant.PV; END_IF;
    flags := 0;
    IF closed AND plant.PV > 74.0 AND plant.PV < 86.0 THEN
        flags := flags + 1;
    END_IF;
    IF NOT plant.Err AND NOT pid.Err THEN flags := flags + 2; END_IF;
    IF plant.Done THEN flags := flags + 4; END_IF;
    IF u >= 0.0 AND u <= 100.0 THEN flags := flags + 8; END_IF;
    IF maxPv > 60.0 THEN flags := flags + 16; END_IF;
    IF pvEarly > 0.0 AND pvEarly < 30.0 THEN flags := flags + 32; END_IF;
END_PROGRAM
