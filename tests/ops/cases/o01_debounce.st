// o01_debounce.st — FB_AtcDebounce: OnTime=0.3 s sets Q, OffTime=0.2 s
// resets it, ct=0.1 s/scan. D high at n=1 -> Q at n=3 (0.3 s).
// D low at n=6 -> Q clears at n=7 (0.2 s). ET counts since change.
// Sticky bits -> flags %QB0: b0 Q set, b1 Q cleared, b2 ET sane.
PROGRAM O01
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    deb : FB_AtcDebounce;
    d : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    IF n = 1 THEN d := TRUE; END_IF;
    IF n = 6 THEN d := FALSE; END_IF;
    deb(D := d, OnTime := 0.3, OffTime := 0.2, CycleTime := 0.1);
    IF n >= 3 AND n < 6 AND deb.Q THEN ok0 := TRUE; END_IF;
    IF n >= 7 AND NOT deb.Q AND n < 30 THEN ok1 := TRUE; END_IF;
    IF n >= 10 AND n < 30 AND deb.ET > 0.05 THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
