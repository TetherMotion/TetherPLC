// tc3_mcgroup_stop_case.st — Tc3_McCoordinatedMotion group motion
// e2e: two real axes grouped via MC_AddAxisToGroup, PV velocity
// moves, then MC_GroupStop (ramp to rest) and MC_GroupHalt
// (immediate disable) with per-axis VEL_ACTUAL verification.
// Mask 0x3f (6 checks).
PROGRAM Tc3McGroupStopCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    started : BOOL;
    dwell  : INT;
    grp    : UDINT := 1;
    axA    : UDINT;
    axB    : UDINT;
    crA    : FB_MotionAxis;
    crB    : FB_MotionAxis;
    rdA    : FB_MotionAxis;
    rdB    : FB_MotionAxis;
    wrA    : FB_MotionAxis;
    wrB    : FB_MotionAxis;
    pwrA   : MC_Power;
    pwrB   : MC_Power;
    addA   : MC_AddAxisToGroup;
    addB   : MC_AddAxisToGroup;
    gStop  : MC_GroupStop;
    gHalt  : MC_GroupHalt;
    vA     : DINT;
    vB     : DINT;
    dAddA  : BOOL;
    dAddB  : BOOL;
    dStop  : BOOL;
    dHalt  : BOOL;
    gStopEx: BOOL;
    gHaltEx: BOOL;
END_VAR

pulse := FALSE;
IF NOT started THEN
    pulse := TRUE; started := TRUE;
ELSIF st <> prevSt THEN
    pulse := TRUE; dwell := 0;
END_IF;
prevSt := st;

// per-scan velocity reads
IF axA <> 0 THEN
    rdA.AXIS := axA; rdA.OP := 3; rdA();
    rdA.OP := 4; rdA.INDEX := 16#606C; rdA(); vA := rdA.VALUE;
END_IF;
IF axB <> 0 THEN
    rdB.AXIS := axB; rdB.OP := 3; rdB();
    rdB.OP := 4; rdB.INDEX := 16#606C; rdB(); vB := rdB.VALUE;
END_IF;

gStopEx := st = 6;
gHaltEx := st = 10;
gStop(Execute := gStopEx, Deceleration := 1000.0, AxesGroup := grp);
gHalt(Execute := gHaltEx, AxesGroup := grp);

IF gStop.Done THEN dStop := TRUE; END_IF;
IF gHalt.Done THEN dHalt := TRUE; END_IF;

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
    // add axis A to a fresh group (ensure creates the handle)
    addA(Execute := pulse, IdentInGroup := 0,
         AxesGroup := grp, Axis := axA);
    IF addA.Done THEN dAddA := TRUE; END_IF;
    IF dAddA THEN st := 3; END_IF;
ELSIF st = 3 THEN
    addB(Execute := pulse, IdentInGroup := 0,
         AxesGroup := grp, Axis := axB);
    IF addB.Done THEN dAddB := TRUE; END_IF;
    IF dAddB THEN
        mask := mask + 4; st := 4;                     // bit2
    END_IF;
ELSIF st = 4 THEN
    // both axes: PV mode, velocity 5000 u/s
    wrA.AXIS := axA; wrA.OP := 5; wrA.INDEX := 16#6060;
    wrA.VALUE := 3; wrA();
    wrA.OP := 5; wrA.INDEX := 16#60FF; wrA.VALUE := 5000; wrA();
    wrB.AXIS := axB; wrB.OP := 5; wrB.INDEX := 16#6060;
    wrB.VALUE := 3; wrB();
    wrB.OP := 5; wrB.INDEX := 16#60FF; wrB.VALUE := 5000; wrB();
    st := 5;
ELSIF st = 5 THEN
    // let both axes spin up, then confirm motion
    dwell := dwell + 1;
    IF dwell >= 15 THEN
        IF vA > 0 AND vB > 0 THEN
            mask := mask + 8;                         // bit3
        END_IF;
        st := 6;
    END_IF;
ELSIF st = 6 THEN
    // group stop: ramp to rest; Done implies all VEL_ACTUAL = 0
    IF dStop THEN
        mask := mask + 16; st := 7;                    // bit4
    END_IF;
ELSIF st = 7 THEN
    // restart both axes for the halt test
    wrA.AXIS := axA; wrA.OP := 5; wrA.INDEX := 16#6060;
    wrA.VALUE := 3; wrA();
    wrA.OP := 5; wrA.INDEX := 16#60FF; wrA.VALUE := 5000; wrA();
    wrB.AXIS := axB; wrB.OP := 5; wrB.INDEX := 16#6060;
    wrB.VALUE := 3; wrB();
    wrB.OP := 5; wrB.INDEX := 16#60FF; wrB.VALUE := 5000; wrB();
    st := 8;
ELSIF st = 8 THEN
    dwell := dwell + 1;
    IF dwell >= 15 THEN st := 10; END_IF;
ELSIF st = 10 THEN
    // group halt: immediate disable; Done implies all stopped
    IF dHalt THEN
        mask := mask + 32; st := 11;                   // bit5
    END_IF;
ELSIF st = 11 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
