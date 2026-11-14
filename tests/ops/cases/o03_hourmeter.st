// o03_hourmeter.st — FB_AtcHourMeter: ct=3600 s/scan so each scan is
// one hour. Run scans 1-3 -> Hours=3.0, Preset=2.5 -> Alarm + Pct=120.
// Run off at n=4 (hours hold), Reset at n=6 -> zeroed.
// Sticky: b0 hours accumulate, b1 alarm+pct, b2 reset clears.
PROGRAM O03
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    hm : FB_AtcHourMeter;
    run, rst : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    run := n <= 3;
    IF n = 6 THEN rst := TRUE; END_IF;
    hm(Run := run, CycleTime := 3600.0, Preset := 2.5, Reset := rst);
    IF n = 3 AND hm.Hours > 2.9 AND hm.Hours < 3.1 THEN ok0 := TRUE; END_IF;
    IF n = 4 AND hm.Alarm AND hm.Pct > 110.0 THEN ok1 := TRUE; END_IF;
    IF n = 7 AND hm.Hours = 0.0 AND NOT hm.Alarm THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
