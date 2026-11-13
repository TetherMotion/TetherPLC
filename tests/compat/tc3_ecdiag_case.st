// tc3_ecdiag_case.st — Tc3_EtherCATDiag e2e against
// tether_io_motion_server (TPLC_EC_IO): read the simulated CoE
// 0x10F3 diag history for slave 0 (3 entries: info/warning/error),
// verify decoded fields, acknowledge, re-read, then check slave 1
// (2 entries) and slave 5 (empty).  ELSIF chain prevents same-scan
// state cascade.
PROGRAM tc3_ecdiag_case
VAR
    rd : FB_CoE_DiagHistory_Read;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    ok1 : BOOL;
    ok2 : BOOL;
    ok3 : BOOL;
    ok4 : BOOL;
    ok5 : BOOL;
    ok6 : BOOL;
    ok7 : BOOL;
    ok8 : BOOL;
    ok9 : BOOL;
END_VAR
    IF st = 0 THEN
        mask := 0;
        waitN := 0;
        rd.nSlaveAddr := 0;
        rd.bEnable := TRUE;
        rd.bAck := FALSE;
        rd.nReadIntervalScans := 100000;
        st := 1;
    ELSIF st = 1 THEN
        // first read complete when bValid or bError
        IF rd.bValid OR rd.bError THEN
            ok1 := rd.oCount = 3 AND rd.oNewest = 3 AND
                    rd.oAcked = 2 AND rd.oNewMsgs = 1 AND
                    NOT rd.bError;
            IF ok1 THEN mask := mask + 1; END_IF;
            ok2 := rd.aEMsgType[0] = 1 AND rd.aEMsgType[1] = 2 AND
                    rd.aEMsgType[2] = 3;
            IF ok2 THEN mask := mask + 2; END_IF;
            ok3 := rd.aEAckStatus[0] = 1 AND rd.aEAckStatus[1] = 1 AND
                    rd.aEAckStatus[2] = 0;
            IF ok3 THEN mask := mask + 4; END_IF;
            ok4 := rd.aEDiagCode[0] = 16#1000 AND
                    rd.aEDiagCode[1] = 16#1001 AND
                    rd.aEDiagCode[2] = 16#E0008105 AND
                    rd.aETextId[0] = 16#1100 AND
                    rd.aETextId[2] = 16#8105;
            IF ok4 THEN mask := mask + 8; END_IF;
            ok5 := rd.aEModuleNo[2] = 1 AND rd.aEChannelNo[2] = 2 AND
                    rd.aEDiagCodeType[2] = 3;
            IF ok5 THEN mask := mask + 16; END_IF;
            ok6 := rd.aETimestampMs[0] = 1000 AND
                    rd.aETimestampMs[1] = 2000 AND
                    rd.aETimestampMs[2] = 3000;
            IF ok6 THEN mask := mask + 32; END_IF;
            rd.bAck := TRUE;
            st := 2;
        ELSIF waitN > 60 THEN
            st := 90;
        ELSE
            waitN := waitN + 1;
        END_IF;
    ELSIF st = 2 THEN
        // ack edge consumed; wait for the post-ack read
        IF rd.bValid OR rd.bError THEN
            ok7 := rd.oAcked = 3 AND rd.oNewMsgs = 0 AND
                    rd.aEAckStatus[0] = 1 AND rd.aEAckStatus[1] = 1 AND
                    rd.aEAckStatus[2] = 1;
            IF ok7 THEN mask := mask + 64; END_IF;
            rd.bAck := FALSE;
            rd.bEnable := FALSE;
            rd.nSlaveAddr := 1;
            waitN := 0;
            st := 3;
        ELSIF waitN > 60 THEN
            st := 90;
        ELSE
            waitN := waitN + 1;
        END_IF;
    ELSIF st = 3 THEN
        rd.bEnable := TRUE;   // rising edge -> read slave 1
        st := 4;
    ELSIF st = 4 THEN
        IF rd.bValid OR rd.bError THEN
            ok8 := rd.oCount = 2 AND rd.oNewest = 2;
            IF ok8 THEN mask := mask + 128; END_IF;
            rd.bEnable := FALSE;
            rd.nSlaveAddr := 5;
            waitN := 0;
            st := 5;
        ELSIF waitN > 60 THEN
            st := 90;
        ELSE
            waitN := waitN + 1;
        END_IF;
    ELSIF st = 5 THEN
        rd.bEnable := TRUE;
        st := 6;
    ELSIF st = 6 THEN
        IF rd.bValid OR rd.bError THEN
            ok9 := rd.oCount = 0 AND rd.oNewest = 0 AND
                    NOT rd.bError;
            IF ok9 THEN mask := mask + 256; END_IF;
            rd.bEnable := FALSE;
            st := 90;
        ELSIF waitN > 60 THEN
            st := 90;
        ELSE
            waitN := waitN + 1;
        END_IF;
    ELSIF st = 90 THEN
        // done; mask holds results
    END_IF
    rd();
END_PROGRAM
