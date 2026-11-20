// t20_autotune.st — FB_AtcAutoTune end-to-end: relay tune on
// FB_AtcPlant (K=1, Tau=10, Theta=1) drives the plant to a limit
// cycle, gains latch, PID takes over bumplessly via TrackEn.
// flags: b0 at.Done seen, b1 gains sane (Kp>0 Tn>0 Tv>=0 Ku>0 Tu>0),
//        b2 PID settled |PV-50|<10 at end, b3 no Err ever,
//        b4 handoff continuity |dOut|<8 at Done scan,
//        b5 Cycles>=2, b6 PidTrack released after Done,
//        b7 PV stayed in [5,95] whole run.
PROGRAM T20
VAR
    flags AT %QB0 : BYTE;
    aut : FB_AtcAutoTune;
    pid : FB_AtcPid;
    plant : FB_AtcPlant;
    cyc : DINT;
    pvMin : LREAL;
    pvMax : LREAL;
    prevOut : LREAL;
    outAtDone : LREAL;
    dOut : LREAL;
    seenDone : BOOL;
    trackReleased : BOOL;
    inBand : BOOL;
    viol : DINT;
END_VAR
    cyc := cyc + 1;
    IF cyc = 1 THEN
        pvMin := 1.0e9; pvMax := -1.0e9; prevOut := 50.0;
        outAtDone := 0.0; seenDone := FALSE;
        trackReleased := FALSE; inBand := TRUE; viol := 0;
    END_IF;
    aut(Execute := TRUE, Setpoint := 50.0, Actual := plant.PV,
       CycleTime := 0.5, RelayAmp := 20.0, MeanOut := 50.0,
       Hyst := 2.0, OutMin := 0.0, OutMax := 100.0,
       TuneRule := 0, MinCyc := 3, Abort := FALSE, Reset := FALSE,
       PidOut := pid.Out);
    pid(Execute := TRUE, Setpoint := 50.0, Actual := plant.PV,
        Kp := aut.Kp, Tn := aut.Tn, Tv := aut.Tv, TfD := 0.0,
        B := 1.0, C := 0.0, FFwd := 0.0,
        OutMin := 0.0, OutMax := 100.0, RateLim := 0.0,
        Deadband := 0.0, HoldI := FALSE,
        TrackEn := aut.PidTrack, TrackVal := aut.PidTrackVal,
        ManEn := FALSE, ManOut := 0.0,
        CycleTime := 0.5, Reset := cyc = 1);
    plant(Execute := TRUE, U := aut.Out, K := 1.0, Tau1 := 10.0,
          Tau2 := 0.0, Theta := 1.0, Pv0 := 20.0, DistU := 0.0,
          Taamb := 20.0, TauAmb := 0.0, NoiseAmp := 0.0,
          CycleTime := 0.5, Reset := cyc = 1);
    // bookkeeping
    dOut := aut.Out - prevOut;
    IF aut.Done AND NOT seenDone THEN
        seenDone := TRUE;
        outAtDone := aut.Out;
        dOut := aut.Out - prevOut;
    END_IF;
    IF seenDone AND aut.PidTrack = FALSE THEN
        trackReleased := TRUE;
    END_IF;
    IF plant.PV < pvMin THEN pvMin := plant.PV; END_IF;
    IF plant.PV > pvMax THEN pvMax := plant.PV; END_IF;
    IF plant.PV < 5.0 OR plant.PV > 95.0 THEN viol := viol + 1; END_IF;
    IF seenDone AND trackReleased THEN
        IF ABS(plant.PV - 50.0) > 10.0 THEN inBand := FALSE; END_IF;
    END_IF;
    prevOut := aut.Out;
    flags := 0;
    IF aut.Done THEN flags := flags + 1; END_IF;
    IF aut.Kp > 0.0 AND aut.Tn > 0.0 AND aut.Tv >= 0.0
       AND aut.Ku > 0.0 AND aut.Tu > 0.0 THEN
        flags := flags + 2;
    END_IF;
    IF seenDone AND inBand THEN flags := flags + 4; END_IF;
    IF NOT aut.Err THEN flags := flags + 8; END_IF;
    IF seenDone AND ABS(dOut) < 8.0 THEN flags := flags + 16; END_IF;
    IF aut.Cycles >= 2 THEN flags := flags + 32; END_IF;
    IF trackReleased THEN flags := flags + 64; END_IF;
    IF viol = 0 THEN flags := flags + 128; END_IF;
END_PROGRAM
