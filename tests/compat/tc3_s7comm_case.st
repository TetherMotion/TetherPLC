PROGRAM Tc3S7CommCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    conn      : FB_S7_Connect;
    connBad   : FB_S7_Connect;
    rd        : FB_S7_Read;
    rd2       : FB_S7_Read;
    rd3       : FB_S7_Read;
    wr        : FB_S7_Write;
    rdb       : FB_S7_ReadBit;
    wrb       : FB_S7_WriteBit;
    wdata     : ARRAY[0..239] OF BYTE;
END_VAR

// autonomous S7 sequence against tests/compat/s7_test_peer.py
// (TCP 1102). The peer serves DB1 (256B pattern (i*7+3)) and MK
// area 0 (128B pattern 0xA0+(i&15)) with spec bit semantics.
// Each state's bExecute is high only while st equals its index,
// giving every FB one rising edge; bDone advances the machine.
wdata[0] := 16#DE; wdata[1] := 16#AD;
wdata[2] := 16#BE; wdata[3] := 16#EF;

conn(sHost := '127.0.0.1', nPort := 1102, nRack := 0, nSlot := 2,
     bExecute := st = 0, bClose := st = 8, tTimeout := T#3S);
connBad(sHost := '127.0.0.1', nPort := 19999, nRack := 0, nSlot := 2,
        bExecute := st = 7, bClose := FALSE, tTimeout := T#3S);
rd(nConnHandle := conn.nConnHandle, nArea := 16#84, nDb := 1,
   nByteOff := 10, nLen := 16, bExecute := st = 1,
   tTimeout := T#2S);
wr(nConnHandle := conn.nConnHandle, nArea := 16#84, nDb := 1,
   nByteOff := 0, nLen := 4, pData := wdata, bExecute := st = 2,
   tTimeout := T#2S);
rd2(nConnHandle := conn.nConnHandle, nArea := 16#84, nDb := 1,
    nByteOff := 0, nLen := 4, bExecute := st = 3,
    tTimeout := T#2S);
rdb(nConnHandle := conn.nConnHandle, nArea := 16#83, nDb := 0,
    nByteOff := 5, nBitNo := 0, bExecute := st = 4,
    tTimeout := T#2S);
wrb(nConnHandle := conn.nConnHandle, nArea := 16#83, nDb := 0,
    nByteOff := 5, nBitNo := 1, nBitValue := 1, bExecute := st = 5,
    tTimeout := T#2S);
rd3(nConnHandle := conn.nConnHandle, nArea := 16#84, nDb := 99,
    nByteOff := 0, nLen := 4, bExecute := st = 6,
    tTimeout := T#2S);

IF st = 0 AND conn.bDone THEN
    // connect -> handshake completes, handle issued
    IF conn.nStatus = 1 AND conn.nConnHandle <> 0 AND
       conn.nErrId = 0 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND rd.bDone THEN
    // DB1[10..25] = pattern (i*7+3): first byte 0x49
    IF rd.nStatus = 1 AND rd.nDataLen = 16 AND rd.pData[0] = 16#49
       AND rd.nItemRc = 16#FF AND rd.nErrId = 0 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND wr.bDone THEN
    // write DE AD BE EF -> DB1[0..3]
    IF wr.nStatus = 1 AND wr.nItemRc = 16#FF AND wr.nErrId = 0 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND rd2.bDone THEN
    // readback -> DE AD BE EF
    IF rd2.nStatus = 1 AND rd2.nDataLen = 4 AND
       rd2.pData[0] = 16#DE AND rd2.pData[1] = 16#AD AND
       rd2.pData[2] = 16#BE AND rd2.pData[3] = 16#EF THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND rdb.bDone THEN
    // MK5 = 0xA5, bit0 = 1
    IF rdb.nStatus = 1 AND rdb.nBitValue = 1 AND rdb.nErrId = 0 THEN
        mask := mask + 16;
    END_IF;
    st := 5;
ELSIF st = 5 AND wrb.bDone THEN
    // set MK5 bit1 (0xA5 -> 0xA7)
    IF wrb.nStatus = 1 AND wrb.nItemRc = 16#FF AND wrb.nErrId = 0 THEN
        mask := mask + 32;
    END_IF;
    st := 6;
ELSIF st = 6 AND rd3.bDone THEN
    // DB99 unregistered -> item rc 0x0A -> status 5 (s7err)
    IF rd3.nStatus = 5 AND rd3.nItemRc = 16#0A THEN
        mask := mask + 64;
    END_IF;
    st := 7;
ELSIF st = 7 AND connBad.bDone THEN
    // connect to closed port -> conn status 2 (error)
    IF connBad.nStatus = 2 AND connBad.nConnHandle <> 0 THEN
        mask := mask + 128;
    END_IF;
    st := 8;
ELSIF st = 8 THEN
    // bClose := st = 8 gives one rising edge -> session closed
    IF conn.nConnHandle = 0 THEN
        mask := mask + 256;
        st := 9;
    END_IF;
END_IF;

END_PROGRAM
