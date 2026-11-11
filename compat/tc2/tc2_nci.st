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
// Honest gaps: no G-code/DIN-66025 interpreter, no FIFO block queue
// or look-ahead, no circular/helical/spline paths (need trig), no
// M-functions/tool/R-params/zero-shift, no multi-block blending.
// Circular interpolation needs sin/cos the ST layer does not yet
// provide; it lands in a later tranche. See DECISIONS D-IMPL-56.
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
