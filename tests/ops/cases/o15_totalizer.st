// o15_totalizer.st — FB_AtcTotalizer Rate=2 K=3 ct=0.1 -> +0.6/scan.
// Reset at n=4 zeroes; accumulation resumes.
// Sticky: b0 accumulates, b1 reset, b2 resumes.
PROGRAM O15
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    tz : FB_AtcTotalizer;
    rst : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    rst := n = 4;
    tz(Rate := 2.0, K := 3.0, CycleTime := 0.1, Reset := rst);
    IF n = 3 AND tz.Total > 1.7 AND tz.Total < 1.9
       THEN ok0 := TRUE; END_IF;
    IF n = 4 AND tz.Total = 0.0 THEN ok1 := TRUE; END_IF;
    IF n = 6 AND tz.Total > 1.1 AND tz.Total < 1.3
       THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
