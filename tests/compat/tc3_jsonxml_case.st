// tc3_jsonxml_case.st — e2e for tc3_jsonxml (pure-ST JSON).
// Builds a flat JSON object with FB_JsonSaxWriter, checks the
// text shape with FIND (string equality is not usable here),
// then parses it back with FB_JsonSaxReader.
PROGRAM tc3_jsonxml_case
VAR
    w : FB_JsonSaxWriter;
    r : FB_JsonSaxReader;
    doc : STRING[255];
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        w.nCmd := 1;                        // Begin
        st := 1;
    ELSIF st = 1 THEN
        w.nCmd := 0;
        w.sKey := 'sp'; w.fReal := 12.5;
        st := 2;
    ELSIF st = 2 THEN
        w.nCmd := 4;                        // AddReal sp
        st := 3;
    ELSIF st = 3 THEN
        w.nCmd := 0;
        w.sKey := 'n'; w.nInt := 7;
        st := 4;
    ELSIF st = 4 THEN
        w.nCmd := 3;                        // AddInt n
        st := 5;
    ELSIF st = 5 THEN
        w.nCmd := 0;
        w.sKey := 'ok'; w.bBool := TRUE;
        st := 6;
    ELSIF st = 6 THEN
        w.nCmd := 5;                        // AddBool ok
        st := 7;
    ELSIF st = 7 THEN
        w.nCmd := 0;
        w.sKey := 's'; w.sVal := 'abc';
        st := 8;
    ELSIF st = 8 THEN
        w.nCmd := 2;                        // AddString s
        st := 9;
    ELSIF st = 9 THEN
        w.nCmd := 0;
        w.sKey := 'z';
        st := 10;
    ELSIF st = 10 THEN
        w.nCmd := 6;                        // AddNull z
        st := 11;
    ELSIF st = 11 THEN
        w.nCmd := 0;                        // clear so End sees an edge
        st := 12;
    ELSIF st = 12 THEN
        w.nCmd := 7;                        // End
        st := 13;
    ELSIF st = 13 THEN
        w.nCmd := 0;
        doc := w.sDoc;
        // shape checks (FIND, not equality)
        IF FIND(doc, '{"sp":12.500') = 1 THEN mask := mask OR 1; END_IF;
        IF FIND(doc, ',"n":7') > 0 THEN mask := mask OR 2; END_IF;
        IF FIND(doc, ',"ok":true') > 0 THEN mask := mask OR 4; END_IF;
        IF FIND(doc, ',"s":"abc"') > 0 THEN mask := mask OR 8; END_IF;
        IF FIND(doc, ',"z":null}') > 0 THEN mask := mask OR 16; END_IF;
        r.sIn := doc;
        st := 14;
    ELSIF st = 14 THEN
        r.nCmd := 1;                        // Load
        st := 15;
    ELSIF st = 15 THEN
        r.nCmd := 0;
        r.sKey := 'n';
        st := 16;
    ELSIF st = 16 THEN
        r.nCmd := 3;                        // GetInt n
        st := 17;
    ELSIF st = 17 THEN
        r.nCmd := 0;
        IF r.bDone AND r.nOut = 7 THEN mask := mask OR 32; END_IF;
        r.sKey := 'sp';
        st := 18;
    ELSIF st = 18 THEN
        r.nCmd := 4;                        // GetReal sp
        st := 19;
    ELSIF st = 19 THEN
        r.nCmd := 0;
        IF r.bDone AND r.fOut = 12.5 THEN mask := mask OR 64; END_IF;
        r.sKey := 'ok';
        st := 20;
    ELSIF st = 20 THEN
        r.nCmd := 5;                        // GetBool ok
        st := 21;
    ELSIF st = 21 THEN
        r.nCmd := 0;
        IF r.bDone AND r.bOut THEN mask := mask OR 128; END_IF;
        r.sKey := 's';
        st := 22;
    ELSIF st = 22 THEN
        r.nCmd := 2;                        // GetString s
        st := 23;
    ELSIF st = 23 THEN
        r.nCmd := 0;
        IF r.bDone AND FIND(r.sOut, 'abc') = 1 THEN mask := mask OR 256; END_IF;
        r.sKey := 'missing';
        st := 24;
    ELSIF st = 24 THEN
        r.nCmd := 2;                        // absent key -> error
        st := 25;
    ELSIF st = 25 THEN
        r.nCmd := 0;
        IF r.bError THEN mask := mask OR 512; END_IF;
        st := 26;
    END_IF;
    w(); r();
END_PROGRAM
