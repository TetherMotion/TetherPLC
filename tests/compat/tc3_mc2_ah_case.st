// e2e for compat/tc3/tc3_mc2_ah.st — Tc3_MC2_AdvancedHoming:
// step search FBs (ref pulse, absolute switch, limit switch,
// block, block-lag), detection variants, finalizing FBs and a
// flying (passive) reference over a simulated axis.
// Mask bits (expected 0xFFFF):
//  0 axis created   1 powered        2 pos preset     3 refPulse
//  4 absSwitchDet   5 absSwitch      6 limitSwitch    7 block
//  8 blockDet       9 blockLag      10 homeDirect    11 finishHome
// 12 abortHome     13 flyingSwitch  14 errBadVel    15 errBadDir
PROGRAM Tc3Mc2AhCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    started: BOOL;
    pulse  : BOOL;

    ax     : FB_MotionAxis;
    rd     : FB_MotionAxis;
    axM    : UDINT;

    pwr    : MC_Power;
    sp     : MC_SetPosition;
    stp    : MC_Stop;
    mv     : MC_MoveVelocity;

    // homing FBs under test
    refPls : MC_StepReferencePulse;
    refPls2: MC_StepReferencePulse;
    refPls3: MC_StepReferencePulse;
    absDet : MC_StepAbsoluteSwitchDetection;
    absSw  : MC_StepAbsoluteSwitch;
    limSw  : MC_StepLimitSwitch;
    blk    : MC_StepBlock;
    blkDet : MC_StepBlockDetection;
    blkLag : MC_StepBlockLagBased;
    finH   : MC_FinishHoming;
    homeDir: MC_HomeDirect;
    abortH : MC_AbortHoming;
    flySw  : MC_StepReferenceFlyingSwitch;

    // shared MC_HomingParameter (flattened) state
    pStored : BOOL;
    pNcSMin : BOOL;  pNcSMax : BOOL;
    pNcSPosDiff : BOOL;  pNcSVeloDiff : BOOL;
    pNcAMin : BOOL;  pNcAMax : BOOL;
    pNcAPosDiff : BOOL;  pNcAVeloDiff : BOOL;
    pDrvSBip : LREAL;  pDrvSPos : LREAL;  pDrvSNeg : LREAL;
    pDrvABip : LREAL;  pDrvAPos : LREAL;  pDrvANeg : LREAL;

    // simulated reference signals (position-derived levels)
    dRef   : BOOL; dAbsD : BOOL; dAbsS : BOOL; dLim  : BOOL;
    dBlk   : BOOL; dBlkD : BOOL; dBlkL : BOOL; dHomeD : BOOL;
    dFin   : BOOL; dAbort: BOOL; dFly  : BOOL;
    sigZ   : BOOL;
    sigAbs : BOOL;
    sigLim : BOOL;
    sigFly : BOOL;

    posL   : LREAL;
    enSig  : BOOL;
    mvEx   : BOOL;
    spEx   : BOOL;
    stpEx  : BOOL;
    finEx  : BOOL;
    dirEx  : BOOL;
    abortEx: BOOL;
    flyEx  : BOOL;
    errEx  : BOOL;
    errEx2 : BOOL;
    refEx  : BOOL;
    absDetEx : BOOL;
    absSwEx  : BOOL;
    limSwEx  : BOOL;
    blkEx    : BOOL;
    blkDetEx : BOOL;
    blkLagEx : BOOL;
    homeDirEx: BOOL;
    errId1 : UDINT;
    errId2 : UDINT;
END_VAR

// ---- per-scan FB calls ----------------------------------------------
IF NOT started THEN
    pulse := TRUE; started := TRUE;
ELSIF st <> prevSt THEN
    pulse := TRUE;
ELSE
    pulse := FALSE;
END_IF;
prevSt := st;

IF axM <> 0 THEN
    rd.AXIS := axM; rd.OP := 6; rd();
END_IF;
posL := DINT_TO_LREAL(rd.POS_ACTUAL);

// reference signals derived from axis position
sigZ   := posL >= 5000.0;
sigAbs := posL >= 8000.0;
sigLim := posL <= -5000.0;
sigFly := posL >= 3000.0;

// execute pulses: launch stages, waits follow on the next stage
enSig := (st >= 1);
mvEx  := pulse AND (st = 38);
spEx  := pulse AND ((st = 2) OR (st = 6) OR (st = 10) OR (st = 14)
                    OR (st = 18) OR (st = 22) OR (st = 26));
stpEx := pulse AND (st = 42);
finEx := pulse AND (st = 33);
dirEx := pulse AND (st = 31);
abortEx := pulse AND (st = 35);
flyEx := pulse AND (st = 39);
errEx := pulse AND (st = 44);
errEx2 := pulse AND (st = 46);
refEx := pulse AND (st = 4);
absDetEx := pulse AND (st = 8);
absSwEx  := pulse AND (st = 12);
limSwEx  := pulse AND (st = 16);
blkEx    := pulse AND (st = 20);
blkDetEx := pulse AND (st = 24);
blkLagEx := pulse AND (st = 28);
homeDirEx:= dirEx;

pwr(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
    Axis := axM);
sp(Execute := spEx, Position := 0, Mode := FALSE, Axis := axM);
stp(Execute := stpEx, Deceleration := 50000, Axis := axM);
mv(Execute := mvEx, ContinuousUpdate := FALSE, Velocity := 5000,
   Acceleration := 50000, Deceleration := 50000, Axis := axM);

// ---- homing FB calls (every scan; Execute is edge-driven inside) --
refPls(
       Execute := refEx,  Direction := 1,  SwitchMode := 3,
       RefLevel := sigZ,  Velocity := 20000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       SetPosition := 100.0,  TimeLimit := T#0s,
       DistanceLimit := 50000.0,  TorqueLimit := 0.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptEnableLagErrorDetection := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
absDet(
       Execute := absDetEx,  Direction := 3,  SwitchMode := 3,
       RefLevel := (posL <= -3000.0),  Velocity := 20000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       TimeLimit := T#0s,  DistanceLimit := 50000.0,
       TorqueLimit := 0.0,  PositiveLimitSwitch := FALSE,
       NegativeLimitSwitch := FALSE,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptEnableLagErrorDetection := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
absSw(
       Execute := absSwEx,  Direction := 1,  SwitchMode := 3,
       RefLevel := sigAbs,  Velocity := 20000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       SetPosition := 500.0,  TimeLimit := T#0s,
       DistanceLimit := 50000.0,  TorqueLimit := 0.0,
       PositiveLimitSwitch := FALSE,  NegativeLimitSwitch := FALSE,
       BufferMode := 0,  OptDisableDriveAccess := FALSE,
       OptEnableLagErrorDetection := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
limSw(
       Execute := limSwEx,  Direction := 3,  LimitSwitchMode := 3,
       LimitSwitchLevel := sigLim,  Velocity := 20000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       SetPosition := -500.0,  TimeLimit := T#0s,
       DistanceLimit := 50000.0,  TorqueLimit := 0.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptEnableLagErrorDetection := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
blk(
       Execute := blkEx,  Direction := 1,  Velocity := 80000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       SetPosition := 25000.0,  DetectionVelocityLimit := 800.0,
       DetectionVelocityTime := T#100ms,  TimeLimit := T#0s,
       DistanceLimit := 40000.0,  TorqueLimit := 0.0,
       TorqueTolerance := 0.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptInstantLagReduction := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
blkDet(
       Execute := blkDetEx,  Direction := 3,  Velocity := 80000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       DetectionVelocityLimit := 800.0,
       DetectionVelocityTime := T#100ms,  TimeLimit := T#0s,
       DistanceLimit := 40000.0,  TorqueLimit := 0.0,
       TorqueTolerance := 0.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptInstantLagReduction := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
blkLag(
       Execute := blkLagEx,  Direction := 1,  Velocity := 80000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       SetPosition := 20000.0,  DetectionVelocityLimit := 800.0,
       DetectionVelocityTime := T#100ms,  TimeLimit := T#0s,
       DistanceLimit := 40000.0,  TorqueLimit := 0.0,
       LagLimit := 1000.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptInstantLagReduction := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
homeDir(
       Execute := homeDirEx,  SetPosition := 777.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
finH(
       Execute := finEx,  Distance := 500.0,  Velocity := 1000.0,
       Acceleration := 50000.0,  Deceleration := 50000.0,  Jerk := 0.0,
       BufferMode := 0,  OptDisableDriveAccess := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
abortH(
       Execute := abortEx,  OptDisableDriveAccess := FALSE,
       Axis := axM,  P_Stored := pStored,  P_NcSMin := pNcSMin,
       P_NcSMax := pNcSMax,  P_NcSPosDiff := pNcSPosDiff,
       P_NcSVeloDiff := pNcSVeloDiff,  P_NcAMin := pNcAMin,
       P_NcAMax := pNcAMax,  P_NcAPosDiff := pNcAPosDiff,
       P_NcAVeloDiff := pNcAVeloDiff,  P_DrvSBip := pDrvSBip,
       P_DrvSPos := pDrvSPos,  P_DrvSNeg := pDrvSNeg,
       P_DrvABip := pDrvABip,  P_DrvAPos := pDrvAPos,
       P_DrvANeg := pDrvANeg);
flySw(
       Execute := flyEx,  SwitchMode := 3,  RefLevel := sigFly,
       SetPosition := 4242.0,  TimeLimit := T#0s,
       DistanceLimit := 50000.0,  BufferMode := 0,  Axis := axM);
refPls2(
       Execute := errEx,  Direction := 1,  SwitchMode := 3,
       RefLevel := FALSE,  Velocity := 0.0,  Acceleration := 50000.0,
       Deceleration := 50000.0,  Jerk := 0.0,  SetPosition := 0.0,
       TimeLimit := T#0s,  DistanceLimit := 50000.0,
       TorqueLimit := 0.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptEnableLagErrorDetection := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
refPls3(
       Execute := errEx2,  Direction := 9,  SwitchMode := 3,
       RefLevel := FALSE,  Velocity := 100.0,  Acceleration := 50000.0,
       Deceleration := 50000.0,  Jerk := 0.0,  SetPosition := 0.0,
       TimeLimit := T#0s,  DistanceLimit := 50000.0,
       TorqueLimit := 0.0,  BufferMode := 0,
       OptDisableDriveAccess := FALSE,
       OptEnableLagErrorDetection := FALSE,  Axis := axM,
       P_Stored := pStored,  P_NcSMin := pNcSMin,  P_NcSMax := pNcSMax,
       P_NcSPosDiff := pNcSPosDiff,  P_NcSVeloDiff := pNcSVeloDiff,
       P_NcAMin := pNcAMin,  P_NcAMax := pNcAMax,
       P_NcAPosDiff := pNcAPosDiff,  P_NcAVeloDiff := pNcAVeloDiff,
       P_DrvSBip := pDrvSBip,  P_DrvSPos := pDrvSPos,
       P_DrvSNeg := pDrvSNeg,  P_DrvABip := pDrvABip,
       P_DrvAPos := pDrvAPos,  P_DrvANeg := pDrvANeg);
// done latches (Done is a single-scan pulse)
IF refPls.Done THEN dRef := TRUE; END_IF;
IF absDet.Done THEN dAbsD := TRUE; END_IF;
IF absSw.Done THEN dAbsS := TRUE; END_IF;
IF limSw.Done THEN dLim := TRUE; END_IF;
IF blk.Done THEN dBlk := TRUE; END_IF;
IF blkDet.Done THEN dBlkD := TRUE; END_IF;
IF blkLag.Done THEN dBlkL := TRUE; END_IF;
IF homeDir.Done THEN dHomeD := TRUE; END_IF;
IF finH.Done THEN dFin := TRUE; END_IF;
IF abortH.Done THEN dAbort := TRUE; END_IF;
IF flySw.Done THEN dFly := TRUE; END_IF;

// latch single-scan error outputs
IF refPls2.Error THEN errId1 := refPls2.ErrorID; END_IF;
IF refPls3.Error THEN errId2 := refPls3.ErrorID; END_IF;

// ---- stages ----------------------------------------------------------
IF st = 0 THEN
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 100000;
    ax.MAX_ACCEL := 200000;
    ax.MAX_DECEL := 200000;
    ax.POS_MIN := -20000;
    ax.POS_MAX := 30000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 3;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axM := ax.AXIS; END_IF;
    IF axM <> 0 THEN
        mask := mask + 1;                          // bit0
        // pos-limit objects (sim ext): needed on the IO backend
        // where axis create does not carry the config struct
        ax.AXIS := axM;
        ax.OP := 5; ax.INDEX := 16#200A; ax.VALUE := -20000; ax();
        ax.OP := 5; ax.INDEX := 16#200B; ax.VALUE := 30000;  ax();
        ax.OP := 5; ax.INDEX := 16#200C; ax.VALUE := 1;      ax();
    END_IF;
    st := 1;
ELSIF st = 1 THEN
    IF pwr.Status THEN mask := mask + 2; st := 2; END_IF;
ELSIF st = 2 THEN
    IF sp.Done THEN mask := mask + 4; st := 3; END_IF;
ELSIF st = 3 THEN st := 4;
ELSIF st = 4 THEN st := 5;
ELSIF st = 5 THEN
    IF dRef AND ABS(posL - 100.0) < 5.0 AND pStored = TRUE THEN
        mask := mask + 8; st := 6;
    END_IF;
ELSIF st = 6 THEN
    IF sp.Done THEN st := 7; END_IF;
ELSIF st = 7 THEN st := 8;
ELSIF st = 8 THEN st := 9;
ELSIF st = 9 THEN
    IF dAbsD AND absDet.RecordedPosition <= -2700.0
       AND absDet.RecordedPosition >= -3300.0 THEN
        mask := mask + 16; st := 10;
    END_IF;
ELSIF st = 10 THEN
    IF sp.Done THEN st := 11; END_IF;
ELSIF st = 11 THEN st := 12;
ELSIF st = 12 THEN st := 13;
ELSIF st = 13 THEN
    IF dAbsS AND ABS(posL - 500.0) < 5.0 THEN
        mask := mask + 32; st := 14;
    END_IF;
ELSIF st = 14 THEN
    IF sp.Done THEN st := 15; END_IF;
ELSIF st = 15 THEN st := 16;
ELSIF st = 16 THEN st := 17;
ELSIF st = 17 THEN
    IF dLim AND ABS(posL - (-500.0)) < 5.0 THEN
        mask := mask + 64; st := 18;
    END_IF;
ELSIF st = 18 THEN
    IF sp.Done THEN st := 19; END_IF;
ELSIF st = 19 THEN st := 20;
ELSIF st = 20 THEN st := 21;
ELSIF st = 21 THEN
    IF dBlk AND ABS(posL - 25000.0) < 5.0 THEN
        mask := mask + 128; st := 22;
    END_IF;
ELSIF st = 22 THEN
    IF sp.Done THEN st := 23; END_IF;
ELSIF st = 23 THEN st := 24;
ELSIF st = 24 THEN st := 25;
ELSIF st = 25 THEN
    IF dBlkD AND blkDet.RecordedPosition <= -19000.0
       AND blkDet.RecordedPosition >= -21000.0 THEN
        mask := mask + 256; st := 26;
    END_IF;
ELSIF st = 26 THEN
    IF sp.Done THEN st := 27; END_IF;
ELSIF st = 27 THEN st := 28;
ELSIF st = 28 THEN st := 29;
ELSIF st = 29 THEN
    IF dBlkL AND ABS(posL - 20000.0) < 5.0 THEN
        mask := mask + 512; st := 30;
    END_IF;
ELSIF st = 30 THEN st := 31;
ELSIF st = 31 THEN
    IF dHomeD AND ABS(posL - 777.0) < 5.0 THEN
        mask := mask + 1024; st := 32;
    END_IF;
ELSIF st = 32 THEN st := 33;
ELSIF st = 33 THEN
    IF dFin AND ABS(posL - 1277.0) < 60.0
       AND pStored = FALSE THEN
        mask := mask + 2048; st := 34;
    END_IF;
ELSIF st = 34 THEN st := 35;
ELSIF st = 35 THEN
    IF dAbort THEN mask := mask + 4096; st := 36; END_IF;
ELSIF st = 36 THEN st := 37;
ELSIF st = 37 THEN st := 38;
ELSIF st = 38 THEN st := 39;
ELSIF st = 39 THEN
    IF dFly AND posL >= 4242.0 AND posL <= 4800.0 THEN
        mask := mask + 8192; st := 40;
    END_IF;
ELSIF st = 40 THEN st := 41;
ELSIF st = 41 THEN st := 42;
ELSIF st = 42 THEN st := 43;
ELSIF st = 43 THEN st := 44;
ELSIF st = 44 THEN
    IF errId1 = 16#4B93 THEN mask := mask + 16384; END_IF;
    st := 45;
ELSIF st = 45 THEN st := 46;
ELSIF st = 46 THEN
    IF errId2 = 16#4B91 THEN mask := mask + 32768; END_IF;
    st := 47;
END_IF;
END_PROGRAM
