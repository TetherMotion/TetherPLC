// o10_permissive.st — FB_AtcPermissive 8-input AND chain.
// all true n<=2; P3+P5 low n=3..4; only P8 low n=5..6; all n>=7.
// Sticky: b0 permit clear, b1 first-deny=3 mask=20, b2 first-deny=8
// mask=128.
PROGRAM O10
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    pm : FB_AtcPermissive;
    p3, p5, p8 : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    p3 := TRUE; p5 := TRUE; p8 := TRUE;
    IF n >= 3 AND n <= 4 THEN p3 := FALSE; p5 := FALSE; END_IF;
    IF n >= 5 AND n <= 6 THEN p8 := FALSE; END_IF;
    pm(P1 := TRUE, P2 := TRUE, P3 := p3, P4 := TRUE,
       P5 := p5, P6 := TRUE, P7 := TRUE, P8 := p8);
    IF n = 2 AND pm.Permit AND pm.DenyMask = 0 THEN ok0 := TRUE; END_IF;
    IF n = 4 AND NOT pm.Permit AND pm.FirstDeny = 3
       AND pm.DenyMask = 20 THEN ok1 := TRUE; END_IF;
    IF n = 6 AND pm.FirstDeny = 8 AND pm.DenyMask = 128
       THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
