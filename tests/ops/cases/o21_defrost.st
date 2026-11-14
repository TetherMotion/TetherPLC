// o21_defrost.st — FB_AtcDefrost: DefrostEvery=2 s, DefrostTime=0.5 s,
// MinOff=0.5 s, ct=0.1. Cool n=1..19, defrost ~n=20..23, off-delay
// ~n=24..30 (float offT needs an extra scan), cool resumes n=31.
// Door ajar n=30..37: qualified door-time starts n=32 (fridgeOn only
// true from n=31), doorT reaches 0.5 at n=36 -> DoorAlarm latches.
// Ack n=39 clears it. Warm n=42..44 latches TempAlarm at n=44.
// Sticky bits -> flags %QB0: expect 0x1f.
PROGRAM O21
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok3b, ok4 : BOOL;
    df : FB_AtcDefrost;
    door, warm, ack : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    IF n >= 30 AND n <= 37 THEN door := TRUE; ELSE door := FALSE; END_IF;
    IF n >= 42 AND n <= 45 THEN warm := TRUE; ELSE warm := FALSE; END_IF;
    ack := (n = 39);
    df(Enable := TRUE, DoorOpen := door, Warm := warm, Ack := ack,
       DefrostEvery := 2.0, DefrostTime := 0.5, MinOff := 0.5,
       DoorAlarmTime := 0.5, WarmTime := 0.3, CycleTime := 0.1);
    IF (n >= 1 AND n <= 19) OR (n >= 31 AND n <= 33) THEN
        IF df.Fridge THEN ok0 := TRUE; END_IF;
    END_IF;
    IF (n = 21 OR n = 22) AND df.DefrostHeat AND df.InDefrost THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 24 AND df.DefrostCount >= 1 THEN ok2 := TRUE; END_IF;
    IF n >= 36 AND n <= 38 AND df.DoorAlarm THEN ok3 := TRUE; END_IF;
    IF n >= 40 AND n <= 44 AND NOT df.DoorAlarm THEN ok3b := TRUE; END_IF;
    IF n >= 45 AND df.TempAlarm THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 AND ok3b THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
