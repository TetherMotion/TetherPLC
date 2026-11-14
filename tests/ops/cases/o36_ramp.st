// o36_ramp.st — FB_AtcRamp: Target=10 always, MaxVel=5, MaxAcc=10,
// ct=0.1. Trapezoid: accel ~5 scans to v=5, cruise, brake ~1.25 u,
// lands Pos=10 Vel=0 Done by ~n=26. flags %QB0: expect 0x3f.
PROGRAM O36
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    rp : FB_AtcRamp;
    n : DINT;
END_VAR
    n := n + 1;
    rp(Target := 10.0, MaxVel := 5.0, MaxAcc := 10.0,
       Enable := TRUE, CycleTime := 0.1, Reset := FALSE);
    IF n >= 2 AND n <= 4 AND rp.Moving AND rp.Vel > 0.0
        AND rp.Vel < 4.9 THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 10 AND n <= 18 AND rp.Vel = 5.0 AND rp.Moving THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 20 AND n <= 26 AND rp.Vel < 5.0 AND rp.Vel > 0.0
        AND rp.Moving THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 28 AND n <= 40 AND rp.Done AND rp.Pos = 10.0
        AND rp.Vel = 0.0 THEN
        ok3 := TRUE;
    END_IF;
    IF n >= 15 AND n <= 25 AND rp.Pos > 3.0 AND rp.Pos < 10.0 THEN
        ok4 := TRUE;
    END_IF;
    IF n >= 5 AND n <= 9 AND rp.Vel = 5.0 THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
