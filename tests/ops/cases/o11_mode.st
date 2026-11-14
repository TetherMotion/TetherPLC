// o11_mode.st — FB_AtcMode arbitration, priority jog>maint>man>auto.
// n=1..2 auto; n=3..4 man+auto; n=5..6 jog+man+maint; n=7..8 maint;
// n>=9 none. Sticky: b0 auto, b1 man beats auto, b2 jog top,
// b3 maint alone, b4 off-transition Changed.
PROGRAM O11
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    md : FB_AtcMode;
    ra, rm, rj, rh : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    ra := n <= 4;
    rm := n >= 3 AND n <= 6;
    rj := n >= 5 AND n <= 6;
    rh := n >= 5 AND n <= 8;
    md(ReqAuto := ra, ReqMan := rm, ReqJog := rj, ReqMaint := rh);
    IF n = 2 AND md.Mode = 1 AND md.Auto THEN ok0 := TRUE; END_IF;
    IF n = 4 AND md.Mode = 2 AND md.Man AND NOT md.Auto
       THEN ok1 := TRUE; END_IF;
    IF n = 6 AND md.Mode = 3 AND md.Jog THEN ok2 := TRUE; END_IF;
    IF n = 8 AND md.Mode = 4 AND md.Maint THEN ok3 := TRUE; END_IF;
    IF n = 9 AND md.Mode = 0 AND md.Changed THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
