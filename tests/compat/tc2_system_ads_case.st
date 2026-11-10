PROGRAM Tc2SystemAdsCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    scan_n    : UDINT;
    hConn     : UDINT;
    gotData   : STRING[256];
    gotLen    : UDINT;
    wrData    : STRING[256];
    adsState  : DINT;
    devState  : DINT;
    conn1     : FB_AdsConnect;
    connBad   : FB_AdsConnect;
    read1     : FB_AdsRead;
    write1    : FB_AdsWrite;
    rstate1   : FB_AdsReadState;
    wctrl1    : FB_AdsWriteControl;
    bread1    : FB_AdsRead;
    close1    : FB_AdsClose;
END_VAR

// autonomous ADS exchange against the python test server:
// connect -> read -> write -> read-state -> write-control ->
// close, then a refused-port connect+read for the error leg.
wrData := 'WXYZ1234';

conn1(bExecute := st = 0, tTimeout := T#5s,
      sHost := '127.0.0.1', nPort := 18765);
connBad(bExecute := st = 5, tTimeout := T#5s,
        sHost := '127.0.0.1', nPort := 1);
read1(bExecute := st = 1, tTimeout := T#5s, hConn := hConn,
      nIdxGrp := 16#4020, nIdxOff := 0, nLen := 8);
write1(bExecute := st = 2, tTimeout := T#5s, hConn := hConn,
       nIdxGrp := 16#4020, nIdxOff := 8, pData := wrData, nLen := 8);
rstate1(bExecute := st = 3, tTimeout := T#5s, hConn := hConn);
wctrl1(bExecute := st = 4, tTimeout := T#5s, hConn := hConn,
       nAdsState := 5, nDevState := 0);
bread1(bExecute := st = 5 AND connBad.bDone, tTimeout := T#5s,
       hConn := connBad.hConn, nIdxGrp := 16#4020, nIdxOff := 0, nLen := 4);
close1(bExecute := st = 5, tTimeout := T#5s, hConn := hConn);

IF st = 0 AND conn1.bDone THEN
    IF conn1.nErrId = 0 AND conn1.hConn <> 0 THEN
        hConn := conn1.hConn;
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND read1.bDone THEN
    gotData := read1.pData;
    gotLen := read1.nBytes;
    IF read1.nErrId = 0 AND read1.nAdsError = 0 AND
       read1.nBytes = 8 THEN
        mask := mask + 2;
    END_IF;
    IF gotLen = 8 AND gotData = 'ABCDEFGH' THEN
        mask := mask + 4;
    END_IF;
    st := 2;
ELSIF st = 2 AND write1.bDone THEN
    IF write1.nErrId = 0 AND write1.nAdsError = 0 THEN
        mask := mask + 8;
    END_IF;
    st := 3;
ELSIF st = 3 AND rstate1.bDone THEN
    adsState := rstate1.nAdsState;
    devState := rstate1.nDevState;
    IF rstate1.nErrId = 0 AND rstate1.nAdsError = 0 AND
       rstate1.nAdsState = 5 AND rstate1.nDevState = 0 THEN
        mask := mask + 16;
    END_IF;
    st := 4;
ELSIF st = 4 AND wctrl1.bDone THEN
    IF wctrl1.nErrId = 0 AND wctrl1.nAdsError = 0 THEN
        mask := mask + 32;
    END_IF;
    st := 5;
ELSIF st = 5 AND connBad.bDone AND bread1.bDone AND close1.bDone THEN
    IF close1.nErrId = 0 THEN
        mask := mask + 64;
    END_IF;
    IF bread1.nErrId = 3 THEN
        mask := mask + 128;
    END_IF;
    st := 6;
END_IF;

scan_n := scan_n + 1;
END_PROGRAM
