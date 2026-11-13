PROGRAM Tc2S5S7ComCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    lnk       : FB_NLinkOpen;
    lnkBad    : FB_NLinkOpen;
    fch       : FB_NLinkFetch;
    fch2      : FB_NLinkFetch;
    fchBad    : FB_NLinkFetch;
    wr        : FB_NLinkWrite;
    wdata     : ARRAY[0..509] OF BYTE;
END_VAR

// autonomous S5 sequence against tests/compat/s5_test_peer.py
// (TCP 1103, RK512/3964). The peer serves DB1 (256B pattern
// (i*7+3)) and DB2 (256B pattern 0x40+(i&0x3F)). Each state's
// bExecute is high only while st equals its index, giving every
// FB one rising edge; bDone advances the machine.
wdata[0] := 16#DE; wdata[1] := 16#AD;
wdata[2] := 16#BE; wdata[3] := 16#EF;

lnk(sHost := '127.0.0.1', nPort := 1103, bExecute := st = 0,
    bClose := st = 9, tTimeout := T#3S);
lnkBad(sHost := '127.0.0.1', nPort := 19999, bExecute := st = 8,
       bClose := FALSE, tTimeout := T#3S);
fch(hLink := lnk.hLink, nArea := 1, nDB := 1, nOffset := 10,
    nLen := 16, bExecute := st = 1, tTimeout := T#2S);
wr(hLink := lnk.hLink, nArea := 1, nDB := 1, nOffset := 0,
   nLen := 4, pData := wdata, bExecute := st = 2,
   tTimeout := T#2S);
fch2(hLink := lnk.hLink, nArea := 1, nDB := 1, nOffset := 0,
     nLen := 4, bExecute := st = 3, tTimeout := T#2S);
fchBad(hLink := lnk.hLink, nArea := 1, nDB := 77, nOffset := 0,
       nLen := 4, bExecute := st = 4, tTimeout := T#2S);

IF st = 0 AND lnk.bDone THEN
    // connect -> READY, handle issued
    IF lnk.nStatus = 1 AND lnk.hLink <> 0 AND lnk.nErrID = 0 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND fch.bDone THEN
    // DB1[10..25] = pattern (i*7+3): first byte 0x49, len 16
    IF fch.nStatus = 1 AND fch.cbRead = 16 AND fch.pBuffer[0] = 16#49
       AND fch.nErrID = 0 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND wr.bDone THEN
    // SEND accepted
    IF wr.nStatus = 1 AND wr.cbWrite = 4 AND wr.nErrID = 0 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND fch2.bDone THEN
    // read-back of the written bytes: wdata[0..3] = DE AD BE EF
    IF fch2.nStatus = 1 AND fch2.cbRead = 4
       AND fch2.pBuffer[0] = 16#DE AND fch2.pBuffer[1] = 16#AD
       AND fch2.pBuffer[2] = 16#BE AND fch2.pBuffer[3] = 16#EF
       AND fch2.nErrID = 0 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND fchBad.bDone THEN
    // unknown DB -> RK512 status byte nonzero -> s5err
    IF fchBad.nStatus = 5 AND fchBad.nErrID <> 0 THEN
        mask := mask + 16;
    END_IF;
    st := 8;
ELSIF st = 8 AND lnkBad.bDone THEN
    // connect to a dead port -> conn error
    IF lnkBad.nStatus = 2 THEN
        mask := mask + 32;
    END_IF;
    st := 9;
ELSIF st = 9 AND lnk.bDone THEN
    st := 10;
END_IF;
END_PROGRAM
