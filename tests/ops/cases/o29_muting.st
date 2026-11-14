// o29_muting.st — FB_AtcMuting: SafeIn clear n=1..10 & 26..35;
// blocked n=11..25; MutingActive n=11..20 (legit mute) and n=26..30
// (misuse while clear -> MutingBad). flags %QB0: expect 0x1f.
PROGRAM O29
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    mu : FB_AtcMuting;
    n : DINT;
    si, ma : BOOL;
END_VAR
    n := n + 1;
    si := (n <= 10) OR (n >= 26);
    ma := (n >= 11 AND n <= 20) OR (n >= 26 AND n <= 30);
    mu(SafeIn := si, MutingActive := ma, Enable := TRUE,
       CycleTime := 0.1);
    IF n >= 5 AND n <= 10 AND mu.SafeRun AND NOT mu.MuteOut THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 13 AND n <= 19 AND mu.MuteOut AND mu.SafeRun
        AND NOT mu.MutingBad THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 22 AND n <= 25 AND NOT mu.SafeRun AND NOT mu.MuteOut THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 27 AND n <= 30 AND mu.MutingBad AND mu.SafeRun THEN
        ok3 := TRUE;
    END_IF;
    IF n >= 32 AND n <= 35 AND mu.SafeRun AND NOT mu.MuteOut
        AND NOT mu.MutingBad THEN
        ok4 := TRUE;
    END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
