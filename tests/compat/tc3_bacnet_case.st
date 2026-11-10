PROGRAM Tc3BacnetCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    w         : FB_BACnet_WhoIs;
    w2        : FB_BACnet_WhoIs;
    r         : FB_BACnet_ReadProperty;
    wr        : FB_BACnet_WriteProperty;
    rn        : FB_BACnet_ReadProperty;
    re        : FB_BACnet_ReadProperty;
    wval      : ARRAY[0..255] OF BYTE;
END_VAR

// autonomous BACnet/IP sequence against tests/compat/
// bacnet_test_peer.py (UDP 47808): broadcast Who-Is with a
// 1000..2000 instance range -> I-Am; ReadProperty real /
// unsigned / object-name; WriteProperty Simple-ACK; Error for
// the unknown object. Each state's bExecute is high only while
// st equals its index, giving every FB one rising edge.
wval[0] := 16#41;
wval[1] := 16#64;
wval[2] := 16#00;
wval[3] := 16#00;   // REAL 22.5 big-endian payload

w(bExecute := st = 0, sHost := '255.255.255.255', nPort := 47808,
  bHasRange := TRUE, nRangeLo := 1000, nRangeHi := 2000,
  tTimeout := T#1S);
r(bExecute := st = 1, sHost := '127.0.0.1', nPort := 47808,
  nObjType := 0, nObjInst := 1, nPropId := 85,
  bHasIdx := FALSE, nPropIdx := 0, tTimeout := T#1S);
wr(bExecute := st = 2, sHost := '127.0.0.1', nPort := 47808,
   nObjType := 1, nObjInst := 1, nPropId := 85,
   bHasIdx := FALSE, nPropIdx := 0,
   nValueType := 4, nValueLen := 4, pValue := wval,
   bHasPrio := TRUE, nPriority := 8, tTimeout := T#1S);
rn(bExecute := st = 3, sHost := '127.0.0.1', nPort := 47808,
   nObjType := 8, nObjInst := 1234, nPropId := 77,
   bHasIdx := FALSE, nPropIdx := 0, tTimeout := T#1S);
re(bExecute := st = 4, sHost := '127.0.0.1', nPort := 47808,
   nObjType := 0, nObjInst := 9999, nPropId := 85,
   bHasIdx := FALSE, nPropIdx := 0, tTimeout := T#1S);
w2(bExecute := st = 5, sHost := '127.0.0.1', nPort := 47808,
   bHasRange := FALSE, nRangeLo := 0, nRangeHi := 0,
   tTimeout := T#1S);

IF st = 0 AND w.bDone THEN
    // broadcast I-Am: device(8).1234, vendor 260, maxAPDU 1476,
    // segmentation 0; the reply source is the routed interface
    // address, so only check that dev_ip was populated
    IF w.nDevCount >= 1 AND w.nDevId[0] = 33555666 AND
       NOT (w.nDevIp[0] = 0 AND w.nDevIp[1] = 0 AND
            w.nDevIp[2] = 0 AND w.nDevIp[3] = 0) AND
       w.nDevVendor = 260 AND w.nDevMaxApdu = 1476 AND
       w.nDevSeg = 0 AND w.nErrId = 0 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND r.bDone THEN
    // Complex-ACK echoes obj/prop; value = REAL 21.5 (41 AC 00 00)
    IF r.nStatus = 1 AND r.nService = 12 AND r.nValueType = 4 AND
       r.nValueLen = 4 AND r.pValue[0] = 16#41 AND
       r.pValue[1] = 16#AC AND r.pValue[2] = 16#00 AND
       r.pValue[3] = 16#00 AND r.nRObjType = 0 AND
       r.nRObjInst = 1 AND r.nRPropId = 85 AND r.nErrId = 0 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND wr.bDone THEN
    IF wr.nStatus = 1 AND wr.nService = 15 AND wr.nErrId = 0 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND rn.bDone THEN
    // object-name 'TETH-PLC' = 54 45 54 48 2D 50 4C 43
    IF rn.nStatus = 1 AND rn.nValueType = 7 AND rn.nValueLen = 8 AND
       rn.pValue[0] = 16#54 AND rn.pValue[1] = 16#45 AND
       rn.pValue[4] = 16#2D AND rn.pValue[7] = 16#43 AND
       rn.nRObjType = 8 AND rn.nRObjInst = 1234 AND
       rn.nRPropId = 77 AND rn.nErrId = 0 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND re.bDone THEN
    // unknown object -> BACnet Error APDU -> STATUS 3
    IF re.nStatus = 3 AND re.nErrId = 0 THEN
        mask := mask + 16;
    END_IF;
    st := 5;
ELSIF st = 5 AND w2.bDone THEN
    // directed (BVLC unicast) Who-Is: reply source is 127.0.0.1
    IF w2.nDevCount >= 1 AND w2.nDevId[0] = 33555666 AND
       w2.nDevIp[0] = 127 AND w2.nDevIp[1] = 0 AND
       w2.nDevIp[2] = 0 AND w2.nDevIp[3] = 1 AND w2.nErrId = 0 THEN
        mask := mask + 32;
    END_IF;
    st := 6;
END_IF;

END_PROGRAM
