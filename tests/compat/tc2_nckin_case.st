PROGRAM Tc2NcKinCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    cur    : INT;
    pulse  : BOOL;
    cfgA   : FB_Kin;
    cfgB   : FB_Kin;
    grp0   : FB_KinConfigGroup;
    grp1   : FB_KinConfigGroup;
    opst   : F_KinGetChnOperationState;
    ids0   : F_KinGetAcsMcsAxisIds;
    ids1   : F_KinGetAcsMcsAxisIds;
    xf     : FB_KinCalcTrafo;
    xb     : FB_KinCalcTrafo;
    xf5    : FB_KinCalcTrafo;
    xf99   : FB_KinCalcTrafo;
    mf     : FB_KinCalcMultiTrafo;
    mb     : FB_KinCalcMultiTrafo;
    mf2    : FB_KinCalcMultiTrafo;
    unl    : FB_KinUnlockTrafoParam;
    lck    : FB_KinLockTrafoParam;
    rotP   : FB_KinPresetRotation;
    extR   : FB_KinExtendedRotationRange;
    rst    : FB_KinResetGroup;
    chk    : FB_KinCheckActualStatus;
    kinSt  : UDINT;
    inA    : ARRAY[0..7] OF LREAL;
    outA   : ARRAY[0..7] OF LREAL;
    ms     : UDINT;
    me     : UDINT;
    inM    : ARRAY[0..7] OF LREAL;
    outM   : ARRAY[0..15] OF LREAL;
    inM2   : ARRAY[0..7] OF LREAL;
    outM2  : ARRAY[0..15] OF LREAL;
    acsArr : ARRAY[0..7] OF UDINT;
    mcsArr : ARRAY[0..7] OF UDINT;
    acsEmp : ARRAY[0..7] OF UDINT;
    mcsEmp : ARRAY[0..7] OF UDINT;
    rotArr : ARRAY[0..2] OF LREAL;
    tgt    : ARRAY[0..7] OF LREAL;
    jnt    : ARRAY[0..7] OF LREAL;
    posA   : ARRAY[0..7] OF LREAL;
    posB   : ARRAY[0..7] OF LREAL;
    tolA   : ARRAY[0..7] OF LREAL;
    tolOk1 : BOOL;
    tolOk2 : BOOL;
    started : BOOL;
    i      : UDINT;
END_VAR

cur := st;
/* pulse on stage entry; the very first scan counts too (prevSt
   would otherwise equal cur=0 and swallow the st0 edge). */
pulse := (cur <> prevSt) OR NOT started;
started := TRUE;
prevSt := cur;

// Persistent inputs for the edge-triggered wrappers (called every
// scan so their internal Execute edge tracking stays live).
FOR i := 0 TO 7 DO
    acsArr[i] := 0;
    mcsArr[i] := 0;
    inA[i] := 0.0;
    outA[i] := 0.0;
END_FOR;
acsArr[0] := 1; acsArr[1] := 2;
mcsArr[0] := 3; mcsArr[1] := 4; mcsArr[2] := 5;
rotArr[0] := 0.1; rotArr[1] := 0.2; rotArr[2] := 0.3;
kinSt := 0;

// st0: configure obj0 = planar2d (L1=L2=0.5), obj1 = serial6 UR5
grp0(bExecute := FALSE, bCartesianMode := FALSE, oidTrafo := 0,
     nItpChannelId := 0, AcsAxis := acsArr, McsAxis := mcsArr);
grp1(bExecute := FALSE, bCartesianMode := FALSE, oidTrafo := 1,
     nItpChannelId := 0, AcsAxis := acsEmp, McsAxis := mcsEmp);
IF cur = 0 THEN
    IF pulse THEN
        cfgA.OBJ := 0; cfgA.OP := 1; cfgA.TYPEIN := 1;
        cfgA.PRESET := 0; cfgA.NCFG := 2;
        cfgA.CFG[0] := 0.5; cfgA.CFG[1] := 0.5;
        cfgA();
        cfgB.OBJ := 1; cfgB.OP := 1; cfgB.TYPEIN := 4;
        cfgB.PRESET := 0; cfgB.NCFG := 0;
        cfgB();
        IF cfgA.STATUS = 0 AND cfgB.STATUS = 0 THEN
            mask := mask + 1;                   // bit0: configure
        END_IF;
    END_IF;
    st := 1;
// st1: bind axes + channel on both objects
ELSIF cur = 1 THEN
    grp0(bExecute := pulse, bCartesianMode := FALSE, oidTrafo := 0,
         nItpChannelId := 7, AcsAxis := acsArr, McsAxis := mcsArr);
    grp1(bExecute := pulse, bCartesianMode := FALSE, oidTrafo := 1,
         nItpChannelId := 0, AcsAxis := acsEmp, McsAxis := mcsEmp);
    IF pulse AND NOT grp0.bError AND NOT grp1.bError THEN
        mask := mask + 2;                       // bit1: config group
    END_IF;
    st := 2;
// st2: op state + axis list readback
ELSIF cur = 2 THEN
    opst(oidTrafo := 0);
    ids0(oidTrafo := 0);
    ids1(oidTrafo := 1);
    IF opst.OpState = 1 AND opst.ErrorId = 0 AND
       ids0.ErrorId = 0 AND ids0.NAcs = 2 AND ids0.NMcs = 3 AND
       ids0.AcsAxis[0] = 1 AND ids0.AcsAxis[1] = 2 AND
       ids0.McsAxis[0] = 3 AND ids0.McsAxis[2] = 5 AND
       ids1.NAcs = 0 THEN
        mask := mask + 4;                       // bit2: status/axes
    END_IF;
    st := 3;
// st3: forward transform planar2d q={0.3,0.4}
ELSIF cur = 3 THEN
    inA[0] := 0.3; inA[1] := 0.4;
    xf(bExecute := pulse, bForward := TRUE, oidTrafo := 0,
       AxesPosIn := inA, AxesPosOut := outA,
       MetaState := ms, MetaError := me);
    IF pulse AND NOT xf.bError AND
       ABS(outA[0] - 0.8601) < 0.01 AND
       ABS(outA[1] - 0.4699) < 0.01 AND
       ABS(outA[2]) < 0.001 THEN
        mask := mask + 8;                       // bit3: forward
    END_IF;
    st := 4;
// st4: backward IK to the same target, seeded near the solution
ELSIF cur = 4 THEN
    inA[0] := 0.8601; inA[1] := 0.4699; inA[2] := 0.0;
    inA[3] := 0.3; inA[4] := 0.4;
    xb(bExecute := pulse, bForward := FALSE, oidTrafo := 0,
       AxesPosIn := inA, AxesPosOut := outA,
       MetaState := ms, MetaError := me);
    IF pulse AND NOT xb.bError AND me = 0 AND
       ABS(outA[0] - 0.3) < 0.05 AND
       ABS(outA[1] - 0.4) < 0.05 THEN
        mask := mask + 16;                      // bit4: inverse IK
    END_IF;
    st := 5;
// st5: CalcMultiTrafo forward on serial6 UR5, q={0.1..0.6}
ELSIF cur = 5 THEN
    inM[0] := 0.1; inM[1] := 0.2; inM[2] := 0.3;
    inM[3] := 0.4; inM[4] := 0.5; inM[5] := 0.6;
    mf(bExecute := pulse, bForward := TRUE, oidTrafo := 1,
       nSizeIn := 6, nSizeOut := 16,
       InData := inM, OutData := outM);
    IF pulse AND NOT mf.bError THEN
        tgt[0] := outM[0]; tgt[1] := outM[1]; tgt[2] := outM[2];
        mask := mask + 32;                      // bit5: multi fwd
    END_IF;
    st := 6;
// st6: CalcMultiTrafo backward IK on the serial6 target
ELSIF cur = 6 THEN
    inM2[0] := tgt[0]; inM2[1] := tgt[1]; inM2[2] := tgt[2];
    inM2[3] := 0.1; inM2[4] := 0.2; inM2[5] := 0.3;
    mb(bExecute := pulse, bForward := FALSE, oidTrafo := 1,
       nSizeIn := 7, nSizeOut := 16,
       InData := inM2, OutData := outM2);
    IF pulse AND NOT mb.bError THEN
        FOR i := 0 TO 5 DO jnt[i] := outM2[i]; END_FOR;
        mask := mask + 64;                      // bit6: multi inv
    END_IF;
    st := 7;
// st7: forward again with the IK joints; pose must match target
ELSIF cur = 7 THEN
    FOR i := 0 TO 5 DO inM[i] := jnt[i]; END_FOR;
    mf2(bExecute := pulse, bForward := TRUE, oidTrafo := 1,
        nSizeIn := 6, nSizeOut := 16,
        InData := inM, OutData := outM2);
    IF pulse AND NOT mf2.bError AND
       ABS(outM2[0] - tgt[0]) < 0.001 AND
       ABS(outM2[1] - tgt[1]) < 0.001 AND
       ABS(outM2[2] - tgt[2]) < 0.001 THEN
        mask := mask + 128;                     // bit7: ik roundtrip
    END_IF;
    st := 8;
// st8: preset rotation A = {0.1,0.2,0.3}
ELSIF cur = 8 THEN
    rotP(bExecute := pulse, oidTrafo := 0, Rotation := rotArr);
    st := 9;
// st9: extended range saves A (edge), then preset B overwrites
ELSIF cur = 9 THEN
    extR(bExecute := TRUE, oidTrafo := 0);
    rotArr[0] := 0.9; rotArr[1] := 0.8; rotArr[2] := 0.7;
    rotP(bExecute := pulse, oidTrafo := 0, Rotation := rotArr);
    st := 10;
// st10: hold extended (no edge yet)
ELSIF cur = 10 THEN
    extR(bExecute := TRUE, oidTrafo := 0);
    st := 11;
// st11: release -> restore A
ELSIF cur = 11 THEN
    extR(bExecute := FALSE, oidTrafo := 0);
    st := 12;
// st12: rotation state must be A again
ELSIF cur = 12 THEN
    cfgA.OBJ := 0; cfgA.OP := 7; cfgA.ROP := 0; cfgA();
    IF ABS(cfgA.ROT[0] - 0.1) < 0.0001 AND
       ABS(cfgA.ROT[1] - 0.2) < 0.0001 AND
       ABS(cfgA.ROT[2] - 0.3) < 0.0001 THEN
        mask := mask + 256;                     // bit8: rotation
    END_IF;
    st := 13;
// st13: reset group on obj0
ELSIF cur = 13 THEN
    rst(bExecute := pulse, oidTrafo := 0, nItpChannelId := 0);
    st := 14;
// st14: channel op state now idle; tolerance function checks
ELSIF cur = 14 THEN
    chk(bExecute := pulse, oidTrafo := 0, eKinStatus := kinSt);
    FOR i := 0 TO 7 DO
        posA[i] := 0.0; posB[i] := 0.0; tolA[i] := 0.0;
    END_FOR;
    posA[0] := 1.0; posA[1] := 2.0; posA[2] := 3.0;
    posB[0] := 1.0000001; posB[1] := 2.0; posB[2] := 3.0;
    tolA[0] := 0.001; tolA[1] := 0.001; tolA[2] := 0.001;
    tolOk1 := F_KinAxesInTolerance(posA, posB, tolA);
    posB[0] := 2.0;
    tolOk2 := F_KinAxesInTolerance(posA, posB, tolA);
    IF kinSt = 0 AND tolOk1 AND NOT tolOk2 THEN
        mask := mask + 512;                     // bit9: reset+tolerance
    END_IF;
    st := 15;
// st15: transform on an unconfigured object -> MetaError = 1
ELSIF cur = 15 THEN
    xf5(bExecute := pulse, bForward := TRUE, oidTrafo := 5,
        AxesPosIn := inA, AxesPosOut := outA,
        MetaState := ms, MetaError := me);
    IF pulse AND NOT xf5.bError AND me = 1 THEN
        mask := mask + 1024;                    // bit10: no-model err
    END_IF;
    st := 16;
// st16: out-of-range object id -> transport-level error
ELSIF cur = 16 THEN
    xf99(bExecute := pulse, bForward := TRUE, oidTrafo := 99,
         AxesPosIn := inA, AxesPosOut := outA,
         MetaState := ms, MetaError := me);
    IF pulse AND xf99.bError THEN
        mask := mask + 2048;                    // bit11: bad obj
    END_IF;
    st := 17;
// st17: parameter lock — unlock, reconfigure OK, lock, configure
// must be rejected with E_LOCKED (-4)
ELSIF cur = 17 THEN
    IF pulse THEN
        unl(bExecute := TRUE, oidTrafo := 0);
        cfgA.OBJ := 0; cfgA.OP := 1; cfgA.TYPEIN := 1;
        cfgA.PRESET := 0; cfgA.NCFG := 0; cfgA();
        IF unl.bError = FALSE AND cfgA.STATUS = 0 THEN
            lck(bExecute := TRUE, oidTrafo := 0);
            cfgA.OBJ := 0; cfgA.OP := 1; cfgA.TYPEIN := 3;
            cfgA.PRESET := 0; cfgA.NCFG := 0; cfgA();
            IF lck.bError = FALSE AND cfgA.STATUS = -4 THEN
                mask := mask + 4096;            // bit12: lock
            END_IF;
        END_IF;
    END_IF;
    st := 18;
ELSIF cur = 18 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
