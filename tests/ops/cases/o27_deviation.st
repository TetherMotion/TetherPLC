// o27_deviation.st — FB_AtcDeviation: SP=50, Band=5, OnDelay=0.5,
// OffDelay=0.3, ct=0.1. PV=58 n=11..20 (dev 8): Alarm at ~n=15.
// PV=52 n=21..30 (in band): clears ~n=23. PV=54.5 n=31..35 in band.
// PV=60 n=36..45 (dev 10): Alarm again ~n=40.
// flags %QB0: expect 0x3f.
PROGRAM O27
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    dv : FB_AtcDeviation;
    n : DINT;
    pv : LREAL;
END_VAR
    n := n + 1;
    pv := 50.0;
    IF n >= 11 AND n <= 20 THEN pv := 58.0; END_IF;
    IF n >= 21 AND n <= 30 THEN pv := 52.0; END_IF;
    IF n >= 31 AND n <= 35 THEN pv := 54.5; END_IF;
    IF n >= 36 THEN pv := 60.0; END_IF;
    dv(PV := pv, SP := 50.0, Band := 5.0, OnDelay := 0.5,
       OffDelay := 0.3, Enable := TRUE, CycleTime := 0.1,
       Reset := FALSE);
    IF n >= 12 AND n <= 14 AND dv.Dev AND NOT dv.Alarm THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 17 AND n <= 20 AND dv.Alarm THEN ok1 := TRUE; END_IF;
    IF n = 22 AND dv.Alarm THEN ok2 := TRUE; END_IF;
    IF n >= 25 AND n <= 29 AND NOT dv.Alarm THEN ok3 := TRUE; END_IF;
    IF n >= 41 AND n <= 44 AND dv.Alarm THEN ok4 := TRUE; END_IF;
    IF n >= 17 AND n <= 19 AND dv.ET > 0.2 THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
