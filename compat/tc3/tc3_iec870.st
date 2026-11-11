// Tc2_IEC60870_5_10x -> IEC 60870 telecontrol wrappers (104 client
// plus pure ASDU helpers). TwinCAT's FB_IEC870_5_104Master/Slave
// and FB_IEC870_5_101/102/103TProtocol blocks are built on AMS-routed
// AO databases, PVOID buffers, POINTER TO ARRAY and generated
// server objects. TetherPLC adapts the supported surface to the
// poll-driven service model used by the other compat libraries:
// FB_Iec870Conn holds one IEC 60870-5-104 session (TCP 2404,
// STARTDT handshake), FB_Iec870Tx runs one ASDU send per EXEC edge,
// FB_Iec870Rx polls the received-ASDU queue every scan.
// Differences from the TwinCAT FBs, documented in
// docs/plc/compat/README.md:
//   - Master side only: 104 client. No 101/102/103 serial or HDLC
//     transports, no slave/server mode, no AO database, no AMS
//     NetID routing, no PVOID/HRESULT object model, no redundancy
//     groups, no file transfer, no TLS.
//   - ASDU payloads are raw info-object bytes; per-type object
//     codecs (measured-value scaling, CP24/CP56 timestamps,
//     sequence addressing expansion) stay in user code or the
//     pure helpers below.
//   - The pure helpers take flat scalar/array arguments instead
//     of TwinCAT STRUCT/STRING types: CP56Time2a is a 7-byte
//     ARRAY OF BYTE, SYSTEMTIME is six scalar outputs.
// Sources: docs/plc/plan/21-twincat-libraries.md
// (Tc2_IEC60870_5_10x), IEC 60870-5-104/-5-101, lib60870-CS104.

// Open/poll/close an IEC 60870-5-104 master connection —
// covers FB_IEC870_5_104Master and the generic
// FB_IEC870_5_10xTProtocol link layer for the TCP transport.
FUNCTION_BLOCK FB_IEC870_5_104Master
VAR_INPUT
    sHost : STRING[15];          // dotted-quad PLC IPv4
    nPort : UDINT := 0;          // 0 = 2404
    nCA_Size : UDINT := 2;       // common-address octets 1 or 2
    nCOT_Size : UDINT := 2;      // COT octets 1 or 2 (2nd = OA)
    bExecute : BOOL;             // rising edge = (re)connect
    bClose : BOOL;               // rising edge = disconnect
    tTimeout : TIME := T#5S;
    tKeepAlive : TIME := T#10S;  // TESTFR interval
END_VAR
VAR_OUTPUT
    nConnHandle : UDINT;         // 0 = no live session
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;              // 0=pending 1=ready 2=error
    nSubState : DINT;            // 0=none 1=tcp 2=startdt 3=active
    nErrId : UDINT;
    nRxDropped : UDINT;
END_VAR
VAR
    conn : FB_Iec870Conn;
END_VAR
    conn.HOST := sHost;
    conn.PORT := nPort;
    conn.CA_SIZE := nCA_Size;
    conn.COT_SIZE := nCOT_Size;
    conn.TIMEOUT := tTimeout;
    conn.CLOSE := bClose;
    conn.EXEC := bExecute;
    conn();
    nConnHandle := conn.CONN_HANDLE;
    bBusy := conn.BUSY;
    bDone := conn.DONE;
    nStatus := conn.STATUS;
    nSubState := conn.SUBSTATE;
    nErrId := conn.ERRID;
    nRxDropped := conn.RX_DROPPED;
    bError := (conn.ERRID <> 0) OR (conn.STATUS = 2);
END_FUNCTION_BLOCK

// Send one ASDU as an I-frame — covers the master-side transmit
// path of FB_IEC870_5_104Master (cyclic/spontaneous ASDUs).
// nTypeId/nVSQ/nCOT/nCA are the raw ASDU header fields; pPayload
// carries the info-object bytes. One outstanding send per
// connection; the k-window (12 unacked I-frames) applies.
FUNCTION_BLOCK FB_IEC870_SendAsdu
VAR_INPUT
    nConnHandle : UDINT;         // from FB_IEC870_5_104Master
    nTypeId : UDINT;             // ASDU type identifier
    nVSQ : UDINT;                // raw VSQ octet (bit7 seq flag)
    nCOT : UDINT;                // encoded COT (+OA bits 8..15)
    nCA : UDINT;                 // common address
    nLen : UDINT;                // payload bytes 0..240
    pPayload : ARRAY[0..239] OF BYTE;
    bExecute : BOOL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nStatus : DINT;              // 0=pending 1=ok 2=error
    nErrId : UDINT;
END_VAR
VAR
    tx : FB_Iec870Tx;
    i : INT;
END_VAR
    tx.CONN_HANDLE := nConnHandle;
    tx.TYPE_ID := nTypeId;
    tx.VSQ := nVSQ;
    tx.COT := nCOT;
    tx.CA := nCA;
    tx.LEN := nLen;
    i := 0;
    WHILE i < 240 DO
        tx.PAYLOAD[i] := pPayload[i];
        i := i + 1;
    END_WHILE;
    tx.EXEC := bExecute;
    tx();
    bBusy := tx.BUSY;
    bDone := tx.DONE;
    nStatus := tx.STATUS;
    nErrId := tx.ERRID;
    bError := (tx.ERRID <> 0) OR (tx.STATUS = 2);
END_FUNCTION_BLOCK

// Poll one received ASDU — call every scan with the connection
// handle; bDone pulses TRUE once per delivered ASDU
// (consume-on-read). nTypeId/nVSQ/nCOT/nCA describe the raw ASDU
// header parsed per the conn COT_SIZE/CA_SIZE; pPayload holds
// the info-object bytes (nLen valid).
FUNCTION_BLOCK FB_IEC870_RecvAsdu
VAR_INPUT
    nConnHandle : UDINT;
END_VAR
VAR_OUTPUT
    bDone : BOOL;
    nTypeId : UDINT;
    nVSQ : UDINT;
    nCOT : UDINT;
    nCA : UDINT;
    nLen : UDINT;
    nIFrameSeq : UDINT;
    nErrId : UDINT;
    pPayload : ARRAY[0..239] OF BYTE;
END_VAR
VAR
    rx : FB_Iec870Rx;
    i : INT;
END_VAR
    rx.CONN_HANDLE := nConnHandle;
    rx();
    bDone := rx.DONE;
    nTypeId := rx.TYPE_ID;
    nVSQ := rx.VSQ;
    nCOT := rx.COT;
    nCA := rx.CA;
    nLen := rx.LEN;
    nIFrameSeq := rx.I_FRAME_SEQ;
    nErrId := rx.ERRID;
    IF rx.DONE THEN
        i := 0;
        WHILE i < 240 DO
            pPayload[i] := rx.PAYLOAD[i];
            i := i + 1;
        END_WHILE;
    END_IF
END_FUNCTION_BLOCK

// ------------------------------------------------------------------
// Pure helpers (TwinCAT helper FBs/functions, flat signatures)
// ------------------------------------------------------------------

// ASDU101COT_TO_STRING — cause-of-transmission type to text.
// Covers the 101/104 COT table.
FUNCTION ASDU101COT_TO_STRING : STRING[31]
VAR_INPUT
    nCOT : UDINT;                // COT type bits 0..5
END_VAR
VAR
    t : UDINT;
END_VAR
    t := nCOT AND 16#3F;
    CASE t OF
        1:  ASDU101COT_TO_STRING := 'PERIODIC';
        2:  ASDU101COT_TO_STRING := 'BACKGROUND';
        3:  ASDU101COT_TO_STRING := 'SPONTANEOUS';
        4:  ASDU101COT_TO_STRING := 'INITIALIZED';
        5:  ASDU101COT_TO_STRING := 'REQUEST';
        6:  ASDU101COT_TO_STRING := 'ACTIVATION';
        7:  ASDU101COT_TO_STRING := 'ACT_CON';
        8:  ASDU101COT_TO_STRING := 'DEACTIVATION';
        9:  ASDU101COT_TO_STRING := 'DEACT_CON';
        10: ASDU101COT_TO_STRING := 'ACT_TERMINATION';
        11: ASDU101COT_TO_STRING := 'REMOTE_TAG';
        12: ASDU101COT_TO_STRING := 'LOCAL_TAG';
        20: ASDU101COT_TO_STRING := 'UNKNOWN_TYPE';
    ELSE
        ASDU101COT_TO_STRING := 'RESERVED';
    END_CASE
END_FUNCTION

// ASDU102COT_TO_STRING — 102 (dial-up) COT table adds file-ready.
FUNCTION ASDU102COT_TO_STRING : STRING[31]
VAR_INPUT
    nCOT : UDINT;
END_VAR
VAR
    t : UDINT;
END_VAR
    t := nCOT AND 16#3F;
    CASE t OF
        1:  ASDU102COT_TO_STRING := 'PERIODIC';
        2:  ASDU102COT_TO_STRING := 'BACKGROUND';
        3:  ASDU102COT_TO_STRING := 'SPONTANEOUS';
        4:  ASDU102COT_TO_STRING := 'INITIALIZED';
        5:  ASDU102COT_TO_STRING := 'REQUEST';
        6:  ASDU102COT_TO_STRING := 'ACTIVATION';
        7:  ASDU102COT_TO_STRING := 'ACT_CON';
        8:  ASDU102COT_TO_STRING := 'DEACTIVATION';
        9:  ASDU102COT_TO_STRING := 'DEACT_CON';
        10: ASDU102COT_TO_STRING := 'ACT_TERMINATION';
        11: ASDU102COT_TO_STRING := 'REMOTE_TAG';
        12: ASDU102COT_TO_STRING := 'LOCAL_TAG';
        13: ASDU102COT_TO_STRING := 'FILE_READY';
        20: ASDU102COT_TO_STRING := 'UNKNOWN_TYPE';
    ELSE
        ASDU102COT_TO_STRING := 'RESERVED';
    END_CASE
END_FUNCTION

// ASDU103COT_TO_STRING — 103 (relay protection) COT table.
FUNCTION ASDU103COT_TO_STRING : STRING[31]
VAR_INPUT
    nCOT : UDINT;
END_VAR
VAR
    t : UDINT;
END_VAR
    t := nCOT AND 16#3F;
    CASE t OF
        1:  ASDU103COT_TO_STRING := 'PERIODIC';
        2:  ASDU103COT_TO_STRING := 'BACKGROUND';
        3:  ASDU103COT_TO_STRING := 'SPONTANEOUS';
        4:  ASDU103COT_TO_STRING := 'INITIALIZED';
        5:  ASDU103COT_TO_STRING := 'REQUEST';
        6:  ASDU103COT_TO_STRING := 'ACTIVATION';
        7:  ASDU103COT_TO_STRING := 'ACT_CON';
        8:  ASDU103COT_TO_STRING := 'DEACTIVATION';
        9:  ASDU103COT_TO_STRING := 'DEACT_CON';
        10: ASDU103COT_TO_STRING := 'ACT_TERMINATION';
        11: ASDU103COT_TO_STRING := 'REMOTE_TAG';
        12: ASDU103COT_TO_STRING := 'LOCAL_TAG';
        13: ASDU103COT_TO_STRING := 'FILE_READY';
        20: ASDU103COT_TO_STRING := 'UNKNOWN_TYPE';
    ELSE
        ASDU103COT_TO_STRING := 'RESERVED';
    END_CASE
END_FUNCTION

// ASDU101TYPEID_TO_STRING — common 101/104 ASDU type names.
FUNCTION ASDU101TYPEID_TO_STRING : STRING[31]
VAR_INPUT
    nTypeId : UDINT;
END_VAR
VAR
    t : UDINT;
END_VAR
    t := nTypeId;
    CASE t OF
        1:   ASDU101TYPEID_TO_STRING := 'M_SP_NA_1';
        2:   ASDU101TYPEID_TO_STRING := 'M_SP_TA_1';
        3:   ASDU101TYPEID_TO_STRING := 'M_DP_NA_1';
        4:   ASDU101TYPEID_TO_STRING := 'M_DP_TA_1';
        5:   ASDU101TYPEID_TO_STRING := 'M_ST_NA_1';
        6:   ASDU101TYPEID_TO_STRING := 'M_ST_TA_1';
        7:   ASDU101TYPEID_TO_STRING := 'M_BO_NA_1';
        8:   ASDU101TYPEID_TO_STRING := 'M_BO_TA_1';
        9:   ASDU101TYPEID_TO_STRING := 'M_ME_NA_1';
        10:  ASDU101TYPEID_TO_STRING := 'M_ME_TA_1';
        11:  ASDU101TYPEID_TO_STRING := 'M_ME_NB_1';
        12:  ASDU101TYPEID_TO_STRING := 'M_ME_TB_1';
        13:  ASDU101TYPEID_TO_STRING := 'M_ME_NC_1';
        14:  ASDU101TYPEID_TO_STRING := 'M_ME_TC_1';
        15:  ASDU101TYPEID_TO_STRING := 'M_IT_NA_1';
        16:  ASDU101TYPEID_TO_STRING := 'M_IT_TA_1';
        17:  ASDU101TYPEID_TO_STRING := 'M_EP_TA_1';
        18:  ASDU101TYPEID_TO_STRING := 'M_EP_TB_1';
        19:  ASDU101TYPEID_TO_STRING := 'M_EP_TC_1';
        20:  ASDU101TYPEID_TO_STRING := 'M_EP_TD_1';
        21:  ASDU101TYPEID_TO_STRING := 'M_EP_TE_1';
        30:  ASDU101TYPEID_TO_STRING := 'M_SP_TB_1';
        31:  ASDU101TYPEID_TO_STRING := 'M_DP_TB_1';
        32:  ASDU101TYPEID_TO_STRING := 'M_ST_TB_1';
        33:  ASDU101TYPEID_TO_STRING := 'M_BO_TB_1';
        34:  ASDU101TYPEID_TO_STRING := 'M_ME_TD_1';
        35:  ASDU101TYPEID_TO_STRING := 'M_ME_TE_1';
        36:  ASDU101TYPEID_TO_STRING := 'M_ME_TF_1';
        37:  ASDU101TYPEID_TO_STRING := 'M_IT_TB_1';
        38:  ASDU101TYPEID_TO_STRING := 'M_EP_TF_1';
        39:  ASDU101TYPEID_TO_STRING := 'M_EP_TG_1';
        40:  ASDU101TYPEID_TO_STRING := 'M_EP_TH_1';
        41:  ASDU101TYPEID_TO_STRING := 'M_EP_TI_1';
        42:  ASDU101TYPEID_TO_STRING := 'M_EP_TJ_1';
        43:  ASDU101TYPEID_TO_STRING := 'M_EP_TK_1';
        44:  ASDU101TYPEID_TO_STRING := 'M_EP_TL_1';
        45:  ASDU101TYPEID_TO_STRING := 'C_SC_NA_1';
        46:  ASDU101TYPEID_TO_STRING := 'C_DC_NA_1';
        47:  ASDU101TYPEID_TO_STRING := 'C_RC_NA_1';
        48:  ASDU101TYPEID_TO_STRING := 'C_SE_NA_1';
        49:  ASDU101TYPEID_TO_STRING := 'C_SE_NB_1';
        50:  ASDU101TYPEID_TO_STRING := 'C_SE_NC_1';
        51:  ASDU101TYPEID_TO_STRING := 'C_BO_NA_1';
        58:  ASDU101TYPEID_TO_STRING := 'C_SC_TA_1';
        59:  ASDU101TYPEID_TO_STRING := 'C_DC_TA_1';
        60:  ASDU101TYPEID_TO_STRING := 'C_RC_TA_1';
        61:  ASDU101TYPEID_TO_STRING := 'C_SE_TA_1';
        62:  ASDU101TYPEID_TO_STRING := 'C_SE_TB_1';
        63:  ASDU101TYPEID_TO_STRING := 'C_SE_TC_1';
        64:  ASDU101TYPEID_TO_STRING := 'C_BO_TA_1';
        70:  ASDU101TYPEID_TO_STRING := 'M_EI_NA_1';
        100: ASDU101TYPEID_TO_STRING := 'C_IC_NA_1';
        101: ASDU101TYPEID_TO_STRING := 'C_CI_NA_1';
        102: ASDU101TYPEID_TO_STRING := 'C_RD_NA_1';
        103: ASDU101TYPEID_TO_STRING := 'C_CS_NA_1';
        104: ASDU101TYPEID_TO_STRING := 'C_TS_NA_1';
        105: ASDU101TYPEID_TO_STRING := 'C_RP_NA_1';
        106: ASDU101TYPEID_TO_STRING := 'C_CD_NA_1';
        107: ASDU101TYPEID_TO_STRING := 'C_TS_TA_1';
    ELSE
        ASDU101TYPEID_TO_STRING := 'UNKNOWN';
    END_CASE
END_FUNCTION

// ASDU102TYPEID_TO_STRING / ASDU103TYPEID_TO_STRING — the 102/103
// tables share the common monitoring/command IDs with 101 plus
// file-transfer and protection-event types; the common subset
// delegates to ASDU101TYPEID_TO_STRING.
FUNCTION ASDU102TYPEID_TO_STRING : STRING[31]
VAR_INPUT
    nTypeId : UDINT;
END_VAR
VAR
    t : UDINT;
END_VAR
    t := nTypeId;
    IF (t >= 1) AND (t <= 71) THEN
        ASDU102TYPEID_TO_STRING := ASDU101TYPEID_TO_STRING(t);
    ELSIF (t >= 100) AND (t <= 107) THEN
        ASDU102TYPEID_TO_STRING := ASDU101TYPEID_TO_STRING(t);
    ELSIF (t >= 110) AND (t <= 115) THEN
        ASDU102TYPEID_TO_STRING := 'FILE_SEGMENT';
    ELSE
        ASDU102TYPEID_TO_STRING := 'UNKNOWN';
    END_IF
END_FUNCTION

FUNCTION ASDU103TYPEID_TO_STRING : STRING[31]
VAR_INPUT
    nTypeId : UDINT;
END_VAR
VAR
    t : UDINT;
END_VAR
    t := nTypeId;
    IF (t >= 1) AND (t <= 44) THEN
        ASDU103TYPEID_TO_STRING := ASDU101TYPEID_TO_STRING(t);
    ELSIF (t >= 45) AND (t <= 64) THEN
        ASDU103TYPEID_TO_STRING := ASDU101TYPEID_TO_STRING(t);
    ELSIF t = 70 THEN
        ASDU103TYPEID_TO_STRING := 'M_EI_NA_1';
    ELSIF t = 71 THEN
        ASDU103TYPEID_TO_STRING := 'M_EI_TA_1';
    ELSIF (t >= 90) AND (t <= 99) THEN
        ASDU103TYPEID_TO_STRING := 'PROTECTION_EVENT';
    ELSE
        ASDU103TYPEID_TO_STRING := 'UNKNOWN';
    END_IF
END_FUNCTION

// QOI_TO_COT — quality-of-information flags to the QDS/COT-style
// flag bits (OV=0x01, BL=0x10, SUC=0x20, IV=0x80).
FUNCTION QOI_TO_COT : UDINT
VAR_INPUT
    bIV : BOOL;                  // invalid
    bSU : BOOL;                  // substituted
    bBL : BOOL;                  // blocked
    bOV : BOOL;                  // overflow
END_VAR
VAR
    r : UDINT;
END_VAR
    r := 0;
    IF bOV THEN r := r OR 16#01; END_IF;
    IF bBL THEN r := r OR 16#10; END_IF;
    IF bSU THEN r := r OR 16#20; END_IF;
    IF bIV THEN r := r OR 16#80; END_IF;
    QOI_TO_COT := r;
END_FUNCTION

// RQT_TO_COT — request-type to COT cause for command ASDUs:
// select=activation(6), execute=activation(6),
// abort/deactivate=deactivation(8).
FUNCTION RQT_TO_COT : UDINT
VAR_INPUT
    nRqt : UDINT;
END_VAR
    IF nRqt >= 4 THEN
        RQT_TO_COT := 8;          // abort family -> deactivation
    ELSE
        RQT_TO_COT := 6;          // select/execute -> activation
    END_IF
END_FUNCTION

// CP24IOA_TO_DWORD — pack a 24-bit information object address
// (3 big-endian octets) into a DWORD for table lookups.
FUNCTION CP24IOA_TO_DWORD : UDINT
VAR_INPUT
    b0 : UDINT;                  // IOA octet 0 (MSB)
    b1 : UDINT;                  // IOA octet 1
    b2 : UDINT;                  // IOA octet 2 (LSB)
END_VAR
    CP24IOA_TO_DWORD := (b0 AND 16#FF) * 65536 +
                        (b1 AND 16#FF) * 256 +
                        (b2 AND 16#FF);
END_FUNCTION

FUNCTION DWORD_TO_CP24IOA : UDINT
VAR_INPUT
    nIOA : UDINT;                // packed 24-bit value
END_VAR
    DWORD_TO_CP24IOA := nIOA AND 16#FFFFFF;
END_FUNCTION

// CP56Time2a <-> flat system-time scalars. The 7-octet CP56Time2a
// layout (IEC 60870-5-4): octets 0-1 = millisecond-of-day LE16,
// octet 2 = minute (bits 0..5) + SUC (bit6) + IV (bit7),
// octet 3 = hour (bits 0..4) + SU (bit7),
// octet 4 = day-of-week (bits 5..7) + day-of-month (bits 0..4),
// octet 5 = month (bits 0..3), octet 6 = year-1900 (bits 0..6) +
// IV2 (bit7). TwinCAT returns/accepts a TIMESTRUCT; here the
// fields are scalar outputs / inputs.
FUNCTION_BLOCK CP56Time2a_TO_SYSTEMTIME
VAR_INPUT
    pTime : ARRAY[0..6] OF BYTE;
END_VAR
VAR_OUTPUT
    nYear : UDINT;               // full year (1900 + field)
    nMonth : UDINT;              // 1..12
    nDay : UDINT;                // 1..31
    nHour : UDINT;               // 0..23
    nMinute : UDINT;             // 0..59
    nSecond : UDINT;             // 0..59
    nMs : UDINT;                 // 0..999
    bIV : BOOL;                  // invalid flag
    bSU : BOOL;                  // substituted / summer-time
END_VAR
VAR
    msod : UDINT;
END_VAR
    /* octets 0-1 = millisecond-of-minute LE16 (sec*1000+ms,
       max 59999); octet 2 = minute (bits 0..5) + SUC + IV */
    msod := (BYTE_TO_UDINT(pTime[0]) +
             BYTE_TO_UDINT(pTime[1]) * 256);
    nMs := msod MOD 1000;
    msod := msod / 1000;
    nSecond := msod MOD 60;
    nMinute := BYTE_TO_UDINT(pTime[2]) AND 16#3F;
    nHour := BYTE_TO_UDINT(pTime[3]) AND 16#1F;
    nDay := BYTE_TO_UDINT(pTime[4]) AND 16#1F;
    nMonth := BYTE_TO_UDINT(pTime[5]) AND 16#0F;
    nYear := 1900 + (BYTE_TO_UDINT(pTime[6]) AND 16#7F);
    bIV := (BYTE_TO_UDINT(pTime[2]) AND 16#80) <> 0;
    bSU := ((BYTE_TO_UDINT(pTime[2]) AND 16#40) <> 0) OR
           ((BYTE_TO_UDINT(pTime[3]) AND 16#80) <> 0);
END_FUNCTION_BLOCK

FUNCTION_BLOCK SYSTEMTIME_TO_CP56Time2a
VAR_INPUT
    nYear : UDINT;
    nMonth : UDINT;
    nDay : UDINT;
    nHour : UDINT;
    nMinute : UDINT;
    nSecond : UDINT;
    nMs : UDINT;
    bIV : BOOL;
    bSU : BOOL;
END_VAR
VAR_OUTPUT
    pTime : ARRAY[0..6] OF BYTE;
END_VAR
VAR
    msod : UDINT;
    y : UDINT;
END_VAR
    /* octets 0-1 = (sec*1000+ms) LE16, the ms-within-minute
       field (max 59999) */
    msod := nSecond * 1000 + nMs;
    pTime[0] := UDINT_TO_BYTE(msod MOD 256);
    pTime[1] := UDINT_TO_BYTE((msod / 256) MOD 256);
    pTime[2] := UDINT_TO_BYTE(nMinute AND 16#3F);
    IF bSU THEN pTime[2] := UDINT_TO_BYTE(BYTE_TO_UDINT(pTime[2]) OR 16#40); END_IF;
    IF bIV THEN pTime[2] := UDINT_TO_BYTE(BYTE_TO_UDINT(pTime[2]) OR 16#80); END_IF;
    pTime[3] := UDINT_TO_BYTE(nHour AND 16#1F);
    IF bSU THEN pTime[3] := UDINT_TO_BYTE(BYTE_TO_UDINT(pTime[3]) OR 16#80); END_IF;
    pTime[4] := UDINT_TO_BYTE(nDay AND 16#1F);
    pTime[5] := UDINT_TO_BYTE(nMonth AND 16#0F);
    y := nYear;
    IF y >= 1900 THEN y := y - 1900; END_IF;
    pTime[6] := UDINT_TO_BYTE(y AND 16#7F);
    IF bIV THEN pTime[6] := UDINT_TO_BYTE(BYTE_TO_UDINT(pTime[6]) OR 16#80); END_IF;
END_FUNCTION_BLOCK

// CP56Time2a_TO_CP24Time2a / CP56Time2a_TO_CP32Time2a — truncate
// the 7-octet CP56Time2a to the shorter CP24 (3 octets) /
// CP32 (4 octets) time tags.
FUNCTION_BLOCK CP56Time2a_TO_CP24Time2a
VAR_INPUT
    pIn : ARRAY[0..6] OF BYTE;
END_VAR
VAR_OUTPUT
    pOut : ARRAY[0..2] OF BYTE;
END_VAR
    pOut[0] := pIn[0];
    pOut[1] := pIn[1];
    pOut[2] := pIn[2];
END_FUNCTION_BLOCK

FUNCTION_BLOCK CP56Time2a_TO_CP32Time2a
VAR_INPUT
    pIn : ARRAY[0..6] OF BYTE;
END_VAR
VAR_OUTPUT
    pOut : ARRAY[0..3] OF BYTE;
END_VAR
    pOut[0] := pIn[0];
    pOut[1] := pIn[1];
    pOut[2] := pIn[2];
    pOut[3] := pIn[3];
END_FUNCTION_BLOCK

// F_iecGetSPI / F_iecSetSPI — single-point information bit
// (bit 0 of the 1-octet M_SP info element).
FUNCTION F_iecGetSPI : BOOL
VAR_INPUT
    nValue : UDINT;               // raw info element octet
END_VAR
    F_iecGetSPI := (nValue AND 16#01) <> 0;
END_FUNCTION

FUNCTION F_iecSetSPI : UDINT
VAR_INPUT
    nValue : UDINT;
    bSPI : BOOL;
END_VAR
    IF bSPI THEN
        F_iecSetSPI := (nValue AND 16#FE) OR 16#01;
    ELSE
        F_iecSetSPI := nValue AND 16#FE;
    END_IF
END_FUNCTION

// F_iecGetDPI / F_iecSetDPI — double-point information (bits 0..1:
// 0=off 1=on 2=indeterminate 3=off+on invalid).
FUNCTION F_iecGetDPI : UDINT
VAR_INPUT
    nValue : UDINT;
END_VAR
    F_iecGetDPI := nValue AND 16#03;
END_FUNCTION

FUNCTION F_iecSetDPI : UDINT
VAR_INPUT
    nValue : UDINT;
    nDPI : UDINT;
END_VAR
    F_iecSetDPI := (nValue AND 16#FC) OR (nDPI AND 16#03);
END_FUNCTION

// F_iecGetSCS / F_iecSetSCS — single-command state (C_SC select/
// execute state bits in the command response info element).
FUNCTION F_iecGetSCS : UDINT
VAR_INPUT
    nValue : UDINT;
END_VAR
    F_iecGetSCS := (nValue / 16) AND 16#03;
END_FUNCTION

FUNCTION F_iecSetSCS : UDINT
VAR_INPUT
    nValue : UDINT;
    nSCS : UDINT;
END_VAR
    F_iecSetSCS := (nValue AND 16#CF) OR ((nSCS AND 16#03) * 16);
END_FUNCTION

// F_iecGetDCS / F_iecSetDCS — double-command state (C_DC).
FUNCTION F_iecGetDCS : UDINT
VAR_INPUT
    nValue : UDINT;
END_VAR
    F_iecGetDCS := (nValue / 16) AND 16#03;
END_FUNCTION

FUNCTION F_iecSetDCS : UDINT
VAR_INPUT
    nValue : UDINT;
    nDCS : UDINT;
END_VAR
    F_iecSetDCS := (nValue AND 16#CF) OR ((nDCS AND 16#03) * 16);
END_FUNCTION

// F_iecGetQDS — quality-descriptor octet mask (bit7 IV, bit6 NT,
// bit5 SUC, bit4 BL, bit0 OV).
FUNCTION F_iecGetQDS : UDINT
VAR_INPUT
    nValue : UDINT;
END_VAR
    F_iecGetQDS := nValue AND 16#FF;
END_FUNCTION

FUNCTION F_iecSetQDS : UDINT
VAR_INPUT
    nValue : UDINT;
    nQDS : UDINT;
END_VAR
    F_iecSetQDS := (nValue AND 16#FF00) OR (nQDS AND 16#FF);
END_FUNCTION

// F_iecIncVTI / F_iecDecVTI — wrap-around increment/decrement of
// a 15-bit IEC sequence counter (sequence numbers mod 32768).
FUNCTION F_iecIncVTI : UDINT
VAR_INPUT
    nVTI : UDINT;
END_VAR
VAR
    v : UDINT;
END_VAR
    v := nVTI AND 16#7FFF;
    IF v >= 32767 THEN
        F_iecIncVTI := 0;
    ELSE
        F_iecIncVTI := v + 1;
    END_IF
END_FUNCTION

FUNCTION F_iecDecVTI : UDINT
VAR_INPUT
    nVTI : UDINT;
END_VAR
VAR
    v : UDINT;
END_VAR
    v := nVTI AND 16#7FFF;
    IF v = 0 THEN
        F_iecDecVTI := 32767;
    ELSE
        F_iecDecVTI := v - 1;
    END_IF
END_FUNCTION

// F_iecGetAOQuality / F_iecSetAOQuality / F_iecGetAOTimeTag /
// F_iecInitAOEntry — AO (application object) entry helpers on a
// flat UDINT slot: bits 8..15 quality, bits 0..15 time tag.
FUNCTION F_iecGetAOQuality : UDINT
VAR_INPUT
    nAO : UDINT;
END_VAR
    F_iecGetAOQuality := (nAO / 256) AND 16#FF;
END_FUNCTION

FUNCTION F_iecSetAOQuality : UDINT
VAR_INPUT
    nAO : UDINT;
    nQuality : UDINT;
END_VAR
    F_iecSetAOQuality := (nAO AND 16#FFFF00FF) OR
                         ((nQuality AND 16#FF) * 256);
END_FUNCTION

FUNCTION F_iecGetAOTimeTag : UDINT
VAR_INPUT
    nAO : UDINT;
END_VAR
    F_iecGetAOTimeTag := nAO AND 16#FFFF;
END_FUNCTION

FUNCTION F_iecInitAOEntry : UDINT
VAR_INPUT
    nAO : UDINT;
END_VAR
    F_iecInitAOEntry := nAO AND 16#FFFF0000;
END_FUNCTION

// Version function covering FB_GetIec870_Version-style queries.
FUNCTION GetIec870_Version : UDINT
    GetIec870_Version := 16#0100;   // v1.00
END_FUNCTION
