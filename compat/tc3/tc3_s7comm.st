// Tc3_S7Comm + Tc2_S5S7Com -> Siemens S7comm client wrappers.
// TwinCAT's FB_S7CommConnection owns an AMS-routed session with
// I_S7CommRequest objects, PVOID buffers and HRESULT returns;
// Tc2_S5S7Com's FB_NLink* blocks map fetch/write requests onto
// T_HNLINK handles. TetherPLC adapts both to the poll-driven
// service model used by the other compat libraries: FB_S7Conn
// holds the persistent RFC1006/COTP session, FB_S7Io runs one
// Read Var / Write Var transaction per EXEC edge.
// Differences from the TwinCAT FBs, documented in
// docs/plc/compat/README.md:
//   - No AMS NetID routing, no PVOID/HRESULT/interface request
//     objects, no cyclic request queues (AddRequest* variants).
//     Requests are explicit poll-model FBs; one outstanding
//     transaction per connection.
//   - IPv4 dotted-quad hosts only, no DNS.
//   - nArea is the raw S7-Any area code: 16#81=PE(I), 16#82=PA(Q),
//     16#83=MK(M), 16#84=DB, 16#1C=CT, 16#1D=TM.
//   - nStatus: conn 0=pending 1=ready 2=error; io 0=pending
//     1=ok 2=timeout 3=error 4=badresp 5=s7err. nErrId carries
//     the transport errno or S7 error word; nItemRc the response
//     item return code (16#FF ok).
//   - Bit reads return one byte in pData[0] with the bit value
//     right-justified (bit 0); bit writes take pData[0]&1.
// Sources: docs/plc/plan/21-twincat-libraries.md (Tc2_S5S7Com,
// Tc3_S7Comm).

// Open/poll/close a persistent S7 connection — covers
// FB_S7CommConnection and the FB_NLinkOpen/FB_NLinkClose pair.
FUNCTION_BLOCK FB_S7_Connect
VAR_INPUT
    sHost : STRING[15];          // dotted-quad PLC IPv4
    nPort : UDINT := 0;          // 0 = 102
    nRack : UDINT := 0;          // 0..15
    nSlot : UDINT := 2;          // 0..31
    bExecute : BOOL;             // rising edge = (re)connect
    bClose : BOOL;               // rising edge = disconnect
    tTimeout : TIME := T#5S;
END_VAR
VAR_OUTPUT
    nConnHandle : UDINT;         // 0 = no live session
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;              // 0=pending 1=ready 2=error
    nErrId : UDINT;
END_VAR
VAR
    conn : FB_S7Conn;
END_VAR
    conn.HOST := sHost;
    conn.PORT := nPort;
    conn.RACK := nRack;
    conn.SLOT := nSlot;
    conn.TIMEOUT := tTimeout;
    conn.CLOSE := bClose;
    conn.EXEC := bExecute;
    conn();
    nConnHandle := conn.CONN_HANDLE;
    bBusy := conn.BUSY;
    bDone := conn.DONE;
    nStatus := conn.STATUS;
    nErrId := conn.ERRID;
    bError := (conn.ERRID <> 0) OR (conn.STATUS = 2);
END_FUNCTION_BLOCK

// Read bytes — covers FB_S7CommCyclicRequest/AddReadVar/
// AddReadByteArray and FB_NLinkFetch.
FUNCTION_BLOCK FB_S7_Read
VAR_INPUT
    nConnHandle : UDINT;         // from FB_S7_Connect
    nArea : UDINT;               // S7-Any area code
    nDb : UDINT := 0;            // DB number (DB area only)
    nByteOff : UDINT;            // byte offset (24-bit)
    nLen : UDINT;                // bytes, 1..240
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;              // 0=pending 1=ok 2=timeout 3=err
                                  // 4=badresp 5=s7err
    nItemRc : UDINT;             // response item return code
    nDataLen : UDINT;            // valid bytes in pData
    pData : ARRAY[0..239] OF BYTE;
END_VAR
VAR
    io : FB_S7Io;
    i : UDINT;
END_VAR
    io.CONN_HANDLE := nConnHandle;
    io.MODE := 0;
    io.AREA := nArea;
    io.DB := nDb;
    io.BYTE_OFF := nByteOff;
    io.BIT_ACCESS := FALSE;
    io.BIT_NO := 0;
    io.LEN := nLen;
    io.TIMEOUT := tTimeout;
    io.EXEC := bExecute;
    io();
    bBusy := io.BUSY;
    bDone := io.DONE;
    nStatus := io.STATUS;
    nErrId := io.ERRID;
    nItemRc := io.ITEM_RC;
    nDataLen := io.DATA_LEN;
    IF io.DONE THEN
        i := 0;
        WHILE i < io.DATA_LEN AND i < 240 DO
            pData[i] := io.R_DATA[i];
            i := i + 1;
        END_WHILE;
    END_IF;
    bError := (io.ERRID <> 0) OR (io.STATUS >= 2);
END_FUNCTION_BLOCK

// Write bytes — covers AddWriteVar/AddWriteByteArray and
// FB_NLinkWrite.
FUNCTION_BLOCK FB_S7_Write
VAR_INPUT
    nConnHandle : UDINT;
    nArea : UDINT;
    nDb : UDINT := 0;
    nByteOff : UDINT;
    nLen : UDINT;                // bytes, 1..240
    pData : ARRAY[0..239] OF BYTE;
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nItemRc : UDINT;
END_VAR
VAR
    io : FB_S7Io;
    i : UDINT;
END_VAR
    io.CONN_HANDLE := nConnHandle;
    io.MODE := 1;
    io.AREA := nArea;
    io.DB := nDb;
    io.BYTE_OFF := nByteOff;
    io.BIT_ACCESS := FALSE;
    io.BIT_NO := 0;
    io.LEN := nLen;
    io.TIMEOUT := tTimeout;
    i := 0;
    WHILE i < nLen AND i < 240 DO
        io.DATA[i] := pData[i];
        i := i + 1;
    END_WHILE;
    io.EXEC := bExecute;
    io();
    bBusy := io.BUSY;
    bDone := io.DONE;
    nStatus := io.STATUS;
    nErrId := io.ERRID;
    nItemRc := io.ITEM_RC;
    bError := (io.ERRID <> 0) OR (io.STATUS >= 2);
END_FUNCTION_BLOCK

// Read one bit — covers AddReadBit. nBitNo 0..7 selects the bit
// inside nByteOff; pData[0] bit 0 holds the value when done.
FUNCTION_BLOCK FB_S7_ReadBit
VAR_INPUT
    nConnHandle : UDINT;
    nArea : UDINT;
    nDb : UDINT := 0;
    nByteOff : UDINT;
    nBitNo : UDINT;              // 0..7
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nItemRc : UDINT;
    nBitValue : UDINT;           // 0 or 1
END_VAR
VAR
    io : FB_S7Io;
END_VAR
    io.CONN_HANDLE := nConnHandle;
    io.MODE := 0;
    io.AREA := nArea;
    io.DB := nDb;
    io.BYTE_OFF := nByteOff;
    io.BIT_ACCESS := TRUE;
    io.BIT_NO := nBitNo;
    io.LEN := 1;
    io.TIMEOUT := tTimeout;
    io.EXEC := bExecute;
    io();
    bBusy := io.BUSY;
    bDone := io.DONE;
    nStatus := io.STATUS;
    nErrId := io.ERRID;
    nItemRc := io.ITEM_RC;
    IF io.DONE AND io.STATUS = 1 THEN
        nBitValue := BYTE_TO_UDINT(io.R_DATA[0]) AND 16#01;
    ELSE
        nBitValue := 0;
    END_IF;
    bError := (io.ERRID <> 0) OR (io.STATUS >= 2);
END_FUNCTION_BLOCK

// Write one bit — covers the AddWrite* bit variants.
// nBitValue 0 clears, nonzero sets.
FUNCTION_BLOCK FB_S7_WriteBit
VAR_INPUT
    nConnHandle : UDINT;
    nArea : UDINT;
    nDb : UDINT := 0;
    nByteOff : UDINT;
    nBitNo : UDINT;              // 0..7
    nBitValue : UDINT;           // 0 or 1
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nItemRc : UDINT;
END_VAR
VAR
    io : FB_S7Io;
END_VAR
    io.CONN_HANDLE := nConnHandle;
    io.MODE := 1;
    io.AREA := nArea;
    io.DB := nDb;
    io.BYTE_OFF := nByteOff;
    io.BIT_ACCESS := TRUE;
    io.BIT_NO := nBitNo;
    io.LEN := 1;
    io.TIMEOUT := tTimeout;
    io.DATA[0] := UDINT_TO_BYTE(nBitValue AND 16#01);
    io.EXEC := bExecute;
    io();
    bBusy := io.BUSY;
    bDone := io.DONE;
    nStatus := io.STATUS;
    nErrId := io.ERRID;
    nItemRc := io.ITEM_RC;
    bError := (io.ERRID <> 0) OR (io.STATUS >= 2);
END_FUNCTION_BLOCK
