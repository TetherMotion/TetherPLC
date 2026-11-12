// tc2_eip.st — TwinCAT EtherNet/IP client surface (Tc3_EtherNetIP /
// TF6281 shape) over the tplc-eip service + FB_Eip builtin.
//
// The PLC is the scanner-side explicit-messaging client:
// FB_EipConnect opens TCP 44818 + RegisterSession, FB_EipGetAttribute /
// FB_EipSetAttribute carry unconnected CIP Get/Set_Attribute_Single
// inside SendRRData, FB_EipListIdentity issues the discovery command,
// FB_EipDisconnect unregisters.  Cyclic implicit (class-1) I/O is
// deliberately out of scope for this tranche.
//
// Object addressing is class/instance/attribute u16 — the codec picks
// the 8-bit or 16-bit path segment automatically.  Attribute payloads
// travel as bounded STRING bytes (TwinCAT uses PVOID buffers — the
// string is the practical carrier here).  STATUS carries the service
// return code, ErrorID its positive form + the last CIP general
// status via FB_EipStatus.

// ---- FB_EipConnect: open TCP + RegisterSession ----
FUNCTION_BLOCK FB_EipConnect
VAR_INPUT
    Execute : BOOL;
    RemoteHost : STRING[63];
    RemotePort : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    SessionHdl : UDINT;
END_VAR
VAR
    ch : FB_Eip;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.HOST := RemoteHost;
        ch.PORT := RemotePort;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            SessionHdl := ch.HANDLE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_EipDisconnect: unregister + close ----
FUNCTION_BLOCK FB_EipDisconnect
VAR_INPUT
    Execute : BOOL;
    SessionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Eip;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 2;
        ch.HANDLE := SessionHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_EipStatus: session status ----
FUNCTION_BLOCK FB_EipStatus
VAR_INPUT
    Execute : BOOL;
    SessionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    SessionState : DINT;
    SessionId : UDINT;
    LastCipStatus : UDINT;
END_VAR
VAR
    ch : FB_Eip;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 3;
        ch.HANDLE := SessionHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            SessionState := ch.STATE;
            SessionId := ch.SESSION;
            LastCipStatus := ch.LASTCIP;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_EipListIdentity: discovery ----
FUNCTION_BLOCK FB_EipListIdentity
VAR_INPUT
    Execute : BOOL;
    RemoteHost : STRING[63];
    RemotePort : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Identity : STRING[255];
END_VAR
VAR
    ch : FB_Eip;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HOST := RemoteHost;
        ch.PORT := RemotePort;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            Identity := ch.DATA;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_EipGetAttribute: CIP Get_Attribute_Single ----
FUNCTION_BLOCK FB_EipGetAttribute
VAR_INPUT
    Execute : BOOL;
    SessionHdl : UDINT;
    ClassId : UDINT;
    InstanceId : UDINT;
    AttributeId : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Value : STRING[255];
    ValueLen : UDINT;
END_VAR
VAR
    ch : FB_Eip;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.HANDLE := SessionHdl;
        ch.CLS := ClassId;
        ch.INST := InstanceId;
        ch.ATTR := AttributeId;
        ch();
        IF ch.STATUS >= 0 THEN
            Done := TRUE;
            Value := ch.DATA;
            ValueLen := ch.DATALEN;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_EipSetAttribute: CIP Set_Attribute_Single ----
FUNCTION_BLOCK FB_EipSetAttribute
VAR_INPUT
    Execute : BOOL;
    SessionHdl : UDINT;
    ClassId : UDINT;
    InstanceId : UDINT;
    AttributeId : UDINT;
    Value : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Eip;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 6;
        ch.HANDLE := SessionHdl;
        ch.CLS := ClassId;
        ch.INST := InstanceId;
        ch.ATTR := AttributeId;
        ch.DATA := Value;
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
