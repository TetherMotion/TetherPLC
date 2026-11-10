PROGRAM Tc2SerialCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    scan_n    : UDINT;
    hPort     : UDINT;
    gotData   : STRING[256];
    gotLen    : UDINT;
    sentMsg   : STRING[256];
    open1     : FB_SerialOpen;
    openBad   : FB_SerialOpen;
    send1     : FB_SerialSend;
    recv1     : FB_SerialReceive;
    flush1    : FB_SerialFlush;
    rs485_1   : FB_SerialRs485;
    close1    : FB_SerialClose;
END_VAR

// autonomous pty loopback sequence: open -> send -> receive ->
// flush -> rs485 -> close, then a bad-path open for the error leg.
// sDevice is substituted by the test script with the pty slave path.
sentMsg := 'PING';

open1(bExecute := st = 0, tTimeout := T#5s,
      sDevice := '__PTY_SLAVE__', nBaud := 9600,
      nParity := 0, nDataBits := 8, nStopBits := 1, nFlags := 1);
openBad(bExecute := st = 5, tTimeout := T#5s,
        sDevice := '/nonexistent-tplc-serial', nBaud := 9600,
        nParity := 0, nDataBits := 8, nStopBits := 1, nFlags := 1);
send1(bExecute := st = 1, tTimeout := T#5s, hPort := hPort,
      pData := sentMsg, nLen := 4);
recv1(bExecute := st = 2, tTimeout := T#5s, hPort := hPort);
flush1(bExecute := st = 3, tTimeout := T#5s, hPort := hPort, nPurge := 3);
rs485_1(bExecute := st = 4, tTimeout := T#5s, hPort := hPort,
        bEnable := TRUE);
close1(bExecute := st = 5, tTimeout := T#5s, hPort := hPort);

IF st = 0 AND open1.bDone THEN
    IF open1.nErrId = 0 AND open1.hPort <> 0 THEN
        hPort := open1.hPort;
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND send1.bDone THEN
    IF send1.nErrId = 0 AND send1.nSent = 4 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND recv1.bDone THEN
    gotData := recv1.pData;
    gotLen := recv1.nRecBytes;
    IF recv1.nErrId = 0 AND recv1.nRecBytes = 4 THEN
        mask := mask + 4;
    END_IF;
    IF gotLen = 4 AND gotData = 'PING' THEN
        mask := mask + 8;
    END_IF;
    st := 3;
ELSIF st = 3 AND flush1.bDone THEN
    IF flush1.nErrId = 0 THEN
        mask := mask + 16;
    END_IF;
    st := 4;
ELSIF st = 4 AND rs485_1.bDone THEN
    IF rs485_1.nErrId = 0 THEN
        mask := mask + 32;
    END_IF;
    st := 5;
ELSIF st = 5 AND close1.bDone AND openBad.bDone THEN
    IF close1.nErrId = 0 AND openBad.nErrId = 3 THEN
        mask := mask + 64;
    END_IF;
    IF openBad.nErrId = 3 THEN
        mask := mask + 128;
    END_IF;
    st := 6;
END_IF;

scan_n := scan_n + 1;
END_PROGRAM
