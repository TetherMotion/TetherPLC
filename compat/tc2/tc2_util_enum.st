// Tc2_Utilities enumeration helpers (TwinCAT 3 PLC Library Tc2_Utilities).
// FB_EnumStringNumbers scans a string for numeric substrings.
// Beckhoff types E_EnumCmdType (First=0, Next=1, Abort=2) and
// E_NumGroupTypes (Float=0, Unsigned=1, Signed=2) are plain INTs here —
// TwinCAT callers pass the enum ordinals directly.
// Sources: docs/plc/plan/21-twincat-libraries.md §7.
// Approximation: the consume set is per eType (Float also takes
// '.', 'e', 'E'; Signed takes '+'/'-'), a sign only starts a number
// when followed by a digit, and nPos is the 1-based index after the
// last consumed character.

FUNCTION F_IsDigit : BOOL
VAR_INPUT
    ch : STRING[1];
END_VAR
    F_IsDigit := (ch = '0') OR (ch = '1') OR (ch = '2') OR (ch = '3')
              OR (ch = '4') OR (ch = '5') OR (ch = '6') OR (ch = '7')
              OR (ch = '8') OR (ch = '9');
END_FUNCTION

FUNCTION_BLOCK FB_EnumStringNumbers
VAR_INPUT
    sSearch : STRING;
    eCmd    : INT;
    eType   : INT;
END_VAR
VAR_OUTPUT
    sNumber : STRING;
    nPos    : INT;
    bEOS    : BOOL;
END_VAR
VAR
    cur    : INT;
    i      : INT;
    start  : INT;
    cnt    : INT;
    len    : INT;
    ch     : STRING[1];
    nx     : STRING[1];
    found  : BOOL;
END_VAR
    sNumber := '';
    nPos := 0;
    bEOS := FALSE;
    start := 0;
    cnt := 0;
    found := FALSE;
    IF eCmd = 0 THEN
        cur := 1;
    ELSIF eCmd = 2 THEN
        bEOS := TRUE;
        cur := 9999;
    END_IF;
    len := LEN(sSearch);
    /* locate the first digit (a leading sign counts for eType 0/2
       when a digit follows) */
    i := cur;
    WHILE i <= len DO
        ch := MID(sSearch, 1, i);
        IF F_IsDigit(ch) THEN
            start := i;
            found := TRUE;
            EXIT;
        END_IF
        IF (eType = 0 OR eType = 2) AND i < len
           AND (ch = '-' OR ch = '+') THEN
            nx := MID(sSearch, 1, i + 1);
            IF F_IsDigit(nx) THEN
                start := i;
                found := TRUE;
                EXIT;
            END_IF;
        END_IF;
        i := i + 1;
    END_WHILE;
    IF NOT found THEN
        bEOS := TRUE;
        nPos := len + 1;
        cur := nPos;
    ELSE
        /* consume characters legal for the number format */
        i := start;
        WHILE i <= len DO
            ch := MID(sSearch, 1, i);
            IF F_IsDigit(ch) THEN
                cnt := cnt + 1;
                i := i + 1;
            ELSIF eType = 0 AND (ch = '.' OR ch = 'e' OR ch = 'E'
                                 OR ch = '-' OR ch = '+') THEN
                cnt := cnt + 1;
                i := i + 1;
            ELSIF eType = 2 AND (ch = '-' OR ch = '+') THEN
                cnt := cnt + 1;
                i := i + 1;
            ELSE
                EXIT;
            END_IF;
        END_WHILE;
        sNumber := MID(sSearch, cnt, start);
        nPos := start + cnt;
        cur := nPos;
    END_IF;
END_FUNCTION_BLOCK
