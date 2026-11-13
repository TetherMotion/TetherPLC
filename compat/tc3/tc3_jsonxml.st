// Tc3_JsonXml -> JSON document surface (pure ST, no filesystem).
// Beckhoff's library is a 322-entry DOM/SAX/XML framework with
// heap-backed documents.  This surface implements the JSON half
// that is meaningful without a DOM allocator:
//   - FB_JsonSaxWriter: builds a JSON document text in a STRING.
//     Commands: 1 Begin (root object), 2 AddString, 3 AddInt,
//     4 AddReal, 5 AddBool, 6 AddNull, 7 End (root close),
//     8 Clear, 9 ObjBegin, 10 ArrBegin, 11 EndContainer.
//     Nesting is tracked on a fixed 16-level container stack;
//     members inside arrays are written without a key.
//   - FB_JsonSaxReader: loads a JSON document and reads members
//     back by key.  sKey accepts a dotted path through objects
//     ('a.b.c') and numeric array indices ('arr.0.name').
//     Get* on a container value returns its raw JSON text.
//   - FB_JsonWriter / FB_JsonReader: TwinCAT-shaped aliases.
// Documented reductions:
//   - No file access (LoadDocumentFromFile / SaveDocumentToFile);
//     persistence is the filesystem service's job.
//   - Strings are fixed-capacity (255); numbers print with 3
//     decimals for REAL members (documented limit).
//   - Escaping covers '"' and '\' only (not \n, \t, \uXXXX).
//   - Path keys cannot contain '.' (it is the separator).

// ---- helpers: numeric -> text (no TO_STRING builtin) ----
FUNCTION F_JsonDigit : STRING[1]
VAR_INPUT
    n : DINT;
END_VAR
    CASE n OF
        0: F_JsonDigit := '0';
        1: F_JsonDigit := '1';
        2: F_JsonDigit := '2';
        3: F_JsonDigit := '3';
        4: F_JsonDigit := '4';
        5: F_JsonDigit := '5';
        6: F_JsonDigit := '6';
        7: F_JsonDigit := '7';
        8: F_JsonDigit := '8';
        9: F_JsonDigit := '9';
    ELSE
        F_JsonDigit := '?';
    END_CASE;
END_FUNCTION

FUNCTION F_JsonIntStr : STRING[24]
VAR_INPUT
    v : DINT;
END_VAR
VAR
    x : DINT;
    s : STRING[24];
END_VAR
    IF v = 0 THEN
        F_JsonIntStr := '0';
        RETURN;
    END_IF;
    x := v;
    IF x < 0 THEN x := -x; END_IF;
    s := '';
    WHILE x > 0 DO
        s := CONCAT(F_JsonDigit(x MOD 10), s);
        x := x / 10;
    END_WHILE;
    IF v < 0 THEN s := CONCAT('-', s); END_IF;
    F_JsonIntStr := s;
END_FUNCTION

// fixed 3 decimals: 12.5 -> "12.500"
FUNCTION F_JsonRealStr : STRING[24]
VAR_INPUT
    f : LREAL;
END_VAR
VAR
    neg : BOOL;
    ip : DINT;
    frac : DINT;
    s : STRING[24];
END_VAR
    neg := f < 0.0;
    IF neg THEN f := -f; END_IF;
    ip := LREAL_TO_DINT(f);
    frac := LREAL_TO_DINT((f - DINT_TO_LREAL(ip)) * 1000.0 + 0.5);
    IF frac > 999 THEN
        ip := ip + 1;
        frac := 0;
    END_IF;
    s := CONCAT(F_JsonIntStr(ip), '.');
    IF frac < 10 THEN
        s := CONCAT(s, '00');
    ELSIF frac < 100 THEN
        s := CONCAT(s, '0');
    END_IF;
    s := CONCAT(s, F_JsonIntStr(frac));
    IF neg THEN s := CONCAT('-', s); END_IF;
    F_JsonRealStr := s;
END_FUNCTION

// parse a leading decimal integer (no STRING_TO_DINT builtin)
FUNCTION F_JsonStrInt : DINT
VAR_INPUT
    s : STRING[127];
END_VAR
VAR
    i : DINT;
    n : DINT;
    neg : BOOL;
    c : STRING[1];
    v : DINT;
END_VAR
    n := 0; neg := FALSE; i := 1;
    IF LEN(s) = 0 THEN
        F_JsonStrInt := 0;
        RETURN;
    END_IF;
    IF FIND(MID(s, 1, 1), '-') = 1 THEN neg := TRUE; i := 2; END_IF;
    WHILE i <= LEN(s) DO
        c := MID(s, 1, i);
        // FIND returns the 1-based digit position, 0 if not a digit
        v := FIND('0123456789', c) - 1;
        IF v < 0 THEN
            i := LEN(s) + 1;   // stop at first non-digit
        ELSE
            n := n * 10 + v;
            i := i + 1;
        END_IF;
    END_WHILE;
    IF neg THEN n := -n; END_IF;
    F_JsonStrInt := n;
END_FUNCTION

// parse a leading decimal number with up tend 3 decimals
FUNCTION F_JsonStrReal : LREAL
VAR_INPUT
    s : STRING[127];
END_VAR
VAR
    pos : DINT;
    ipart : DINT;
    fpart : DINT;
    f : LREAL;
    i : DINT;
    c : STRING[1];
    v : DINT;
END_VAR
    ipart := F_JsonStrInt(s);
    f := DINT_TO_LREAL(ipart);
    pos := FIND(s, '.');
    IF pos > 0 AND pos < LEN(s) THEN
        fpart := 0; i := pos + 1; v := 0;
        WHILE i <= LEN(s) AND v >= 0 DO
            c := MID(s, 1, i);
            v := FIND('0123456789', c) - 1;
            IF v >= 0 THEN
                fpart := fpart * 10 + v;
                i := i + 1;
            END_IF;
        END_WHILE;
        f := f + DINT_TO_LREAL(fpart) / 1000.0;
    END_IF;
    F_JsonStrReal := f;
END_FUNCTION

// escape '"' and '\' for JSON string values
FUNCTION F_JsonEsc : STRING[255]
VAR_INPUT
    s : STRING[127];
END_VAR
VAR
    i : DINT;
    c : STRING[1];
    o : STRING[255];
END_VAR
    o := '';
    FOR i := 1 TO LEN(s) DO
        c := MID(s, 1, i);
        IF FIND(c, '"') = 1 OR FIND(c, '\') = 1 THEN
            o := CONCAT(o, '\');
        END_IF;
        o := CONCAT(o, c);
    END_FOR;
    F_JsonEsc := o;
END_FUNCTION

// member/element prefix for the writer: ',' when the container
// already holds a value, plus '"key":' when the parent is an
// object.  Returns '' at depth 0 (caller must not be there).
FUNCTION F_JsonPreamble : STRING[40]
VAR_INPUT
    bObjParent : BOOL;
    bFirst : BOOL;
    sKey : STRING[31];
END_VAR
VAR
    p : STRING[40];
END_VAR
    p := '';
    IF NOT bFirst THEN p := ','; END_IF;
    IF bObjParent THEN
        p := CONCAT(p, CONCAT('"', CONCAT(sKey, '":')));
    END_IF;
    F_JsonPreamble := p;
END_FUNCTION

// position (1-based) of the bracket matching the '{' or '[' at
// pos, honouring strings and escapes; 0 when unbalanced.
FUNCTION F_JsonScanEnd : DINT
VAR_INPUT
    doc : STRING[255];
    pos : DINT;
END_VAR
VAR
    i : DINT;
    d : DINT;
    c : STRING[1];
    ins : BOOL;
    esc : BOOL;
END_VAR
    d := 0; ins := FALSE; esc := FALSE; i := pos;
    WHILE i <= LEN(doc) DO
        c := MID(doc, 1, i);
        IF ins THEN
            IF esc THEN
                esc := FALSE;
            ELSIF FIND(c, '\') = 1 THEN
                esc := TRUE;
            ELSIF FIND(c, '"') = 1 THEN
                ins := FALSE;
            END_IF;
        ELSE
            IF FIND(c, '"') = 1 THEN
                ins := TRUE;
            ELSIF FIND(c, '{') = 1 OR FIND(c, '[') = 1 THEN
                d := d + 1;
            ELSIF FIND(c, '}') = 1 OR FIND(c, ']') = 1 THEN
                d := d - 1;
                IF d = 0 THEN
                    F_JsonScanEnd := i;
                    RETURN;
                END_IF;
            END_IF;
        END_IF;
        i := i + 1;
    END_WHILE;
    F_JsonScanEnd := 0;
END_FUNCTION

// first value position (1-based) of member key inside the object
// spanning (frm..tend); nested containers are skipped by depth.
// Returns 0 when absent.
FUNCTION F_JsonObjFind : DINT
VAR_INPUT
    doc : STRING[255];
    frm : DINT;
    tend : DINT;
    key : STRING[31];
END_VAR
VAR
    i : DINT;
    j : DINT;
    k : DINT;
    v : DINT;
    d : DINT;
    c : STRING[1];
    cj : STRING[1];
    tok : STRING[35];
    closed : BOOL;
    esc2 : BOOL;
END_VAR
    i := frm; d := 0;
    WHILE i < tend DO
        c := MID(doc, 1, i);
        IF FIND(c, '"') = 1 THEN
            // scan tend the closing quote
            j := i + 1; closed := FALSE; esc2 := FALSE;
            WHILE j < tend AND NOT closed DO
                cj := MID(doc, 1, j);
                IF esc2 THEN
                    esc2 := FALSE;
                ELSIF FIND(cj, '\') = 1 THEN
                    esc2 := TRUE;
                ELSIF FIND(cj, '"') = 1 THEN
                    closed := TRUE;
                END_IF;
                j := j + 1;
            END_WHILE;
            IF NOT closed THEN
                i := tend;
            ELSE
                IF d = 0 THEN
                    tok := MID(doc, j - i - 2, i + 1);
                    k := j;
                    WHILE k < tend AND FIND(MID(doc, 1, k), ' ') = 1 DO
                        k := k + 1;
                    END_WHILE;
                    IF k < tend AND FIND(MID(doc, 1, k), ':') = 1 THEN
                        IF LEN(tok) = LEN(key) AND FIND(tok, key) = 1 THEN
                            v := k + 1;
                            WHILE v < tend
                                  AND FIND(MID(doc, 1, v), ' ') = 1 DO
                                v := v + 1;
                            END_WHILE;
                            F_JsonObjFind := v;
                            RETURN;
                        END_IF;
                    END_IF;
                END_IF;
                i := j;
            END_IF;
        ELSIF FIND(c, '{') = 1 OR FIND(c, '[') = 1 THEN
            d := d + 1;
            i := i + 1;
        ELSIF FIND(c, '}') = 1 OR FIND(c, ']') = 1 THEN
            d := d - 1;
            i := i + 1;
        ELSE
            i := i + 1;
        END_IF;
    END_WHILE;
    F_JsonObjFind := 0;
END_FUNCTION

// first value position (1-based) of element idx (0-based) inside
// the array spanning (frm..tend); returns 0 when out of range.
FUNCTION F_JsonArrElem : DINT
VAR_INPUT
    doc : STRING[255];
    frm : DINT;
    tend : DINT;
    idx : DINT;
END_VAR
VAR
    i : DINT;
    e : DINT;
    cnt : DINT;
    c : STRING[1];
    ins : BOOL;
    esc : BOOL;
    done : BOOL;
    skip : BOOL;
END_VAR
    i := frm; cnt := 0;
    WHILE i < tend DO
        // skip separators / blanks at element boundary
        skip := TRUE;
        WHILE i < tend AND skip DO
            c := MID(doc, 1, i);
            IF FIND(c, ' ') = 1 OR FIND(c, ',') = 1 THEN
                i := i + 1;
            ELSE
                skip := FALSE;
            END_IF;
        END_WHILE;
        IF i >= tend THEN
            i := tend;
        ELSE
            IF cnt = idx THEN
                F_JsonArrElem := i;
                RETURN;
            END_IF;
            cnt := cnt + 1;
            c := MID(doc, 1, i);
            IF FIND(c, '{') = 1 OR FIND(c, '[') = 1 THEN
                e := F_JsonScanEnd(doc, i);
                IF e = 0 THEN
                    i := tend;
                ELSE
                    i := e + 1;
                END_IF;
            ELSE
                // scalar / string element: run tend the next ','
                ins := FALSE; esc := FALSE; done := FALSE;
                WHILE i < tend AND NOT done DO
                    c := MID(doc, 1, i);
                    IF ins THEN
                        IF esc THEN
                            esc := FALSE;
                        ELSIF FIND(c, '\') = 1 THEN
                            esc := TRUE;
                        ELSIF FIND(c, '"') = 1 THEN
                            ins := FALSE;
                        END_IF;
                        i := i + 1;
                    ELSE
                        IF FIND(c, '"') = 1 THEN
                            ins := TRUE;
                            i := i + 1;
                        ELSIF FIND(c, ',') = 1 THEN
                            done := TRUE;
                        ELSE
                            i := i + 1;
                        END_IF;
                    END_IF;
                END_WHILE;
            END_IF;
        END_IF;
    END_WHILE;
    F_JsonArrElem := 0;
END_FUNCTION

// ---- FB_JsonSaxWriter: build a JSON document ----
FUNCTION_BLOCK FB_JsonSaxWriter
VAR_INPUT
    nCmd : UDINT;               // 1 Begin 2 AddString 3 AddInt
                                // 4 AddReal 5 AddBool 6 AddNull
                                // 7 End 8 Clear
                                // 9 ObjBegin 10 ArrBegin 11 EndAny
    sKey : STRING[31];
    sVal : STRING[127];
    nInt : DINT;
    fReal : LREAL;
    bBool : BOOL;
END_VAR
VAR_OUTPUT
    sDoc : STRING[255];
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    depth : UDINT;
    ctype : ARRAY[0..15] OF UDINT;   // 1 = object, 2 = array
    cfirst : ARRAY[0..15] OF BOOL;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 8 THEN
            sDoc := ''; depth := 0; bDone := TRUE;
        ELSIF nCmd = 1 THEN
            sDoc := '{'; depth := 1;
            ctype[1] := 1; cfirst[1] := TRUE; bDone := TRUE;
        ELSIF nCmd = 9 OR nCmd = 10 THEN
            IF depth >= 15 THEN
                bError := TRUE; nErrId := 4;
            ELSIF depth = 0 THEN
                IF nCmd = 9 THEN sDoc := '{'; ELSE sDoc := '['; END_IF
                depth := 1;
                ctype[1] := nCmd - 8; cfirst[1] := TRUE;
                bDone := TRUE;
            ELSE
                sDoc := CONCAT(sDoc, F_JsonPreamble(
                    ctype[depth] = 1, cfirst[depth], sKey));
                IF nCmd = 9 THEN
                    sDoc := CONCAT(sDoc, '{');
                ELSE
                    sDoc := CONCAT(sDoc, '[');
                END_IF
                depth := depth + 1;
                ctype[depth] := nCmd - 8;
                cfirst[depth] := TRUE;
                cfirst[depth - 1] := FALSE;
                bDone := TRUE;
            END_IF;
        ELSIF nCmd = 7 THEN
            IF depth <> 1 THEN
                bError := TRUE; nErrId := 5;
            ELSE
                IF ctype[1] = 1 THEN
                    sDoc := CONCAT(sDoc, '}');
                ELSE
                    sDoc := CONCAT(sDoc, ']');
                END_IF
                depth := 0; bDone := TRUE;
            END_IF;
        ELSIF nCmd = 11 THEN
            IF depth = 0 THEN
                bError := TRUE; nErrId := 5;
            ELSE
                IF ctype[depth] = 1 THEN
                    sDoc := CONCAT(sDoc, '}');
                ELSE
                    sDoc := CONCAT(sDoc, ']');
                END_IF
                depth := depth - 1;
                IF depth > 0 THEN cfirst[depth] := FALSE; END_IF
                bDone := TRUE;
            END_IF;
        ELSE
            IF depth = 0 THEN
                bError := TRUE; nErrId := 3;
            ELSE
                sDoc := CONCAT(sDoc, F_JsonPreamble(
                    ctype[depth] = 1, cfirst[depth], sKey));
                IF nCmd = 2 THEN
                    sDoc := CONCAT(sDoc, CONCAT('"',
                        CONCAT(F_JsonEsc(sVal), '"')));
                ELSIF nCmd = 3 THEN
                    sDoc := CONCAT(sDoc, F_JsonIntStr(nInt));
                ELSIF nCmd = 4 THEN
                    sDoc := CONCAT(sDoc, F_JsonRealStr(fReal));
                ELSIF nCmd = 5 THEN
                    IF bBool THEN
                        sDoc := CONCAT(sDoc, 'true');
                    ELSE
                        sDoc := CONCAT(sDoc, 'false');
                    END_IF;
                ELSIF nCmd = 6 THEN
                    sDoc := CONCAT(sDoc, 'null');
                ELSE
                    bError := TRUE; nErrId := 1;
                END_IF;
                cfirst[depth] := FALSE;
                bDone := TRUE;
            END_IF;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_JsonSaxReader: member access, dotted paths ----
FUNCTION_BLOCK FB_JsonSaxReader
VAR_INPUT
    nCmd : UDINT;               // 1 Load 2 GetString 3 GetInt
                                // 4 GetReal 5 GetBool
    sIn : STRING[255];
    sKey : STRING[31];
END_VAR
VAR_OUTPUT
    sOut : STRING[127];
    nOut : DINT;
    fOut : LREAL;
    bOut : BOOL;
    bFound : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    doc : STRING[255];
    pos : DINT;
    tend : DINT;
    v : DINT;
    i : DINT;
    j : DINT;
    e : DINT;
    k : DINT;
    nseg : INT;
    idx : DINT;
    ok : BOOL;
    extracted : BOOL;
    root : DINT;
    c : STRING[1];
    c2 : STRING[1];
    seg : ARRAY[1..8] OF STRING[31];
    val : STRING[127];
    ins : BOOL;
    esc : BOOL;
    done2 : BOOL;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            doc := sIn; bDone := TRUE;
        ELSE
            bFound := FALSE;
            // split sKey on '.' into up tend 8 path segments
            nseg := 1;
            FOR i := 1 TO 8 DO seg[i] := ''; END_FOR;
            seg[1] := '';
            FOR i := 1 TO LEN(sKey) DO
                c := MID(sKey, 1, i);
                IF FIND(c, '.') = 1 THEN
                    nseg := nseg + 1;
                    IF nseg > 8 THEN nseg := 8; END_IF;
                ELSE
                    IF nseg <= 8 THEN
                        seg[nseg] := CONCAT(seg[nseg], c);
                    END_IF;
                END_IF;
            END_FOR;
            // locate the root container
            i := 1;
            WHILE i <= LEN(doc)
                  AND FIND(MID(doc, 1, i), ' ') = 1 DO
                i := i + 1;
            END_WHILE;
            root := i;
            IF root > LEN(doc)
               OR (FIND(MID(doc, 1, root), '{') <> 1
                   AND FIND(MID(doc, 1, root), '[') <> 1) THEN
                bError := TRUE; nErrId := 2;
            ELSE
                pos := root;
                tend := F_JsonScanEnd(doc, root);
                IF tend = 0 THEN
                    bError := TRUE; nErrId := 2;
                ELSE
                    ok := TRUE; extracted := FALSE; k := 1;
                    WHILE k <= nseg AND ok AND NOT extracted DO
                        c := MID(doc, 1, pos);
                        IF FIND(c, '[') = 1 THEN
                            idx := F_JsonStrInt(seg[k]);
                            v := F_JsonArrElem(doc, pos + 1, tend, idx);
                        ELSE
                            v := F_JsonObjFind(doc, pos + 1, tend, seg[k]);
                        END_IF;
                        IF v = 0 THEN
                            ok := FALSE;
                        ELSIF k = nseg THEN
                            // extract the value at v
                            WHILE v <= LEN(doc)
                                  AND FIND(MID(doc, 1, v), ' ') = 1 DO
                                v := v + 1;
                            END_WHILE;
                            IF v <= LEN(doc) THEN
                                c := MID(doc, 1, v);
                                IF FIND(c, '"') = 1 THEN
                                    // JSON string value
                                    j := v + 1; ins := FALSE;
                                    esc := FALSE; done2 := FALSE;
                                    WHILE j <= LEN(doc) AND NOT done2 DO
                                        c2 := MID(doc, 1, j);
                                        IF ins THEN
                                            IF esc THEN
                                                esc := FALSE;
                                            ELSIF FIND(c2, '\') = 1 THEN
                                                esc := TRUE;
                                            ELSIF FIND(c2, '"') = 1 THEN
                                                done2 := TRUE;
                                            END_IF;
                                        ELSE
                                            IF FIND(c2, '"') = 1 THEN
                                                ins := TRUE;
                                            END_IF;
                                        END_IF;
                                        j := j + 1;
                                    END_WHILE;
                                    // j-2 .. j-2? content = v+1 .. j-2
                                    sOut := MID(doc, j - v - 2, v + 1);
                                    bFound := TRUE;
                                ELSIF FIND(c, '{') = 1
                                      OR FIND(c, '[') = 1 THEN
                                    // container value: raw JSON text
                                    e := F_JsonScanEnd(doc, v);
                                    IF e > 0 THEN
                                        sOut := MID(doc, e - v + 1, v);
                                        bFound := TRUE;
                                    END_IF;
                                ELSE
                                    // scalar / literal: tend , } ]
                                    e := v; ins := FALSE;
                                    esc := FALSE; done2 := FALSE;
                                    WHILE e <= LEN(doc) AND NOT done2 DO
                                        c2 := MID(doc, 1, e);
                                        IF ins THEN
                                            IF esc THEN
                                                esc := FALSE;
                                            ELSIF FIND(c2, '\') = 1 THEN
                                                esc := TRUE;
                                            ELSIF FIND(c2, '"') = 1 THEN
                                                ins := FALSE;
                                            END_IF;
                                            e := e + 1;
                                        ELSE
                                            IF FIND(c2, '"') = 1 THEN
                                                ins := TRUE;
                                                e := e + 1;
                                            ELSIF FIND(c2, ',') = 1
                                                  OR FIND(c2, '}') = 1
                                                  OR FIND(c2, ']') = 1 THEN
                                                done2 := TRUE;
                                            ELSE
                                                e := e + 1;
                                            END_IF;
                                        END_IF;
                                    END_WHILE;
                                    val := MID(doc, e - v, v);
                                    WHILE LEN(val) > 0
                                          AND FIND(MID(val, 1, LEN(val)),
                                                   ' ') = 1 DO
                                        val := MID(val, LEN(val) - 1, 1);
                                    END_WHILE;
                                    IF FIND(val, 'true') = 1 THEN
                                        bOut := TRUE;
                                    ELSIF FIND(val, 'false') = 1 THEN
                                        bOut := FALSE;
                                    ELSE
                                        nOut := F_JsonStrInt(val);
                                        fOut := F_JsonStrReal(val);
                                    END_IF;
                                    bFound := TRUE;
                                END_IF;
                            END_IF;
                            extracted := TRUE;
                        ELSE
                            // descend into the container at v
                            WHILE v <= LEN(doc)
                                  AND FIND(MID(doc, 1, v), ' ') = 1 DO
                                v := v + 1;
                            END_WHILE;
                            IF v <= LEN(doc) THEN
                                c := MID(doc, 1, v);
                                IF FIND(c, '{') = 1
                                   OR FIND(c, '[') = 1 THEN
                                    tend := F_JsonScanEnd(doc, v);
                                    IF tend = 0 THEN
                                        ok := FALSE;
                                    ELSE
                                        pos := v;
                                        k := k + 1;
                                    END_IF;
                                ELSE
                                    ok := FALSE;
                                END_IF;
                            ELSE
                                ok := FALSE;
                            END_IF;
                        END_IF;
                    END_WHILE;
                    IF bFound THEN
                        bDone := TRUE;
                    ELSE
                        bError := TRUE; nErrId := 2;
                    END_IF;
                END_IF;
            END_IF;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- TwinCAT-shaped aliases ----
FUNCTION_BLOCK FB_JsonWriter
VAR_INPUT
    nCmd : UDINT;
    sKey : STRING[31];
    sVal : STRING[127];
    nInt : DINT;
    fReal : LREAL;
    bBool : BOOL;
END_VAR
VAR_OUTPUT
    sDoc : STRING[255];
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    w : FB_JsonSaxWriter;
END_VAR
    w.nCmd := nCmd; w.sKey := sKey; w.sVal := sVal;
    w.nInt := nInt; w.fReal := fReal; w.bBool := bBool;
    w();
    sDoc := w.sDoc; bDone := w.bDone;
    bError := w.bError; nErrId := w.nErrId;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_JsonReader
VAR_INPUT
    nCmd : UDINT;
    sIn : STRING[255];
    sKey : STRING[31];
END_VAR
VAR_OUTPUT
    sOut : STRING[127];
    nOut : DINT;
    fOut : LREAL;
    bOut : BOOL;
    bFound : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    r : FB_JsonSaxReader;
END_VAR
    r.nCmd := nCmd; r.sIn := sIn; r.sKey := sKey;
    r();
    sOut := r.sOut; nOut := r.nOut; fOut := r.fOut;
    bOut := r.bOut; bFound := r.bFound; bDone := r.bDone;
    bError := r.bError; nErrId := r.nErrId;
END_FUNCTION_BLOCK

// ---- XML half: reduced FB_XmlDomParser ----
// Beckhoff's FB_XmlDomParser is a heap DOM with iterators, typed
// Append*/Get* methods and path queries.  This reduced surface keeps
// the same idea on fixed tables (no allocator):
//   - cmd 1 Load parses sIn (STRING[1023]) into a node table;
//     cmd 2 Clear resets it.
//   - Node ids are UDINT handles; 0 = none.  Node 0 internally is
//     the document; Root returns its first element child.
//   - cmd 3 Root, 4 Child(parent,nIdx 0-based), 5 ChildByName,
//     6 NodeName, 7 NodeText, 8 AttrCount, 9 ChildCount,
//     10 AttrByName, 11 NodeXml (serialize subtree),
//     12 ToXml (serialize whole document).
//   - Build side: 13 AppendChild(parent,name)->hOut,
//     14 AppendAttribute(node,name,value), 15 AppendText(node,text).
// Limits: 48 nodes, depth 16, 4 attributes per node, names 31,
// text 127, attr values 127, output text 511.  No entity decoding
// (&amp; etc. stored raw), no namespaces, no comments/CDATA/PI
// content (skipped), whitespace inside tags is ' ' only, mixed
// content keeps only the concatenated text run.
FUNCTION F_XmlEscV : STRING[511]
VAR_INPUT
    s : STRING[127];
END_VAR
VAR
    i : DINT;
    c : STRING[1];
    o : STRING[511];
END_VAR
    o := '';
    FOR i := 1 TO LEN(s) DO
        c := MID(s, 1, i);
        IF FIND(c, '&') = 1 THEN
            o := CONCAT(o, '&amp;');
        ELSIF FIND(c, '<') = 1 THEN
            o := CONCAT(o, '&lt;');
        ELSIF FIND(c, '>') = 1 THEN
            o := CONCAT(o, '&gt;');
        ELSIF FIND(c, '"') = 1 THEN
            o := CONCAT(o, '&quot;');
        ELSE
            o := CONCAT(o, c);
        END_IF;
    END_FOR;
    F_XmlEscV := o;
END_FUNCTION

FUNCTION_BLOCK FB_XmlDomParser
VAR_INPUT
    nCmd : UDINT;
    hNode : UDINT;
    nIdx : UDINT;
    sName : STRING[31];
    sVal : STRING[127];
    sIn : STRING[1023];
END_VAR
VAR_OUTPUT
    hOut : UDINT;
    sOut : STRING[511];
    nOut : UDINT;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : UDINT;
    nn : UDINT;
    NPAR : ARRAY[0..47] OF INT;
    NFIRST : ARRAY[0..47] OF INT;
    NLAST : ARRAY[0..47] OF INT;
    NNEXT : ARRAY[0..47] OF INT;
    NNAME : ARRAY[0..47] OF STRING[31];
    NTEXT : ARRAY[0..47] OF STRING[127];
    NATT : ARRAY[0..47] OF UDINT;
    XAN : ARRAY[0..191] OF STRING[31];
    XAV : ARRAY[0..191] OF STRING[127];
    // parse scratch
    pos : DINT;
    lt : DINT;
    i : DINT;
    j : DINT;
    k : DINT;
    k2 : DINT;
    idx : UDINT;
    cur : INT;
    depth : INT;
    CSTK : ARRAY[0..15] OF INT;
    nm : STRING[31];
    an : STRING[31];
    av : STRING[127];
    c : STRING[1];
    c2 : STRING[1];
    q : STRING[1];
    ac : UDINT;
    selfc : BOOL;
    ended : BOOL;
    nmstop : BOOL;
    bad : BOOL;
    // serializer scratch
    sp : INT;
    XSX : ARRAY[0..63] OF INT;
    XSK : ARRAY[0..63] OF INT;
    XSA : ARRAY[0..63] OF INT;
    sn : INT;
    sk : INT;
    sa : INT;
    a : UDINT;
    ch : INT;
    cnt : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    hOut := 0; sOut := ''; nOut := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 2 THEN
            nn := 1;
            NPAR[0] := -1; NFIRST[0] := 0; NLAST[0] := 0;
            NNEXT[0] := 0; NNAME[0] := '#doc'; NTEXT[0] := '';
            NATT[0] := 0;
            FOR i := 1 TO 47 DO
                NPAR[i] := 0; NFIRST[i] := 0; NLAST[i] := 0;
                NNEXT[i] := 0; NNAME[i] := ''; NTEXT[i] := '';
                NATT[i] := 0;
                FOR a := 0 TO 3 DO
                    XAN[i * 4 + a] := ''; XAV[i * 4 + a] := '';
                END_FOR;
            END_FOR;
            FOR i := 0 TO 191 DO
                XAN[i] := ''; XAV[i] := '';
            END_FOR;
            bDone := TRUE;
        ELSIF nCmd = 1 THEN
            // ---- parse sIn ----
            nn := 1;
            NPAR[0] := -1; NFIRST[0] := 0; NLAST[0] := 0;
            NNEXT[0] := 0; NNAME[0] := '#doc'; NTEXT[0] := '';
            NATT[0] := 0;
            FOR i := 1 TO 47 DO
                NPAR[i] := 0; NFIRST[i] := 0; NLAST[i] := 0;
                NNEXT[i] := 0; NNAME[i] := ''; NTEXT[i] := '';
                NATT[i] := 0;
            END_FOR;
            depth := 0; pos := 1; cur := 0; bad := FALSE;
            WHILE pos <= LEN(sIn) AND NOT bad DO
                // find next '<'
                lt := 0; i := pos;
                WHILE i <= LEN(sIn) DO
                    IF FIND(MID(sIn, 1, i), '<') = 1 THEN
                        lt := i; i := LEN(sIn) + 1;
                    ELSE
                        i := i + 1;
                    END_IF;
                END_WHILE;
                IF lt = 0 THEN
                    IF cur > 0 AND pos <= LEN(sIn) THEN
                        NTEXT[cur] := CONCAT(NTEXT[cur],
                            MID(sIn, LEN(sIn) - pos + 1, pos));
                    END_IF;
                    pos := LEN(sIn) + 1;
                ELSE
                    IF lt > pos AND cur > 0 THEN
                        NTEXT[cur] := CONCAT(NTEXT[cur],
                            MID(sIn, lt - pos, pos));
                    END_IF;
                    IF lt + 1 > LEN(sIn) THEN
                        bad := TRUE;
                    ELSE
                        c := MID(sIn, 1, lt + 1);
                        IF FIND(c, '/') = 1 THEN
                            // close tag: pop the open element
                            j := lt + 2;
                            WHILE j <= LEN(sIn)
                                  AND FIND(MID(sIn, 1, j), '>') <> 1 DO
                                j := j + 1;
                            END_WHILE;
                            IF depth > 0 THEN
                                cur := NPAR[CSTK[depth]];
                                depth := depth - 1;
                            END_IF;
                            IF j > LEN(sIn) THEN bad := TRUE;
                            ELSE pos := j + 1; END_IF;
                        ELSIF FIND(c, '?') = 1 THEN
                            // processing instruction: skip to '>'
                            j := lt + 2;
                            WHILE j <= LEN(sIn)
                                  AND FIND(MID(sIn, 1, j), '>') <> 1 DO
                                j := j + 1;
                            END_WHILE;
                            IF j > LEN(sIn) THEN bad := TRUE;
                            ELSE pos := j + 1; END_IF;
                        ELSIF FIND(c, '!') = 1 THEN
                            // comment / CDATA / doctype
                            IF FIND(MID(sIn, lt + 1, 8), '[CDATA[') = 1 THEN
                                j := lt + 8;
                                WHILE j + 2 <= LEN(sIn)
                                      AND NOT (FIND(MID(sIn, 1, j), '-') = 1
                                               AND FIND(MID(sIn, 1, j + 1), '-') = 1
                                               AND FIND(MID(sIn, 1, j + 2), ']') = 1) DO
                                    j := j + 1;
                                END_WHILE;
                                IF j + 2 > LEN(sIn) THEN bad := TRUE;
                                ELSE pos := j + 3; END_IF;
                            ELSIF FIND(MID(sIn, lt + 1, 3), '!--') = 1 THEN
                                j := lt + 4;
                                WHILE j + 2 <= LEN(sIn)
                                      AND NOT (FIND(MID(sIn, 1, j), '-') = 1
                                               AND FIND(MID(sIn, 1, j + 1), '-') = 1
                                               AND FIND(MID(sIn, 1, j + 2), '>') = 1) DO
                                    j := j + 1;
                                END_WHILE;
                                IF j + 2 > LEN(sIn) THEN bad := TRUE;
                                ELSE pos := j + 3; END_IF;
                            ELSE
                                j := lt + 2;
                                WHILE j <= LEN(sIn)
                                      AND FIND(MID(sIn, 1, j), '>') <> 1 DO
                                    j := j + 1;
                                END_WHILE;
                                IF j > LEN(sIn) THEN bad := TRUE;
                                ELSE pos := j + 1; END_IF;
                            END_IF;
                        ELSE
                            // open tag: name then attributes
                            k := lt + 1; nm := ''; nmstop := FALSE;
                            WHILE k <= LEN(sIn) AND NOT nmstop DO
                                c2 := MID(sIn, 1, k);
                                IF FIND(c2, '=') = 1 OR FIND(c2, '/') = 1
                                   OR FIND(c2, '>') = 1
                                   OR FIND(c2, ' ') = 1 THEN
                                    nmstop := TRUE;
                                ELSE
                                    nm := CONCAT(nm, c2); k := k + 1;
                                END_IF;
                            END_WHILE;
                            IF LEN(nm) = 0 OR nn >= 48 THEN
                                bad := TRUE;
                            ELSE
                                idx := nn; nn := nn + 1;
                                NNAME[idx] := nm; NPAR[idx] := cur;
                                IF NLAST[cur] = 0 THEN
                                    NFIRST[cur] := idx;
                                ELSE
                                    NNEXT[NLAST[cur]] := idx;
                                END_IF;
                                NLAST[cur] := idx; NNEXT[idx] := 0;
                                NATT[idx] := 0; ac := 0;
                                selfc := FALSE; ended := FALSE;
                                WHILE k <= LEN(sIn) AND NOT ended
                                      AND NOT bad DO
                                    c2 := MID(sIn, 1, k);
                                    IF FIND(c2, ' ') = 1 THEN
                                        k := k + 1;
                                    ELSIF FIND(c2, '>') = 1 THEN
                                        ended := TRUE; pos := k + 1;
                                    ELSIF FIND(c2, '/') = 1 THEN
                                        k2 := k + 1;
                                        WHILE k2 <= LEN(sIn)
                                              AND FIND(MID(sIn, 1, k2), ' ') = 1 DO
                                            k2 := k2 + 1;
                                        END_WHILE;
                                        IF k2 <= LEN(sIn)
                                           AND FIND(MID(sIn, 1, k2), '>') = 1 THEN
                                            selfc := TRUE; ended := TRUE;
                                            pos := k2 + 1;
                                        ELSE
                                            bad := TRUE;
                                        END_IF;
                                    ELSE
                                        // attribute name
                                        an := ''; nmstop := FALSE;
                                        WHILE k <= LEN(sIn) AND NOT nmstop DO
                                            c2 := MID(sIn, 1, k);
                                            IF FIND(c2, '=') = 1
                                               OR FIND(c2, ' ') = 1
                                               OR FIND(c2, '/') = 1
                                               OR FIND(c2, '>') = 1 THEN
                                                nmstop := TRUE;
                                            ELSE
                                                an := CONCAT(an, c2);
                                                k := k + 1;
                                            END_IF;
                                        END_WHILE;
                                        WHILE k <= LEN(sIn)
                                              AND FIND(MID(sIn, 1, k), ' ') = 1 DO
                                            k := k + 1;
                                        END_WHILE;
                                        IF k > LEN(sIn)
                                           OR FIND(MID(sIn, 1, k), '=') <> 1 THEN
                                            bad := TRUE;
                                        ELSE
                                            k := k + 1;
                                            WHILE k <= LEN(sIn)
                                                  AND FIND(MID(sIn, 1, k), ' ') = 1 DO
                                                k := k + 1;
                                            END_WHILE;
                                            IF k > LEN(sIn) THEN
                                                bad := TRUE;
                                            ELSE
                                                // whatever char opens the
                                                // value is the delimiter
                                                q := MID(sIn, 1, k);
                                                k := k + 1; av := '';
                                                WHILE k <= LEN(sIn)
                                                      AND MID(sIn, 1, k) <> q DO
                                                    av := CONCAT(av, MID(sIn, 1, k));
                                                    k := k + 1;
                                                END_WHILE;
                                                IF k > LEN(sIn) THEN
                                                    bad := TRUE;
                                                ELSE
                                                    k := k + 1;
                                                    IF ac < 4 AND LEN(an) > 0 THEN
                                                        XAN[idx * 4 + ac] := an;
                                                        XAV[idx * 4 + ac] := av;
                                                        ac := ac + 1;
                                                    END_IF;
                                                END_IF;
                                            END_IF;
                                        END_IF;
                                    END_IF;
                                END_WHILE;
                                NATT[idx] := ac;
                                IF NOT ended THEN bad := TRUE; END_IF;
                                IF NOT selfc AND NOT bad THEN
                                    IF depth >= 16 THEN
                                        bad := TRUE;
                                    ELSE
                                        depth := depth + 1;
                                        CSTK[depth] := idx; cur := idx;
                                    END_IF;
                                END_IF;
                            END_IF;
                        END_IF;
                    END_IF;
                END_IF;
            END_WHILE;
            IF bad THEN
                bError := TRUE; nErrId := 2;
            ELSE
                bDone := TRUE;
            END_IF;
        ELSIF nCmd = 3 THEN
            hOut := NFIRST[0]; bDone := TRUE;
        ELSIF nCmd = 4 THEN
            // Child: nIdx-th element child of hNode (0-based)
            IF hNode >= nn THEN
                bError := TRUE; nErrId := 3;
            ELSE
                ch := NFIRST[hNode]; cnt := 0; hOut := 0;
                WHILE ch <> 0 DO
                    IF cnt = nIdx THEN
                        hOut := ch; ch := 0;
                    ELSE
                        cnt := cnt + 1; ch := NNEXT[ch];
                    END_IF;
                END_WHILE;
                bDone := TRUE;
            END_IF;
        ELSIF nCmd = 5 THEN
            // ChildByName
            IF hNode >= nn THEN
                bError := TRUE; nErrId := 3;
            ELSE
                ch := NFIRST[hNode]; hOut := 0;
                WHILE ch <> 0 AND hOut = 0 DO
                    IF NNAME[ch] = sName THEN hOut := ch;
                    ELSE ch := NNEXT[ch]; END_IF;
                END_WHILE;
                bDone := TRUE;
            END_IF;
        ELSIF nCmd = 6 THEN
            IF hNode < nn THEN sOut := NNAME[hNode]; bDone := TRUE;
            ELSE bError := TRUE; nErrId := 3; END_IF;
        ELSIF nCmd = 7 THEN
            IF hNode < nn THEN sOut := NTEXT[hNode]; bDone := TRUE;
            ELSE bError := TRUE; nErrId := 3; END_IF;
        ELSIF nCmd = 8 THEN
            IF hNode < nn THEN nOut := NATT[hNode]; bDone := TRUE;
            ELSE bError := TRUE; nErrId := 3; END_IF;
        ELSIF nCmd = 9 THEN
            IF hNode < nn THEN
                ch := NFIRST[hNode]; cnt := 0;
                WHILE ch <> 0 DO cnt := cnt + 1; ch := NNEXT[ch]; END_WHILE;
                nOut := cnt; bDone := TRUE;
            ELSE bError := TRUE; nErrId := 3; END_IF;
        ELSIF nCmd = 10 THEN
            // AttrByName
            sOut := ''; bDone := TRUE;
            IF hNode < nn THEN
                IF NATT[hNode] > 0 THEN
                    FOR a := 0 TO NATT[hNode] - 1 DO
                        IF XAN[hNode * 4 + a] = sName THEN
                            sOut := XAV[hNode * 4 + a];
                        END_IF;
                    END_FOR;
                END_IF;
            ELSE
                bError := TRUE; nErrId := 3;
            END_IF;
        ELSIF nCmd = 11 OR nCmd = 12 THEN
            // NodeXml / ToXml: iterative subtree serializer
            sOut := ''; sp := 0;
            IF nCmd = 12 THEN
                sp := 1; XSX[1] := 0; XSK[1] := 2; XSA[1] := NFIRST[0];
            ELSE
                IF hNode = 0 OR hNode >= nn THEN
                    bError := TRUE; nErrId := 3;
                ELSE
                    sp := 1; XSX[1] := hNode; XSK[1] := 1; XSA[1] := 0;
                END_IF;
            END_IF;
            IF NOT bError THEN
                WHILE sp > 0 DO
                    sn := XSX[sp]; sk := XSK[sp]; sa := XSA[sp];
                    sp := sp - 1;
                    IF sk = 1 THEN
                        // open node sn
                        sOut := CONCAT(sOut, CONCAT('<', NNAME[sn]));
                        IF NATT[sn] > 0 THEN
                        FOR a := 0 TO NATT[sn] - 1 DO
                            sOut := CONCAT(sOut, CONCAT(' ',
                                CONCAT(XAN[sn * 4 + a],
                                    CONCAT('="',
                                        CONCAT(F_XmlEscV(XAV[sn * 4 + a]),
                                            '"')))));
                        END_FOR;
                        END_IF;
                        IF NFIRST[sn] = 0 THEN
                            IF NTEXT[sn] = '' THEN
                                sOut := CONCAT(sOut, '/>');
                            ELSE
                                sOut := CONCAT(sOut, CONCAT('>',
                                    CONCAT(F_XmlEscV(NTEXT[sn]),
                                        CONCAT('</',
                                            CONCAT(NNAME[sn], '>')))));
                            END_IF;
                        ELSE
                            sOut := CONCAT(sOut, '>');
                            sp := sp + 1; XSX[sp] := sn; XSK[sp] := 3;
                            XSA[sp] := 0;
                            sp := sp + 1; XSX[sp] := sn; XSK[sp] := 2;
                            XSA[sp] := NFIRST[sn];
                        END_IF;
                    ELSIF sk = 2 THEN
                        // advance to next sibling sa under parent sn
                        IF sa <> 0 THEN
                            sp := sp + 1; XSX[sp] := sn; XSK[sp] := 2;
                            XSA[sp] := NNEXT[sa];
                            sp := sp + 1; XSX[sp] := sa; XSK[sp] := 1;
                            XSA[sp] := 0;
                        END_IF;
                    ELSE
                        // close node sn
                        sOut := CONCAT(sOut, CONCAT('</',
                            CONCAT(NNAME[sn], '>')));
                    END_IF;
                END_WHILE;
                bDone := TRUE;
            END_IF;
        ELSIF nCmd = 13 THEN
            // AppendChild: new element under hNode named sName
            IF hNode >= nn OR nn >= 48 THEN
                bError := TRUE; nErrId := 3;
            ELSE
                idx := nn; nn := nn + 1;
                NNAME[idx] := sName; NPAR[idx] := hNode;
                NTEXT[idx] := ''; NATT[idx] := 0;
                IF NLAST[hNode] = 0 THEN
                    NFIRST[hNode] := idx;
                ELSE
                    NNEXT[NLAST[hNode]] := idx;
                END_IF;
                NLAST[hNode] := idx; NNEXT[idx] := 0;
                hOut := idx; bDone := TRUE;
            END_IF;
        ELSIF nCmd = 14 THEN
            // AppendAttribute on hNode
            IF hNode >= nn THEN
                bError := TRUE; nErrId := 3;
            ELSIF NATT[hNode] >= 4 THEN
                bError := TRUE; nErrId := 4;
            ELSE
                a := NATT[hNode];
                XAN[hNode * 4 + a] := sName;
                XAV[hNode * 4 + a] := sVal;
                NATT[hNode] := a + 1; bDone := TRUE;
            END_IF;
        ELSIF nCmd = 15 THEN
            // AppendText on hNode
            IF hNode >= nn THEN
                bError := TRUE; nErrId := 3;
            ELSE
                NTEXT[hNode] := CONCAT(NTEXT[hNode], sVal);
                bDone := TRUE;
            END_IF;
        ELSE
            bError := TRUE; nErrId := 1;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// TwinCAT-shaped alias
FUNCTION_BLOCK FB_XmlDom
VAR_INPUT
    nCmd : UDINT;
    hNode : UDINT;
    nIdx : UDINT;
    sName : STRING[31];
    sVal : STRING[127];
    sIn : STRING[1023];
END_VAR
VAR_OUTPUT
    hOut : UDINT;
    sOut : STRING[511];
    nOut : UDINT;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    x : FB_XmlDomParser;
END_VAR
    x.nCmd := nCmd; x.hNode := hNode; x.nIdx := nIdx;
    x.sName := sName; x.sVal := sVal; x.sIn := sIn;
    x();
    hOut := x.hOut; sOut := x.sOut; nOut := x.nOut;
    bDone := x.bDone; bError := x.bError; nErrId := x.nErrId;
END_FUNCTION_BLOCK
