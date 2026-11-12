// Tc3_EventLogger -> FB_TcEvent / FB_TcMessage / FB_TcAlarm over
// the tplc-eventlog bounded journal (128 entries, deterministic).
// Documented reductions vs the 146-entry catalogue:
//   - METHOD calls are edge-triggered command inputs (nCmd):
//     FB_TcEvent: 1=Create+Send 2=Clear 3=Confirm.
//   - Event classes are UDINT ids (no GUID type in this ST).
//   - Text args are passed raw (nArgs pack) — no .tmx resource
//     lookup or language switching; text is a literal STRING.
//   - Async text-request / remote-logger / query-API families
//     (FB_Request*, I_TcRemoteEventLogger) are not modelled;
//     FB_TcEventQuery covers synchronous journal reads.
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
