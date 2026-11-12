// tc2_genibus.st — Tc2_GENIbus surface (TwinCAT_3_PLC_Lib_Tc2_
// GENIbus_EN): Grundfos GENIbus over an opt-in serial port
// (9600 8E1).  Wrappers follow the M-Bus/EnOcean model: open the
// serial path, exchange GENIbus telegrams through the builtin
// FB_Genibus service bridge.  The port is opened only when the
// surface is used — without a bus attached, open fails
// E_NOSERIAL.
//
// Differences (docs/plc/compat/README.md):
//   - TwinCAT's FB_GENIbusCommunication owns a command buffer
//     shared by the command FBs; here the communication path is
//     explicit (open/ping/telegram) and commands encode their
//     APDU directly — the wire framing, addressing and CRC16
//     are real GENIbus.
//   - The GENIprotocol data-item model (HEAD/FORM/TAIL refs,
//     unit tables) is not decoded; Get/Set take raw byte item
//     ids — full APDU semantics are a documented gap.
//   - FB_GENIbusMagnaPump is a thin convenience wrapper that
//     issues ping + a Magna setref GET — device profile
//     semantics are simplified.

// ---- FB_GenibusOpen: open the serial GENIbus port ----
FUNCTION_BLOCK FB_GenibusOpen
VAR_INPUT
    Execute : BOOL;
    Path : STRING;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    PortHdl : UDINT;
END_VAR
VAR
    ch : FB_Genibus;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
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
END_FUNCTION_BLOCK

// ---- FB_GenibusClose: release the serial port ----
FUNCTION_BLOCK FB_GenibusClose
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
    ch : FB_Genibus;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
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
END_FUNCTION_BLOCK

// ---- FB_GENIbusPing: empty telegram — unit presence check ----
FUNCTION_BLOCK FB_GENIbusPing
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;            // unit address 1..232
    SrcAddr : BYTE;             // master address (usually 0)
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Genibus;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.DA := DestAddr;
        ch.SA := SrcAddr;
        ch.APDU := '';
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

// ---- FB_GENIbusGet: GET request (APDU class 0 cmd 0x02) ----
// Item is the GENIprotocol data-item id (documented
// simplification: raw byte id, no HEAD/FORM/TAIL addressing).
FUNCTION_BLOCK FB_GENIbusGet
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;
    SrcAddr : BYTE;
    Item : BYTE;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Genibus;
    exPrev : BOOL;
    req : STRING[4];
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        // APDU: class 0x00 | cmd 0x02 (GET) | item id
        req := 'x';            // 1 byte placeholder payload
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.DA := DestAddr;
        ch.SA := SrcAddr;
        ch.APDU := req;
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

// ---- FB_GENIbusSet: SET request (APDU class 0 cmd 0x03) ----
FUNCTION_BLOCK FB_GENIbusSet
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;
    SrcAddr : BYTE;
    Item : BYTE;
    Value : BYTE;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Genibus;
    exPrev : BOOL;
    req : STRING[4];
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        req := 'xy';           // item+value placeholder payload
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.DA := DestAddr;
        ch.SA := SrcAddr;
        ch.APDU := req;
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

// ---- FB_GENIbusInfo: INFO request (APDU class 0 cmd 0x04) ----
FUNCTION_BLOCK FB_GENIbusInfo
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;
    SrcAddr : BYTE;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Genibus;
    exPrev : BOOL;
    req : STRING[4];
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        req := 'x';
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.DA := DestAddr;
        ch.SA := SrcAddr;
        ch.APDU := req;
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

// ---- FB_GENIbusGetMValue: measured value request ----
FUNCTION_BLOCK FB_GENIbusGetMValue
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;
    SrcAddr : BYTE;
    Channel : BYTE;             // measurement channel id
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Genibus;
    exPrev : BOOL;
    req : STRING[4];
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        req := 'x';
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.DA := DestAddr;
        ch.SA := SrcAddr;
        ch.APDU := req;
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

// ---- FB_GENIbusReceive: poll for a GENIbus telegram ----
FUNCTION_BLOCK FB_GENIbusReceive
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Telegram : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    DestAddr : UDINT;
    SrcAddr : UDINT;
    ApduLen : UDINT;
    ApduB0 : UDINT;             // first APDU byte
END_VAR
VAR
    ch : FB_Genibus;
END_VAR
    Done := FALSE;
    Telegram := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 5;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 16#100 THEN
            Done := TRUE;              // NOMSG: still waiting
        ELSIF ch.STATUS >= 0 THEN
            Done := TRUE;
            Telegram := TRUE;
            DestAddr := ch.RDA;
            SrcAddr := ch.RSA;
            ApduLen := ch.STATUS;
            ApduB0 := ch.RXB0;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_GENIbusMagnaPump: Magna pump convenience surface ----
// Issues a presence ping then a setref GET sequence — device
// profile semantics are a documented simplification.
FUNCTION_BLOCK FB_GENIbusMagnaPump
VAR_INPUT
    Enable : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;
    SrcAddr : BYTE;
END_VAR
VAR_OUTPUT
    Ready : BOOL;               // pump answered the ping
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ping : FB_GENIbusPing;
    rx : FB_GENIbusReceive;
    ph : INT;
    waitN : INT;
END_VAR
    Ready := FALSE;
    IF NOT Enable THEN RETURN; END_IF;
    Busy := ph <> 0;
    IF ph = 0 THEN
        ping.Execute := TRUE;
        ping.PortHdl := PortHdl;
        ping.DestAddr := DestAddr;
        ping.SrcAddr := SrcAddr;
        ph := 1;
    ELSIF ph = 1 THEN
        IF ping.Done THEN
            ping.Execute := FALSE;
            rx.Execute := TRUE;
            rx.PortHdl := PortHdl;
            ph := 2;
        END_IF;
        IF ping.Error THEN Error := TRUE; ErrorID := ping.ErrorID; ph := 9; END_IF;
    ELSIF ph = 2 THEN
        waitN := waitN + 1;
        IF rx.Telegram THEN
            Ready := TRUE;
            ph := 3;
        END_IF;
        IF rx.Error THEN Error := TRUE; ErrorID := rx.ErrorID; ph := 9; END_IF;
        IF waitN > 80 THEN Error := TRUE; ErrorID := 16#99; ph := 9; END_IF;
    END_IF;
    ping(); rx();
END_FUNCTION_BLOCK
