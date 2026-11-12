// tc3_xmlsrv_case.st — e2e for tc3_xmlsrv (file-backed XML store).
PROGRAM tc3_xmlsrv_case
VAR
    wr : FB_XmlSrvWrite;
    rd : FB_XmlSrvRead;
    wrn : FB_XmlSrvWriteByName;
    rdn : FB_XmlSrvReadByName;
    obf : FB_XmlSrvIsObfuscationEnabled;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        wr.sFilePath := '/tmp/tplc_xmlsrv_e2e.xml';
        wr.sXPath := 'PLC/Tags/Temperature';
        wr.sValue := '21.5';
        wr.bExecute := TRUE;
        st := 1;
    ELSIF st = 1 THEN
        IF wr.bDone THEN mask := mask OR 1; END_IF;
        wr.bExecute := FALSE;
        rd.sFilePath := '/tmp/tplc_xmlsrv_e2e.xml';
        rd.sXPath := 'PLC/Tags/Temperature';
        st := 2;
    ELSIF st = 2 THEN
        rd.bExecute := TRUE;
        st := 3;
    ELSIF st = 3 THEN
        IF NOT rd.bError AND FIND(rd.sValue, '21.5') = 1 THEN
            mask := mask OR 2;
        END_IF;
        rd.bExecute := FALSE;
        // second element via the ByName variant
        wrn.sFilePath := '/tmp/tplc_xmlsrv_e2e.xml';
        wrn.sXPath := 'PLC/Tags/Pressure';
        wrn.sValue := '7.25';
        st := 4;
    ELSIF st = 4 THEN
        wrn.bExecute := TRUE;
        st := 5;
    ELSIF st = 5 THEN
        IF wrn.bDone THEN mask := mask OR 4; END_IF;
        wrn.bExecute := FALSE;
        rdn.sFilePath := '/tmp/tplc_xmlsrv_e2e.xml';
        rdn.sXPath := 'PLC/Tags/Pressure';
        st := 6;
    ELSIF st = 6 THEN
        rdn.bExecute := TRUE;
        st := 7;
    ELSIF st = 7 THEN
        IF NOT rdn.bError AND FIND(rdn.sValue, '7.25') = 1 THEN
            mask := mask OR 8;
        END_IF;
        rdn.bExecute := FALSE;
        // first element must still be intact after the rewrite
        rd.sXPath := 'PLC/Tags/Temperature';
        st := 8;
    ELSIF st = 8 THEN
        rd.bExecute := TRUE;
        st := 9;
    ELSIF st = 9 THEN
        IF NOT rd.bError AND FIND(rd.sValue, '21.5') = 1 THEN
            mask := mask OR 16;
        END_IF;
        rd.bExecute := FALSE;
        // missing key -> error
        rd.sXPath := 'PLC/Tags/Absent';
        st := 10;
    ELSIF st = 10 THEN
        rd.bExecute := TRUE;
        st := 11;
    ELSIF st = 11 THEN
        IF rd.bError THEN mask := mask OR 32; END_IF;
        rd.bExecute := FALSE;
        obf.bExecute := TRUE;
        st := 12;
    ELSIF st = 12 THEN
        IF obf.bDone AND NOT obf.bEnabled THEN mask := mask OR 64; END_IF;
        obf.bExecute := FALSE;
        st := 13;
    END_IF;
    wr(); rd(); wrn(); rdn(); obf();
END_PROGRAM
