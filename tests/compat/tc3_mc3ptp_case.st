PROGRAM Tc3Mc3PtpCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    started : BOOL;
    axM    : UDINT;
    ax     : FB_MotionAxis;
    rd     : FB_MotionAxis;
    pwr    : MC_Power;
    rst    : MC_Reset;
    tq     : MC_ReadActualTorque;
    hwr    : MC_WriteHardwareParameter;
    hrr    : MC_ReadHardwareParameter;
    wfs    : MC_WaitForStableActualPosition;
    te     : MC_TriggerError;
    hs     : MC_SetHomingState;
    lc     : MC_LoadControl;
    esg    : MC_ActivateExternalSetpointGeneration;
    axErr  : MC_ReadAxisError;
    vel    : DINT;
END_VAR

// per-scan snapshot
IF axM <> 0 THEN
    rd.AXIS := axM; rd.OP := 6; rd();
    vel := rd.VEL_ACTUAL;
END_IF;

pulse := FALSE;
IF NOT started THEN
    pulse := TRUE; started := TRUE;
ELSIF st <> prevSt THEN
    pulse := TRUE;
END_IF;
prevSt := st;

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
    IF axM <> 0 THEN mask := mask + 1; END_IF;   // bit0: create
    st := 1;
ELSIF st = 1 THEN
    pwr(Enable := TRUE, Enable_Positive := TRUE,
        Enable_Negative := TRUE, Axis := axM);
    IF pwr.Status THEN mask := mask + 2; st := 2; END_IF;  // bit1: power
ELSIF st = 2 THEN
    tq(Enable := TRUE, Axis := axM);
    hwr(Execute := pulse, ParameterNumber := 16#6065,
        Value := 5, Axis := axM);
    IF pulse AND hwr.Done AND tq.Valid THEN
        mask := mask + 4;                        // bit2: torque+hw wr
    END_IF;
    st := 3;
ELSIF st = 3 THEN
    hrr(Enable := TRUE, ParameterNumber := 16#6065, Axis := axM);
    IF hrr.Valid AND hrr.Value = 5 THEN
        mask := mask + 8;                        // bit3: hw param rd
    END_IF;
    st := 4;
ELSIF st = 4 THEN
    wfs(Execute := pulse, PositionWindow := 10,
        VelocityWindow := 1, Axis := axM);
    IF wfs.Done THEN mask := mask + 16; st := 5; END_IF;  // bit4: stable
ELSIF st = 5 THEN
    hs(Execute := pulse, HomingState := 1, Axis := axM);
    lc(Execute := pulse, Axis := axM);
    esg(Execute := pulse, Axis := axM);
    IF pulse AND hs.Done AND lc.Done AND esg.Done THEN
        mask := mask + 32;                       // bit5: no-op FBs
    END_IF;
    st := 6;
ELSIF st = 6 THEN
    te(Execute := pulse, ErrorCode := 4660, Axis := axM); // 0x1234
    IF pulse AND te.Done THEN mask := mask + 64; st := 7; END_IF; // bit6
ELSIF st = 7 THEN
    axErr(Enable := TRUE, Axis := axM);
    IF axErr.Valid AND axErr.AxisErrorID = 4660 THEN
        mask := mask + 128;                      // bit7: fault latched
    END_IF;
    rst(Execute := pulse, Axis := axM);
    IF pulse AND rst.Done THEN st := 8; END_IF;
ELSIF st = 8 THEN
    axErr(Enable := TRUE, Axis := axM);
    IF axErr.Valid AND axErr.AxisErrorID = 0 THEN
        mask := mask + 256;                      // bit8: fault cleared
    END_IF;
    st := 9;
ELSIF st = 9 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
