// Tc3_sACN compatibility (TwinCAT 3 PLC Library Tc3_sACN, ANSI
// E1.31 streaming ACN). Beckhoff-shaped wrappers over the builtin
// FB_SacnSource / FB_SacnReceiver, which drive the host "tplc-sacn"
// service: nonblocking UDP multicast source and receiver on port
// 5568 (239.255.x.y per universe).
// Differences from TwinCAT, documented in docs/plc/compat/README.md:
//   - TwinCAT's I_sAcnCommunicator / I_sAcnSource / I_sAcnReceiver
//     interfaces, OnSACN*PacketReceived callbacks, T_HSOCKET
//     handles, and METHODs have no dialect equivalent; sources and
//     receivers are poll-driven FBs exposing counters and the
//     latest packet state. Poll nDataPackets (or bDataReady) for
//     new frames instead of the packet-received callback.
//   - FB_sACN_ReceiverAndMerger merge rules, FB_sACN_SourceExtDmx
//     external-array binding, and FB_sACN_UDP_Communicator shared
//     socket management are not provided.
//   - DMX payloads travel as bounded ARRAY[0..511] OF BYTE instead
//     of POINTER TO USINT.
// Sources: docs/plc/compat/twincat/66-tc3-sacn.md.

// ---- sources ----

FUNCTION_BLOCK FB_sACN_Source
VAR_INPUT
    bEnable : BOOL;
    nUniverse : UINT;
    nPriority : UINT := 100;
    nSyncUniverse : UINT := 0;
    sSourceName : STRING[63] := '';
    nSendIntervalMs : UDINT := 0;
    nTimeToLive : UDINT := 0;
    bOptionForceSync : BOOL := FALSE;
    bOptionStreamTerminated : BOOL := FALSE;
    bOptionPreviewData : BOOL := FALSE;
    nDmxStartCode : BYTE := 0;
    bSendSync : BOOL := FALSE;
    bSendDiscovery : BOOL := FALSE;
    cid : ARRAY[0..15] OF BYTE;
    DmxData : ARRAY[0..511] OF BYTE;
    nDmxLength : UDINT := 0;
END_VAR
VAR_OUTPUT
    bIsSending : BOOL;
    nSentPackets : ULINT;
    nAuxPackets : ULINT;
    nSequence : UDINT;
    nErrId : UDINT;
    nLastErr : DINT;
END_VAR
VAR
    src : FB_SacnSource;
    i : UDINT;
    opts : BYTE;
    flgs : BYTE;
END_VAR
    src.ENABLE := bEnable;
    src.UNIVERSE := nUniverse;
    src.PRIORITY := nPriority;
    src.SYNC_ADDR := nSyncUniverse;
    src.INTERVAL := nSendIntervalMs;
    src.TTL := nTimeToLive;
    opts := 0;
    IF bOptionForceSync THEN
        opts := opts + 16#20;
    END_IF;
    IF bOptionStreamTerminated THEN
        opts := opts + 16#40;
    END_IF;
    IF bOptionPreviewData THEN
        opts := opts + 16#80;
    END_IF;
    src.OPTIONS := opts;
    flgs := 0;
    IF bSendSync THEN
        flgs := flgs + 1;
    END_IF;
    IF bSendDiscovery THEN
        flgs := flgs + 2;
    END_IF;
    src.FLAGS := flgs;
    src.START_CODE := nDmxStartCode;
    src.NAME := sSourceName;
    i := 0;
    WHILE i < 16 DO
        src.CID[i] := cid[i];
        i := i + 1;
    END_WHILE;
    src.DLEN := nDmxLength;
    i := 0;
    WHILE i < nDmxLength AND i < 512 DO
        src.DMX[i] := DmxData[i];
        i := i + 1;
    END_WHILE;
    src();
    bIsSending := src.BUSY;
    nSentPackets := src.SENT;
    nAuxPackets := src.AUX;
    nSequence := src.SEQ;
    nErrId := src.ERRID;
    nLastErr := src.LAST_ERR;
END_FUNCTION_BLOCK

// ---- receivers ----

FUNCTION_BLOCK FB_sACN_Receiver
VAR_INPUT
    bEnable : BOOL;
    nUniverse : UINT;
    nSyncUniverse : UINT := 0;
    nPort : UINT := 5568;
END_VAR
VAR_OUTPUT
    bIsListening : BOOL;
    bDataReady : BOOL;
    bHaveData : BOOL;
    bPreviewData : BOOL;
    bStreamTerminated : BOOL;
    bForceSync : BOOL;
    bHaveSync : BOOL;
    bHaveDiscovery : BOOL;
    nDataPackets : ULINT;
    nDroppedPackets : ULINT;
    nSyncPackets : ULINT;
    nDiscoveryPackets : ULINT;
    nErrId : UDINT;
    nDmxLength : UDINT;
    nSequence : UDINT;
    nPriority : UDINT;
    nSyncAddress : UDINT;
    nSyncSequence : UDINT;
    nSyncAddressSeen : UDINT;
    nUniverseCount : UDINT;
    nUniversePage : UDINT;
    nUniverseLastPage : UDINT;
    nStartCode : BYTE;
    nRemoteIp1 : BYTE;
    nRemoteIp2 : BYTE;
    nRemoteIp3 : BYTE;
    nRemoteIp4 : BYTE;
    sSourceName : STRING[63];
    DmxData : ARRAY[0..511] OF BYTE;
    Universes : ARRAY[0..63] OF WORD;
END_VAR
VAR
    rcv : FB_SacnReceiver;
    i : UDINT;
    nLastDataRx : ULINT;
    rip : UDINT;
END_VAR
    rcv.ENABLE := bEnable;
    rcv.RCV_UNIVERSE := nUniverse;
    rcv.SYNC_UNIVERSE := nSyncUniverse;
    rcv.PORT := nPort;
    rcv();
    bIsListening := rcv.BUSY;
    bDataReady := rcv.DATA_RX <> nLastDataRx;
    nLastDataRx := rcv.DATA_RX;
    bHaveData := rcv.HAVE_DATA;
    bPreviewData := rcv.PREVIEW;
    bStreamTerminated := rcv.STREAM_TERM;
    bForceSync := rcv.FORCE_SYNC;
    bHaveSync := rcv.HAVE_SYNC;
    bHaveDiscovery := rcv.HAVE_DISC;
    nDataPackets := rcv.DATA_RX;
    nDroppedPackets := rcv.DATA_DROP;
    nSyncPackets := rcv.SYNC_RX;
    nDiscoveryPackets := rcv.DISC_RX;
    nErrId := rcv.ERRID;
    nDmxLength := rcv.DLEN;
    nSequence := BYTE_TO_UDINT(rcv.SEQ);
    nPriority := BYTE_TO_UDINT(rcv.PRIORITY);
    nSyncAddress := WORD_TO_UDINT(rcv.SYNC_ADDR);
    nSyncSequence := BYTE_TO_UDINT(rcv.SYNC_SEQ);
    nSyncAddressSeen := WORD_TO_UDINT(rcv.SYNC_ADDR_SEEN);
    nUniverseCount := WORD_TO_UDINT(rcv.UNI_COUNT);
    nUniversePage := BYTE_TO_UDINT(rcv.UNI_PAGE);
    nUniverseLastPage := BYTE_TO_UDINT(rcv.UNI_LAST);
    nStartCode := rcv.START_CODE;
    rip := rcv.REMOTE_IP;
    nRemoteIp1 := UDINT_TO_BYTE(rip / 16777216);
    nRemoteIp2 := UDINT_TO_BYTE(rip / 65536 MOD 256);
    nRemoteIp3 := UDINT_TO_BYTE(rip / 256 MOD 256);
    nRemoteIp4 := UDINT_TO_BYTE(rip MOD 256);
    sSourceName := rcv.SRC_NAME;
    i := 0;
    WHILE i < rcv.DLEN AND i < 512 DO
        DmxData[i] := rcv.DMX[i];
        i := i + 1;
    END_WHILE;
    i := 0;
    WHILE i < 64 DO
        Universes[i] := rcv.UNIS[i];
        i := i + 1;
    END_WHILE;
END_FUNCTION_BLOCK

// ---- helpers ----

// F_HighLowArr_TO_UINT — big-endian byte pair to UINT (network
// order decode for universe numbers and packet fields).
FUNCTION F_HighLowArr_TO_UINT : UINT
VAR_INPUT
    Value : ARRAY[1..2] OF BYTE;
END_VAR
    F_HighLowArr_TO_UINT := BYTE_TO_UINT(Value[1]) * 256
        + BYTE_TO_UINT(Value[2]);
END_FUNCTION

// F_UINT_TO_HighLowArr — UINT to big-endian byte pair. TwinCAT
// returns the array; this dialect has no array-returning FUNCTION,
// so the pair is exposed as outputs on an execute-gated FB.
FUNCTION_BLOCK FB_UINT_TO_HighLowArr
VAR_INPUT
    bExecute : BOOL;
    Value : UINT;
END_VAR
VAR_OUTPUT
    bDone : BOOL;
    HighByte : BYTE;
    LowByte : BYTE;
END_VAR
VAR
    bLast : BOOL;
END_VAR
    IF bExecute AND NOT bLast THEN
        HighByte := UINT_TO_BYTE(Value / 256);
        LowByte := UINT_TO_BYTE(Value MOD 256);
        bDone := TRUE;
    ELSE
        bDone := FALSE;
    END_IF;
    bLast := bExecute;
END_FUNCTION_BLOCK

// F_UNIVERSE_TO_MulticastAdr — ANSI E1.31 multicast group for a
// universe: 239.255.(universe>>8).(universe&0xFF). TwinCAT returns
// T_IPv4Addr (a packed 4-byte struct); this dialect has no int→string
// conversion, so the address is returned packed big-endian in a
// UDINT: octet A in bits 31..24 down to octet D in bits 7..0.
FUNCTION F_UNIVERSE_TO_MulticastAdr : UDINT
VAR_INPUT
    Universe : UINT;
END_VAR
    F_UNIVERSE_TO_MulticastAdr := 16#EFFF0000
        + WORD_TO_UDINT(Universe / 256) * 256
        + WORD_TO_UDINT(Universe MOD 256);
END_FUNCTION
