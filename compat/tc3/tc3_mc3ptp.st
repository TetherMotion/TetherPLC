// Tc3_Mc3Ptp — MC3 point-to-point single-axis FBs missing from
// the Tc2_MC2 surface (TF55xx_MC3 bundle).
//
// Implemented over the builtin FB_MotionAxis (tplc-motion service):
//   MC_ReadActualTorque               torque actual (0x6077)
//   MC_ReadHardwareParameter          drive-param read (OD index)
//   MC_WriteHardwareParameter         drive-param write (OD index)
//   MC_WaitForStableActualPosition    velocity-window settle wait
//   MC_TriggerError                   fault inject (sim-ext 0x200D)
//   MC_SetHomingState                 accepted, no-op (documented)
//   MC_LoadControl                    accepted, no-op (documented)
//   MC_ActivateExternalSetpointGeneration  accepted, no-op
//
// Deviation notes: hardware parameters share the standard OD index
// space (TwinCAT drive-vendor parameter ids have no sim equivalent);
// SetHomingState/LoadControl/ActivateExternalSetpointGeneration
// configure drive-internal behaviour the simulated CiA402 drive does
// not model, so they complete with Done and change nothing; the
// camming (MC_Cam*), phasing (MC_Phasing*/MC_GearInPos/MC_HaltPhasing)
// and fluid-power (MC_ValveCharacterization) FBs plus the TcCOM
// AXIS_REF/ENCODER_AXIS_REF objects are deferred.

// ---- MC_ReadActualTorque: read actual torque (0x6077) ----
FUNCTION_BLOCK MC_ReadActualTorque
VAR_INPUT
    Enable : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Torque : DINT;
    Busy : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    ax.OP := 3; ax();
    ax.AXIS := Axis;
    ax.OP := 4;
    ax.INDEX := 16#6077;
    ax();
    IF Enable AND ax.STATUS = 0 THEN
        Valid := TRUE;
        Torque := ax.VALUE;
    ELSE
        Valid := FALSE;
        Torque := 0;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ReadHardwareParameter: read a drive parameter ----
// ParameterNumber is the DS402 object-dictionary index, same
// convention as MC_ReadParameter.
FUNCTION_BLOCK MC_ReadHardwareParameter
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

// ---- MC_WriteHardwareParameter: write a drive parameter ----
FUNCTION_BLOCK MC_WriteHardwareParameter
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

// ---- MC_WaitForStableActualPosition ----
// Execute edge arms the wait; Done fires once |VelocityActual|
// stays at or below VelocityWindow for SettleScans consecutive
// scans (position is implicitly stable because the sim integrates
// position from velocity). PositionWindow is accepted for
// signature fidelity: the sim exposes no sub-window position
// quantization, so only the velocity window gates completion.
FUNCTION_BLOCK MC_WaitForStableActualPosition
VAR_INPUT
    Execute : BOOL;
    PositionWindow : DINT;
    VelocityWindow : DINT;
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
    armed : BOOL;
    still : INT;
    SettleScans : INT := 5;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        armed := TRUE; still := 0;
    END_IF;
    IF armed THEN
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID; armed := FALSE;
        ELSIF ABS(ax.VEL_ACTUAL) <= VelocityWindow THEN
            still := still + 1;
            IF still >= SettleScans THEN
                Done := TRUE; armed := FALSE;
            END_IF;
        ELSE
            still := 0;
        END_IF;
    END_IF;
    Busy := armed;
END_FUNCTION_BLOCK

// ---- MC_TriggerError: inject a drive fault ----
// Writes ErrorCode to the sim-ext error-inject object (0x200D):
// nonzero latches the CiA402 fault state with that error code
// (readable via MC_ReadAxisError / 0x603F); write 0 to clear.
// ErrorCode 0 completes without faulting (used to clear).
FUNCTION_BLOCK MC_TriggerError
VAR_INPUT
    Execute : BOOL;
    ErrorCode : DINT;
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
        ax.INDEX := 16#200D;
        ax.VALUE := ErrorCode;
        ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_SetHomingState: accepted no-op ----
// TwinCAT programs the drive homing state; the simulated drive
// derives homing state from the controlword, so the command
// completes with Done and HomingState is ignored.
FUNCTION_BLOCK MC_SetHomingState
VAR_INPUT
    Execute : BOOL;
    HomingState : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN Done := TRUE; END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_LoadControl: accepted no-op ----
// Loads controller parameters into the drive; the simulated drive
// has no separate control-parameter store, so the command
// completes with Done.
FUNCTION_BLOCK MC_LoadControl
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
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN Done := TRUE; END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- MC_ActivateExternalSetpointGeneration: accepted no-op ----
// Selects drive-external setpoint generation; the sim has no
// external-setpoint mode distinct from the written targets, so the
// command completes with Done.
FUNCTION_BLOCK MC_ActivateExternalSetpointGeneration
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
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN Done := TRUE; END_IF;
    Busy := FALSE;
END_FUNCTION_BLOCK
