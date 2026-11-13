// tc2_pfwlib.st — PfwLib_Processing surface (TF8540 plastic-
// processing framework).  This tranche implements the TcTvA alarm-
// visualization message family (FB_MsgAppend/ClearPending/
// ClearSignal/Deactivate/GarbageCollect/UpdateTime) as one fixed-
// table queue FB: the TwinCAT originals take POINTER TO
// ST_TcTvA_Alarm_Itf arrays, which the runtime does not support, so
// the buffer lives inside the instance (32 slots, parallel arrays).
//
// Semantics per slot: an entry matches on (Idx1, Idx2, ErrorId, Prio).
// Append re-activates a matching entry or appends a new one;
// Deactivate clears Active but keeps the history entry;
// ClearSignal clears the Signal flag;
// ClearPending removes inactive entries in [IdxFirst..IdxLast];
// GarbageCollect compacts unused slots upward and reports whether
// any active message remains; UpdateTime stamps entries with the
// caller-supplied tick (the platform has no Windows time source).
//
// Commands are edge-triggered: hold nCmd = 0 for one full scan
// between commands.

FUNCTION_BLOCK FB_TcTvAMsgQueue
VAR_INPUT
    nCmd : UDINT;
    // append/deactivate/clearsignal match tuple
    Idx1 : INT;
    Idx2 : INT;
    ErrorId : DINT;
    Prio : INT;
    // clearpending range
    IdxFirst : INT;
    IdxLast : INT;
    // query slot
    QSlot : UDINT;
    // update-time stamp (ms tick supplied by the application)
    Tick : UDINT;
END_VAR
VAR_OUTPUT
    bDone : BOOL;
    bError : BOOL;
    ErrCode : UDINT;
    Count : INT;
    // query outputs for slot QSlot
    QUsed : BOOL;
    QActive : BOOL;
    QSignaled : BOOL;
    QIdx1 : INT;
    QIdx2 : INT;
    QErrorId : DINT;
    QPrio : INT;
    QTick : UDINT;
    // garbage-collect result
    bSignaled : BOOL;
END_VAR
VAR
    cmdPrev : UDINT;
    // slot tables
    U : ARRAY[0..31] OF BOOL;    // slot used
    A : ARRAY[0..31] OF BOOL;    // active alarm
    S : ARRAY[0..31] OF BOOL;    // signaled (visible)
    I1 : ARRAY[0..31] OF INT;
    I2 : ARRAY[0..31] OF INT;
    EI : ARRAY[0..31] OF DINT;
    PR : ARRAY[0..31] OF INT;
    TK : ARRAY[0..31] OF UDINT;
    cnt : INT;
    i : INT;
    j : INT;
    found : BOOL;
END_VAR
    bDone := FALSE;
    bError := FALSE;
    ErrCode := 0;
    bSignaled := FALSE;
    IF nCmd <> 0 AND cmdPrev = 0 THEN
        bDone := TRUE;
        IF nCmd = 1 THEN
        // ---- append: match tuple -> reactivate, else append ----
        found := FALSE;
        FOR i := 0 TO cnt - 1 DO
            IF I1[i] = Idx1 AND I2[i] = Idx2 AND EI[i] = ErrorId
               AND PR[i] = Prio THEN
                A[i] := TRUE;
                S[i] := TRUE;
                TK[i] := Tick;
                found := TRUE;
            END_IF;
        END_FOR;
        IF NOT found THEN
            IF cnt < 32 THEN
                U[cnt] := TRUE;
                A[cnt] := TRUE;
                S[cnt] := TRUE;
                I1[cnt] := Idx1;
                I2[cnt] := Idx2;
                EI[cnt] := ErrorId;
                PR[cnt] := Prio;
                TK[cnt] := Tick;
                cnt := cnt + 1;
            ELSE
                bError := TRUE;
                ErrCode := 16#8001;   // queue full
            END_IF;
        END_IF;

    ELSIF nCmd = 2 THEN
        // ---- clear pending: drop inactive entries in range ----
        i := 0;
        WHILE i < cnt DO
            IF (I1[i] >= IdxFirst AND I1[i] <= IdxLast)
               AND NOT A[i] THEN
                // remove slot i by shifting down
                FOR j := i TO cnt - 2 DO
                    U[j] := U[j+1]; A[j] := A[j+1]; S[j] := S[j+1];
                    I1[j] := I1[j+1]; I2[j] := I2[j+1];
                    EI[j] := EI[j+1]; PR[j] := PR[j+1]; TK[j] := TK[j+1];
                END_FOR;
                U[cnt-1] := FALSE; A[cnt-1] := FALSE; S[cnt-1] := FALSE;
                I1[cnt-1] := 0; I2[cnt-1] := 0; EI[cnt-1] := 0;
                PR[cnt-1] := 0; TK[cnt-1] := 0;
                cnt := cnt - 1;
            ELSE
                i := i + 1;
            END_IF;
        END_WHILE;

    ELSIF nCmd = 3 THEN
        // ---- clear signal on matching entry ----
        FOR i := 0 TO cnt - 1 DO
            IF I1[i] = Idx1 AND I2[i] = Idx2 AND EI[i] = ErrorId
               AND PR[i] = Prio THEN
                S[i] := FALSE;
            END_IF;
        END_FOR;

    ELSIF nCmd = 4 THEN
        // ---- deactivate matching entry (kept in history) ----
        FOR i := 0 TO cnt - 1 DO
            IF I1[i] = Idx1 AND I2[i] = Idx2 AND EI[i] = ErrorId
               AND PR[i] = Prio THEN
                A[i] := FALSE;
            END_IF;
        END_FOR;

    ELSIF nCmd = 5 THEN
        // ---- garbage collect: compact used slots, report active ----
        j := 0;
        FOR i := 0 TO cnt - 1 DO
            IF U[i] THEN
                IF j <> i THEN
                    U[j] := U[i]; A[j] := A[i]; S[j] := S[i];
                    I1[j] := I1[i]; I2[j] := I2[i];
                    EI[j] := EI[i]; PR[j] := PR[i]; TK[j] := TK[i];
                    U[i] := FALSE; A[i] := FALSE; S[i] := FALSE;
                END_IF;
                j := j + 1;
            END_IF;
        END_FOR;
        cnt := j;
        FOR i := 0 TO cnt - 1 DO
            IF A[i] THEN bSignaled := TRUE; END_IF;
        END_FOR;

    ELSIF nCmd = 6 THEN
        // ---- update time: stamp all active entries ----
        FOR i := 0 TO cnt - 1 DO
            IF A[i] THEN TK[i] := Tick; END_IF;
        END_FOR;

        ELSE
            bError := TRUE;
            ErrCode := 16#8002;   // unknown command
        END_IF;
    END_IF;
    cmdPrev := nCmd;
    // query outputs reflect post-command state
    IF QSlot < 32 THEN
        QUsed := U[QSlot];
        QActive := A[QSlot];
        QSignaled := S[QSlot];
        QIdx1 := I1[QSlot];
        QIdx2 := I2[QSlot];
        QErrorId := EI[QSlot];
        QPrio := PR[QSlot];
        QTick := TK[QSlot];
    ELSE
        QUsed := FALSE; QActive := FALSE; QSignaled := FALSE;
        QIdx1 := 0; QIdx2 := 0; QErrorId := 0; QPrio := 0; QTick := 0;
    END_IF;
    Count := cnt;
END_FUNCTION_BLOCK
