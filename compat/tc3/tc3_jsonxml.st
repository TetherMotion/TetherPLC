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
