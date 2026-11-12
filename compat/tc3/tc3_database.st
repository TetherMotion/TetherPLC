// Tc2_Database / Tc3_Database -> SQL surface over the tplc-db
// service (embedded SQLite, no TwinCAT Database Server, no
// ODBC driver manager).  Documented differences:
//   - Statements are plain SQL text (SQLite dialect).
//   - Query results cross as tab-separated rows in a bounded
//     255-byte buffer; ROWS/COLS report the produced shape.
//   - METHOD calls are edge-triggered command inputs (nCmd).
//   - The TF6420 configuration/AutoLog/event families
//     (FB_ConfigTcDBSrvEvt, FB_PLCDBAutoLogEvt) are not
//     modelled; the PLC DB access blocks are.
//   - nErrId carries tplc-db service codes; GetError returns the
//     engine's last error text.

// ---- FB_PLCDBOpen: open/create a database ----
FUNCTION_BLOCK FB_PLCDBOpen
VAR_INPUT
    nCmd : UDINT;               // 1 open, 4 close
    sPath : STRING[127];
END_VAR
VAR_OUTPUT
    hDb : UDINT;
    bDone : BOOL;
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    db : FB_Db;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0; bBusy := FALSE;
    IF nCmd <> 0 AND prev = 0 THEN
        bBusy := TRUE;
        db.OP := nCmd;
        db.HANDLE := hDb;
        db.PATH := sPath;
        db();
        bBusy := FALSE;
        IF db.STATUS = 0 THEN
            bDone := TRUE;
            IF nCmd = 1 THEN hDb := db.HANDLE; ELSE hDb := 0; END_IF;
        ELSE
            bError := TRUE; nErrId := db.ERRID;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_PLCDBCreateEvt: CREATE/DDL and other exec statements ----
FUNCTION_BLOCK FB_PLCDBCreateEvt
VAR_INPUT
    nCmd : UDINT;               // 2 exec
    hDb : UDINT;
    sSql : STRING[255];
END_VAR
VAR_OUTPUT
    nChanges : UDINT;
    bDone : BOOL;
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    db : FB_Db;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0; bBusy := FALSE;
    IF nCmd <> 0 AND prev = 0 THEN
        bBusy := TRUE;
        db.OP := 2; db.HANDLE := hDb; db.SQL := sSql;
        db();
        bBusy := FALSE;
        IF db.STATUS = 0 THEN
            bDone := TRUE; nChanges := db.CHANGES;
        ELSE
            bError := TRUE; nErrId := db.ERRID;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_PLCDBWriteEvt: INSERT/UPDATE/DELETE ----
FUNCTION_BLOCK FB_PLCDBWriteEvt
VAR_INPUT
    nCmd : UDINT;               // 2 exec
    hDb : UDINT;
    sSql : STRING[255];
END_VAR
VAR_OUTPUT
    nChanges : UDINT;
    bDone : BOOL;
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    db : FB_Db;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0; bBusy := FALSE;
    IF nCmd <> 0 AND prev = 0 THEN
        bBusy := TRUE;
        db.OP := 2; db.HANDLE := hDb; db.SQL := sSql;
        db();
        bBusy := FALSE;
        IF db.STATUS = 0 THEN
            bDone := TRUE; nChanges := db.CHANGES;
        ELSE
            bError := TRUE; nErrId := db.ERRID;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_PLCDBReadEvt: SELECT -> rows/cols + text buffer ----
FUNCTION_BLOCK FB_PLCDBReadEvt
VAR_INPUT
    nCmd : UDINT;               // 3 query
    hDb : UDINT;
    sSql : STRING[255];
END_VAR
VAR_OUTPUT
    nRows : UDINT;
    nCols : UDINT;
    sResult : STRING[255];      // tab-separated, '\n' per row
    bDone : BOOL;
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    db : FB_Db;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0; bBusy := FALSE;
    IF nCmd <> 0 AND prev = 0 THEN
        bBusy := TRUE;
        db.OP := 3; db.HANDLE := hDb; db.SQL := sSql;
        db();
        bBusy := FALSE;
        IF db.STATUS = 0 THEN
            bDone := TRUE;
            nRows := db.ROWS; nCols := db.COLS;
            sResult := db.BUF;
        ELSE
            bError := TRUE; nErrId := db.ERRID;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_PLCDBGetError: engine error text ----
FUNCTION_BLOCK FB_PLCDBGetError
VAR_INPUT
    nCmd : UDINT;               // 5 error
    hDb : UDINT;
END_VAR
VAR_OUTPUT
    sError : STRING[127];
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    db : FB_Db;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        db.OP := 5; db.HANDLE := hDb;
        db();
        IF db.STATUS = 0 THEN
            bDone := TRUE; sError := db.ERR;
        ELSE
            bError := TRUE; nErrId := db.ERRID;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK
