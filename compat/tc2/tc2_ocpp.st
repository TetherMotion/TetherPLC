// tc2_ocpp.st — OCPP-J charge-point client surface (TwinCAT
// Tc3_OCPP/TF6741-ish shape) over the tplc-ocpp service + FB_Ocpp
// builtin.
//
// The PLC is the charge-point client over OCPP-J: FB_OcppConnect runs
// the WebSocket upgrade (HTTP GET + Sec-WebSocket-Key + ocpp1.6
// subprotocol); FB_OcppCall carries a JSON-RPC call
// ([2,msgId,action,payload]); FB_OcppResult/CallError carry the
// reply frames; FB_OcppReceive polls for a pending message each
// scan.  Payloads are raw JSON text (bounded STRING[255]); the
// caller composes/parses them.  OCPP 2.x, security profiles, and
// per-action schema validation are out of scope for this tranche.

// ---- FB_OcppConnect: WS upgrade + session ----
FUNCTION_BLOCK FB_OcppConnect
VAR_INPUT
    Execute : BOOL;
    Host : STRING[63];
    Port : UDINT;
    Path : STRING[63];
    ChargePointId : STRING[63];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ConnectionHdl : UDINT;
END_VAR
VAR
    ch : FB_Ocpp;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.HOST := Host;
        ch.PORT := Port;
        ch.PATH := Path;
        ch.OCPPID := ChargePointId;
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

// ---- FB_OcppDisconnect: WS close + release ----
FUNCTION_BLOCK FB_OcppDisconnect
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
    ch : FB_Ocpp;
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

// ---- FB_OcppCall: OCPP-J call [2,msgId,action,payload] ----
FUNCTION_BLOCK FB_OcppCall
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    MessageId : STRING[63];
    ActName : STRING[63];
    Payload : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Ocpp;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := ConnectionHdl;
        ch.MSGID := MessageId;
        ch.ACT := ActName;
        ch.PAYLOAD := Payload;
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

// ---- FB_OcppResult: OCPP-J result [3,msgId,payload] ----
FUNCTION_BLOCK FB_OcppResult
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    MessageId : STRING[63];
    Payload : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Ocpp;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.HANDLE := ConnectionHdl;
        ch.MSGID := MessageId;
        ch.PAYLOAD := Payload;
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

// ---- FB_OcppCallError: OCPP-J error [4,msgId,code,desc,details] ----
FUNCTION_BLOCK FB_OcppCallError
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    MessageId : STRING[63];
    ErrorCode : STRING[63];
    Description : STRING[127];
    Details : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Ocpp;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 6;
        ch.HANDLE := ConnectionHdl;
        ch.MSGID := MessageId;
        ch.CODE := ErrorCode;
        ch.DESC := Description;
        ch.PAYLOAD := Details;
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

// ---- FB_OcppReceive: poll for a pending OCPP-J message ----
FUNCTION_BLOCK FB_OcppReceive
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Message : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    MessageType : UDINT;
    ActName : STRING[63];
    Payload : STRING[255];
END_VAR
VAR
    ch : FB_Ocpp;
END_VAR
    Done := FALSE;
    Message := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 7;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            Message := TRUE;
            MessageType := ch.RXMSGTYPE;
            ActName := ch.RXACTION;
            Payload := ch.RXPAYLOAD;
        ELSIF ch.STATUS = 1 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
