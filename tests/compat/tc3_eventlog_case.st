// tc3_eventlog_case.st — e2e for tc3_eventlog (tplc-eventlog).
PROGRAM tc3_eventlog_case
VAR
    msg : FB_TcMessage;
    alm : FB_TcAlarm;
    qry : FB_TcEventQuery;
    lg : FB_TcEventLogger;
    eb : FB_TcEventBase;
    ar : FB_TcArguments;
    si : FB_TcSourceInfo;
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
    ELSIF st = 14 THEN
        // second alarm for the aggregate/logger surface
        alm.nEventClass := 8; alm.nEventId := 77;
        alm.nSeverity := 3; alm.sText := 'overtemp';
        alm.nCmd := 1;
        lg.nFilterClass := 8; lg.nFilterEid := 0;
        st := 15;
    ELSIF st = 15 THEN
        IF alm.bDone THEN mask := mask OR 128; END_IF;
        alm.nCmd := 0;
        lg.nCmd := 7;                           // IsAlarmRaised
        st := 16;
    ELSIF st = 16 THEN
        lg.nCmd := 0;
        IF lg.bFound AND lg.nEidOut = 77 AND lg.nState = 1 THEN
            mask := mask OR 256;
        END_IF;
        st := 17;
    ELSIF st = 17 THEN
        lg.nCmd := 8;                           // GetAlarm
        st := 18;
    ELSIF st = 18 THEN
        lg.nCmd := 0;
        IF lg.bFound AND FIND(lg.sTextOut, 'overtemp') > 0 THEN
            mask := mask OR 512;
        END_IF;
        // EqualsToEventEntry against journal id 2
        eb.EventClass := 8; eb.EventId := 77;
        eb.Severity := 3; eb.Text := 'overtemp';
        eb.cEntryId := 2; eb.nCmd := 3;
        st := 19;
    ELSIF st = 19 THEN
        eb.nCmd := 0;
        IF eb.bEqual THEN mask := mask OR 1024; END_IF;
        lg.nCmd := 9;                           // ExportLoggedEvents
        st := 20;
    ELSIF st = 20 THEN
        lg.nCmd := 0;
        IF LEN(lg.sExport) > 0
           AND FIND(lg.sExport, ';') > 0
           AND FIND(lg.sExport, 'overtemp') > 0 THEN
            mask := mask OR 2048;
        END_IF;
        st := 21;
    ELSIF st = 21 THEN
        lg.nCmd := 2;                           // ClearAlarms (class 8)
        st := 22;
    ELSIF st = 22 THEN
        lg.nCmd := 0;
        IF lg.nCount = 1 THEN mask := mask OR 4096; END_IF;
        st := 23;
    ELSIF st = 23 THEN
        lg.nCmd := 5;                           // ConfirmAlarms
        st := 24;
    ELSIF st = 24 THEN
        lg.nCmd := 0;
        IF lg.nCount = 1 THEN mask := mask OR 8192; END_IF;
        // third alarm for the *All* variants
        alm.nEventClass := 8; alm.nEventId := 78;
        alm.nSeverity := 3; alm.sText := 'vibration';
        alm.nCmd := 1;
        st := 25;
    ELSIF st = 25 THEN
        IF alm.bDone THEN mask := mask OR 16384; END_IF;
        alm.nCmd := 0;
        lg.nCmd := 3;                           // ClearAllAlarms
        st := 26;
    ELSIF st = 26 THEN
        lg.nCmd := 0;
        IF lg.nCount >= 1 THEN mask := mask OR 32768; END_IF;
        st := 27;
    ELSIF st = 27 THEN
        lg.nCmd := 6;                           // ConfirmAllAlarms
        st := 28;
    ELSIF st = 28 THEN
        lg.nCmd := 0;
        IF lg.nCount >= 1 THEN mask := mask OR 65536; END_IF;
        st := 29;
    ELSIF st = 29 THEN
        lg.nCmd := 4;                           // ClearLoggedEvents
        st := 30;
    ELSIF st = 30 THEN
        lg.nCmd := 0;
        IF lg.nCount >= 1 THEN mask := mask OR 131072; END_IF;
        // FB_TcEventBase comparisons + text requests
        eb.EventClass := 8; eb.EventId := 77;
        eb.Severity := 3; eb.Text := 'overtemp';
        eb.cClass := 8; eb.cId := 77; eb.cSev := 3;
        eb.cText := 'overtemp'; eb.nCmd := 1;   // EqualsTo
        st := 31;
    ELSIF st = 31 THEN
        eb.nCmd := 0;
        IF eb.bEqual THEN mask := mask OR 262144; END_IF;
        st := 32;
    ELSIF st = 32 THEN
        eb.nCmd := 2;                           // EqualsToEventClass
        st := 33;
    ELSIF st = 33 THEN
        eb.nCmd := 0;
        IF eb.bEqual THEN mask := mask OR 524288; END_IF;
        st := 34;
    ELSIF st = 34 THEN
        eb.nCmd := 5;                           // RequestEventClassName
        st := 35;
    ELSIF st = 35 THEN
        eb.nCmd := 0;
        IF FIND(eb.sOut, '8') = 1 THEN mask := mask OR 1048576; END_IF;
        st := 36;
    ELSIF st = 36 THEN
        eb.nCmd := 6;                           // RequestEventText
        st := 37;
    ELSIF st = 37 THEN
        eb.nCmd := 0;
        IF FIND(eb.sOut, 'overtemp') = 1 THEN
            mask := mask OR 2097152;
        END_IF;
        ar.nInt := 5; ar.nCmd := 1;             // AddInt
        st := 38;
    ELSIF st = 38 THEN
        ar.nCmd := 0;
        st := 39;
    ELSIF st = 39 THEN
        IF ar.nCount = 1 THEN mask := mask OR 4194304; END_IF;
        ar.sArg := 'hi'; ar.nCmd := 3;          // AddString
        st := 40;
    ELSIF st = 40 THEN
        ar.nCmd := 0;
        st := 41;
    ELSIF st = 41 THEN
        IF ar.nCount = 2 THEN mask := mask OR 8388608; END_IF;
        si.Name := 'main'; si.nCmd := 1;        // ExtendName
        st := 42;
    ELSIF st = 42 THEN
        si.nCmd := 0;
        IF si.Extended THEN mask := mask OR 16777216; END_IF;
        st := 43;
    ELSIF st = 43 THEN
        ar.nCmd := 5;                           // Clear args
        st := 44;
    ELSIF st = 44 THEN
        ar.nCmd := 0;
        st := 45;
    ELSIF st = 45 THEN
        IF ar.bEmpty THEN mask := mask OR 33554432; END_IF;
        st := 46;
    END_IF;
    msg(); alm(); qry(); lg(); eb(); ar(); si();
END_PROGRAM
