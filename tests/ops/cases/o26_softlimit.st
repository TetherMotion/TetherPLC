// o26_softlimit.st — FB_AtcSoftLimit: [0,100], Homed from n=11,
// Enable always, Reset n=31. Pos=50 (in), 120 (over n=21..25),
// -10 (under n=26..30), 50 again. PosValid needs Homed; Fault
// latches on first violation, Reset clears.
// flags %QB0: expect 0x3f.
PROGRAM O26
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    sl : FB_AtcSoftLimit;
    n : DINT;
    pos : LREAL;
    homed : BOOL;
END_VAR
    n := n + 1;
    pos := 50.0;
    IF n >= 21 AND n <= 25 THEN pos := 120.0; END_IF;
    IF n >= 26 AND n <= 30 THEN pos := -10.0; END_IF;
    homed := (n >= 11);
    sl(Pos := pos, MinPos := 0.0, MaxPos := 100.0, Homed := homed,
       Enable := TRUE, Reset := (n = 31));
    IF n >= 5 AND n <= 10 AND NOT sl.PosValid AND sl.NeedHome THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 15 AND n <= 20 AND sl.InWindow AND NOT sl.Fault THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 22 AND n <= 25 AND sl.OverPos AND sl.Fault THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 27 AND n <= 30 AND sl.UnderPos AND sl.Fault THEN
        ok3 := TRUE;
    END_IF;
    IF n >= 32 AND n <= 35 AND NOT sl.Fault AND sl.InWindow THEN
        ok4 := TRUE;
    END_IF;
    IF n >= 12 AND n <= 20 AND sl.PosValid AND NOT sl.NeedHome THEN
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
