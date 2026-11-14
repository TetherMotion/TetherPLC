// o33_counter.st — FB_AtcCounter: Preset=5, ct irrelevant. Up pulses
// n=13,17,21,25,29 (Count->5, AtPreset); Down n=38,44,50 (->2);
// Reset n=55. flags %QB0: expect 0x3f.
PROGRAM O33
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    ct : FB_AtcCounter;
    n : DINT;
    up, dn : BOOL;
END_VAR
    n := n + 1;
    up := (n >= 11 AND n <= 30) AND (n MOD 4) = 1;
    dn := (n >= 35 AND n <= 52) AND (n MOD 6) = 2;
    ct(Up := up, Down := dn, Preset := 5, Enable := TRUE,
       Reset := (n = 55));
    IF n >= 8 AND n <= 10 AND ct.Count = 0 AND ct.Zero THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 22 AND n <= 24 AND ct.Count = 3 THEN ok1 := TRUE; END_IF;
    IF n >= 30 AND n <= 33 AND ct.AtPreset AND ct.Count = 5 THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 45 AND n <= 49 AND ct.Count = 3 THEN ok3 := TRUE; END_IF;
    IF n >= 51 AND n <= 54 AND ct.Count = 2 AND NOT ct.AtPreset THEN
        ok4 := TRUE;
    END_IF;
    IF n >= 57 AND n <= 60 AND ct.Count = 0 AND ct.Zero THEN
        ok5 := TRUE;
    END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
