// Tc2_ModbusRTU Modbus RTU master FBs (TwinCAT 3 PLC Library
// Tc2_ModbusRTU, manual TF6255). Beckhoff-shaped wrappers over the
// builtin FB_ModbusRtu, which drives the host "tplc-modbus" service
// RTU transport (serial device opened per transaction, RTU framing
// unit|func|data|crc16, CRC-validated responses, nonblocking).
// TwinCAT exposes each Modbus function as an action on the
// ModbusRtuMasterV2_* FBs; here each action is a standalone FB with
// the same parameters. Differences from TwinCAT, documented in
// docs/plc/compat/README.md:
//   - POINTER TO BYTE payloads are ARRAY[0..255] OF BYTE buffers
//     (bounded; cbLength/cbSrcLength/cbDestLength still apply).
//   - serial line settings are explicit inputs (sDevice, nBaud,
//     nParity, nDataBits, nStopBits) instead of TwinCAT System
//     Manager COM-port bindings.
//   - nErrId carries tplc-modbus transport codes, or the Modbus
//     exception code when the slave returns an exception response.
//   - ModbusRtuSlave_* and the *_Generic ComBuffer variants are not
//     provided; KL6x*B bus-terminal variants need EtherCAT terminal
//     integration.
// Sources: docs/plc/plan/21-twincat-libraries.md §7 (TF6255).

FUNCTION_BLOCK FB_MBRtuReadCoils
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
END_VAR
    mb.FUNC := 1;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuReadInputs
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
END_VAR
    mb.FUNC := 2;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuReadRegs
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
END_VAR
    mb.FUNC := 3;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuReadInputRegs
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
END_VAR
    mb.FUNC := 4;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuWriteSingleCoil
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
END_VAR
    mb.FUNC := 5;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuWriteSingleReg
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
END_VAR
    mb.FUNC := 6;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuWriteCoils
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
    n : UDINT;
END_VAR
    mb.FUNC := 15;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuWriteRegs
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
    n : UDINT;
END_VAR
    mb.FUNC := 16;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuReadWriteRegs
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
    n : UDINT;
END_VAR
    mb.FUNC := 23;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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

FUNCTION_BLOCK FB_MBRtuDiagnose
VAR_INPUT
    sDevice : STRING[63];
    nBaud : UDINT := 9600;
    nParity : UDINT := 0;
    nDataBits : UDINT := 8;
    nStopBits : UDINT := 1;
    nUnitID : BYTE := 1;
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
    mb : FB_ModbusRtu;
    i : UDINT;
END_VAR
    mb.FUNC := 8;
    mb.DEV := sDevice;
    mb.BAUD := nBaud;
    mb.PARITY := nParity;
    mb.DBITS := nDataBits;
    mb.SBITS := nStopBits;
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
