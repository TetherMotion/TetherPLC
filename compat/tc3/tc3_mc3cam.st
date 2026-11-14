// Tc3_Mc3Camming — MC3 phasing and gear-in-position blocks
// (TF55xx_MC3 camming/phasing subset).
//
// Implemented PLC-side over FB_MotionAxis, mirroring the MC_GearIn
// feedforward pattern in tc2_mc2.st: while the command is active
// each scan reads master position (0x6064) and velocity (0x606C),
// computes the slave position target and commands slave velocity
// (PV mode, 0x60FF)
//     velCmd = masterVel + Kp * (target - slavePos)
// The proportional term is the phase correction; Kp = 10 gives a
// ~100 ms position time constant in the simulated drive. Done /
// InSync report once the position error stays inside TOL for
// SUS scans. This is a documented approximation: real phasing and
// gear-in run inside the NC kernel with profile supervision and a
// persistent cam coupling; here the command ends at Done and the
// slave keeps its last velocity command until another motion
// command (MC_HaltPhasing, MC_GearOut, MC_Stop, ...) takes over.
//
//   MC_PhasingAbsolute  target = masterPos + PhaseShift
//   MC_PhasingRelative  target = masterPos + offset0 + PhaseShift
//                      (offset0 = slavePos - masterPos at engage)
//   MC_GearInPos        target = slavePos0 + ratio*(masterPos - masterPos0)
//   MC_HaltPhasing      ramp slave to rest, Done at VEL_ACTUAL = 0
//
// The cam-table FBs (MC_CamIn/Add/Exchange/Remove/Scaling/
// TableSelect/ReadCamInfo) need the NCI path kernel and remain
// deferred; MC_Cam* table storage is covered by tc3_nccam's
// single-table coupling engine, not this API surface.

// convergence window and sustain count shared by all phasing FBs
// (ST has no constants; literals inlined per FB)

FUNCTION_BLOCK MC_PhasingAbsolute
VAR_INPUT
    Execute : BOOL;
    PhaseShift : DINT;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Jerk : DINT;
    Master : UDINT;
    Slave : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    good : INT;
    mpos : DINT;
    mvel : DINT;
    spos : DINT;
    tgt : DINT;
    vcmd : DINT;
END_VAR
    axM.AXIS := Master;
    axS.AXIS := Slave;
    axM.OP := 3; axM();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge AND NOT active THEN
        axS.OP := 5; axS.INDEX := 16#6040; axS.VALUE := 16#000F; axS();
        axS.OP := 5; axS.INDEX := 16#6060; axS.VALUE := 3; axS();
        active := TRUE; good := 0;
    END_IF;
    IF active THEN
        axM.OP := 4; axM.INDEX := 16#6064; axM(); mpos := axM.VALUE;
        axM.OP := 4; axM.INDEX := 16#606C; axM(); mvel := axM.VALUE;
        axS.OP := 4; axS.INDEX := 16#6064; axS(); spos := axS.VALUE;
        tgt := mpos + PhaseShift;
        vcmd := mvel + (tgt - spos) * 10;
        axS.OP := 5; axS.INDEX := 16#60FF; axS.VALUE := vcmd; axS();
        axS.OP := 6; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; active := FALSE;
        ELSIF ABS(spos - tgt) <= 250 THEN
            good := good + 1;
            IF good >= 5 THEN
                Done := TRUE; active := FALSE;
            END_IF;
        ELSE
            good := 0;
        END_IF;
    END_IF;
    Busy := active;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_PhasingRelative
VAR_INPUT
    Execute : BOOL;
    PhaseShift : DINT;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Jerk : DINT;
    Master : UDINT;
    Slave : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    haveOff : BOOL;
    good : INT;
    off : DINT;
    mpos : DINT;
    mvel : DINT;
    spos : DINT;
    tgt : DINT;
    vcmd : DINT;
END_VAR
    axM.AXIS := Master;
    axS.AXIS := Slave;
    axM.OP := 3; axM();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge AND NOT active THEN
        axS.OP := 5; axS.INDEX := 16#6040; axS.VALUE := 16#000F; axS();
        axS.OP := 5; axS.INDEX := 16#6060; axS.VALUE := 3; axS();
        active := TRUE; good := 0; haveOff := FALSE;
    END_IF;
    IF active THEN
        axM.OP := 4; axM.INDEX := 16#6064; axM(); mpos := axM.VALUE;
        axM.OP := 4; axM.INDEX := 16#606C; axM(); mvel := axM.VALUE;
        axS.OP := 4; axS.INDEX := 16#6064; axS(); spos := axS.VALUE;
        IF NOT haveOff THEN
            off := spos - mpos;
            haveOff := TRUE;
        END_IF;
        tgt := mpos + off + PhaseShift;
        vcmd := mvel + (tgt - spos) * 10;
        axS.OP := 5; axS.INDEX := 16#60FF; axS.VALUE := vcmd; axS();
        axS.OP := 6; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; active := FALSE;
        ELSIF ABS(spos - tgt) <= 250 THEN
            good := good + 1;
            IF good >= 5 THEN
                Done := TRUE; active := FALSE;
            END_IF;
        ELSE
            good := 0;
        END_IF;
    END_IF;
    Busy := active;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_GearInPos
VAR_INPUT
    Execute : BOOL;
    RatioNumerator : DINT;
    RatioDenominator : DINT;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Jerk : DINT;
    Master : UDINT;
    Slave : UDINT;
END_VAR
VAR_OUTPUT
    InSync : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    haveAnchor : BOOL;
    good : INT;
    mpos0 : DINT;
    spos0 : DINT;
    mpos : DINT;
    mvel : DINT;
    spos : DINT;
    tgt : DINT;
    vcmd : DINT;
END_VAR
    axM.AXIS := Master;
    axS.AXIS := Slave;
    axM.OP := 3; axM();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    InSync := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge AND NOT active THEN
        IF RatioDenominator = 0 THEN
            Error := TRUE; ErrorID := 1;
        ELSE
            axS.OP := 5; axS.INDEX := 16#6040; axS.VALUE := 16#000F; axS();
            axS.OP := 5; axS.INDEX := 16#6060; axS.VALUE := 3; axS();
            active := TRUE; good := 0; haveAnchor := FALSE;
        END_IF;
    END_IF;
    IF active THEN
        axM.OP := 4; axM.INDEX := 16#6064; axM(); mpos := axM.VALUE;
        axM.OP := 4; axM.INDEX := 16#606C; axM(); mvel := axM.VALUE;
        axS.OP := 4; axS.INDEX := 16#6064; axS(); spos := axS.VALUE;
        IF NOT haveAnchor THEN
            mpos0 := mpos; spos0 := spos; haveAnchor := TRUE;
        END_IF;
        tgt := spos0 + (mpos - mpos0) * RatioNumerator / RatioDenominator;
        vcmd := mvel * RatioNumerator / RatioDenominator
                + (tgt - spos) * 10;
        axS.OP := 5; axS.INDEX := 16#60FF; axS.VALUE := vcmd; axS();
        axS.OP := 6; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; active := FALSE;
        ELSIF ABS(spos - tgt) <= 250 THEN
            good := good + 1;
            IF good >= 5 THEN
                InSync := TRUE; active := FALSE;
            END_IF;
        ELSE
            good := 0;
        END_IF;
    END_IF;
    Busy := active;
END_FUNCTION_BLOCK

// ---- MC_HaltPhasing: ramp the slave to rest and disengage ----
FUNCTION_BLOCK MC_HaltPhasing
VAR_INPUT
    Execute : BOOL;
    Deceleration : DINT;
    Jerk : DINT;
    Master : UDINT;
    Slave : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
END_VAR
    axS.AXIS := Slave;
    axS.OP := 3; axS();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge AND NOT active THEN
        axS.OP := 5; axS.INDEX := 16#60FF; axS.VALUE := 0; axS();
        active := TRUE;
    END_IF;
    IF active THEN
        axS.OP := 6; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; active := FALSE;
        ELSIF axS.VEL_ACTUAL = 0 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF;
    Busy := active;
END_FUNCTION_BLOCK

// ===========================================================================
// Cam table objects + cam coupling FBs (Tc3_Mc3Camming §5.4.3).
//
// TwinCAT models cam tables as MC_CamObject / Type_CouplingFunction1D
// objects with METHOD-based point access; this dialect has no object
// references or METHODs, so a cam object flattens to a UDINT handle in
// the tplc-cam service. FB_CamTable is the compat-side object holder
// (create/write/config edges — the flattened METHOD surface);
// MC_CamTableSelect applies periodic/continuity to a handle;
// MC_CamIn/Add/Exchange bind a slave axis to master+cam and stream the
// interpolated position target over 0x607A (CSP) while the binding
// lives; MC_CamRemove/MC_CamScaling/MC_ReadCamInfo operate on the
// shared binding registry.
//
// Documented deviations: one binding per slave (multi-plate cam
// summation deferred — cam_bind returns E_BOUND), cam tables are
// point lists in axis user units (LREAL storage, DINT boundary like
// tc3_nccam), continuity requirement is stored but not enforced,
// ActivationOptions/BufferMode inputs accepted but not modeled.
// ===========================================================================

// ---- FB_CamTable: flattened MC_CamObject (table holder) ----
FUNCTION_BLOCK FB_CamTable
VAR_INPUT
    WritePt : BOOL;
    PtIndex : UDINT;
    PtMaster : LREAL;
    PtSlave : LREAL;
    ClearTable : BOOL;
    Config : BOOL;
    TableCount : UDINT;
    Periodic : BOOL;
    Continuity : UDINT;
END_VAR
VAR_OUTPUT
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    cam : FB_Cam;
    prevWr : BOOL;
    prevClr : BOOL;
    prevCfg : BOOL;
    flags : UDINT;
    Handle : UDINT;   // public: cam object handle for MC_CAM_REF flattening
END_VAR
    Error := FALSE; ErrorID := 0;
    IF Handle = 0 THEN
        cam.OP := 0; cam();
        IF cam.STATUS = 0 THEN Handle := cam.HANDLE; END_IF;
    END_IF;
    IF Handle <> 0 THEN
        cam.HANDLE := Handle;
        IF WritePt AND NOT prevWr THEN
            cam.OP := 2; cam.AUX := PtIndex;
            cam.F1 := PtMaster; cam.F2 := PtSlave; cam();
            IF cam.STATUS <> 0 THEN
                Error := TRUE; ErrorID := cam.ERRID;
            END_IF;
        END_IF;
        IF ClearTable AND NOT prevClr THEN
            cam.OP := 1; cam();
            IF cam.STATUS <> 0 THEN
                Error := TRUE; ErrorID := cam.ERRID;
            END_IF;
        END_IF;
        IF Config AND NOT prevCfg THEN
            flags := Continuity * 2;
            IF Periodic THEN flags := flags + 1; END_IF;
            cam.OP := 3; cam.AUX := TableCount; cam.FLAGS := flags;
            cam();
            IF cam.STATUS <> 0 THEN
                Error := TRUE; ErrorID := cam.ERRID;
            END_IF;
        END_IF;
    END_IF;
    prevWr := WritePt; prevClr := ClearTable; prevCfg := Config;
END_FUNCTION_BLOCK

// shared cam-stream loop shape (inlined per FB; the dialect has no
// inheritance):
//   OP7 bound query -> cam/master/scalings (fresh each scan so
//     MC_CamScaling updates propagate and MC_CamRemove stops the
//     stream when the binding disappears)
//   master POS_ACTUAL -> mt = act*mscale+moff -> OP4 eval -> st
//   st = st*sscale+soff -> write slave 0x607A = LREAL_TO_DINT(st)
//   (CSP position-target streaming, same as tc3_nccam)
//   InSync when |slavePosActual - tgt| <= 250 for 5 scans

FUNCTION_BLOCK MC_CamIn
VAR_INPUT
    Execute : BOOL;
    Master : UDINT;
    Slave : UDINT;
    CamObject : UDINT;
    MasterScaling : LREAL := 1.0;
    MasterOffset : LREAL := 0.0;
    SlaveScaling : LREAL := 1.0;
    SlaveOffset : LREAL := 0.0;
    ActivationMode : UDINT := 0;
    ActivationPosition : LREAL := 0.0;
    ActivationReference : UDINT := 0;
    BufferMode : UDINT := 0;
END_VAR
VAR_OUTPUT
    InSync : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    cam : FB_Cam;
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    good : INT;
    bCam : UDINT;
    bMaster : UDINT;
    mAct : DINT;
    sAct : DINT;
    mt : LREAL;
    st : LREAL;
    tgt : DINT;
    msc : LREAL;
    mofs : LREAL;
    ssc : LREAL;
    sofs : LREAL;
END_VAR
    cam.AUX := Slave;
    cam.OP := 7; cam();
    bCam := cam.HANDLE;
    msc := cam.F1; mofs := cam.F2; ssc := cam.F3; sofs := cam.F4;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    InSync := FALSE; Busy := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        cam.OP := 6;
        cam.AUX := Slave; cam.AUX2 := Master; cam.HANDLE := CamObject;
        cam.F1 := MasterScaling; cam.F2 := MasterOffset;
        cam.F3 := SlaveScaling; cam.F4 := SlaveOffset;
        cam();
        IF cam.STATUS <> 0 THEN
            Error := TRUE; ErrorID := cam.ERRID;
        END_IF;
    END_IF;
    IF bCam <> 0 AND CamObject <> 0 AND bCam = CamObject THEN
        Busy := TRUE;
        axM.AXIS := bMaster; axM.OP := 3; axM();
        axM.OP := 6; axM(); mAct := axM.POS_ACTUAL;
        mt := DINT_TO_LREAL(mAct) * msc + mofs;
        cam.OP := 4; cam.HANDLE := bCam; cam.F1 := mt; cam();
        st := cam.F2 * ssc + sofs;
        tgt := LREAL_TO_DINT(st);
        axS.AXIS := Slave; axS.OP := 3; axS();
        axS.OP := 6; axS(); sAct := axS.POS_ACTUAL;
        axS.OP := 5; axS.INDEX := 16#607A; axS.VALUE := tgt; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; Busy := FALSE;
        ELSIF ABS(sAct - tgt) <= 250 THEN
            good := good + 1;
            IF good >= 5 THEN InSync := TRUE; END_IF;
        ELSE
            good := 0;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- MC_CamAdd: additional cam coupling (single-binding rule) ----
// Multi-plate summation needs the NC kernel; cam_bind rejects a
// second binding on the same slave, so this behaves like MC_CamIn
// when the slave is free and errors when it is already coupled.
FUNCTION_BLOCK MC_CamAdd
VAR_INPUT
    Execute : BOOL;
    Master : UDINT;
    Slave : UDINT;
    CamObject : UDINT;
    MasterScaling : LREAL := 1.0;
    MasterOffset : LREAL := 0.0;
    SlaveScaling : LREAL := 1.0;
    SlaveOffset : LREAL := 0.0;
    ActivationMode : UDINT := 0;
    ActivationPosition : LREAL := 0.0;
    ActivationReference : UDINT := 0;
    BufferMode : UDINT := 0;
END_VAR
VAR_OUTPUT
    InSync : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    cam : FB_Cam;
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    good : INT;
    bCam : UDINT;
    mAct : DINT;
    sAct : DINT;
    mt : LREAL;
    st : LREAL;
    tgt : DINT;
    msc : LREAL;
    mofs : LREAL;
    ssc : LREAL;
    sofs : LREAL;
END_VAR
    cam.AUX := Slave;
    cam.OP := 7; cam();
    bCam := cam.HANDLE;
    msc := cam.F1; mofs := cam.F2; ssc := cam.F3; sofs := cam.F4;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    InSync := FALSE; Busy := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        cam.OP := 6;
        cam.AUX := Slave; cam.AUX2 := Master; cam.HANDLE := CamObject;
        cam.F1 := MasterScaling; cam.F2 := MasterOffset;
        cam.F3 := SlaveScaling; cam.F4 := SlaveOffset;
        cam();
        IF cam.STATUS <> 0 THEN
            Error := TRUE; ErrorID := cam.ERRID;
        END_IF;
    END_IF;
    IF bCam <> 0 AND CamObject <> 0 AND bCam = CamObject THEN
        Busy := TRUE;
        axM.AXIS := Master; axM.OP := 3; axM();
        axM.OP := 6; axM(); mAct := axM.POS_ACTUAL;
        mt := DINT_TO_LREAL(mAct) * msc + mofs;
        cam.OP := 4; cam.HANDLE := bCam; cam.F1 := mt; cam();
        st := cam.F2 * ssc + sofs;
        tgt := LREAL_TO_DINT(st);
        axS.AXIS := Slave; axS.OP := 3; axS();
        axS.OP := 6; axS(); sAct := axS.POS_ACTUAL;
        axS.OP := 5; axS.INDEX := 16#607A; axS.VALUE := tgt; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; Busy := FALSE;
        ELSIF ABS(sAct - tgt) <= 250 THEN
            good := good + 1;
            IF good >= 5 THEN InSync := TRUE; END_IF;
        ELSE
            good := 0;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- MC_CamExchange: swap the slave's cam coupling ----
FUNCTION_BLOCK MC_CamExchange
VAR_INPUT
    Execute : BOOL;
    Master : UDINT;
    Slave : UDINT;
    CamObjectToRemove : UDINT;
    CamObjectToAdd : UDINT;
    MasterScaling : LREAL := 1.0;
    MasterOffset : LREAL := 0.0;
    SlaveScaling : LREAL := 1.0;
    SlaveOffset : LREAL := 0.0;
    ActivationMode : UDINT := 0;
    ActivationPosition : LREAL := 0.0;
    ActivationReference : UDINT := 0;
    BufferMode : UDINT := 0;
END_VAR
VAR_OUTPUT
    InSync : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    cam : FB_Cam;
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    good : INT;
    myCam : UDINT;
    mAct : DINT;
    sAct : DINT;
    mt : LREAL;
    st : LREAL;
    tgt : DINT;
    msc : LREAL;
    mofs : LREAL;
    ssc : LREAL;
    sofs : LREAL;
END_VAR
    cam.AUX := Slave;
    cam.OP := 7; cam();
    msc := cam.F1; mofs := cam.F2; ssc := cam.F3; sofs := cam.F4;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    InSync := FALSE; Busy := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        // remove must target the currently bound cam object
        cam.OP := 8; cam.AUX := Slave; cam.HANDLE := CamObjectToRemove;
        cam();
        IF cam.STATUS = 0 THEN
            cam.OP := 6;
            cam.AUX := Slave; cam.AUX2 := Master;
            cam.HANDLE := CamObjectToAdd;
            cam.F1 := MasterScaling; cam.F2 := MasterOffset;
            cam.F3 := SlaveScaling; cam.F4 := SlaveOffset;
            cam();
            IF cam.STATUS = 0 THEN myCam := CamObjectToAdd; END_IF;
        END_IF;
        IF cam.STATUS <> 0 THEN
            Error := TRUE; ErrorID := cam.ERRID;
        END_IF;
    END_IF;
    IF myCam <> 0 THEN
        Busy := TRUE;
        axM.AXIS := Master; axM.OP := 3; axM();
        axM.OP := 6; axM(); mAct := axM.POS_ACTUAL;
        mt := DINT_TO_LREAL(mAct) * msc + mofs;
        cam.OP := 4; cam.HANDLE := myCam; cam.F1 := mt; cam();
        st := cam.F2 * ssc + sofs;
        tgt := LREAL_TO_DINT(st);
        axS.AXIS := Slave; axS.OP := 3; axS();
        axS.OP := 6; axS(); sAct := axS.POS_ACTUAL;
        axS.OP := 5; axS.INDEX := 16#607A; axS.VALUE := tgt; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; Busy := FALSE;
        ELSIF ABS(sAct - tgt) <= 250 THEN
            good := good + 1;
            IF good >= 5 THEN InSync := TRUE; END_IF;
        ELSE
            good := 0;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- MC_CamRemove: drop the slave's cam coupling ----
FUNCTION_BLOCK MC_CamRemove
VAR_INPUT
    Execute : BOOL;
    Slave : UDINT;
    CamObject : UDINT;
    ActivationMode : UDINT := 0;
    ActivationPosition : LREAL := 0.0;
    ActivationReference : UDINT := 0;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    cam : FB_Cam;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    cam.AUX := Slave;
    cam.OP := 7; cam();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Busy := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        cam.OP := 8; cam.AUX := Slave; cam.HANDLE := CamObject; cam();
        IF cam.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := cam.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- MC_CamScaling: update coupling scalings on a bound slave ----
FUNCTION_BLOCK MC_CamScaling
VAR_INPUT
    Execute : BOOL;
    Slave : UDINT;
    CamObject : UDINT;
    MasterScaling : LREAL := 1.0;
    MasterOffset : LREAL := 0.0;
    SlaveScaling : LREAL := 1.0;
    SlaveOffset : LREAL := 0.0;
    ActivationMode : UDINT := 0;
    ActivationPosition : LREAL := 0.0;
    ActivationReference : UDINT := 0;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    cam : FB_Cam;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    cam.AUX := Slave;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Busy := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        cam.OP := 9;
        cam.AUX := Slave;
        cam.F1 := MasterScaling; cam.F2 := MasterOffset;
        cam.F3 := SlaveScaling; cam.F4 := SlaveOffset;
        cam();
        IF cam.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := cam.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- MC_CamTableSelect: apply periodic/continuity to a cam object ----
// CamRef and CamObject both flatten to the same UDINT handle (the
// object IS the table); CamObjectOut echoes the configured handle.
FUNCTION_BLOCK MC_CamTableSelect
VAR_INPUT
    Execute : BOOL;
    CamRef : UDINT;
    Periodic : BOOL;
    ContinuityRequirement : UDINT := 0;
    CamObjectIn : UDINT := 0;
END_VAR
VAR_OUTPUT
    CamObjectOut : UDINT;
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    cam : FB_Cam;
    exPrev : BOOL;
    exEdge : BOOL;
    flags : UDINT;
    h : UDINT;
END_VAR
    h := CamObjectIn;
    IF h = 0 THEN h := CamRef; END_IF;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    CamObjectOut := h;
    Done := FALSE; Busy := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge AND h <> 0 THEN
        flags := ContinuityRequirement * 2;
        IF Periodic THEN flags := flags + 1; END_IF;
        cam.OP := 3; cam.HANDLE := h; cam.AUX := 0; cam.FLAGS := flags;
        cam();
        IF cam.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := cam.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- MC_ReadCamInfo: report the slave's cam coupling ----
// CouplingInfo[] flattens to scalar outputs for the (single)
// binding: CamHandle/MasterAxis/InSync/SlaveScaling.
FUNCTION_BLOCK MC_ReadCamInfo
VAR_INPUT
    Execute : BOOL;
    Slave : UDINT;
    NumberOfCouplings : UDINT := 8;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    NumberOfCouplingsFound : UDINT;
    CamHandle : UDINT;
    MasterAxis : UDINT;
    CouplingInSync : BOOL;
    SlaveScaling : LREAL;
END_VAR
VAR
    cam : FB_Cam;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    cam.AUX := Slave;
    cam.OP := 7; cam();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Busy := FALSE;
    Error := FALSE; ErrorID := 0;
    NumberOfCouplingsFound := 0;
    CamHandle := 0; MasterAxis := 0;
    CouplingInSync := FALSE; SlaveScaling := 1.0;
    IF exEdge THEN
        IF cam.STATUS = 0 THEN
            Done := TRUE;
            IF cam.HANDLE <> 0 THEN
                NumberOfCouplingsFound := 1;
                CamHandle := cam.HANDLE;
                MasterAxis := cam.AUX2;
                SlaveScaling := cam.F3;
            END_IF;
        ELSE
            Error := TRUE; ErrorID := cam.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
