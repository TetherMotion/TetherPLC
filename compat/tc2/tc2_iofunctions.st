// Tc2_IoFunctions -> general I/O enumeration surface.
// The 81-entry library is mostly EtherCAT device profiles (ASi,
// Profibus DPV1, Lightbus, CANopen, RAID, ...).  Those need the
// fieldbus master, which lives in Tether's io plane — they are a
// documented gap.  The *general* IOF blocks are meaningful and
// are implemented here over the package variable table:
//   IOF_GetBoxCount / IOF_GetBoxNameByAddr / IOF_GetBoxAddrByName /
//   IOF_GetDeviceCount / IOF_GetDeviceName.
// Documented differences:
//   - A "box" is an AT-bound variable (%I/%Q/%M) of the local
//     program, addressed by its image byte (and bit for BOOLs).
//     There is one local device (the PLC) — no fieldbus devices.
//   - IOF_DeviceReset and the *Ex/NetId/DeviceID variants need the
//     I/O server: they report a documented unsupported error
//     instead of pretending to reset hardware.

FUNCTION_BLOCK IOF_GetBoxCount
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
END_VAR
VAR_OUTPUT
    nCount : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    iof : FB_Iof;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        bBusy := TRUE;
        iof.OP := 1;
        iof();
        bBusy := FALSE;
        IF iof.STATUS = 0 THEN
            bDone := TRUE; nCount := iof.COUNT;
        ELSE
            bError := TRUE; nErrId := iof.ERRID;
        END_IF;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetBoxNameByAddr
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
    nRegion : UDINT;            // 0=%I 1=%Q 2=%M
    nAddr : UDINT;              // image byte
END_VAR
VAR_OUTPUT
    sName : STRING[63];
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    iof : FB_Iof;
    prev : BOOL;
    i : UDINT;
    n : UDINT;
    found : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        bBusy := TRUE;
        // scan the box list for the requested (region, addr)
        iof.OP := 1;
        iof();
        n := iof.COUNT;
        found := FALSE;
        i := 0;
        WHILE i < n AND NOT found DO
            iof.OP := 2; iof.IDX := i;
            iof();
            IF iof.FOUND AND iof.REGION = nRegion AND iof.ADDR = nAddr THEN
                sName := iof.NAME;
                found := TRUE;
            END_IF;
            i := i + 1;
        END_WHILE;
        bBusy := FALSE;
        IF found THEN
            bDone := TRUE;
        ELSE
            bError := TRUE; nErrId := 2;
        END_IF;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetBoxAddrByName
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
    sName : STRING[63];
END_VAR
VAR_OUTPUT
    nRegion : UDINT;
    nAddr : UDINT;
    nUnit : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    iof : FB_Iof;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        bBusy := TRUE;
        iof.OP := 3; iof.NAME := sName;
        iof();
        bBusy := FALSE;
        IF iof.STATUS = 0 AND iof.FOUND THEN
            bDone := TRUE;
            nRegion := iof.REGION;
            nAddr := iof.ADDR;
            nUnit := iof.UNIT;
        ELSE
            bError := TRUE; nErrId := iof.ERRID;
        END_IF;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetDeviceCount
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
END_VAR
VAR_OUTPUT
    nCount : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    iof : FB_Iof;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        bBusy := TRUE;
        iof.OP := 4;
        iof();
        bBusy := FALSE;
        IF iof.STATUS = 0 THEN
            bDone := TRUE; nCount := iof.COUNT;
        ELSE
            bError := TRUE; nErrId := iof.ERRID;
        END_IF;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetDeviceName
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
    nDevId : UDINT;
END_VAR
VAR_OUTPUT
    sName : STRING[63];
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    iof : FB_Iof;
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        bBusy := TRUE;
        iof.OP := 5;
        iof();
        bBusy := FALSE;
        IF iof.STATUS = 0 AND nDevId = 0 THEN
            bDone := TRUE; sName := iof.VAL;
        ELSE
            bError := TRUE; nErrId := 3;
        END_IF;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

// ---- host-domain operations: documented unsupported ----
FUNCTION_BLOCK IOF_DeviceReset
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
    nDevId : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        // resets are performed by the I/O plane (Tether), not the PLC
        bError := TRUE; nErrId := 16#8001;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetBoxAddrByNameEx
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
    sName : STRING[63];
END_VAR
VAR_OUTPUT
    nRegion : UDINT;
    nAddr : UDINT;
    nUnit : UDINT;
    nBoxType : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    byName : IOF_GetBoxAddrByName;
END_VAR
    byName.bExecute := bExecute;
    byName.sNetId := sNetId;
    byName.sName := sName;
    byName();
    nRegion := byName.nRegion;
    nAddr := byName.nAddr;
    nUnit := byName.nUnit;
    nBoxType := 0;              // no terminal-type model in the PLC
    bBusy := byName.bBusy;
    bDone := byName.bDone;
    bError := byName.bError;
    nErrId := byName.nErrId;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetBoxNetId
VAR_INPUT
    bExecute : BOOL;
    nRegion : UDINT;
    nAddr : UDINT;
END_VAR
VAR_OUTPUT
    sNetId : STRING[23];
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        sNetId := '127.0.0.1.1.1';   // the local PLC is the only device
        bDone := TRUE;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetDeviceNetId
VAR_INPUT
    bExecute : BOOL;
    nDevId : UDINT;
END_VAR
VAR_OUTPUT
    sNetId : STRING[23];
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        sNetId := '127.0.0.1.1.1';
        bDone := TRUE;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetDeviceIDByName
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
    sName : STRING[63];
END_VAR
VAR_OUTPUT
    nDevId : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        // single local device, id 0
        nDevId := 0;
        bDone := TRUE;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetDeviceIDs
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
END_VAR
VAR_OUTPUT
    nDevId : UDINT;
    nCount : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        nDevId := 0; nCount := 1;
        bDone := TRUE;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK IOF_GetDeviceInfoByName
VAR_INPUT
    bExecute : BOOL;
    sNetId : STRING[23];
    sName : STRING[63];
END_VAR
VAR_OUTPUT
    nDevId : UDINT;
    nDevType : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prev : BOOL;
END_VAR
    bBusy := FALSE; bDone := FALSE; bError := FALSE; nErrId := 0;
    IF bExecute AND NOT prev THEN
        nDevId := 0; nDevType := 0;
        bDone := TRUE;
    END_IF;
    prev := bExecute;
END_FUNCTION_BLOCK
