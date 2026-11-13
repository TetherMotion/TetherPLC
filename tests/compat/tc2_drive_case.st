// tc2_drive_case.st — Tc2_Drive e2e over the Tether IO backend.
// Exercises the SoE surface (state read, diag fields, password,
// command control/exec/state, error+reset, AX5000 blocks) against
// tether_io_motion_server's ec.soe.* params, plus the
// SimplePlcMotion CoE/SoE enable + velocity blocks on a simulated
// CiA402 axis. 14 checks packed into %QD0; last step expects
// mask 0x3fff.
PROGRAM tc2_drive_case
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    ph     : INT;
    wrN    : INT;
    cur    : INT;
    tgl    : BOOL;
    axH    : UDINT;
    pos    : DINT;

    ax     : FB_MotionAxis;
    axC    : FB_MotionAxis;

    stFb   : FB_SoEReadStatus;
    msgFb  : FB_SoEReadDiagMessage;
    numFb  : FB_SoEReadDiagNumber;
    lstFb  : FB_SoEReadDiagNumberList;
    clsFb  : FB_SoEReadClassXDiag;
    atFb   : FB_SoEReadAmplifierTemperature;
    mtFb   : FB_SoEReadMotorTemperature;
    dvFb   : FB_SoEReadDcBusVoltage;
    dcFb   : FB_SoEReadDcBusCurrent;
    pwFb   : FB_SoEWritePassword;
    ccFb   : FB_SoEWriteCommandControl;
    exFb   : FB_SoEExecuteCommand;
    rsFb   : FB_SoEReset;
    vmFb   : FB_SoEAX5000ReadActMainVoltage;
    mcFb   : FB_SoEAX5000SetMotorCtrlWord;
    fwFb   : FB_SoEAX5000FirmwareUpdate;
    pkFb   : FB_SoEAX5000ParkAxis;
    enFb   : FB_CoEDriveEnable;
    mvFb   : FB_SoEDriveMoveVelocity;
END_VAR

cur := st;

// ---- st=0: create the motion axis (for SimplePlcMotion) ----
IF cur = 0 THEN
    mask := 0;
    ph := 0;
    wrN := 0;
    tgl := FALSE;
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 4000;
    ax.MAX_ACCEL := 20000;
    ax.MAX_DECEL := 20000;
    ax.POS_MIN := -100000;
    ax.POS_MAX := 100000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 10;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    axH := ax.AXIS;
    st := 1;
END_IF;

// ---- st=1: axis created? ----
IF cur = 1 THEN
    IF axH <> 0 THEN
        mask := mask + 1;          // bit0
        st := 2;
    ELSIF wrN > 50 THEN
        st := 90;
    ELSE
        wrN := wrN + 1;
    END_IF;
END_IF;

// ---- st=2/3: SoE status read (slave 0) ----
IF cur = 2 THEN
    stFb.Drive := 0;
    stFb.Execute := TRUE;
    st := 3;
END_IF;
IF cur = 3 THEN
    stFb.Execute := FALSE;
    IF stFb.bDone AND stFb.StatusWord = 3 AND stFb.CommandState = 0 THEN
        mask := mask + 2;          // bit1
    END_IF;
    st := 4;
END_IF;

// ---- st=4/5: diag number + list + class-X (slave 0) ----
IF cur = 4 THEN
    numFb.Drive := 0; numFb.Execute := TRUE;
    lstFb.Drive := 0; lstFb.Execute := TRUE;
    clsFb.Drive := 0; clsFb.Execute := TRUE;
    st := 5;
END_IF;
IF cur = 5 THEN
    numFb.Execute := FALSE;
    lstFb.Execute := FALSE;
    clsFb.Execute := FALSE;
    IF numFb.bDone AND numFb.DiagNumber = 16#8105 AND
       clsFb.bDone AND clsFb.ClassXDiag = 3 AND
       lstFb.bDone AND lstFb.DiagCount = 3 AND
       lstFb.DiagList[0] = 16#8105 THEN
        mask := mask + 4;          // bit2
    END_IF;
    st := 6;
END_IF;

// ---- st=6/7: diag message + temperatures + bus values ----
IF cur = 6 THEN
    msgFb.Drive := 0; msgFb.Execute := TRUE;
    atFb.Drive := 0;  atFb.Execute := TRUE;
    mtFb.Drive := 0;  mtFb.Execute := TRUE;
    dvFb.Drive := 0;  dvFb.Execute := TRUE;
    dcFb.Drive := 0;  dcFb.Execute := TRUE;
    st := 7;
END_IF;
IF cur = 7 THEN
    msgFb.Execute := FALSE;
    atFb.Execute := FALSE;
    mtFb.Execute := FALSE;
    dvFb.Execute := FALSE;
    dcFb.Execute := FALSE;
    IF msgFb.bDone AND msgFb.DiagMessage = 'OVERLOAD' AND
       atFb.bDone AND atFb.Value = 61.5 AND
       mtFb.bDone AND mtFb.Value = 58.25 AND
       dvFb.bDone AND dvFb.Value = 565.5 AND
       dcFb.bDone AND dcFb.Value = 12.75 THEN
        mask := mask + 8;          // bit3
    END_IF;
    st := 8;
END_IF;

// ---- st=8/9/10/11: password write, command-control write,
//      then one status re-read verifying both readbacks ----
IF cur = 8 THEN
    pwFb.Drive := 0;
    pwFb.Password := 16#1234;
    pwFb.Execute := TRUE;
    st := 9;
END_IF;
IF cur = 9 THEN
    pwFb.Execute := FALSE;
    ccFb.Drive := 0;
    ccFb.CommandControl := 16#55AA;
    ccFb.Execute := TRUE;
    st := 10;
END_IF;
IF cur = 10 THEN
    ccFb.Execute := FALSE;
    stFb.Drive := 0;
    stFb.Execute := TRUE;
    st := 11;
END_IF;
IF cur = 11 THEN
    stFb.Execute := FALSE;
    IF stFb.bDone AND stFb.Password = 16#1234 THEN
        mask := mask + 16;         // bit4
    END_IF;
    IF stFb.bDone AND stFb.CmdCtrl = 16#55AA THEN
        mask := mask + 32;         // bit5
    END_IF;
    st := 12;
END_IF;

// ---- st=12/13/14: execute command 0x0007, poll for done (2) ----
IF cur = 12 THEN
    exFb.Drive := 0;
    exFb.CommandId := 16#0007;
    exFb.Execute := TRUE;
    st := 13;
END_IF;
IF cur = 13 THEN
    exFb.Execute := FALSE;
    tgl := FALSE;
    wrN := 0;
    st := 14;
END_IF;
IF cur = 14 THEN
    tgl := NOT tgl;
    stFb.Drive := 0;
    stFb.Execute := tgl;
    IF stFb.bDone AND stFb.CommandState = 2 THEN
        mask := mask + 64;         // bit6
        stFb.Execute := FALSE;
        st := 15;
    ELSIF wrN > 80 THEN
        stFb.Execute := FALSE;
        st := 90;
    ELSE
        wrN := wrN + 1;
    END_IF;
END_IF;

// ---- st=15/16/17: execute command 0x8001 -> error state + fault ----
IF cur = 15 THEN
    exFb.Drive := 0;
    exFb.CommandId := 16#8001;
    exFb.Execute := TRUE;
    st := 16;
END_IF;
IF cur = 16 THEN
    exFb.Execute := FALSE;
    tgl := FALSE;
    wrN := 0;
    st := 17;
END_IF;
IF cur = 17 THEN
    tgl := NOT tgl;
    stFb.Drive := 0;
    stFb.Execute := tgl;
    IF stFb.bDone AND stFb.CommandState = 3 AND
       (stFb.StatusWord AND 4) <> 0 THEN
        mask := mask + 128;        // bit7
        stFb.Execute := FALSE;
        st := 18;
    ELSIF wrN > 80 THEN
        stFb.Execute := FALSE;
        st := 90;
    ELSE
        wrN := wrN + 1;
    END_IF;
END_IF;

// ---- st=18/19/20: reset clears the fault + command state ----
IF cur = 18 THEN
    rsFb.Drive := 0;
    rsFb.Execute := TRUE;
    st := 19;
END_IF;
IF cur = 19 THEN
    rsFb.Execute := FALSE;
    tgl := FALSE;
    wrN := 0;
    st := 20;
END_IF;
IF cur = 20 THEN
    tgl := NOT tgl;
    stFb.Drive := 0;
    stFb.Execute := tgl;
    IF stFb.bDone AND stFb.CommandState = 0 AND
       (stFb.StatusWord AND 4) = 0 THEN
        mask := mask + 256;        // bit8
        stFb.Execute := FALSE;
        st := 21;
    ELSIF wrN > 80 THEN
        stFb.Execute := FALSE;
        st := 90;
    ELSE
        wrN := wrN + 1;
    END_IF;
END_IF;

// ---- st=21..24: AX5000 motor ctrl word, park, mains, fw ----
IF cur = 21 THEN
    mcFb.Drive := 0;
    mcFb.MotorCtrlWord := 16#A5A5;
    mcFb.Execute := TRUE;
    st := 22;
END_IF;
IF cur = 22 THEN
    mcFb.Execute := FALSE;
    pkFb.Drive := 0;
    pkFb.Park := TRUE;
    pkFb.Execute := TRUE;
    st := 23;
END_IF;
IF cur = 23 THEN
    pkFb.Execute := FALSE;
    vmFb.Drive := 0; vmFb.Execute := TRUE;
    fwFb.Drive := 0; fwFb.Execute := TRUE;
    stFb.Drive := 0; stFb.Execute := TRUE;
    st := 24;
END_IF;
IF cur = 24 THEN
    vmFb.Execute := FALSE;
    fwFb.Execute := FALSE;
    stFb.Execute := FALSE;
    IF stFb.bDone AND stFb.AxCtrl = 16#A5A5 AND
       stFb.AxPark = 1 AND stFb.AxFwRev = 1 AND stFb.AxFwBuild = 5 AND
       vmFb.bDone AND vmFb.Voltage >= 230.0 AND vmFb.Voltage < 240.0 AND
       fwFb.bDone AND fwFb.FirmwareRevision = 1 AND
       fwFb.FirmwareBuild = 5 THEN
        mask := mask + 512;        // bit9
    END_IF;
    st := 25;
END_IF;

// ---- st=25/26: slave 1 diag number (OVERTEMP) ----
IF cur = 25 THEN
    numFb.Drive := 1;
    numFb.Execute := TRUE;
    st := 26;
END_IF;
IF cur = 26 THEN
    numFb.Execute := FALSE;
    IF numFb.bDone AND numFb.DiagNumber = 16#8201 THEN
        mask := mask + 1024;       // bit10
    END_IF;
    st := 27;
END_IF;

// ---- st=27/28: slave 5 has no diag entries ----
IF cur = 27 THEN
    lstFb.Drive := 5;
    lstFb.Execute := TRUE;
    st := 28;
END_IF;
IF cur = 28 THEN
    lstFb.Execute := FALSE;
    IF lstFb.bDone AND lstFb.DiagCount = 0 THEN
        mask := mask + 2048;       // bit11
    END_IF;
    st := 29;
END_IF;

// ---- st=29/30: CoE drive enable on the motion axis ----
IF cur = 29 THEN
    enFb.Enable := TRUE;
    enFb.Drive := axH;
    enFb.Execute := TRUE;
    st := 30;
END_IF;
IF cur = 30 THEN
    enFb.Execute := FALSE;
    IF enFb.bDone THEN
        mask := mask + 4096;       // bit12
    END_IF;
    mvFb.Drive := axH;
    mvFb.Velocity := 500.0;
    mvFb.Execute := TRUE;
    wrN := 0;
    st := 31;
END_IF;

// ---- st=31: SoE velocity move; wait for position > 50 ----
IF cur = 31 THEN
    axC.AXIS := axH;
    axC.OP := 4; axC.INDEX := 16#6064; axC();
    pos := axC.VALUE;
    IF pos > 50 THEN
        mask := mask + 8192;       // bit13
        mvFb.Execute := FALSE;
        st := 90;
    ELSIF wrN > 300 THEN
        mvFb.Execute := FALSE;
        st := 90;
    ELSE
        wrN := wrN + 1;
    END_IF;
END_IF;



// FB invocations (one call per scan each)
ax(); axC();
stFb(); msgFb(); numFb(); lstFb(); clsFb();
atFb(); mtFb(); dvFb(); dcFb();
pwFb(); ccFb(); exFb(); rsFb();
vmFb(); mcFb(); fwFb(); pkFb();
enFb(); mvFb();
END_PROGRAM
