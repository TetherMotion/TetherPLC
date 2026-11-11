PROGRAM Tc2Mc2GearCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    ax     : FB_MotionAxis;
    axM    : UDINT;
    axS    : UDINT;
    pwrM   : MC_Power;
    pwrS   : MC_Power;
    mvV    : MC_MoveVelocity;
    tp     : MC_TouchProbe;
    gi     : MC_GearIn;
    go     : MC_GearOut;
    hlt    : MC_Halt;
    rposM  : MC_ReadActualPosition;
    rvelM  : MC_ReadActualVelocity;
    rvelS  : MC_ReadActualVelocity;
    enSig  : BOOL;
    mvVEx  : BOOL;
    tpEx   : BOOL;
    tpTrig : BOOL;
    giEx   : BOOL;
    goEx   : BOOL;
    hltEx  : BOOL;
END_VAR

// Create the two simulated axes once (master + slave).
IF st = 0 THEN
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 2000;
    ax.MAX_ACCEL := 8000;
    ax.MAX_DECEL := 8000;
    ax.POS_MIN := -200000;
    ax.POS_MAX := 200000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 3;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axM := ax.AXIS; END_IF;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axS := ax.AXIS; END_IF;
    IF axM <> 0 AND axS <> 0 THEN
        mask := mask + 1;                       // bit0: two axes
    END_IF;
    st := 1;
    prevSt := 0;
END_IF;

IF st <> prevSt THEN pulse := TRUE; ELSE pulse := FALSE; END_IF;
prevSt := st;

enSig  := (st >= 1) AND (st <= 7);    // both powered through st=7
mvVEx  := pulse AND (st = 2);
tpEx   := (st >= 3) AND (st <= 4);    // probe armed while moving
tpTrig := rposM.Position > 500;       // trigger as master crosses 500
giEx   := (st = 5);                   // gear held during st=5
goEx   := pulse AND (st = 6);
hltEx  := pulse AND (st = 7);

pwrM(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
     Axis := axM);
pwrS(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
     Axis := axS);
mvV(Execute := mvVEx, ContinuousUpdate := FALSE, Velocity := 400,
    Acceleration := 8000, Deceleration := 8000, Axis := axM);
tp(Execute := tpEx, TriggerInput := tpTrig, WindowOnly := FALSE,
   FirstPosition := 0, LastPosition := 0, Axis := axM);
gi(Execute := giEx, RatioNumerator := 2, RatioDenominator := 1,
   Acceleration := 8000, Deceleration := 8000,
   Master := axM, Slave := axS);
go(Execute := goEx, Deceleration := 8000, Slave := axS);
hlt(Execute := hltEx, Deceleration := 8000, Axis := axM);
rposM(Enable := TRUE, Axis := axM);
rvelM(Enable := TRUE, Axis := axM);
rvelS(Enable := TRUE, Axis := axS);

IF st = 1 THEN
    IF pwrM.Status AND pwrS.Status THEN
        mask := mask + 2;                       // bit1: both powered
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    // master axis into velocity mode at 400
    IF mvV.InVelocity THEN
        mask := mask + 4;                       // bit2: master vel
        st := 3;
    END_IF;
ELSIF st = 3 THEN
    // MC_TouchProbe captures the position as the master crosses 500
    IF tp.Done THEN
        mask := mask + 8;                       // bit3: probe fired
        st := 4;
    END_IF;
ELSIF st = 4 THEN
    // recorded position is within one scan of the 500 trigger point
    IF tp.RecordedPosition >= 500 AND tp.RecordedPosition <= 520 THEN
        mask := mask + 16;                      // bit4: latch in window
    END_IF;
    st := 5;
ELSIF st = 5 THEN
    // MC_GearIn ratio 2/1: slave velocity tracks master at 2x = 800
    IF gi.InGear AND rvelS.Valid AND
       (rvelS.Velocity >= 780) AND (rvelS.Velocity <= 820) THEN
        mask := mask + 32;                      // bit5: geared at 800
        st := 6;
    END_IF;
ELSIF st = 6 THEN
    // gear released; MC_GearOut ramps the slave to rest
    IF go.Done THEN
        mask := mask + 64;                      // bit6: slave stopped
        st := 7;
    END_IF;
ELSIF st = 7 THEN
    // MC_Halt ramps the master to rest
    IF hlt.Done THEN
        mask := mask + 128;                     // bit7: master stopped
        st := 8;
    END_IF;
ELSIF st = 8 THEN
    // both axes powered off
    IF NOT pwrM.Status AND NOT pwrS.Status THEN
        mask := mask + 256;                     // bit8: powered off
        st := 9;
    END_IF;
END_IF;

END_PROGRAM
