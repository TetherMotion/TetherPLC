// Tc2_DataExchange -> symbolic read/write by variable name over
// the tplc-sym service (package var table -> live state blob).
// Documented differences vs. the Beckhoff library:
//   - Names address variables of the *local* program (no AMS
//     NetID routing): sNetId is accepted and ignored.
//   - Values cross typed (BOOL/DINT/LREAL): the builtin encodes
//     them into the variable's own width/type.  Byte-level access
//     is available through FB_Sym OP 1/2 for callers that need it.
//   - OnDelta blocks compare against the last transmitted value
//     and only transmit when the variable actually changed.
//   - Watchdog blocks read/write the runtime's cycle counters via
//     the same by-name path (documented reduction).

// ---- FB_ReadAdsSymByName: read a variable by name ----
FUNCTION_BLOCK FB_ReadAdsSymByName
VAR_INPUT
    bRead : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
END_VAR
VAR_OUTPUT
    nIntValue : DINT;
    fRealValue : LREAL;
    bBoolValue : BOOL;
    nTypeCode : UDINT;          // TPLC_IOT_* of the variable
    nSize : UDINT;              // variable size in bytes
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    sym : FB_Sym;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE; nErrorId := 0;
    IF bRead AND NOT prev THEN
        bBusy := TRUE;
        sym.OP := 9; sym.NAME := sVarName;
        sym();
        IF sym.STATUS = 0 THEN
            IF sym.VTYP = 0 THEN
                sym.OP := 8;
            ELSIF sym.VTYP = 9 OR sym.VTYP = 10 THEN
                sym.OP := 7;
            ELSE
                sym.OP := 6;
            END_IF;
            sym();
        END_IF;
        bBusy := FALSE;
        IF sym.STATUS = 0 THEN
            nTypeCode := sym.VTYP;
            nSize := sym.SIZE;
            IF sym.VTYP = 0 THEN
                bBoolValue := sym.BB;
            ELSIF sym.VTYP = 9 OR sym.VTYP = 10 THEN
                fRealValue := sym.FB;
            ELSE
                nIntValue := sym.IB;
            END_IF;
        ELSE
            bError := TRUE; nErrorId := sym.ERRID;
        END_IF;
    END_IF;
    prev := bRead;
END_FUNCTION_BLOCK

// ---- FB_WriteAdsSymByName: write a variable by name ----
FUNCTION_BLOCK FB_WriteAdsSymByName
VAR_INPUT
    bWrite : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    nIntValue : DINT;
    fRealValue : LREAL;
    bBoolValue : BOOL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    sym : FB_Sym;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE; bDone := FALSE; nErrorId := 0;
    IF bWrite AND NOT prev THEN
        bBusy := TRUE;
        sym.OP := 9; sym.NAME := sVarName;
        sym();
        IF sym.STATUS = 0 THEN
            sym.IB := nIntValue; sym.FB := fRealValue; sym.BB := bBoolValue;
            IF sym.VTYP = 0 THEN
                sym.OP := 5;
            ELSIF sym.VTYP = 9 OR sym.VTYP = 10 THEN
                sym.OP := 4;
            ELSE
                sym.OP := 3;
            END_IF;
            sym();
        END_IF;
        bBusy := FALSE;
        IF sym.STATUS = 0 THEN
            bDone := TRUE;
        ELSE
            bError := TRUE; nErrorId := sym.ERRID;
        END_IF;
    END_IF;
    prev := bWrite;
END_FUNCTION_BLOCK

// ---- OnDelta writers: transmit only when the value changed ----
// One shared body per type; the delta test compares the current
// bytes against the last transmitted snapshot.
FUNCTION_BLOCK FB_WriteBoolOnDelta
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    bValue : BOOL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bLastSignal : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
    nErrorCnt : UDINT;
END_VAR
VAR
    wr : FB_WriteAdsSymByName;
    sent : BOOL;
    haveSent : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE;
    IF bEnable AND (NOT haveSent OR bValue <> sent) THEN
        bBusy := TRUE;
        wr.sVarName := sVarName;
        wr.bBoolValue := bValue;
        wr.bWrite := TRUE;
        wr();
        wr.bWrite := FALSE;
        bBusy := FALSE;
        IF wr.bError THEN
            bError := TRUE; nErrorId := wr.nErrorId;
            nErrorCnt := nErrorCnt + 1;
        ELSE
            sent := bValue; haveSent := TRUE;
            bLastSignal := bValue;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_WriteByteOnDelta
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    nValue : BYTE;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    nLastSignal : BYTE;
    bError : BOOL;
    nErrorId : UDINT;
    nErrorCnt : UDINT;
END_VAR
VAR
    wr : FB_WriteAdsSymByName;
    sent : BYTE;
    haveSent : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE;
    IF bEnable AND (NOT haveSent OR nValue <> sent) THEN
        bBusy := TRUE;
        wr.sVarName := sVarName;
        wr.nIntValue := BYTE_TO_DINT(nValue);
        wr.bWrite := TRUE;
        wr();
        wr.bWrite := FALSE;
        bBusy := FALSE;
        IF wr.bError THEN
            bError := TRUE; nErrorId := wr.nErrorId;
            nErrorCnt := nErrorCnt + 1;
        ELSE
            sent := nValue; haveSent := TRUE;
            nLastSignal := nValue;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_WriteWordOnDelta
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    nValue : WORD;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    nLastSignal : WORD;
    bError : BOOL;
    nErrorId : UDINT;
    nErrorCnt : UDINT;
END_VAR
VAR
    wr : FB_WriteAdsSymByName;
    sent : WORD;
    haveSent : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE;
    IF bEnable AND (NOT haveSent OR nValue <> sent) THEN
        bBusy := TRUE;
        wr.sVarName := sVarName;
        wr.nIntValue := WORD_TO_DINT(nValue);
        wr.bWrite := TRUE;
        wr();
        wr.bWrite := FALSE;
        bBusy := FALSE;
        IF wr.bError THEN
            bError := TRUE; nErrorId := wr.nErrorId;
            nErrorCnt := nErrorCnt + 1;
        ELSE
            sent := nValue; haveSent := TRUE;
            nLastSignal := nValue;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_WriteDWordOnDelta
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    nValue : DWORD;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    nLastSignal : DWORD;
    bError : BOOL;
    nErrorId : UDINT;
    nErrorCnt : UDINT;
END_VAR
VAR
    wr : FB_WriteAdsSymByName;
    sent : DWORD;
    haveSent : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE;
    IF bEnable AND (NOT haveSent OR nValue <> sent) THEN
        bBusy := TRUE;
        wr.sVarName := sVarName;
        wr.nIntValue := DWORD_TO_DINT(nValue);
        wr.bWrite := TRUE;
        wr();
        wr.bWrite := FALSE;
        bBusy := FALSE;
        IF wr.bError THEN
            bError := TRUE; nErrorId := wr.nErrorId;
            nErrorCnt := nErrorCnt + 1;
        ELSE
            sent := nValue; haveSent := TRUE;
            nLastSignal := nValue;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_WriteRealOnDelta
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    fValue : REAL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    fLastSignal : REAL;
    bError : BOOL;
    nErrorId : UDINT;
    nErrorCnt : UDINT;
END_VAR
VAR
    wr : FB_WriteAdsSymByName;
    sent : REAL;
    haveSent : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE;
    IF bEnable AND (NOT haveSent OR fValue <> sent) THEN
        bBusy := TRUE;
        wr.sVarName := sVarName;
        wr.fRealValue := fValue;
        wr.bWrite := TRUE;
        wr();
        wr.bWrite := FALSE;
        bBusy := FALSE;
        IF wr.bError THEN
            bError := TRUE; nErrorId := wr.nErrorId;
            nErrorCnt := nErrorCnt + 1;
        ELSE
            sent := fValue; haveSent := TRUE;
            fLastSignal := fValue;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_WriteLRealOnDelta
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    fValue : LREAL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    fLastSignal : LREAL;
    bError : BOOL;
    nErrorId : UDINT;
    nErrorCnt : UDINT;
END_VAR
VAR
    wr : FB_WriteAdsSymByName;
    sent : LREAL;
    haveSent : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE;
    IF bEnable AND (NOT haveSent OR fValue <> sent) THEN
        bBusy := TRUE;
        wr.sVarName := sVarName;
        wr.fRealValue := fValue;
        wr.bWrite := TRUE;
        wr();
        wr.bWrite := FALSE;
        bBusy := FALSE;
        IF wr.bError THEN
            bError := TRUE; nErrorId := wr.nErrorId;
            nErrorCnt := nErrorCnt + 1;
        ELSE
            sent := fValue; haveSent := TRUE;
            fLastSignal := fValue;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- Watchdog blocks ----
// FB_WriteWatchdog stamps a by-name counter variable (typically the
// program's own heartbeat), FB_CheckWatchdog verifies that a peer's
// stamp is still advancing within a timeout.
FUNCTION_BLOCK FB_WriteWatchdog
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
    nStamps : UDINT;
END_VAR
VAR
    wr : FB_WriteAdsSymByName;
    n : DWORD;
END_VAR
    bBusy := FALSE; bError := FALSE;
    IF bEnable THEN
        n := n + 1;
        wr.sVarName := sVarName;
        wr.nIntValue := DWORD_TO_DINT(n);
        wr.bWrite := TRUE;
        wr();
        wr.bWrite := FALSE;
        IF wr.bError THEN
            bError := TRUE; nErrorId := wr.nErrorId;
        ELSE
            nStamps := nStamps + 1;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_CheckWatchdog
VAR_INPUT
    bEnable : BOOL;
    sNetId : STRING[23];
    sVarName : STRING[63];
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bAlive : BOOL;
    bTimeout : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    rd : FB_ReadAdsSymByName;
    last : DINT;
    haveLast : BOOL;
    elapsed : UDINT;            // scans since the last change
    limit : UDINT;
END_VAR
    bError := FALSE;
    IF bEnable THEN
        rd.sVarName := sVarName;
        rd.bRead := TRUE;
        rd();
        rd.bRead := FALSE;
        IF rd.bError THEN
            bError := TRUE; nErrorId := rd.nErrorId;
        ELSE
            elapsed := elapsed + 1;
            IF NOT haveLast THEN
                haveLast := TRUE;
                last := rd.nIntValue;
                elapsed := 0;
            ELSIF rd.nIntValue <> last THEN
                last := rd.nIntValue;
                elapsed := 0;
            END_IF;
            // tTimeout is expressed in scans here (documented
            // reduction: no per-instance wall clock in ST)
            limit := DINT_TO_UDINT(TIME_TO_DINT(tTimeout)) / 1000000;
            IF limit = 0 THEN limit := 1; END_IF;
            bAlive := elapsed <= limit;
            bTimeout := NOT bAlive;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
