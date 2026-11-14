// o35_delay.st — FB_AtcDelay: Delay=0.5 s, ct=0.1 -> 5 samples.
// In=100 pulse at n=3 only; In=50 from n=20. Pulse must appear at
// Out n=8 (5 scans later), step at n=25. Filled once ring >= 5.
// flags %QB0: expect 0x3f.
PROGRAM O35
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    dl : FB_AtcDelay;
    n : DINT;
    vin : LREAL;
END_VAR
    n := n + 1;
    vin := 0.0;
    IF n = 3 THEN vin := 100.0; END_IF;
    IF n >= 20 THEN vin := 50.0; END_IF;
    dl(In := vin, Delay := 0.5, CycleTime := 0.1, Enable := TRUE,
       Reset := FALSE);
    IF n >= 4 AND n <= 7 AND dl.Out = 0.0 THEN ok0 := TRUE; END_IF;
    IF n = 8 AND dl.Out = 100.0 THEN ok1 := TRUE; END_IF;
    IF n = 9 AND dl.Out = 0.0 THEN ok2 := TRUE; END_IF;
    IF n >= 23 AND n <= 24 AND dl.Out = 0.0 THEN ok3 := TRUE; END_IF;
    IF n >= 26 AND n <= 30 AND dl.Out = 50.0 THEN ok4 := TRUE; END_IF;
    IF n >= 6 AND n <= 10 AND dl.Filled THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
