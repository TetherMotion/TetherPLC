// t15_lethality.st — FB_AtcLethality: Tref=70, Z=10, F0Target=1.0 min,
// Temp=90 -> rate=10^2/60=1.667 F0/min, +0.1667/scan. InProduct from
// n=1: F0 hits 1.0 at n=6 (Done). ExitProduct pulse n=4 catches
// acc=0.667<target -> Divert. SensorOK drops n=20 -> Frozen latches;
// ExitProduct n=25 diverts on freeze. Reset n=30 clears acc but the
// same scan re-accumulates, so F0=0.167 at n=30.
// Sticky bits -> flags %QB0: expect 0x3f.
PROGRAM T15
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    le : FB_AtcLethality;
    n : DINT;
    sok, ex : BOOL;
END_VAR
    n := n + 1;
    sok := NOT (n >= 20 AND n <= 29);
    ex := (n = 4) OR (n = 25);
    le(Temp := 90.0, InProduct := TRUE, SensorOK := sok,
       ExitProduct := ex, Tref := 70.0, Z := 10.0,
       F0Target := 1.0, CycleTime := 0.1,
       Reset := (n = 30));
    IF n = 5 AND le.F0 > 0.5 AND le.F0 < 0.9 THEN ok0 := TRUE; END_IF;
    IF n >= 7 AND n < 15 AND le.Done THEN ok1 := TRUE; END_IF;
    IF n = 4 AND le.Divert THEN ok2 := TRUE; END_IF;
    IF n >= 21 AND n <= 24 AND le.Frozen THEN ok3 := TRUE; END_IF;
    IF n = 25 AND le.Divert THEN ok4 := TRUE; END_IF;
    IF n = 30 AND le.F0 < 0.3 THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
