// Tc3_JsonXml -> JSON document surface (pure ST, no filesystem).
// Beckhoff's library is a 322-entry DOM/SAX/XML framework with
// heap-backed documents.  This surface implements the part that
// is meaningful without a DOM allocator:
//   - FB_JsonSaxWriter: builds a flat JSON object text in a
//     STRING (Begin / AddString / AddInt / AddReal / AddBool /
//     AddNull / End / Clear).
//   - FB_JsonSaxReader: loads a flat JSON object text and reads
//     members back by key (GetString / GetInt / GetReal /
//     GetBool).
//   - FB_JsonWriter / FB_JsonReader: TwinCAT-shaped aliases.
// Documented reductions:
//   - Flat objects only: nested objects/arrays, Base64/DateTime/
//     FileTime members and document copy are not modelled.
//   - No escaping: member text must not contain '"' or '\'.
//   - No file access (LoadDocumentFromFile / SaveDocumentToFile);
//     persistence is the filesystem service's job.
//   - Strings are fixed-capacity (255); numbers print with 3
//     decimals for REAL members (documented limit).

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

// parse a leading decimal number with up to 3 decimals
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

// ---- FB_JsonSaxWriter: build a flat JSON object ----
FUNCTION_BLOCK FB_JsonSaxWriter
VAR_INPUT
    nCmd : UDINT;               // 1 Begin 2 AddString 3 AddInt
                                // 4 AddReal 5 AddBool 6 AddNull
                                // 7 End 8 Clear
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
    first : BOOL;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 8 THEN
            sDoc := ''; first := TRUE; bDone := TRUE;
        ELSIF nCmd = 1 THEN
            sDoc := '{'; first := TRUE; bDone := TRUE;
        ELSIF nCmd = 7 THEN
            sDoc := CONCAT(sDoc, '}'); bDone := TRUE;
        ELSE
            IF NOT first THEN sDoc := CONCAT(sDoc, ','); END_IF;
            sDoc := CONCAT(sDoc, CONCAT('"', CONCAT(sKey, '":')));
            IF nCmd = 2 THEN
                sDoc := CONCAT(sDoc, CONCAT('"', CONCAT(sVal, '"')));
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
            first := FALSE;
            bDone := TRUE;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_JsonSaxReader: flat-object member access ----
// Value spans are located with FIND/MID over the loaded text:
// the key is searched as "key", then the value starts after ':'.
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
    rest : STRING[255];
    val : STRING[127];
    q : STRING[35];
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            doc := sIn; bDone := TRUE;
        ELSE
            bFound := FALSE;
            // locate "key"
            q := CONCAT('"', CONCAT(sKey, '"'));
            pos := FIND(doc, q);
            IF pos > 0 THEN
                rest := MID(doc, LEN(doc) - pos + 1 - LEN(q), pos + LEN(q));
                // rest starts right after the closing quote
                pos := FIND(rest, ':');
                IF pos > 0 THEN
                    rest := MID(rest, LEN(rest) - pos, pos + 1);
                    // skip blanks
                    WHILE LEN(rest) > 0
                          AND FIND(MID(rest, 1, 1), ' ') = 1 DO
                        rest := MID(rest, LEN(rest) - 1, 2);
                    END_WHILE;
                    IF LEN(rest) > 0 THEN
                        IF FIND(MID(rest, 1, 1), '"') = 1 THEN
                            // string value: up to next quote
                            rest := MID(rest, LEN(rest) - 1, 2);
                            pos := FIND(rest, '"');
                            IF pos > 0 THEN
                                val := MID(rest, pos - 1, 1);
                                bFound := TRUE;
                                sOut := val;
                            END_IF;
                        ELSE
                            // number / literal: up to , or }
                            pos := FIND(rest, ',');
                            IF pos = 0 THEN pos := FIND(rest, '}'); END_IF;
                            IF pos = 0 THEN pos := LEN(rest) + 1; END_IF;
                            val := MID(rest, pos - 1, 1);
                            bFound := TRUE;
                            IF FIND(val, 'true') = 1 THEN
                                bOut := TRUE;
                            ELSIF FIND(val, 'false') = 1 THEN
                                bOut := FALSE;
                            ELSE
                                nOut := F_JsonStrInt(val);
                                fOut := F_JsonStrReal(val);
                            END_IF;
                        END_IF;
                    END_IF;
                END_IF;
            END_IF;
            IF bFound THEN bDone := TRUE;
            ELSE bError := TRUE; nErrId := 2;
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
