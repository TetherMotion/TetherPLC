// Tc2_SerialCom serial FBs (TwinCAT 3 PLC Library Tc2_SerialCom).
// Beckhoff-shaped wrappers over the builtin FB_Serial, which drives
// the host "tplc-serial" service (POSIX termios, nonblocking,
// deadline-bounded).
// TwinCAT's ComBuffer/SerialLineControl model keeps per-port state in
// a runtime-owned buffer object; this port uses explicit UDINT
// handles returned by FB_SerialOpen instead. Payload pointers
// (POINTER TO BYTE) are STRING[256] buffers.
// nParity: 0=none, 1=even, 2=odd. nStopBits: 1 or 2. nFlags is the
// TPLC_SERIAL_F_* bitmask (1=raw, 2=RS485, 4=noctty).
// Sources: docs/plc/plan/21-twincat-libraries.md §7 (TF6000).

FUNCTION_BLOCK FB_SerialOpen
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    sDevice : STRING[63];
    nBaud : UDINT;
    nParity : UDINT;
    nDataBits : UDINT;
    nStopBits : UDINT;
    nFlags : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    hPort : UDINT;
END_VAR
VAR
    ser : FB_Serial;
END_VAR
    ser.OP := 1;
    ser.DEV := sDevice;
    ser.BAUD := nBaud;
    ser.PARITY := nParity;
    ser.DBITS := nDataBits;
    ser.SBITS := nStopBits;
    ser.FLAGS := nFlags;
    ser.TIMEOUT := tTimeout;
    ser.EXEC := bExecute;
    ser();
    bBusy := ser.BUSY;
    bDone := ser.DONE;
    nErrId := ser.ERRID;
    hPort := ser.HANDLE;
    bError := ser.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SerialSend
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hPort : UDINT;
    pData : STRING[256];
    nLen : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nSent : DINT;
END_VAR
VAR
    ser : FB_Serial;
END_VAR
    ser.OP := 2;
    ser.HANDLE := hPort;
    ser.DATA := pData;
    ser.DLEN := nLen;
    ser.TIMEOUT := tTimeout;
    ser.EXEC := bExecute;
    ser();
    bBusy := ser.BUSY;
    bDone := ser.DONE;
    nErrId := ser.ERRID;
    nSent := ser.RESULT;
    bError := ser.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SerialReceive
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hPort : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    pData : STRING[256];
    nRecBytes : UDINT;
END_VAR
VAR
    ser : FB_Serial;
END_VAR
    ser.OP := 3;
    ser.HANDLE := hPort;
    ser.TIMEOUT := tTimeout;
    ser.EXEC := bExecute;
    ser();
    bBusy := ser.BUSY;
    bDone := ser.DONE;
    nErrId := ser.ERRID;
    pData := ser.DATA;
    nRecBytes := ser.DLEN;
    bError := ser.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SerialClose
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hPort : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ser : FB_Serial;
END_VAR
    ser.OP := 4;
    ser.HANDLE := hPort;
    ser.TIMEOUT := tTimeout;
    ser.EXEC := bExecute;
    ser();
    bBusy := ser.BUSY;
    bDone := ser.DONE;
    nErrId := ser.ERRID;
    bError := ser.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SerialFlush
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hPort : UDINT;
    nPurge : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ser : FB_Serial;
END_VAR
    ser.OP := 5;
    ser.HANDLE := hPort;
    ser.AUX := nPurge;
    ser.TIMEOUT := tTimeout;
    ser.EXEC := bExecute;
    ser();
    bBusy := ser.BUSY;
    bDone := ser.DONE;
    nErrId := ser.ERRID;
    bError := ser.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SerialRs485
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hPort : UDINT;
    bEnable : BOOL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ser : FB_Serial;
    en : UDINT;
END_VAR
    IF bEnable THEN
        en := 1;
    ELSE
        en := 0;
    END_IF
    ser.OP := 6;
    ser.HANDLE := hPort;
    ser.AUX := en;
    ser.TIMEOUT := tTimeout;
    ser.EXEC := bExecute;
    ser();
    bBusy := ser.BUSY;
    bDone := ser.DONE;
    nErrId := ser.ERRID;
    bError := ser.ERRID <> 0;
END_FUNCTION_BLOCK
