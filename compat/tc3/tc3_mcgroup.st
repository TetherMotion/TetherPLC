// Tc3_McCoordinatedMotion — MC Group administration surface
// (TF5410 collision-avoidance / coordinated-motion bundle).
//
// This tranche implements the group bookkeeping FBs.  Motion
// execution (MC_MovePath, preparations, blockers, dwell), track
// following (MC_ActivateTrack, MC_TrackConveyorBelt,
// MC_ReadTrackPositions), collision-avoidance moves
// (MC_MoveAbsoluteCA etc.) and axis compensation
// (MC_RegisterCompensation, MC_TransformPosition) require the
// trajectory engine and are deferred.
//
// Flattened types (the ST dialect has no STRUCT/REFERENCE support):
//   AXES_GROUP_REF        -> AxesGroup : UDINT group handle (1..8)
//   AXIS_REF              -> Axis : UDINT axis handle
//   IDENT_IN_GROUP_REF    -> IdentInGroup : UDINT
//                            (UDINT_TO_IDENTINGROUP is the identity)
//   MC_COORD_REF          -> CoordTransform : UDINT
//   ECmSetpointSource     -> SetpointSource : UDINT
//                            (1=Group, 2=MCS, 3=ACS)
//   MC_LREAL              -> LREAL
//
// Group states (approximation of TwinCAT mcGroupState*):
//   0 empty, 1 disabled, 2 standby, 3 moving, 4 error-disabled,
//   5 group-error.  The service is synchronous, so Busy stays
//   FALSE and Done/Error pulse on the execute scan.
//   ErrorId carries the service status magnitude.

// ---- UDINT_TO_IDENTINGROUP: identity conversion ----
FUNCTION UDINT_TO_IDENTINGROUP : UDINT
VAR_INPUT
    InVal : UDINT;
END_VAR
UDINT_TO_IDENTINGROUP := InVal;
END_FUNCTION

// ---- shared command skeleton: rising-edge Execute -> FB_McGroup --
// Each public FB owns one FB_McGroup instance `mg`.

FUNCTION_BLOCK MC_AddAxisToGroup
VAR_INPUT
    Execute : BOOL;
    IdentInGroup : UDINT;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 2; mg.AXIS := Axis; mg.IDENT := IdentInGroup; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_RemoveAxisFromGroup
VAR_INPUT
    Execute : BOOL;
    IdentInGroup : UDINT;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 3; mg.AXIS := Axis; mg.IDENT := IdentInGroup; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_UngroupAllAxes
VAR_INPUT
    Execute : BOOL;
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
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
IF edge THEN
    mg.OP := 4; mg.GRP := AxesGroup; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_GroupEnable
VAR_INPUT
    Execute : BOOL;
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
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 5; mg.ENABLE := 1; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_GroupDisable
VAR_INPUT
    Execute : BOOL;
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
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 5; mg.ENABLE := 0; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_GroupReset
VAR_INPUT
    Execute : BOOL;
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
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 5; mg.ENABLE := 2; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

// ---- level-triggered status readers (Enable input) ----

FUNCTION_BLOCK MC_GroupReadStatus
VAR_INPUT
    Enable : BOOL;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    Busy : BOOL;
    GroupMoving : BOOL;
    GroupHoming : BOOL;
    GroupErrorStop : BOOL;
    GroupNotReady : BOOL;
    GroupStandby : BOOL;
    GroupStopping : BOOL;
    GroupDisabled : BOOL;
    AllAxesStanding : BOOL;
    InPosition : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    mg : FB_McGroup;
END_VAR
Valid := FALSE; Busy := FALSE; Error := FALSE; ErrorId := 0;
GroupMoving := FALSE; GroupHoming := FALSE; GroupErrorStop := FALSE;
GroupNotReady := FALSE; GroupStandby := FALSE; GroupStopping := FALSE;
GroupDisabled := FALSE; AllAxesStanding := FALSE; InPosition := FALSE;
IF Enable THEN
    mg.OP := 6; mg.GRP := AxesGroup; mg();
    IF mg.STATUS = 0 THEN
        Valid := TRUE;
        GroupMoving := mg.STATE = 3;
        GroupStandby := mg.STATE = 2;
        GroupDisabled := mg.STATE = 0 OR mg.STATE = 1 OR
                        mg.STATE = 4;
        AllAxesStanding := mg.STATE = 2 OR mg.STATE = 3;
        /* GroupHoming/GroupErrorStop/GroupNotReady/GroupStopping/
           InPosition need the motion engine; always FALSE here. */
    ELSE
        Error := TRUE;
        IF mg.STATUS < 0 THEN ErrorId := (0 - mg.STATUS); END_IF;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_GroupReadError
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
    GroupErrorId : UDINT;
END_VAR
VAR
    mg : FB_McGroup;
END_VAR
Valid := FALSE; Busy := FALSE; Error := FALSE;
ErrorId := 0; GroupErrorId := 0;
IF Enable THEN
    mg.OP := 6; mg.GRP := AxesGroup; mg();
    IF mg.STATUS = 0 THEN
        Valid := TRUE;
        GroupErrorId := mg.ERRID;
    ELSE
        Error := TRUE;
        IF mg.STATUS < 0 THEN ErrorId := (0 - mg.STATUS); END_IF;
    END_IF;
END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_GroupSetOverride
VAR_INPUT
    Execute : BOOL;
    Enable : BOOL;
    VelFactor : LREAL;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    OverrideActual : LREAL;
END_VAR
VAR
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
    ovr : LREAL;
END_VAR
ok := FALSE; ovr := 1.0;
edge := Execute AND NOT exPrev;
IF Enable THEN ovr := VelFactor; END_IF;
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 7; mg.OVR := ovr; mg();
    IF mg.STATUS = 0 THEN
        ok := TRUE;
        OverrideActual := mg.OVRACT;
    END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_GroupSetSetpointSource
VAR_INPUT
    Execute : BOOL;
    SetpointSource : UDINT;
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
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
    src : UDINT;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
src := SetpointSource - 1; /* 1/2/3 -> 0/1/2 */
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 8; mg.SOURCE := src; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK

FUNCTION_BLOCK MC_SetCoordinateTransform
VAR_INPUT
    Execute : BOOL;
    CoordTransform : UDINT;
END_VAR
VAR_IN_OUT
    AxesGroup : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    mg : FB_McGroup;
    exPrev : BOOL;
    edge : BOOL;
    ok : BOOL;
    act : BOOL;
END_VAR
ok := FALSE;
edge := Execute AND NOT exPrev;
IF edge THEN
    mg.OP := 1; mg.GRP := AxesGroup; mg();
    AxesGroup := mg.GRP;
    mg.OP := 9; mg.COORD := CoordTransform; mg();
    IF mg.STATUS = 0 THEN ok := TRUE; act := TRUE; END_IF;
END_IF;
exPrev := Execute;
Done := ok; Busy := FALSE; CommandAborted := FALSE;
Active := act;
Error := edge AND NOT ok AND mg.STATUS <> 0;
IF edge AND mg.STATUS < 0 THEN ErrorId := 0 - mg.STATUS; END_IF;
END_FUNCTION_BLOCK
