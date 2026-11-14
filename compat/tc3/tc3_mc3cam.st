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
