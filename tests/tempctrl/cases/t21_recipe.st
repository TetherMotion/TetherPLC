// t21_recipe.st — FB_AtcRecipe: preset-gain profile run with trend.
// 4 segments: RAMP 20->60 @5deg/s (PvStart), DWELL 60 for 20 s,
// RAMP ->30 @5deg/s, END.  PID (preset Kp=3,Tn=12) regulates
// FB_AtcPlant; trend triggers on segment 2.
// flags: b0 Done, b1 Kp stayed preset 3.0, b2 SegIdx hit END (>=4),
//        b3 trend Count>=50, b4 SP reached >=59.5, b5 no Err,
//        b6 trend triggered (TrigIdx>=0), b7 final SP=30.
PROGRAM T21
VAR
    flags AT %QB0 : BYTE;
    rcp : FB_AtcRecipe;
    pid : FB_AtcPid;
    plant : FB_AtcPlant;
    cyc : DINT;
    maxSP : LREAL;
    started : BOOL;
END_VAR
    cyc := cyc + 1;
    IF cyc = 1 THEN
        maxSP := 0.0; started := FALSE;
        // segment table (member config, like prof.SEG[i].KIND := ...)
        rcp.SEG[1].KIND := 2;  rcp.SEG[1].TGT := 60.0;
        rcp.SEG[1].P1 := 5.0;  rcp.SEG[1].P2 := 0.0;
        rcp.SEG[2].KIND := 1;  rcp.SEG[2].TGT := 60.0;
        rcp.SEG[2].P1 := 20.0; rcp.SEG[2].P2 := 0.0;
        rcp.SEG[3].KIND := 2;  rcp.SEG[3].TGT := 30.0;
        rcp.SEG[3].P1 := 5.0;  rcp.SEG[3].P2 := 0.0;
        rcp.SEG[4].KIND := 0;
        rcp.nseg := 4;
    END_IF;
    rcp(Execute := TRUE, START := cyc = 2, HOLD := FALSE,
        RESET := cyc = 1, PV := plant.PV, CycleTime := 0.5,
        AutoTune := FALSE, SP_Tune := 50.0,
        RelayAmp := 20.0, MeanOut := 50.0, Hyst := 2.0,
        OutMin := 0.0, OutMax := 100.0, TuneRule := 0, MinCyc := 3,
        AbortTune := FALSE,
        PresetKp := 3.0, PresetTn := 12.0, PresetTv := 0.0,
        TrendEn := TRUE, TrendDepth := 128, PostN := 8,
        TrigSeg := 2, PidOut := pid.Out);
    pid(Execute := TRUE, Setpoint := rcp.SP, Actual := plant.PV,
        Kp := rcp.Kp, Tn := rcp.Tn, Tv := rcp.Tv, TfD := 0.0,
        B := 1.0, C := 0.0, FFwd := 0.0,
        OutMin := 0.0, OutMax := 100.0, RateLim := 0.0,
        Deadband := 0.0, HoldI := FALSE,
        TrackEn := rcp.PidTrack, TrackVal := rcp.PidTrackVal,
        ManEn := FALSE, ManOut := 0.0,
        CycleTime := 0.5, Reset := cyc = 1);
    plant(Execute := TRUE, U := rcp.Out, K := 1.5, Tau1 := 12.0,
          Tau2 := 0.0, Theta := 1.0, Pv0 := 20.0, DistU := 0.0,
          Taamb := 20.0, TauAmb := 0.0, NoiseAmp := 0.0,
          CycleTime := 0.5, Reset := cyc = 1);
    IF rcp.Busy THEN started := TRUE; END_IF;
    IF rcp.SP > maxSP THEN maxSP := rcp.SP; END_IF;
    flags := 0;
    IF rcp.Done THEN flags := flags + 1; END_IF;
    IF rcp.Kp = 3.0 AND rcp.Tn = 12.0 THEN flags := flags + 2; END_IF;
    IF rcp.SegIdx >= 4 THEN flags := flags + 4; END_IF;
    IF rcp.tr.Count >= 50 THEN flags := flags + 8; END_IF;
    IF maxSP >= 59.5 THEN flags := flags + 16; END_IF;
    IF NOT rcp.Err THEN flags := flags + 32; END_IF;
    IF rcp.TrendTrigIdx >= 0 THEN flags := flags + 64; END_IF;
    IF rcp.Done AND rcp.SP = 30.0 THEN flags := flags + 128; END_IF;
END_PROGRAM
