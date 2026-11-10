// Tc2_ModbusSrv Modbus TCP client FBs (TwinCAT 3 PLC Library
// Tc2_ModbusSrv). Beckhoff-shaped wrappers over the builtin
// FB_Modbus, which drives the host "tplc-modbus" service
// (Modbus TCP, nonblocking, connect-per-transaction).
// Differences from TwinCAT, documented in docs/plc/compat/README.md:
//   - POINTER TO BYTE payloads are ARRAY[0..255] OF BYTE buffers
//     (bounded; cbLength/cbSrcLength/cbDestLength still apply).
//   - sIPAddr is sHost (IPv4 dotted quad; no DNS).
//   - nErrId carries tplc-modbus transport codes, or the Modbus
//     exception code when the slave returns an exception response.
//   - Coil/register payloads are raw big-endian bytes; bit packing
//     for coil arrays is the caller's job, as in TwinCAT.
//   - UDP variants (FB_MBUdp*) and the RTU library are not provided.
// Sources: docs/plc/plan/21-twincat-libraries.md §7 (TF6250).

FUNCTION_BLOCK FB_MBReadCoils
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nQuantity : WORD;
    nMBAddr : WORD;
    cbLength : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
    pData : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
END_VAR
    mb.FUNC := 1;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nQuantity / 256);
    mb.DATA[3] := UINT_TO_BYTE(nQuantity MOD 256);
    mb.RLEN := 4;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    IF mb.DONE THEN
        i := 0;
        WHILE i < mb.DLEN AND i < 256 DO
            pData[i] := mb.DATA[i];
            i := i + 1;
        END_WHILE;
        cbRead := mb.DLEN;
    END_IF;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBReadInputs
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nQuantity : WORD;
    nMBAddr : WORD;
    cbLength : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
    pData : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
END_VAR
    mb.FUNC := 2;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nQuantity / 256);
    mb.DATA[3] := UINT_TO_BYTE(nQuantity MOD 256);
    mb.RLEN := 4;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    IF mb.DONE THEN
        i := 0;
        WHILE i < mb.DLEN AND i < 256 DO
            pData[i] := mb.DATA[i];
            i := i + 1;
        END_WHILE;
        cbRead := mb.DLEN;
    END_IF;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBReadRegs
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nQuantity : WORD;
    nMBAddr : WORD;
    cbLength : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
    pData : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
END_VAR
    mb.FUNC := 3;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nQuantity / 256);
    mb.DATA[3] := UINT_TO_BYTE(nQuantity MOD 256);
    mb.RLEN := 4;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    IF mb.DONE THEN
        i := 0;
        WHILE i < mb.DLEN AND i < 256 DO
            pData[i] := mb.DATA[i];
            i := i + 1;
        END_WHILE;
        cbRead := mb.DLEN;
    END_IF;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBReadInputRegs
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nQuantity : WORD;
    nMBAddr : WORD;
    cbLength : UDINT;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
    pData : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
END_VAR
    mb.FUNC := 4;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nQuantity / 256);
    mb.DATA[3] := UINT_TO_BYTE(nQuantity MOD 256);
    mb.RLEN := 4;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    IF mb.DONE THEN
        i := 0;
        WHILE i < mb.DLEN AND i < 256 DO
            pData[i] := mb.DATA[i];
            i := i + 1;
        END_WHILE;
        cbRead := mb.DLEN;
    END_IF;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBWriteSingleCoil
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nMBAddr : WORD;
    nValue : WORD;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
END_VAR
VAR
    mb : FB_Modbus;
END_VAR
    mb.FUNC := 5;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nValue / 256);
    mb.DATA[3] := UINT_TO_BYTE(nValue MOD 256);
    mb.RLEN := 4;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    cbRead := 0;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBWriteSingleReg
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nMBAddr : WORD;
    nValue : WORD;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
END_VAR
VAR
    mb : FB_Modbus;
END_VAR
    mb.FUNC := 6;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nValue / 256);
    mb.DATA[3] := UINT_TO_BYTE(nValue MOD 256);
    mb.RLEN := 4;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    cbRead := 0;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBWriteCoils
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nQuantity : WORD;
    nMBAddr : WORD;
    cbLength : UDINT;
    pSrcData : ARRAY[0..255] OF BYTE;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
    n : UDINT;
END_VAR
    mb.FUNC := 15;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nQuantity / 256);
    mb.DATA[3] := UINT_TO_BYTE(nQuantity MOD 256);
    n := cbLength;
    IF n > 246 THEN
        n := 246;
    END_IF;
    mb.DATA[4] := UDINT_TO_BYTE(n);
    i := 0;
    WHILE i < n DO
        mb.DATA[5 + i] := pSrcData[i];
        i := i + 1;
    END_WHILE;
    mb.RLEN := 5 + n;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    cbRead := 0;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBWriteRegs
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nQuantity : WORD;
    nMBAddr : WORD;
    cbLength : UDINT;
    pSrcData : ARRAY[0..255] OF BYTE;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
    n : UDINT;
END_VAR
    mb.FUNC := 16;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nQuantity / 256);
    mb.DATA[3] := UINT_TO_BYTE(nQuantity MOD 256);
    n := cbLength;
    IF n > 246 THEN
        n := 246;
    END_IF;
    mb.DATA[4] := UDINT_TO_BYTE(n);
    i := 0;
    WHILE i < n DO
        mb.DATA[5 + i] := pSrcData[i];
        i := i + 1;
    END_WHILE;
    mb.RLEN := 5 + n;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    cbRead := 0;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBReadWriteRegs
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nReadQuantity : WORD;
    nMBReadAddr : WORD;
    nWriteQuantity : WORD;
    nMBWriteAddr : WORD;
    cbDestLength : UDINT;
    cbSrcLength : UDINT;
    pSrcData : ARRAY[0..255] OF BYTE;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
    pDestData : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
    n : UDINT;
END_VAR
    mb.FUNC := 23;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nMBReadAddr / 256);
    mb.DATA[1] := UINT_TO_BYTE(nMBReadAddr MOD 256);
    mb.DATA[2] := UINT_TO_BYTE(nReadQuantity / 256);
    mb.DATA[3] := UINT_TO_BYTE(nReadQuantity MOD 256);
    mb.DATA[4] := UINT_TO_BYTE(nMBWriteAddr / 256);
    mb.DATA[5] := UINT_TO_BYTE(nMBWriteAddr MOD 256);
    mb.DATA[6] := UINT_TO_BYTE(nWriteQuantity / 256);
    mb.DATA[7] := UINT_TO_BYTE(nWriteQuantity MOD 256);
    n := cbSrcLength;
    IF n > 242 THEN
        n := 242;
    END_IF;
    mb.DATA[8] := UDINT_TO_BYTE(n);
    i := 0;
    WHILE i < n DO
        mb.DATA[9 + i] := pSrcData[i];
        i := i + 1;
    END_WHILE;
    mb.RLEN := 9 + n;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    IF mb.DONE THEN
        i := 0;
        WHILE i < mb.DLEN AND i < 256 DO
            pDestData[i] := mb.DATA[i];
            i := i + 1;
        END_WHILE;
        cbRead := mb.DLEN;
    END_IF;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_MBDiagnose
VAR_INPUT
    sHost : STRING[15];
    nTCPPort : UINT := 502;
    nUnitID : BYTE := 16#FF;
    nWriteData : WORD;
    bExecute : BOOL;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    cbRead : UDINT;
    pData : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    mb : FB_Modbus;
    i : UDINT;
END_VAR
    mb.FUNC := 8;
    mb.HOST := sHost;
    mb.PORT := nTCPPort;
    mb.UNIT := nUnitID;
    mb.TIMEOUT := tTimeout;
    mb.DATA[0] := UINT_TO_BYTE(nWriteData / 256);
    mb.DATA[1] := UINT_TO_BYTE(nWriteData MOD 256);
    mb.RLEN := 2;
    mb.EXEC := bExecute;
    mb();
    bBusy := mb.BUSY;
    bDone := mb.DONE;
    IF mb.DONE THEN
        i := 0;
        WHILE i < mb.DLEN AND i < 256 DO
            pData[i] := mb.DATA[i];
            i := i + 1;
        END_WHILE;
        cbRead := mb.DLEN;
    END_IF;
    nErrId := mb.ERRID;
    IF mb.RESULT = 1 THEN
        nErrId := 16#8000 + mb.EXC;
    END_IF;
    bError := (mb.ERRID <> 0) OR (mb.RESULT = 1);
END_FUNCTION_BLOCK
