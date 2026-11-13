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

// ---- FB_Iec61850RCBGet: MMS GetRCBValues on an RCB reference ----
// RcbRef is the MMS object reference of the report control block,
// e.g. 'tetherIEDtetherTestLD/LLN0$BR$MeasURCB'.
FUNCTION_BLOCK FB_Iec61850RCBGet
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    RcbRef : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RptId : STRING[31];
    DataSet : STRING[63];
    ConfRev : UDINT;
    BufTm : UDINT;
    IntgPd : UDINT;
    SqNum : UDINT;
    OptFlds : UDINT;
    TrgOps : UDINT;
    RptEna : BOOL;
    Resv : BOOL;
    Buffered : BOOL;
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 7;
        ch.HANDLE := ConnectionHdl;
        ch.RCBREF := RcbRef;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            RptId := ch.RPTID;
            DataSet := ch.DATASET;
            ConfRev := ch.CONFREV;
            BufTm := ch.BUFTM;
            IntgPd := ch.INTGPD;
            SqNum := ch.SQNUM;
            OptFlds := ch.OPTFLDS;
            TrgOps := ch.TRGOPS;
            RptEna := ch.RPTENA <> 0;
            Resv := ch.RESV <> 0;
            Buffered := ch.BUFFERED <> 0;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_Iec61850RCBSet: MMS SetRCBValues (URCB-safe subset) ----
// Flags: bit0 RptEna, bit1 Resv, bit2 GI, bit3 PurgeBuf (buffered
// RCBs only).  Once the RCB is enabled the service narrows the write
// to the attributes servers accept in that state (RptEna disable,
// GI, PurgeBuf); a redundant enable is a no-op.  Enabling installs
// the report handler on the connection.
FUNCTION_BLOCK FB_Iec61850RCBSet
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    RcbRef : STRING[127];
    Flags : UDINT;
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
        ch.OP := 8;
        ch.HANDLE := ConnectionHdl;
        ch.RCBREF := RcbRef;
        ch.FLAGS := Flags;
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

// ---- FB_Iec61850ReportPoll: drain received reports ----
// Report holds newline-separated "dataset|v0,v1,..." lines received
// since the last poll; ReportBytes is the drained byte count.
FUNCTION_BLOCK FB_Iec61850ReportPoll
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Report : STRING[255];
    ReportBytes : UDINT;
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    Report := '';
    ReportBytes := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 9;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS >= 0 THEN
            Done := TRUE;
            Report := ch.VAL;
            ReportBytes := ch.RCNT;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_Iec61850Control: controllable data object command ----
// ObjectRef is the "domain/LN.DO" reference (e.g.
// "tetherTestLD/LLN0.SWI1").  Mode: 1=Select, 2=SelectWithValue,
// 3=Operate, 4=Cancel.  CtlValue is the ASCII control value
// ("true"/"false", integer or float text) for SelectWithValue and
// Operate.  Command termination arrives asynchronously — poll it
// with FB_Iec61850ControlPoll.
FUNCTION_BLOCK FB_Iec61850Control
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    ObjectRef : STRING[127];
    Mode : UDINT;
    CtlValue : STRING[63];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Value : STRING[63];
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    Value := '';
    IF Execute AND NOT exPrev THEN
        ch.OP := 10;
        ch.HANDLE := ConnectionHdl;
        ch.CTRLREF := ObjectRef;
        ch.CMODE := Mode;
        ch.CTLVAL := CtlValue;
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

// ---- FB_Iec61850ControlPoll: drain a CommandTermination event ----
// Pending is TRUE when an event was returned: Success marks
// CommandTermination+ vs -, Type is 0=select 1=operate 2=cancel,
// CtlNum echoes the server's control number.
FUNCTION_BLOCK FB_Iec61850ControlPoll
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Pending : BOOL;
    InvokeID : UDINT;
    Success : BOOL;
    Err : UDINT;
    EvType : UDINT;
    CtlNum : UDINT;
END_VAR
VAR
    ch : FB_Iec61850;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    Pending := FALSE;
    InvokeID := 0;
    Success := FALSE;
    Err := 0;
    EvType := 0;
    CtlNum := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 11;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS >= 0 THEN
            Done := TRUE;
            Pending := ch.CTLPEND > 0;
            InvokeID := ch.CTLINVOKE;
            Success := ch.CTLSUCCESS > 0;
            Err := ch.CTLERR;
            EvType := ch.CTLTYPE;
            CtlNum := ch.CTLNUM;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK
