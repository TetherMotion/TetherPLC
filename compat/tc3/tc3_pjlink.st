// Tc3_PJLink -> JBMIA PJLink projector client wrappers.
// TwinCAT's FB_PJLink is an auto-managed connection object with
// EventLogger events and AMS NetID routing; TetherPLC adapts it
// to the poll-driven service model used by the other compat
// libraries (one transaction per EXEC edge, service-enforced
// timeout, connect-per-command).
// Differences from the TwinCAT FB, documented in
// docs/plc/compat/README.md:
//   - No persistent keep-alive session, no tStatusUpdateTime
//     polling, no sServerNetID routing (direct IPv4 only),
//     no I_TcMessage/HRESULT/EventLogger surfaces.
//   - PASSWORD drives the PJLink digest exchange automatically
//     when the projector greets `PJLINK 1 <nonce>`.
//   - PJLink "class" is the digit after '%' (standard commands
//     use class 1); nCmd is the 4-char body ("POWR","INPT",
//     "AVMT","ERST","INFO","NAME","INFM","LAMP", ...).
//   - nStatus mirrors TplcPjlinkRespInfo: 0=pending 1=ok
//     2=timeout 3=error 4=badresp 5=authfail. PJLink-level
//     command errors arrive as sValue="ERR<n>" with nStatus=1.
// Sources: docs/plc/plan/21-twincat-libraries.md (TF8020 PJLink).

FUNCTION_BLOCK FB_PJLink
VAR_INPUT
    sHost : STRING[15];          // dotted-quad projector IPv4
    nPort : UDINT := 4352;       // 0 = 4352
    nCls : UDINT := 1;           // PJLink class digit 0..9
    sCmd : STRING[4];            // 4-char command body
    sParam : STRING[128];        // "?" for queries
    sPassword : STRING[32];      // "" = no auth
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    bAuthed : BOOL;
    nRespCls : UDINT;
    nRespCmd : UDINT;            // 4-char body packed BE ASCII
    nValueLen : UDINT;           // full value length
    sValue : STRING[127];        // response value text
END_VAR
VAR
    pjl : FB_Pjlink;
END_VAR
    pjl.HOST := sHost;
    pjl.PORT := nPort;
    pjl.CLS := nCls;
    pjl.CMD := sCmd;
    pjl.PARAM := sParam;
    pjl.PASSWORD := sPassword;
    pjl.TIMEOUT := tTimeout;
    pjl.EXEC := bExecute;
    pjl();
    bBusy := pjl.BUSY;
    bDone := pjl.DONE;
    nStatus := pjl.STATUS;
    nErrId := pjl.ERRID;
    bAuthed := pjl.AUTHED;
    nRespCls := pjl.R_CLS;
    nRespCmd := pjl.R_CMD;
    nValueLen := pjl.R_VLEN;
    sValue := pjl.R_VALUE;
    bError := (pjl.ERRID <> 0) OR (pjl.STATUS >= 2);
END_FUNCTION_BLOCK

// Power on/off/query — TwinCAT PowerOn/PowerOff methods folded
// into one poll-model FB. nMode: 0=off 1=on 2=query.
FUNCTION_BLOCK FB_PJLink_Power
VAR_INPUT
    sHost : STRING[15];
    nPort : UDINT := 4352;
    sPassword : STRING[32] := '';
    nMode : UDINT;               // 0=off 1=on 2=query
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    bAuthed : BOOL;
    sValue : STRING[127];        // query: "0".."3" power state
END_VAR
VAR
    pjl : FB_PJLink;
END_VAR
    pjl.sHost := sHost;
    pjl.nPort := nPort;
    pjl.nCls := 1;
    pjl.sCmd := 'POWR';
    IF nMode = 2 THEN
        pjl.sParam := '?';
    ELSIF nMode = 1 THEN
        pjl.sParam := '1';
    ELSE
        pjl.sParam := '0';
    END_IF;
    pjl.sPassword := sPassword;
    pjl.tTimeout := tTimeout;
    pjl.bExecute := bExecute;
    pjl();
    bBusy := pjl.bBusy;
    bDone := pjl.bDone;
    bError := pjl.bError;
    nErrId := pjl.nErrId;
    nStatus := pjl.nStatus;
    bAuthed := pjl.bAuthed;
    sValue := pjl.sValue;
END_FUNCTION_BLOCK

// Input select/query — TwinCAT SetInputType/SetInputChannel
// methods. nMode: 0=set (sParam = input number, e.g. "31")
// 1=query.
FUNCTION_BLOCK FB_PJLink_Input
VAR_INPUT
    sHost : STRING[15];
    nPort : UDINT := 4352;
    sPassword : STRING[32] := '';
    nMode : UDINT;               // 0=set 1=query
    sInput : STRING[8];          // input number when nMode=0
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    bAuthed : BOOL;
    sValue : STRING[127];        // query: current input number
END_VAR
VAR
    pjl : FB_PJLink;
END_VAR
    pjl.sHost := sHost;
    pjl.nPort := nPort;
    pjl.nCls := 1;
    pjl.sCmd := 'INPT';
    IF nMode = 1 THEN
        pjl.sParam := '?';
    ELSE
        pjl.sParam := sInput;
    END_IF;
    pjl.sPassword := sPassword;
    pjl.tTimeout := tTimeout;
    pjl.bExecute := bExecute;
    pjl();
    bBusy := pjl.bBusy;
    bDone := pjl.bDone;
    bError := pjl.bError;
    nErrId := pjl.nErrId;
    nStatus := pjl.nStatus;
    bAuthed := pjl.bAuthed;
    sValue := pjl.sValue;
END_FUNCTION_BLOCK

// AV mute on/off/query — TwinCAT SetMute/QueryMute methods.
// PJLink AVMT: 30=mute off (open), 31=mute on (closed);
// query returns 11/21/30/31.
FUNCTION_BLOCK FB_PJLink_Mute
VAR_INPUT
    sHost : STRING[15];
    nPort : UDINT := 4352;
    sPassword : STRING[32] := '';
    nMode : UDINT;               // 0=mute on 1=mute off 2=query
    bExecute : BOOL;
    tTimeout : TIME := T#2S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    bAuthed : BOOL;
    sValue : STRING[127];        // query: "11"/"21"/"30"/"31"
END_VAR
VAR
    pjl : FB_PJLink;
END_VAR
    pjl.sHost := sHost;
    pjl.nPort := nPort;
    pjl.nCls := 1;
    pjl.sCmd := 'AVMT';
    IF nMode = 2 THEN
        pjl.sParam := '?';
    ELSIF nMode = 0 THEN
        pjl.sParam := '31';
    ELSE
        pjl.sParam := '30';
    END_IF;
    pjl.sPassword := sPassword;
    pjl.tTimeout := tTimeout;
    pjl.bExecute := bExecute;
    pjl();
    bBusy := pjl.bBusy;
    bDone := pjl.bDone;
    bError := pjl.bError;
    nErrId := pjl.nErrId;
    nStatus := pjl.nStatus;
    bAuthed := pjl.bAuthed;
    sValue := pjl.sValue;
END_FUNCTION_BLOCK
