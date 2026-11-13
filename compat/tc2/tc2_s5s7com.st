// Tc2_S5S7Com -> Siemens S5 network-link wrappers (RK512 over
// TCP, 3964 STX/ETX+BCC framing) plus the S7 side already covered
// by Tc3_S7Comm (compat/tc3/tc3_s7comm.st). TwinCAT's
// FB_NLinkOpen/FB_NLinkFetch/FB_NLinkWrite/FB_NLinkClose take a
// T_HNLINK in-out handle, an E_NLinkArea selector, devAddr and a
// POINTER TO BYTE buffer; TetherPLC adapts them to the
// poll-driven service model: FB_S5Conn holds the TCP session and
// FB_S5Io runs one RK512 FETCH / SEND transaction per EXEC edge.
// Differences from the TwinCAT FBs, documented in
// docs/plc/compat/README.md and docs/plc/compat/tc2-s5s7com.md:
//   - hLink is a UDINT connection handle from FB_NLinkOpen;
//     nArea accepts only the DB value (eNLinkArea_DB); other RK512
//     areas (I/O, memory, flags, timers, counters) are not
//     implemented.
//   - POINTER TO BYTE buffers are ARRAY[0..509] OF BYTE; the
//     wire transfers 16-bit words (max 255 words = 510 bytes per
//     transaction) and nOffset/nDB are byte/number fields the
//     service converts to word addressing.
//   - nStatus: conn 0=pending 1=ready 2=error; io 0=pending
//     1=ok 2=timeout 3=error 4=badresp 5=s5err. nErrId carries
//     the transport errno or RK512 status byte.
//   - One outstanding transaction per connection; no continuation
//     telegrams.
// Sources: docs/plc/plan/21-twincat-libraries.md (Tc2_S5S7Com).

// Open/poll/close an S5 connection — covers FB_NLinkOpen and
// FB_NLinkClose.
FUNCTION_BLOCK FB_NLinkOpen
VAR_INPUT
    sHost : STRING[15];          // dotted-quad PLC IPv4
    nPort : UDINT := 0;          // 0 = 102
    bExecute : BOOL;
    bClose : BOOL;
    tTimeout : TIME := T#45S;
END_VAR
VAR_OUTPUT
    hLink : UDINT;               // 0 = no live session
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrID : UDINT;
    nStatus : DINT;              // 0=pending 1=ready 2=error
END_VAR
VAR
    conn : FB_S5Conn;
END_VAR
    conn.HOST := sHost;
    conn.PORT := nPort;
    conn.TIMEOUT := tTimeout;
    conn.CLOSE := bClose;
    conn.EXEC := bExecute;
    conn();
    hLink := conn.CONN_HANDLE;
    bBusy := conn.BUSY;
    bDone := conn.DONE;
    nStatus := conn.STATUS;
    nErrID := conn.ERRID;
    bError := (conn.ERRID <> 0) OR (conn.STATUS = 2);
END_FUNCTION_BLOCK

// Read DB bytes (RK512 FETCH) — covers FB_NLinkFetch.
FUNCTION_BLOCK FB_NLinkFetch
VAR_INPUT
    hLink : UDINT;               // from FB_NLinkOpen
    nArea : UDINT := 1;          // only eNLinkArea_DB (=1) is valid
    nDB : UDINT := 0;            // DB number 0..255
    nOffset : UDINT;             // byte offset in the DB, even, <=510
    nLen : UDINT;                // bytes, 1..510
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrID : UDINT;
    nStatus : DINT;              // 0=pending 1=ok 2=timeout
                                  // 3=error 4=badresp 5=s5err
    cbRead : UDINT;              // bytes returned
    pBuffer : ARRAY[0..509] OF BYTE;
END_VAR
VAR
    io : FB_S5Io;
    i : UDINT;
END_VAR
    io.CONN_HANDLE := hLink;
    io.MODE := 0;
    io.DB := nDB;
    io.BYTE_OFF := nOffset;
    io.LEN := nLen;
    io.TIMEOUT := tTimeout;
    io.EXEC := bExecute;
    io();
    bBusy := io.BUSY;
    bDone := io.DONE;
    nStatus := io.STATUS;
    nErrID := io.ERRID;
    cbRead := io.DATA_LEN;
    IF io.DONE THEN
        i := 0;
        WHILE i < io.DATA_LEN AND i < 510 DO
            pBuffer[i] := io.R_DATA[i];
            i := i + 1;
        END_WHILE;
    END_IF;
    bError := (io.ERRID <> 0) OR (io.STATUS >= 2) OR (nArea <> 1);
END_FUNCTION_BLOCK

// Write DB bytes (RK512 SEND) — covers FB_NLinkWrite.
FUNCTION_BLOCK FB_NLinkWrite
VAR_INPUT
    hLink : UDINT;
    nArea : UDINT := 1;          // only eNLinkArea_DB (=1) is valid
    nDB : UDINT := 0;
    nOffset : UDINT;             // byte offset in the DB, even, <=510
    nLen : UDINT;                // bytes, 1..510
    pData : ARRAY[0..509] OF BYTE;
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrID : UDINT;
    nStatus : DINT;
    cbWrite : UDINT;             // bytes accepted
END_VAR
VAR
    io : FB_S5Io;
    i : UDINT;
END_VAR
    io.CONN_HANDLE := hLink;
    io.MODE := 1;
    io.DB := nDB;
    io.BYTE_OFF := nOffset;
    io.LEN := nLen;
    io.TIMEOUT := tTimeout;
    i := 0;
    WHILE i < 510 DO
        io.DATA[i] := pData[i];
        i := i + 1;
    END_WHILE;
    io.EXEC := bExecute;
    io();
    bBusy := io.BUSY;
    bDone := io.DONE;
    nStatus := io.STATUS;
    nErrID := io.ERRID;
    cbWrite := 0;
    IF io.DONE AND io.STATUS = 1 THEN
        cbWrite := nLen;
    END_IF;
    bError := (io.ERRID <> 0) OR (io.STATUS >= 2) OR (nArea <> 1);
END_FUNCTION_BLOCK

// Library version query — covers F_GetVersionTcS5S7Comm.
FUNCTION F_GetVersionTcS5S7Comm : UINT
VAR_INPUT
    END_VAR
    F_GetVersionTcS5S7Comm := 16#0100;
END_FUNCTION
