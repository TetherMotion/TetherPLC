// tc2_aes70.st — TwinCAT audio-control client surface (Tc3_AES70 /
// TF6301-ish shape) over the tplc-aes70 service + FB_Aes70 builtin.
//
// The PLC is the OCA client over OSCP TCP (AES70/Open Control
// Architecture): FB_Aes70Connect opens the TCP session on the OCA
// control port (default 65000); FB_Aes70Call carries an OcaCommand
// (targetONo + methodID + parameter bytes);
// FB_Aes70Receive polls for a pending OcaResponse each scan;
// Keepalive sends an OSCP heartbeat; Disconnect releases.
//
// Parameters and response payloads are raw bytes bounded
// STRING[255] — the caller composes/parses the OCA-serialized
// parameter set.  Notification subscriptions, the full OCA class
// catalog, and property/get-set shorthands are out of scope for
// this tranche.

// ---- FB_Aes70Connect: TCP session on the OCA control port ----
FUNCTION_BLOCK FB_Aes70Connect
VAR_INPUT
    Execute : BOOL;
    Host : STRING[63];
    Port : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ConnectionHdl : UDINT;
END_VAR
VAR
    ch : FB_Aes70;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.HOST := Host;
        ch.PORT := Port;
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

// ---- FB_Aes70Disconnect: release the session ----
FUNCTION_BLOCK FB_Aes70Disconnect
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
    ch : FB_Aes70;
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

// ---- FB_Aes70Call: OcaCommand on an object ----
FUNCTION_BLOCK FB_Aes70Call
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    TargetONo : UDINT;
    MethodID : UDINT;
    Params : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RequestHdl : UDINT;
END_VAR
VAR
    ch : FB_Aes70;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := ConnectionHdl;
        ch.TARGET := TargetONo;
        ch.METH := MethodID;
        ch.PARAMS := Params;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            RequestHdl := ch.HANDLE2;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_Aes70Keepalive: OSCP heartbeat ----
FUNCTION_BLOCK FB_Aes70Keepalive
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Aes70;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
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
END_FUNCTION_BLOCK

// ---- FB_Aes70Receive: poll for a pending OcaResponse ----
FUNCTION_BLOCK FB_Aes70Receive
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Message : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RequestHdl : UDINT;
    StatusCode : UDINT;
    Payload : STRING[255];
END_VAR
VAR
    ch : FB_Aes70;
END_VAR
    Done := FALSE;
    Message := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 6;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS >= 0 THEN
            Done := TRUE;
            Message := TRUE;
            RequestHdl := ch.HANDLE2;
            StatusCode := ch.STATUSCODE;
            Payload := ch.RXPAYLOAD;
        ELSIF ch.STATUS = 1 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
