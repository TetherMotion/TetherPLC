// Tc3_FTP + Tc2_FTP -> FTP client wrappers (RFC 959, PASV data mode).
// TwinCAT's FB_FtpConnection/FB_FtpExplorer/FB_FtpTransfer objects
// and the Tc2_FTP FB_* blocks map onto the poll-driven service
// model used by the other compat libraries: FB_FtpConn holds the
// control session, FB_FtpCmd runs one control command per EXEC
// edge, FB_FtpXfer runs one RETR/STOR/LIST data transfer.
// Differences from the TwinCAT FBs, documented in
// docs/plc/compat/README.md:
//   - No FTP profile objects (AddProfile/GetProfile), no AMS/ADS
//     routing, no I_TcMessage/EventLogger/HRESULT surfaces.
//     Credentials are plain STRING inputs per connection.
//   - Passive (PASV) data connections only: no PORT/active mode,
//     no EPSV, no FTPS/TLS, no SFTP/SSH, no REST resume, no
//     MLSD/MLST, IPv4 dotted-quad hosts only (no DNS —
//     FB_FTP_HostResolve is a documented gap).
//   - Rename/move is RNFR+RNTO (works for files and directories
//     on most servers); true server-side rename semantics depend
//     on the FTP server.
//   - Downloads stream 512-byte chunks per scan (BUF + CHUNK +
//     TOTAL); uploads take UP_DATA[0..UP_LEN) once per EXEC.
//   - nStatus: conn 0=pending 1=ready 2=error; cmd 0=pending
//     1=ok(2xx) 2=fail(4xx/5xx) 3=timeout 4=error 5=badresp;
//     xfer adds 6=more-data-expected while streaming.
// Sources: docs/plc/plan/21-twincat-libraries.md (Tc2_FTP,
// Tc3_FTP).

// Open/poll/close an FTP control connection — covers
// FB_FtpConnection.Connect/Disconnect and FB_FTP_Open/
// FB_FTP_OpenEX/FB_FTP_Close/FB_FTP_Info.
FUNCTION_BLOCK FB_FTP_Connect
VAR_INPUT
    sHost : STRING[15];          // dotted-quad PLC IPv4
    nPort : UDINT := 0;          // 0 = 21
    sUser : STRING[31] := 'anonymous';
    sPass : STRING[31] := '';
    bExecute : BOOL;             // rising edge = (re)connect
    bClose : BOOL;               // rising edge = disconnect
    tTimeout : TIME := T#10S;
END_VAR
VAR_OUTPUT
    nConnHandle : UDINT;         // 0 = no live session
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;              // 0=pending 1=ready 2=error
    nErrId : UDINT;
END_VAR
VAR
    conn : FB_FtpConn;
END_VAR
    conn.HOST := sHost;
    conn.PORT := nPort;
    conn.USER := sUser;
    conn.PASS := sPass;
    conn.TIMEOUT := tTimeout;
    conn.CLOSE := bClose;
    conn.EXEC := bExecute;
    conn();
    nConnHandle := conn.CONN_HANDLE;
    bBusy := conn.BUSY;
    bDone := conn.DONE;
    nStatus := conn.STATUS;
    nErrId := conn.ERRID;
    bError := (conn.ERRID <> 0) OR (conn.STATUS = 2);
END_FUNCTION_BLOCK

// One FTP control command — covers the explorer operations
// (MKD/RMD/DELE/RNFR/RNTO/CWD/PWD/SIZE/TYPE/SYST/NOOP/SITE)
// and is the building block for the method wrappers below.
FUNCTION_BLOCK FB_FTP_Cmd
VAR_INPUT
    nConnHandle : UDINT;         // from FB_FTP_Connect
    sCmd : STRING[127];          // e.g. 'MKD /dir'
    bExecute : BOOL;
    tTimeout : TIME := T#5S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;              // 0=pending 1=ok 2=fail 3=timeout 4=error 5=badresp
    nRespCode : UDINT;           // final 3-digit FTP reply code
    nErrId : UDINT;
    sText : STRING[127];         // reply text (last line)
END_VAR
VAR
    c : FB_FtpCmd;
END_VAR
    c.CONN_HANDLE := nConnHandle;
    c.CMD := sCmd;
    c.TIMEOUT := tTimeout;
    c.EXEC := bExecute;
    c();
    bBusy := c.BUSY;
    bDone := c.DONE;
    nStatus := c.STATUS;
    nRespCode := c.RESP_CODE;
    nErrId := c.ERRID;
    sText := c.TEXT;
    bError := (c.ERRID <> 0) OR (c.STATUS >= 2);
END_FUNCTION_BLOCK

// Download a remote file (RETR) — covers
// FB_FtpTransfer.Download and FB_FTP_FileDownload(FileDownloadEx).
// DATA receives up to 512 bytes per BUSY scan; append chunks at
// nChunk until nStatus leaves 0/6.
FUNCTION_BLOCK FB_FTP_Download
VAR_INPUT
    nConnHandle : UDINT;
    sRemotePath : STRING[127];
    bExecute : BOOL;
    tTimeout : TIME := T#30S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;              // 0=pending 6=more 1=ok 2..5=fail
    nRespCode : UDINT;
    nTotal : UDINT;              // bytes received so far
    nChunk : UDINT;              // bytes in DATA this scan
    DATA : ARRAY[0..511] OF BYTE; // latest chunk
END_VAR
VAR
    x : FB_FtpXfer;
END_VAR
    x.CONN_HANDLE := nConnHandle;
    x.OP := 0;
    x.PATH := sRemotePath;
    x.TIMEOUT := tTimeout;
    x.EXEC := bExecute;
    x();
    bBusy := x.BUSY;
    bDone := x.DONE;
    nStatus := x.STATUS;
    nRespCode := x.RESP_CODE;
    nTotal := x.TOTAL;
    nChunk := x.CHUNK;
    DATA := x.BUF;
    bError := (x.ERRID <> 0) OR ((x.STATUS >= 1) AND (x.STATUS <= 5));
END_FUNCTION_BLOCK

// Upload a local file (STOR) — covers FB_FtpTransfer.Upload and
// FB_FTP_FileUpload(FileUploadEx). DATA holds the bytes to send
// (nLen valid, <= 512 per transfer).
FUNCTION_BLOCK FB_FTP_Upload
VAR_INPUT
    nConnHandle : UDINT;
    sRemotePath : STRING[127];
    nLen : UDINT;                // bytes in DATA, <= 512
    DATA : ARRAY[0..511] OF BYTE;
    bExecute : BOOL;
    tTimeout : TIME := T#30S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;
    nRespCode : UDINT;
    nTotal : UDINT;              // bytes accepted/sent so far
END_VAR
VAR
    x : FB_FtpXfer;
    i : UDINT;
END_VAR
    x.CONN_HANDLE := nConnHandle;
    x.OP := 1;
    x.PATH := sRemotePath;
    x.TIMEOUT := tTimeout;
    x.UP_LEN := nLen;
    FOR i := 0 TO 511 DO
        x.UP_DATA[i] := DATA[i];
    END_FOR;
    x.EXEC := bExecute;
    x();
    bBusy := x.BUSY;
    bDone := x.DONE;
    nStatus := x.STATUS;
    nRespCode := x.RESP_CODE;
    nTotal := x.TOTAL;
    bError := (x.ERRID <> 0) OR ((x.STATUS >= 1) AND (x.STATUS <= 5));
END_FUNCTION_BLOCK

// List a remote directory (LIST/NLST) — covers
// FB_FTP_FileList(FileListEx) and the explorer listing path.
// Same chunked download semantics as FB_FTP_Download.
FUNCTION_BLOCK FB_FTP_List
VAR_INPUT
    nConnHandle : UDINT;
    sRemotePath : STRING[127];   // '' = current directory
    bExecute : BOOL;
    tTimeout : TIME := T#30S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;
    nRespCode : UDINT;
    nTotal : UDINT;
    nChunk : UDINT;
    DATA : ARRAY[0..511] OF BYTE; // latest listing chunk
END_VAR
VAR
    x : FB_FtpXfer;
END_VAR
    x.CONN_HANDLE := nConnHandle;
    x.OP := 2;
    x.PATH := sRemotePath;
    x.TIMEOUT := tTimeout;
    x.EXEC := bExecute;
    x();
    bBusy := x.BUSY;
    bDone := x.DONE;
    nStatus := x.STATUS;
    nRespCode := x.RESP_CODE;
    nTotal := x.TOTAL;
    nChunk := x.CHUNK;
    DATA := x.BUF;
    bError := (x.ERRID <> 0) OR ((x.STATUS >= 1) AND (x.STATUS <= 5));
END_FUNCTION_BLOCK

// Create a remote directory — covers FB_FtpExplorer.DirCreate
// and FB_FTP_DirCreate.
FUNCTION_BLOCK FB_FTP_MkDir
VAR_INPUT
    nConnHandle : UDINT;
    sPath : STRING[127];
    bExecute : BOOL;
    tTimeout : TIME := T#5S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;
    nRespCode : UDINT;
    nErrId : UDINT;
END_VAR
VAR
    c : FB_FTP_Cmd;
    sCmd : STRING[127];
END_VAR
    sCmd := CONCAT('MKD ', sPath);
    c.nConnHandle := nConnHandle;
    c.sCmd := sCmd;
    c.tTimeout := tTimeout;
    c.bExecute := bExecute;
    c();
    bBusy := c.bBusy;
    bDone := c.bDone;
    nStatus := c.nStatus;
    nRespCode := c.nRespCode;
    nErrId := c.nErrId;
    bError := c.bError;
END_FUNCTION_BLOCK

// Remove an empty remote directory — covers
// FB_FtpExplorer.DirRemove and FB_FTP_DirRemove.
FUNCTION_BLOCK FB_FTP_RemoveDir
VAR_INPUT
    nConnHandle : UDINT;
    sPath : STRING[127];
    bExecute : BOOL;
    tTimeout : TIME := T#5S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;
    nRespCode : UDINT;
    nErrId : UDINT;
END_VAR
VAR
    c : FB_FTP_Cmd;
    sCmd : STRING[127];
END_VAR
    sCmd := CONCAT('RMD ', sPath);
    c.nConnHandle := nConnHandle;
    c.sCmd := sCmd;
    c.tTimeout := tTimeout;
    c.bExecute := bExecute;
    c();
    bBusy := c.bBusy;
    bDone := c.bDone;
    nStatus := c.nStatus;
    nRespCode := c.nRespCode;
    nErrId := c.nErrId;
    bError := c.bError;
END_FUNCTION_BLOCK

// Delete a remote file — covers FB_FtpExplorer.FileRemove
// and FB_FTP_FileRemove.
FUNCTION_BLOCK FB_FTP_DeleteFile
VAR_INPUT
    nConnHandle : UDINT;
    sPath : STRING[127];
    bExecute : BOOL;
    tTimeout : TIME := T#5S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;
    nRespCode : UDINT;
    nErrId : UDINT;
END_VAR
VAR
    c : FB_FTP_Cmd;
    sCmd : STRING[127];
END_VAR
    sCmd := CONCAT('DELE ', sPath);
    c.nConnHandle := nConnHandle;
    c.sCmd := sCmd;
    c.tTimeout := tTimeout;
    c.bExecute := bExecute;
    c();
    bBusy := c.bBusy;
    bDone := c.bDone;
    nStatus := c.nStatus;
    nRespCode := c.nRespCode;
    nErrId := c.nErrId;
    bError := c.bError;
END_FUNCTION_BLOCK

// Check remote file existence — covers
// FB_FtpExplorer.FileExists and FB_FTP_FileExist. Issues SIZE;
// 213 = exists, 550 = not found.
FUNCTION_BLOCK FB_FTP_FileExists
VAR_INPUT
    nConnHandle : UDINT;
    sPath : STRING[127];
    bExecute : BOOL;
    tTimeout : TIME := T#5S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    bExists : BOOL;
    nStatus : DINT;
    nRespCode : UDINT;
    nErrId : UDINT;
END_VAR
VAR
    c : FB_FTP_Cmd;
    sCmd : STRING[127];
END_VAR
    sCmd := CONCAT('SIZE ', sPath);
    c.nConnHandle := nConnHandle;
    c.sCmd := sCmd;
    c.tTimeout := tTimeout;
    c.bExecute := bExecute;
    c();
    bBusy := c.bBusy;
    bDone := c.bDone;
    nStatus := c.nStatus;
    nRespCode := c.nRespCode;
    nErrId := c.nErrId;
    bExists := (c.nRespCode = 213);
    bError := c.bError AND (c.nStatus <> 2);
END_FUNCTION_BLOCK

// Check remote directory existence — covers
// FB_FtpExplorer.DirExists. Issues a LIST on the path; a
// completed transfer means the directory is listable (exists).
// No working-directory side effects.
FUNCTION_BLOCK FB_FTP_DirExists
VAR_INPUT
    nConnHandle : UDINT;
    sPath : STRING[127];
    bExecute : BOOL;
    tTimeout : TIME := T#10S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    bExists : BOOL;
    nStatus : DINT;
    nRespCode : UDINT;
END_VAR
VAR
    x : FB_FtpXfer;
    sink : ARRAY[0..511] OF BYTE;
END_VAR
    x.CONN_HANDLE := nConnHandle;
    x.OP := 2;
    x.PATH := sPath;
    x.TIMEOUT := tTimeout;
    x.EXEC := bExecute;
    x();
    sink := x.BUF;
    bBusy := x.BUSY;
    bDone := x.DONE;
    nStatus := x.STATUS;
    nRespCode := x.RESP_CODE;
    bExists := (x.STATUS = 1);
    bError := (x.ERRID <> 0) OR ((x.STATUS >= 2) AND (x.STATUS <= 5));
END_FUNCTION_BLOCK

// Rename/move a remote file or directory — covers
// FB_FtpExplorer.FileRename/FileMove/DirRename/DirMove and
// FB_FTP_FileRename. Issues RNFR <old> then RNTO <new> on the
// same connection; success requires 350 then 250/2xx.
FUNCTION_BLOCK FB_FTP_Rename
VAR_INPUT
    nConnHandle : UDINT;
    sOldPath : STRING[127];
    sNewPath : STRING[127];
    bExecute : BOOL;
    tTimeout : TIME := T#10S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;              // 0=pending 1=ok 2=fail 3=timeout 4=error 5=badresp
    nRespCode : UDINT;           // final reply code (RNTO)
    nErrId : UDINT;
END_VAR
VAR
    rnfr : FB_FTP_Cmd;
    rnto : FB_FTP_Cmd;
    sCmd1 : STRING[127];
    sCmd2 : STRING[127];
    step : UDINT := 0;           // 0=RNFR 1=RNTO 2=report-fail
    prevExec : BOOL := FALSE;
END_VAR
    sCmd1 := CONCAT('RNFR ', sOldPath);
    sCmd2 := CONCAT('RNTO ', sNewPath);
    IF bExecute AND NOT prevExec THEN
        step := 0;
    END_IF;
    prevExec := bExecute;
    IF step = 0 THEN
        rnfr.nConnHandle := nConnHandle;
        rnfr.sCmd := sCmd1;
        rnfr.tTimeout := tTimeout;
        rnfr.bExecute := bExecute;
        rnfr();
        IF rnfr.bDone THEN
            IF rnfr.nRespCode = 350 THEN
                step := 1;
            ELSE
                step := 2;
            END_IF;
        END_IF;
        bBusy := rnfr.bBusy;
        bDone := FALSE;
        nStatus := rnfr.nStatus;
        nRespCode := rnfr.nRespCode;
        nErrId := rnfr.nErrId;
        bError := rnfr.bError;
    ELSIF step = 1 THEN
        rnto.nConnHandle := nConnHandle;
        rnto.sCmd := sCmd2;
        rnto.tTimeout := tTimeout;
        rnto.bExecute := TRUE;    // level-held; builtin edge-fires once
        rnto();
        IF rnto.bDone THEN
            step := 0;
        END_IF;
        bBusy := rnto.bBusy;
        bDone := rnto.bDone;
        nStatus := rnto.nStatus;
        nRespCode := rnto.nRespCode;
        nErrId := rnto.nErrId;
        bError := rnto.bError;
    ELSE
        // step 2: RNFR rejected — hold the failure visible until
        // bExecute drops, then arm for the next request
        bBusy := FALSE;
        bDone := TRUE;
        nStatus := 2;
        nRespCode := rnfr.nRespCode;
        nErrId := rnfr.nErrId;
        bError := TRUE;
        IF NOT bExecute THEN
            step := 0;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
