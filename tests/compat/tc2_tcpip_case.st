PROGRAM Tc2TcpIpCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    scan_n    : UDINT;
    hListener : UDINT;
    hCli      : UDINT;
    hSrv      : UDINT;
    gotData   : STRING[256];
    gotLen    : UDINT;
    sentMsg   : STRING[256];
    listen1   : FB_SocketListen;
    conn1     : FB_SocketConnect;
    acc1      : FB_SocketAccept;
    send1     : FB_SocketSend;
    recv1     : FB_SocketReceive;
    closeCli  : FB_SocketClose;
    closeSrv  : FB_SocketClose;
    closeLst  : FB_SocketClose;
END_VAR

// autonomous loopback sequence: listen -> connect -> accept ->
// send -> receive -> close. Each state's bExecute is high only
// while st equals its index, giving every FB one rising edge.
sentMsg := 'PING';

listen1(bExecute := st = 0, tTimeout := T#5s,
        sHost := '', nPort := 17531);
conn1(bExecute := st = 1, tTimeout := T#5s,
      sHost := '127.0.0.1', nPort := 17531);
acc1(bExecute := st = 2, tTimeout := T#5s, hListener := hListener);
send1(bExecute := st = 3, tTimeout := T#5s, hSocket := hCli,
      pData := sentMsg, nLen := 4);
recv1(bExecute := st = 4, tTimeout := T#5s, hSocket := hSrv);
closeCli(bExecute := st = 5, tTimeout := T#5s, hSocket := hCli);
closeSrv(bExecute := st = 5, tTimeout := T#5s, hSocket := hSrv);
closeLst(bExecute := st = 5, tTimeout := T#5s, hSocket := hListener);

IF st = 0 AND listen1.bDone THEN
    IF listen1.nErrId = 0 AND listen1.hListener <> 0 THEN
        hListener := listen1.hListener;
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND conn1.bDone THEN
    IF conn1.nErrId = 0 AND conn1.hSocket <> 0 THEN
        hCli := conn1.hSocket;
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND acc1.bDone THEN
    IF acc1.bAccepted AND acc1.nErrId = 0 AND acc1.hSocket <> 0 THEN
        hSrv := acc1.hSocket;
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND send1.bDone THEN
    IF send1.nErrId = 0 AND send1.nSent = 4 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND recv1.bDone THEN
    gotData := recv1.pData;
    gotLen := recv1.nRecBytes;
    IF recv1.nErrId = 0 AND recv1.nRecBytes = 4 THEN
        mask := mask + 16;
    END_IF;
    IF gotLen = 4 AND gotData = 'PING' THEN
        mask := mask + 32;
    END_IF;
    st := 5;
ELSIF st = 5 AND closeCli.bDone AND closeSrv.bDone AND closeLst.bDone THEN
    IF closeCli.nErrId = 0 AND closeSrv.nErrId = 0 AND
       closeLst.nErrId = 0 THEN
        mask := mask + 64;
    END_IF;
    st := 6;
END_IF;

scan_n := scan_n + 1;
END_PROGRAM
