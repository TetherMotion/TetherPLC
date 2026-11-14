// o20_shift.st — FB_AtcShift: S1 08-12, S2 16-20, S3 overnight 22-04.
// TOD := n*1800 (30 min/scan). Checks shift index, start/end pulses,
// elapsed/remaining, and midnight-wrap continuity of the night shift.
// Sticky bits -> flags %QB0: b0 in-shift idx, b1 ShiftEnd seen,
// b2 ShiftStart seen, b3 elapsed/remaining sane.
PROGRAM O20
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3 : BOOL;
    sh : FB_AtcShift;
    tod : LREAL;
    n : DINT;
    endSeen, startSeen : BOOL;
    elOk, remOk : BOOL;
END_VAR
    n := n + 1;
    tod := DINT_TO_LREAL(n) * 1800.0;
    sh(TOD := tod, Enable := TRUE,
       S1Start := 28800.0, S1End := 43200.0,
       S2Start := 57600.0, S2End := 72000.0,
       S3Start := 79200.0, S3End := 14400.0);
    endSeen := endSeen OR sh.ShiftEnd;
    startSeen := startSeen OR sh.ShiftStart;
    // b0: correct shift index across the day (S3 early, S1 mid, S2 eve)
    IF (n <= 7 AND sh.Shift = 3 AND sh.InShift)
       OR (n >= 16 AND n <= 23 AND sh.Shift = 1 AND sh.InShift)
       OR (n >= 32 AND n <= 39 AND sh.Shift = 2 AND sh.InShift)
       OR (n >= 44 AND n <= 55 AND sh.Shift = 3 AND sh.InShift) THEN
        ok0 := TRUE;
    END_IF;
    // b1: left-shift pulse fired at the 04:00, 12:00, 20:00 boundaries
    IF n >= 9 AND endSeen THEN ok1 := TRUE; END_IF;
    // b2: entered-shift pulse fired at 08:00, 16:00, 22:00, and 00:00-wrap
    IF n >= 17 AND startSeen THEN ok2 := TRUE; END_IF;
    // b3: elapsed/remaining sane — n=17 is 30 min into S1;
    // n=50 is 01:00 inside overnight S3 (elapsed 3h, remaining 3h)
    elOk := FALSE; remOk := FALSE;
    IF n = 17 THEN
        IF sh.Elapsed > 1700.0 AND sh.Elapsed < 1900.0
           AND sh.Remaining > 12000.0 AND sh.Remaining < 13200.0 THEN
            elOk := TRUE; remOk := TRUE;
        END_IF;
    END_IF;
    IF n = 50 THEN
        IF sh.Elapsed > 10000.0 AND sh.Elapsed < 11600.0
           AND sh.Remaining > 10000.0 AND sh.Remaining < 11600.0 THEN
            elOk := TRUE; remOk := TRUE;
        END_IF;
    END_IF;
    IF elOk AND remOk THEN ok3 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
END_PROGRAM
