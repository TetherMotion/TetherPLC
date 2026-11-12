// Tc2_NCI — TwinCAT NC-I (TF5100) channel-interpolation surface over
// the builtin FB_MotionAxis (tplc-motion service). This file
// implements the interpolation core: coordinated multi-axis
// setpoint streaming in CyclicSyncPosition (CSP, mode 8).
//
// Model: TwinCAT's NC-I runs a G-code / FIFO interpreter in the NC
// kernel that emits per-axis position setpoints every NC cycle to
// axes in CSP mode. The compat model collapses one in-flight
// geometric block into each FB: FB_NciLine parameterizes a straight
// line across a 3-axis group, advances the path parameter s by
// Velocity*CycleUs each scan, and streams per-axis targets
// (0x607A) with the axes in CSP mode — which is exactly how the NC
// kernel drives CSP axes. There is no backend group registry, so a
// "3D group" is just the X/Y/Z axis handles passed to the block
// (TwinCAT's CfgBuild3DGroup maps to passing those handles).
//
// Time base: the FB has no access to the host clock, so the caller
// supplies the scan period via CycleUs (microseconds); path
// Velocity is then in user-units/second and s += Velocity*CycleUs/1e6
// per scan. Positions/velocities are the backend's int32 user
// units (TwinCAT LREAL — documented gap). Determinism: under --sim,
// CycleUs = step_us makes the path advance a fixed amount per step,
// and CSP snaps each axis to the streamed target so commanded and
// actual positions match.
//
// G-code interpretation lives in Tether: tether_io_motion_server
// exposes an nc.* channel that compiles program text with its G-code
// interpreter, time-parameterizes it with the motion planner (corner
// blending, jerk-limited profiles), and streams CSP setpoints to the
// drives on the mc.time clock. The FB_NciChan blocks below are the
// PLC-side control surface for that channel (TwinCAT Itp* shape);
// there is no in-process interpreter — with no TPLC_MOTION_IO
// endpoint the blocks report the service as unavailable.
//
// Honest gaps: FB_NciLine is the foundational interpreter-free path
// (single block, no queue); the nc.* channel has no PLC-visible
// M-function handshake, R-parameters, zero-shift, or cyclic-channel
// offset tables yet. See DECISIONS D-IMPL-56.
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

// ---- F_NciSqrt: Newton (Babylonian) square root for LREAL ----
// The ST layer has no SQRT builtin, so the path length is computed
// with a fixed-iteration Newton iteration. r0 = x/2 + 1 converges
// monotonically for x >= 0; 48 iterations is enough for the full
// int32 path range and keeps the result deterministic.
FUNCTION F_NciSqrt : LREAL
VAR_INPUT
    x : LREAL;
END_VAR
VAR
    r : LREAL;
    k : INT;
END_VAR
    IF x <= 0.0 THEN
        F_NciSqrt := 0.0;
    ELSE
        r := x * 0.5 + 1.0;
        k := 0;
        REPEAT
            r := 0.5 * (r + x / r);
            k := k + 1;
        UNTIL k >= 48
        END_REPEAT;
        F_NciSqrt := r;
    END_IF;
END_FUNCTION

// ---- FB_NciLine: coordinated 3-axis linear move (CSP stream) ----
// On Execute, latches each axis's actual position as the segment
// start, computes deltas and the Euclidean path length, switches
// all three axes to CSP mode, then each scan advances s toward len
// and writes each axis's interpolated TargetPosition. Done when
// s reaches len (all axes arrive simultaneously — a straight line).
// Releasing Execute mid-move aborts: streaming stops, so each axis
// holds its last streamed target (CommandAborted pulses).
FUNCTION_BLOCK FB_NciLine
VAR_INPUT
    Execute : BOOL;
    PosX : DINT;
    PosY : DINT;
    PosZ : DINT;
    Velocity : DINT;    // resultant path velocity, user-units/sec
    CycleUs : DINT;     // scan period, microseconds
    AxisX : UDINT;
    AxisY : UDINT;
    AxisZ : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    CommandAborted : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ActPosX : DINT;     // interpolated commanded positions
    ActPosY : DINT;
    ActPosZ : DINT;
    PathLen : DINT;     // Euclidean path length (user units)
    PathS : DINT;       // current path parameter along the line
END_VAR
VAR
    axX : FB_MotionAxis;
    axY : FB_MotionAxis;
    axZ : FB_MotionAxis;
    exPrev : BOOL;
    exEdge : BOOL;
    active : BOOL;
    sxr : LREAL;
    syr : LREAL;
    szr : LREAL;
    dxr : LREAL;
    dyr : LREAL;
    dzr : LREAL;
    len : LREAL;
    s : LREAL;
    dt : LREAL;
    vel : LREAL;
END_VAR
    // advance every axis's dynamics once per scan (one poll advances
    // the whole pool); bind each instance to its group axis
    axX.OP := 3; axX();
    axX.AXIS := AxisX; axY.AXIS := AxisY; axZ.AXIS := AxisZ;
    exEdge := Execute AND NOT exPrev;
    exPrev := Execute;
    Done := FALSE;
    CommandAborted := FALSE;
    Error := FALSE;
    ErrorID := 0;

    IF exEdge THEN
        // latch the segment start = current actual positions
        axX.OP := 4; axX.INDEX := 16#6064; axX();
        sxr := DINT_TO_LREAL(axX.VALUE);
        axY.OP := 4; axY.INDEX := 16#6064; axY();
        syr := DINT_TO_LREAL(axY.VALUE);
        axZ.OP := 4; axZ.INDEX := 16#6064; axZ();
        szr := DINT_TO_LREAL(axZ.VALUE);
        dxr := DINT_TO_LREAL(PosX) - sxr;
        dyr := DINT_TO_LREAL(PosY) - syr;
        dzr := DINT_TO_LREAL(PosZ) - szr;
        len := F_NciSqrt(dxr * dxr + dyr * dyr + dzr * dzr);
        PathLen := TRUNC(len);
        dt := DINT_TO_LREAL(CycleUs) / 1000000.0;
        vel := DINT_TO_LREAL(Velocity);
        s := 0.0;
        IF len < 0.5 THEN
            // already at the target — nothing to stream
            active := FALSE;
            Done := TRUE;
        ELSE
            // put the group into CyclicSyncPosition (mode 8)
            axX.OP := 5; axX.INDEX := 16#6060; axX.VALUE := 8; axX();
            axY.OP := 5; axY.INDEX := 16#6060; axY.VALUE := 8; axY();
            axZ.OP := 5; axZ.INDEX := 16#6060; axZ.VALUE := 8; axZ();
            active := TRUE;
        END_IF;
    END_IF;

    IF active THEN
        IF Execute THEN
            // advance the path parameter and clamp at the end
            s := s + vel * dt;
            IF s >= len THEN
                s := len;
                active := FALSE;
                Done := TRUE;
            END_IF;
            PathS := TRUNC(s);
            // stream the interpolated setpoints to each CSP axis
            ActPosX := TRUNC(sxr + dxr * s / len);
            ActPosY := TRUNC(syr + dyr * s / len);
            ActPosZ := TRUNC(szr + dzr * s / len);
            axX.OP := 5; axX.INDEX := 16#607A; axX.VALUE := ActPosX; axX();
            axY.OP := 5; axY.INDEX := 16#607A; axY.VALUE := ActPosY; axY();
            axZ.OP := 5; axZ.INDEX := 16#607A; axZ.VALUE := ActPosZ; axZ();
        ELSE
            // aborted: stop streaming so each axis holds its last
            // CSP setpoint (position freezes in place)
            active := FALSE;
            CommandAborted := TRUE;
        END_IF;
    END_IF;
    Busy := active;
END_FUNCTION_BLOCK

// ---- FB_NciEStop: cut power to a 3-axis group ----
// On Execute it writes DisableVoltage (controlword 0x0000) to each
// axis so the group coasts to a stop; Status reports all three
// axes left OperationEnabled. Honest E-stop: a real NC fault also
// freezes the interpolator, which the caller models by releasing
// FB_NciLine's Execute.
FUNCTION_BLOCK FB_NciEStop
VAR_INPUT
    Execute : BOOL;
    AxisX : UDINT;
    AxisY : UDINT;
    AxisZ : UDINT;
END_VAR
VAR_OUTPUT
    Status : BOOL;
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
    exEdge := Execute AND NOT exPrev;
    exPrev := Execute;
    Error := FALSE;
    ErrorID := 0;
    IF exEdge THEN
        ax.AXIS := AxisX; ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 0; ax();
        ax.AXIS := AxisY; ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 0; ax();
        ax.AXIS := AxisZ; ax.OP := 5; ax.INDEX := 16#6040; ax.VALUE := 0; ax();
    END_IF;
    // Status = all three axes out of OperationEnabled (state <> 39)
    ax.AXIS := AxisX; ax.OP := 6; ax();
    Status := (ax.STATUS <> 39);
    ax.AXIS := AxisY; ax.OP := 6; ax();
    Status := Status AND (ax.STATUS <> 39);
    ax.AXIS := AxisZ; ax.OP := 6; ax();
    Status := Status AND (ax.STATUS <> 39);
    Busy := Execute AND NOT Status;
END_FUNCTION_BLOCK

// ---- NC interpreter channel (Tether-owned G-code) ----
// Command codes written to FB_NciChan.CMD (nc.cmd on the wire):
//   1=run 2=step 3=pause 4=resume 5=stop 6=reset 7=verify
// Channel states reported by FB_NciChan.STATE (nc.state):
//   0=idle 1=ready 2=running 3=paused 4=finished 5=error 6=stopped

// ---- FB_ItpLoadText: upload inline program text to the channel ----
// Rising edge on Execute writes PROG to nc.prog; the server compiles
// it immediately. Done means the text was accepted (compile status is
// visible via FB_ItpStatus/nc.state afterwards). TwinCAT loads files
// through ItpLoadProg; inline text is the Tether-native path.
FUNCTION_BLOCK FB_ItpLoadText
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
    ProgText : STRING[4095];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.CHAN := ChanId;
        ch.PROG := ProgText;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpLoadProg: load a program file on the server host ----
// Rising edge on Execute writes PathName to nc.prog_path; the server
// opens and compiles the file. Mirrors TwinCAT ItpLoadProg(Ex).
FUNCTION_BLOCK FB_ItpLoadProg
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
    PathName : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 2;
        ch.CHAN := ChanId;
        ch.PATH := PathName;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpStartStop: start/stop the NC channel ----
// Rising edge on Start issues cmd RUN, on Stop issues cmd STOP.
// Mirrors TwinCAT ItpStartStopEx (bStart/bStop).
FUNCTION_BLOCK FB_ItpStartStop
VAR_INPUT
    Start : BOOL;
    Stop : BOOL;
    ChanId : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    stPrev : BOOL;
    spPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Start AND NOT stPrev THEN
        ch.OP := 3;
        ch.CHAN := ChanId;
        ch.CMD := 1;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    IF Stop AND NOT spPrev THEN
        ch.OP := 3;
        ch.CHAN := ChanId;
        ch.CMD := 5;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    stPrev := Start;
    spPrev := Stop;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpReset: reset the NC channel ----
// Rising edge issues cmd RESET: clears program, state and errors.
// Mirrors TwinCAT ItpResetEx.
FUNCTION_BLOCK FB_ItpReset
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 3;
        ch.CHAN := ChanId;
        ch.CMD := 6;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpEStop: emergency stop on the channel ----
// Rising edge issues cmd STOP: the channel drops out of RUNNING and
// the planner holds the last setpoints (the sim axes freeze). A
// machine E-stop also cuts axis power — compose with FB_NciEStop
// for that. Mirrors TwinCAT ItpEStopEx.
FUNCTION_BLOCK FB_ItpEStop
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 3;
        ch.CHAN := ChanId;
        ch.CMD := 5;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpStatus: cyclic channel status ----
// Polls nc.state/line/total/errcode/progress/duration every scan and
// mirrors them to the outputs (call unconditionally each cycle).
// Finished is TRUE in FINISHED/ERROR/STOPPED; IsEStop is TRUE when
// the channel is in STOPPED, matching TwinCAT ItpIsEStop semantics.
FUNCTION_BLOCK FB_ItpStatus
VAR_INPUT
    ChanId : UDINT;
END_VAR
VAR_OUTPUT
    Valid : BOOL;
    State : DINT;
    Line : UDINT;
    Total : UDINT;
    ErrCode : DINT;
    Finished : BOOL;
    IsEStop : BOOL;
    ProgressMs : DINT;
    DurationMs : DINT;
    MFunc : UDINT;
    FifoLen : UDINT;
    FifoSeq : UDINT;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
END_VAR
    ch.OP := 4;
    ch.CHAN := ChanId;
    ch();
    Valid := (ch.STATUS = 0);
    Error := NOT Valid;
    ErrorID := ch.ERRID;
    State := 0;
    Line := 0;
    Total := 0;
    ErrCode := 0;
    Finished := FALSE;
    IsEStop := FALSE;
    ProgressMs := 0;
    DurationMs := 0;
    IF Valid THEN
        State := ch.STATE;
        Line := ch.LINE;
        Total := ch.TOTAL;
        ErrCode := ch.NERR;
        Finished := (State = 4) OR (State = 5) OR (State = 6);
        IsEStop := (State = 6);
        ProgressMs := ch.PROGRESS;
        DurationMs := ch.DURATION;
        MFunc := ch.MFUNC;
        FifoLen := ch.FIFOLEN;
        FifoSeq := ch.FIFOSEQ;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_ItpFiFoWrite: append a block to the channel FIFO queue ----
// Rising edge on Execute writes Block (one G-code block) to nc.fifo;
// queued entries execute in order after the active plan, anchored at
// the last commanded position. Mirrors TwinCAT FiFoWrite semantics.
FUNCTION_BLOCK FB_ItpFiFoWrite
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
    Block : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.CHAN := ChanId;
        ch.PROG := Block;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpConfirmHsk: confirm a pending M-function ----
// Rising edge confirms the M-code reported in sts.MFunc; the held
// channel resumes. Mirrors TwinCAT ItpConfirmHsk.
FUNCTION_BLOCK FB_ItpConfirmHsk
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 6;
        ch.CHAN := ChanId;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpWriteRParam: write one R-parameter register ----
// Rising edge stores Value into R-parameter Index on the channel;
// applied on the next program compile. Mirrors ItpWriteRParams.
FUNCTION_BLOCK FB_ItpWriteRParam
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
    Index : UDINT;
    Value : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 7;
        ch.CHAN := ChanId;
        ch.AUX := Index;
        ch.RVAL := Value;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpReadRParam: read one R-parameter register ----
// Rising edge reads R-parameter Index into Value. Mirrors
// ItpReadRParams.
FUNCTION_BLOCK FB_ItpReadRParam
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
    Index : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Value : LREAL;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 8;
        ch.CHAN := ChanId;
        ch.AUX := Index;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            Value := ch.RVAL;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_ItpZeroShift: write the channel zero-shift offsets ----
// Rising edge stores the XYZ offsets; applied as a G92 prefix on the
// next program compile.
FUNCTION_BLOCK FB_ItpZeroShift
VAR_INPUT
    Execute : BOOL;
    ChanId : UDINT;
    OffX : LREAL;
    OffY : LREAL;
    OffZ : LREAL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_NciChan;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 9;
        ch.CHAN := ChanId;
        ch.ZSX := OffX;
        ch.ZSY := OffY;
        ch.ZSZ := OffZ;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK
