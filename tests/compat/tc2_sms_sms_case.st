PROGRAM Tc2SmsSmtpCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    hPort     : UDINT;
    open1     : FB_SerialOpen;
    close1    : FB_SerialClose;
    sms1      : FB_SendSMS;
    smtp1     : FB_SmtpV3;
    smtp2     : FB_SmtpV3_Full;
    smtpBad   : FB_SmtpV3_Full;
    smtpDead  : FB_SmtpV3;
    attach1   : FB_SmtpAttach;
    ver1      : FB_GetTcPlcSMS_Version;
END_VAR

// autonomous sequence: open pty serial -> AT SMS -> SMTP plain
// -> SMTP auth -> SMTP reject -> attach-unsupported -> version
// -> refused connect -> close. sDevice is substituted by the
// test script with the pty slave path.

open1(bExecute := st = 0, tTimeout := T#5S,
      sDevice := '__PTY_SLAVE__', nBaud := 9600,
      nParity := 0, nDataBits := 8, nStopBits := 1, nFlags := 1);
close1(bExecute := st = 8, tTimeout := T#5S, hPort := hPort);

sms1(bExecute := st = 1, tTimeout := T#10S, hPort := hPort,
     sNumber := '+4915123456', sText := 'Hello TetherPLC');

smtp1(bExecute := st = 2, tTimeout := T#10S,
      sServer := '127.0.0.1', nPort := 2525,
      sFrom := 'plc@home.local', sTo := 'ops@example.com',
      sSubject := 'Plain Send',
      sBody := 'first line$N.second line dot-stuffed');

smtp2(bExecute := st = 3, tTimeout := T#10S,
      sServer := '127.0.0.1', nPort := 2525,
      sUser := 'alice', sPass := 's3cret',
      sFrom := 'plc@home.local', sTo := 'auth@example.com',
      sSubject := 'Auth Send', sBody := 'authenticated body');

smtpBad(bExecute := st = 4, tTimeout := T#10S,
        sServer := '127.0.0.1', nPort := 2525,
        sUser := 'alice', sPass := 's3cret',
        sFrom := 'plc@home.local', sTo := 'reject@example.com',
        sSubject := 'Reject', sBody := 'should fail');

attach1(bExecute := st = 5, tTimeout := T#10S,
        sServer := '127.0.0.1', nPort := 2525,
        sUser := '', sPass := '',
        sFrom := 'plc@home.local', sTo := 'ops@example.com',
        sSubject := 'Attach', sBody := 'x',
        sAttachPath := '/tmp/nope.bin');

ver1(bExecute := st = 6);

smtpDead(bExecute := st = 7, tTimeout := T#5S,
         sServer := '127.0.0.1', nPort := 2999,
         sFrom := 'plc@home.local', sTo := 'ops@example.com',
         sSubject := 'Dead', sBody := 'x');

IF st = 0 AND open1.bDone THEN
    IF open1.nErrId = 0 AND open1.hPort <> 0 THEN
        hPort := open1.hPort;
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND sms1.bDone THEN
    IF sms1.nStatus = 0 AND sms1.bError = FALSE THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND smtp1.bDone THEN
    IF smtp1.nStatus = 1 AND smtp1.nRespCode = 250 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND smtp2.bDone THEN
    IF smtp2.nStatus = 1 AND smtp2.nRespCode = 250 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND smtpBad.bDone THEN
    IF smtpBad.nStatus = 2 AND smtpBad.nRespCode = 550 THEN
        mask := mask + 16;
    END_IF;
    st := 5;
ELSIF st = 5 AND attach1.bDone THEN
    IF attach1.nErrId = 90 AND attach1.bError THEN
        mask := mask + 32;
    END_IF;
    st := 6;
ELSIF st = 6 AND ver1.bDone THEN
    IF ver1.sVersion = 'TetherPLC-TcSMS 1.0' THEN
        mask := mask + 64;
    END_IF;
    st := 7;
ELSIF st = 7 AND smtpDead.bDone THEN
    IF smtpDead.nStatus >= 3 THEN
        mask := mask + 128;
    END_IF;
    st := 8;
ELSIF st = 8 AND close1.bDone THEN
    IF close1.nErrId = 0 THEN
        mask := mask + 256;
    END_IF;
    st := 9;
END_IF;

END_PROGRAM
