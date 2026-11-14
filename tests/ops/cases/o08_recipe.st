// o08_recipe.st — FB_AtcRecipe: 16x16 LREAL store.
//   n=1 write (set2,param3)=42.5 ; n=2 read -> 42.5 Valid
//   n=3 write (set5,param0)=7.0  ; n=4 read -> 7.0 (same instance,
//        different addressing — proves set/param independence)
//   n=5 Clear -> n=6 read -> 0.0
//   n=7 invalid read (set99) -> Valid FALSE Val 0
// Sticky: b0 roundtrip, b1 set independence, b2 clear, b3 invalid.
PROGRAM O08
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3 : BOOL;
    rc : FB_AtcRecipe;
    wrSet, wrPar, rdSet, rdPar : DINT;
    wrVal : LREAL;
    n : DINT;
END_VAR
    n := n + 1;
    // default: read back set2 param3; write set2 param3 = 42.5
    rdSet := 2; rdPar := 3;
    wrSet := 2; wrPar := 3; wrVal := 42.5;
    IF n = 1 OR n = 3 THEN
        // n=3 retargets the write to set5 param0 = 7.0
        IF n = 3 THEN
            wrSet := 5; wrPar := 0; wrVal := 7.0;
        END_IF;
    END_IF;
    IF n = 4 OR n = 6 THEN rdSet := 5; rdPar := 0; END_IF;
    IF n = 7 THEN rdSet := 99; rdPar := 0; END_IF;
    rc(WrSet := wrSet, WrParam := wrPar, WrVal := wrVal,
       WrStrobe := (n = 1) OR (n = 3),
       RdSet := rdSet, RdParam := rdPar, Clear := n = 5);
    IF n = 2 AND rc.Val > 42.4 AND rc.Val < 42.6 AND rc.Valid
       THEN ok0 := TRUE; END_IF;
    IF n = 4 AND rc.Val > 6.9 AND rc.Val < 7.1 AND rc.Valid
       THEN ok1 := TRUE; END_IF;
    IF n = 6 AND rc.Val = 0.0 AND rc.Valid THEN ok2 := TRUE; END_IF;
    IF n = 7 AND NOT rc.Valid AND rc.Val = 0.0 THEN ok3 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
END_PROGRAM
