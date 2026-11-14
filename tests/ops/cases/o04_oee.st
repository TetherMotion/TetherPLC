// o04_oee.st — FB_AtcOee: ct=3600 -> 1 h/scan. Planned+Running all
// scans; IdealCycle=10 s. GoodPart pulse n=1, TotalPart pulses n=1,2.
// After n=2: A=100, P=10/(2*3600)*100=0.139, Q=50, OEE~0.069.
// n=8 Reset -> all counters zero, A back to 0 (no plan time).
// Sticky: b0 availability 100, b1 quality 50, b2 perf in band,
// b3 oee in band, b4 reset.
PROGRAM O04
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    oee : FB_AtcOee;
    good, tot, rst : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    good := n = 1;
    tot := n <= 2;
    rst := n = 8;
    oee(Planned := TRUE, Running := TRUE, IdealCycle := 10.0,
        GoodPart := good, TotalPart := tot, Reset := rst,
        CycleTime := 3600.0);
    IF n = 2 AND oee.Availability > 99.9 THEN ok0 := TRUE; END_IF;
    IF n = 2 AND oee.Quality > 49.9 AND oee.Quality < 50.1
       THEN ok1 := TRUE; END_IF;
    IF n = 2 AND oee.Performance > 0.1 AND oee.Performance < 0.2
       THEN ok2 := TRUE; END_IF;
    IF n = 2 AND oee.OEE > 0.05 AND oee.OEE < 0.09 THEN ok3 := TRUE; END_IF;
    IF n = 9 AND oee.GoodCount = 0 AND oee.TotalCount = 0
       AND oee.PlannedH < 1.5 THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
