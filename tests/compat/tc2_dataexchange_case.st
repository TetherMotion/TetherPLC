// tc2_dataexchange_case.st — e2e for tc2_dataexchange (tplc-sym).
// The program's own variables are addressed by name through the
// package's exported var table.
PROGRAM tc2_dataexchange_case
VAR
    rd : FB_ReadAdsSymByName;
    wr : FB_WriteAdsSymByName;
    dI : FB_WriteDWordOnDelta;
    dR : FB_WriteLRealOnDelta;
    wd : FB_CheckWatchdog;
    heartbeat : DINT;
    setpoint : LREAL;
    flag : BOOL;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        wr.sVarName := 'heartbeat';
        wr.nIntValue := 1234;
        wr.bWrite := TRUE;
        st := 1;
    ELSIF st = 1 THEN
        IF wr.bDone THEN mask := mask OR 1; END_IF;
        wr.bWrite := FALSE;
        IF heartbeat = 1234 THEN mask := mask OR 2; END_IF;
        rd.sVarName := 'heartbeat';
        st := 2;
    ELSIF st = 2 THEN
        rd.bRead := TRUE;
        st := 3;
    ELSIF st = 3 THEN
        IF rd.nIntValue = 1234 AND rd.nSize = 4 THEN mask := mask OR 4; END_IF;
        rd.bRead := FALSE;
        wr.sVarName := 'setpoint';
        wr.fRealValue := 12.5;
        st := 4;
    ELSIF st = 4 THEN
        wr.bWrite := TRUE;
        st := 5;
    ELSIF st = 5 THEN
        IF wr.bDone AND setpoint = 12.5 THEN mask := mask OR 8; END_IF;
        wr.bWrite := FALSE;
        wr.sVarName := 'flag';
        wr.bBoolValue := TRUE;
        st := 6;
    ELSIF st = 6 THEN
        wr.bWrite := TRUE;
        st := 7;
    ELSIF st = 7 THEN
        IF wr.bDone AND flag THEN mask := mask OR 16; END_IF;
        wr.bWrite := FALSE;
        // unknown name must error
        wr.sVarName := 'no_such_var';
        st := 8;
    ELSIF st = 8 THEN
        wr.bWrite := TRUE;
        st := 9;
    ELSIF st = 9 THEN
        IF wr.bError THEN mask := mask OR 32; END_IF;
        wr.bWrite := FALSE;
        // delta writer: transmits on change only
        dI.sVarName := 'heartbeat';
        dI.nValue := 7;
        st := 10;
    ELSIF st = 10 THEN
        dI.bEnable := TRUE;
        st := 11;
    ELSIF st = 11 THEN
        IF dI.nLastSignal = 7 AND heartbeat = 7 THEN mask := mask OR 64; END_IF;
        dR.sVarName := 'setpoint';
        dR.fValue := 3.25;
        st := 12;
    ELSIF st = 12 THEN
        dR.bEnable := TRUE;
        st := 13;
    ELSIF st = 13 THEN
        IF dR.fLastSignal = 3.25 AND setpoint = 3.25 THEN
            mask := mask OR 128;
        END_IF;
        // watchdog: heartbeat is written every scan by the delta
        // block?  No — use a dedicated stamp variable.
        wd.sVarName := 'heartbeat';
        wd.tTimeout := T#1S;
        st := 14;
    ELSIF st = 14 THEN
        wd.bEnable := TRUE;
        st := 15;
    ELSIF st = 15 THEN
        IF wd.bAlive AND NOT wd.bTimeout THEN mask := mask OR 256; END_IF;
        st := 16;
    END_IF;
    rd(); wr(); dI(); dR(); wd();
END_PROGRAM
