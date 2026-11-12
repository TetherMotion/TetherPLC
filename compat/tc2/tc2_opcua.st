// tc2_opcua.st — TwinCAT OPC UA client surface (Tc2_OPCUA /
// Tc3_PlcOpenOpcUa shape) over the tplc-opcua service + FB_UaClient.
//
// The PLC is the OPC UA *client*: UA_Connect opens a session to an
// opc.tcp:// endpoint, UA_Read/UA_Write map node-id text
// ("ns=2;s=name", "i=85", "ns=1;i=1001") through the connection.
// The service is only present when libopen62541 was linked into the
// host — otherwise every block reports Error + ErrorID 99 (service
// absent).  For the simulated path the endpoint is Tether's
// tether_io_motion_server --opcua listener, which exposes the motion
// Registry as ns=2;s=<name> nodes.
//
// Coverage (honest subset of TwinCAT's UA_* surface):
//   FB_UA_Connect / FB_UA_Disconnect / FB_UA_ConnectGetStatus
//   FB_UA_Read / FB_UA_Write / FB_UA_GetNamespaceIndex
// Gaps (documented): browse, node-handle registration (UA_NodeGetHandle
// /Release), subscriptions, method calls, history.  Node ids are passed
// as text per call (no handle table) — TwinCAT resolves them once up
// front; the semantics are the same but without the handle round trip.
//
// Value model: numeric kinds widen into Value : LREAL; signed/unsigned
// 64-bit nodes lose precision above 2^53 (documented LREAL limit).
// String values land in StrValue : STRING[255].  ValueType is the
// TPLC_UA_TYPE_* code (1=BOOL 2=I64 3=U64 4=F64 5=STRING).

// ---- FB_UA_Connect: open a session to a UA server ----
// Rising edge on Execute opens a session to ServerUrl and returns a
// ConnectionHdl used by every other UA_* block. TwinCAT mirrors this
// as UA_Connect returning ConnectionHdl.
FUNCTION_BLOCK FB_UA_Connect
VAR_INPUT
    Execute : BOOL;
    ServerUrl : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ConnectionHdl : UDINT;
END_VAR
VAR
    ch : FB_UaClient;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.URL := ServerUrl;
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

// ---- FB_UA_Disconnect: close a session ----
FUNCTION_BLOCK FB_UA_Disconnect
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
    ch : FB_UaClient;
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

// ---- FB_UA_ConnectGetStatus: connection state ----
// Cyclic (call while Enable): STATE becomes TPLC_UA_CONN_*.
FUNCTION_BLOCK FB_UA_ConnectGetStatus
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ConnectionState : DINT;
END_VAR
VAR
    ch : FB_UaClient;
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
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_UA_Read: read a node's scalar value ----
// NodeId text ("ns=2;s=name", "i=85") -> Value/StrValue + ValueType.
FUNCTION_BLOCK FB_UA_Read
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    NodeId : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Value : LREAL;
    StrValue : STRING[255];
    ValueType : UDINT;
END_VAR
VAR
    ch : FB_UaClient;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := ConnectionHdl;
        ch.NODE := NodeId;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            Value := ch.VAL;
            StrValue := ch.SVAL;
            ValueType := ch.VARTYPE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_UA_Write: write a node's scalar value ----
// ValueType picks the wire kind: 1=BOOL 2=I64 3=U64 4=F64 5=STRING
// (StrValue used when 5, Value otherwise).
FUNCTION_BLOCK FB_UA_Write
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    NodeId : STRING[255];
    Value : LREAL;
    StrValue : STRING[255];
    ValueType : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_UaClient;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.HANDLE := ConnectionHdl;
        ch.NODE := NodeId;
        ch.VAL := Value;
        ch.SVAL := StrValue;
        ch.VARTYPE := ValueType;
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

// ---- FB_UA_GetNamespaceIndex: resolve a namespace URI ----
// NamespaceUri -> NamespaceIndex (for building ns=<idx>;s=... ids).
FUNCTION_BLOCK FB_UA_GetNamespaceIndex
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    NamespaceUri : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    NamespaceIndex : UDINT;
END_VAR
VAR
    ch : FB_UaClient;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 6;
        ch.HANDLE := ConnectionHdl;
        ch.URL := NamespaceUri;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            NamespaceIndex := ch.IDX;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK
