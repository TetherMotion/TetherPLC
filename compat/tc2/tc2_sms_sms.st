// Tc2_SMS + Tc2_SMTP -> SMS-over-GSM-modem and SMTP client wrappers.
// TwinCAT's FB_Smtp* blocks proxy through a TwinCAT SMTP server
// over ADS; TetherPLC adapts them to a direct RFC 5321 client on
// the tplc-smtp service (poll model, one transaction per EXEC
// edge, plain text only, AUTH LOGIN when USER/PASS are set).
// SendSMS drives a GSM modem through the Tc2_SerialCom wrappers
// (tplc-serial service) with an AT-command state machine in text
// mode (AT+CMGF=1, AT+CMGS, SUB-terminated body).
// Differences from the TwinCAT blocks, documented in
// docs/plc/compat/README.md:
//   - No ADS SMTP server proxy, no TLS/SSL, no attachments
//     (FB_SmtpAttach reports an unsupported error), no delivery
//     receipts, no multi-recipient envelopes (single RCPT TO),
//     no MIME headers beyond Subject/From/To/Date.
//   - SMS: PDU mode, concatenated SMS (UCS2), status reports and
//     delivery reports are not implemented; the modem must
//     support AT+CMGF=1 text mode. sText is limited to 160 chars
//     (one GSM-7 segment); longer texts are truncated by the
//     serial channel.
// Sources: docs/plc/plan/21-twincat-libraries.md (TF6350 SMS/SMTP).

// SMTP send without authentication — TwinCAT FB_SmtpV3 subset.
// nStatus mirrors TplcSmtpInfo: 0=pending 1=ok 2=fail(smtp reply
// code in nRespCode) 3=timeout 4=error 5=badresp.
FUNCTION_BLOCK FB_SmtpV3
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME := T#10S;
    sServer : STRING[15];          // dotted-quad IPv4
    nPort : UDINT := 25;           // 0 = 25
    sFrom : STRING[63];
    sTo : STRING[63];
    sSubject : STRING[63];
    sBody : STRING[255];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nRespCode : UDINT;
END_VAR
VAR
    smtp : FB_SmtpSend;
END_VAR
    smtp.HOST := sServer;
    smtp.PORT := nPort;
    smtp.USER := '';
    smtp.PASS := '';
    smtp.FROM := sFrom;
    smtp.RCPT := sTo;
    smtp.SUBJECT := sSubject;
    smtp.TIMEOUT := tTimeout;
    smtp.BODY := sBody;
    smtp.EXEC := bExecute;
    smtp();
    bBusy := smtp.BUSY;
    bDone := smtp.DONE;
    nStatus := smtp.STATUS;
    nRespCode := smtp.RESP_CODE;
    nErrId := smtp.ERRID;
    bError := smtp.BUSY = FALSE AND smtp.DONE = TRUE AND smtp.STATUS >= 2;
END_FUNCTION_BLOCK

// SMTP send with AUTH LOGIN — TwinCAT FB_SmtpV3_Full subset.
// sUser/sPass empty disables authentication (same as FB_SmtpV3).
FUNCTION_BLOCK FB_SmtpV3_Full
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME := T#10S;
    sServer : STRING[15];
    nPort : UDINT := 25;
    sUser : STRING[31];
    sPass : STRING[31];
    sFrom : STRING[63];
    sTo : STRING[63];
    sSubject : STRING[63];
    sBody : STRING[255];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nRespCode : UDINT;
END_VAR
VAR
    smtp : FB_SmtpSend;
END_VAR
    smtp.HOST := sServer;
    smtp.PORT := nPort;
    smtp.USER := sUser;
    smtp.PASS := sPass;
    smtp.FROM := sFrom;
    smtp.RCPT := sTo;
    smtp.SUBJECT := sSubject;
    smtp.TIMEOUT := tTimeout;
    smtp.BODY := sBody;
    smtp.EXEC := bExecute;
    smtp();
    bBusy := smtp.BUSY;
    bDone := smtp.DONE;
    nStatus := smtp.STATUS;
    nRespCode := smtp.RESP_CODE;
    nErrId := smtp.ERRID;
    bError := smtp.BUSY = FALSE AND smtp.DONE = TRUE AND smtp.STATUS >= 2;
END_FUNCTION_BLOCK

// Obsolete TwinCAT FB_Smtp — kept as an alias of FB_SmtpV3.
FUNCTION_BLOCK FB_Smtp
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME := T#10S;
    sServer : STRING[15];
    nPort : UDINT := 25;
    sFrom : STRING[63];
    sTo : STRING[63];
    sSubject : STRING[63];
    sBody : STRING[255];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nRespCode : UDINT;
END_VAR
VAR
    v3 : FB_SmtpV3;
END_VAR
    v3.bExecute := bExecute;
    v3.tTimeout := tTimeout;
    v3.sServer := sServer;
    v3.nPort := nPort;
    v3.sFrom := sFrom;
    v3.sTo := sTo;
    v3.sSubject := sSubject;
    v3.sBody := sBody;
    v3();
    bBusy := v3.bBusy;
    bDone := v3.bDone;
    bError := v3.bError;
    nErrId := v3.nErrId;
    nStatus := v3.nStatus;
    nRespCode := v3.nRespCode;
END_FUNCTION_BLOCK

// Obsolete TwinCAT FB_SmtpFull — alias of FB_SmtpV3_Full.
FUNCTION_BLOCK FB_SmtpFull
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME := T#10S;
    sServer : STRING[15];
    nPort : UDINT := 25;
    sUser : STRING[31];
    sPass : STRING[31];
    sFrom : STRING[63];
    sTo : STRING[63];
    sSubject : STRING[63];
    sBody : STRING[255];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nRespCode : UDINT;
END_VAR
VAR
    v3 : FB_SmtpV3_Full;
END_VAR
    v3.bExecute := bExecute;
    v3.tTimeout := tTimeout;
    v3.sServer := sServer;
    v3.nPort := nPort;
    v3.sUser := sUser;
    v3.sPass := sPass;
    v3.sFrom := sFrom;
    v3.sTo := sTo;
    v3.sSubject := sSubject;
    v3.sBody := sBody;
    v3();
    bBusy := v3.bBusy;
    bDone := v3.bDone;
    bError := v3.bError;
    nErrId := v3.nErrId;
    nStatus := v3.nStatus;
    nRespCode := v3.nRespCode;
END_FUNCTION_BLOCK

// TwinCAT FB_SmtpAttach — MIME file attachment. TetherPLC has no
// MIME/base64 encoder in the ST layer, so this FB reports an
// unsupported error immediately on EXEC. nErrId = 90.
FUNCTION_BLOCK FB_SmtpAttach
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME := T#10S;
    sServer : STRING[15];
    nPort : UDINT := 25;
    sUser : STRING[31];
    sPass : STRING[31];
    sFrom : STRING[63];
    sTo : STRING[63];
    sSubject : STRING[63];
    sBody : STRING[255];
    sAttachPath : STRING[63];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nRespCode : UDINT;
END_VAR
VAR
    lastExec : BOOL;
END_VAR
    IF bExecute AND NOT lastExec THEN
        bBusy := FALSE;
        bDone := TRUE;
        bError := TRUE;
        nErrId := 90;
        nStatus := 4;
        nRespCode := 0;
    ELSE
        bBusy := FALSE;
        bDone := FALSE;
        bError := FALSE;
        nErrId := 0;
        nStatus := 0;
        nRespCode := 0;
    END_IF;
    lastExec := bExecute;
END_FUNCTION_BLOCK

// GSM SMS send — TwinCAT SendSMS adapted to the poll model.
// hPort is an open serial handle from FB_SerialOpen (the modem
// UART). The FB runs the AT sequence autonomously after a
// rising edge on bExecute:
//   AT -> AT+CMGF=1 -> AT+CMGS="<sNumber>" -> sText + SUB(0x1A)
// nStatus: 0=ok 1=timeout 2=modem error 3=serial error.
// sLastRx keeps the tail of the modem response for diagnostics.
FUNCTION_BLOCK FB_SendSMS
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME := T#10S;   // per-command budget hint
    hPort : UDINT;              // open handle from FB_SerialOpen
    sNumber : STRING[31];       // destination number
    sText : STRING[160];        // GSM-7 text, 160 chars max
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    sLastRx : STRING[255];
    dbgPhase : INT;
    dbgTries : UDINT;
END_VAR
VAR
    st : INT;
    phase : INT;                // 0=send cmd 1=await reply
    tries : UDINT;
    cmd : STRING[256];
    tokOk : STRING[8];
    tokOk2 : STRING[8];
    tokErr : STRING[8];
    rxAcc : STRING[512];
    ctrlZ : STRING[1];
    lastExec : BOOL;
    snd : FB_SerialSend;
    rcv : FB_SerialReceive;
    sndKick : BOOL;
    rcvKick : BOOL;
END_VAR

    IF bExecute AND NOT lastExec AND st = 0 THEN
        st := 1;
        phase := 0;
        tries := 0;
        rxAcc := '';
        ctrlZ := '';
        cmd := 'AT$R';
        tokOk := 'OK';
        tokOk2 := '';
        tokErr := '';
        sndKick := TRUE;
    END_IF;
    lastExec := bExecute;

    // Both serial FBs are called every scan with bExecute pulsed
    // only on kick scans. FB_Serial arms on a 0->1 EXEC edge, so
    // EXEC must see a low scan between transactions; gating the
    // calls by phase would freeze EXEC high after a fast write
    // and swallow the next kick.
    snd.bExecute := sndKick AND (phase = 0);
    sndKick := FALSE;
    snd.tTimeout := T#2S;
    snd.hPort := hPort;
    snd.pData := cmd;
    snd.nLen := 0;
    snd();

    rcv.bExecute := rcvKick AND (phase = 1);
    rcvKick := FALSE;
    rcv.tTimeout := T#400MS;
    rcv.hPort := hPort;
    rcv();

    IF st >= 1 AND st <= 4 THEN
        IF phase = 0 AND snd.bDone THEN
            IF snd.nErrId <> 0 THEN
                st := 9;
                nStatus := 3;
                nErrId := snd.nErrId;
            ELSE
                phase := 1;
                tries := 0;
                rxAcc := '';
                rcvKick := TRUE;
            END_IF;
        ELSIF phase = 1 AND rcv.bDone THEN
            // await leg: each read completion appends to rxAcc
            // and rescans for the expected token. ~25 x 400ms.
                IF rcv.nErrId = 0 AND rcv.nRecBytes > 0 THEN
                    IF LEN(rxAcc) > 200 THEN
                        st := 9;
                        nStatus := 2;
                        nErrId := 91;
                    ELSE
                        rxAcc := CONCAT(rxAcc, rcv.pData);
                        sLastRx := rcv.pData;
                        IF tokErr <> '' AND FIND(rxAcc, tokErr) > 0 THEN
                            st := 9;
                            nStatus := 2;
                            nErrId := 92;
                        ELSIF FIND(rxAcc, tokOk) > 0
                            OR (tokOk2 <> '' AND FIND(rxAcc, tokOk2) > 0) THEN
                            IF st = 1 THEN
                                st := 2; phase := 0; sndKick := TRUE;
                                cmd := 'AT+CMGF=1$R';
                                tokOk := 'OK'; tokOk2 := '';
                            ELSIF st = 2 THEN
                                st := 3; phase := 0; sndKick := TRUE;
                                cmd := CONCAT('AT+CMGS="', sNumber);
                                cmd := CONCAT(cmd, '"$R');
                                tokOk := '>'; tokOk2 := '';
                            ELSIF st = 3 THEN
                                st := 4; phase := 0; sndKick := TRUE;
                                cmd := CONCAT(sText, ctrlZ);
                                tokOk := '+CMGS'; tokOk2 := 'OK';
                                tokErr := 'ERROR';
                            ELSE
                                st := 5;
                            END_IF;
                            tries := 0;
                            rxAcc := '';
                        ELSE
                            tries := tries + 1;
                            IF tries > 25 THEN
                                st := 9;
                                nStatus := 1;
                                nErrId := 5;
                            ELSE
                                rcvKick := TRUE;
                            END_IF;
                        END_IF;
                    END_IF;
                ELSE
                    tries := tries + 1;
                    IF tries > 25 THEN
                        st := 9;
                        nStatus := 1;
                        nErrId := 5;
                    ELSE
                        rcvKick := TRUE;
                    END_IF;
                END_IF;
        END_IF;
        bBusy := TRUE;
        bDone := FALSE;
        bError := FALSE;
        dbgPhase := st * 10 + phase;
        dbgTries := tries;
    ELSIF st = 5 THEN
        bBusy := FALSE;
        bDone := TRUE;
        bError := FALSE;
        nStatus := 0;
        nErrId := 0;
        st := 6;
    ELSIF st = 6 THEN
        bBusy := FALSE;
        bDone := FALSE;
        bError := FALSE;
        nStatus := 0;
        nErrId := 0;
        st := 0;
    ELSIF st = 9 THEN
        bBusy := FALSE;
        bDone := TRUE;
        bError := TRUE;
        st := 10;
    ELSIF st = 10 THEN
        bBusy := FALSE;
        bDone := FALSE;
        bError := FALSE;
        st := 0;
    ELSE
        bBusy := FALSE;
        bDone := FALSE;
        bError := FALSE;
        nStatus := 0;
        nErrId := 0;
    END_IF;
END_FUNCTION_BLOCK

// TwinCAT Get_TcPlcSMS_Version — returns the SMS layer version
// string after one EXEC edge. sVersion is fixed; nErrId = 0.
FUNCTION_BLOCK FB_GetTcPlcSMS_Version
VAR_INPUT
    bExecute : BOOL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    sVersion : STRING[31];
END_VAR
VAR
    lastExec : BOOL;
    fired : BOOL;
END_VAR
    IF bExecute AND NOT lastExec AND NOT fired THEN
        fired := TRUE;
        sVersion := 'TetherPLC-TcSMS 1.0';
    END_IF;
    lastExec := bExecute;
    bBusy := FALSE;
    bDone := fired;
END_FUNCTION_BLOCK
