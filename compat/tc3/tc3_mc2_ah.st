// Tc3_MC2_AdvancedHoming — user-programmable homing sequences.
//
// All 16 documented APIs over the builtin FB_MotionAxis
// (tplc-motion service), flattened to this dialect:
//
//   AXIS_REF                -> Axis : UDINT (FB_MotionAxis handle)
//   MC_HomingParameter      -> 15 VAR_IN_OUT members P_* (Stored flag
//                             plus the NC soft-limit/lag/vel-diff and
//                             drive torque Stored/Actual fields; the
//                             sim exposes no such drive params so the
//                             fields are pass-through bookkeeping)
//   MC_Ref_Signal_Ref       -> RefLevel : BOOL (the .Level member;
//                             SignalSource/TouchProbe have no sim
//                             equivalent and are dropped)
//   MC_Home_Direction       -> Direction : UDINT
//                             (1=mcPositiveDirection, 3=mcNegative-
//                             Direction, 5=mcSwitchPositive,
//                             7=mcSwitchNegative)
//   MC_Switch_Mode          -> SwitchMode : UDINT
//                             (1=mcOn, 2=mcOff, 3=mcRisingEdge,
//                             4=mcFallingEdge, 5=mcEdgeSwitchPositive,
//                             6=mcEdgeSwitchNegative,
//                             11=mcRisingEdgeInverse,
//                             12=mcFallingEdgeInverse)
//   MC_BufferMode           -> BufferMode : UDINT (not implemented,
//                             accepted and ignored like TwinCAT)
//   ST_Home_Options         -> (empty struct, no member)
//   ST_Home_Options2        -> OptDisableDriveAccess : BOOL
//   ST_Home_Options3        -> OptDisableDriveAccess +
//                             OptInstantLagReduction : BOOL
//   ST_Home_Options4        -> OptDisableDriveAccess +
//                             OptEnableLagErrorDetection : BOOL
//   E_HomingErrorCodes      -> ErrorID values 16#4B90..16#4B98
//
// Motion model: step FBs run a PV-mode (0x6060=3) velocity search at
// Velocity user units/s (0x60FF), poll the OP6 snapshot, and evaluate
// the switch/block condition each scan.  On detection the axis is
// stopped (0x60FF=0), allowed to settle, then the position is assigned
// via a direct 0x6064 write (MC_SetPosition equivalent) unless the FB
// is a "..Detection" variant, which instead returns RecordedPosition.
// The sensor "signal" is the user-fed RefLevel BOOL; the sim has no
// physical switch/encoder inputs.  Block detection uses the
// velocity-undershoot criterion (|VEL_ACTUAL| below
// DetectionVelocityLimit for DetectionVelocityTime); the travel-limit
// clamp added to the velocity modes makes a fixed stop produce that
// undershoot.  TorqueLimit/LagLimit/Acceleration/Deceleration/Jerk are
// accepted but inert (no torque or jerk model in the sim; PV ramps at
// the axis' configured accel/decel).
//
// Internal engine: AH_Run.  Kind codes: 1 absSwitch, 2 absSwitchDet,
// 3 block, 4 blockDet, 5 blockLag, 6 blockLagDet, 7 limitSwitch,
// 8 limitSwitchDet, 9 refPulse, 10 refPulseDet, 11 flyingRefPulse,
// 12 flyingSwitch, 20 distance move (FinishHoming).

// ---- AH_Run: shared homing sequence engine (internal helper) ----
FUNCTION_BLOCK AH_Run
VAR_INPUT
    Execute : BOOL;
    Axis : UDINT;
    Kind : UDINT;
    Direction : UDINT;
    SwitchMode : UDINT;
    RefLevel : BOOL;
    Velocity : LREAL;
    SetPosEn : BOOL;
    SetPosition : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    DetVelLimit : LREAL;
    DetVelTime : TIME;
    LagLimit : LREAL;
    PosLimSw : BOOL;
    NegLimSw : BOOL;
    Distance : LREAL;
    ParamIn : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RecordedPosition : LREAL;
    ParamOut : BOOL;
END_VAR
VAR
    ax : FB_MotionAxis;
    clk : TON;
    exPrev : BOOL;
    phase : UDINT;          // 0 idle, 1 search, 2 settle, 3 distmove, 4 observe
    refPrev : BOOL;
    moveDir : DINT;
    startPos : DINT;
    startNs : LINT;
    detNs : LINT;           // undershoot start, -1 = not undershooting
    velCmd : DINT;
    pendSet : BOOL;
    recPos : LREAL;
    distGoal : DINT;
    nowNs : LINT;
    pos : DINT;
    vel : DINT;
    cond : BOOL;
    isSwitch : BOOL;
    isBlock : BOOL;
    isDet : BOOL;
    edge : BOOL;
    pStored : BOOL;
END_VAR
    clk(IN := TRUE, PT := T#1000000h);
    ax.AXIS := Axis;
    ax.OP := 3; ax();
    ax.OP := 6; ax();
    Done := FALSE; Busy := FALSE; Active := FALSE;
    CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    RecordedPosition := 0.0;
    nowNs := TIME_TO_LINT(clk.ET);
    pos := ax.POS_ACTUAL;
    vel := ax.VEL_ACTUAL;
    pStored := ParamIn;
    isSwitch := (Kind >= 1) AND (Kind <= 2) OR (Kind >= 7) AND (Kind <= 10);
    isBlock := (Kind >= 3) AND (Kind <= 6);
    isDet := (Kind = 2) OR (Kind = 4) OR (Kind = 6) OR (Kind = 8)
             OR (Kind = 10);
    edge := Execute AND NOT exPrev;
    exPrev := Execute;

    IF phase = 0 THEN
        IF edge THEN
            // ---- validation ----
            IF Kind <> 11 AND Kind <> 12 AND Velocity <= 0.0 THEN
                Error := TRUE; ErrorID := 16#4B93;   // MC_HOMINGERROR_MODE
            ELSIF Kind <= 12 AND DistanceLimit < 0.0 THEN
                Error := TRUE; ErrorID := 16#4B96;   // DISTANCELIMIT
            ELSIF (Kind >= 1) AND (Kind <= 10)
                  AND Direction <> 1 AND Direction <> 3
                  AND Direction <> 5 AND Direction <> 7 THEN
                Error := TRUE; ErrorID := 16#4B91;   // DIRECTION
            ELSIF isSwitch AND SwitchMode <> 1 AND SwitchMode <> 2
                  AND SwitchMode <> 3 AND SwitchMode <> 4
                  AND SwitchMode <> 5 AND SwitchMode <> 6
                  AND SwitchMode <> 11 AND SwitchMode <> 12 THEN
                Error := TRUE; ErrorID := 16#4B92;   // SWITCHMODE
            ELSE
                // ---- initialise sequence ----
                IF Kind = 20 THEN
                    // distance move (FinishHoming backoff)
                    IF Distance = 0.0 THEN
                        pStored := FALSE;
                        Done := TRUE;
                    ELSE
                        moveDir := 1;
                        IF Distance < 0.0 THEN moveDir := -1; END_IF;
                        startPos := pos; startNs := nowNs; detNs := -1;
                        distGoal := pos + LREAL_TO_DINT(Distance);
                        refPrev := RefLevel;
                        pStored := TRUE;
                        velCmd := LREAL_TO_DINT(DINT_TO_LREAL(moveDir)
                                                * Velocity);
                        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 3; ax();
                        ax.OP := 5; ax.INDEX := 16#60FF;
                        ax.VALUE := velCmd; ax();
                        ax.OP := 5; ax.INDEX := 16#6040;
                        ax.VALUE := 16#000F; ax();
                        IF ax.STATUS = 0 THEN
                            phase := 3; Active := TRUE;
                        ELSE
                            Error := TRUE; ErrorID := ax.ERRID;
                            pStored := FALSE;
                        END_IF;
                    END_IF;
                ELSIF Kind = 11 OR Kind = 12 THEN
                    // passive observation, no motion commanded
                    startPos := pos; startNs := nowNs; detNs := -1;
                    refPrev := RefLevel;
                    pStored := TRUE;
                    phase := 4; Active := TRUE;
                ELSE
                    // search move direction
                    moveDir := 1;
                    IF Direction = 3 THEN moveDir := -1; END_IF;
                    IF Direction = 5 THEN
                        moveDir := 1;
                        IF RefLevel THEN moveDir := -1; END_IF;
                    END_IF;
                    IF Direction = 7 THEN
                        moveDir := -1;
                        IF RefLevel THEN moveDir := 1; END_IF;
                    END_IF;
                    startPos := pos; startNs := nowNs; detNs := -1;
                    refPrev := RefLevel;
                    pStored := TRUE;
                    velCmd := LREAL_TO_DINT(DINT_TO_LREAL(moveDir)
                                            * Velocity);
                    ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 3; ax();
                    ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := velCmd; ax();
                    ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 16#000F; ax();
                    IF ax.STATUS = 0 THEN
                        phase := 1; Active := TRUE;
                    ELSE
                        Error := TRUE; ErrorID := ax.ERRID;
                        pStored := FALSE;
                    END_IF;
                END_IF;
            END_IF;
        END_IF;

    ELSIF phase = 1 THEN
        // ---- search: abort limits then detection ----
        IF (TimeLimit > T#0s) AND
           (nowNs - startNs > TIME_TO_LINT(TimeLimit)) THEN
            ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
            phase := 0; CommandAborted := TRUE;
        ELSIF ABS(DINT_TO_LREAL(pos - startPos)) > DistanceLimit THEN
            ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
            phase := 0; CommandAborted := TRUE;
        ELSIF (PosLimSw AND moveDir > 0) OR (NegLimSw AND moveDir < 0) THEN
            ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
            phase := 0; CommandAborted := TRUE;
        ELSE
            cond := FALSE;
            IF isSwitch THEN
                // MC_Switch_Mode evaluation; inverse variants share
                // the physical edge (approach side is absorbed by
                // the position assignment)
                IF SwitchMode = 1 THEN
                    cond := RefLevel;
                ELSIF SwitchMode = 2 THEN
                    cond := NOT RefLevel;
                ELSIF SwitchMode = 3 OR SwitchMode = 11 THEN
                    cond := RefLevel AND NOT refPrev;
                ELSIF SwitchMode = 4 OR SwitchMode = 12 THEN
                    cond := NOT RefLevel AND refPrev;
                ELSIF SwitchMode = 5 THEN
                    IF moveDir > 0 THEN
                        cond := RefLevel AND NOT refPrev;
                    ELSE
                        cond := NOT RefLevel AND refPrev;
                    END_IF;
                ELSIF SwitchMode = 6 THEN
                    IF moveDir > 0 THEN
                        cond := NOT RefLevel AND refPrev;
                    ELSE
                        cond := RefLevel AND NOT refPrev;
                    END_IF;
                END_IF;
            ELSIF isBlock THEN
                // velocity undershoot below DetVelLimit sustained
                // for DetVelTime; lag-based additionally requires
                // the velocity deficit proxy above LagLimit
                IF ABS(vel) < LREAL_TO_DINT(DetVelLimit) THEN
                    IF detNs < 0 THEN detNs := nowNs; END_IF;
                    IF nowNs - detNs >= TIME_TO_LINT(DetVelTime)
                       AND (Kind < 5 OR
                            ABS(velCmd - vel) > LREAL_TO_DINT(LagLimit)) THEN
                        cond := TRUE;
                    END_IF;
                ELSE
                    detNs := -1;
                END_IF;
            END_IF;
            refPrev := RefLevel;
            IF cond THEN
                ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
                recPos := DINT_TO_LREAL(pos);
                pendSet := SetPosEn AND NOT isDet;
                phase := 2; Active := TRUE;
            ELSE
                Active := TRUE; Busy := TRUE;
            END_IF;
        END_IF;

    ELSIF phase = 2 THEN
        // ---- settle: wait for the velocity ramp-down, then assign
        // the position (or report RecordedPosition) ----
        Active := TRUE; Busy := TRUE;
        IF ABS(vel) <= 1 THEN
            IF pendSet THEN
                ax.OP := 5; ax.INDEX := 16#6064;
                ax.VALUE := LREAL_TO_DINT(SetPosition); ax();
                IF ax.STATUS <> 0 THEN
                    Error := TRUE; ErrorID := ax.ERRID;
                    phase := 0;
                ELSE
                    phase := 0; Done := TRUE;
                    IF Kind = 20 THEN pStored := FALSE; END_IF;
                END_IF;
            ELSE
                IF isDet THEN RecordedPosition := recPos; END_IF;
                phase := 0; Done := TRUE;
                IF Kind = 20 THEN pStored := FALSE; END_IF;
            END_IF;
        END_IF;

    ELSIF phase = 3 THEN
        // ---- distance move: stop at the goal ----
        Active := TRUE; Busy := TRUE;
        IF (moveDir > 0 AND pos >= distGoal)
           OR (moveDir < 0 AND pos <= distGoal) THEN
            ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
            phase := 2;   // reuse settle; pendSet stays FALSE
            pendSet := FALSE;
        END_IF;
        IF (TimeLimit > T#0s) AND
           (nowNs - startNs > TIME_TO_LINT(TimeLimit)) THEN
            ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
            phase := 0; CommandAborted := TRUE; pStored := FALSE;
        END_IF;

    ELSIF phase = 4 THEN
        // ---- passive observation: no motion commanded ----
        Active := TRUE; Busy := TRUE;
        cond := FALSE;
        IF Kind = 11 THEN
            cond := RefLevel AND NOT refPrev;   // encoder zero pulse
        ELSE
            // flying switch: same MC_Switch_Mode evaluation,
            // direction taken as +1 (no search direction)
            IF SwitchMode = 1 THEN cond := RefLevel;
            ELSIF SwitchMode = 2 THEN cond := NOT RefLevel;
            ELSIF SwitchMode = 3 OR SwitchMode = 11 THEN
                cond := RefLevel AND NOT refPrev;
            ELSIF SwitchMode = 4 OR SwitchMode = 12 THEN
                cond := NOT RefLevel AND refPrev;
            ELSIF SwitchMode = 5 THEN
                cond := RefLevel AND NOT refPrev;
            ELSIF SwitchMode = 6 THEN
                cond := NOT RefLevel AND refPrev;
            END_IF;
        END_IF;
        refPrev := RefLevel;
        IF (TimeLimit > T#0s) AND
           (nowNs - startNs > TIME_TO_LINT(TimeLimit)) THEN
            phase := 0; CommandAborted := TRUE; pStored := FALSE;
        ELSIF ABS(DINT_TO_LREAL(pos - startPos)) > DistanceLimit THEN
            phase := 0; CommandAborted := TRUE; pStored := FALSE;
        ELSIF cond THEN
            ax.OP := 5; ax.INDEX := 16#6064;
            ax.VALUE := LREAL_TO_DINT(SetPosition); ax();
            IF ax.STATUS = 0 THEN
                phase := 0; Done := TRUE; pStored := FALSE;
            ELSE
                phase := 0; Error := TRUE; ErrorID := ax.ERRID;
                pStored := FALSE;
            END_IF;
        END_IF;
    END_IF;
    ParamOut := pStored;
END_FUNCTION_BLOCK

// ======================= finalizing FBs =======================

// MC_FinishHoming — completes a homing sequence: optional Distance
// backoff move at Velocity, then restores the stored parameters.
FUNCTION_BLOCK MC_FinishHoming
VAR_INPUT
    Execute : BOOL;
    Distance : LREAL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis; eng.Kind := 20;
    eng.Direction := 1; eng.SwitchMode := 0; eng.RefLevel := FALSE;
    eng.Velocity := Velocity; eng.SetPosEn := FALSE;
    eng.SetPosition := 0.0;
    eng.TimeLimit := T#0s; eng.DistanceLimit := -1.0;
    eng.DetVelLimit := 0.0; eng.DetVelTime := T#0s; eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE; eng.NegLimSw := FALSE;
    eng.Distance := Distance;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// MC_HomeDirect — completes homing and assigns the axis position
// directly (no motion).  Axis leaves the homing state.
FUNCTION_BLOCK MC_HomeDirect
VAR_INPUT
    Execute : BOOL;
    SetPosition : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    exPrev : BOOL;
END_VAR
    ax.AXIS := Axis;
    Done := FALSE; Busy := FALSE; Active := FALSE;
    CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        Active := TRUE;
        ax.OP := 5; ax.INDEX := 16#6064;
        ax.VALUE := LREAL_TO_DINT(SetPosition); ax();
        IF ax.STATUS = 0 THEN
            Done := TRUE; Active := FALSE;
            P_Stored := FALSE;
        ELSE
            Error := TRUE; ErrorID := ax.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// MC_AbortHoming — aborts a homing sequence, stops any search move
// and restores the stored parameters.
FUNCTION_BLOCK MC_AbortHoming
VAR_INPUT
    Execute : BOOL;
    OptDisableDriveAccess : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
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
END_VAR
    ax.AXIS := Axis;
    Done := FALSE; Busy := FALSE;
    CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ax.OP := 5; ax.INDEX := 16#60FF; ax.VALUE := 0; ax();
        ax.OP := 5; ax.INDEX := 16#6060; ax.VALUE := 0; ax();
        P_Stored := FALSE;
        Done := (ax.STATUS = 0);
        IF ax.STATUS <> 0 THEN Error := TRUE; ErrorID := ax.ERRID; END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ==================== referencing (passive) FBs ====================

// MC_AbortPassiveHoming — aborts passive homing.  The sim keeps no
// NC-side passive state, so this reports Done immediately.
FUNCTION_BLOCK MC_AbortPassiveHoming
VAR_INPUT
    Execute : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    exPrev : BOOL;
END_VAR
    Done := FALSE; Busy := FALSE; Active := FALSE;
    CommandAborted := FALSE; Error := FALSE; ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        Done := TRUE;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// MC_StepReferenceFlyingRefPulse — latch the position when the
// encoder zero pulse (RefLevel rising edge) arrives during an
// externally driven move, then assign SetPosition.
FUNCTION_BLOCK MC_StepReferenceFlyingRefPulse
VAR_INPUT
    Execute : BOOL;
    RefLevel : BOOL;
    SetPosition : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    BufferMode : UDINT;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis; eng.Kind := 11;
    eng.Direction := 1; eng.SwitchMode := 0; eng.RefLevel := RefLevel;
    eng.Velocity := 0.0; eng.SetPosEn := TRUE;
    eng.SetPosition := SetPosition;
    eng.TimeLimit := TimeLimit; eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0; eng.DetVelTime := T#0s; eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE; eng.NegLimSw := FALSE; eng.Distance := 0.0;
    eng.ParamIn := FALSE;
    eng();
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// MC_StepReferenceFlyingSwitch — same as FlyingRefPulse but the
// trigger is a MC_Switch_Mode condition on RefLevel.
FUNCTION_BLOCK MC_StepReferenceFlyingSwitch
VAR_INPUT
    Execute : BOOL;
    SwitchMode : UDINT;
    RefLevel : BOOL;
    SetPosition : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    BufferMode : UDINT;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis; eng.Kind := 12;
    eng.Direction := 1; eng.SwitchMode := SwitchMode;
    eng.RefLevel := RefLevel;
    eng.Velocity := 0.0; eng.SetPosEn := TRUE;
    eng.SetPosition := SetPosition;
    eng.TimeLimit := TimeLimit; eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0; eng.DetVelTime := T#0s; eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE; eng.NegLimSw := FALSE; eng.Distance := 0.0;
    eng.ParamIn := FALSE;
    eng();
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// ========================= step FBs =========================

// MC_StepAbsoluteSwitch — searches for an absolutely positioned external switch
FUNCTION_BLOCK MC_StepAbsoluteSwitch
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    SwitchMode : UDINT;
    RefLevel : BOOL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    SetPosition : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    PositiveLimitSwitch : BOOL;
    NegativeLimitSwitch : BOOL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptEnableLagErrorDetection : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 1;
    eng.Direction := Direction;
    eng.SwitchMode := SwitchMode;
    eng.RefLevel := RefLevel;
    eng.Velocity := Velocity;
    eng.SetPosEn := TRUE;
    eng.SetPosition := SetPosition;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0;
    eng.DetVelTime := T#0s;
    eng.LagLimit := 0.0;
    eng.PosLimSw := PositiveLimitSwitch;
    eng.NegLimSw := NegativeLimitSwitch;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// MC_StepAbsoluteSwitchDetection — absolute switch search; returns RecordedPosition
FUNCTION_BLOCK MC_StepAbsoluteSwitchDetection
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    SwitchMode : UDINT;
    RefLevel : BOOL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    PositiveLimitSwitch : BOOL;
    NegativeLimitSwitch : BOOL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptEnableLagErrorDetection : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RecordedPosition : LREAL;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 2;
    eng.Direction := Direction;
    eng.SwitchMode := SwitchMode;
    eng.RefLevel := RefLevel;
    eng.Velocity := Velocity;
    eng.SetPosEn := FALSE;
    eng.SetPosition := 0.0;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0;
    eng.DetVelTime := T#0s;
    eng.LagLimit := 0.0;
    eng.PosLimSw := PositiveLimitSwitch;
    eng.NegLimSw := NegativeLimitSwitch;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
    RecordedPosition := eng.RecordedPosition;
END_FUNCTION_BLOCK

// MC_StepBlock — search for a mechanical fixed stop (velocity undershoot)
FUNCTION_BLOCK MC_StepBlock
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    SetPosition : LREAL;
    DetectionVelocityLimit : LREAL;
    DetectionVelocityTime : TIME;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    TorqueTolerance : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptInstantLagReduction : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 3;
    eng.Direction := Direction;
    eng.SwitchMode := 0;
    eng.RefLevel := FALSE;
    eng.Velocity := Velocity;
    eng.SetPosEn := TRUE;
    eng.SetPosition := SetPosition;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := DetectionVelocityLimit;
    eng.DetVelTime := DetectionVelocityTime;
    eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// MC_StepBlockDetection — block search; returns RecordedPosition
FUNCTION_BLOCK MC_StepBlockDetection
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    DetectionVelocityLimit : LREAL;
    DetectionVelocityTime : TIME;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    TorqueTolerance : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptInstantLagReduction : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RecordedPosition : LREAL;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 4;
    eng.Direction := Direction;
    eng.SwitchMode := 0;
    eng.RefLevel := FALSE;
    eng.Velocity := Velocity;
    eng.SetPosEn := FALSE;
    eng.SetPosition := 0.0;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := DetectionVelocityLimit;
    eng.DetVelTime := DetectionVelocityTime;
    eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
    RecordedPosition := eng.RecordedPosition;
END_FUNCTION_BLOCK

// MC_StepBlockLagBased — block search with lag-error criterion
FUNCTION_BLOCK MC_StepBlockLagBased
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    SetPosition : LREAL;
    DetectionVelocityLimit : LREAL;
    DetectionVelocityTime : TIME;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    LagLimit : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptInstantLagReduction : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 5;
    eng.Direction := Direction;
    eng.SwitchMode := 0;
    eng.RefLevel := FALSE;
    eng.Velocity := Velocity;
    eng.SetPosEn := TRUE;
    eng.SetPosition := SetPosition;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := DetectionVelocityLimit;
    eng.DetVelTime := DetectionVelocityTime;
    eng.LagLimit := LagLimit;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// MC_StepBlockLagBasedDetection — lag-based block search; returns RecordedPosition
FUNCTION_BLOCK MC_StepBlockLagBasedDetection
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    SetPosition : LREAL;
    DetectionVelocityLimit : LREAL;
    DetectionVelocityTime : TIME;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    LagLimit : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptInstantLagReduction : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RecordedPosition : LREAL;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 6;
    eng.Direction := Direction;
    eng.SwitchMode := 0;
    eng.RefLevel := FALSE;
    eng.Velocity := Velocity;
    eng.SetPosEn := FALSE;
    eng.SetPosition := 0.0;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := DetectionVelocityLimit;
    eng.DetVelTime := DetectionVelocityTime;
    eng.LagLimit := LagLimit;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
    RecordedPosition := eng.RecordedPosition;
END_FUNCTION_BLOCK

// MC_StepLimitSwitch — search for a hardware limit switch
FUNCTION_BLOCK MC_StepLimitSwitch
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    LimitSwitchMode : UDINT;
    LimitSwitchLevel : BOOL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    SetPosition : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptEnableLagErrorDetection : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 7;
    eng.Direction := Direction;
    eng.SwitchMode := LimitSwitchMode;
    eng.RefLevel := LimitSwitchLevel;
    eng.Velocity := Velocity;
    eng.SetPosEn := TRUE;
    eng.SetPosition := SetPosition;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0;
    eng.DetVelTime := T#0s;
    eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// MC_StepLimitSwitchDetection — limit switch search; returns RecordedPosition
FUNCTION_BLOCK MC_StepLimitSwitchDetection
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    LimitSwitchMode : UDINT;
    LimitSwitchLevel : BOOL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptEnableLagErrorDetection : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RecordedPosition : LREAL;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 8;
    eng.Direction := Direction;
    eng.SwitchMode := LimitSwitchMode;
    eng.RefLevel := LimitSwitchLevel;
    eng.Velocity := Velocity;
    eng.SetPosEn := FALSE;
    eng.SetPosition := 0.0;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0;
    eng.DetVelTime := T#0s;
    eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
    RecordedPosition := eng.RecordedPosition;
END_FUNCTION_BLOCK

// MC_StepReferencePulse — search for the encoder zero pulse
FUNCTION_BLOCK MC_StepReferencePulse
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    SwitchMode : UDINT;
    RefLevel : BOOL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    SetPosition : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptEnableLagErrorDetection : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 9;
    eng.Direction := Direction;
    eng.SwitchMode := SwitchMode;
    eng.RefLevel := RefLevel;
    eng.Velocity := Velocity;
    eng.SetPosEn := TRUE;
    eng.SetPosition := SetPosition;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0;
    eng.DetVelTime := T#0s;
    eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
END_FUNCTION_BLOCK

// MC_StepReferencePulseDetection — zero pulse search; returns RecordedPosition
FUNCTION_BLOCK MC_StepReferencePulseDetection
VAR_INPUT
    Execute : BOOL;
    Direction : UDINT;
    SwitchMode : UDINT;
    RefLevel : BOOL;
    Velocity : LREAL;
    Acceleration : LREAL;
    Deceleration : LREAL;
    Jerk : LREAL;
    TimeLimit : TIME;
    DistanceLimit : LREAL;
    TorqueLimit : LREAL;
    BufferMode : UDINT;
    OptDisableDriveAccess : BOOL;
    OptEnableLagErrorDetection : BOOL;
END_VAR
VAR_IN_OUT
    Axis : UDINT;
    P_Stored : BOOL;
    P_NcSMin : BOOL;  P_NcSMax : BOOL;
    P_NcSPosDiff : BOOL;  P_NcSVeloDiff : BOOL;
    P_NcAMin : BOOL;  P_NcAMax : BOOL;
    P_NcAPosDiff : BOOL;  P_NcAVeloDiff : BOOL;
    P_DrvSBip : LREAL;  P_DrvSPos : LREAL;  P_DrvSNeg : LREAL;
    P_DrvABip : LREAL;  P_DrvAPos : LREAL;  P_DrvANeg : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Active : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    RecordedPosition : LREAL;
END_VAR
VAR
    eng : AH_Run;
END_VAR
    eng.Execute := Execute; eng.Axis := Axis;
    eng.Kind := 10;
    eng.Direction := Direction;
    eng.SwitchMode := SwitchMode;
    eng.RefLevel := RefLevel;
    eng.Velocity := Velocity;
    eng.SetPosEn := FALSE;
    eng.SetPosition := 0.0;
    eng.TimeLimit := TimeLimit;
    eng.DistanceLimit := DistanceLimit;
    eng.DetVelLimit := 0.0;
    eng.DetVelTime := T#0s;
    eng.LagLimit := 0.0;
    eng.PosLimSw := FALSE;
    eng.NegLimSw := FALSE;
    eng.Distance := 0.0;
    eng.ParamIn := P_Stored;
    eng();
    P_Stored := eng.ParamOut;
    Done := eng.Done; Busy := eng.Busy; Active := eng.Active;
    CommandAborted := eng.CommandAborted;
    Error := eng.Error; ErrorID := eng.ErrorID;
    RecordedPosition := eng.RecordedPosition;
END_FUNCTION_BLOCK
