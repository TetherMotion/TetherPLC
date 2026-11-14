// o19_twohand.st — FB_AtcTwoHand anti-tie-down, ct=0.1 MaxGap=0.5.
// Btn1 n=2..5, Btn2 n=4..7: valid press -> Out while both held.
// Btn1 released n=6 -> Out drops. n=8+ both released -> disarm.
// n=9 Btn1 held alone -> gap expires n=14; Btn2 tap n=15 rejected.
// Sticky: b0 permit, b1 release-drop, b2 disarmed, b3 late-tap reject.
PROGRAM O19
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3 : BOOL;
    th : FB_AtcTwoHand;
    b1, b2 : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    b1 := (n >= 2 AND n <= 5) OR (n >= 9 AND n <= 16);
    b2 := (n >= 4 AND n <= 7) OR n = 15;
    th(Btn1 := b1, Btn2 := b2, MaxGap := 0.5,
       CycleTime := 0.1, Reset := FALSE);
    IF n = 5 AND th.Out AND th.Armed THEN ok0 := TRUE; END_IF;
    IF n = 6 AND NOT th.Out THEN ok1 := TRUE; END_IF;
    IF n = 14 AND NOT th.Armed THEN ok2 := TRUE; END_IF;
    IF n = 16 AND NOT th.Out AND NOT th.Armed THEN ok3 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
END_PROGRAM
