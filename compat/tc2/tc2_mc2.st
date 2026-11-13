// Tc2_MC2 — TwinCAT PLCopen motion function blocks over the
// builtin FB_MotionAxis (tplc-motion service), a simulated
// CiA402/DS402-shaped axis pool.
//
// AXIS_REF mapping: TwinCAT's AXIS_REF is a large structure
// (PlcToDrive/DriveToPlc sub-records, axis id, parameters). The
// simulated backend identifies an axis by a bounded UDINT handle
// returned from FB_MotionAxis OP=1 create, so the compat surface
// takes that handle as the `Axis` input. Position/velocity values
// are the backend's native int32 user units (TwinCAT uses LREAL
// engineering units; the LREAL<->int conversion is a documented
// gap). This file implements the axis-management blocks
// (MC_Power/MC_Reset/MC_SetPosition/MC_Read*), the point-to-point /
// velocity / stop / halt / homing / jog command blocks, and the
// modulo / override / axis-parameter FBs. The motion-command FBs
// drive the DS402 controlword command pattern (NewSetPoint for
// PP/HM, QuickStop for MC_Stop) and the mode/target objects;
// per-command Velocity/Acceleration/Deceleration inputs are
// accepted for signature compatibility but the simulated axis
// applies its configured dynamics limits.
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

// DS402 object indices used by the wrappers.
// (declared as constants via VAR in each FB that needs them)

// ---- MC_Power: software enable of the axis ----
// Enable=TRUE drives the controlword to EnableOperation (0x000F);
// Enable=FALSE drives DisableVoltage (0x0000). The controlword is
// written only on an Enable transition (edge-triggered) so the
// motion-command FBs can layer NewSetPoint/QuickStop command bits
// on top without being overwritten every scan; the drive holds the
// commanded state. Status is TRUE once the axis reports CiA402
// OperationEnabled (state 0x27 = 39). Enable_Positive/
// Enable_Negative are accepted for signature compatibility; the
// simulated axis is bidirectional so only Enable gates power.
FUNCTION_BLOCK MC_Power
VAR_INPUT
    Enable : BOOL;
    Enable_Positive : BOOL;
    Enable_Negative : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Status : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    enPrev : BOOL;
END_VAR
    // advance dynamics once per scan
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    // edge-trigger the enable command so command FBs own the
    // controlword's command bits between transitions
    IF Enable <> enPrev THEN
        ax.OP := 5;
        ax.INDEX := 16#6040;
        IF Enable THEN
            ax.VALUE := 16#000F;
        ELSE
            ax.VALUE := 0;
        END_IF;
        ax();
        enPrev := Enable;
    END_IF;
    // snapshot for Status (CiA402 state in STATUS for OP=6)
    ax.OP := 6;
    ax();
    Status := (ax.STATUS = 39);
    Busy := FALSE;
    Error := FALSE;
    ErrorID := 0;
END_FUNCTION_BLOCK

// ---- MC_Reset: reset the axis (fault reset controlword bit) ----
// Execute rising edge pulses controlword bit7 (FaultReset,
// 0x0080) onto the enable command for one scan, then releases it
// the next scan. To avoid fighting MC_Power over 0x6040, the
// controlword is only written during that two-scan window; on
// other scans the FB leaves the controlword to MC_Power. Done
// reports the reset write succeeded.
FUNCTION_BLOCK MC_Reset
VAR_INPUT
    Execute : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    edge : BOOL;
    rst : BOOL;
    rel : BOOL;
END_VAR
    edge := Execute AND NOT rst;
    rst := Execute;
    ax.AXIS := Axis;
    ax.OP := 3; ax();   // advance dynamics/state
    IF edge OR rel THEN
        ax.OP := 5;
        ax.INDEX := 16#6040;
        IF edge THEN
            ax.VALUE := 16#008F;   // FaultReset | EnableOperation
        ELSE
            ax.VALUE := 16#000F;   // release reset, stay enabled
        END_IF;
        ax();
        rel := edge;              // next scan releases bit7
        Done := (edge AND ax.STATUS = 0);
    ELSE
        rel := FALSE;
        Done := FALSE;
    END_IF;
    Busy := FALSE;
    Error := FALSE;
    ErrorID := 0;
END_FUNCTION_BLOCK

// ---- MC_SetPosition: preset the axis position ----
// Execute rising edge writes PositionActualValue (0x6064): the
// absolute value, or current+Position when Mode=TRUE (relative).
// Position is int32 user units (LREAL engineering units is a gap).
FUNCTION_BLOCK MC_SetPosition
VAR_INPUT
    Execute : BOOL;
    Position : DINT;
    Mode : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    edge : BOOL;
    rst : BOOL;
    cur : DINT;
    np : DINT;
END_VAR
    edge := Execute AND NOT rst;
    rst := Execute;
    // Position write is edge-triggered only; rewriting every scan would
    // re-apply a stale Position input and revert later moves.
    IF edge THEN
        np := Position;
        IF Mode THEN
            // relative: read current actual position first
            ax.AXIS := Axis;
            ax.OP := 4;
            ax.INDEX := 16#6064;
            ax();
            cur := ax.VALUE;
            np := cur + Position;
        END_IF;
        ax.AXIS := Axis;
        ax.OP := 5;
        ax.INDEX := 16#6064;
        ax.VALUE := np;
        ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Done := FALSE;
            Error := TRUE;
            ErrorID := ax.STATUS;
        END_IF;
    ELSE
        Done := FALSE;
        Error := FALSE;
        ErrorID := 0;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ReadActualPosition: read position actual value ----
// Enable=TRUE reads cyclically; Valid/Position carry the value.
FUNCTION_BLOCK MC_ReadActualPosition
VAR_INPUT
    Enable : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Position : DINT;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 4;
    ax.INDEX := 16#6064;
    ax();
    IF Enable AND ax.STATUS = 0 THEN
        Valid := TRUE;
        Position := ax.VALUE;
    ELSE
        Valid := FALSE;
        Position := 0;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ReadActualVelocity: read velocity actual value ----
FUNCTION_BLOCK MC_ReadActualVelocity
VAR_INPUT
    Enable : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Velocity : DINT;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 4;
    ax.INDEX := 16#606C;
    ax();
    IF Enable AND ax.STATUS = 0 THEN
        Valid := TRUE;
        Velocity := ax.VALUE;
    ELSE
        Valid := FALSE;
        Velocity := 0;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ReadAxisError: read the axis error code ----
// Valid=TRUE when the read succeeded; AxisErrorID is the DS402
// error code object (0x603F), 0 when no fault is latched.
FUNCTION_BLOCK MC_ReadAxisError
VAR_INPUT
    Enable : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    AxisErrorID : DINT;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 4;
    ax.INDEX := 16#603F;
    ax();
    IF Enable AND ax.STATUS = 0 THEN
        Valid := TRUE;
        AxisErrorID := ax.VALUE;
    ELSE
        Valid := FALSE;
        AxisErrorID := 0;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ReadStatus: read the axis operating state ----
// Reports the PLCopen operating state derived from the CiA402
// status, mode display, actual velocity and fault flags:
//   0=Disabled, 1=Standstill, 2=Homing, 3=DiscreteMotion,
//   4=ContinuousMotion, 6=Stopping, 7=ErrorStop (-1=read fail).
// (5=SynchronizedMotion is reserved for gearing/camming, not yet
// produced.) Valid signals a successful read.
FUNCTION_BLOCK MC_ReadStatus
VAR_INPUT
    Enable : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    OperatingState : INT;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
    stt : DINT;
    md : DINT;
    tr : DINT;
    va : DINT;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 6;
    ax();
    IF Enable AND ax.STATUS >= 0 THEN
        Valid := TRUE;
        stt := ax.STATUS;            // CiA402 state
        md := ax.MODE;               // mode of operation display
        tr := ax.TARGET_REACHED;
        va := ax.VEL_ACTUAL;
        IF ax.FAULT <> 0 OR stt = 8 OR stt = 15 THEN
            OperatingState := 7;     // ErrorStop (fault latched)
        ELSIF stt = 7 THEN
            OperatingState := 6;     // Stopping (QuickStopActive)
        ELSIF stt <> 39 THEN
            OperatingState := 0;     // Disabled / not ready
        ELSIF md = 6 THEN
            OperatingState := 2;     // Homing
        ELSIF md = 1 AND tr = 0 THEN
            OperatingState := 3;     // DiscreteMotion (PP move active)
        ELSIF (md = 2 OR md = 3 OR md = 8 OR md = 9 OR md = 10)
              AND va <> 0 THEN
            OperatingState := 4;     // ContinuousMotion (moving)
        ELSE
            OperatingState := 1;     // Standstill (enabled, at rest)
        END_IF;
    ELSE
        Valid := FALSE;
        OperatingState := -1;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ================================================================== //
// Motion-command FBs. Each drives the DS402 controlword command
// pattern on its Execute edge: PP/HM modes pulse NewSetPoint
// (0x0010) with the enable pattern for one scan then release to
// 0x000F; velocity modes write mode+target velocity under the
// enable pattern; MC_Stop commands QuickStop (controlword QS bit
// cleared); MC_Halt ramps profile-velocity to zero. The controlword
// is written only on the command edge (plus the single NSP release
// scan), so MC_Power's edge-triggered enable is not fought and a
// later MC_Stop/MC_Power transition can override a running command.
// ================================================================== //

// ---- MC_MoveAbsolute: absolute point-to-point move (PP mode) ----
// Execute edge: mode=ProfilePosition, TargetPosition=Position,
// controlword=EnableOperation|NewSetPoint (0x001F). Done when the
// axis reports TargetReached (statusword bit 0x0400). Velocity/
// Acceleration/Deceleration are signature-compatible; the axis
// applies its configured dynamics.
FUNCTION_BLOCK MC_MoveAbsolute
VAR_INPUT
    Execute : BOOL;
    Position : DINT;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    rel : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF NOT active AND exEdge THEN
        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 1; ax();
        ax.OP := 5; ax.INDEX := 16#607A; ax.VALUE := Position; ax();
        ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#001F; ax();
        IF ax.STATUS = 0 THEN
            active := TRUE; rel := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    ELSIF active THEN
        IF rel THEN
            ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
            rel := FALSE;
        END_IF;
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; active := FALSE;
        ELSIF ax.TARGET_REACHED = 1 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF
    Busy := active;
END_FUNCTION_BLOCK

// ---- MC_MoveRelative: relative point-to-point move (PP mode) ----
// Execute edge reads PositionActualValue, adds Distance, then runs
// the same PP/NewSetPoint sequence as MC_MoveAbsolute.
FUNCTION_BLOCK MC_MoveRelative
VAR_INPUT
    Execute : BOOL;
    Distance : DINT;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    rel : BOOL;
    cur : DINT;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF NOT active AND exEdge THEN
        ax.OP := 4; ax.INDEX := 16#6064; ax();   // read pos actual
        cur := ax.VALUE;
        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 1; ax();
        ax.OP := 5; ax.INDEX := 16#607A; ax.VALUE := cur + Distance; ax();
        ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#001F; ax();
        IF ax.STATUS = 0 THEN
            active := TRUE; rel := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    ELSIF active THEN
        IF rel THEN
            ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
            rel := FALSE;
        END_IF;
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; active := FALSE;
        ELSIF ax.TARGET_REACHED = 1 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF
    Busy := active;
END_FUNCTION_BLOCK

// ---- MC_MoveVelocity: endless travel at Velocity (PV mode) ----
// Execute edge: mode=ProfileVelocity, TargetVelocity=Velocity,
// controlword=EnableOperation. The axis ramps to the target velocity
// and holds it; the move ends via MC_Stop/MC_Halt. ContinuousUpdate
// rewrites TargetVelocity each scan for live dynamics changes.
// InVelocity reports TargetReached (|vel-target| within window).
FUNCTION_BLOCK MC_MoveVelocity
VAR_INPUT
    Execute : BOOL;
    ContinuousUpdate : BOOL;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    InVelocity : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    engaged : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    InVelocity := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 3; ax();
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := Velocity; ax();
        engaged := TRUE;
    END_IF
    IF engaged AND ContinuousUpdate THEN
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := Velocity; ax();
    END_IF
    IF engaged THEN
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; engaged := FALSE;
        ELSE
            InVelocity := (ax.TARGET_REACHED = 1);
        END_IF
    END_IF
    Busy := engaged;
END_FUNCTION_BLOCK

// ---- MC_Stop: quick stop, locks the axis ----
// Execute edge commands QuickStop by clearing the controlword QS
// bit (write 0x000B = enable pattern without QuickStop-request).
// The drive decelerates to rest and holds QuickStopActive; Done
// reports when VelocityActualValue reaches zero. A later motion
// command re-asserts the enable pattern and resumes.
FUNCTION_BLOCK MC_Stop
VAR_INPUT
    Execute : BOOL;
    Deceleration : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF NOT active AND exEdge THEN
        ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000B; ax();
        IF ax.STATUS = 0 THEN
            active := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    ELSIF active THEN
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; active := FALSE;
        ELSIF ax.VEL_ACTUAL = 0 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF
    Busy := active;
END_FUNCTION_BLOCK

// ---- MC_Halt: controlled stop with ramp (stays enabled) ----
// Execute edge: mode=ProfileVelocity, TargetVelocity=0 under the
// enable pattern, so the axis ramps to rest at its configured
// deceleration and remains OperationEnabled. Done when velocity
// reaches zero. Unlike MC_Stop the axis is not locked and a new
// command can resume immediately.
FUNCTION_BLOCK MC_Halt
VAR_INPUT
    Execute : BOOL;
    Deceleration : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF NOT active AND exEdge THEN
        ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 3; ax();
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
        active := TRUE;
    ELSIF active THEN
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; active := FALSE;
        ELSIF ax.VEL_ACTUAL = 0 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF
    Busy := active;
END_FUNCTION_BLOCK

// ---- MC_Home: homing move to the home offset (HM mode) ----
// Execute edge: writes HomeOffset=Position (0x607C, the reference
// position after homing), mode=Homing, controlword=enable|NewSet-
// Point. Done when the axis reports TargetReached/homing complete.
FUNCTION_BLOCK MC_Home
VAR_INPUT
    Execute : BOOL;
    Position : DINT;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    rel : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF NOT active AND exEdge THEN
        ax.OP := 5; ax.INDEX := 16#607C; ax.VALUE := Position; ax();
        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 6; ax();
        ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#001F; ax();
        IF ax.STATUS = 0 THEN
            active := TRUE; rel := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    ELSIF active THEN
        IF rel THEN
            ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
            rel := FALSE;
        END_IF;
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; active := FALSE;
        ELSIF ax.TARGET_REACHED = 1 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF
    Busy := active;
END_FUNCTION_BLOCK

// ---- MC_Jog: manual jog while a direction key is held (PV mode) ----
// JogForward/JogBackwards are level inputs: while held the axis ramps
// to +/-Velocity in ProfileVelocity mode; on release TargetVelocity
// returns to 0 and the axis ramps to rest. Busy is TRUE while a key
// is held; Done pulses once the axis is back at rest after a jog.
// The mode is latched to ProfileVelocity on the first key press.
FUNCTION_BLOCK MC_Jog
VAR_INPUT
    JogForward : BOOL;
    JogBackwards : BOOL;
    Velocity : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    modeSet : BOOL;
    wasMoving : BOOL;
    key : BOOL;
    stopped : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    key := JogForward OR JogBackwards;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF key AND NOT modeSet THEN
        ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 3; ax();
        modeSet := TRUE;
    END_IF
    // velocity target follows the held key every scan
    IF JogForward THEN
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := Velocity; ax();
    ELSIF JogBackwards THEN
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := -Velocity; ax();
    ELSE
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
    END_IF
    ax.OP := 6; ax();
    stopped := (NOT key) AND (ax.VEL_ACTUAL = 0);
    IF stopped AND wasMoving THEN
        Done := TRUE;
    END_IF
    wasMoving := NOT stopped;
    Busy := NOT stopped;
END_FUNCTION_BLOCK

// ================================================================== //
// Tranche-4 blocks: modulo positioning, velocity override and the
// axis-parameter access FBs. The parameter FBs treat ParameterNumber
// as the DS402 object-dictionary index (0x0000..0xFFFF), giving
// generic object access through the PLCopen-style signature; the
// TwinCAT numeric parameter ids map onto indices in a real system,
// which is a documented simplification. Gearing/phasing/camming and
// hardware touch-probe latch need multi-axis coupling / latch
// objects the simulated backend does not expose — see README gaps.
// ================================================================== //

// ---- MC_MoveModulo: modulo positioning (PP mode) ----
// Position is the modulo target in [0,ModuloFactor). Direction
// selects the travel path: 0=shortest (default), 1=positive
// direction only, 2=negative direction only. The absolute target
// is computed from the current position so the axis lands on the
// requested modulo angle, then a standard PP/NewSetPoint move runs.
FUNCTION_BLOCK MC_MoveModulo
VAR_INPUT
    Execute : BOOL;
    Position : DINT;
    Velocity : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Direction : INT;
    ModuloFactor : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    rel : BOOL;
    cur : DINT;
    tgt : DINT;
    delta : DINT;
    goal : DINT;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF NOT active AND exEdge THEN
        IF ModuloFactor <= 0 THEN
            Error := TRUE; ErrorID := 1;
        ELSE
            ax.OP := 4; ax.INDEX := 16#6064; ax();   // pos actual
            // normalize current and target into [0,ModuloFactor)
            cur := ax.VALUE - (ax.VALUE / ModuloFactor) * ModuloFactor;
            IF cur < 0 THEN cur := cur + ModuloFactor; END_IF;
            tgt := Position - (Position / ModuloFactor) * ModuloFactor;
            IF tgt < 0 THEN tgt := tgt + ModuloFactor; END_IF;
            delta := tgt - cur;
            IF Direction = 1 THEN
                IF delta < 0 THEN delta := delta + ModuloFactor; END_IF;
            ELSIF Direction = 2 THEN
                IF delta > 0 THEN delta := delta - ModuloFactor; END_IF;
            ELSE
                IF delta > ModuloFactor / 2 THEN
                    delta := delta - ModuloFactor;
                ELSIF delta < -ModuloFactor / 2 THEN
                    delta := delta + ModuloFactor;
                END_IF;
            END_IF;
            goal := ax.VALUE + delta;
            ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 1; ax();
            ax.OP := 5; ax.INDEX := 16#607A; ax.VALUE := goal; ax();
            ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#001F; ax();
            IF ax.STATUS = 0 THEN
                active := TRUE; rel := TRUE;
            ELSE
                Error := TRUE; ErrorID := ax.ERRID;
            END_IF;
        END_IF;
    ELSIF active THEN
        IF rel THEN
            ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
            rel := FALSE;
        END_IF;
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; active := FALSE;
        ELSIF ax.TARGET_REACHED = 1 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF
    Busy := active;
END_FUNCTION_BLOCK

// ---- MC_SetOverride: velocity override factor ----
// While Enable is held the velocity-mode target (0x60FF) is scaled
// to baseVel * VelFactor / 100, where baseVel is latched on the
// Enable rising edge; on release the unscaled base is restored.
// VelFactor is an integer percent (100 = 100%, 0 = hold at rest).
// Applies to velocity modes (PV/VL); ProfilePosition speed comes
// from axis config so the write is a harmless no-op there.
FUNCTION_BLOCK MC_SetOverride
VAR_INPUT
    Enable : BOOL;
    VelFactor : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Enabled : BOOL;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
    enPrev : BOOL;
    hasBase : BOOL;
    baseVel : DINT;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    IF Enable AND NOT enPrev THEN
        ax.OP := 4; ax.INDEX := 16#60FF; ax();
        baseVel := ax.VALUE;
        hasBase := TRUE;
    END_IF;
    enPrev := Enable;
    IF Enable AND hasBase THEN
        ax.OP := 5; ax.INDEX := 16#60FF;
        ax.VALUE := baseVel * VelFactor / 100; ax();
        Enabled := (ax.STATUS = 0);
    ELSIF (NOT Enable) AND hasBase THEN
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := baseVel; ax();
        hasBase := FALSE;
        Enabled := FALSE;
    ELSE
        Enabled := FALSE;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ReadParameter: read a DS402 object as an axis parameter ----
// ParameterNumber is the DS402 object index. Valid reports a
// successful read into Value.
FUNCTION_BLOCK MC_ReadParameter
VAR_INPUT
    Enable : BOOL;
    ParameterNumber : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Value : DINT;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 4;
    ax.INDEX := ParameterNumber;
    ax();
    IF Enable AND ax.STATUS = 0 THEN
        Valid := TRUE;
        Value := ax.VALUE;
    ELSE
        Valid := FALSE;
        Value := 0;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_WriteParameter: write a DS402 object as an axis parameter ----
// Execute edge writes Value to the object named by ParameterNumber.
FUNCTION_BLOCK MC_WriteParameter
VAR_INPUT
    Execute : BOOL;
    ParameterNumber : UDINT;
    Value : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5;
        ax.INDEX := ParameterNumber;
        ax.VALUE := Value;
        ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ReadBoolParameter: read a DS402 object as a BOOL ----
// Value is TRUE when the object at ParameterNumber is nonzero.
FUNCTION_BLOCK MC_ReadBoolParameter
VAR_INPUT
    Enable : BOOL;
    ParameterNumber : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Value : BOOL;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 4;
    ax.INDEX := ParameterNumber;
    ax();
    IF Enable AND ax.STATUS = 0 THEN
        Valid := TRUE;
        Value := (ax.VALUE <> 0);
    ELSE
        Valid := FALSE;
        Value := FALSE;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_WriteBoolParameter: write a DS402 object from a BOOL ----
// Execute edge writes 1/0 to the object named by ParameterNumber.
FUNCTION_BLOCK MC_WriteBoolParameter
VAR_INPUT
    Execute : BOOL;
    ParameterNumber : UDINT;
    Value : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    raw : DINT;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        IF Value THEN raw := 1; ELSE raw := 0; END_IF;
        ax.OP := 5;
        ax.INDEX := ParameterNumber;
        ax.VALUE := raw;
        ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ================================================================== //
// Tranche-5 blocks: touch-probe latch and master/slave gearing.
// MC_TouchProbe captures PositionActualValue on the TriggerInput
// rising edge — a cycle-level software latch (TwinCAT latches in
// drive hardware, sub-cycle; the simulated backend has no latch
// object, so capture latency is one scan — documented gap).
// MC_GearIn couples the slave axis to the master at a velocity
// ratio by reading VelocityActualValue each scan and driving the
// slave's TargetVelocity in PV mode — velocity-ratio coupling, not
// an exact position-phase lock (that needs cyclic setpoint
// coupling / a coupling object; documented gap). MC_AbortTrigger
// cancels an armed probe through the per-axis probe-abort object
// 0x2001 (see below); MC_TouchProbe self-aborts on a re-Execute.
// ================================================================== //

// ---- MC_TouchProbe: position capture on a trigger edge ----
// Execute edge arms the probe. While armed the axis position is
// polled each scan; on a TriggerInput rising edge the position is
// recorded into RecordedPosition (optionally only when inside
// [FirstPosition,LastPosition] when WindowOnly is TRUE). Done
// reports a captured value. Re-executing while armed re-arms.
// The arm edge also clears the axis probe-abort object (0x2001) so
// an abort issued before the arm cannot kill the new cycle; while
// armed each scan polls 0x2001 and a nonzero value consumes the
// request (writes 0 back), disarms and pulses CommandAborted.
FUNCTION_BLOCK MC_TouchProbe
VAR_INPUT
    Execute : BOOL;
    TriggerInput : BOOL;
    WindowOnly : BOOL;
    FirstPosition : DINT;
    LastPosition : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RecordedPosition : DINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    armed : BOOL;
    trigPrev : BOOL;
    inWin : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        armed := TRUE;               // (re)arm the probe
        ax.OP := 5; ax.INDEX := 16#2001; ax.VALUE := 0; ax();
    END_IF;
    IF armed THEN
        ax.OP := 4; ax.INDEX := 16#2001; ax();   // abort request?
        IF ax.VALUE <> 0 THEN
            ax.OP := 5; ax.INDEX := 16#2001; ax.VALUE := 0; ax();
            armed := FALSE;
            CommandAborted := TRUE;
        ELSE
            ax.OP := 4; ax.INDEX := 16#6064; ax();   // pos actual
            IF TriggerInput AND NOT trigPrev THEN
                inWin := TRUE;
                IF WindowOnly THEN
                    inWin := (ax.VALUE >= FirstPosition)
                             AND (ax.VALUE <= LastPosition);
                END_IF;
                IF inWin THEN
                    RecordedPosition := ax.VALUE;
                    Done := TRUE;
                    armed := FALSE;
                END_IF;
            END_IF;
        END_IF;
    END_IF;
    trigPrev := TriggerInput;
    Busy := armed;
END_FUNCTION_BLOCK

// ---- MC_AbortTrigger: cancel an armed touch-probe cycle ----
// Execute edge writes 1 to the axis probe-abort object (0x2001,
// manufacturer-specific sim extension shared with MC_TouchProbe).
// Done reports once the request has been consumed, i.e. the flag
// reads 0 again (an armed MC_TouchProbe cleared it after
// disarming). Busy stays TRUE while the request is pending; issue
// aborts only while a probe cycle is active, matching TwinCAT
// usage. TriggerInput is accepted for signature fidelity; the
// single-probe-per-axis model aborts the axis's armed probe
// regardless of which trigger reference is passed.
FUNCTION_BLOCK MC_AbortTrigger
VAR_INPUT
    Execute : BOOL;
    TriggerInput : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    pending : BOOL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Busy := pending; CommandAborted := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge AND NOT pending THEN
        ax.OP := 5; ax.INDEX := 16#2001; ax.VALUE := 1; ax();
        IF ax.STATUS = 0 THEN
            pending := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
    IF pending THEN
        ax.OP := 4; ax.INDEX := 16#2001; ax();
        IF ax.STATUS = 0 AND ax.VALUE = 0 THEN
            Done := TRUE; pending := FALSE;
        END_IF;
    END_IF;
    Busy := pending;
END_FUNCTION_BLOCK

// ---- MC_GearIn: velocity-ratio master/slave coupling ----
// While Execute is held the slave axis runs in ProfileVelocity at
// TargetVelocity = master VelocityActualValue * RatioNumerator /
// RatioDenominator, so the slave velocity tracks the master at the
// gear ratio (positions track proportionally but not phase-locked).
// Engage edge puts the slave into PV mode + enable; on release the
// slave keeps its last velocity until MC_GearOut/MC_Halt stops it.
FUNCTION_BLOCK MC_GearIn
VAR_INPUT
    Execute : BOOL;
    RatioNumerator : DINT;
    RatioDenominator : DINT;
    Acceleration : DINT;
    Deceleration : DINT;
    Master : UDINT;
    Slave : UDINT;
END_VAR
VAR_OUTPUT
    InGear : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    mvel : DINT;
END_VAR
    axM.AXIS := Master;
    axS.AXIS := Slave;
    axM.OP := 3; axM();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    InGear := FALSE; CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        IF RatioDenominator = 0 THEN
            Error := TRUE; ErrorID := 1;
        ELSE
            axS.OP := 5; axS.INDEX := 16#6040; axS.VALUE := 16#000F; axS();
            axS.OP := 5; axS.INDEX := 16#6060; axS.VALUE := 3; axS();
        END_IF;
    END_IF;
    IF Execute AND RatioDenominator <> 0 THEN
        // slave velocity follows the master at the gear ratio
        axM.OP := 4; axM.INDEX := 16#606C; axM();
        mvel := axM.VALUE;
        axS.OP := 5; axS.INDEX := 16#60FF;
        axS.VALUE := mvel * RatioNumerator / RatioDenominator; axS();
        axS.OP := 6; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID;
        ELSE
            InGear := TRUE;
        END_IF;
    END_IF;
    Busy := Execute AND (RatioDenominator <> 0);
END_FUNCTION_BLOCK

// ---- MC_GearOut: uncouple the slave and ramp it to rest ----
// Execute edge commands the slave TargetVelocity to 0 (PV decel);
// Done when VelocityActualValue reaches zero. The slave stays
// enabled in velocity mode; use MC_Halt/MC_Stop for a hard stop.
FUNCTION_BLOCK MC_GearOut
VAR_INPUT
    Execute : BOOL;
    Deceleration : DINT;
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
    IF NOT active AND exEdge THEN
        axS.OP := 5; axS.INDEX := 16#60FF; axS.VALUE := 0; axS();
        active := TRUE;
    ELSIF active THEN
        axS.OP := 6; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID; active := FALSE;
        ELSIF axS.VEL_ACTUAL = 0 THEN
            Done := TRUE; active := FALSE;
        END_IF;
    END_IF;
    Busy := active;
END_FUNCTION_BLOCK
