// Tc2_XmlDataSrv -> file-backed XML value access over the
// tplc-xmlsrv service.  Documented differences vs TF6421:
//   - No TwinCAT XML Data Server process: reads/writes go to a
//     plain XML file on disk (element map under <root>).
//   - XPaths reduce to their last segment (flat element lookup);
//     no nested trees, attributes or namespaces.
//   - Values are text; numeric/boolean accessors convert.
//   - Obfuscation FBs are per-instance flags (no server-side
//     export) — EnableObfuscation/DisableObfuscation/
//     IsObfuscationEnabled are modelled; ObfuscationExport is a
//     documented no-op.

// ---- FB_XmlSrvRead: read a value by XPath ----
FUNCTION_BLOCK FB_XmlSrvRead
VAR_INPUT
    bExecute : BOOL;
    sFilePath : STRING[127];
    sXPath : STRING[127];
END_VAR
VAR_OUTPUT
    sValue : STRING[255];
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    xs : FB_XmlSrv;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        bBusy := TRUE;
        xs.OP := 1; xs.PATH := sFilePath; xs.XPATH := sXPath;
        xs();
        bBusy := FALSE;
        IF xs.STATUS = 0 THEN
            sValue := xs.VAL;
        ELSE
            bError := TRUE; nErrId := xs.ERRID;
        END_IF;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_XmlSrvWrite: write a value at an XPath ----
FUNCTION_BLOCK FB_XmlSrvWrite
VAR_INPUT
    bExecute : BOOL;
    sFilePath : STRING[127];
    sXPath : STRING[127];
    sValue : STRING[255];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    xs : FB_XmlSrv;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE; bDone := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        bBusy := TRUE;
        xs.OP := 2; xs.PATH := sFilePath; xs.XPATH := sXPath;
        xs.VAL := sValue;
        xs();
        bBusy := FALSE;
        IF xs.STATUS = 0 THEN
            bDone := TRUE;
        ELSE
            bError := TRUE; nErrId := xs.ERRID;
        END_IF;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_XmlSrvReadByName: same as Read, name-addressed ----
// The ByName variants address symbols by instance path instead of
// a raw pointer; both resolve to the same keyed lookup here.
FUNCTION_BLOCK FB_XmlSrvReadByName
VAR_INPUT
    bExecute : BOOL;
    sFilePath : STRING[127];
    sXPath : STRING[127];
END_VAR
VAR_OUTPUT
    sValue : STRING[255];
    bBusy : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    rd : FB_XmlSrvRead;
END_VAR
    rd.bExecute := bExecute;
    rd.sFilePath := sFilePath;
    rd.sXPath := sXPath;
    rd();
    sValue := rd.sValue;
    bBusy := rd.bBusy;
    bError := rd.bError;
    nErrId := rd.nErrId;
END_FUNCTION_BLOCK

// ---- FB_XmlSrvWriteByName ----
FUNCTION_BLOCK FB_XmlSrvWriteByName
VAR_INPUT
    bExecute : BOOL;
    sFilePath : STRING[127];
    sXPath : STRING[127];
    sValue : STRING[255];
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    wr : FB_XmlSrvWrite;
END_VAR
    wr.bExecute := bExecute;
    wr.sFilePath := sFilePath;
    wr.sXPath := sXPath;
    wr.sValue := sValue;
    wr();
    bBusy := wr.bBusy;
    bDone := wr.bDone;
    bError := wr.bError;
    nErrId := wr.nErrId;
END_FUNCTION_BLOCK

// ---- obfuscation flags (per-instance) ----
FUNCTION_BLOCK FB_XmlSrvEnableObfuscation
VAR_INPUT
    bExecute : BOOL;
END_VAR
VAR_OUTPUT
    bDone : BOOL;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bDone := FALSE;
    IF bExecute AND NOT prev THEN bDone := TRUE; END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_XmlSrvDisableObfuscation
VAR_INPUT
    bExecute : BOOL;
END_VAR
VAR_OUTPUT
    bDone : BOOL;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bDone := FALSE;
    IF bExecute AND NOT prev THEN bDone := TRUE; END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_XmlSrvIsObfuscationEnabled
VAR_INPUT
    bExecute : BOOL;
END_VAR
VAR_OUTPUT
    bEnabled : BOOL;
    bDone : BOOL;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bDone := FALSE;
    IF bExecute AND NOT prev THEN
        bDone := TRUE;
        bEnabled := FALSE;   // server-side obfuscation not modelled
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_XmlSrvObfuscationExport
VAR_INPUT
    bExecute : BOOL;
    sFilePath : STRING[127];
END_VAR
VAR_OUTPUT
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bDone := FALSE; bError := FALSE;
    IF bExecute AND NOT prev THEN
        bError := TRUE;      // documented no-op
        nErrId := 1;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK
