PROGRAM Tc3FtpCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    conn      : FB_FTP_Connect;
    connBad   : FB_FTP_Connect;
    sz        : FB_FTP_Cmd;
    mkd       : FB_FTP_MkDir;
    dl        : FB_FTP_Download;
    ls        : FB_FTP_List;
    up        : FB_FTP_Upload;
    szUp      : FB_FTP_Cmd;
    fex       : FB_FTP_FileExists;
    dex       : FB_FTP_DirExists;
    rn        : FB_FTP_Rename;
    acc       : ARRAY[0..1023] OF BYTE;
    accLen    : UDINT;
    i         : UDINT;
    upData    : ARRAY[0..511] OF BYTE;
END_VAR

// autonomous FTP sequence against tests/compat/ftp_test_peer.py
// (TCP 2121, PASV). The peer serves /data.txt (68B), /bin.dat,
// dirs / and /pub. Each state's bExecute is high only while st
// equals its index, giving every FB one rising edge; bDone
// advances the machine.
upData[0] := 16#50; upData[1] := 16#4C; upData[2] := 16#43;   // 'P','L','C'
upData[3] := 16#20; upData[4] := 16#75; upData[5] := 16#70;   // ' ','u','p'
// "PLC upload payload 12345" = 24 bytes
upData[6] := 16#6C; upData[7] := 16#6F; upData[8] := 16#61;
upData[9] := 16#64; upData[10] := 16#20; upData[11] := 16#70;
upData[12] := 16#61; upData[13] := 16#79; upData[14] := 16#6C;
upData[15] := 16#6F; upData[16] := 16#61; upData[17] := 16#64;
upData[18] := 16#20; upData[19] := 16#31; upData[20] := 16#32;
upData[21] := 16#33; upData[22] := 16#34; upData[23] := 16#35;

conn(sHost := '127.0.0.1', nPort := 2121, sUser := 'u', sPass := 'p',
     bExecute := st = 0, bClose := st = 11, tTimeout := T#5S);
connBad(sHost := '127.0.0.1', nPort := 21999, sUser := 'u', sPass := 'p',
        bExecute := st = 10, bClose := FALSE, tTimeout := T#3S);
sz(nConnHandle := conn.nConnHandle, sCmd := 'SIZE /data.txt',
   bExecute := st = 1, tTimeout := T#3S);
mkd(nConnHandle := conn.nConnHandle, sPath := '/newdir',
    bExecute := st = 2, tTimeout := T#3S);
dl(nConnHandle := conn.nConnHandle, sRemotePath := '/data.txt',
   bExecute := st = 3, tTimeout := T#10S);
ls(nConnHandle := conn.nConnHandle, sRemotePath := '/',
   bExecute := st = 4, tTimeout := T#10S);
up(nConnHandle := conn.nConnHandle, sRemotePath := '/up.bin',
   nLen := 24, DATA := upData, bExecute := st = 5, tTimeout := T#10S);
szUp(nConnHandle := conn.nConnHandle, sCmd := 'SIZE /up.bin',
     bExecute := st = 6, tTimeout := T#3S);
fex(nConnHandle := conn.nConnHandle, sPath := '/data.txt',
    bExecute := st = 7, tTimeout := T#3S);
dex(nConnHandle := conn.nConnHandle, sPath := '/pub',
    bExecute := st = 8, tTimeout := T#10S);
rn(nConnHandle := conn.nConnHandle, sOldPath := '/data.txt',
   sNewPath := '/renamed.txt', bExecute := st = 9, tTimeout := T#10S);

// accumulate download chunks every scan while st = 3
IF dl.nChunk > 0 AND accLen + dl.nChunk <= 1024 THEN
    FOR i := 0 TO dl.nChunk - 1 DO
        acc[accLen + i] := dl.DATA[i];
    END_FOR;
    accLen := accLen + dl.nChunk;
END_IF;

IF st = 0 AND conn.bDone THEN
    // connect -> login completes, handle issued
    IF conn.nStatus = 1 AND conn.nConnHandle <> 0 AND
       conn.nErrId = 0 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND sz.bDone THEN
    // SIZE /data.txt -> 213, text "68"
    IF sz.nStatus = 1 AND sz.nRespCode = 213 AND
       sz.sText = '68' THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND mkd.bDone THEN
    // MKD /newdir -> 257
    IF mkd.nStatus = 1 AND mkd.nRespCode = 257 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND dl.bDone THEN
    // RETR /data.txt -> 68 bytes "Hello FTP World!\n"x4
    IF dl.nStatus = 1 AND dl.nTotal = 68 AND accLen = 68 AND
       acc[0] = 16#48 AND acc[16] = 16#0A THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND ls.bDone THEN
    // LIST / -> non-empty listing
    IF ls.nStatus = 1 AND ls.nTotal > 0 THEN
        mask := mask + 16;
    END_IF;
    st := 5;
ELSIF st = 5 AND up.bDone THEN
    // STOR /up.bin -> 226
    IF up.nStatus = 1 AND up.nRespCode = 226 THEN
        mask := mask + 32;
    END_IF;
    st := 6;
ELSIF st = 6 AND szUp.bDone THEN
    // SIZE /up.bin -> 213 "24"
    IF szUp.nStatus = 1 AND szUp.nRespCode = 213 AND
       szUp.sText = '24' THEN
        mask := mask + 64;
    END_IF;
    st := 7;
ELSIF st = 7 AND fex.bDone THEN
    // FileExists /data.txt -> SIZE 213
    IF fex.bExists AND fex.nRespCode = 213 THEN
        mask := mask + 128;
    END_IF;
    st := 8;
ELSIF st = 8 AND dex.bDone THEN
    // DirExists /pub -> LIST ok
    IF dex.bExists AND dex.nStatus = 1 THEN
        mask := mask + 256;
    END_IF;
    st := 9;
ELSIF st = 9 AND rn.bDone THEN
    // Rename /data.txt -> /renamed.txt: RNFR 350 + RNTO 250
    IF rn.nStatus = 1 AND rn.nRespCode = 250 AND NOT rn.bError THEN
        mask := mask + 512;
    END_IF;
    st := 10;
ELSIF st = 10 AND connBad.bDone THEN
    // refused connect -> STATUS 2
    IF connBad.nStatus = 2 THEN
        mask := mask + 1024;
    END_IF;
    st := 11;
ELSIF st = 11 AND conn.bDone THEN
    // CLOSE -> session torn down
    IF conn.nConnHandle = 0 THEN
        mask := mask + 2048;
    END_IF;
    st := 12;
END_IF;
END_PROGRAM
