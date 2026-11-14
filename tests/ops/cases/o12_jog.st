// o12_jog.st — FB_AtcJog deadman + anti-run timeout, ct=0.1 MaxOn=0.3.
// En+Dir=+1 n=2..5: ET hits 0.3 at n=4 -> Out drops, TimedOut latches
// while En held; n=6 En released -> clears.
// Sticky: b0 jogging, b1 timed out, b2 released-clear.
PROGRAM O12
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    jg : FB_AtcJog;
    en : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    en := n >= 2 AND n <= 5;
    jg(DirIn := 1, En := en, MaxOn := 0.3, CycleTime := 0.1);
    IF n = 3 AND jg.Out AND jg.Dir = 1 AND NOT jg.TimedOut
       THEN ok0 := TRUE; END_IF;
    IF n = 5 AND NOT jg.Out AND jg.TimedOut THEN ok1 := TRUE; END_IF;
    IF n = 7 AND NOT jg.TimedOut AND NOT jg.Out THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
