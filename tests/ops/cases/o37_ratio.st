// o37_ratio.st — FB_AtcRatio: Ratio=0.5, FlowB=10 -> FlowA=6 Total=16;
// Enable off n=11..20 -> FlowA=0 Total=10; FlowB=0 n>=21 -> bias only;
// Bias=2 n>=31 -> FlowA=2. flags %QB0: expect 0x1f.
PROGRAM O37
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    ra : FB_AtcRatio;
    n : DINT;
    fb : LREAL;
    bs : LREAL;
    en : BOOL;
END_VAR
    n := n + 1;
    fb := 10.0;
    IF n >= 21 THEN fb := 0.0; END_IF;
    bs := 1.0;
    IF n >= 31 THEN bs := 2.0; END_IF;
    en := NOT (n >= 11 AND n <= 20);
    ra(FlowB := fb, Ratio := 0.5, Bias := bs, Enable := en);
    IF n >= 5 AND n <= 10 AND ra.FlowA = 6.0 AND ra.Total = 16.0
    THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 13 AND n <= 19 AND ra.FlowA = 0.0 AND ra.Total = 10.0
    THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 23 AND n <= 30 AND ra.FlowA = 1.0 AND ra.Total = 1.0
    THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 32 AND n <= 38 AND ra.FlowA = 2.0 AND ra.Total = 2.0
    THEN
        ok3 := TRUE;
    END_IF;
    IF n = 9 AND ra.FlowA = 6.0 THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
