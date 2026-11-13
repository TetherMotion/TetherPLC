// tc2_pfwlib_case.st — TcTvA alarm-queue e2e: append x2, query,
// deactivate, garbage-collect, clear-pending, clear-signal,
// update-time.  Commands are edge-triggered (nCmd held 0 between);
// ELSIF chain prevents same-scan state cascade.
PROGRAM tc2_pfwlib_case
VAR
    q : FB_TcTvAMsgQueue;
    st : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        q.nCmd := 0;
        st := 1;
    ELSIF st = 1 THEN
        q.Idx1 := 1; q.Idx2 := 2; q.ErrorId := 100; q.Prio := 3;
        q.Tick := 1000; q.nCmd := 1;
        st := 2;
    ELSIF st = 2 THEN
        q.nCmd := 0;
        IF q.bDone AND q.Count = 1 THEN mask := mask + 1; END_IF;
        st := 3;
    ELSIF st = 3 THEN
        q.Idx1 := 1; q.Idx2 := 2; q.ErrorId := 200; q.Prio := 2;
        q.Tick := 1001; q.nCmd := 1;
        st := 4;
    ELSIF st = 4 THEN
        q.nCmd := 0;
        IF q.bDone AND q.Count = 2 THEN mask := mask + 2; END_IF;
        st := 5;
    ELSIF st = 5 THEN
        // duplicate of A -> reactivates, count stays 2
        q.Idx1 := 1; q.Idx2 := 2; q.ErrorId := 100; q.Prio := 3;
        q.Tick := 1002; q.nCmd := 1;
        st := 6;
    ELSIF st = 6 THEN
        q.nCmd := 0;
        IF q.bDone AND q.Count = 2 THEN mask := mask + 4; END_IF;
        q.QSlot := 0;
        st := 7;
    ELSIF st = 7 THEN
        IF q.QUsed AND q.QActive AND q.QSignaled
           AND q.QIdx1 = 1 AND q.QIdx2 = 2 AND q.QErrorId = 100
           AND q.QPrio = 3 THEN
            mask := mask + 8;
        END_IF;
        st := 8;
    ELSIF st = 8 THEN
        q.Idx1 := 1; q.Idx2 := 2; q.ErrorId := 100; q.Prio := 3;
        q.nCmd := 4;             // deactivate A
        st := 9;
    ELSIF st = 9 THEN
        q.nCmd := 0;
        q.QSlot := 0;
        IF q.bDone AND NOT q.QActive AND q.QUsed THEN
            mask := mask + 16;
        END_IF;
        st := 10;
    ELSIF st = 10 THEN
        q.nCmd := 5;             // garbage collect
        st := 11;
    ELSIF st = 11 THEN
        q.nCmd := 0;
        IF q.bDone AND q.bSignaled AND q.Count = 2 THEN
            mask := mask + 32;
        END_IF;
        st := 12;
    ELSIF st = 12 THEN
        q.Idx1 := 1; q.Idx2 := 2; q.ErrorId := 200; q.Prio := 2;
        q.QSlot := 1;
        q.nCmd := 3;             // clear signal on B
        st := 13;
    ELSIF st = 13 THEN
        q.nCmd := 0;
        IF q.bDone AND NOT q.QSignaled AND q.QActive THEN
            mask := mask + 64;
        END_IF;
        st := 14;
    ELSIF st = 14 THEN
        // clear pending: drop inactive entries with Idx1 in [1..1];
        // A (inactive) removed, B (active) kept -> count 1
        q.IdxFirst := 1; q.IdxLast := 1;
        q.QSlot := 0;
        q.nCmd := 2;
        st := 15;
    ELSIF st = 15 THEN
        q.nCmd := 0;
        IF q.bDone AND q.Count = 1 THEN mask := mask + 128; END_IF;
        st := 16;
    ELSIF st = 16 THEN
        // remaining slot0 is B: active, signal cleared
        IF q.QUsed AND q.QErrorId = 200 AND q.QActive
           AND NOT q.QSignaled THEN
            mask := mask + 256;
        END_IF;
        st := 17;
    ELSIF st = 17 THEN
        q.Tick := 7777; q.nCmd := 6;   // update time
        st := 18;
    ELSIF st = 18 THEN
        q.nCmd := 0;
        IF q.bDone AND q.QTick = 7777 THEN mask := mask + 512; END_IF;
        st := 19;
    ELSIF st = 19 THEN
        q.nCmd := 99;            // unknown command
        st := 20;
    ELSIF st = 20 THEN
        q.nCmd := 0;
        IF q.bError AND q.ErrCode = 16#8002 THEN
            mask := mask + 1024;
        END_IF;
        st := 21;
    END_IF;
    q();
END_PROGRAM
