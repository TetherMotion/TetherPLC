PROGRAM Tc3SacnCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    inited    : BOOL;
    src       : FB_sACN_Source;
    rcv       : FB_sACN_Receiver;
    hil       : FB_UINT_TO_HighLowArr;
    pair      : ARRAY[1..2] OF BYTE;
    cid       : ARRAY[0..15] OF BYTE;
    dmx       : ARRAY[0..511] OF BYTE;
    dropped0  : ULINT;
END_VAR

IF NOT inited THEN
    inited := TRUE;
    dmx[0] := 16#AA;
    dmx[1] := 16#55;
    dmx[2] := 16#01;
    dmx[3] := 16#02;
END_IF;

// source: universe 1, priority 100, 50 ms interval, auto CID
src(bEnable := TRUE, nUniverse := 1, nPriority := 100,
    nSyncUniverse := 0, sSourceName := 'SRC1',
    nSendIntervalMs := 50, nTimeToLive := 0,
    bOptionForceSync := FALSE, bOptionStreamTerminated := FALSE,
    bOptionPreviewData := FALSE, nDmxStartCode := 0,
    bSendSync := FALSE, bSendDiscovery := FALSE,
    cid := cid, DmxData := dmx, nDmxLength := 4);

// receiver: universe 1 on the default sACN port
rcv(bEnable := TRUE, nUniverse := 1, nSyncUniverse := 0,
    nPort := 5568);

// phase A: only our source is on the wire -> payload/name/prio
IF st = 0 AND rcv.nDataPackets >= 3 AND rcv.bHaveData THEN
    IF rcv.nDmxLength = 4 AND rcv.DmxData[0] = 16#AA AND
       rcv.DmxData[1] = 16#55 AND rcv.DmxData[2] = 16#01 AND
       rcv.DmxData[3] = 16#02 AND rcv.sSourceName = 'SRC1' AND
       rcv.nPriority = 100 AND rcv.nErrId = 0 THEN
        mask := mask + 1;
    END_IF;
    dropped0 := rcv.nDroppedPackets;
    st := 1;
// phase B: peer's priority-10 HIPRI packet preempts the source
ELSIF st = 1 AND rcv.sSourceName = 'HIPRI' THEN
    IF rcv.nPriority = 10 AND rcv.nDmxLength = 3 AND
       rcv.DmxData[0] = 16#33 AND rcv.DmxData[1] = 16#44 AND
       rcv.DmxData[2] = 16#55 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
// phase C: peer's priority-200 LOWPR and our priority-100 source
// are both dropped once HIPRI holds the channel
ELSIF st = 2 AND rcv.sSourceName = 'HIPRI' AND
          rcv.nDroppedPackets > dropped0 + 1 THEN
    mask := mask + 4;
    st := 3;
// phase D: pure-IEC helpers
ELSIF st = 3 THEN
    pair[1] := 0;
    pair[2] := 1;
    hil(bExecute := TRUE, Value := 16#1234);
    IF F_HighLowArr_TO_UINT(pair) = 1 AND
       F_UNIVERSE_TO_MulticastAdr(1) = 16#EFFF0001 AND
       F_UNIVERSE_TO_MulticastAdr(63999) = 16#EFFFF9FF AND
       hil.bDone AND hil.HighByte = 16#12 AND hil.LowByte = 16#34 THEN
        mask := mask + 8;
        st := 4;
    END_IF;
END_IF;

END_PROGRAM
