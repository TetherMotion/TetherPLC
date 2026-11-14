// o14_blinker.st — FB_AtcBlinker Rate=2Hz Duty=0.5 ct=0.1 -> 5-scan
// period, Q on ~first half. frac sequence 0.2,0.4,0.6,0.8,wrap.
// Sample away from the 0.5 boundary for float safety.
// Sticky: b0 on, b1 off, b2 wrapped-on, b3 off again.
PROGRAM O14
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3 : BOOL;
    bl : FB_AtcBlinker;
    n : DINT;
END_VAR
    n := n + 1;
    bl(En := TRUE, Rate := 2.0, Duty := 0.5, CycleTime := 0.1);
    IF n = 1 AND bl.Q THEN ok0 := TRUE; END_IF;
    IF n = 3 AND NOT bl.Q THEN ok1 := TRUE; END_IF;
    IF n = 6 AND bl.Q THEN ok2 := TRUE; END_IF;
    IF n = 8 AND NOT bl.Q THEN ok3 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
END_PROGRAM
