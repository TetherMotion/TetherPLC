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
        w.nCmd := 1;                        // Begin nested doc
        st := 27;
    ELSIF st = 27 THEN
        w.nCmd := 0;
        w.sKey := 'dev';
        st := 28;
    ELSIF st = 28 THEN
        w.nCmd := 9;                        // ObjBegin dev
        st := 29;
    ELSIF st = 29 THEN
        w.nCmd := 0;
        w.sKey := 'name'; w.sVal := 'ax1';
        st := 30;
    ELSIF st = 30 THEN
        w.nCmd := 2;                        // AddString name
        st := 31;
    ELSIF st = 31 THEN
        w.nCmd := 0;
        w.sKey := 'spd'; w.nInt := 2;
        st := 32;
    ELSIF st = 32 THEN
        w.nCmd := 3;                        // AddInt spd
        st := 33;
    ELSIF st = 33 THEN
        w.nCmd := 0;
        st := 34;
    ELSIF st = 34 THEN
        w.nCmd := 11;                       // EndAny -> close dev
        st := 35;
    ELSIF st = 35 THEN
        w.nCmd := 0;
        w.sKey := 'axes';
        st := 36;
    ELSIF st = 36 THEN
        w.nCmd := 10;                       // ArrBegin axes
        st := 37;
    ELSIF st = 37 THEN
        w.nCmd := 0;
        w.sKey := ''; w.nInt := 0;
        st := 38;
    ELSIF st = 38 THEN
        w.nCmd := 3;                        // array elem 0 (scalar)
        st := 39;
    ELSIF st = 39 THEN
        w.nCmd := 0;
        w.sKey := '';
        st := 40;
    ELSIF st = 40 THEN
        w.nCmd := 9;                        // ObjBegin elem object
        st := 41;
    ELSIF st = 41 THEN
        w.nCmd := 0;
        w.sKey := 'id'; w.nInt := 0;
        st := 42;
    ELSIF st = 42 THEN
        w.nCmd := 3;                        // AddInt id
        st := 43;
    ELSIF st = 43 THEN
        w.nCmd := 0;
        w.sKey := 'pos'; w.nInt := 100;
        st := 44;
    ELSIF st = 44 THEN
        w.nCmd := 3;                        // AddInt pos
        st := 45;
    ELSIF st = 45 THEN
        w.nCmd := 0;
        st := 46;
    ELSIF st = 46 THEN
        w.nCmd := 11;                       // EndAny -> close elem obj
        st := 47;
    ELSIF st = 47 THEN
        w.nCmd := 0;
        w.sKey := '';
        st := 48;
    ELSIF st = 48 THEN
        w.nCmd := 9;                        // ObjBegin elem 1
        st := 49;
    ELSIF st = 49 THEN
        w.nCmd := 0;
        w.sKey := 'id'; w.nInt := 1;
        st := 50;
    ELSIF st = 50 THEN
        w.nCmd := 3;
        st := 51;
    ELSIF st = 51 THEN
        w.nCmd := 0;
        w.sKey := 'pos'; w.nInt := 250;
        st := 52;
    ELSIF st = 52 THEN
        w.nCmd := 3;
        st := 53;
    ELSIF st = 53 THEN
        w.nCmd := 0;
        st := 54;
    ELSIF st = 54 THEN
        w.nCmd := 11;                       // EndAny -> close elem 1
        st := 55;
    ELSIF st = 55 THEN
        w.nCmd := 0;
        st := 56;
    ELSIF st = 56 THEN
        w.nCmd := 11;                       // EndAny -> close axes
        st := 57;
    ELSIF st = 57 THEN
        w.nCmd := 0;
        st := 58;
    ELSIF st = 58 THEN
        w.nCmd := 7;                        // End root
        st := 59;
    ELSIF st = 59 THEN
        w.nCmd := 0;
        doc := w.sDoc;
        IF FIND(doc, '"dev":{"name":"ax1","spd":2}') > 0
        THEN mask := mask OR 1024; END_IF;
        IF FIND(doc, ',"axes":[0,{"id":0,"pos":100}') > 0
        THEN mask := mask OR 2048; END_IF;
        IF FIND(doc, '{"id":1,"pos":250}]}') > 0
        THEN mask := mask OR 4096; END_IF;
        r.sIn := doc;
        st := 60;
    ELSIF st = 60 THEN
        r.nCmd := 1;                        // Load
        st := 61;
    ELSIF st = 61 THEN
        r.nCmd := 0;
        r.sKey := 'dev.spd';
        st := 62;
    ELSIF st = 62 THEN
        r.nCmd := 3;                        // GetInt dev.spd
        st := 63;
    ELSIF st = 63 THEN
        r.nCmd := 0;
        IF r.bDone AND r.nOut = 2 THEN mask := mask OR 8192; END_IF;
        r.sKey := 'axes.2.pos';
        st := 64;
    ELSIF st = 64 THEN
        r.nCmd := 3;                        // GetInt axes.2.pos
        st := 65;
    ELSIF st = 65 THEN
        r.nCmd := 0;
        IF r.bDone AND r.nOut = 250 THEN mask := mask OR 16384; END_IF;
        r.sKey := 'dev';
        st := 66;
    ELSIF st = 66 THEN
        r.nCmd := 2;                        // GetString dev -> raw obj
        st := 67;
    ELSIF st = 67 THEN
        r.nCmd := 0;
        IF r.bDone AND FIND(r.sOut, '"name"') > 0
        THEN mask := mask OR 32768; END_IF;
        r.sKey := 'axes.0.id';
        st := 68;
    ELSIF st = 68 THEN
        r.nCmd := 3;                        // GetInt axes.0.id -> elem1 obj? no:
        st := 69;                           // axes.0 is scalar 0, .id fails
    ELSIF st = 69 THEN
        r.nCmd := 0;
        // axes.0 is the scalar element 0; path 'axes.0.id' must fail
        IF r.bError THEN mask := mask OR 65536; END_IF;
        st := 70;
    END_IF;
    w(); r();
END_PROGRAM
