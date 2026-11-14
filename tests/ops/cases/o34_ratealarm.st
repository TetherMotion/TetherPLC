// o34_ratealarm.st — FB_AtcRateAlarm: MaxRate=5, OnDelay=0.5,
// OffDelay=0.3, ct=0.1. PV ramps +2/scan n=11..20 (rate 20 -> Alarm
// ~n=15), holds 70 n=21..30 (clears ~n=23), 72 n=31..35, ramps
// -10/scan n=36..45 (rate -100 -> Alarm ~n=40). flags %QB0: 0x3f.
PROGRAM O34
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4, ok5 : BOOL;
    ra : FB_AtcRateAlarm;
    n : DINT;
    pv : LREAL;
END_VAR
    n := n + 1;
    pv := 50.0;
    IF n >= 11 AND n <= 20 THEN
        pv := 50.0 + 2.0 * DINT_TO_LREAL(n - 10);
    END_IF;
    IF n >= 21 AND n <= 30 THEN pv := 70.0; END_IF;
    IF n >= 31 AND n <= 35 THEN pv := 72.0; END_IF;
    IF n >= 36 AND n <= 45 THEN
        pv := 72.0 - 10.0 * DINT_TO_LREAL(n - 35);
    END_IF;
    IF n >= 46 THEN pv := -28.0; END_IF;
    ra(PV := pv, MaxRate := 5.0, OnDelay := 0.5, OffDelay := 0.3,
       Enable := TRUE, CycleTime := 0.1, Reset := FALSE);
    IF n >= 5 AND n <= 9 AND ra.RateNow = 0.0 AND NOT ra.Alarm THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 12 AND n <= 14 AND ra.RateNow > 15.0 AND NOT ra.Alarm THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 17 AND n <= 20 AND ra.Alarm AND ra.RateNow > 15.0 THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 25 AND n <= 29 AND NOT ra.Alarm THEN ok3 := TRUE; END_IF;
    IF n >= 41 AND n <= 44 AND ra.Alarm AND ra.RateNow < -50.0 THEN
        ok4 := TRUE;
    END_IF;
    IF n = 22 AND ra.Alarm THEN ok5 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
    IF ok5 THEN flags := flags + 32; END_IF;
END_PROGRAM
