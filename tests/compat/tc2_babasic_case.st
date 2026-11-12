// tc2_babasic_case.st — pure-ST e2e for tc2_babasic.
// Exercises the core blocks; mask bits land per check.
PROGRAM tc2_babasic_case
VAR
    thr : FB_ThresholdSwitch;
    ramp : FB_Ramp;
    lgt : FB_Light;
    stair : FB_StairwellLight;
    mdc : FB_MaximumDemandController;
    sched : FB_DailyScheduler;
    scn : FB_ScenesLighting;
    scl : LREAL;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        // F_Scale: 50 in 0..100 -> 0..10 = 5
        scl := F_Scale(50.0, 0.0, 100.0, 0.0, 10.0);
        IF scl > 4.9 AND scl < 5.1 THEN mask := mask OR 1; END_IF;
        st := 1;
    ELSIF st = 1 THEN
        thr.x := 10.0; thr.On := 5.0; thr.Off := 1.0;
        thr();
        IF thr.Q THEN mask := mask OR 2; END_IF;
        st := 2;
    ELSIF st = 2 THEN
        ramp.Enable := TRUE;
        ramp.Setpoint := 10.0;
        ramp.SlewUp := 5.0;
        ramp.SlewDown := 5.0;
        ramp.CycleTime := 0.1;
        IF ramp.Out > 0.4 THEN mask := mask OR 4; st := 3; END_IF;
    ELSIF st = 3 THEN
        // run ramp to target over 25 scans
        diag := diag + 0;         // keep diag = st
        IF ramp.AtTarget THEN mask := mask OR 8; st := 4; END_IF;
    ELSIF st = 4 THEN
        lgt.On := TRUE;
        lgt.OffDelay := 0.3;
        lgt.CycleTime := 0.1;
        IF lgt.Q THEN mask := mask OR 16; st := 5; END_IF;
    ELSIF st = 5 THEN
        stair.Trigger := TRUE;
        stair.Duration := 0.5;
        stair.CycleTime := 0.1;
        IF stair.Q THEN mask := mask OR 32; st := 6; END_IF;
    ELSIF st = 6 THEN
        mdc.Enable := TRUE;
        mdc.In1 := TRUE; mdc.In2 := TRUE; mdc.In3 := TRUE; mdc.In4 := TRUE;
        mdc.Limit := 2;
        st := 7;
    ELSIF st = 7 THEN
        IF mdc.Shedding AND mdc.Q1 AND mdc.Q2 AND NOT mdc.Q3 AND NOT mdc.Q4 THEN
            mask := mask OR 64;
        END_IF;
        sched.Enable := TRUE;
        sched.MinuteOfDay := 700;   // 11:40
        sched.OnMin := 600;         // 10:00
        sched.OffMin := 720;        // 12:00
        scn.Enable := TRUE;
        scn.Scene := 2;
        scn.L2 := 77.0;
        st := 8;
    ELSIF st = 8 THEN
        IF sched.Q AND scn.Q AND scn.Level = 77.0 THEN
            mask := mask OR 128;
        END_IF;
        st := 9;
    END_IF;
    thr(); ramp(); lgt(); stair(); mdc(); sched(); scn();
END_PROGRAM
