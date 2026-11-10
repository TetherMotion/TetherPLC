PROGRAM Tc2ModbusSrvCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    scan_n    : UDINT;
    rregs     : FB_MBReadRegs;
    wsreg     : FB_MBWriteSingleReg;
    wregs     : FB_MBWriteRegs;
    rcoils    : FB_MBReadCoils;
    rbad      : FB_MBReadRegs;
    rwregs    : FB_MBReadWriteRegs;
    wsrc      : ARRAY[0..255] OF BYTE;
END_VAR

// autonomous Modbus TCP sequence against tests/compat/
// modbus_test_server.py: read regs -> write single reg ->
// write regs -> read coils -> exception read -> read/write regs.
// Each state's bExecute is high only while st equals its index,
// giving every FB one rising edge.
wsrc[0] := 16#12;
wsrc[1] := 16#34;

rregs(bExecute := st = 0, tTimeout := T#5s,
      sHost := '127.0.0.1', nTCPPort := 15088,
      nUnitID := 16#FF, nQuantity := 2, nMBAddr := 0,
      cbLength := 256);
wsreg(bExecute := st = 1, tTimeout := T#5s,
      sHost := '127.0.0.1', nTCPPort := 15088,
      nUnitID := 16#FF, nMBAddr := 0, nValue := 16#BEEF);
wregs(bExecute := st = 2, tTimeout := T#5s,
      sHost := '127.0.0.1', nTCPPort := 15088,
      nUnitID := 16#FF, nQuantity := 2, nMBAddr := 0,
      cbLength := 4, pSrcData := wsrc);
rcoils(bExecute := st = 3, tTimeout := T#5s,
       sHost := '127.0.0.1', nTCPPort := 15088,
       nUnitID := 16#FF, nQuantity := 8, nMBAddr := 0,
       cbLength := 256);
rbad(bExecute := st = 4, tTimeout := T#5s,
     sHost := '127.0.0.1', nTCPPort := 15088,
     nUnitID := 16#FF, nQuantity := 2, nMBAddr := 99,
     cbLength := 256);
rwregs(bExecute := st = 5, tTimeout := T#5s,
       sHost := '127.0.0.1', nTCPPort := 15088,
       nUnitID := 16#FF, nReadQuantity := 2, nMBReadAddr := 0,
       nWriteQuantity := 1, nMBWriteAddr := 0,
       cbDestLength := 256, cbSrcLength := 2,
       pSrcData := wsrc);

IF st = 0 AND rregs.bDone THEN
    // func3 resp PDU data = bytecount + reg bytes
    IF rregs.bError = FALSE AND rregs.cbRead = 5 AND
       rregs.pData[0] = 4 AND rregs.pData[1] = 16#10 AND
       rregs.pData[2] = 16#00 AND rregs.pData[3] = 16#10 AND
       rregs.pData[4] = 16#01 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND wsreg.bDone THEN
    IF wsreg.bError = FALSE AND wsreg.nErrId = 0 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND wregs.bDone THEN
    IF wregs.bError = FALSE AND wregs.nErrId = 0 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND rcoils.bDone THEN
    // func1 resp = bytecount(1) + 0xA5
    IF rcoils.bError = FALSE AND rcoils.cbRead = 2 AND
       rcoils.pData[0] = 1 AND rcoils.pData[1] = 16#A5 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND rbad.bDone THEN
    // addr 99 -> server exception 2 -> nErrId = 16#8002
    IF rbad.bError = TRUE AND rbad.nErrId = 16#8002 THEN
        mask := mask + 16;
    END_IF;
    st := 5;
ELSIF st = 5 AND rwregs.bDone THEN
    IF rwregs.bError = FALSE AND rwregs.cbRead = 5 AND
       rwregs.pDestData[0] = 4 AND rwregs.pDestData[1] = 16#10 AND
       rwregs.pDestData[4] = 16#01 THEN
        mask := mask + 32;
    END_IF;
    st := 6;
END_IF;

scan_n := scan_n + 1;
END_PROGRAM
