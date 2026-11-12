// tc2_rfid_case.st — wired e2e for tc2_rfid (FB_RFIDReader).
PROGRAM tc2_rfid_case
VAR
    rd : FB_RFIDReader;
    st : INT;
    tries : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        rd.sDevice := '/tmp/rfid_pty';
        rd.nBaud := 9600; rd.nParity := 0;
        rd.nDataBits := 8; rd.nStopBits := 1;
        rd.sTxBuffer := 'READ';
        rd.nTxLen := 4;
        rd.tTimeout := T#500MS;
        rd.bExecute := TRUE;
        st := 1;
    ELSIF st = 1 THEN
        IF rd.bDone THEN mask := mask OR 1; END_IF;
        IF rd.hReader <> 0 THEN mask := mask OR 2; END_IF;
        IF rd.nRxLen > 0 THEN mask := mask OR 4; END_IF;
        IF FIND(rd.sRxBuffer, 'TAG:ABC12345') > 0 THEN
            mask := mask OR 8;
            rd.bExecute := FALSE;
            st := 2;
        ELSIF tries < 20 THEN
            // reader answers asynchronously: release, re-arm, retry
            tries := tries + 1;
            rd.bExecute := FALSE;
            st := 5;
        ELSE
            rd.bExecute := FALSE;
            st := 2;
        END_IF;
    ELSIF st = 5 THEN
        rd.bExecute := TRUE;                 // re-arm the cycle
        st := 6;
    ELSIF st = 6 THEN
        st := 1;
    ELSIF st = 2 THEN
        rd.bClose := TRUE;
        st := 3;
    ELSIF st = 3 THEN
        IF rd.hReader = 0 THEN mask := mask OR 16; END_IF;
        rd.bClose := FALSE;
        st := 4;
    END_IF;
    rd();
END_PROGRAM
