// tc2_qrc_case.st — QRC e2e: EAN-13 + UPC-A check-digit
// validation on known barcodes.
PROGRAM tc2_qrc_case
VAR
    q : FB_QrcValidate;
    st : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        q.Execute := TRUE;
        st := 1;
    ELSIF st = 1 THEN
        // EAN-13 5901234123457 — check digit 7
        q.D0 := 5; q.D1 := 9; q.D2 := 0; q.D3 := 1; q.D4 := 2;
        q.D5 := 3; q.D6 := 4; q.D7 := 1; q.D8 := 2; q.D9 := 3;
        q.D10 := 4; q.D11 := 5; q.D12 := 7;
        q.IsEan13 := TRUE;
        st := 2;
    ELSIF st = 2 THEN
        IF q.CodeOk THEN mask := mask + 1; END_IF;
        st := 3;
    ELSIF st = 3 THEN
        // EAN-13 wrong check — should be invalid
        q.D12 := 9;
        st := 4;
    ELSIF st = 4 THEN
        IF NOT q.CodeOk THEN mask := mask + 2; END_IF;
        st := 5;
    ELSIF st = 5 THEN
        // UPC-A 036000291452 — check digit 2
        q.D0 := 0; q.D1 := 3; q.D2 := 6; q.D3 := 0; q.D4 := 0;
        q.D5 := 0; q.D6 := 2; q.D7 := 9; q.D8 := 1; q.D9 := 4;
        q.D10 := 5; q.D11 := 2;
        q.IsEan13 := FALSE;
        st := 6;
    ELSIF st = 6 THEN
        IF q.CodeOk THEN mask := mask + 4; END_IF;
        st := 7;
    END_IF;

    q();
END_PROGRAM
