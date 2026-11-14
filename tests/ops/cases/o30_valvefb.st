// o30_valvefb.st — FB_AtcValveFb: FbDelay=0.5, ct=0.1. Cmd FALSE
// n=11..26 while Fb stays TRUE till n=15 (stuck-open 0.5 s -> Fault
// ~n=15). Cmd=Fb FALSE n=16..26 but Fault latched; Reset n=26.
// Cmd=Fb TRUE again n=28+. flags %QB0: expect 0x3f.
PROGRAM O30
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    vf : FB_AtcValveFb;
    n : DINT;
    cmd, fb : BOOL;
END_VAR
    n := n + 1;
    cmd := (n <= 10) OR (n >= 27);
    fb := (n <= 15) OR (n >= 28);
    vf(Cmd := cmd, Fb := fb, FbDelay := 0.5, Enable := TRUE,
       CycleTime := 0.1, Reset := (n = 26));
    IF n >= 5 AND n <= 10 AND vf.OK THEN ok0 := TRUE; END_IF;
    IF n >= 12 AND n <= 14 AND NOT vf.OK AND NOT vf.Fault THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 16 AND n <= 20 AND vf.Fault THEN ok2 := TRUE; END_IF;
    IF n >= 21 AND n <= 25 AND NOT vf.OK THEN ok3 := TRUE; END_IF;
    IF n >= 29 AND n <= 33 AND vf.OK AND NOT vf.Fault THEN
        ok4 := TRUE;
    END_IF;
    IF n >= 14 AND n <= 15 AND vf.ET >= 0.3 THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
