// tc3_eventlog_case.st — e2e for tc3_eventlog (tplc-eventlog).
PROGRAM tc3_eventlog_case
VAR
    msg : FB_TcMessage;
    alm : FB_TcAlarm;
    qry : FB_TcEventQuery;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        msg.nEventClass := 7; msg.nEventId := 42;
        msg.sText := 'pump on'; msg.nCmd := 1;  // send INFO
        st := 1;
    ELSIF st = 1 THEN
        IF msg.bDone AND msg.hEvent <> 0 THEN mask := mask OR 1; END_IF;
        msg.nCmd := 0;
        alm.nEventClass := 7; alm.nEventId := 99;
        alm.nSeverity := 3; alm.sText := 'temp high';
        st := 2;
    ELSIF st = 2 THEN
        alm.nCmd := 1;                          // raise alarm
        st := 3;
    ELSIF st = 3 THEN
        IF alm.bDone AND alm.bActive AND alm.nState = 1 THEN
            mask := mask OR 2;
        END_IF;
        alm.nCmd := 0;
        st := 4;
    ELSIF st = 4 THEN
        alm.nCmd := 2;                          // clear
        st := 5;
    ELSIF st = 5 THEN
        IF alm.bDone AND alm.nState = 2 AND alm.bActive THEN
            mask := mask OR 4;
        END_IF;
        alm.nCmd := 0;
        st := 6;
    ELSIF st = 6 THEN
        qry.nId := alm.hEvent; qry.nCmd := 4;  // query the entry
        st := 7;
    ELSIF st = 7 THEN
        IF qry.bDone AND qry.nState = 2 AND qry.nSeverity = 3
           AND qry.nEventId = 99 THEN
            mask := mask OR 8;
        END_IF;
        qry.nCmd := 0;
        st := 8;
    ELSIF st = 8 THEN
        alm.nCmd := 3;                          // confirm
        st := 9;
    ELSIF st = 9 THEN
        IF alm.bDone AND NOT alm.bActive THEN mask := mask OR 16; END_IF;
        alm.nCmd := 0;
        st := 10;
    ELSIF st = 10 THEN
        // wrong-state op must error: query the released id
        qry.nId := 255; qry.nCmd := 4;
        st := 11;
    ELSIF st = 11 THEN
        IF qry.bError THEN mask := mask OR 32; END_IF;
        qry.nCmd := 0;
        st := 12;
    ELSIF st = 12 THEN
        qry.nCmd := 5;                          // stats
        st := 13;
    ELSIF st = 13 THEN
        IF qry.bDone AND qry.nRaised = 2 AND qry.nCleared = 1
           AND qry.nConfirmed = 1 AND qry.nLive = 1 THEN
            mask := mask OR 64;
        END_IF;
        qry.nCmd := 0;
        st := 14;
    END_IF;
    msg(); alm(); qry();
END_PROGRAM
