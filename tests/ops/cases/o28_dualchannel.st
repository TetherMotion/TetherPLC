// o28_dualchannel.st — FB_AtcDualChannel: MismatchTime=0.5, ct=0.1.
// Channels disagree n=11..20: 0.4 s bounce filtered (no fault at
// n=13), 0.5 s at n=15 -> Fault latches; agrees again n=21 but Fault
// stays until Reset n=31. flags %QB0: expect 0x3f.
PROGRAM O28
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    dc : FB_AtcDualChannel;
    n : DINT;
    i1, i2 : BOOL;
END_VAR
    n := n + 1;
    i1 := TRUE;
    i2 := NOT (n >= 11 AND n <= 20);
    dc(In1 := i1, In2 := i2, MismatchTime := 0.5,
       Enable := TRUE, CycleTime := 0.1, Reset := (n = 31));
    IF n >= 5 AND n <= 10 AND dc.Agree AND dc.Safe THEN ok0 := TRUE;
    END_IF;
    IF n >= 12 AND n <= 14 AND NOT dc.Agree AND NOT dc.Fault THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 16 AND n <= 20 AND dc.Fault AND NOT dc.Safe THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 22 AND n <= 30 AND dc.Agree AND dc.Fault AND NOT dc.Safe
    THEN
        ok3 := TRUE;
    END_IF;
    IF n >= 32 AND n <= 38 AND dc.Safe AND NOT dc.Fault THEN
        ok4 := TRUE;
    END_IF;
    IF n >= 13 AND n <= 14 AND dc.ET > 0.3 THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
