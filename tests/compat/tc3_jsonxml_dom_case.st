// tc3_jsonxml_dom_case.st — e2e for FB_JsonDomParser: load a
// document, query the tree, serialize, build a tree, error path.
// Every command needs a dedicated arm state + a reap state that
// clears nCmd (edge detect prev=0).
PROGRAM tc3_jsonxml_dom_case
VAR
    d : FB_JsonDomParser;
    doc : STRING[255];
    hR : UDINT;
    hDev : UDINT;
    hId : UDINT;
    hAx : UDINT;
    hA1 : UDINT;
    hF : UDINT;
    hN : UDINT;
    hR2 : UDINT;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        doc := CONCAT('{"name":"ax1","dev":{"id":7,"spd":1.5}',
            ',"axes":[1,2,3],"flag":true,"note":null}');
        d.sIn := doc;
        d.nCmd := 1;                        // Load
        st := 1;
    ELSIF st = 1 THEN                        // reap Load
        d.nCmd := 0;
        IF d.bDone THEN mask := mask OR 1; END_IF;
        st := 2;
    ELSIF st = 2 THEN                        // arm Root
        d.nCmd := 3;
        st := 3;
    ELSIF st = 3 THEN                        // reap Root
        d.nCmd := 0;
        hR := d.hOut;
        IF hR <> 0 THEN mask := mask OR 2; END_IF;
        st := 4;
    ELSIF st = 4 THEN                        // arm NodeType root
        d.hNode := hR;
        d.nCmd := 7;
        st := 5;
    ELSIF st = 5 THEN                        // reap
        d.nCmd := 0;
        IF d.nOut = 1 THEN mask := mask OR 4; END_IF;
        st := 6;
    ELSIF st = 6 THEN                        // arm ChildCount root
        d.hNode := hR;
        d.nCmd := 6;
        st := 7;
    ELSIF st = 7 THEN                        // reap
        d.nCmd := 0;
        IF d.nOut = 5 THEN mask := mask OR 8; END_IF;
        st := 8;
    ELSIF st = 8 THEN                        // arm ChildByName dev
        d.hNode := hR; d.sKey := 'dev';
        d.nCmd := 5;
        st := 9;
    ELSIF st = 9 THEN                        // reap -> hDev
        d.nCmd := 0;
        hDev := d.hOut;
        IF hDev <> 0 THEN mask := mask OR 16; END_IF;
        st := 10;
    ELSIF st = 10 THEN                       // arm NodeType dev
        d.hNode := hDev;
        d.nCmd := 7;
        st := 11;
    ELSIF st = 11 THEN                       // reap
        d.nCmd := 0;
        IF d.nOut = 1 THEN mask := mask OR 32; END_IF;
        st := 12;
    ELSIF st = 12 THEN                       // arm ChildByName id
        d.hNode := hDev; d.sKey := 'id';
        d.nCmd := 5;
        st := 13;
    ELSIF st = 13 THEN                       // reap -> hId
        d.nCmd := 0;
        hId := d.hOut;
        st := 14;
    ELSIF st = 14 THEN                       // arm NodeText id
        d.hNode := hId;
        d.nCmd := 9;
        st := 15;
    ELSIF st = 15 THEN                       // reap
        d.nCmd := 0;
        IF FIND(d.sOut, '7') = 1 THEN
            mask := mask OR 64;
        END_IF;
        st := 16;
    ELSIF st = 16 THEN                       // arm ChildByName axes
        d.hNode := hR; d.sKey := 'axes';
        d.nCmd := 5;
        st := 17;
    ELSIF st = 17 THEN                       // reap -> hAx
        d.nCmd := 0;
        hAx := d.hOut;
        st := 18;
    ELSIF st = 18 THEN                       // arm ChildCount axes
        d.hNode := hAx;
        d.nCmd := 6;
        st := 19;
    ELSIF st = 19 THEN                       // reap
        d.nCmd := 0;
        IF d.nOut = 3 THEN mask := mask OR 128; END_IF;
        st := 20;
    ELSIF st = 20 THEN                       // arm Child axes[1]
        d.hNode := hAx; d.nIdx := 1;
        d.nCmd := 4;
        st := 21;
    ELSIF st = 21 THEN                       // reap -> hA1
        d.nCmd := 0;
        hA1 := d.hOut;
        st := 22;
    ELSIF st = 22 THEN                       // arm NodeText axes[1]
        d.hNode := hA1;
        d.nCmd := 9;
        st := 23;
    ELSIF st = 23 THEN                       // reap
        d.nCmd := 0;
        IF FIND(d.sOut, '2') = 1 THEN
            mask := mask OR 256;
        END_IF;
        st := 24;
    ELSIF st = 24 THEN                       // arm ChildByName flag
        d.hNode := hR; d.sKey := 'flag';
        d.nCmd := 5;
        st := 25;
    ELSIF st = 25 THEN                       // reap -> hF
        d.nCmd := 0;
        hF := d.hOut;
        st := 26;
    ELSIF st = 26 THEN                       // arm NodeText flag
        d.hNode := hF;
        d.nCmd := 9;
        st := 27;
    ELSIF st = 27 THEN                       // reap
        d.nCmd := 0;
        IF FIND(d.sOut, 'true') = 1 THEN
            mask := mask OR 512;
        END_IF;
        st := 28;
    ELSIF st = 28 THEN                       // arm ChildByName note
        d.hNode := hR; d.sKey := 'note';
        d.nCmd := 5;
        st := 29;
    ELSIF st = 29 THEN                       // reap -> hN
        d.nCmd := 0;
        hN := d.hOut;
        st := 30;
    ELSIF st = 30 THEN                       // arm NodeType note
        d.hNode := hN;
        d.nCmd := 7;
        st := 31;
    ELSIF st = 31 THEN                       // reap
        d.nCmd := 0;
        IF d.nOut = 6 THEN mask := mask OR 1024; END_IF;
        st := 32;
    ELSIF st = 32 THEN                       // arm NodeKey dev
        d.hNode := hDev;
        d.nCmd := 8;
        st := 33;
    ELSIF st = 33 THEN                       // reap
        d.nCmd := 0;
        IF FIND(d.sOut, 'dev') = 1 THEN
            mask := mask OR 2048;
        END_IF;
        st := 34;
    ELSIF st = 34 THEN                       // arm ToJson whole doc
        d.hNode := 0;
        d.nCmd := 10;
        st := 35;
    ELSIF st = 35 THEN                       // reap
        d.nCmd := 0;
        IF FIND(d.sOut, '"name":"ax1"') > 0
           AND FIND(d.sOut, '"axes":[1,2,3]') > 0 THEN
            mask := mask OR 4096;
        END_IF;
        st := 36;
    ELSIF st = 36 THEN                       // arm Clear
        d.nCmd := 2;
        st := 37;
    ELSIF st = 37 THEN                       // reap Clear
        d.nCmd := 0;
        st := 38;
    ELSIF st = 38 THEN                       // arm AppendObject r
        d.hNode := 0; d.sKey := 'r';
        d.nCmd := 11;
        st := 39;
    ELSIF st = 39 THEN                       // reap -> hR2
        d.nCmd := 0;
        hR2 := d.hOut;
        st := 40;
    ELSIF st = 40 THEN                       // arm AppendString a=v
        d.hNode := hR2; d.sKey := 'a'; d.sVal := 'v';
        d.nCmd := 13;
        st := 41;
    ELSIF st = 41 THEN                       // reap
        d.nCmd := 0;
        st := 42;
    ELSIF st = 42 THEN                       // arm AppendInt n=42
        d.hNode := hR2; d.sKey := 'n'; d.nInt := 42;
        d.nCmd := 14;
        st := 43;
    ELSIF st = 43 THEN                       // reap
        d.nCmd := 0;
        st := 44;
    ELSIF st = 44 THEN                       // arm ToJson built tree
        d.hNode := 0;
        d.nCmd := 10;
        st := 45;
    ELSIF st = 45 THEN                       // reap
        d.nCmd := 0;
        IF FIND(d.sOut, '{"a":"v","n":42}') = 1 THEN
            mask := mask OR 8192;
        END_IF;
        st := 46;
    ELSIF st = 46 THEN                       // arm malformed Load
        d.sIn := '{"a":}';
        d.nCmd := 1;
        st := 47;
    ELSIF st = 47 THEN                       // reap error
        d.nCmd := 0;
        IF d.bError THEN mask := mask OR 16384; END_IF;
        st := 48;
    END_IF;
    d();
END_PROGRAM
