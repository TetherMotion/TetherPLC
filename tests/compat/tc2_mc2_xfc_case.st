// e2e for compat/tc2/tc2_mc2_xfc.st — XFC kinematics, cams,
// digital cam switches and touch probe over a simulated axis.
// Mask bits (expected 0xFFFF):
//  0 axis created   1 powered        2 pos preset     3 vel reached
//  4 posAtTime#1    5 posAtTime#2    6 timeOfPos      7 timeToPos
//  8 timeToModulo   9 extendDcTime  10 curDcTime     11 camPos early-off
// 12 cams crossed  13 timeCam fired+cleared
// 14 breakcam+dcs state  15 touchprobe done+position
PROGRAM Tc2Mc2XfcCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;

    ax     : FB_MotionAxis;
    rd     : FB_MotionAxis;
    axM    : UDINT;

    pwr    : MC_Power;
    sp     : MC_SetPosition;
    mv     : MC_MoveVelocity;
    stp    : MC_Stop;

    curTm  : XFC_GetCurDcTaskTime;
    posAt  : XFC_PositionAtTime;
    posAtB : XFC_PositionAtTime;
    timOf  : XFC_TimeOfPosition;
    timTo  : XFC_TimeToPosition;
    timMod : XFC_TimeToModuloPosition;
    timMod2: XFC_TimeToModuloPosition;
    extDc  : XFC_ExtendDcTime;
    camPos : XFC_PositionCam;
    camTim : XFC_TimeCam;
    camBrk : XFC_BreakCam;
    dcs    : MC_DigitalCamSwitch;
    dcsMe  : MC_DigitalCamSwitch_MultiEdge;
    prb    : XFC_TouchProbe;

    enSig  : BOOL;
    mvEx   : BOOL;
    spEx   : BOOL;
    stpEx  : BOOL;
    prbEx  : BOOL;
    prbTrig: BOOL;

    posL   : LREAL;
    velL   : LREAL;
    basePos: LREAL;
    camOn  : LREAL;
    camOff : LREAL;
    t0     : UDINT;
    dDt    : DINT;
    exp    : LREAL;
    posAtDcA : UDINT;
    posAtDcB : UDINT;
    timOfPos : LREAL;
    timToPos : LREAL;
    timModPos  : LREAL;
    timModPos2 : LREAL;

    camOnSeen  : BOOL;
    dcsOnSeen  : BOOL;
    dcsMeOnSeen: BOOL;
    tcamOn     : BOOL;
    prbDone    : BOOL;
    prbRec     : LREAL;
END_VAR

// ---- per-scan FB calls ----------------------------------------------
IF st <> prevSt THEN pulse := TRUE; ELSE pulse := FALSE; END_IF;
prevSt := st;

IF axM <> 0 THEN
    rd.AXIS := axM; rd.OP := 6; rd();
END_IF;
posL := DINT_TO_LREAL(rd.POS_ACTUAL);
velL := DINT_TO_LREAL(rd.VEL_ACTUAL);

curTm();
// kinematic FBs run every scan so their internal clocks share the
// program-start epoch; query inputs are preset one stage ahead
posAt(DcTime := posAtDcA, Axis := axM);
posAtB(DcTime := posAtDcB, Axis := axM);
timOf(Position := timOfPos, Axis := axM);
timTo(Position := timToPos, Axis := axM);
timMod(Position := timModPos, ModuloFactor := 4000.0, Axis := axM);
timMod2(Position := timModPos2, ModuloFactor := 4000.0, Axis := axM);

camPos(FirstOnPosition := camOn, LastOnPosition := camOff,
       Modulo := FALSE, ModuloFactor := 0.0, Axis := axM);
camTim(FirstOnPosition := camOn + 100.0, Duration := 0.02,
       Modulo := FALSE, ModuloFactor := 0.0, Axis := axM);
camBrk(BreakRelease := TRUE, Axis := axM);
dcs(Enable := TRUE, FirstOnPosition := camOn, LastOnPosition := camOff,
    Modulo := FALSE, ModuloFactor := 0.0, Axis := axM);
dcsMe(Enable := TRUE, FirstOnPosition := camOn, LastOnPosition := camOff,
      Modulo := FALSE, ModuloFactor := 0.0, Axis := axM);

IF camPos.Output THEN camOnSeen := TRUE; END_IF;
IF dcs.Output THEN dcsOnSeen := TRUE; END_IF;
IF dcsMe.Output THEN dcsMeOnSeen := TRUE; END_IF;
IF camTim.Output THEN tcamOn := TRUE; END_IF;
IF prb.Done THEN
    prbDone := TRUE;
    prbRec := prb.RecordedPosition;
END_IF;

// ---- stage inputs ----------------------------------------------------
enSig   := (st >= 1) AND (st <= 30);
mvEx    := pulse AND (st = 5);
spEx    := pulse AND (st = 2);
stpEx   := pulse AND (st = 12);
prbEx   := (st >= 15);
prbTrig := (st >= 17);

pwr(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
    Axis := axM);
sp(Execute := spEx, Position := 0, Mode := FALSE, Axis := axM);
mv(Execute := mvEx, ContinuousUpdate := FALSE, Velocity := 5000,
   Acceleration := 50000, Deceleration := 50000, Axis := axM);
stp(Execute := stpEx, Deceleration := 50000, Axis := axM);
prb(Execute := prbEx, TriggerInput := prbTrig, Axis := axM);

// ---- stages ----------------------------------------------------------
IF st = 0 THEN
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 10000;
    ax.MAX_ACCEL := 50000;
    ax.MAX_DECEL := 50000;
    ax.POS_MIN := -2000000;
    ax.POS_MAX := 2000000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 3;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axM := ax.AXIS; END_IF;
    IF axM <> 0 THEN
        mask := mask + 1;                       // bit0
    END_IF;
    st := 1;
ELSIF st = 1 THEN
    IF pwr.Status THEN
        mask := mask + 2;                       // bit1
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    IF sp.Done THEN
        mask := mask + 4;                       // bit2
        st := 3;
    END_IF;
ELSIF st = 3 THEN
    st := 4;
ELSIF st = 4 THEN
    st := 5;
ELSIF st = 5 THEN
    st := 6;
ELSIF st = 6 THEN
    // wait for the velocity ramp (>2500 of 5000 u/s)
    IF rd.VEL_ACTUAL > 2500 THEN
        mask := mask + 8;                       // bit3
        st := 7;
    END_IF;
ELSIF st = 7 THEN
    basePos := posL;
    camOn := basePos + 200.0;
    camOff := basePos + 500.0;
    t0 := curTm.DcTime;
    posAtDcA := t0;
    posAtDcB := t0 + 300000000;
    timOfPos := posL + 500.0;
    timToPos := posL + 500.0;
    timModPos := posL + 500.0;
    timModPos2 := posL - 3500.0;
    st := 8;
ELSIF st = 8 THEN
    // kinematic FB outputs from the per-scan calls (inputs preset
    // at st7 against the shared clock epoch)
    dDt := UDINT_TO_DINT(posAtDcA - curTm.DcTime);
    exp := posL + velL * DINT_TO_LREAL(dDt) / 1000000000.0;
    IF posAt.Error = FALSE AND
       (posAt.Position - exp > -150.0) AND
       (posAt.Position - exp < 150.0) THEN
        mask := mask + 16;                      // bit4
    END_IF;
    dDt := UDINT_TO_DINT(posAtDcB - curTm.DcTime);
    exp := posL + velL * DINT_TO_LREAL(dDt) / 1000000000.0;
    IF posAtB.Error = FALSE AND
       (posAtB.Position - exp > -150.0) AND
       (posAtB.Position - exp < 150.0) THEN
        mask := mask + 32;                      // bit5
    END_IF;
    // time-of / time-to position: +500 u at ~5000 u/s ~ 1e8 ns
    dDt := UDINT_TO_DINT(timOf.DcTime - t0);
    IF timOf.Error = FALSE AND dDt > 30000000 AND dDt < 300000000 THEN
        mask := mask + 64;                      // bit6
    END_IF;
    IF timTo.Error = FALSE AND timTo.Duration > 30000000
       AND timTo.Duration < 300000000 THEN
        mask := mask + 128;                     // bit7
    END_IF;
    IF timMod.Error = FALSE AND timMod2.Error = FALSE
       AND timMod.Duration > 30000000 AND timMod.Duration < 300000000
       AND timMod2.Duration > 30000000 AND timMod2.Duration < 300000000 THEN
        mask := mask + 256;                     // bit8
    END_IF;
    extDc(DcTime32 := 501234, RefDcTime := 500000);
    IF extDc.DeltaNs = 1234 THEN
        mask := mask + 512;                     // bit9
    END_IF;
    IF curTm.DcTime <> 0 THEN
        mask := mask + 1024;                    // bit10
    END_IF;
    IF NOT camPos.Output THEN
        mask := mask + 2048;                     // bit11: cam still off
    END_IF;
    st := 9;
ELSIF st = 9 THEN
    st := 10;
ELSIF st = 10 THEN
    st := 11;
ELSIF st = 11 THEN
    // move still running; cam windows are crossed here
    st := 12;
ELSIF st = 12 THEN
    st := 13;
ELSIF st = 13 THEN
    // wait for the stop to complete
    IF rd.VEL_ACTUAL < 50 THEN
        st := 14;
    END_IF;
ELSIF st = 14 THEN
    st := 15;
ELSIF st = 15 THEN
    st := 16;
ELSIF st = 16 THEN
    st := 17;
ELSIF st = 17 THEN
    IF camOnSeen AND dcsOnSeen AND dcsMeOnSeen THEN
        mask := mask + 4096;                     // bit12
    END_IF;
    IF tcamOn AND NOT camTim.Output THEN
        mask := mask + 8192;                     // bit13
    END_IF;
    IF camBrk.Output AND dcs.InOperation
       AND dcsMe.InOperation THEN
        mask := mask + 16384;                    // bit14
    END_IF;
    st := 18;
ELSIF st = 18 THEN
    st := 19;
ELSIF st = 19 THEN
    IF prbDone AND (prbRec - posL > -200.0)
       AND (prbRec - posL < 200.0) THEN
        mask := mask + 32768;                    // bit15
    END_IF;
    st := 20;
ELSIF st = 20 THEN
    st := 21;
END_IF;

END_PROGRAM
