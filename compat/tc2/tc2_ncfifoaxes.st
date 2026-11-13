// tc2_ncfifoaxes.st — Tc2_NcFifoAxes: FiFoAxes synchronous numeric
// setpoint FIFO channels (8 APIs) over the tplc-fifo service /
// FB_Fifo builtin.
//
// A FIFO channel queues rows of LREAL target positions (one column
// per integrated axis, group position = column) and emits one row
// per drive-clock tick as DS402 0x607A CSP targets. This is the
// numeric FiFoAxes surface; it is NOT the NC channel's G-code block
// queue (FB_ItpFiFoWrite / nc.fifo in tc2_nci.st).
//
// Flattening vs the TwinCAT signatures:
//   iChannelId    -> iChannelId (UDINT, 0..TPLC_FIFO_CHAN_MAX-1)
//   iAxisId       -> axis handle (UDINT; the same handle space as
//                    AXIS_REF / FB_MotionAxis, slave = handle-1 on
//                    Tether IO)
//   iGroupPosition-> 1-based in TwinCAT; mapped to the 0-based wire
//                    column internally
//   AdrDataArray  -> AdrDataArray : ARRAY[0..255] OF LREAL (flattened
//                    POINTER TO LREAL; the caller fills the wrapper's
//                    array, iRowsToWrite bounds the valid prefix,
//                    max 31 rows per call — chunk larger streams)
//   tTimeout      -> accepted, ignored (service calls are synchronous)
//   outputs       -> bBusy / bError / nErrorId plus domain outputs;
//                    bBusy pulses TRUE only during the Execute edge
// Channel state is visible on the internal FB_Fifo instance fields
// (STATE/QLEN/SEQ/NAXES/MAXROWS) for callers that need polling.

// ---- FiFoGetDimension: axes + capacity of a channel ----
FUNCTION_BLOCK FiFoGetDimension
VAR_INPUT
    iChannelId : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
    iAxes : UDINT;
    iMaxRows : UDINT;
END_VAR
VAR
    ch : FB_Fifo;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        ch.OP := 8;
        ch.CHAN := iChannelId;
        ch();
        IF ch.STATUS = 0 THEN
            iAxes := ch.NAXES;
            iMaxRows := ch.MAXROWS;
        ELSE
            bError := TRUE;
            nErrorId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FiFoGroupIntegrate: add an axis to the channel ----
FUNCTION_BLOCK FiFoGroupIntegrate
VAR_INPUT
    iChannelId : UDINT;
    iAxisId : UDINT;
    iGroupPosition : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ch : FB_Fifo;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        ch.OP := 1;
        ch.CHAN := iChannelId;
        /* TwinCAT group positions are 1-based; the wire column is
           0-based. */
        ch.AUX := iAxisId OR ((iGroupPosition - 1) * 65536);
        ch();
        IF ch.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FiFoGroupDisintegrate: clear axes + queued rows ----
FUNCTION_BLOCK FiFoGroupDisintegrate
VAR_INPUT
    iChannelId : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ch : FB_Fifo;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        ch.OP := 2;
        ch.CHAN := iChannelId;
        ch();
        IF ch.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FiFoWrite / FiFoOverwrite shared row pump ----
// Both copy caller rows into the FB_Fifo ROWS staging buffer in
// chunks of 32 rows (256 LREAL) and issue OP3 (append) / OP4
// (replace queue first) per chunk.
FUNCTION_BLOCK FiFoWritePump
VAR_INPUT
    iChannelId : UDINT;
    AdrDataArray : ARRAY[0..255] OF LREAL;
    iRowsToWrite : UDINT;
    bOverwrite : BOOL;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ch : FB_Fifo;
    exPrev : BOOL;
    i : UDINT;
    r : UDINT;
    n : UDINT;
    base : UDINT;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        i := 0;
        WHILE i < iRowsToWrite AND NOT bError DO
            n := iRowsToWrite - i;
            IF n > 32 THEN n := 32; END_IF;
            ch.OP := 3;
            IF bOverwrite THEN ch.OP := 4; END_IF;
            ch.CHAN := iChannelId;
            ch.NROWS := n;
            base := i * 8;
            FOR r := 0 TO 255 DO
                ch.ROWS[r] := 0.0;
            END_FOR;
            FOR r := 0 TO n * 8 - 1 DO
                ch.ROWS[r] := AdrDataArray[base + r];
            END_FOR;
            ch();
            IF ch.STATUS <> 0 THEN
                bError := TRUE;
                nErrorId := ch.ERRID;
            END_IF;
            i := i + n;
        END_WHILE;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FiFoWrite
VAR_INPUT
    iChannelId : UDINT;
    AdrDataArray : ARRAY[0..255] OF LREAL;
    iRowsToWrite : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    pump : FiFoWritePump;
END_VAR
    pump.iChannelId := iChannelId;
    pump.AdrDataArray := AdrDataArray;
    pump.iRowsToWrite := iRowsToWrite;
    pump.bOverwrite := FALSE;
    pump.bExecute := bExecute;
    pump.tTimeout := tTimeout;
    pump();
    bBusy := pump.bBusy;
    bError := pump.bError;
    nErrorId := pump.nErrorId;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FiFoOverwrite
VAR_INPUT
    iChannelId : UDINT;
    AdrDataArray : ARRAY[0..255] OF LREAL;
    iRowsToWrite : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    pump : FiFoWritePump;
END_VAR
    pump.iChannelId := iChannelId;
    pump.AdrDataArray := AdrDataArray;
    pump.iRowsToWrite := iRowsToWrite;
    pump.bOverwrite := TRUE;
    pump.bExecute := bExecute;
    pump.tTimeout := tTimeout;
    pump();
    bBusy := pump.bBusy;
    bError := pump.bError;
    nErrorId := pump.nErrorId;
END_FUNCTION_BLOCK

// ---- FiFoStart: begin streaming queued rows ----
FUNCTION_BLOCK FiFoStart
VAR_INPUT
    iChannelId : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ch : FB_Fifo;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        ch.OP := 5;
        ch.CHAN := iChannelId;
        ch.AUX := 1;
        ch();
        IF ch.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FiFoStop: halt the channel (queued rows kept) ----
FUNCTION_BLOCK FiFoStop
VAR_INPUT
    iChannelId : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ch : FB_Fifo;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        ch.OP := 5;
        ch.CHAN := iChannelId;
        ch.AUX := 2;
        ch();
        IF ch.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FiFoSetChannelOverride: override percent (recorded) ----
FUNCTION_BLOCK FiFoSetChannelOverride
VAR_INPUT
    iChannelId : UDINT;
    iOverride : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ch : FB_Fifo;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        ch.OP := 7;
        ch.CHAN := iChannelId;
        ch.VAL := UDINT_TO_LREAL(iOverride);
        ch();
        IF ch.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

FUNCTION F_GetVersionTcFifoAxes : STRING[32]
    F_GetVersionTcFifoAxes := 'Tc2_NcFifoAxes 1.0';
END_FUNCTION
