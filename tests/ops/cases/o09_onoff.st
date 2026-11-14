// o09_onoff.st — FB_AtcOnOff hysteresis. SP=20 Hyst=4 -> band 18..22.
// pv=10 on-side n<=3, pv=20 inside n=4..6 (holds), pv=23 off-side n>=7.
// Sticky: b0 on, b1 held in deadband, b2 off.
PROGRAM O09
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    oo : FB_AtcOnOff;
    pv : LREAL;
    n : DINT;
END_VAR
    n := n + 1;
    pv := 23.0;
    IF n <= 3 THEN pv := 10.0; END_IF;
    IF n >= 4 AND n <= 6 THEN pv := 20.0; END_IF;
    oo(PV := pv, SP := 20.0, Hyst := 4.0, Enable := TRUE);
    IF n = 3 AND oo.Out THEN ok0 := TRUE; END_IF;
    IF n = 5 AND oo.Out THEN ok1 := TRUE; END_IF;
    IF n = 8 AND NOT oo.Out THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
