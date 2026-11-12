// tc2_mbus.st — wired M-Bus (EN 13757) metering-master surface
// (TwinCAT Tc3_MBus/TF6681-ish shape) over the tplc-mbus service +
// FB_Mbus builtin.
//
// The PLC is the M-Bus master over an opt-in serial port (2400 8E1):
// FB_MbusOpen opens the UART; FB_MbusShort carries the short frames
// (SND_NKE 0x40, REQ_UD2 0x5B, REQ_UD1 0x7B); FB_MbusLong carries
// long frames (SND_UD etc.); FB_MbusReceive polls for a response
// (E5 ack or a 68.. long frame); FB_MbusStatus reports counters.
//
// Payload fields are raw bytes bounded STRING[255] — the caller
// composes/parses the M-Bus DIF/VIF record.  Secondary addressing,
// the application-layer record decode, and the collision/FDB
// handling a full master does are honest gaps for a later tranche.

// ---- FB_MbusOpen: open the serial port (2400 8E1) ----
FUNCTION_BLOCK FB_MbusOpen
VAR_INPUT
    Execute : BOOL;
    Path : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    PortHdl : UDINT;
END_VAR
VAR
    ch : FB_Mbus;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.PATH := Path;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            PortHdl := ch.HANDLE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_MbusClose: release the serial port ----
FUNCTION_BLOCK FB_MbusClose
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mbus;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 2;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_MbusShort: short frame (SND_NKE/REQ_UD2/REQ_UD1) ----
FUNCTION_BLOCK FB_MbusShort
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    Control : UDINT;
    Address : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mbus;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.CONTROL := Control;
        ch.ADDRESS := Address;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_MbusLong: long frame (SND_UD etc.) ----
FUNCTION_BLOCK FB_MbusLong
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    Control : UDINT;
    Address : UDINT;
    Ci : UDINT;
    Data : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mbus;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.HANDLE := PortHdl;
        ch.CONTROL := Control;
        ch.ADDRESS := Address;
        ch.CI := Ci;
        ch.DATA := Data;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_MbusReceive: poll for a response (E5 / 68..) ----
FUNCTION_BLOCK FB_MbusReceive
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Message : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Kind : UDINT;
    Payload : STRING[255];
END_VAR
VAR
    ch : FB_Mbus;
END_VAR
    Done := FALSE;
    Message := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 6;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 1 THEN
            Done := TRUE;               // NOMSG: still waiting
        ELSIF ch.STATUS >= 0 THEN
            Done := TRUE;
            Message := TRUE;
            Kind := ch.KIND;
            Payload := ch.RXPAYLOAD;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_MbusStatus: frame counters + state ----
FUNCTION_BLOCK FB_MbusStatus
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    State : DINT;
    TxTotal : UDINT;
    RxTotal : UDINT;
    LastErr : DINT;
END_VAR
VAR
    ch : FB_Mbus;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 3;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            State := ch.STATE;
            TxTotal := ch.TXTOT;
            RxTotal := ch.RXTOT;
            LastErr := ch.LASTERR;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
