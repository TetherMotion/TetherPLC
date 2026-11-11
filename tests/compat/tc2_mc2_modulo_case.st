PROGRAM Tc2Mc2ModuloCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    ovrCnt : INT;
    ax     : FB_MotionAxis;
    axH    : UDINT;
    pwr    : MC_Power;
    mvMod  : MC_MoveModulo;
    mvV    : MC_MoveVelocity;
    hlt    : MC_Halt;
    ovr    : MC_SetOverride;
    rpos   : MC_ReadActualPosition;
    rvel   : MC_ReadActualVelocity;
    rstat  : MC_ReadStatus;
    rp     : MC_ReadParameter;
    rbp    : MC_ReadBoolParameter;
    wp     : MC_WriteParameter;
    wbp    : MC_WriteBoolParameter;
    enSig  : BOOL;
    mvModEx: BOOL;
    mvVEx  : BOOL;
    hltEx  : BOOL;
    wpEx   : BOOL;
    wbpEx  : BOOL;
    ovrEn  : BOOL;
    mvModP : DINT;
    mvModD : INT;
    rpNum  : UDINT;
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

enSig   := (st >= 1) AND (st <= 11);  // powered through the commands
mvModEx := pulse AND ((st = 2) OR (st = 4));
mvModP  := 270; IF st = 4 THEN mvModP := 90; END_IF;
mvModD  := 0;   IF st = 4 THEN mvModD := 1; END_IF;
mvVEx   := pulse AND (st = 9);
hltEx   := pulse AND (st = 11);
wpEx    := pulse AND (st = 6);
wbpEx   := pulse AND (st = 8);
ovrEn   := (st = 10);
rpNum   := 16#6065; IF st = 8 THEN rpNum := 16#607C; END_IF;

// Drive every MC wrapper each scan. ovr is called before hlt so a
// simultaneous override-restore does not clobber the halt target.
pwr(Enable := enSig, Enable_Positive := TRUE,
    Enable_Negative := TRUE, Axis := axH);
mvMod(Execute := mvModEx, Position := mvModP, Velocity := 2000,
      Acceleration := 8000, Deceleration := 8000, Direction := mvModD,
      ModuloFactor := 360, Axis := axH);
mvV(Execute := mvVEx, ContinuousUpdate := FALSE, Velocity := 400,
    Acceleration := 8000, Deceleration := 8000, Axis := axH);
ovr(Enable := ovrEn, VelFactor := 50, Axis := axH);
hlt(Execute := hltEx, Deceleration := 8000, Axis := axH);
wp(Execute := wpEx, ParameterNumber := 16#6065, Value := 30, Axis := axH);
wbp(Execute := wbpEx, ParameterNumber := 16#607C, Value := TRUE, Axis := axH);
rp(Enable := TRUE, ParameterNumber := rpNum, Axis := axH);
rbp(Enable := TRUE, ParameterNumber := 16#6041, Axis := axH);
rpos(Enable := TRUE, Axis := axH);
rvel(Enable := TRUE, Axis := axH);
rstat(Enable := TRUE, Axis := axH);

IF st = 1 THEN
    IF pwr.Status THEN
        mask := mask + 2;                       // bit1: powered
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    // MC_MoveModulo shortest to 270deg from 0 -> lands at -90
    IF mvMod.Done THEN
        mask := mask + 4;                       // bit2: modulo done
        st := 3;
    END_IF;
ELSIF st = 3 THEN
    IF rpos.Valid AND (rpos.Position >= -93) AND (rpos.Position <= -87) THEN
        mask := mask + 8;                       // bit3: pos==-90
    END_IF;
    st := 4;
ELSIF st = 4 THEN
    // MC_MoveModulo positive-direction to 90deg from -90 -> lands 90
    IF mvMod.Done THEN
        mask := mask + 16;                      // bit4: modulo done
        st := 5;
    END_IF;
ELSIF st = 5 THEN
    IF rpos.Valid AND (rpos.Position >= 87) AND (rpos.Position <= 93) THEN
        mask := mask + 32;                      // bit5: pos==90
    END_IF;
    st := 6;
ELSIF st = 6 THEN
    // MC_WriteParameter: following window 0x6065 := 30
    IF wp.Done THEN
        mask := mask + 64;                      // bit6: write done
        st := 7;
    END_IF;
ELSIF st = 7 THEN
    // MC_ReadParameter(0x6065)==30 and MC_ReadBoolParameter(0x6041)
    IF rp.Valid AND rp.Value = 30 AND rbp.Valid AND rbp.Value THEN
        mask := mask + 128;                     // bit7: param readback
        st := 8;
    END_IF;
ELSIF st = 8 THEN
    // MC_WriteBoolParameter: home offset 0x607C := 1, read back ==1
    IF wbp.Done AND rp.Valid AND rp.Value = 1 THEN
        mask := mask + 256;                     // bit8: bool param rt
        st := 9;
    END_IF;
ELSIF st = 9 THEN
    // MC_MoveVelocity ramps to 400 and holds
    IF mvV.InVelocity THEN
        st := 10;
        ovrCnt := 0;
    END_IF;
ELSIF st = 10 THEN
    // MC_SetOverride 50% halves the velocity target to ~200; the
    // axis should report ContinuousMotion (PLCopen state 4)
    ovrCnt := ovrCnt + 1;
    IF ovrCnt > 40 THEN
        IF rvel.Valid AND (rvel.Velocity >= 190) AND (rvel.Velocity <= 210)
           AND rstat.OperatingState = 4 THEN
            mask := mask + 512;                 // bit9: override+state
        END_IF;
        st := 11;
    END_IF;
ELSIF st = 11 THEN
    // override releases, MC_Halt ramps to rest and stays enabled
    IF hlt.Done AND rstat.OperatingState = 1 THEN
        mask := mask + 1024;                    // bit10: halt+standstill
        st := 12;
    END_IF;
ELSIF st = 12 THEN
    // MC_Power disable: wait for Status to drop
    IF NOT pwr.Status THEN
        mask := mask + 2048;                    // bit11: powered off
        st := 13;
    END_IF;
END_IF;

END_PROGRAM
