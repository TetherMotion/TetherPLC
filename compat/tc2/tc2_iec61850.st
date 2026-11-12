// tc2_iec61850.st — TwinCAT IEC 61850 client surface (Tc3_IEC61850 /
// TF6101 shape) over the tplc-iec61850 service + FB_Iec61850 builtin.
//
// The PLC is the client (control-center side) over the full OSI
// stack via libiec61850: FB_Iec61850Connect runs TPKT/COTP/Session/
// Presentation/ACSE/MMS Initiate; FB_Iec61850Read/Write carry
// Confirmed-Request Read/Write on domain+item named variables;
// FB_Iec61850GetNameList lists a domain's objects; Disconnect
// releases.  Reporting/RCB, GOOSE/SMV, and the full data model are
// out of scope for this tranche.
//
// Object addressing is a domain (Logical Device) name + an item
// (VariableAccess) name — "tetherTestLD" + "LLN0$Health$stVal".
// Values travel as bounded STRING[255] (numerics as ASCII repr,
// structured values as their MmsValue string).  STATUS carries the
// service return code, ErrorID its positive form + the last MMS
// service error via FB_Iec61850Status.

// ---- FB_Iec61850Connect: TCP + OSI associate + MMS Initiate ----
FUNCTION_BLOCK FB_Iec61850Connect
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
    ConnectionHdl : UDINT;
END_VAR
VAR
    ch : FB_Iec61850;
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
            ConnectionHdl := ch.HANDLE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_Iec61850Disconnect: release the association ----
FUNCTION_BLOCK FB_Iec61850Disconnect
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 2;
        ch.HANDLE := ConnectionHdl;
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

// ---- FB_Iec61850Status: association status ----
FUNCTION_BLOCK FB_Iec61850Status
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ConnectionState : DINT;
    InvokeId : UDINT;
    LastMmsError : UDINT;
END_VAR
VAR
    ch : FB_Iec61850;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 3;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            ConnectionState := ch.STATE;
            InvokeId := ch.INVOKE;
            LastMmsError := ch.LASTERR;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_Iec61850Read: Confirmed-Request Read ----
FUNCTION_BLOCK FB_Iec61850Read
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    Domain : STRING[63];
    Item : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Value : STRING[255];
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := ConnectionHdl;
        ch.DOMAIN := Domain;
        ch.ITEM := Item;
        ch();
        IF ch.STATUS >= 0 THEN
            Done := TRUE;
            Value := ch.VAL;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_Iec61850Write: Confirmed-Request Write ----
FUNCTION_BLOCK FB_Iec61850Write
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    Domain : STRING[63];
    Item : STRING[127];
    Value : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.HANDLE := ConnectionHdl;
        ch.DOMAIN := Domain;
        ch.ITEM := Item;
        ch.VAL := Value;
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

// ---- FB_Iec61850GetNameList: MMS GetNameList on a domain ----
FUNCTION_BLOCK FB_Iec61850GetNameList
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    Domain : STRING[63];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    NameList : STRING[255];
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 6;
        ch.HANDLE := ConnectionHdl;
        ch.DOMAIN := Domain;
        ch();
        IF ch.STATUS >= 0 THEN
            Done := TRUE;
            NameList := ch.VAL;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK
