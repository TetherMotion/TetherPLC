// t16_heatch.st — FB_AtcHeaterChannel: auto PWM phase, interlock
// inhibit, manual select, and duty clamp. n drives the phases:
//   1-10   auto MV=50%, Period=1s, CycleTime=0.1 -> Q toggles
//   11-20  InterlockOK=FALSE -> Q=FALSE, InterlockBad=TRUE
//   21-30  Auto=FALSE, ManualMV=25 -> DutyApplied=25
//   31-35  MV=150 -> DutyApplied clamped to 100
//   36-45  Execute=FALSE -> Q=FALSE, InterlockBad=FALSE
// Sticky bits -> flags %QB0: expect 0x1f.
PROGRAM T16
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    hc : FB_AtcHeaterChannel;
    n : DINT;
    sawQ : BOOL;
END_VAR
    n := n + 1;
    hc.Execute := n <= 35;
    hc.Enable := TRUE;
    hc.Auto := NOT (n >= 21 AND n <= 30);
    hc.MV := 50.0;
    IF n >= 31 THEN hc.MV := 150.0; END_IF;
    hc.ManualMV := 25.0;
    hc.InterlockOK := NOT (n >= 11 AND n <= 20);
    hc.Period := 1.0;
    hc.MinOn := 0.0;
    hc.CycleTime := 0.1;
    hc();
    IF hc.Q THEN sawQ := TRUE; END_IF;
    ok0 := sawQ;
    IF (n = 15) AND hc.InterlockBad AND NOT hc.Q THEN
        ok1 := TRUE;
    END_IF;
    IF (n = 25) AND (hc.DutyApplied = 25.0) THEN
        ok2 := TRUE;
    END_IF;
    IF (n = 33) AND (hc.DutyApplied = 100.0) THEN
        ok3 := TRUE;
    END_IF;
    IF (n = 40) AND NOT hc.Q AND NOT hc.InterlockBad THEN
        ok4 := TRUE;
    END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
