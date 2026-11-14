// o25_toollife.st — FB_AtcToolLife: LifeLimit=20, WarnPct=80,
// Cycle pulses every even n, Enable while n<45. Cycles hit 16 (80%)
// at n=32 -> Warn; 20 at n=40 -> Due. Reset n=45 clears.
// flags %QB0: expect 0x3f.
PROGRAM O25
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    tl : FB_AtcToolLife;
    n : DINT;
    cyc : BOOL;
END_VAR
    n := n + 1;
    cyc := (n MOD 2) = 0;
    tl(Cycle := cyc, Enable := (n < 45), LifeLimit := 20,
       WarnPct := 80.0, CycleTime := 0.1, Reset := (n = 45));
    IF n >= 10 AND tl.Cycles >= 5 THEN ok0 := TRUE; END_IF;
    IF n >= 20 AND tl.PctUsed >= 40.0 AND tl.PctUsed < 80.0 THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 33 AND n <= 39 AND tl.Warn AND NOT tl.Due THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 41 AND n <= 44 AND tl.Due THEN ok3 := TRUE; END_IF;
    IF n >= 46 AND tl.Cycles = 0 AND tl.Hours < 0.02 THEN ok4 := TRUE;
    END_IF;
    IF n >= 20 AND n <= 30 AND tl.Hours > 0.0 THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
