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
// (MC_Power/MC_Reset/MC_SetPosition/MC_Read*); the motion-command
// blocks (MC_Move*/MC_Stop/MC_Halt/MC_Home/MC_Jog) and the full
// PLCopen state machine land in the next tranche.
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

// DS402 object indices used by the wrappers.
// (declared as constants via VAR in each FB that needs them)

// ---- MC_Power: software enable of the axis ----
// Enable=TRUE drives the controlword to EnableOperation (0x000F);
// Enable=FALSE drives DisableVoltage (0x0000). Status is TRUE once
// the axis reports CiA402 OperationEnabled (state 0x27 = 39).
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
END_VAR
    // advance dynamics once per scan
    ax.OP := 3; ax();
    // drive the controlword from the Enable level
    ax.AXIS := Axis;
    ax.OP := 5;
    ax.INDEX := 16#6040;
    IF Enable THEN
        ax.VALUE := 16#000F;
    ELSE
        ax.VALUE := 0;
    END_IF;
    ax();
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
// Reports the PLCopen-style operating state derived from the
// CiA402 status: 0=Disabled,1=Standstill,2=DiscreteMotion,
// 3=ContinuousMotion,4=Homing, plus ErrorStop on fault. Valid
// signals a successful read. (Full PLCopen state machine is a
// later tranche; this maps the DS402 state directly.)
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
    mv : DINT;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 6;
    ax();
    IF Enable AND ax.STATUS >= 0 THEN
        Valid := TRUE;
        stt := ax.STATUS;   // CiA402 state
        mv := ax.BUSY;      // motion-in-progress flag
        // map DS402 state -> PLCopen operating state
        IF stt = 8 OR stt = 15 THEN
            OperatingState := 5;   // ErrorStop
        ELSIF mv <> 0 THEN
            OperatingState := 2;   // DiscreteMotion (moving to target)
        ELSIF stt = 39 THEN
            OperatingState := 1;   // Standstill (enabled, at rest)
        ELSE
            OperatingState := 0;   // Disabled / not ready
        END_IF;
    ELSE
        Valid := FALSE;
        OperatingState := -1;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK
