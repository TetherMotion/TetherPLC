// o22_zonebalance.st — FB_AtcZoneBalance: Gain=2, TrimMax=5, RateMax=10
// (1 degC/scan at ct=0.1). MasterErr=3 at n=1 -> want=6 clamps to 5,
// rate-limited ramp reaches 5 by n=5, Sat set. MasterErr=0 at n=10 ->
// ramps down 1/scan, hits 0 at n=14. Disable n=20 -> bumpless reset;
// re-enable n=25 starts at 0 again. Per-scan |dTrim| <= 1.01 always.
// Sticky bits -> flags %QB0: expect 0x1f.
PROGRAM O22
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    zb : FB_AtcZoneBalance;
    err : LREAL;
    en : BOOL;
    n : DINT;
    prev : LREAL;
    dmax : LREAL;
    first : BOOL;
END_VAR
    n := n + 1;
    first := (n = 1);
    IF n = 1 THEN err := 3.0; END_IF;
    IF n = 10 THEN err := 0.0; END_IF;
    en := NOT (n >= 20 AND n <= 24);
    zb(MasterErr := err, Enable := en, Gain := 2.0,
       TrimMax := 5.0, RateMax := 10.0, CycleTime := 0.1);
    // per-scan rate-limit invariant
    IF NOT first THEN
        IF zb.TrimOut - prev > dmax THEN
            dmax := zb.TrimOut - prev;
        END_IF;
        IF prev - zb.TrimOut > dmax THEN
            dmax := prev - zb.TrimOut;
        END_IF;
    END_IF;
    prev := zb.TrimOut;
    IF n >= 6 AND n <= 9 AND zb.TrimOut > 4.9 AND zb.Sat THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 6 AND n <= 9 AND zb.Sat THEN ok1 := TRUE; END_IF;
    IF n = 11 AND zb.TrimOut > 2.9 AND zb.TrimOut < 3.1 THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 21 AND n <= 24 AND zb.TrimOut = 0.0 THEN ok3 := TRUE; END_IF;
    IF n = 30 AND zb.TrimOut < 0.2 THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 AND dmax <= 1.01 THEN flags := flags + 16; END_IF;
END_PROGRAM
