PROGRAM Tc2Mc2MotionCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    jogCnt : INT;
    ax     : FB_MotionAxis;
    axH    : UDINT;
    pwr    : MC_Power;
    mvA    : MC_MoveAbsolute;
    mvR    : MC_MoveRelative;
    mvV    : MC_MoveVelocity;
    stp    : MC_Stop;
    hlt    : MC_Halt;
    hom    : MC_Home;
    jog    : MC_Jog;
    rpos   : MC_ReadActualPosition;
    enSig  : BOOL;
    mvAEx  : BOOL;
    mvREx  : BOOL;
    mvVEx  : BOOL;
    stpEx  : BOOL;
    hltEx  : BOOL;
    homEx  : BOOL;
    jogF   : BOOL;
END_VAR

// Create the simulated axis once (raw FB_MotionAxis OP=1).
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
    ax.OP := 1;
    ax();
    IF ax.AXIS <> 0 AND ax.STATUS = 0 THEN
        axH := ax.AXIS;
        mask := mask + 1;                       // bit0: axis created
    END_IF;
    st := 1;
    prevSt := 0;
END_IF;

// pulse is TRUE only on the first scan of a new phase so every
// Execute input sees exactly one rising edge per command.
IF st <> prevSt THEN pulse := TRUE; ELSE pulse := FALSE; END_IF;
prevSt := st;

// Level/edge inputs derived from the current phase.
enSig := (st >= 1) AND (st <= 11);   // powered through the commands
mvAEx := pulse AND (st = 2);
mvREx := pulse AND (st = 4);
mvVEx := pulse AND ((st = 6) OR (st = 8));
stpEx := pulse AND (st = 7);
hltEx := pulse AND (st = 9);
homEx := pulse AND (st = 10);
jogF  := (st = 11) AND (jogCnt < 60);

// Drive every MC wrapper each scan.
pwr(Enable := enSig, Enable_Positive := TRUE,
    Enable_Negative := TRUE, Axis := axH);
mvA(Execute := mvAEx, Position := 1000, Velocity := 2000,
    Acceleration := 8000, Deceleration := 8000, Axis := axH);
mvR(Execute := mvREx, Distance := 500, Velocity := 2000,
    Acceleration := 8000, Deceleration := 8000, Axis := axH);
mvV(Execute := mvVEx, ContinuousUpdate := FALSE, Velocity := 400,
    Acceleration := 8000, Deceleration := 8000, Axis := axH);
stp(Execute := stpEx, Deceleration := 8000, Axis := axH);
hlt(Execute := hltEx, Deceleration := 8000, Axis := axH);
hom(Execute := homEx, Position := 0, Velocity := 2000,
    Acceleration := 8000, Deceleration := 8000, Axis := axH);
jog(JogForward := jogF, JogBackwards := FALSE, Velocity := 400,
    Axis := axH);
rpos(Enable := TRUE, Axis := axH);

IF st = 1 THEN
    // MC_Power enable: wait for Status (OperationEnabled)
    IF pwr.Status THEN
        mask := mask + 2;                       // bit1: powered
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    // MC_MoveAbsolute to 1000
    IF mvA.Done THEN
        mask := mask + 4;                       // bit2: abs move done
        st := 3;
    END_IF;
ELSIF st = 3 THEN
    // verify absolute position near 1000 (within follow window)
    IF rpos.Valid AND (rpos.Position >= 997) AND (rpos.Position <= 1003) THEN
        mask := mask + 8;                       // bit3: pos==1000
    END_IF;
    st := 4;
ELSIF st = 4 THEN
    // MC_MoveRelative +500 -> ~1500
    IF mvR.Done THEN
        mask := mask + 16;                      // bit4: rel move done
        st := 5;
    END_IF;
ELSIF st = 5 THEN
    IF rpos.Valid AND (rpos.Position >= 1497) AND (rpos.Position <= 1503) THEN
        mask := mask + 32;                      // bit5: pos==1500
    END_IF;
    st := 6;
ELSIF st = 6 THEN
    // MC_MoveVelocity ramps to 400 and holds
    IF mvV.InVelocity THEN
        mask := mask + 64;                      // bit6: in velocity
        st := 7;
    END_IF;
ELSIF st = 7 THEN
    // MC_Stop quick-stop locks the axis at rest
    IF stp.Done THEN
        mask := mask + 128;                     // bit7: stopped
        st := 8;
    END_IF;
ELSIF st = 8 THEN
    // re-issue MC_MoveVelocity (recovers from quick stop)
    IF mvV.InVelocity THEN
        st := 9;
    END_IF;
ELSIF st = 9 THEN
    // MC_Halt ramps to rest but keeps the axis enabled
    IF hlt.Done THEN
        mask := mask + 256;                     // bit8: halted
        st := 10;
    END_IF;
ELSIF st = 10 THEN
    // MC_Home moves back to the home offset (0)
    IF hom.Done THEN
        mask := mask + 512;                     // bit9: homed
        st := 11;
        jogCnt := 0;
    END_IF;
ELSIF st = 11 THEN
    // MC_Jog forward ~60 scans then release; Done on rest
    jogCnt := jogCnt + 1;
    IF jog.Done AND rpos.Valid AND (rpos.Position > 50) THEN
        mask := mask + 1024;                    // bit10: jog done
        mask := mask + 2048;                    // bit11: jogged +pos
        st := 12;
    END_IF;
ELSIF st = 12 THEN
    // MC_Power disable: wait for Status to drop
    IF NOT pwr.Status THEN
        mask := mask + 4096;                    // bit12: powered off
        st := 13;
    END_IF;
END_IF;

END_PROGRAM
