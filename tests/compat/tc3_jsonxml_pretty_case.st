// tc3_jsonxml_pretty_case.st — e2e for FB_JsonSaxPrettyWriter:
// indented JSON output.  Every command needs a dedicated arm
// state and a reap state that clears nCmd (edge detect prev=0).
PROGRAM tc3_jsonxml_pretty_case
VAR
    pw : FB_JsonSaxPrettyWriter;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        pw.nInt := 2;
        pw.nCmd := 12;                       // SetIndent 2
        st := 1;
    ELSIF st = 1 THEN                        // reap
        pw.nCmd := 0;
        st := 2;
    ELSIF st = 2 THEN                        // arm Begin
        pw.nCmd := 1;
        st := 3;
    ELSIF st = 3 THEN                        // reap
        pw.nCmd := 0;
        st := 4;
    ELSIF st = 4 THEN                        // arm AddString name
        pw.sKey := 'name'; pw.sVal := 'ax1';
        pw.nCmd := 2;
        st := 5;
    ELSIF st = 5 THEN                        // reap
        pw.nCmd := 0;
        st := 6;
    ELSIF st = 6 THEN                        // arm ObjBegin dev
        pw.sKey := 'dev';
        pw.nCmd := 9;
        st := 7;
    ELSIF st = 7 THEN                        // reap
        pw.nCmd := 0;
        st := 8;
    ELSIF st = 8 THEN                        // arm AddInt id
        pw.sKey := 'id'; pw.nInt := 7;
        pw.nCmd := 3;
        st := 9;
    ELSIF st = 9 THEN                        // reap
        pw.nCmd := 0;
        st := 10;
    ELSIF st = 10 THEN                       // arm AddReal spd
        pw.sKey := 'spd'; pw.fReal := 1.5;
        pw.nCmd := 4;
        st := 11;
    ELSIF st = 11 THEN                       // reap
        pw.nCmd := 0;
        st := 12;
    ELSIF st = 12 THEN                       // arm AddBool en
        pw.sKey := 'en'; pw.bBool := TRUE;
        pw.nCmd := 5;
        st := 13;
    ELSIF st = 13 THEN                       // reap
        pw.nCmd := 0;
        st := 14;
    ELSIF st = 14 THEN                       // arm EndAny dev
        pw.nCmd := 11;
        st := 15;
    ELSIF st = 15 THEN                       // reap
        pw.nCmd := 0;
        st := 16;
    ELSIF st = 16 THEN                       // arm ArrBegin axes
        pw.sKey := 'axes';
        pw.nCmd := 10;
        st := 17;
    ELSIF st = 17 THEN                       // reap
        pw.nCmd := 0;
        st := 18;
    ELSIF st = 18 THEN                       // arm AddInt 1
        pw.nInt := 1;
        pw.nCmd := 3;
        st := 19;
    ELSIF st = 19 THEN                       // reap
        pw.nCmd := 0;
        st := 20;
    ELSIF st = 20 THEN                       // arm AddInt 2
        pw.nInt := 2;
        pw.nCmd := 3;
        st := 21;
    ELSIF st = 21 THEN                       // reap
        pw.nCmd := 0;
        st := 22;
    ELSIF st = 22 THEN                       // arm EndAny axes
        pw.nCmd := 11;
        st := 23;
    ELSIF st = 23 THEN                       // reap
        pw.nCmd := 0;
        st := 24;
    ELSIF st = 24 THEN                       // arm End root
        pw.nCmd := 7;
        st := 25;
    ELSIF st = 25 THEN                       // reap + checks
        pw.nCmd := 0;
        IF pw.bDone THEN mask := mask OR 1; END_IF;
        IF FIND(pw.sDoc, '"dev": {') > 0 THEN
            mask := mask OR 2;
        END_IF;
        IF FIND(pw.sDoc, '
    "id": 7') > 0 THEN
            mask := mask OR 4;
        END_IF;
        IF FIND(pw.sDoc, '"axes": [') > 0 THEN
            mask := mask OR 8;
        END_IF;
        IF FIND(pw.sDoc, '1.500') > 0 THEN
            mask := mask OR 16;
        END_IF;
        IF FIND(pw.sDoc, '
}') > 0 AND NOT pw.bError THEN
            mask := mask OR 32;
        END_IF;
        st := 26;
    END_IF;
    pw();
END_PROGRAM
