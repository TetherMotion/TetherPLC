// tc3_jsonxml_xml_case.st — e2e for the XML half of tc3_jsonxml:
// FB_XmlDomParser load/query/serialize + build commands.
// Command FBs need a full scan with nCmd=0 between commands (edge
// detect on prev=0): every command gets a dedicated arm state, then
// a reap state that clears nCmd and reads the outputs.
PROGRAM tc3_jsonxml_xml_case
VAR
    x : FB_XmlDomParser;
    xdoc : STRING[1023];
    hRoot : UDINT;
    hDev : UDINT;
    hNm : UDINT;
    hPos : UDINT;
    hAx : UDINT;
    hTmp : UDINT;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        xdoc := CONCAT('<?xml version="1.0"?>',
            CONCAT('<root><dev id="ax1" unit="mm">',
            CONCAT('<name>motor</name><pos>100</pos></dev>',
            CONCAT('<axes><axis pos="1"/><axis pos="2"/>',
                   '<axis pos="3"/></axes></root>'))));
        x.sIn := xdoc; x.nCmd := 1;           // Load
        st := 1;
    // reap Load
    ELSIF st = 1 THEN
        x.nCmd := 0;
        IF x.bDone THEN mask := mask OR 1; END_IF;
        st := 2;
    // arm Root
    ELSIF st = 2 THEN
        x.nCmd := 3;
        st := 3;
    // reap Root
    ELSIF st = 3 THEN
        x.nCmd := 0;
        hRoot := x.hOut;
        IF hRoot <> 0 THEN mask := mask OR 2; END_IF;
        st := 4;
    // arm NodeName root
    ELSIF st = 4 THEN
        x.hNode := hRoot;
        x.nCmd := 6;
        st := 5;
    // reap NodeName
    ELSIF st = 5 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, 'root') = 1 THEN
            mask := mask OR 4;
        END_IF;
        st := 6;
    // arm ChildByName dev
    ELSIF st = 6 THEN
        x.hNode := hRoot; x.sName := 'dev';
        x.nCmd := 5;
        st := 7;
    // reap -> hDev
    ELSIF st = 7 THEN
        x.nCmd := 0;
        hDev := x.hOut;
        IF hDev <> 0 THEN mask := mask OR 8; END_IF;
        st := 8;
    // arm AttrByName id
    ELSIF st = 8 THEN
        x.hNode := hDev; x.sName := 'id';
        x.nCmd := 10;
        st := 9;
    // reap attr id
    ELSIF st = 9 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, 'ax1') = 1 THEN
            mask := mask OR 16;
        END_IF;
        st := 10;
    // arm AttrCount dev
    ELSIF st = 10 THEN
        x.hNode := hDev;
        x.nCmd := 8;
        st := 11;
    // reap AttrCount
    ELSIF st = 11 THEN
        x.nCmd := 0;
        IF x.nOut = 2 THEN mask := mask OR 32; END_IF;
        st := 12;
    // arm ChildByName name
    ELSIF st = 12 THEN
        x.hNode := hDev; x.sName := 'name';
        x.nCmd := 5;
        st := 13;
    // reap -> hNm
    ELSIF st = 13 THEN
        x.nCmd := 0;
        hNm := x.hOut;
        st := 14;
    // arm NodeText name
    ELSIF st = 14 THEN
        x.hNode := hNm;
        x.nCmd := 7;
        st := 15;
    // reap NodeText
    ELSIF st = 15 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, 'motor') = 1 THEN
            mask := mask OR 64;
        END_IF;
        st := 16;
    // arm ChildByName pos
    ELSIF st = 16 THEN
        x.hNode := hDev; x.sName := 'pos';
        x.nCmd := 5;
        st := 17;
    // reap -> hPos
    ELSIF st = 17 THEN
        x.nCmd := 0;
        hPos := x.hOut;
        st := 18;
    // arm NodeText pos
    ELSIF st = 18 THEN
        x.hNode := hPos;
        x.nCmd := 7;
        st := 19;
    // reap NodeText pos
    ELSIF st = 19 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, '100') = 1 THEN
            mask := mask OR 128;
        END_IF;
        st := 20;
    // arm ChildByName axes
    ELSIF st = 20 THEN
        x.hNode := hRoot; x.sName := 'axes';
        x.nCmd := 5;
        st := 21;
    // reap -> hAx
    ELSIF st = 21 THEN
        x.nCmd := 0;
        hAx := x.hOut;
        st := 22;
    // arm ChildCount axes
    ELSIF st = 22 THEN
        x.hNode := hAx;
        x.nCmd := 9;
        st := 23;
    // reap ChildCount
    ELSIF st = 23 THEN
        x.nCmd := 0;
        IF x.nOut = 3 THEN mask := mask OR 256; END_IF;
        st := 24;
    // arm Child axes[1] (0-based -> second axis)
    ELSIF st = 24 THEN
        x.hNode := hAx; x.nIdx := 1;
        x.nCmd := 4;
        st := 25;
    // reap -> hTmp
    ELSIF st = 25 THEN
        x.nCmd := 0;
        hTmp := x.hOut;
        st := 26;
    // arm AttrByName pos on axis2
    ELSIF st = 26 THEN
        x.hNode := hTmp; x.sName := 'pos';
        x.nCmd := 10;
        st := 27;
    // reap attr pos
    ELSIF st = 27 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, '2') = 1 THEN
            mask := mask OR 512;
        END_IF;
        st := 28;
    // arm NodeXml dev
    ELSIF st = 28 THEN
        x.hNode := hDev;
        x.nCmd := 11;
        st := 29;
    // reap NodeXml
    ELSIF st = 29 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, 'id="ax1"') > 0
           AND FIND(x.sOut, '<name>') > 0 THEN
            mask := mask OR 1024;
        END_IF;
        st := 30;
    // arm ToXml
    ELSIF st = 30 THEN
        x.hNode := 0;
        x.nCmd := 12;
        st := 31;
    // reap ToXml
    ELSIF st = 31 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, '</root>') > 0
           AND FIND(x.sOut, '<axis') > 0 THEN
            mask := mask OR 2048;
        END_IF;
        st := 32;
    // arm Clear
    ELSIF st = 32 THEN
        x.nCmd := 2;
        st := 33;
    // reap Clear
    ELSIF st = 33 THEN
        x.nCmd := 0;
        st := 34;
    // arm AppendChild r under #doc
    ELSIF st = 34 THEN
        x.hNode := 0; x.sName := 'r';
        x.nCmd := 13;
        st := 35;
    // reap -> hRoot (new tree)
    ELSIF st = 35 THEN
        x.nCmd := 0;
        hRoot := x.hOut;
        st := 36;
    // arm AppendAttribute a=v on r
    ELSIF st = 36 THEN
        x.hNode := hRoot; x.sName := 'a'; x.sVal := 'v';
        x.nCmd := 14;
        st := 37;
    // reap attr
    ELSIF st = 37 THEN
        x.nCmd := 0;
        st := 38;
    // arm AppendChild c under r
    ELSIF st = 38 THEN
        x.hNode := hRoot; x.sName := 'c';
        x.nCmd := 13;
        st := 39;
    // reap -> hNm
    ELSIF st = 39 THEN
        x.nCmd := 0;
        hNm := x.hOut;
        st := 40;
    // arm AppendText hi on c
    ELSIF st = 40 THEN
        x.hNode := hNm; x.sVal := 'hi';
        x.nCmd := 15;
        st := 41;
    // reap text
    ELSIF st = 41 THEN
        x.nCmd := 0;
        st := 42;
    // arm ToXml of built tree
    ELSIF st = 42 THEN
        x.hNode := 0;
        x.nCmd := 12;
        st := 43;
    // reap ToXml
    ELSIF st = 43 THEN
        x.nCmd := 0;
        IF FIND(x.sOut, '<r a="v"><c>hi</c></r>') = 1 THEN
            mask := mask OR 4096;
        END_IF;
        st := 44;
    // arm malformed Load
    ELSIF st = 44 THEN
        x.sIn := '<a x></a>';
        x.nCmd := 1;
        st := 45;
    // reap error
    ELSIF st = 45 THEN
        x.nCmd := 0;
        IF x.bError THEN mask := mask OR 8192; END_IF;
        st := 46;
    END_IF;
    x();
END_PROGRAM
