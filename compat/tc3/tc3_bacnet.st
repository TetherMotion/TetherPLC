// Tc3_sACN -> Tc3_BACnetRev14 BACnet/IP client wrappers.
// Beckhoff's BACnet integration is device-configuration-driven
// (FBs are generated per configured device in TwinCAT), so there
// is no static TwinCAT FB API to mirror; these wrappers expose
// the tplc-bacnet service (ASHRAE 135 over UDP 47808) as
// poll-driven function blocks in the same shape as the other
// compat libraries.
// Differences from a TwinCAT BACnet device binding, documented in
// docs/plc/compat/README.md:
//   - Who-Is/ReadProperty/WriteProperty only; no COV subscriptions,
//     no device-object discovery beyond I-Am fields, no segmentation.
//   - Values cross as raw BACnet application-tag payload bytes:
//     nValueType selects the tag (1=boolean, 2=unsigned, 3=integer,
//     4=real, 5=double, 6=octetString, 7=characterString,
//     9=enumerated, 12=objectIdentifier) and pValue holds the tag
//     payload in wire byte order.
//   - nErrId carries tplc-bacnet service/transport codes.
// Sources: docs/plc/plan/21-twincat-libraries.md §7 (TF8020).

FUNCTION_BLOCK FB_BACnet_WhoIs
VAR_INPUT
    sHost : STRING[15];        // "" or "255.255.255.255" = broadcast
    nPort : UDINT := 47808;
    bHasRange : BOOL;
    nRangeLo : UDINT;
    nRangeHi : UDINT;
    bExecute : BOOL;
    tTimeout : TIME := T#1S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nDevCount : UDINT;
    nDevId : ARRAY[0..7] OF UDINT;   // (objType<<22)|instance
    nDevIp : ARRAY[0..31] OF BYTE;   // 4 bytes per device
    nDevVendor : UDINT;
    nDevMaxApdu : UDINT;
    nDevSeg : UDINT;
END_VAR
VAR
    bac : FB_Bacnet;
    i : UDINT;
END_VAR
    bac.FUNC := 1;
    bac.HOST := sHost;
    bac.PORT := nPort;
    bac.HAS_RANGE := bHasRange;
    bac.RANGE_LO := nRangeLo;
    bac.RANGE_HI := nRangeHi;
    bac.TIMEOUT := tTimeout;
    bac.EXEC := bExecute;
    bac();
    bBusy := bac.BUSY;
    bDone := bac.DONE;
    nStatus := bac.STATUS;
    nErrId := bac.ERRID;
    nDevCount := bac.DEV_COUNT;
    nDevVendor := bac.DEV_VENDOR;
    nDevMaxApdu := bac.DEV_MAX_APDU;
    nDevSeg := bac.DEV_SEG;
    IF bac.DONE THEN
        i := 0;
        WHILE i < bac.DEV_COUNT AND i < 8 DO
            nDevId[i] := bac.DEV_ID[i];
            nDevIp[i*4] := bac.DEV_IP[i*4];
            nDevIp[i*4+1] := bac.DEV_IP[i*4+1];
            nDevIp[i*4+2] := bac.DEV_IP[i*4+2];
            nDevIp[i*4+3] := bac.DEV_IP[i*4+3];
            i := i + 1;
        END_WHILE;
    END_IF;
    bError := (bac.ERRID <> 0) OR (bac.STATUS >= 2);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_BACnet_ReadProperty
VAR_INPUT
    sHost : STRING[15];
    nPort : UDINT := 47808;
    nObjType : UDINT;          // e.g. 8=device, 0=analogInput
    nObjInst : UDINT;
    nPropId : UDINT;           // e.g. 85=present-value, 77=object-name
    bHasIdx : BOOL;
    nPropIdx : UDINT;          // array index when bHasIdx
    bExecute : BOOL;
    tTimeout : TIME := T#1S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nService : UDINT;
    nApdu : UDINT;
    nRObjType : UDINT;
    nRObjInst : UDINT;
    nRPropId : UDINT;
    nValueType : UDINT;        // app tag of returned value
    nValueLen : UDINT;
    pValue : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    bac : FB_Bacnet;
    i : UDINT;
END_VAR
    bac.FUNC := 2;
    bac.HOST := sHost;
    bac.PORT := nPort;
    bac.OBJ_TYPE := nObjType;
    bac.OBJ_INST := nObjInst;
    bac.PROP_ID := nPropId;
    bac.HAS_IDX := bHasIdx;
    bac.PROP_IDX := nPropIdx;
    bac.TIMEOUT := tTimeout;
    bac.EXEC := bExecute;
    bac();
    bBusy := bac.BUSY;
    bDone := bac.DONE;
    nStatus := bac.STATUS;
    nErrId := bac.ERRID;
    nService := bac.R_SERVICE;
    nApdu := bac.R_APDU;
    nRObjType := bac.R_OBJ_TYPE;
    nRObjInst := bac.R_OBJ_INST;
    nRPropId := bac.R_PROP_ID;
    nValueType := bac.R_VTYPE;
    nValueLen := bac.R_VLEN;
    IF bac.DONE THEN
        i := 0;
        WHILE i < bac.R_VLEN AND i < 256 DO
            pValue[i] := bac.R_VAL[i];
            i := i + 1;
        END_WHILE;
    END_IF;
    bError := (bac.ERRID <> 0) OR (bac.STATUS >= 2);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_BACnet_WriteProperty
VAR_INPUT
    sHost : STRING[15];
    nPort : UDINT := 47808;
    nObjType : UDINT;
    nObjInst : UDINT;
    nPropId : UDINT;
    bHasIdx : BOOL;
    nPropIdx : UDINT;
    nValueType : UDINT;        // app tag for the value
    nValueLen : UDINT;
    pValue : ARRAY[0..255] OF BYTE;
    bHasPrio : BOOL;
    nPriority : UDINT;         // 1..16 when bHasPrio
    bExecute : BOOL;
    tTimeout : TIME := T#1S;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nStatus : DINT;
    nService : UDINT;
END_VAR
VAR
    bac : FB_Bacnet;
    i : UDINT;
END_VAR
    bac.FUNC := 3;
    bac.HOST := sHost;
    bac.PORT := nPort;
    bac.OBJ_TYPE := nObjType;
    bac.OBJ_INST := nObjInst;
    bac.PROP_ID := nPropId;
    bac.HAS_IDX := bHasIdx;
    bac.PROP_IDX := nPropIdx;
    bac.VAL_TYPE := nValueType;
    bac.VLEN := nValueLen;
    i := 0;
    WHILE i < nValueLen AND i < 256 DO
        bac.VAL[i] := pValue[i];
        i := i + 1;
    END_WHILE;
    bac.HAS_PRIO := bHasPrio;
    bac.PRIORITY := nPriority;
    bac.TIMEOUT := tTimeout;
    bac.EXEC := bExecute;
    bac();
    bBusy := bac.BUSY;
    bDone := bac.DONE;
    nStatus := bac.STATUS;
    nErrId := bac.ERRID;
    nService := bac.R_SERVICE;
    bError := (bac.ERRID <> 0) OR (bac.STATUS >= 2);
END_FUNCTION_BLOCK
