// tc3_robotics.st — Tc3_mxAutomation (TF5120, KUKA.PLC mxA) and
// Tc3_uniValPlc (TF5130, Stäubli uniVAL plc) surfaces.  Both are
// licensed Beckhoff robotics engines talking to proprietary robot
// controllers; TetherPLC ships neither engine nor license, so
// these blocks are honest license/engine-absent stubs: commands
// are validated (bad inputs fail immediately), Busy runs a
// bounded handshake, then Error with a documented ErrorId — no
// robot state, motion, or I/O result is ever faked.
//
// Differences (docs/plc/compat/README.md):
//   - mxAutomation's FB_KRC_Group / FB_KRC_Move* family and
//     uniVAL's FB_Val3 group/command family are unified here as
//     FB_RobotGroup (connect/poll) and FB_RobotCommand
//     (PTP/LIN/CIRC dispatch) — the proprietary protocol surfaces
//     (mxA telegram, uniVAL CS8/CS9) have no backend.
//   - ErrorId: 0 none, 1 engine/license-unavailable,
//     2 bad-address, 3 bad-move, 4 aborted, 5 not-connected.

// ---- FB_RobotGroup: robot-controller group connection ----
// Covers FB_KRC_Group.Open/Disconnect and FB_Val3 group attach.
FUNCTION_BLOCK FB_RobotGroup
VAR_INPUT
    Connect : BOOL;             // rising edge opens the session
    Disconnect : BOOL;
    Controller : STRING;        // controller address/endpoint
    RobotCount : UDINT;         // robots in the group (>=1)
END_VAR
VAR_OUTPUT
    Connected : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    phase : INT;
    tmo : UDINT;
END_VAR
    IF phase = 0 THEN
        Busy := FALSE;
        IF Connect THEN
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            Connected := FALSE;
            tmo := 0;
            IF Controller = '' OR RobotCount = 0 THEN
                Error := TRUE;
                ErrorId := 2;        // bad-address
                Busy := FALSE;
                phase := 2;
            ELSE
                phase := 1;
            END_IF;
        END_IF;
    ELSIF phase = 1 THEN
        tmo := tmo + 1;
        IF tmo >= 4 THEN
            Busy := FALSE;
            Error := TRUE;
            ErrorId := 1;            // license/engine unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Connect THEN
            phase := 0;
        END_IF;
    END_IF;
    IF Disconnect THEN
        Connected := FALSE;
        Busy := FALSE;
        phase := 0;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_RobotCommand: PTP/LIN/CIRC motion dispatch ----
// Covers the FB_KRC_Move*/FB_Val3Move* async-command spine.
FUNCTION_BLOCK FB_RobotCommand
VAR_INPUT
    Execute : BOOL;
    Abort : BOOL;
    GroupConnected : BOOL;      // from FB_RobotGroup.Connected
    MoveType : UDINT;           // 1 PTP, 2 LIN, 3 CIRC
    X : LREAL; Y : LREAL; Z : LREAL;
    Velocity : LREAL;           // mm/s or % override (>0)
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Active : BOOL;
    Done : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    ReachedTarget : BOOL;        // never TRUE without an engine
END_VAR
VAR
    phase : INT;
    tmo : UDINT;
END_VAR
    Done := FALSE;
    Active := FALSE;
    CommandAborted := FALSE;
    ReachedTarget := FALSE;
    IF phase = 0 THEN
        Busy := FALSE;
        IF Execute THEN
            Busy := TRUE;
            Active := TRUE;
            Error := FALSE;
            ErrorId := 0;
            tmo := 0;
            IF NOT GroupConnected THEN
                Error := TRUE;
                ErrorId := 5;        // not-connected
                Active := FALSE;
                Busy := FALSE;
                phase := 2;
            ELSIF MoveType = 0 OR MoveType > 3 OR Velocity <= 0.0 THEN
                Error := TRUE;
                ErrorId := 3;        // bad-move
                Active := FALSE;
                Busy := FALSE;
                phase := 2;
            ELSE
                phase := 1;
            END_IF;
        END_IF;
    ELSIF phase = 1 THEN
        tmo := tmo + 1;
        IF tmo >= 4 THEN
            Busy := FALSE;
            Active := FALSE;
            Error := TRUE;
            ErrorId := 1;            // license/engine unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Execute THEN
            phase := 0;
        END_IF;
    END_IF;
    IF Abort AND phase = 1 THEN
        Busy := FALSE;
        Active := FALSE;
        CommandAborted := TRUE;
        Error := TRUE;
        ErrorId := 4;
        phase := 2;
    END_IF;
    Done := (phase = 2);
END_FUNCTION_BLOCK
