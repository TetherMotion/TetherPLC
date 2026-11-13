PROGRAM Tc2Mc2DriveCase
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
    hlt    : MC_Halt;
    info   : FB_ReadDriveInfo;
    coeWr  : FB_CoEWrite;
    coeRd  : FB_CoERead;
    coeCmd : FB_CoEExecuteCommand;
    setOff : FB_SetPositionOffset;
    delOff : FB_DeletePositionOffset;
    brk    : FB_BrakeControl;
    rdPar  : FB_ReadDriveParameter;
    rdPar2 : FB_ReadDriveParameter;
    ax8kBrk : FB_CoEAX8000BrakeControl;
    ax8kTst : FB_CoEAX8000BrakeTest;
    ax8kPrk : FB_CoEAX8000ParkAxis;
    elBrk  : FB_CoEEL72xxBrakeControl;
    elSet  : FB_CoEEL72xxSetPositionOffset;
    saw    : MC_ReadFlyingSawCharacteristics;
    soeRd  : FB_SoERead;
    soeWrPw: FB_SoEWritePassword;
    soeCmd : FB_SoEExecuteCommand;
    soeCmd2: FB_SoEExecuteCommand;
    soeCmdSt: FB_SoEReadCommandState;
    soeRst : FB_SoEReset;
    ax5kV  : FB_SoEAX5000ReadActMainVoltage;
    ax5kFw : FB_SoEAX5000FirmwareUpdate;
    gv     : MC_GearInVelo;
    gp     : MC_GearInPos;
    rvelS  : MC_ReadActualVelocity;
    rposS  : MC_ReadActualPosition;
    rposM  : MC_ReadActualPosition;
    enSig  : BOOL;
    mvVEx  : BOOL;
    gvEx   : BOOL;
    gpEx   : BOOL;
    hltEx  : BOOL;
    cmdErrSeen : BOOL;
    cmdDoneSeen : BOOL;
    tog     : BOOL;
END_VAR

// Create two simulated axes (master slave0, slave slave1).
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
tog := NOT tog;   // re-pulse helper for the command-state polls

enSig  := (st >= 1) AND (st <= 29);
mvVEx  := pulse AND (st = 26);       // master velocity for gear tests
gvEx   := (st >= 27) AND (st <= 28); // velocity gear held
gpEx   := (st = 29);                 // position gear one scan
hltEx  := pulse AND (st = 30);

pwrM(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
    Axis := axM);
pwrS(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
    Axis := axS);
mvV(Execute := mvVEx, ContinuousUpdate := FALSE, Velocity := 400,
    Acceleration := 8000, Deceleration := 8000, Axis := axM);
hlt(Execute := hltEx, Deceleration := 8000, Axis := axM);
gv(Execute := gvEx, RatioNumerator := 2, RatioDenominator := 1,
   Acceleration := 8000, Deceleration := 8000, Master := axM, Slave := axS);
gp(Execute := gpEx, RatioNumerator := 1, RatioDenominator := 1,
   Acceleration := 8000, Deceleration := 8000, Master := axM, Slave := axS);
rvelS(Enable := TRUE, Axis := axS);
rposS(Enable := TRUE, Axis := axS);
rposM(Enable := TRUE, Axis := axM);

IF st = 1 THEN
    IF pwrM.Status AND pwrS.Status THEN
        mask := mask + 2;                       // bit1: powered
        st := 2;
    END_IF;
ELSIF st = 2 THEN
    info(Execute := pulse, Axis := axM);
    IF info.Valid AND info.State > 0 THEN
        mask := mask + 4;                       // bit2: drive info
        st := 3;
    END_IF;
ELSIF st = 3 THEN
    coeWr(Execute := pulse, Index := 16#2003, Value := 7, Axis := axM);
    IF coeWr.Done THEN
        mask := mask + 8;                       // bit3: CoE write
        st := 4;
    END_IF;
ELSIF st = 4 THEN
    coeRd(Execute := pulse, Index := 16#2003, Axis := axM);
    IF coeRd.Done AND coeRd.Value = 7 THEN
        mask := mask + 16;                      // bit4: CoE readback
        st := 5;
    END_IF;
ELSIF st = 5 THEN
    coeCmd(Execute := pulse, Command := 42, Axis := axM);
    IF coeCmd.Done THEN
        mask := mask + 32;                      // bit5: CoE command
        st := 6;
    END_IF;
ELSIF st = 6 THEN
    setOff(Execute := pulse, Position := 5000.0, Relative := FALSE,
           Feedback := 0, Memory := 0, Axis := axM);
    IF setOff.Done THEN
        mask := mask + 64;                      // bit6: pos offset
        st := 7;
    END_IF;
ELSIF st = 7 THEN
    rdPar(Execute := pulse, ParameterNumber := 16#2002, Mode := 0,
          Interval := 0, Axis := axM);
    IF rdPar.Valid AND rdPar.Value = 5000 THEN
        mask := mask + 128;                     // bit7: offset=5000
        st := 8;
    END_IF;
ELSIF st = 8 THEN
    delOff(Execute := pulse, Feedback := 0, Memory := 0, Axis := axM);
    IF delOff.Done THEN
        mask := mask + 256;                     // bit8: offset del
        st := 9;
    END_IF;
ELSIF st = 9 THEN
    brk(Execute := pulse, Mode := 3, Axis := axM);
    IF brk.Done THEN
        mask := mask + 512;                     // bit9: brake ctrl
        st := 10;
    END_IF;
ELSIF st = 10 THEN
    ax8kBrk(Execute := pulse, Mode := 1, Axis := axM);
    IF ax8kBrk.Done THEN
        mask := mask + 1024;                    // bit10: ax8k brake
        st := 11;
    END_IF;
ELSIF st = 11 THEN
    ax8kTst(Execute := pulse, Axis := axM);
    IF ax8kTst.Succeeded THEN
        mask := mask + 2048;                    // bit11: brake test
        st := 12;
    END_IF;
ELSIF st = 12 THEN
    ax8kPrk(Execute := pulse, Mode := 1, Axis := axM);
    IF ax8kPrk.Done THEN
        mask := mask + 4096;                    // bit12: ax8k park
        st := 13;
    END_IF;
ELSIF st = 13 THEN
    rdPar2(Execute := pulse, ParameterNumber := 16#2004, Mode := 0,
           Interval := 0, Axis := axM);
    IF rdPar2.Valid AND rdPar2.Value = 1 THEN
        mask := mask + 8192;                    // bit13: park=1
        st := 14;
    END_IF;
ELSIF st = 14 THEN
    elBrk(Execute := pulse, Mode := 2, Axis := axM);
    IF elBrk.Done THEN
        mask := mask + 16384;                   // bit14: el72xx brk
        st := 15;
    END_IF;
ELSIF st = 15 THEN
    elSet(Execute := pulse, Position := 7000.0, Relative := FALSE,
          Feedback := 0, Memory := 0, Axis := axM);
    IF elSet.Done THEN
        mask := mask + 32768;                   // bit15: el72xx off
        st := 16;
    END_IF;
ELSIF st = 16 THEN
    saw(Execute := pulse, Slave := axM);
    IF saw.Done AND saw.Value0 = 1000 AND saw.Value3 = 4000 THEN
        mask := mask + 65536;                   // bit16: saw charac
        st := 17;
    END_IF;
ELSIF st = 17 THEN
    soeRd(Execute := pulse, Index := 16#0135, Axis := axM);
    IF soeRd.Done AND soeRd.Value > 0.0 THEN
        mask := mask + 131072;                  // bit17: SoE status
        st := 18;
    END_IF;
ELSIF st = 18 THEN
    soeWrPw(Execute := pulse, Password := 1234, Axis := axM);
    IF soeWrPw.Done THEN
        mask := mask + 262144;                  // bit18: SoE passwd
        st := 19;
    END_IF;
ELSIF st = 19 THEN
    soeCmd(Execute := pulse, CommandId := 5, Axis := axM);
    IF soeCmd.Done THEN
        mask := mask + 524288;                  // bit19: SoE exec
        st := 20;
    END_IF;
ELSIF st = 20 THEN
    // command state polls to done (2) within the sim window;
    // tog re-pulses Execute every other scan while waiting
    IF cmdDoneSeen = FALSE THEN
        soeCmdSt(Execute := (st = 20) AND tog, Axis := axM);
        IF soeCmdSt.Done AND soeCmdSt.CommandState = 2 THEN
            cmdDoneSeen := TRUE;
            mask := mask + 1048576;             // bit20: cmd done
        END_IF;
    END_IF;
    IF cmdDoneSeen THEN st := 21; END_IF;
ELSIF st = 21 THEN
    soeCmd2(Execute := pulse, CommandId := 16#8005, Axis := axM);
    IF soeCmd2.Done THEN
        st := 22;
    END_IF;
ELSIF st = 22 THEN
    // error command (bit15) latches cmd state 3
    IF cmdErrSeen = FALSE THEN
        soeCmdSt(Execute := (st = 22) AND tog, Axis := axM);
        IF soeCmdSt.Done AND soeCmdSt.CommandState = 3 THEN
            cmdErrSeen := TRUE;
            mask := mask + 2097152;             // bit21: cmd error
        END_IF;
    END_IF;
    IF cmdErrSeen THEN st := 23; END_IF;
ELSIF st = 23 THEN
    soeRst(Execute := pulse, Axis := axM);
    IF soeRst.Done THEN
        mask := mask + 4194304;                 // bit22: SoE reset
        st := 24;
    END_IF;
ELSIF st = 24 THEN
    ax5kV(Execute := pulse, Axis := axM);
    // mains = 230 + 0.5*(sim clock steps) -> 230.0..234.5
    IF ax5kV.Done AND ax5kV.Voltage >= 230.0
       AND ax5kV.Voltage <= 234.5 THEN
        mask := mask + 8388608;                 // bit23: ax5k mains
        st := 25;
    END_IF;
ELSIF st = 25 THEN
    ax5kFw(Execute := pulse, Axis := axM);
    IF ax5kFw.Done AND ax5kFw.FirmwareRevision = 1
       AND ax5kFw.FirmwareBuild = 5 THEN
        mask := mask + 16777216;                // bit24: ax5k fw
        st := 26;
    END_IF;
ELSIF st = 26 THEN
    // master velocity move starts (mvVEx fired this scan)
    st := 27;
ELSIF st = 27 THEN
    // MC_GearInVelo 2:1: slave velocity tracks master at 2x
    IF gv.InGear AND rvelS.Valid AND
       (rvelS.Velocity >= 780) AND (rvelS.Velocity <= 820) THEN
        mask := mask + 33554432;                // bit25: gear velo
        st := 28;
    END_IF;
ELSIF st = 28 THEN
    st := 29;
ELSIF st = 29 THEN
    // MC_GearInPos 1:1: slave CSP position tracks master position
    IF gp.InGear AND rposS.Valid AND rposM.Valid AND
       (rposS.Position >= rposM.Position - 50)
       AND (rposS.Position <= rposM.Position + 50) THEN
        mask := mask + 67108864;                // bit26: gear pos
        st := 30;
    END_IF;
ELSIF st = 30 THEN
    IF NOT pwrM.Status AND NOT pwrS.Status THEN
        mask := mask + 134217728;               // bit27: powered off
        st := 31;
    END_IF;
END_IF;

END_PROGRAM
