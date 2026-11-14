// o16_avg.st — FB_AtcAvg N=3 window over av=n*10.
// n=3 avg 20; n=4 window {40,20,30} avg 30; n=5 {40,50,30} avg 40.
// Sticky: b0/b1/b2 window means.
PROGRAM O16
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    av2 : FB_AtcAvg;
    av : LREAL;
    n : DINT;
END_VAR
    n := n + 1;
    av := n * 10.0;
    av2(In := av, N := 3);
    IF n = 3 AND av2.Avg > 19.9 AND av2.Avg < 20.1
       AND av2.Count = 3 THEN ok0 := TRUE; END_IF;
    IF n = 4 AND av2.Avg > 29.9 AND av2.Avg < 30.1
       THEN ok1 := TRUE; END_IF;
    IF n = 5 AND av2.Avg > 39.9 AND av2.Avg < 40.1
       THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
