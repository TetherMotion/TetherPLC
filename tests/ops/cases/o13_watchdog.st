// o13_watchdog.st — FB_AtcWatchdog heartbeat, ct=0.1 Timeout=0.25.
// Kick pulses n=1,3,5,7 then stops: ET grows past 0.25 at n=10.
// Sticky: b0 ok between kicks, b1 ok near timeout, b2 err latched.
PROGRAM O13
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    wd : FB_AtcWatchdog;
    kick : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    kick := n = 1 OR n = 3 OR n = 5 OR n = 7;
    wd(Kick := kick, Timeout := 0.25, CycleTime := 0.1,
       Reset := FALSE);
    IF n = 6 AND wd.OK AND wd.ET > 0.05 AND wd.ET < 0.15
       THEN ok0 := TRUE; END_IF;
    IF n = 9 AND wd.OK AND wd.ET > 0.15 THEN ok1 := TRUE; END_IF;
    IF n = 11 AND wd.Err AND NOT wd.OK THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
