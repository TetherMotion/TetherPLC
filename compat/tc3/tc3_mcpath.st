// Tc3_McCoordinatedMotion — MC path preparation + execution
// (TF5410 collision-avoidance / coordinated-motion bundle).
//
// This tranche implements the linear/dwell path subset:
//   MC_MoveLinearAbsolutePreparation, MC_MoveDirectPreparation
//   (flattened alias of the linear absolute segment),
//   MC_DwellTimePreparation, MC_MovePath.
// Path objects live in the tplc-path service via the FB_Path
// builtin; group membership comes from FB_McGroup.  Circular,
// blending and blocker segment types, MC_CalcTrafo, the CA move
// family, track following and compensation remain deferred.
//
// Flattened types (the ST dialect has no STRUCT/POINTER/REFERENCE
// support):
//   MC_PATH_DATA_REF     -> PathData : UDINT path handle (1..8)
//   Position (POINTER TO LREAL array) -> Position : ARRAY[0..15]
//                            OF LREAL + PositionCount
//   DwellTime (TIME)     -> DwellTime : UDINT milliseconds
//   BufferMode           -> UDINT (accepted, buffered queuing not
//                            modeled: every Execute edge appends a
//                            segment immediately)
//   AXES_GROUP_REF       -> AxesGroup : UDINT group handle
//
// Semantics:
//   * Preparation FBs append one segment on the Execute rising
//     edge. Linear segments store absolute user-unit positions and
//     trapezoid limits (Velocity/Acceleration; Deceleration and
//     Jerk are accepted but the service uses Acceleration for both
//     ramps). Dwell segments hold position for DwellTime ms.
//   * MC_MovePath on Execute captures each member axis's current
//     position (0x6064) as the path start, switches every member
//     to CSP mode (0x6060=8), then streams interpolated 0x607A
//     targets each scan until the path duration elapses. Positions
//     are interpolated in path parameter space on the dominant
//     axis, so all axes arrive together (coordinated motion).
//   * Segment targets chain: segment k starts where segment k-1
//     ended; the first segment starts at the captured positions.
//   * Busy stays TRUE while the path runs; Done pulses for one
//     scan at path end; Error carries service/axis failures.

// ---- shared path helper: ensure a valid PathData handle ----
// Returns TRUE and leaves `h` set to a usable path handle, or
// FALSE when the service table is full.

FUNCTION_BLOCK MC_MoveLinearAbsolutePreparation
VAR_INPUT
    Execute : BOOL;
    Position : ARRAY[0..15] OF LREAL;
    PositionCount : UDINT;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL := 0.0;
    Jerk : LREAL := 0.0;
    BufferMode : UDINT := 1;
END_VAR
VAR_IN_OUT
    PathData : UDINT;
END_VAR
VAR_OUTPUT
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    exPrev : BOOL;
    edge : BOOL;
    i : UDINT;
END_VAR
edge := Execute AND NOT exPrev; exPrev := Execute;
Error := FALSE; ErrorId := 0;
IF edge THEN
    p.HANDLE := PathData;
    IF PathData = 0 THEN
        p.OP := 0; p();
        IF p.STATUS = 0 THEN PathData := p.HANDLE; END_IF;
    END_IF;
    IF PathData <> 0 AND PositionCount >= 1 AND
       PositionCount <= 16 THEN
        FOR i := 0 TO 15 DO p.POS[i] := Position[i]; END_FOR;
        p.OP := 2; p.AUX := 1; p.AUX2 := PositionCount;
        p.F1 := Velocity; p.F2 := Acceleration; p.F3 := 0.0;
        p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
        END_IF;
    ELSE
        Error := TRUE; ErrorId := 16#4D01;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

// MC_MoveDirectPreparation: TwinCAT "direct" segments are absolute
// linear moves without blending lookahead.  Flattened to the same
// linear segment; BufferMode is accepted for signature parity.
FUNCTION_BLOCK MC_MoveDirectPreparation
VAR_INPUT
    Execute : BOOL;
    Position : ARRAY[0..15] OF LREAL;
    PositionCount : UDINT;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL := 0.0;
    Jerk : LREAL := 0.0;
    BufferMode : UDINT := 1;
END_VAR
VAR_IN_OUT
    PathData : UDINT;
END_VAR
VAR_OUTPUT
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    exPrev : BOOL;
    edge : BOOL;
    i : UDINT;
END_VAR
edge := Execute AND NOT exPrev; exPrev := Execute;
Error := FALSE; ErrorId := 0;
IF edge THEN
    p.HANDLE := PathData;
    IF PathData = 0 THEN
        p.OP := 0; p();
        IF p.STATUS = 0 THEN PathData := p.HANDLE; END_IF;
    END_IF;
    IF PathData <> 0 AND PositionCount >= 1 AND
       PositionCount <= 16 THEN
        FOR i := 0 TO 15 DO p.POS[i] := Position[i]; END_FOR;
        p.OP := 2; p.AUX := 1; p.AUX2 := PositionCount;
        p.F1 := Velocity; p.F2 := Acceleration; p.F3 := 0.0;
        p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
        END_IF;
    ELSE
        Error := TRUE; ErrorId := 16#4D01;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_DwellTimePreparation
VAR_INPUT
    Execute : BOOL;
    DwellTime : UDINT;   // milliseconds (TIME flattened)
    BufferMode : UDINT := 1;
END_VAR
VAR_IN_OUT
    PathData : UDINT;
END_VAR
VAR_OUTPUT
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    exPrev : BOOL;
    edge : BOOL;
END_VAR
edge := Execute AND NOT exPrev; exPrev := Execute;
Error := FALSE; ErrorId := 0;
IF edge THEN
    p.HANDLE := PathData;
    IF PathData = 0 THEN
        p.OP := 0; p();
        IF p.STATUS = 0 THEN PathData := p.HANDLE; END_IF;
    END_IF;
    IF PathData <> 0 THEN
        p.OP := 2; p.AUX := 3; p.AUX2 := 0;
        p.F1 := 0.0; p.F2 := 0.0; p.F3 := DwellTime;
        p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
        END_IF;
    ELSE
        Error := TRUE; ErrorId := 16#4D01;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_MovePath
VAR_INPUT
    Execute : BOOL;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
    PathData : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    mg : FB_McGroup;
    p : FB_Path;
    ax : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    issued : BOOL;
    t0 : LREAL;
    pathT : LREAL;
    lastNow : LREAL;
    blkWait : BOOL;
    dn : LREAL;
    i : UDINT;
    n : UDINT;
    axh : UDINT;
    startPos : ARRAY[0..15] OF LREAL;
END_VAR
mg.GRP := AxesGroup;
mg.OP := 10; mg();
AxesGroup := mg.GRP;
n := mg.NAXES;
exEdge := Execute AND NOT exPrev; exPrev := Execute;
Done := FALSE; Busy := FALSE;
CommandAborted := FALSE; Error := FALSE; ErrorId := 0;
IF exEdge THEN
    active := TRUE; issued := FALSE;
END_IF;
IF active THEN
    IF NOT issued THEN
        /* capture current positions as the path start */
        FOR i := 0 TO 15 DO
            startPos[i] := 0.0;
            IF i < n THEN
                axh := mg.AXES[i];
                ax.AXIS := axh; ax.OP := 4; ax.INDEX := 16#6064; ax();
                startPos[i] := DINT_TO_LREAL(ax.VALUE);
            END_IF;
        END_FOR;
        FOR i := 0 TO 15 DO p.POS[i] := startPos[i]; END_FOR;
        p.OP := 3; p.AUX := n; p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
            active := FALSE;
        ELSE
            PathData := p.HANDLE;
            /* bind the path to the group for trajectory readback */
            p.OP := 9; p.AUX := AxesGroup; p.HANDLE := PathData; p();
            t0 := p.NOW;
            pathT := 0.0; lastNow := p.NOW;
            /* switch members to CSP mode for 0x607A streaming */
            FOR i := 0 TO 15 DO
                IF i < n THEN
                    axh := mg.AXES[i];
                    ax.AXIS := axh; ax.OP := 5;
                    ax.INDEX := 16#6060; ax.VALUE := 8; ax();
                END_IF;
            END_FOR;
            issued := TRUE;
        END_IF;
    END_IF;
    IF issued THEN
        /* blocker query freezes path time while the path is held
           at an unreleased blocker gate */
        p.OP := 7; p.HANDLE := PathData; p();
        blkWait := p.AUX = 1;
        dn := p.NOW - lastNow; lastNow := p.NOW;
        IF NOT blkWait THEN pathT := pathT + dn; END_IF;
        p.OP := 4; p.F1 := pathT; p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
            active := FALSE; CommandAborted := TRUE;
        ELSE
            FOR i := 0 TO 15 DO
                IF i < n THEN
                    axh := mg.AXES[i];
                    ax.AXIS := axh; ax.OP := 5;
                    ax.INDEX := 16#607A;
                    ax.VALUE := LREAL_TO_DINT(p.POS[i]); ax();
                END_IF;
            END_FOR;
            IF p.DONE = 1 THEN
                Done := TRUE; active := FALSE;
            END_IF;
        END_IF;
    END_IF;
    Busy := active;
END_IF;
END_FUNCTION_BLOCK

// MC_MoveCircularAbsolutePreparation: appends a circular segment.
// CircMode flattens MC_CIRC_MODE (1=mcCircModeCenter: AuxPoint is
// the circle center and must be equidistant from start/end;
// 2=mcCircModeBorder: AuxPoint lies on the circle). The circle is
// evaluated in the plane of position entries 0/1; entries >=2
// interpolate linearly. PathChoice: 1=counter-clockwise,
// 2=clockwise. TransitionMode/TransitionParameter/InvokeId/
// DynamicConstraints are accepted for signature parity but not
// modeled.
FUNCTION_BLOCK MC_MoveCircularAbsolutePreparation
VAR_INPUT
    Execute : BOOL;
    CircMode : UDINT;
    AuxPoint : ARRAY[0..15] OF LREAL;
    AuxPointCount : UDINT;
    EndPoint : ARRAY[0..15] OF LREAL;
    EndPointCount : UDINT;
    PathChoice : UDINT := 1;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL := 0.0;
    Jerk : LREAL := 0.0;
    BufferMode : UDINT := 1;
    TransitionMode : UDINT := 0;
    TransitionParameter : ARRAY[0..15] OF LREAL;
    TransitionParameterCount : UDINT := 0;
    InvokeId : UDINT := 0;
END_VAR
VAR_IN_OUT
    PathData : UDINT;
END_VAR
VAR_OUTPUT
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    exPrev : BOOL;
    edge : BOOL;
    i : UDINT;
END_VAR
edge := Execute AND NOT exPrev; exPrev := Execute;
Error := FALSE; ErrorId := 0;
IF edge THEN
    p.HANDLE := PathData;
    IF PathData = 0 THEN
        p.OP := 0; p();
        IF p.STATUS = 0 THEN PathData := p.HANDLE; END_IF;
    END_IF;
    IF PathData <> 0 AND EndPointCount >= 1 AND EndPointCount <= 16
       AND CircMode >= 1 AND CircMode <= 2 THEN
        FOR i := 0 TO 15 DO
            p.POS[i] := EndPoint[i];
            p.AUXP[i] := AuxPoint[i];
        END_FOR;
        p.OP := 6; p.AUX := CircMode; p.FLAGS := AuxPointCount;
        p.AUX2 := EndPointCount; p.F3 := PathChoice;
        p.F1 := Velocity; p.F2 := Acceleration;
        p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
        END_IF;
    ELSE
        Error := TRUE; ErrorId := 16#4D01;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

// MC_MoveBlendingPreparation: appends a junction transition segment.
// TwinCAT uses it to program velocity blending between segments;
// this implementation holds the path end position for
// TransitionParameter[0] milliseconds (real corner-cutting/lookahead
// blending is not modeled — documented deviation). Velocity/
// Acceleration/Deceleration/Jerk are accepted for signature parity.
FUNCTION_BLOCK MC_MoveBlendingPreparation
VAR_INPUT
    Execute : BOOL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL := 0.0;
    Jerk : LREAL := 0.0;
    BufferMode : UDINT := 1;
    TransitionMode : UDINT := 0;
    TransitionParameter : ARRAY[0..15] OF LREAL;
    TransitionParameterCount : UDINT := 0;
    InvokeId : UDINT := 0;
END_VAR
VAR_IN_OUT
    PathData : UDINT;
END_VAR
VAR_OUTPUT
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    exPrev : BOOL;
    edge : BOOL;
END_VAR
edge := Execute AND NOT exPrev; exPrev := Execute;
Error := FALSE; ErrorId := 0;
IF edge THEN
    p.HANDLE := PathData;
    IF PathData = 0 THEN
        p.OP := 0; p();
        IF p.STATUS = 0 THEN PathData := p.HANDLE; END_IF;
    END_IF;
    IF PathData <> 0 THEN
        p.OP := 2; p.AUX := 4; p.AUX2 := 0;
        p.F1 := 0.0; p.F2 := 0.0;
        IF TransitionParameterCount >= 1 THEN
            p.F3 := TransitionParameter[0];
        ELSE
            p.F3 := 0.0;
        END_IF;
        p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
        END_IF;
    ELSE
        Error := TRUE; ErrorId := 16#4D01;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

// MC_BlockerPreparation: appends a blocker gate at the current path
// junction. The path holds position there during MC_MovePath until
// MC_ReleaseBlocker opens the gate. BufferMode is accepted for
// signature parity.
FUNCTION_BLOCK MC_BlockerPreparation
VAR_INPUT
    Execute : BOOL;
    BlockerId : UDINT;
    BufferMode : UDINT := 1;
END_VAR
VAR_IN_OUT
    PathData : UDINT;
END_VAR
VAR_OUTPUT
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    exPrev : BOOL;
    edge : BOOL;
END_VAR
edge := Execute AND NOT exPrev; exPrev := Execute;
Error := FALSE; ErrorId := 0;
IF edge THEN
    p.HANDLE := PathData;
    IF PathData = 0 THEN
        p.OP := 0; p();
        IF p.STATUS = 0 THEN PathData := p.HANDLE; END_IF;
    END_IF;
    IF PathData <> 0 AND BlockerId > 0 THEN
        p.OP := 2; p.AUX := 5; p.AUX2 := 0;
        p.F1 := 0.0; p.F2 := 0.0; p.F3 := BlockerId;
        p.HANDLE := PathData; p();
        IF p.STATUS <> 0 THEN
            Error := TRUE;
            IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
        END_IF;
    ELSE
        Error := TRUE; ErrorId := 16#4D01;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

// MC_ReleaseBlocker: opens every blocker gate with the given id on
// the next eval. The TwinCAT signature carries only AxesGroup; the
// flattened implementation releases by BlockerId across all path
// objects (blocker ids are user-unique per machine).
FUNCTION_BLOCK MC_ReleaseBlocker
VAR_INPUT
    Execute : BOOL;
    BlockerId : UDINT;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
mg.GRP := AxesGroup; mg.OP := 1; mg();
AxesGroup := mg.GRP;
IF edge THEN
    p.OP := 8; p.F1 := BlockerId; p();
    IF p.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok;
IF edge AND p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
END_FUNCTION_BLOCK

// MC_GroupReadBlockerStatus: reports whether the most recent path
// eval is held at an unreleased blocker. The group linkage is
// approximated — the service reports the last-evaluated path's gate
// state.
FUNCTION_BLOCK MC_GroupReadBlockerStatus
VAR_INPUT
    Enable : BOOL;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Busy : BOOL;
    Blocked : BOOL;
    BlockerId : UDINT;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    p : FB_Path;
    mg : FB_McGroup;
END_VAR
Valid := FALSE; Busy := FALSE; Blocked := FALSE;
BlockerId := 0; Error := FALSE; ErrorId := 0;
mg.GRP := AxesGroup; mg.OP := 6; mg();
IF Enable AND mg.STATUS = 0 THEN
    Valid := TRUE;
    p.OP := 7; p();
    IF p.STATUS = 0 THEN
        Blocked := p.AUX = 1;
        BlockerId := LREAL_TO_DINT(p.F1);
    ELSE
        Error := TRUE;
        IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

// MC_GroupReadTrajectoryInformation: reads the current and next
// path segments for the group the executing MC_MovePath is bound
// to. Entries flatten MC_TRAJECTORY_ENTRY: InvokeId is approximated
// by the 1-based segment index, Active marks blend/blocker
// segments, RemainingDuration is seconds and
// RemainingCartesianLength is dominant-axis user units, all as of
// the most recent path eval.
FUNCTION_BLOCK MC_GroupReadTrajectoryInformation
VAR_INPUT
    Enable : BOOL;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    EntryCount : UDINT;
    TrajInvokeId : ARRAY[0..9] OF UDINT;
    TrajActive : ARRAY[0..9] OF BOOL;
    TrajRemLen : ARRAY[0..9] OF LREAL;
    TrajRemDur : ARRAY[0..9] OF LREAL;
END_VAR
VAR
    p : FB_Path;
    mg : FB_McGroup;
    i : UDINT;
END_VAR
Valid := FALSE; Busy := FALSE; Error := FALSE; ErrorId := 0;
EntryCount := 0;
mg.GRP := AxesGroup; mg.OP := 6; mg();
IF Enable AND mg.STATUS = 0 THEN
    Valid := TRUE;
    p.OP := 10; p.AUX := AxesGroup; p();
    IF p.STATUS = 0 THEN
        EntryCount := p.NSEG;
        FOR i := 0 TO 9 DO
            TrajInvokeId[i] := p.TRID[i];
            TrajActive[i] := p.TACT[i] = 1;
            TrajRemLen[i] := p.AUXP[i];
            TrajRemDur[i] := p.POS[i];
        END_FOR;
    ELSE
        Error := TRUE;
        IF p.STATUS < 0 THEN ErrorId := 0 - p.STATUS; END_IF;
    END_IF;
END_IF;
END_FUNCTION_BLOCK
