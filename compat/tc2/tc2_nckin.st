// tc2_nckin.st — Tc2_NcKinematicTransformation: kinematic
// transformation objects (12 APIs) over the tplc-kin service /
// FB_Kin builtin. The transform engine is the Tether kinematics
// library (ForwardKinematics.hpp): forward maps joint values to a
// Pose6D, backward runs deterministic damped least-squares position
// IK (orientation unconstrained, max 8 DOF).
//
// Object model: up to 8 transformation objects (oidTrafo 0..7),
// each with a configured chain type (cartesian, planar2d,
// articulated3, scara, serial6, serial7, gantry5, delta, stewart),
// optional ACS (joint) and MCS (Cartesian) axis lists, a parameter
// lock, a rotation preset, and an NC channel id. Chain geometry is
// configured through FB_Kin directly (OP1) or the compat case; the
// TwinCAT project-side configuration surface is flattened into the
// oidTrafo object index.
//
// Flattening vs the TwinCAT signatures:
//   ST_KinAxes          -> AcsAxis/McsAxis : ARRAY[0..7] OF UDINT
//                          (axis handles, 0 = empty slot; the
//                          configured count is the leading run of
//                          non-zero entries)
//   NCTOPLC_NCICHANNEL_REF -> nItpChannelId : UDINT
//   U_KinMetaInfo       -> oidTrafo input (object id) + MetaState /
//                          MetaError outputs
//   oidTrafo on blocks  -> kinematic object index 0..7 (TwinCAT
//                          binds the object via the project)
//   pDataIn/pDataOut    -> InData/OutData : ARRAY[0..7] OF LREAL +
//                          nSizeIn/nSizeOut element counts (the
//                          byte-pointer contract is flattened)
//   stAxesPosIn/Out     -> VAR_IN_OUT ARRAY[0..7] OF LREAL
//   stRotation          -> Rotation : ARRAY[0..2] OF LREAL
//   E_KINSTATUS         -> UDINT (0 idle 1 ready 2 error 3 homing)
//   FUNCTIONs with output params -> FUNCTION_BLOCKs (dialect rule)
//
// FB_Kin OP map: 1 configure, 2 set axes, 3 command, 4 calc input,
// 5 result, 6 set locked, 7 rotation, 8 status, 9 set channel,
// 10 get axes.

// Shared pump: run one FB_Kin op sequence on the rising edge.
FUNCTION_BLOCK KinPump
VAR_INPUT
    OP      : UDINT;
    OBJ     : UDINT;
    TYPE_   : DINT;
    PRESET  : UDINT;
    NCFG    : UDINT;
    NIN     : UDINT;
    NACS    : UDINT;
    NMCS    : UDINT;
    CMD     : UDINT;
    ROP     : UDINT;
    CHN     : UDINT;
    LOCKED  : UDINT;
    bExecute : BOOL;
END_VAR
VAR_OUTPUT
    bBusy   : BOOL;
    bError  : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
    i : UDINT;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        kin.OBJ := OBJ;
        kin.OP := OP;
        kin.TYPEIN := TYPE_;
        kin.PRESET := PRESET;
        kin.NCFG := NCFG;
        kin.NIN := NIN;
        kin.NACS := NACS;
        kin.NMCS := NMCS;
        kin.CMD := CMD;
        kin.ROP := ROP;
        kin.CHN := CHN;
        kin.LOCKED := LOCKED;
        FOR i := 0 TO 31 DO
            kin.CFG[i] := 0.0;
        END_FOR;
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_KinConfigGroup: bind ACS/MCS axes + channel to object ----
FUNCTION_BLOCK FB_KinConfigGroup
VAR_INPUT
    bExecute : BOOL;
    bCartesianMode : BOOL;
    oidTrafo : UDINT;
    nItpChannelId : UDINT;
    AcsAxis : ARRAY[0..7] OF UDINT;
    McsAxis : ARRAY[0..7] OF UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
    i : UDINT;
    nAcs : UDINT;
    nMcs : UDINT;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        /* count the leading run of non-zero (1-based) handles */
        nAcs := 0;
        FOR i := 0 TO 7 DO
            IF AcsAxis[i] <> 0 AND i = nAcs THEN
                nAcs := nAcs + 1;
            END_IF;
        END_FOR;
        nMcs := 0;
        FOR i := 0 TO 7 DO
            IF McsAxis[i] <> 0 AND i = nMcs THEN
                nMcs := nMcs + 1;
            END_IF;
        END_FOR;
        kin.OBJ := oidTrafo;
        kin.OP := 2;
        kin.NACS := nAcs;
        kin.NMCS := nMcs;
        FOR i := 0 TO 7 DO
            kin.ACS[i] := 0;
            kin.MCS[i] := 0;
        END_FOR;
        IF nAcs > 0 THEN
            FOR i := 0 TO nAcs - 1 DO
                kin.ACS[i] := AcsAxis[i];
            END_FOR;
        END_IF;
        IF nMcs > 0 THEN
            FOR i := 0 TO nMcs - 1 DO
                kin.MCS[i] := McsAxis[i];
            END_FOR;
        END_IF;
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        ELSE
            kin.OP := 9;
            kin.CHN := nItpChannelId;
            kin();
            IF kin.STATUS <> 0 THEN
                bError := TRUE;
                nErrorId := kin.ERRID;
            END_IF;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_KinResetGroup: reset the kinematic group ----
FUNCTION_BLOCK FB_KinResetGroup
VAR_INPUT
    bExecute : BOOL;
    oidTrafo : UDINT;
    nItpChannelId : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        kin.OBJ := oidTrafo;
        kin.OP := 3;
        kin.CMD := 3;   /* TPLC_KIN_CMD_RESET */
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        END_IF;
        /* re-associate the channel when one is given */
        IF nItpChannelId <> 0 AND NOT bError THEN
            kin.OP := 9;
            kin.CHN := nItpChannelId;
            kin();
            IF kin.STATUS <> 0 THEN
                bError := TRUE;
                nErrorId := kin.ERRID;
            END_IF;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_KinCalcTrafo: forward / backward transform ----
FUNCTION_BLOCK FB_KinCalcTrafo
VAR_INPUT
    bExecute : BOOL;
    bForward : BOOL;
    oidTrafo : UDINT;
END_VAR
VAR_IN_OUT
    AxesPosIn : ARRAY[0..7] OF LREAL;
    AxesPosOut : ARRAY[0..7] OF LREAL;
    MetaState : UDINT;
    MetaError : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
    i : UDINT;
    cmd : UDINT;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    MetaState := 0;
    MetaError := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        kin.OBJ := oidTrafo;
        /* input: forward = joints, backward = target xyz + seed */
        kin.OP := 4;
        kin.NIN := 8;
        FOR i := 0 TO 7 DO
            kin.IN[i] := AxesPosIn[i];
        END_FOR;
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        ELSE
            cmd := 2;   /* backward (IK) */
            IF bForward THEN cmd := 1; END_IF;
            kin.OP := 3;
            kin.CMD := cmd;
            kin();
            IF kin.STATUS <> 0 THEN
                bError := TRUE;
                nErrorId := kin.ERRID;
            ELSE
                kin.OP := 5;
                kin();
                IF kin.STATUS <> 0 THEN
                    bError := TRUE;
                    nErrorId := kin.ERRID;
                ELSE
                    /* ERR/STATE only refresh on OP8; poll once so
                       MetaError carries the object's error code
                       (e.g. 1 = no model, 3 = IK no convergence). */
                    kin.OP := 8;
                    kin();
                    MetaState := kin.STATE;
                    MetaError := kin.ERR;
                    IF bForward THEN
                        /* pose x,y,z,roll,pitch,yaw -> out[0..5] */
                        FOR i := 0 TO 5 DO
                            AxesPosOut[i] := kin.OUT[i];
                        END_FOR;
                    ELSE
                        /* IK joint solution q -> out[0..7] */
                        FOR i := 0 TO 7 DO
                            AxesPosOut[i] := kin.Q[i];
                        END_FOR;
                    END_IF;
                END_IF;
            END_IF;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_KinCalcMultiTrafo: flattened single-object transform ----
// TwinCAT packs a caller buffer of transform requests; the flattened
// form carries one object's input vector and returns the result
// vector (pose for forward, joints for backward).
FUNCTION_BLOCK FB_KinCalcMultiTrafo
VAR_INPUT
    bExecute : BOOL;
    bForward : BOOL;
    oidTrafo : UDINT;
    nSizeIn : UDINT;
    nSizeOut : UDINT;
END_VAR
VAR_IN_OUT
    InData : ARRAY[0..7] OF LREAL;
    OutData : ARRAY[0..15] OF LREAL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
    nOutLen : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
    i : UDINT;
    n : UDINT;
    cmd : UDINT;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    nOutLen := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        kin.OBJ := oidTrafo;
        n := nSizeIn;
        IF n > 8 THEN n := 8; END_IF;
        kin.OP := 4;
        kin.NIN := n;
        FOR i := 0 TO 7 DO
            kin.IN[i] := 0.0;
        END_FOR;
        IF n > 0 THEN
            FOR i := 0 TO n - 1 DO
                kin.IN[i] := InData[i];
            END_FOR;
        END_IF;
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        ELSE
            cmd := 2;
            IF bForward THEN cmd := 1; END_IF;
            kin.OP := 3;
            kin.CMD := cmd;
            kin();
            IF kin.STATUS <> 0 THEN
                bError := TRUE;
                nErrorId := kin.ERRID;
            ELSE
                kin.OP := 5;
                kin();
                IF kin.STATUS <> 0 THEN
                    bError := TRUE;
                    nErrorId := kin.ERRID;
                ELSE
                    IF bForward THEN
                        FOR i := 0 TO 7 DO
                            OutData[i] := kin.OUT[i];
                        END_FOR;
                    ELSE
                        FOR i := 0 TO 7 DO
                            OutData[i] := kin.Q[i];
                        END_FOR;
                    END_IF;
                    nOutLen := 8;
                END_IF;
            END_IF;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_KinUnlockTrafoParam / FB_KinLockTrafoParam ----
FUNCTION_BLOCK FB_KinUnlockTrafoParam
VAR_INPUT
    bExecute : BOOL;
    oidTrafo : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    p : KinPump;
END_VAR
    p.OP := 6; p.OBJ := oidTrafo; p.LOCKED := 0;
    p.bExecute := bExecute;
    p();
    bBusy := p.bBusy; bError := p.bError; nErrorId := p.nErrorId;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_KinLockTrafoParam
VAR_INPUT
    bExecute : BOOL;
    oidTrafo : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    p : KinPump;
END_VAR
    p.OP := 6; p.OBJ := oidTrafo; p.LOCKED := 1;
    p.bExecute := bExecute;
    p();
    bBusy := p.bBusy; bError := p.bError; nErrorId := p.nErrorId;
END_FUNCTION_BLOCK

// ---- FB_KinExtendedRotationRange: save on execute edge, restore
// on release (extends the achievable rotation range temporarily).
FUNCTION_BLOCK FB_KinExtendedRotationRange
VAR_INPUT
    bExecute : BOOL;
    oidTrafo : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
    rop : UDINT;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    rop := 0;
    IF bExecute AND NOT exPrev THEN
        rop := 2;   /* TPLC_KIN_ROT_SAVE */
        bBusy := TRUE;
    ELSIF NOT bExecute AND exPrev THEN
        rop := 3;   /* TPLC_KIN_ROT_RESTORE */
        bBusy := TRUE;
    END_IF;
    IF rop <> 0 THEN
        kin.OBJ := oidTrafo;
        kin.OP := 7;
        kin.ROP := rop;
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- FB_KinPresetRotation: set the rotational state ----
FUNCTION_BLOCK FB_KinPresetRotation
VAR_INPUT
    bExecute : BOOL;
    oidTrafo : UDINT;
END_VAR
VAR_IN_OUT
    Rotation : ARRAY[0..2] OF LREAL;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
    i : UDINT;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        kin.OBJ := oidTrafo;
        kin.OP := 7;
        kin.ROP := 1;   /* TPLC_KIN_ROT_PRESET */
        FOR i := 0 TO 2 DO
            kin.ROT[i] := Rotation[i];
        END_FOR;
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

// ---- F_KinGetChnOperationState (FUNCTION -> FB): channel op state.
// Flattened: the channel ref becomes the kinematic object id.
FUNCTION_BLOCK F_KinGetChnOperationState
VAR_INPUT
    oidTrafo : UDINT;
END_VAR
VAR_OUTPUT
    OpState : UDINT;
    ErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
END_VAR
    OpState := 0;
    ErrorId := 0;
    kin.OBJ := oidTrafo;
    kin.OP := 8;
    kin();
    IF kin.STATUS = 0 THEN
        OpState := kin.CHNST;
    ELSE
        ErrorId := kin.ERRID;
    END_IF;
END_FUNCTION_BLOCK

// ---- F_KinGetAcsMcsAxisIds (FUNCTION -> FB): read configured
// axis lists. Flattened: channel ref becomes the object id; the
// ST_KinAxes struct becomes the two output arrays + counts.
FUNCTION_BLOCK F_KinGetAcsMcsAxisIds
VAR_INPUT
    oidTrafo : UDINT;
END_VAR
VAR_OUTPUT
    AcsAxis : ARRAY[0..7] OF UDINT;
    McsAxis : ARRAY[0..7] OF UDINT;
    NAcs : UDINT;
    NMcs : UDINT;
    ErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    i : UDINT;
END_VAR
    NAcs := 0;
    NMcs := 0;
    ErrorId := 0;
    FOR i := 0 TO 7 DO
        AcsAxis[i] := 0;
        McsAxis[i] := 0;
    END_FOR;
    kin.OBJ := oidTrafo;
    kin.OP := 10;
    kin();
    IF kin.STATUS = 0 THEN
        NAcs := kin.NACS;
        NMcs := kin.NMCS;
        FOR i := 0 TO 7 DO
            IF i < NAcs THEN AcsAxis[i] := kin.ACS[i]; END_IF;
            IF i < NMcs THEN McsAxis[i] := kin.MCS[i]; END_IF;
        END_FOR;
    ELSE
        ErrorId := kin.ERRID;
    END_IF;
END_FUNCTION_BLOCK

// ---- F_KinAxesInTolerance: element-wise array compare ----
FUNCTION F_KinAxesInTolerance : BOOL
VAR_INPUT
    AxesPosIn1 : ARRAY[0..7] OF LREAL;
    AxesPosIn2 : ARRAY[0..7] OF LREAL;
    AxesTolerance : ARRAY[0..7] OF LREAL;
END_VAR
VAR
    i : UDINT;
    ok : BOOL;
END_VAR
    ok := TRUE;
    FOR i := 0 TO 7 DO
        IF ABS(AxesPosIn1[i] - AxesPosIn2[i]) > AxesTolerance[i] THEN
            ok := FALSE;
        END_IF;
    END_FOR;
    F_KinAxesInTolerance := ok;
END_FUNCTION

// ---- FB_KinCheckActualStatus (legacy): kinematic channel status.
// eKinStatus: 0 idle 1 ready 2 error (E_KINSTATUS values).
FUNCTION_BLOCK FB_KinCheckActualStatus
VAR_INPUT
    bExecute : BOOL;
    oidTrafo : UDINT;
END_VAR
VAR_IN_OUT
    eKinStatus : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    kin : FB_Kin;
    exPrev : BOOL;
END_VAR
    bBusy := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF bExecute AND NOT exPrev THEN
        bBusy := TRUE;
        kin.OBJ := oidTrafo;
        kin.OP := 8;
        kin();
        IF kin.STATUS <> 0 THEN
            bError := TRUE;
            nErrorId := kin.ERRID;
        ELSE
            eKinStatus := kin.STATE;
        END_IF;
    END_IF;
    exPrev := bExecute;
END_FUNCTION_BLOCK

FUNCTION F_GetVersionTcNcKin : UDINT
    F_GetVersionTcNcKin := 16#0100;
END_FUNCTION
