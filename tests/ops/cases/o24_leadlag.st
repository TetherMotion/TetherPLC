// o24_leadlag.st — FB_AtcLeadLag: 3 units, StageDelay=0.5 s, ct=0.1.
// Stage changes step ONE unit per StageDelay. dem=2 n=11..30:
// onT 0->1 n=11 sel={1}; 1->2 n=16 re-picks least hours -> {2,3}
// (stage1 accrued 0.5 s, 2&3 zero). dem=0 n=31..50: onT 2->1 n=31
// sel={1} (least hrs), 1->0 n=36. dem=1 n=51..60: onT=1 sel={1}
// (1.0 s vs 1.5 s on 2&3). flags %QB0: expect 0x3f.
PROGRAM O24
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    ll : FB_AtcLeadLag;
    n : DINT;
    dem : LREAL;
END_VAR
    n := n + 1;
    dem := 0.0;
    IF n >= 11 AND n <= 30 THEN dem := 2.0; END_IF;
    IF n >= 51 THEN dem := 1.0; END_IF;
    ll(Demand := dem, NumUnits := 3, StageDelay := 0.5,
       Enable := TRUE, CycleTime := 0.1, Reset := FALSE);
    IF n >= 5 AND n <= 10 AND ll.NumOn = 0 THEN ok0 := TRUE; END_IF;
    IF n >= 12 AND n <= 14 AND ll.NumOn = 1 AND ll.Stage[1]
        AND NOT ll.Stage[2] AND NOT ll.Stage[3] THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 25 AND n <= 30 AND ll.NumOn = 2 AND ll.Stage[2]
        AND ll.Stage[3] AND NOT ll.Stage[1] AND ll.LeadIdx = 2 THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 33 AND n <= 35 AND ll.NumOn = 1 THEN ok3 := TRUE; END_IF;
    IF n >= 40 AND n <= 50 AND ll.NumOn = 0 THEN ok4 := TRUE; END_IF;
    IF n >= 55 AND n <= 60 AND ll.NumOn = 1 AND ll.Stage[1]
        AND ll.LeadIdx = 1 AND NOT ll.Stage[2] AND NOT ll.Stage[3]
    THEN
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
