// o32_loto.st — FB_AtcLoto: L2+L5 locked n=11..25 (L2 releases n=21),
// Enable FALSE n=31..35. flags %QB0: expect 0x1f.
PROGRAM O32
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    lo : FB_AtcLoto;
    n : DINT;
    l2, l5 : BOOL;
END_VAR
    n := n + 1;
    l2 := (n >= 11 AND n <= 20);
    l5 := (n >= 11 AND n <= 25);
    lo(L1 := FALSE, L2 := l2, L3 := FALSE, L4 := FALSE,
       L5 := l5, L6 := FALSE, L7 := FALSE, L8 := FALSE,
       Enable := (n < 31 OR n > 35));
    IF n >= 5 AND n <= 10 AND lo.Permit AND lo.LockedCount = 0 THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 13 AND n <= 19 AND NOT lo.Permit AND lo.LockedCount = 2
        AND lo.FirstLock = 2 THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 22 AND n <= 25 AND NOT lo.Permit AND lo.LockedCount = 1
        AND lo.FirstLock = 5 THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 27 AND n <= 30 AND lo.Permit THEN ok3 := TRUE; END_IF;
    IF n >= 32 AND n <= 35 AND NOT lo.Permit THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
