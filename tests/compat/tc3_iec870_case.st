PROGRAM Tc3Iec870Case
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    i2        : INT;
    conn      : FB_IEC870_5_104Master;
    connBad   : FB_IEC870_5_104Master;
    snd       : FB_IEC870_SendAsdu;
    snd2      : FB_IEC870_SendAsdu;
    snd3      : FB_IEC870_SendAsdu;
    rcv       : FB_IEC870_RecvAsdu;
    pay1      : ARRAY[0..239] OF BYTE;
    pay2      : ARRAY[0..239] OF BYTE;
    pay3      : ARRAY[0..239] OF BYTE;
    gotSp     : BOOL;
    gotMe     : BOOL;
    gotCs     : BOOL;
    gotSc     : BOOL;
    // helper-test vars
    o64       : ARRAY[0..6] OF BYTE;
    tenc      : SYSTEMTIME_TO_CP56Time2a;
    tdec      : CP56Time2a_TO_SYSTEMTIME;
    hY        : UDINT;
    hMo       : UDINT;
    hD        : UDINT;
    hH        : UDINT;
    hMi       : UDINT;
    hS        : UDINT;
    hMs       : UDINT;
    hbIV      : BOOL;
    hbSU      : BOOL;
    hVer      : UDINT;
    hCot      : STRING[31];
    hTid      : STRING[31];
    hIoa      : UDINT;
    hSpi      : BOOL;
    hDpi      : UDINT;
    hVti      : UDINT;
END_VAR

// autonomous IEC 60870-5-104 exchange against
// tests/compat/iec870_test_peer.py (TCP 2480). The peer is a
// slave: STARTDT handshake, then answers C_IC_NA_1 with
// M_SP_NA_1 + M_ME_NA_1, echoes C_CS_NA_1, answers C_SC_NA_1
// with an act-con. rcv() runs every scan (consume-on-read);
// each send gets one EXEC edge via its own st step.
rcv(nConnHandle := conn.nConnHandle);
IF rcv.bDone THEN
    // M_SP_NA_1: type1 vsq1 cot3 ca1 payload = IOA(3)+SPI(1)
    IF rcv.nTypeId = 1 AND rcv.nLen = 4 AND
       rcv.pPayload[2] = 1 AND rcv.pPayload[3] = 1 THEN
        gotSp := TRUE;
    END_IF;
    // M_ME_NA_1: type9 payload = IOA(3)+int16+QDS = 6 bytes
    IF rcv.nTypeId = 9 AND rcv.nLen = 6 AND
       rcv.pPayload[2] = 2 AND rcv.pPayload[3] = 16#34 AND
       rcv.pPayload[4] = 16#12 AND rcv.pPayload[5] = 0 THEN
        gotMe := TRUE;
    END_IF;
    IF rcv.nTypeId = 103 THEN
        gotCs := TRUE;
    END_IF;
    IF rcv.nTypeId = 45 THEN
        gotSc := TRUE;
    END_IF;
END_IF;

conn(sHost := '127.0.0.1', nPort := 2480, nCA_Size := 2,
     nCOT_Size := 2, bExecute := st = 0, bClose := st = 10,
     tTimeout := T#3S);
connBad(sHost := '127.0.0.1', nPort := 19999,
        bExecute := st = 8, bClose := FALSE, tTimeout := T#3S);

// payload staging
// pay1: interrogation C_IC_NA_1 has no info objects (len 0)
// pay2: C_CS_NA_1 clock sync, CP56Time2a = 13:45:30.250
// octets0-1 = sec*1000+ms = 30250 -> LE16 0x762A = 42,118
pay2[0] := 42; pay2[1] := 118;
pay2[2] := 45;                  // minute=45
pay2[3] := 13;                  // hour=13
pay2[4] := 15;                  // day=15
pay2[5] := 6;                   // month=6
pay2[6] := 124;                 // year-1900 = 2024-1900 = 124
// pay3: C_SC_NA_1 single command IOA=5, SCS=on -> 4 bytes
pay3[0] := 0; pay3[1] := 0; pay3[2] := 5; pay3[3] := 16#80;

snd(nConnHandle := conn.nConnHandle, nTypeId := 100, nVSQ := 0,
    nCOT := 5, nCA := 1, nLen := 0, pPayload := pay1,
    bExecute := st = 1);
snd2(nConnHandle := conn.nConnHandle, nTypeId := 103, nVSQ := 0,
     nCOT := 6, nCA := 1, nLen := 7, pPayload := pay2,
     bExecute := st = 4);
snd3(nConnHandle := conn.nConnHandle, nTypeId := 45, nVSQ := 1,
     nCOT := 6, nCA := 1, nLen := 4, pPayload := pay3,
     bExecute := st = 6);

IF st = 0 AND conn.bDone THEN
    IF conn.nStatus = 1 AND conn.nConnHandle <> 0 AND
       conn.nSubState = 3 AND conn.nErrId = 0 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND snd.bDone THEN
    IF snd.nStatus = 1 AND snd.nErrId = 0 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND gotSp AND gotMe THEN
    mask := mask + 4;
    st := 3;
ELSIF st = 3 THEN
    st := 4;
ELSIF st = 4 AND snd2.bDone THEN
    IF snd2.nStatus = 1 AND snd2.nErrId = 0 THEN
        mask := mask + 8;
    END_IF;
    st := 5;
ELSIF st = 5 AND gotCs THEN
    mask := mask + 16;
    st := 6;
ELSIF st = 6 AND snd3.bDone THEN
    IF snd3.nStatus = 1 AND snd3.nErrId = 0 THEN
        mask := mask + 32;
    END_IF;
    st := 7;
ELSIF st = 7 AND gotSc THEN
    mask := mask + 64;
    st := 8;
ELSIF st = 8 AND connBad.bDone THEN
    IF connBad.nStatus = 2 THEN
        mask := mask + 128;
    END_IF;
    st := 9;
ELSIF st = 9 THEN
    // pure-helper checks (single scan)
    // CP56Time2a roundtrip: encode 2024-06-15 13:45:30.250.
    // FB VAR_OUTPUT stays on the instance; copy pTime -> o64.
    tenc(nYear := 2024, nMonth := 6, nDay := 15,
         nHour := 13, nMinute := 45, nSecond := 30, nMs := 250,
         bIV := FALSE, bSU := FALSE);
    i2 := 0;
    WHILE i2 < 7 DO
        o64[i2] := tenc.pTime[i2];
        i2 := i2 + 1;
    END_WHILE;
    tdec(pTime := o64);
    hY := tdec.nYear; hMo := tdec.nMonth; hD := tdec.nDay;
    hH := tdec.nHour; hMi := tdec.nMinute; hS := tdec.nSecond;
    hMs := tdec.nMs; hbIV := tdec.bIV; hbSU := tdec.bSU;
    IF hY = 2024 AND hMo = 6 AND hD = 15 AND hH = 13 AND
       hMi = 45 AND hS = 30 AND hMs = 250 AND NOT hbIV THEN
        mask := mask + 256;
    END_IF;
    hCot := ASDU101COT_TO_STRING(3);
    hTid := ASDU101TYPEID_TO_STRING(9);
    IF hCot = 'SPONTANEOUS' AND hTid = 'M_ME_NA_1' THEN
        mask := mask + 512;
    END_IF;
    hIoa := CP24IOA_TO_DWORD(0, 0, 5);
    hSpi := F_iecGetSPI(1);
    hDpi := F_iecGetDPI(2);
    hVti := F_iecIncVTI(32767);
    hVer := GetIec870_Version();
    IF hIoa = 5 AND hSpi AND hDpi = 2 AND hVti = 0 AND
       hVer = 16#0100 THEN
        mask := mask + 1024;
    END_IF;
    st := 10;
ELSIF st = 10 THEN
    // bClose := st = 10 gave one rising edge -> session closed
    IF conn.nConnHandle = 0 THEN
        mask := mask + 2048;
        st := 11;
    END_IF;
END_IF;

END_PROGRAM
