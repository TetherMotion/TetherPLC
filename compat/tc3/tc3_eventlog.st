// Tc3_EventLogger -> FB_TcEvent / FB_TcMessage / FB_TcAlarm over
// the tplc-eventlog bounded journal (128 entries, deterministic).
// Documented reductions vs the 146-entry catalogue:
//   - METHOD calls are edge-triggered command inputs (nCmd):
//     FB_TcEvent: 1=Create+Send 2=Clear 3=Confirm.
//   - Event classes are UDINT ids (no GUID type in this ST).
//   - Text args are passed raw (nArgs pack) — no .tmx resource
//     lookup or language switching; text is a literal STRING.
//   - Async text-request / remote-logger families are modelled
//     synchronously: FB_TcEventQuery covers journal reads and
//     FB_TcEventBase.RequestEventText/ClassName copy descriptor
//     fields (no .tmx resource lookup, no ADS notifications —
//     FB_TcRemoteEventLogger stays a documented gap).
//   - nErrId carries tplc-eventlog service codes.

// ---- FB_TcEvent: create/raise/clear/confirm one event ----
FUNCTION_BLOCK FB_TcEvent
VAR_INPUT
    nCmd : UDINT;               // 0 none, 1 raise, 2 clear, 3 confirm
    nEventClass : UDINT;
    nEventId : UDINT;
    nSeverity : UDINT;          // 0 verbose..4 critical
    sText : STRING[80];
    nArgs : UDINT;
END_VAR
VAR_OUTPUT
    hEvent : UDINT;             // journal id, 0 = none
    nState : UDINT;             // 1 raised 2 cleared 3 confirmed
    bDone : BOOL;
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ev : FB_EventLog;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0; bBusy := FALSE;
    IF nCmd <> 0 AND prev = 0 THEN
        bBusy := TRUE;
        ev.ID := hEvent;
        IF nCmd = 1 THEN
            ev.OP := 1; ev.ECLASS := nEventClass;
            ev.EVTID := nEventId; ev.SEV := nSeverity;
            ev.ARGS := nArgs;
            ev.TEXT := sText;
        ELSIF nCmd = 2 THEN
            ev.OP := 2;
        ELSIF nCmd = 3 THEN
            ev.OP := 3;
        END_IF;
        ev();
        IF ev.STATUS = 0 THEN
            bDone := TRUE; bBusy := FALSE;
            IF nCmd = 1 THEN
                hEvent := ev.ID; nState := 1;
            ELSIF nCmd = 2 THEN
                nState := 2;
            ELSE
                nState := 3; hEvent := 0;
            END_IF;
        ELSE
            bError := TRUE; nErrId := ev.ERRID;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_TcMessage: INFO-severity event (send/reset) ----
FUNCTION_BLOCK FB_TcMessage
VAR_INPUT
    nCmd : UDINT;               // 1 send, 3 confirm+forget
    nEventClass : UDINT;
    nEventId : UDINT;
    sText : STRING[80];
    nArgs : UDINT;
END_VAR
VAR_OUTPUT
    hEvent : UDINT;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ev : FB_TcEvent;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    ev.nCmd := nCmd;
    ev.nEventClass := nEventClass;
    ev.nEventId := nEventId;
    ev.nSeverity := 1;              // INFO
    ev.sText := sText;
    ev.nArgs := nArgs;
    ev();
    hEvent := ev.hEvent;
    bDone := ev.bDone;
    bError := ev.bError;
    nErrId := ev.nErrId;
END_FUNCTION_BLOCK

// ---- FB_TcAlarm: raisable/clearable/confirmable alarm ----
FUNCTION_BLOCK FB_TcAlarm
VAR_INPUT
    nCmd : UDINT;               // 1 raise, 2 clear, 3 confirm
    nEventClass : UDINT;
    nEventId : UDINT;
    nSeverity : UDINT;          // typically WARNING/ERROR
    sText : STRING[80];
    nArgs : UDINT;
END_VAR
VAR_OUTPUT
    hEvent : UDINT;
    nState : UDINT;
    bActive : BOOL;             // raised and not confirmed
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ev : FB_TcEvent;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    ev.nCmd := nCmd;
    ev.nEventClass := nEventClass;
    ev.nEventId := nEventId;
    ev.nSeverity := nSeverity;
    ev.sText := sText;
    ev.nArgs := nArgs;
    ev();
    hEvent := ev.hEvent;
    nState := ev.nState;
    bDone := ev.bDone;
    bError := ev.bError;
    nErrId := ev.nErrId;
    bActive := ev.nState = 1 OR ev.nState = 2;
END_FUNCTION_BLOCK

// ---- FB_TcEventQuery: synchronous journal read + stats ----
FUNCTION_BLOCK FB_TcEventQuery
VAR_INPUT
    nCmd : UDINT;               // 4 query, 5 stats
    nId : UDINT;
END_VAR
VAR_OUTPUT
    nState : UDINT;
    nSeverity : UDINT;
    nEventClass : UDINT;
    nEventId : UDINT;
    sText : STRING[80];
    nLive : UDINT;
    nRaised : UDINT;
    nCleared : UDINT;
    nConfirmed : UDINT;
    nDropped : UDINT;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ev : FB_EventLog;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 4 OR nCmd = 5 THEN
            ev.OP := nCmd; ev.ID := nId;
            ev();
            IF ev.STATUS = 0 THEN
                bDone := TRUE;
                IF nCmd = 4 THEN
                    nState := ev.STATE;
                    nSeverity := ev.SEVO;
                    nEventClass := ev.CLSO;
                    nEventId := ev.EVTIDO;
                    sText := ev.TXTO;
                ELSE
                    nLive := ev.LIVE;
                    nRaised := ev.RAISED;
                    nCleared := ev.CLEARED;
                    nConfirmed := ev.CONFIRMED;
                    nDropped := ev.DROPPED;
                END_IF;
            ELSE
                bError := TRUE; nErrId := ev.ERRID;
            END_IF;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- helpers ----
// UDINT -> decimal text (no STRING_TO_UDINT builtin)
FUNCTION F_ElU32Str : STRING[10]
VAR_INPUT
    v : UDINT;
END_VAR
VAR
    x : UDINT;
    d : UDINT;
    s : STRING[10];
    c : STRING[1];
END_VAR
    IF v = 0 THEN
        F_ElU32Str := '0';
        RETURN;
    END_IF;
    x := v; s := '';
    WHILE x > 0 DO
        d := x MOD 10;
        CASE d OF
            0: c := '0';  1: c := '1';  2: c := '2';  3: c := '3';
            4: c := '4';  5: c := '5';  6: c := '6';  7: c := '7';
            8: c := '8';  ELSE c := '9';
        END_CASE;
        s := CONCAT(c, s);
        x := x / 10;
    END_WHILE;
    F_ElU32Str := s;
END_FUNCTION

// string equality via FIND (direct = is not usable on strings)
FUNCTION F_ElStreq : BOOL
VAR_INPUT
    a : STRING[80];
    b : STRING[80];
END_VAR
    F_ElStreq := LEN(a) = LEN(b) AND FIND(a, b) = 1;
END_FUNCTION

// ---- FB_TcEventBase: event descriptor + comparisons ----
// Descriptor fields (EventClass/EventId/Severity/Text) are public
// members set directly, mirroring the TwinCAT property surface;
// nCmd selects the METHOD-equivalent operation.
FUNCTION_BLOCK FB_TcEventBase
VAR_INPUT
    nCmd : UDINT;               // 1 EqualsTo 2 EqualsToEventClass
                                // 3 EqualsToEventEntry 4 GetJsonAttr
                                // 5 RequestEventClassName
                                // 6 RequestEventText 7 Release
    cClass : UDINT;
    cId : UDINT;
    cSev : UDINT;
    cText : STRING[80];
    cEntryId : UDINT;           // journal id for EqualsToEventEntry
    sAttr : STRING[31];         // 'class' 'id' 'severity' 'text'
END_VAR
VAR_OUTPUT
    bEqual : BOOL;
    sOut : STRING[80];
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    ev : FB_EventLog;
    // public descriptor members (TwinCAT ipEventClass etc.)
    EventClass : UDINT;
    EventId : UDINT;
    Severity : UDINT;
    Text : STRING[80];
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0; bEqual := FALSE;
    sOut := '';
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 7 THEN
            EventClass := 0; EventId := 0; Severity := 0; Text := '';
            bDone := TRUE;
        ELSIF nCmd = 1 THEN
            bEqual := EventClass = cClass AND EventId = cId
                      AND Severity = cSev AND F_ElStreq(Text, cText);
            bDone := TRUE;
        ELSIF nCmd = 2 THEN
            bEqual := EventClass = cClass;
            bDone := TRUE;
        ELSIF nCmd = 3 THEN
            ev.OP := 4; ev.ID := cEntryId; ev();
            IF ev.STATUS = 0 THEN
                bEqual := EventClass = ev.CLSO AND EventId = ev.EVTIDO
                          AND Severity = ev.SEVO
                          AND F_ElStreq(Text, ev.TXTO);
            ELSE
                bError := TRUE; nErrId := ev.ERRID;
            END_IF;
            bDone := TRUE;
        ELSIF nCmd = 4 THEN
            IF F_ElStreq(sAttr, 'class') THEN
                sOut := F_ElU32Str(EventClass);
            ELSIF F_ElStreq(sAttr, 'id') THEN
                sOut := F_ElU32Str(EventId);
            ELSIF F_ElStreq(sAttr, 'severity') THEN
                sOut := F_ElU32Str(Severity);
            ELSIF F_ElStreq(sAttr, 'text') THEN
                sOut := Text;
            ELSE
                bError := TRUE; nErrId := 6;
            END_IF;
            bDone := TRUE;
        ELSIF nCmd = 5 THEN
            // no class-name table: the class id is the name
            sOut := F_ElU32Str(EventClass);
            bDone := TRUE;
        ELSIF nCmd = 6 THEN
            sOut := Text;
            bDone := TRUE;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_TcArguments: typed argument list (I_TcArguments) ----
// METHODS Add* become commands; values are kept in fixed arrays
// (8 slots).  IsEmpty/nCount are live outputs.
FUNCTION_BLOCK FB_TcArguments
VAR_INPUT
    nCmd : UDINT;               // 1 AddInt 2 AddReal 3 AddString
                                // 4 AddBool 5 Clear
    nInt : DINT;
    fReal : LREAL;
    sArg : STRING[31];
    bArg : BOOL;
END_VAR
VAR_OUTPUT
    nCount : UDINT;
    bEmpty : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    cnt : UDINT;
    kd : ARRAY[0..7] OF INT;    // 1 int 2 real 3 string 4 bool
    iv : ARRAY[0..7] OF DINT;
    fv : ARRAY[0..7] OF LREAL;
    sv : ARRAY[0..7] OF STRING[31];
    bv : ARRAY[0..7] OF BOOL;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    nCount := cnt; bEmpty := cnt = 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 5 THEN
            cnt := 0; bDone := TRUE;
        ELSE
            IF cnt >= 8 THEN
                bError := TRUE; nErrId := 3;
            ELSE
                IF nCmd = 1 THEN
                    kd[cnt] := 1; iv[cnt] := nInt;
                ELSIF nCmd = 2 THEN
                    kd[cnt] := 2; fv[cnt] := fReal;
                ELSIF nCmd = 3 THEN
                    kd[cnt] := 3; sv[cnt] := sArg;
                ELSIF nCmd = 4 THEN
                    kd[cnt] := 4; bv[cnt] := bArg;
                ELSE
                    bError := TRUE; nErrId := 1;
                END_IF;
                IF NOT bError THEN
                    cnt := cnt + 1; bDone := TRUE;
                END_IF;
            END_IF;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_TcSourceInfo: source-name descriptor ----
// Name/Extended are public members; nCmd mirrors Clear /
// ExtendName / ResetToDefault.
FUNCTION_BLOCK FB_TcSourceInfo
VAR_INPUT
    nCmd : UDINT;               // 1 ExtendName 2 ResetToDefault
                                // 3 Clear
END_VAR
VAR_OUTPUT
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    Name : STRING[31];
    Extended : BOOL;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            Extended := TRUE; bDone := TRUE;
        ELSIF nCmd = 2 THEN
            Extended := FALSE; bDone := TRUE;
        ELSIF nCmd = 3 THEN
            Name := ''; Extended := FALSE; bDone := TRUE;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_TcCauseRemedy: cause/remedy text carrier ----
// Id/Cause/Remedy are public members; nCmd 1 loads them from the
// inputs (TwinCAT GetCause/GetId/GetRemedy read members directly),
// nCmd 2 releases.
FUNCTION_BLOCK FB_TcCauseRemedy
VAR_INPUT
    nCmd : UDINT;               // 1 Load 2 Release
    nId : UDINT;
    sCause : STRING[31];
    sRemedy : STRING[31];
END_VAR
VAR_OUTPUT
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    Id : UDINT;
    Cause : STRING[31];
    Remedy : STRING[31];
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            Id := nId; Cause := sCause; Remedy := sRemedy;
            bDone := TRUE;
        ELSIF nCmd = 2 THEN
            Id := 0; Cause := ''; Remedy := '';
            bDone := TRUE;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_TcDetail: name/text/comment carrier (same shape) ----
FUNCTION_BLOCK FB_TcDetail
VAR_INPUT
    nCmd : UDINT;               // 1 Load 2 Release
    sName : STRING[31];
    sText : STRING[31];
    sComment : STRING[31];
END_VAR
VAR_OUTPUT
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    Name : STRING[31];
    Text : STRING[31];
    Comment : STRING[31];
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            Name := sName; Text := sText; Comment := sComment;
            bDone := TRUE;
        ELSIF nCmd = 2 THEN
            Name := ''; Text := ''; Comment := '';
            bDone := TRUE;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_TcEventLogger: aggregate logger surface ----
// nCmd: 1 SendMessage 2 ClearAlarms(filter) 3 ClearAllAlarms
//       4 ClearLoggedEvents 5 ConfirmAlarms(filter)
//       6 ConfirmAllAlarms 7 IsAlarmRaised(filter)
//       8 GetAlarm(filter) 9 ExportLoggedEvents
// "Alarm" = severity >= WARNING (2); filter wildcards are 0.
FUNCTION_BLOCK FB_TcEventLogger
VAR_INPUT
    nCmd : UDINT;
    nEventClass : UDINT;
    nEventId : UDINT;
    nSeverity : UDINT;
    sText : STRING[80];
    nArgs : UDINT;
    nFilterClass : UDINT;       // 0 = any class
    nFilterEid : UDINT;         // 0 = any event id
END_VAR
VAR_OUTPUT
    hEvent : UDINT;
    bFound : BOOL;
    nCount : UDINT;
    nState : UDINT;
    nSevOut : UDINT;
    nClassOut : UDINT;
    nEidOut : UDINT;
    sTextOut : STRING[80];
    sExport : STRING[255];
    bTruncated : BOOL;
    bDone : BOOL;
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ev : FB_EventLog;
    msg : FB_TcMessage;
    prev : UDINT;
    i : UDINT;
    hit : BOOL;
    filt : BOOL;
    piece : STRING[255];
END_VAR
    bDone := FALSE; bBusy := FALSE; bError := FALSE; nErrId := 0;
    bFound := FALSE; nCount := 0; bTruncated := FALSE; sExport := '';
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            msg.nCmd := 1;
            msg.nEventClass := nEventClass;
            msg.nEventId := nEventId;
            msg.sText := sText;
            msg.nArgs := nArgs;
            msg();
            IF msg.bDone THEN
                hEvent := msg.hEvent; bDone := TRUE;
            ELSE
                bError := TRUE; nErrId := msg.nErrId;
            END_IF;
        ELSE
            FOR i := 1 TO 128 DO
                ev.OP := 4; ev.ID := i; ev();
                IF ev.STATUS = 0 THEN
                    filt := nCmd = 3
                            OR ((nFilterClass = 0 OR ev.CLSO = nFilterClass)
                                AND (nFilterEid = 0
                                     OR ev.EVTIDO = nFilterEid));
                    hit := filt AND ev.SEVO >= 2;
                    IF nCmd = 9 THEN
                        // export every live entry as a CSV row
                        piece := CONCAT(F_ElU32Str(i), ';');
                        piece := CONCAT(piece, F_ElU32Str(ev.CLSO));
                        piece := CONCAT(piece, ';');
                        piece := CONCAT(piece, F_ElU32Str(ev.EVTIDO));
                        piece := CONCAT(piece, ';');
                        piece := CONCAT(piece, F_ElU32Str(ev.SEVO));
                        piece := CONCAT(piece, ';');
                        piece := CONCAT(piece, F_ElU32Str(ev.STATE));
                        piece := CONCAT(piece, ';');
                        piece := CONCAT(piece, ev.TXTO);
                        piece := CONCAT(piece, '|');
                        IF LEN(sExport) + LEN(piece) < 250 THEN
                            sExport := CONCAT(sExport, piece);
                        ELSE
                            bTruncated := TRUE;
                        END_IF;
                    ELSIF hit AND ev.STATE <> 3 THEN
                        // candidate alarm entry
                        // (nCmd 6 is handled by the post-loop pass)
                        IF (nCmd = 2 AND ev.STATE = 1) OR nCmd = 5
                           OR nCmd = 3 THEN
                            IF nCmd = 5 THEN
                                ev.OP := 3; ev.ID := i; ev();
                            ELSE
                                ev.OP := 2; ev.ID := i; ev();
                            END_IF;
                            IF ev.STATUS = 0 THEN nCount := nCount + 1; END_IF;
                        END_IF;
                        IF NOT bFound
                           AND (nCmd = 7 OR nCmd = 8) THEN
                            bFound := TRUE;
                            nState := ev.STATE;
                            nSevOut := ev.SEVO;
                            nClassOut := ev.CLSO;
                            nEidOut := ev.EVTIDO;
                            sTextOut := ev.TXTO;
                        END_IF;
                    END_IF;
                END_IF;
            END_FOR;
            IF nCmd = 4 THEN
                // ClearLoggedEvents: confirm every non-terminal entry
                FOR i := 1 TO 128 DO
                    ev.OP := 4; ev.ID := i; ev();
                    IF ev.STATUS = 0 AND ev.STATE <> 3 THEN
                        ev.OP := 3; ev.ID := i; ev();
                        IF ev.STATUS = 0 THEN nCount := nCount + 1; END_IF;
                    END_IF;
                END_FOR;
            END_IF;
            IF nCmd = 6 THEN
                // ConfirmAllAlarms ignores the filter
                FOR i := 1 TO 128 DO
                    ev.OP := 4; ev.ID := i; ev();
                    IF ev.STATUS = 0 AND ev.STATE <> 3 AND ev.SEVO >= 2 THEN
                        ev.OP := 3; ev.ID := i; ev();
                        IF ev.STATUS = 0 THEN nCount := nCount + 1; END_IF;
                    END_IF;
                END_FOR;
            END_IF;
            bDone := TRUE;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK
