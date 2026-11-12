// tc2_iofunctions_case.st — e2e for tc2_iofunctions (IOF general).
// The program declares three AT-bound "boxes"; the IOF blocks
// enumerate them by count, name and address.
PROGRAM tc2_iofunctions_case
VAR
    nBox : IOF_GetBoxCount;
    nDev : IOF_GetDeviceCount;
    byName : IOF_GetBoxAddrByName;
    byAddr : IOF_GetBoxNameByAddr;
    devName : IOF_GetDeviceName;
    devReset : IOF_DeviceReset;
    netId : IOF_GetDeviceNetId;
    in1 AT %IX0.0 : BOOL;
    out1 AT %QW2 : WORD;
    mem1 AT %MD4 : DWORD;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        nBox.bExecute := TRUE;
        st := 1;
    ELSIF st = 1 THEN
        nBox.bExecute := FALSE;
        IF nBox.bDone AND nBox.nCount = 5 THEN mask := mask OR 1; END_IF;
        nDev.bExecute := TRUE;
        st := 2;
    ELSIF st = 2 THEN
        nDev.bExecute := FALSE;
        IF nDev.bDone AND nDev.nCount = 1 THEN mask := mask OR 2; END_IF;
        byName.sName := 'out1';
        st := 3;
    ELSIF st = 3 THEN
        byName.bExecute := TRUE;
        st := 4;
    ELSIF st = 4 THEN
        byName.bExecute := FALSE;
        IF byName.bDone AND byName.nRegion = 1 AND byName.nAddr = 2 THEN
            mask := mask OR 4;
        END_IF;
        byAddr.nRegion := 2; byAddr.nAddr := 4;
        st := 5;
    ELSIF st = 5 THEN
        byAddr.bExecute := TRUE;
        st := 6;
    ELSIF st = 6 THEN
        byAddr.bExecute := FALSE;
        IF byAddr.bDone AND FIND(byAddr.sName, 'mem1') = 1 THEN
            mask := mask OR 8;
        END_IF;
        devName.nDevId := 0;
        st := 7;
    ELSIF st = 7 THEN
        devName.bExecute := TRUE;
        st := 8;
    ELSIF st = 8 THEN
        devName.bExecute := FALSE;
        IF devName.bDone AND FIND(devName.sName, 'Local') = 1 THEN
            mask := mask OR 16;
        END_IF;
        devReset.nDevId := 0;
        st := 9;
    ELSIF st = 9 THEN
        devReset.bExecute := TRUE;
        st := 10;
    ELSIF st = 10 THEN
        devReset.bExecute := FALSE;
        IF devReset.bError AND devReset.nErrId = 16#8001 THEN
            mask := mask OR 32;
        END_IF;
        netId.nDevId := 0;
        st := 11;
    ELSIF st = 11 THEN
        netId.bExecute := TRUE;
        st := 12;
    ELSIF st = 12 THEN
        netId.bExecute := FALSE;
        IF netId.bDone AND FIND(netId.sNetId, '127.0.0.1') = 1 THEN
            mask := mask OR 64;
        END_IF;
        st := 13;
    END_IF;
    nBox(); nDev(); byName(); byAddr(); devName(); devReset(); netId();
END_PROGRAM
