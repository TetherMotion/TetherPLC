// tc2_mc2_drive.st — Tc2_MC2_Drive compatibility surface.
//
// TwinCAT's MC2 drive library addresses drives through AXIS_REF
// (here the flattened FB_MotionAxis UDINT handle, same convention
// as tc2_mc2.st).  Two transport families are covered:
//
//   CoE / OD  — CiA-402 object dictionary access through
//               FB_MotionAxis OP4/OP5 (works on both the in-process
//               tplc-motion backend and the Tether IO backend).
//               Manufacturer-specific sim objects 0x2002..0x2009
//               carry position offset, brake/park/CoE commands and
//               flying-saw characteristics (stored state, no
//               physical effect — documented).
//   SoE       — Sercos-over-EtherCAT mailbox through the builtin
//               FB_Soe (tplc-soe service, Tether IO only).  The
//               AXIS_REF handle maps to the SoE port as slave =
//               handle - 1 (MotionIoService assigns handle =
//               slave + 1).  In-process there is no SoE service,
//               so SoE FBs report bError/nErrorId = 99.
//
// NAME COLLISION NOTE: the SoE/AX5000 FB names match
// compat/tc2/tc2_drive.st (TwinCAT uses the same names in both
// libraries, differing only in the drive reference type).  This
// dialect has no library namespaces — link tc2_mc2_drive.st OR
// tc2_drive.st into a program, not both.
//
// Mapping notes (documented deviations):
//   - E_* enums (E_ParkMode, E_BrakeMode, E_PositionOffsetFeedback,
//     E_PositionOffsetMemory, E_DriveParameterNumber,
//     E_ReadWriteMode) are flattened to UDINT/INT inputs; values
//     are passed through to the backing object or ignored where
//     the sim has no equivalent.
//   - PVOID/ANY buffers (FB_CoERead/Write, FB_WriteDriveParameter)
//     are flattened to a single DINT Value; sub-index and
//     CompleteAccess are accepted but only sub-index 0 scalar
//     access is modeled.
//   - T_AmsNetID inputs are accepted for signature fidelity and
//     ignored (the axis reference already selects the drive).
//   - The 0x2002..0x2009 objects are stored command/state, not
//     physical: position offset is recorded but not applied to
//     motion, brake/park/CoE commands are recorded, saw
//     characteristics are read/write storage.
//
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

// ---- shared helpers ------------------------------------------------
// SoE slave resolution: AXIS_REF handle -> SoE port address.
// (inlined per FB; handle = slave + 1 on the Tether IO backend)

// ================================================================== //
// §4.1 General Beckhoff
// ================================================================== //

// FB_ReadDriveParameter — read a drive parameter (OD object).
// ParameterNumber is the CiA402 object index; Mode 0 = single shot
// on Execute edge, Mode 1 = cyclic re-read while Execute is held
// (Interval is accepted; the scan rate is the cycle).
FUNCTION_BLOCK FB_ReadDriveParameter
VAR_INPUT
    Execute : BOOL;
    ParameterNumber : UDINT;
    Mode : UDINT;
    Interval : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Value : DINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Valid := FALSE; Busy := FALSE; Done := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge OR (Mode = 1 AND Execute) THEN
        ax.OP := 4;
        ax.INDEX := UDINT_TO_DINT(ParameterNumber);
        ax();
        IF ax.STATUS = 0 THEN
            Value := ax.VALUE;
            Valid := TRUE;
            IF exEdge THEN Done := TRUE; END_IF;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_WriteDriveParameter — write a drive parameter (OD object).
// Value is the flattened ANY buffer (single i32).
FUNCTION_BLOCK FB_WriteDriveParameter
VAR_INPUT
    Execute : BOOL;
    ParameterNumber : UDINT;
    Mode : UDINT;
    Interval : UDINT;
    Value : DINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Valid := FALSE; Busy := FALSE; Done := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5;
        ax.INDEX := UDINT_TO_DINT(ParameterNumber);
        ax.VALUE := Value;
        ax();
        IF ax.STATUS = 0 THEN
            Valid := TRUE;
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SetPositionOffset — store a position offset (OD 0x2002).
// Absolute mode: offset = Position - PositionActualValue.
// Relative mode: offset accumulates (old + Position).  Feedback
// and Memory select encoder/drive/nameplate in TwinCAT; the sim
// has a single offset object, both are accepted and ignored.
FUNCTION_BLOCK FB_SetPositionOffset
VAR_INPUT
    Execute : BOOL;
    Position : LREAL;
    Relative : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    posAct : DINT;
    offCur : DINT;
    np : DINT;
    p : DINT;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        p := LREAL_TO_DINT(Position);
        IF Relative THEN
            ax.OP := 4; ax.INDEX := 16#2002; ax();
            offCur := ax.VALUE;
            np := offCur + p;
        ELSE
            ax.OP := 4; ax.INDEX := 16#6064; ax();
            posAct := ax.VALUE;
            np := p - posAct;
        END_IF;
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := np; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_DeletePositionOffset — clear the position offset (OD 0x2002).
FUNCTION_BLOCK FB_DeletePositionOffset
VAR_INPUT
    Execute : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := 0; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_BrakeControl — brake command (OD 0x2003).  Mode is stored
// raw; the sim records the command without physical effect.
FUNCTION_BLOCK FB_BrakeControl
VAR_INPUT
    Execute : BOOL;
    Mode : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2003;
        ax.VALUE := UDINT_TO_DINT(Mode); ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_ParkAxis — park/release via the SoE park mailbox (OP 7).
// Mode 0 = release, nonzero = park.  Requires the SoE backend
// (Tether IO); in-process reports bError.
FUNCTION_BLOCK FB_ParkAxis
VAR_INPUT
    Execute : BOOL;
    Mode : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 7;
        raw.SLAVE := slave;
        IF Mode <> 0 THEN raw.VALUE := 1; ELSE raw.VALUE := 0; END_IF;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_ReadDriveInfo — axis/drive info snapshot (FB_MotionAxis OP6).
FUNCTION_BLOCK FB_ReadDriveInfo
VAR_INPUT
    Execute : BOOL;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    State : DINT;
    Fault : DINT;
    ModeOp : DINT;
    TargetReached : BOOL;
    PosActual : DINT;
    VelActual : DINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Valid := FALSE; Busy := FALSE; Done := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID;
        ELSE
            Valid := TRUE;
            Done := TRUE;
            State := ax.STATUS;
            Fault := ax.FAULT;
            ModeOp := ax.MODE;
            TargetReached := (ax.TARGET_REACHED <> 0);
            PosActual := ax.POS_ACTUAL;
            VelActual := ax.VEL_ACTUAL;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ================================================================== //
// §4.2 General SoE
// ================================================================== //
// All SoE FBs resolve slave = Axis - 1 and drive the FB_Soe builtin.

// FB_SoERead — generic SoE element read.  Index is the SoE element
// id; the supported subset is the state-snapshot fields:
//   16#0135 -> status word (S-0-0135), 16#0267 -> password.
// Other element ids report Error (arbitrary P-0-xxxx access is not
// modeled).  Value is LREAL; u16/u32 fields are returned raw.
FUNCTION_BLOCK FB_SoERead
VAR_INPUT
    Execute : BOOL;
    Index : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            IF Index = 16#0135 THEN
                Value := UDINT_TO_LREAL(raw.STATUSW);
                Done := TRUE;
            ELSIF Index = 16#0267 THEN
                Value := UDINT_TO_LREAL(raw.PASSWD);
                Done := TRUE;
            ELSE
                Error := TRUE; ErrorID := 2;
            END_IF;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEWrite — generic SoE element write.  Supported subset:
// Index 16#0267 -> drive password (S-0-0267).  Other ids report
// Error (arbitrary P-0-xxxx writes are not modeled).
FUNCTION_BLOCK FB_SoEWrite
VAR_INPUT
    Execute : BOOL;
    Index : UDINT;
    Value : DINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        IF Index = 16#0267 THEN
            raw.OP := 2;
            raw.SLAVE := slave;
            raw.VALUE := Value;
            raw();
            IF raw.STATUS < 0 THEN
                Error := TRUE; ErrorID := raw.ERRID;
            ELSE
                Done := TRUE;
            END_IF;
        ELSE
            Error := TRUE; ErrorID := 2;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoESetDataAccessMode — SoE parameter access mode.  The sim
// mailbox always uses parallel access; the FB accepts the request
// and completes (documented no-op).
FUNCTION_BLOCK FB_SoESetDataAccessMode
VAR_INPUT
    Execute : BOOL;
    Mode : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        Done := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- command FBs ----------------------------------------------------

// FB_SoEReset — S-0-0099 drive reset.
FUNCTION_BLOCK FB_SoEReset
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 5;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEWritePassword — S-0-0267 drive password.
FUNCTION_BLOCK FB_SoEWritePassword
VAR_INPUT
    Execute : BOOL;
    Password : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 2;
        raw.SLAVE := slave;
        raw.VALUE := UDINT_TO_DINT(Password);
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEExecuteCommand — execute an SoE command id.
FUNCTION_BLOCK FB_SoEExecuteCommand
VAR_INPUT
    Execute : BOOL;
    CommandId : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 4;
        raw.SLAVE := slave;
        raw.VALUE := UDINT_TO_DINT(CommandId);
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEWriteCommandControl — command control word.
FUNCTION_BLOCK FB_SoEWriteCommandControl
VAR_INPUT
    Execute : BOOL;
    CommandControl : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 3;
        raw.SLAVE := slave;
        raw.VALUE := UDINT_TO_DINT(CommandControl);
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEReadCommandState — SoE command state
// (0 idle, 1 running, 2 done, 3 error).
FUNCTION_BLOCK FB_SoEReadCommandState
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    CommandState : UDINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            CommandState := raw.CMDSTATE;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- diagnostics FBs -------------------------------------------------

// FB_SoEReadDiagMessage — S-0-0095 diagnosis message.
FUNCTION_BLOCK FB_SoEReadDiagMessage
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    DiagMessage : STRING[63];
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            DiagMessage := raw.DIAGMSG;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEReadDiagNumber — diagnosis number.
FUNCTION_BLOCK FB_SoEReadDiagNumber
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    DiagNumber : UDINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            DiagNumber := raw.DIAGNUM;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEReadDiagNumberList — diagnosis number list (flattened:
// DiagCount + DiagList internal array).
FUNCTION_BLOCK FB_SoEReadDiagNumberList
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    DiagCount : UDINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
    i : UDINT;
    DiagList : ARRAY[0..7] OF UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            DiagCount := raw.DIAGCOUNT;
            FOR i := 0 TO 7 DO
                DiagList[i] := raw.DIAGLIST[i];
            END_FOR;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEReadClassXDiag — class-X diagnosis.
FUNCTION_BLOCK FB_SoEReadClassXDiag
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    ClassXDiag : UDINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            ClassXDiag := raw.CLASSX;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- current-value readers -------------------------------------------

FUNCTION_BLOCK FB_SoEReadAmplifierTemperature
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Value := raw.AMPTEMP;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SoEReadMotorTemperature
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Value := raw.MTEMPT;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SoEReadDcBusVoltage
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Value := raw.DCVOLT;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SoEReadDcBusCurrent
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Value := raw.DCCURR;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ================================================================== //
// §4.3 General CoE — SDO object access via FB_MotionAxis OP4/OP5.
// Index/SubIndex select the object; SubIndex and CompleteAccess are
// accepted but only sub-index 0 scalar access is modeled.  Value is
// the flattened PVOID buffer (single i32).
// ================================================================== //

// FB_CoERead — CoE SDO upload.
FUNCTION_BLOCK FB_CoERead
VAR_INPUT
    Execute : BOOL;
    Index : UDINT;
    SubIndex : UDINT;
    CompleteAccess : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Value : DINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        IF SubIndex <> 0 THEN
            Error := TRUE; ErrorID := 2;   // sub-index unsupported
        ELSE
            ax.OP := 4;
            ax.INDEX := UDINT_TO_DINT(Index);
            ax();
            IF ax.STATUS = 0 THEN
                Value := ax.VALUE;
                Done := TRUE;
            ELSE
                Error := TRUE; ErrorID := ax.ERRID;
            END_IF;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEWrite — CoE SDO download.
FUNCTION_BLOCK FB_CoEWrite
VAR_INPUT
    Execute : BOOL;
    Index : UDINT;
    SubIndex : UDINT;
    Value : DINT;
    CompleteAccess : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        IF SubIndex <> 0 THEN
            Error := TRUE; ErrorID := 2;
        ELSE
            ax.OP := 5;
            ax.INDEX := UDINT_TO_DINT(Index);
            ax.VALUE := Value;
            ax();
            IF ax.STATUS = 0 THEN
                Done := TRUE;
            ELSE
                Error := TRUE; ErrorID := ax.ERRID;
            END_IF;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEExecuteCommand — execute a CoE drive command (OD 0x2005).
// Command is the flattened command id; the sim records it.
FUNCTION_BLOCK FB_CoEExecuteCommand
VAR_INPUT
    Execute : BOOL;
    Command : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2005;
        ax.VALUE := UDINT_TO_DINT(Command); ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ================================================================== //
// §4.4 AX5000 SoE — same SoE mailbox surface as the general blocks,
// AX5000-specific objects (mains voltage, motor ctrl word, firmware,
// park, position offset).
// ================================================================== //

// FB_SoEAX5000ReadActMainVoltage — P-0-0200 peak mains voltage.
FUNCTION_BLOCK FB_SoEAX5000ReadActMainVoltage
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Voltage : LREAL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Voltage := raw.AXMAINS;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEAX5000SetMotorCtrlWord — motor control word (mailbox OP6).
// bForceUnlock accepted for signature compatibility.
FUNCTION_BLOCK FB_SoEAX5000SetMotorCtrlWord
VAR_INPUT
    Execute : BOOL;
    MotorCtrlWord : UDINT;
    bForceUnlock : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 6;
        raw.SLAVE := slave;
        raw.VALUE := UDINT_TO_DINT(MotorCtrlWord);
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEAX5000FirmwareUpdate — reports simulated firmware
// revision/build; no firmware pool or flashing is simulated.
FUNCTION_BLOCK FB_SoEAX5000FirmwareUpdate
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    FirmwareRevision : UDINT;
    FirmwareBuild : UDINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 1;
        raw.SLAVE := slave;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            FirmwareRevision := raw.AXFWREV;
            FirmwareBuild := raw.AXFWBUILD;
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEAX5000SetPositionOffset — AX5000 position offset (OD 0x2002).
FUNCTION_BLOCK FB_SoEAX5000SetPositionOffset
VAR_INPUT
    Execute : BOOL;
    Position : LREAL;
    Relative : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    posAct : DINT;
    np : DINT;
    p : DINT;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        p := LREAL_TO_DINT(Position);
        IF Relative THEN
            np := p;                    // relative: offset delta
        ELSE
            ax.OP := 4; ax.INDEX := 16#6064; ax();
            posAct := ax.VALUE;
            np := p - posAct;
        END_IF;
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := np; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEAX5000DeletePositionOffset — clear AX5000 position offset.
FUNCTION_BLOCK FB_SoEAX5000DeletePositionOffset
VAR_INPUT
    Execute : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := 0; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEAX5000ParkAxis — park/unpark via SoE mailbox (OP 7).
FUNCTION_BLOCK FB_SoEAX5000ParkAxis
VAR_INPUT
    Execute : BOOL;
    Mode : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    exPrev : BOOL;
    exEdge : BOOL;
    slave : UDINT;
END_VAR
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    IF Axis > 0 THEN slave := Axis - 1; ELSE slave := 0; END_IF;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        raw.OP := 7;
        raw.SLAVE := slave;
        IF Mode <> 0 THEN raw.VALUE := 1; ELSE raw.VALUE := 0; END_IF;
        raw();
        IF raw.STATUS < 0 THEN
            Error := TRUE; ErrorID := raw.ERRID;
        ELSE
            Done := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ================================================================== //
// §4.5 AX8000 CoE — brake/park/offset over OD objects 0x2002-0x2004.
// ================================================================== //

// FB_CoEAX8000BrakeControl — AX8000 brake (OD 0x2003).
FUNCTION_BLOCK FB_CoEAX8000BrakeControl
VAR_INPUT
    Execute : BOOL;
    Mode : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2003;
        ax.VALUE := UDINT_TO_DINT(Mode); ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEAX8000BrakeTest — brake test: writes the test command and
// reports Succeeded (the sim has no torque verification; the test
// always succeeds once the command is accepted — documented).
FUNCTION_BLOCK FB_CoEAX8000BrakeTest
VAR_INPUT
    Execute : BOOL;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Succeeded : BOOL;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    Succeeded := FALSE;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2003; ax.VALUE := 2; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
            Succeeded := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEAX8000SetPositionOffset — AX8000 position offset (0x2002).
FUNCTION_BLOCK FB_CoEAX8000SetPositionOffset
VAR_INPUT
    Execute : BOOL;
    Position : LREAL;
    Relative : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    posAct : DINT;
    np : DINT;
    p : DINT;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        p := LREAL_TO_DINT(Position);
        IF Relative THEN
            np := p;
        ELSE
            ax.OP := 4; ax.INDEX := 16#6064; ax();
            posAct := ax.VALUE;
            np := p - posAct;
        END_IF;
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := np; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEAX8000DeletePositionOffset — clear AX8000 offset (0x2002).
FUNCTION_BLOCK FB_CoEAX8000DeletePositionOffset
VAR_INPUT
    Execute : BOOL;
    Position : LREAL;
    Relative : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := 0; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEAX8000ParkAxis — AX8000 park via CoE object 0x2004.
FUNCTION_BLOCK FB_CoEAX8000ParkAxis
VAR_INPUT
    Execute : BOOL;
    Mode : UDINT;
    NetId : STRING[15];
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2004;
        IF Mode <> 0 THEN ax.VALUE := 1; ELSE ax.VALUE := 0; END_IF;
        ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ================================================================== //
// §4.6 EL72xx CoE — brake/offset over the same OD objects.
// ================================================================== //

// FB_CoEEL72xxBrakeControl — EL72xx brake (OD 0x2003).
FUNCTION_BLOCK FB_CoEEL72xxBrakeControl
VAR_INPUT
    Execute : BOOL;
    Mode : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2003;
        ax.VALUE := UDINT_TO_DINT(Mode); ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEEL72xxSetPositionOffset — EL72xx offset (0x2002).
FUNCTION_BLOCK FB_CoEEL72xxSetPositionOffset
VAR_INPUT
    Execute : BOOL;
    Position : LREAL;
    Relative : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    posAct : DINT;
    np : DINT;
    p : DINT;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        p := LREAL_TO_DINT(Position);
        IF Relative THEN
            np := p;
        ELSE
            ax.OP := 4; ax.INDEX := 16#6064; ax();
            posAct := ax.VALUE;
            np := p - posAct;
        END_IF;
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := np; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_CoEEL72xxDeletePositionOffset — clear EL72xx offset (0x2002).
FUNCTION_BLOCK FB_CoEEL72xxDeletePositionOffset
VAR_INPUT
    Execute : BOOL;
    Position : LREAL;
    Relative : BOOL;
    Feedback : UDINT;
    Memory : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        ax.OP := 5; ax.INDEX := 16#2002; ax.VALUE := 0; ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ================================================================== //
// §5 Motion blocks
// ================================================================== //

// MC_GearInVelo — velocity-ratio gear coupling (same model as
// MC_GearIn in tc2_mc2.st): slave runs ProfileVelocity at
// master VelocityActualValue * RatioNumerator / RatioDenominator.
// StartSync reports once the slave is tracking (InGear).
FUNCTION_BLOCK MC_GearInVelo
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
    StartSync : BOOL;
    InGear : BOOL;
    Busy : BOOL;
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
    InGear := FALSE;
    Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        IF RatioDenominator = 0 THEN
            Error := TRUE; ErrorID := 1;
        ELSE
            axS.OP := 5; axS.INDEX := 16#6040; axS.VALUE := 16#000F; axS();
            axS.OP := 5; axS.INDEX := 16#6060; axS.VALUE := 3; axS();
        END_IF;
    END_IF;
    IF Execute AND RatioDenominator <> 0 THEN
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
    StartSync := InGear;
    Busy := Execute AND (RatioDenominator <> 0);
END_FUNCTION_BLOCK

// MC_GearInPos — position-ratio master/slave coupling: the slave
// tracks master PositionActualValue * RatioNumerator /
// RatioDenominator through CSP setpoints (0x607A).  Engage edge
// enables the slave and selects CSP mode; release leaves the last
// setpoint (stop via MC_Halt/MC_Stop).  Approximates TwinCAT's
// sync-point gearing with a continuous position ratio — no
// acceleration-profile or sync-window handling.
FUNCTION_BLOCK MC_GearInPos
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
    StartSync : BOOL;
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
    mpos : DINT;
END_VAR
    axM.AXIS := Master;
    axS.AXIS := Slave;
    axM.OP := 3; axM();
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    InGear := FALSE;
    CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        IF RatioDenominator = 0 THEN
            Error := TRUE; ErrorID := 1;
        ELSE
            axS.OP := 5; axS.INDEX := 16#6040; axS.VALUE := 16#000F; axS();
            axS.OP := 5; axS.INDEX := 16#6060; axS.VALUE := 8; axS();
        END_IF;
    END_IF;
    IF Execute AND RatioDenominator <> 0 THEN
        axM.OP := 4; axM.INDEX := 16#6064; axM();
        mpos := axM.VALUE;
        axS.OP := 5; axS.INDEX := 16#607A;
        axS.VALUE := mpos * RatioNumerator / RatioDenominator; axS();
        axS.OP := 6; axS();
        IF axS.STATUS < 0 THEN
            Error := TRUE; ErrorID := axS.ERRID;
        ELSE
            InGear := TRUE;
        END_IF;
    END_IF;
    StartSync := InGear;
    Busy := Execute AND (RatioDenominator <> 0);
END_FUNCTION_BLOCK

// MC_ReadFlyingSawCharacteristics — read the flying-saw
// characteristic values (OD 0x2006..0x2009 -> Value0..Value3).
// CamTableCharac is flattened to four DINT outputs.
FUNCTION_BLOCK MC_ReadFlyingSawCharacteristics
VAR_INPUT
    Execute : BOOL;
    Slave : UDINT;
END_VAR
VAR_OUTPUT
    Value0 : DINT;
    Value1 : DINT;
    Value2 : DINT;
    Value3 : DINT;
    Busy : BOOL;
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    i : DINT;
END_VAR
    ax.AXIS := Slave;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        FOR i := 0 TO 3 DO
            ax.OP := 4;
            ax.INDEX := 16#2006 + i;
            ax();
            IF ax.STATUS <> 0 THEN
                Error := TRUE; ErrorID := ax.ERRID;
            ELSIF i = 0 THEN Value0 := ax.VALUE;
            ELSIF i = 1 THEN Value1 := ax.VALUE;
            ELSIF i = 2 THEN Value2 := ax.VALUE;
            ELSE Value3 := ax.VALUE;
            END_IF;
        END_FOR;
        IF NOT Error THEN Done := TRUE; END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- version --------------------------------------------------------

// F_GetVersionTcMc2Drive — library version placeholder.
FUNCTION F_GetVersionTcMc2Drive : UDINT
F_GetVersionTcMc2Drive := 16#0100;
END_FUNCTION
