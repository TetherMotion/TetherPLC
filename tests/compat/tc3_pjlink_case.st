PROGRAM Tc3PjlinkCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    g         : FB_PJLink;
    g2        : FB_PJLink;
    p         : FB_PJLink_Power;
    i         : FB_PJLink_Input;
    m         : FB_PJLink_Mute;
    pa        : FB_PJLink_Power;
    pf        : FB_PJLink_Power;
END_VAR

// autonomous PJLink sequence against tests/compat/
// pjlink_test_peer.py (TCP 4352). The peer picks its greeting by
// connection count: conns 1-3 `PJLINK 0` (no auth), conn 4
// `PJLINK 1 a1b2c3d4` accepting md5(nonce+"secret"), conn 5
// `PJLINK 1 deadbeef` rejecting every digest, conns 6+ `PJLINK 0`.
// Each state's bExecute is high only while st equals its index,
// giving every FB one rising edge.
g(sHost := '127.0.0.1', nPort := 4352, nCls := 1, sCmd := 'POWR',
  sParam := '?', sPassword := '', bExecute := st = 0,
  tTimeout := T#2S);
p(sHost := '127.0.0.1', nPort := 4352, sPassword := '',
  nMode := 2, bExecute := st = 1, tTimeout := T#2S);
i(sHost := '127.0.0.1', nPort := 4352, sPassword := '',
  nMode := 0, sInput := '31', bExecute := st = 2,
  tTimeout := T#2S);
pa(sHost := '127.0.0.1', nPort := 4352, sPassword := 'secret',
   nMode := 2, bExecute := st = 3, tTimeout := T#2S);
pf(sHost := '127.0.0.1', nPort := 4352, sPassword := 'wrongpass',
   nMode := 2, bExecute := st = 4, tTimeout := T#2S);
m(sHost := '127.0.0.1', nPort := 4352, sPassword := '',
  nMode := 2, bExecute := st = 5, tTimeout := T#2S);
g2(sHost := '127.0.0.1', nPort := 4352, nCls := 1, sCmd := 'XXXX',
   sParam := '?', sPassword := '', bExecute := st = 6,
   tTimeout := T#2S);

IF st = 0 AND g.bDone THEN
    // POWR ? -> "%1POWR=1": value "1", no auth exchange
    IF g.nStatus = 1 AND g.sValue = '1' AND g.nRespCls = 1 AND
       g.nRespCmd = 1347376978 AND g.nValueLen = 1 AND
       NOT g.bAuthed AND g.nErrId = 0 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND p.bDone THEN
    // Power query -> "1"
    IF p.nStatus = 1 AND p.sValue = '1' AND p.nErrId = 0 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND i.bDone THEN
    // INPT 31 -> "OK"
    IF i.nStatus = 1 AND i.sValue = 'OK' AND i.nErrId = 0 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND pa.bDone THEN
    // authed POWR ? (digest accepted) -> AUTHED, value "1"
    IF pa.nStatus = 1 AND pa.sValue = '1' AND pa.bAuthed AND
       pa.nErrId = 0 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND pf.bDone THEN
    // wrong password -> PJLINK ERRA -> STATUS 5 (authfail)
    IF pf.nStatus = 5 AND pf.nErrId = 0 THEN
        mask := mask + 16;
    END_IF;
    st := 5;
ELSIF st = 5 AND m.bDone THEN
    // AVMT ? -> "31" (muted)
    IF m.nStatus = 1 AND m.sValue = '31' AND m.nErrId = 0 THEN
        mask := mask + 32;
    END_IF;
    st := 6;
ELSIF st = 6 AND g2.bDone THEN
    // unknown command -> "%1XXXX=ERR1": PJLink-level error text,
    // transport status still OK
    IF g2.nStatus = 1 AND g2.sValue = 'ERR1' AND g2.nErrId = 0 THEN
        mask := mask + 64;
    END_IF;
    st := 7;
END_IF;

END_PROGRAM
