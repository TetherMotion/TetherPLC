PROGRAM Tc3Mc3CamCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    started : BOOL;
    axM    : UDINT;
    axS    : UDINT;
    crM    : FB_MotionAxis;
    crS    : FB_MotionAxis;
    rdM    : FB_MotionAxis;
    wrM    : FB_MotionAxis;
    wrS    : FB_MotionAxis;
    pwrM   : MC_Power;
    pwrS   : MC_Power;
    phA    : MC_PhasingAbsolute;
    phR    : MC_PhasingRelative;
    gip    : MC_GearInPos;
    hph    : MC_HaltPhasing;
    mpos   : DINT;
    spos   : DINT;
    dPhA   : BOOL;
    dPhR   : BOOL;
    dGip   : BOOL;
    dHph   : BOOL;
    gipOff : BOOL;
    phEx   : BOOL;
    relEx  : BOOL;
    gipEx  : BOOL;
    hphEx  : BOOL;
END_VAR

pulse := FALSE;
IF NOT started THEN
    pulse := TRUE; started := TRUE;
ELSIF st <> prevSt THEN
    pulse := TRUE;
END_IF;
prevSt := st;

// per-scan position reads (diagnostic only; FBs do their own)
IF axM <> 0 THEN
    rdM.AXIS := axM; rdM.OP := 4; rdM.INDEX := 16#6064; rdM();
    mpos := rdM.VALUE;
END_IF;
IF axS <> 0 THEN
    rdM.AXIS := axS; rdM.OP := 4; rdM.INDEX := 16#6064; rdM();
    spos := rdM.VALUE;
END_IF;

// Execute is edge-triggered inside each FB; level-held here
phEx  := st = 2;
relEx := st = 4;
gipEx := st = 5;
hphEx := st = 6;

phA(Execute := phEx, PhaseShift := 5000, Velocity := 50000,
    Acceleration := 0, Deceleration := 0, Jerk := 0,
    Master := axM, Slave := axS);
phR(Execute := relEx, PhaseShift := 2000, Velocity := 50000,
    Acceleration := 0, Deceleration := 0, Jerk := 0,
    Master := axM, Slave := axS);
gip(Execute := gipEx, RatioNumerator := 1, RatioDenominator := 1,
    Velocity := 50000, Acceleration := 0, Deceleration := 0,
    Jerk := 0, Master := axM, Slave := axS);
hph(Execute := hphEx, Deceleration := 0, Jerk := 0,
    Master := axM, Slave := axS);

IF phA.Done THEN dPhA := TRUE; END_IF;
IF phR.Done THEN dPhR := TRUE; END_IF;
IF gip.InSync THEN dGip := TRUE; END_IF;
IF hph.Done THEN dHph := TRUE; END_IF;

IF st = 0 THEN
    crM.UNITS_PER_REV := 1000; crM.MAX_VEL := 100000;
    crM.MAX_ACCEL := 200000; crM.MAX_DECEL := 200000;
    crM.POS_MIN := -100000; crM.POS_MAX := 100000;
    crM.HOME_OFFSET := 0; crM.FOLLOW_WINDOW := 3; crM.FLAGS := 1;
    crM.OP := 1; crM();
    IF crM.AXIS <> 0 THEN axM := crM.AXIS; END_IF;
    crS.UNITS_PER_REV := 1000; crS.MAX_VEL := 100000;
    crS.MAX_ACCEL := 200000; crS.MAX_DECEL := 200000;
    crS.POS_MIN := -100000; crS.POS_MAX := 100000;
    crS.HOME_OFFSET := 0; crS.FOLLOW_WINDOW := 3; crS.FLAGS := 1;
    crS.OP := 1; crS();
    IF crS.AXIS <> 0 THEN axS := crS.AXIS; END_IF;
    IF axM <> 0 AND axS <> 0 THEN mask := mask + 1; END_IF; // bit0
    st := 1;
ELSIF st = 1 THEN
    pwrM(Enable := TRUE, Enable_Positive := TRUE,
         Enable_Negative := TRUE, Axis := axM);
    pwrS(Enable := TRUE, Enable_Positive := TRUE,
         Enable_Negative := TRUE, Axis := axS);
    IF pwrM.Status AND pwrS.Status THEN
        mask := mask + 2; st := 2;                     // bit1
    END_IF;
ELSIF st = 2 THEN
    // master: PV mode, constant velocity 20000 u/s; phA edge fires
    wrM.AXIS := axM; wrM.OP := 5; wrM.INDEX := 16#6060;
    wrM.VALUE := 3; wrM();
    wrM.OP := 5; wrM.INDEX := 16#60FF; wrM.VALUE := 20000; wrM();
    st := 3;
ELSIF st = 3 THEN
    // absolute phasing: slave must reach masterPos + 5000
    IF dPhA THEN
        mask := mask + 4; st := 4;                     // bit2
    END_IF;
ELSIF st = 4 THEN
    // relative phasing: +2000 additional phase on top of offset0
    IF dPhR THEN
        mask := mask + 8; st := 5;                     // bit3
    END_IF;
ELSIF st = 5 THEN
    // offset the slave +2000 once; gear-in-position must pull it
    // back onto the anchored trajectory
    IF NOT gipOff THEN
        gipOff := TRUE;
        wrS.AXIS := axS; wrS.OP := 5; wrS.INDEX := 16#6064;
        wrS.VALUE := spos + 2000; wrS();
    END_IF;
    IF dGip THEN
        mask := mask + 16; st := 6;                    // bit4
    END_IF;
ELSIF st = 6 THEN
    // halt phasing: slave ramps to rest
    IF dHph THEN
        mask := mask + 32; st := 7;                    // bit5
    END_IF;
ELSIF st = 7 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
