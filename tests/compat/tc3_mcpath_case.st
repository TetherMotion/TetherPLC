// tc3_mcpath_case.st — Tc3_McCoordinatedMotion path e2e: two real
// axes grouped, a 3-segment path (linear -> dwell -> linear)
// prepared through MC_MoveLinearAbsolutePreparation /
// MC_DwellTimePreparation, executed by MC_MovePath with per-scan
// 0x607A CSP streaming, final-position verification.
// Mask 0x3f (6 checks).
PROGRAM Tc3McPathCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    started : BOOL;
    grp    : UDINT := 1;
    pathH  : UDINT := 0;
    axA    : UDINT;
    axB    : UDINT;
    crA    : FB_MotionAxis;
    crB    : FB_MotionAxis;
    rdA    : FB_MotionAxis;
    rdB    : FB_MotionAxis;
    pwrA   : MC_Power;
    pwrB   : MC_Power;
    addA   : MC_AddAxisToGroup;
    addB   : MC_AddAxisToGroup;
    prep1  : MC_MoveLinearAbsolutePreparation;
    prepD  : MC_DwellTimePreparation;
    prepB  : MC_MoveBlendingPreparation;
    prepK  : MC_BlockerPreparation;
    relBlk : MC_ReleaseBlocker;
    blkSt  : MC_GroupReadBlockerStatus;
    trajRd : MC_GroupReadTrajectoryInformation;
    trafoPrep : MC_SetCoordinateTransformPreparation;
    setTrafo : MC_SetCoordinateTransform;
    mgRd   : FB_McGroup;
    prepC  : MC_MoveCircularAbsolutePreparation;
    prep2  : MC_MoveDirectPreparation;
    mvPath : MC_MovePath;
    pos1   : ARRAY[0..15] OF LREAL;
    pos2   : ARRAY[0..15] OF LREAL;
    auxC   : ARRAY[0..15] OF LREAL;
    endC   : ARRAY[0..15] OF LREAL;
    transP : ARRAY[0..15] OF LREAL;
    dAddA  : BOOL;
    dAddB  : BOOL;
    dPrep1 : BOOL;
    dPrepD : BOOL;
    dPrep2 : BOOL;
    dPath  : BOOL;
    busySeen : BOOL;
    gotBusy : BOOL;
    gotPos  : BOOL;
    sawBlocked : BOOL;
    sawTraj : BOOL;
    coordNow : UDINT;
    mCnt   : INT;
    dRel   : BOOL;
    pA     : DINT;
    pB     : DINT;
END_VAR

pulse := FALSE;
IF NOT started THEN
    pulse := TRUE; started := TRUE;
ELSIF st <> prevSt THEN
    pulse := TRUE;
END_IF;
prevSt := st;

// per-scan position reads
IF axA <> 0 THEN
    rdA.AXIS := axA; rdA.OP := 4; rdA.INDEX := 16#6064; rdA(); pA := rdA.VALUE;
END_IF;
IF axB <> 0 THEN
    rdB.AXIS := axB; rdB.OP := 4; rdB.INDEX := 16#6064; rdB(); pB := rdB.VALUE;
END_IF;

mvPath(Execute := st = 8, AxesGroup := grp, PathData := pathH);
IF mvPath.Busy THEN busySeen := TRUE; END_IF;
IF mvPath.Done THEN dPath := TRUE; END_IF;

IF st = 0 THEN
    crA.UNITS_PER_REV := 1000; crA.MAX_VEL := 100000;
    crA.MAX_ACCEL := 200000; crA.MAX_DECEL := 200000;
    crA.POS_MIN := -100000; crA.POS_MAX := 100000;
    crA.HOME_OFFSET := 0; crA.FOLLOW_WINDOW := 3; crA.FLAGS := 1;
    crA.OP := 1; crA();
    IF crA.AXIS <> 0 THEN axA := crA.AXIS; END_IF;
    crB.UNITS_PER_REV := 1000; crB.MAX_VEL := 100000;
    crB.MAX_ACCEL := 200000; crB.MAX_DECEL := 200000;
    crB.POS_MIN := -100000; crB.POS_MAX := 100000;
    crB.HOME_OFFSET := 0; crB.FOLLOW_WINDOW := 3; crB.FLAGS := 1;
    crB.OP := 1; crB();
    IF crB.AXIS <> 0 THEN axB := crB.AXIS; END_IF;
    IF axA <> 0 AND axB <> 0 THEN mask := mask + 1; END_IF; // bit0
    st := 1;
ELSIF st = 1 THEN
    pwrA(Enable := TRUE, Enable_Positive := TRUE,
         Enable_Negative := TRUE, Axis := axA);
    pwrB(Enable := TRUE, Enable_Positive := TRUE,
         Enable_Negative := TRUE, Axis := axB);
    IF pwrA.Status AND pwrB.Status THEN
        mask := mask + 2; st := 2;                     // bit1
    END_IF;
ELSIF st = 2 THEN
    addA(Execute := pulse, IdentInGroup := 0,
         AxesGroup := grp, Axis := axA);
    IF addA.Done THEN dAddA := TRUE; END_IF;
    IF dAddA THEN st := 3; END_IF;
ELSIF st = 3 THEN
    addB(Execute := pulse, IdentInGroup := 0,
         AxesGroup := grp, Axis := axB);
    setTrafo(Execute := pulse, CoordTransform := 3,
             AxesGroup := grp);
    IF addB.Done THEN dAddB := TRUE; END_IF;
    IF dAddB THEN
        mask := mask + 4; st := 4;                     // bit2
    END_IF;
ELSIF st = 4 THEN
    // segment 1: linear to (1000, 0)
    pos1[0] := 1000.0; pos1[1] := 0.0;
    prep1(Execute := pulse, Position := pos1, PositionCount := 2,
          Velocity := 20000.0, Acceleration := 50000.0,
          Deceleration := 50000.0, Jerk := 0.0, BufferMode := 1,
          PathData := pathH);
    IF prep1.Error THEN mask := mask + 256; END_IF;
    IF NOT prep1.Error THEN dPrep1 := TRUE; END_IF;
    IF dPrep1 THEN st := 5; END_IF;
ELSIF st = 5 THEN
    // segment 2: circular, center (0,0), end (0,1000), CCW
    auxC[0] := 0.0; auxC[1] := 0.0;
    endC[0] := 0.0; endC[1] := 1000.0;
    prepC(Execute := pulse, CircMode := 1,
          AuxPoint := auxC, AuxPointCount := 2,
          EndPoint := endC, EndPointCount := 2,
          PathChoice := 1, Velocity := 20000.0,
          Acceleration := 50000.0, Deceleration := 50000.0,
          Jerk := 0.0, BufferMode := 1, PathData := pathH);
    IF prepC.Error THEN mask := mask + 256; END_IF;
    IF NOT prepC.Error THEN dPrepD := TRUE; END_IF;
    IF dPrepD THEN st := 6; END_IF;
ELSIF st = 6 THEN
    // segment 3: dwell 200 ms, then a 150 ms blend junction
    prepD(Execute := pulse, DwellTime := 200, BufferMode := 1,
          PathData := pathH);
    transP[0] := 150.0;
    prepB(Execute := pulse, Velocity := 20000.0,
          Acceleration := 50000.0, Deceleration := 50000.0,
          Jerk := 0.0, BufferMode := 1, TransitionMode := 0,
          TransitionParameter := transP,
          TransitionParameterCount := 1, InvokeId := 0,
          PathData := pathH);
    IF prepD.Error OR prepB.Error THEN mask := mask + 256; END_IF;
    IF NOT prepD.Error AND NOT prepB.Error THEN st := 7; END_IF;
ELSIF st = 7 THEN
    // transform marker -> coord 9, blocker gate id 7, linear
    trafoPrep(PathData := pathH, CoordTransform := 9);
    prepK(Execute := pulse, BlockerId := 7, BufferMode := 1,
          PathData := pathH);
    pos2[0] := 2000.0; pos2[1] := 2000.0;
    prep2(Execute := pulse, Position := pos2, PositionCount := 2,
          Velocity := 20000.0, Acceleration := 50000.0,
          Deceleration := 50000.0, Jerk := 0.0, BufferMode := 1,
          PathData := pathH);
    IF prepK.Error OR prep2.Error THEN mask := mask + 256; END_IF;
    IF NOT prepK.Error AND NOT prep2.Error THEN dPrep2 := TRUE; END_IF;
    IF dPrep2 THEN
        // 6 segments prepared on a live path handle
        IF pathH <> 0 THEN mask := mask + 8; END_IF;   // bit3
        st := 8;
    END_IF;
ELSIF st = 8 THEN
    // path running; blocker status polled, release after ~1.2 s
    blkSt(Enable := TRUE, AxesGroup := grp);
    IF blkSt.Blocked THEN sawBlocked := TRUE; END_IF;
    trajRd(Enable := TRUE, AxesGroup := grp);
    IF trajRd.Valid AND trajRd.EntryCount >= 3 THEN
        sawTraj := TRUE;
    END_IF;
    IF busySeen THEN gotBusy := TRUE; END_IF;
    mCnt := mCnt + 1;
    IF mCnt >= 120 THEN st := 9; END_IF;
ELSIF st = 9 THEN
    // release the blocker gate
    relBlk(Execute := pulse, BlockerId := 7, AxesGroup := grp);
    IF relBlk.Done THEN dRel := TRUE; END_IF;
    IF dRel THEN st := 10; END_IF;
ELSIF st = 10 THEN
    // path resumes and finishes
    IF dPath THEN
        // final positions near (2000, 2000), tolerance 300, and the
        // coordinate transform must have switched to 9 mid-path
        mgRd.GRP := grp; mgRd.OP := 11; mgRd();
        coordNow := mgRd.COORD;
        IF ABS(pA - 2000) < 300 AND ABS(pB - 2000) < 300 AND
           coordNow = 9 THEN
            gotPos := TRUE;
        END_IF;
        IF gotBusy THEN mask := mask + 16; END_IF;   // bit4
        IF gotPos THEN mask := mask + 32; END_IF;    // bit5
        IF sawBlocked THEN mask := mask + 64; END_IF; // bit6
        IF sawTraj THEN mask := mask + 128; END_IF;  // bit7
        st := 11;
    END_IF;
ELSIF st = 11 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
