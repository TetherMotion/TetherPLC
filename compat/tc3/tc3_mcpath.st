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
            t0 := p.NOW;
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
        p.OP := 4; p.F1 := p.NOW - t0; p.HANDLE := PathData; p();
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
