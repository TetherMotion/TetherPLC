// o39_window.st — FB_AtcWindow: TOD=n. Window 8..20 n<31 (active
// n=8..19); overnight 40..10 n>=31 (active n>=40 or n<10; dead zone
// n=10..39). In=TRUE always. flags %QB0: expect 0x3f.
PROGRAM O39
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    wn : FB_AtcWindow;
    n : DINT;
    s, e : LREAL;
END_VAR
    n := n + 1;
    s := 8.0; e := 20.0;
    IF n >= 31 THEN s := 40.0; e := 10.0; END_IF;
    wn(In := TRUE, TOD := DINT_TO_LREAL(n), StartT := s, EndT := e);
    IF n >= 3 AND n <= 7 AND NOT wn.Active AND NOT wn.Q THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 10 AND n <= 15 AND wn.Active AND wn.Q THEN ok1 := TRUE;
    END_IF;
    IF n >= 22 AND n <= 28 AND NOT wn.Active THEN ok2 := TRUE; END_IF;
    IF n >= 42 AND n <= 50 AND wn.Active THEN ok3 := TRUE; END_IF;
    IF n >= 60 AND n <= 65 AND wn.Active THEN ok4 := TRUE; END_IF;
    IF n >= 32 AND n <= 38 AND NOT wn.Active THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
