// o17_demand.st — FB_AtcDemand Window=0.25 ct=0.1 -> n=2 samples.
// pw=n*10. n=3 window {30,20} -> 25; n=5 {50,40} -> 45;
// n=6 Reset; n=7 single sample 70 -> 70.
// Sticky: b0/b1 windowed mean, b2 post-reset restart.
PROGRAM O17
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    dm : FB_AtcDemand;
    rst : BOOL;
    pw : LREAL;
    n : DINT;
END_VAR
    n := n + 1;
    pw := n * 10.0;
    rst := n = 6;
    dm(Power := pw, Window := 0.25, CycleTime := 0.1,
       Reset := rst);
    IF n = 3 AND dm.Demand > 24.9 AND dm.Demand < 25.1
       THEN ok0 := TRUE; END_IF;
    IF n = 5 AND dm.Demand > 44.9 AND dm.Demand < 45.1
       THEN ok1 := TRUE; END_IF;
    IF n = 7 AND dm.Demand > 69.9 AND dm.Demand < 70.1
       AND dm.Samples = 1 THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
