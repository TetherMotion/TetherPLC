// evlog_case.st — deterministic eventlog scenario for the ctl-side
// EVTLOG_* ops: raise A (stays live, then cleared), raise B
// (confirmed), plus a late INFO event for the EV_USER broadcast.
PROGRAM evlog_case
VAR
    ev : FB_EventLog;
    st : INT;
    idA : UDINT;
    idB : UDINT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        ev.ECLASS := 7; ev.EVTID := 42; ev.SEV := 3;
        ev.TEXT := 'temp high'; ev.OP := 1; ev();
        idA := ev.ID; ev.OP := 0;
        IF ev.STATUS = 0 AND idA <> 0 THEN mask := mask OR 1; END_IF;
        st := 1;
    ELSIF st = 1 THEN
        ev.ECLASS := 8; ev.EVTID := 77; ev.SEV := 2;
        ev.TEXT := 'overtemp'; ev.OP := 1; ev();
        idB := ev.ID; ev.OP := 0;
        IF ev.STATUS = 0 AND idB <> 0 AND idB <> idA THEN
            mask := mask OR 2;
        END_IF;
        st := 2;
    ELSIF st = 2 THEN
        ev.ID := idA; ev.OP := 2; ev(); ev.OP := 0;   // clear A
        IF ev.STATUS = 0 THEN mask := mask OR 4; END_IF;
        st := 3;
    ELSIF st = 3 THEN
        ev.ID := idB; ev.OP := 3; ev(); ev.OP := 0;   // confirm B
        IF ev.STATUS = 0 THEN mask := mask OR 8; END_IF;
        st := 4;
    ELSIF st = 4 THEN
        // query A: expect state=CLEARED(2), sev=3, text 'temp high'
        ev.ID := idA; ev.OP := 4; ev(); ev.OP := 0;
        IF ev.STATUS = 0 AND ev.STATE = 2 AND ev.SEVO = 3
           AND ev.TSTAMP <> 0 THEN
            mask := mask OR 16;
        END_IF;
        st := 5;
    ELSIF st = 5 THEN
        ev.OP := 5; ev(); ev.OP := 0;                 // stats
        // live=1 (A cleared), raised=2, cleared=1, confirmed=1
        IF ev.STATUS = 0 AND ev.LIVE = 1 AND ev.RAISED = 2
           AND ev.CLEARED = 1 AND ev.CONFIRMED = 1 THEN
            mask := mask OR 32;
        END_IF;
        st := 6;
    ELSIF st = 6 THEN
        // late INFO event — the EV_USER broadcast check window
        ev.ECLASS := 9; ev.EVTID := 5; ev.SEV := 1;
        ev.TEXT := 'cycle ok'; ev.OP := 1; ev(); ev.OP := 0;
        IF ev.STATUS = 0 THEN mask := mask OR 64; END_IF;
        st := 7;
    END_IF
END_PROGRAM
