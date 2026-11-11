PROGRAM Tc2Mc2Case
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    ax     : FB_MotionAxis;
    axH    : UDINT;
    pwr    : MC_Power;
    rst    : MC_Reset;
    sp     : MC_SetPosition;
    rpos   : MC_ReadActualPosition;
    rvel   : MC_ReadActualVelocity;
    rerr   : MC_ReadAxisError;
    rstat  : MC_ReadStatus;
    enSig  : BOOL;
    rstExec: BOOL;
    spExec : BOOL;
    spPos  : DINT;
    spMode : BOOL;
END_VAR

// Create the simulated axis once (raw FB_MotionAxis OP=1).
IF st = 0 THEN
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 1000;
    ax.MAX_ACCEL := 5000;
    ax.MAX_DECEL := 5000;
    ax.POS_MIN := -100000;
    ax.POS_MAX := 100000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 3;
    ax.FLAGS := 1;
    ax.OP := 1;
    ax();
    IF ax.AXIS <> 0 AND ax.STATUS = 0 THEN
        axH := ax.AXIS;
        mask := mask + 1;
    END_IF;
    st := 1;
END_IF;

// Command/select signals derived from st (each command state is a
// single scan, giving each Execute FB exactly one rising edge).
enSig   := (st >= 1) AND (st <= 7);   // powered through the reset
rstExec := (st = 6);
spExec  := (st = 2) OR (st = 4);
spPos   := 1000;
IF st = 4 THEN spPos := 500; END_IF;
spMode  := (st = 4);

// Drive every MC wrapper each scan.
pwr(Enable := enSig, Enable_Positive := TRUE,
    Enable_Negative := TRUE, Axis := axH);
rst(Execute := rstExec, Axis := axH);
sp(Execute := spExec, Position := spPos, Mode := spMode, Axis := axH);
rpos(Enable := TRUE, Axis := axH);
rvel(Enable := TRUE, Axis := axH);
rerr(Enable := TRUE, Axis := axH);
rstat(Enable := TRUE, Axis := axH);

IF st = 1 THEN
    // MC_Power enable: wait for Status (OperationEnabled, ~2 scans)
    IF pwr.Status THEN
        mask := mask + 2;
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    // absolute MC_SetPosition to 1000 fired this scan
    IF sp.Done THEN mask := mask + 4; END_IF;
    st := 3;
ELSIF st = 3 THEN
    // verify absolute position readback == 1000
    IF rpos.Valid AND rpos.Position = 1000 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 THEN
    // relative MC_SetPosition +500 -> 1500 fired this scan
    IF sp.Done THEN mask := mask + 16; END_IF;
    st := 5;
ELSIF st = 5 THEN
    // verify relative readback == 1500 and the read FBs are valid
    IF rpos.Valid AND rpos.Position = 1500 THEN
        mask := mask + 32;
    END_IF;
    IF rvel.Valid AND rerr.Valid AND rstat.Valid THEN
        mask := mask + 64;
    END_IF;
    st := 6;
ELSIF st = 6 THEN
    // MC_Reset Execute edge -> Done (axis stays enabled)
    IF rst.Done THEN mask := mask + 128; END_IF;
    st := 7;
ELSIF st = 7 THEN
    // reset release scan; keep waiting for power to drop next
    st := 8;
ELSIF st = 8 THEN
    // enSig now FALSE: MC_Power disabled the axis; wait for
    // Status=FALSE and OperatingState=Disabled (0)
    IF NOT pwr.Status THEN
        mask := mask + 256;
        st := 9;
    END_IF;
ELSIF st = 9 THEN
    IF rstat.OperatingState = 0 THEN
        mask := mask + 512;
        st := 10;
    END_IF;
END_IF;

END_PROGRAM
