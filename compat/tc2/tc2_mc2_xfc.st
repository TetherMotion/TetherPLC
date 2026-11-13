// Tc2_MC2_XFC + Tc2_NciXFC + Tc3_MC2_AdvancedHoming_XFC (subset)
// XFC cyclic setpoint evaluation over FB_MotionAxis.
//
// Time base: each FB owns a TON clock (IN held TRUE) whose ET is the
// nanosecond scan clock; DC time is flattened to the low 32 bits of
// that clock (T_DCTIME32 semantics, wraps at ~4.29 s).  Axis
// kinematics use the OP6 snapshot (POS_ACTUAL/VEL_ACTUAL, user
// units, velocity in user units per second) with linear
// extrapolation — the real XFC kernel evaluates the full motion
// profile, so trapezoid/segment extrapolation is a documented gap.
// AXIS_REF is the UDINT handle; OUTPUT_REF/CAMSWITCH_REF/TRACK_REF
// and the EL terminal structs are flattened to BOOL/LREAL members.

// Shared clock helper: TON ET is ns; expose low32 as UDINT.
FUNCTION_BLOCK XFC_Clock
VAR_OUTPUT
    DcTime : UDINT;   // low 32 bits of the ns scan clock
    NowNs  : LINT;    // full ns (may exceed 32 bits on long runs)
END_VAR
VAR
    clk : TON;
END_VAR
    clk(IN := TRUE, PT := T#1000000h);
    DcTime := TIME_TO_UDINT(clk.ET);
    NowNs := TIME_TO_LINT(clk.ET);
END_FUNCTION_BLOCK

// XFC_PositionAtTime — extrapolated axis position at DcTime.
FUNCTION_BLOCK XFC_PositionAtTime
VAR_INPUT
    DcTime : UDINT;
    Axis   : UDINT;
END_VAR
VAR_OUTPUT
    Position : LREAL;
    Error    : BOOL;
    ErrorID  : UDINT;
END_VAR
VAR
    clk : XFC_Clock;
    ax  : FB_MotionAxis;
    dt  : DINT;
    pos : DINT;
    vel : DINT;
END_VAR
    clk();
    ax.AXIS := Axis;
    ax.OP := 6; ax();
    Error := FALSE; ErrorID := 0;
    IF ax.STATUS < 0 THEN
        Error := TRUE; ErrorID := ax.ERRID;
    ELSE
        pos := ax.POS_ACTUAL;
        vel := ax.VEL_ACTUAL;
        // 32-bit modular delta, interpreted signed (ns)
        dt := UDINT_TO_DINT(DcTime - clk.DcTime);
        Position := DINT_TO_LREAL(pos)
                  + DINT_TO_LREAL(vel) * DINT_TO_LREAL(dt) / 1000000000.0;
    END_IF;
END_FUNCTION_BLOCK

// XFC_TimeOfPosition — DC time at which the axis reaches Position.
FUNCTION_BLOCK XFC_TimeOfPosition
VAR_INPUT
    Position : LREAL;
    Axis     : UDINT;
END_VAR
VAR_OUTPUT
    DcTime  : UDINT;
    Error   : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    clk : XFC_Clock;
    ax  : FB_MotionAxis;
    pos : DINT;
    vel : DINT;
    dtNs : LREAL;
END_VAR
    clk();
    ax.AXIS := Axis;
    ax.OP := 6; ax();
    Error := FALSE; ErrorID := 0; DcTime := clk.DcTime;
    IF ax.STATUS < 0 THEN
        Error := TRUE; ErrorID := ax.ERRID;
    ELSE
        pos := ax.POS_ACTUAL;
        vel := ax.VEL_ACTUAL;
        IF vel <> 0 THEN
            dtNs := (Position - DINT_TO_LREAL(pos))
                    / DINT_TO_LREAL(vel) * 1000000000.0;
            DcTime := clk.DcTime + LREAL_TO_UDINT(dtNs);
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// XFC_TimeToPosition — signed ns until the axis reaches Position
// (negative when the position is in the past).
FUNCTION_BLOCK XFC_TimeToPosition
VAR_INPUT
    Position : LREAL;
    Axis     : UDINT;
END_VAR
VAR_OUTPUT
    Duration : DINT;
    Error    : BOOL;
    ErrorID  : UDINT;
END_VAR
VAR
    clk : XFC_Clock;
    ax  : FB_MotionAxis;
    pos : DINT;
    vel : DINT;
END_VAR
    clk();
    ax.AXIS := Axis;
    ax.OP := 6; ax();
    Error := FALSE; ErrorID := 0; Duration := 0;
    IF ax.STATUS < 0 THEN
        Error := TRUE; ErrorID := ax.ERRID;
    ELSE
        pos := ax.POS_ACTUAL;
        vel := ax.VEL_ACTUAL;
        IF vel <> 0 THEN
            Duration := LREAL_TO_DINT((Position - DINT_TO_LREAL(pos))
                             / DINT_TO_LREAL(vel) * 1000000000.0);
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// XFC_TimeToModuloPosition — time to the nearest modulo position in
// the direction of travel.  ModuloFactor is the position wrap
// (0 disables wrapping).
FUNCTION_BLOCK XFC_TimeToModuloPosition
VAR_INPUT
    Position      : LREAL;
    ModuloFactor  : LREAL;
    Axis          : UDINT;
END_VAR
VAR_OUTPUT
    Duration : DINT;
    Error    : BOOL;
    ErrorID  : UDINT;
END_VAR
VAR
    clk : XFC_Clock;
    ax  : FB_MotionAxis;
    pos : DINT;
    vel : DINT;
    p   : LREAL;
END_VAR
    clk();
    ax.AXIS := Axis;
    ax.OP := 6; ax();
    Error := FALSE; ErrorID := 0; Duration := 0;
    IF ax.STATUS < 0 THEN
        Error := TRUE; ErrorID := ax.ERRID;
    ELSE
        pos := ax.POS_ACTUAL;
        vel := ax.VEL_ACTUAL;
        p := Position;
        IF ModuloFactor > 0.0 THEN
            // wrap target into the revolution around pos
            WHILE p < DINT_TO_LREAL(pos) - ModuloFactor / 2.0 DO
                p := p + ModuloFactor;
            END_WHILE;
            WHILE p > DINT_TO_LREAL(pos) + ModuloFactor / 2.0 DO
                p := p - ModuloFactor;
            END_WHILE;
        END_IF;
        IF vel <> 0 THEN
            Duration := LREAL_TO_DINT((p - DINT_TO_LREAL(pos))
                             / DINT_TO_LREAL(vel) * 1000000000.0);
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// XFC_GetCurDcTaskTime — current DC task time (low32 of the ns
// clock; the FUNCTION form is flattened to an FB).
FUNCTION_BLOCK XFC_GetCurDcTaskTime
VAR_OUTPUT
    DcTime : UDINT;
END_VAR
VAR
    clk : XFC_Clock;
END_VAR
    clk();
    DcTime := clk.DcTime;
END_FUNCTION_BLOCK

// XFC_ExtendDcTime — extend a 32-bit DC time against a reference.
// The sim clock domain is 32-bit, so the FB reports the signed
// nanosecond offset of DcTime32 relative to RefDcTime (the 64-bit
// T_DCTIME result cannot be represented).
FUNCTION_BLOCK XFC_ExtendDcTime
VAR_INPUT
    DcTime32  : UDINT;
    RefDcTime : UDINT;
END_VAR
VAR_OUTPUT
    DeltaNs : DINT;
    DcTime  : UDINT;
END_VAR
    DeltaNs := UDINT_TO_DINT(DcTime32 - RefDcTime);
    DcTime := DcTime32;
END_FUNCTION_BLOCK

// XFC_PositionCam — digital output on inside [FirstOn,LastOn].
// OUTPUT_REF flattened to a BOOL output; Modulo wraps positions by
// ModuloFactor (0 = absolute).
FUNCTION_BLOCK XFC_PositionCam
VAR_INPUT
    FirstOnPosition : LREAL;
    LastOnPosition  : LREAL;
    Modulo          : BOOL;
    ModuloFactor    : LREAL;
    Axis            : UDINT;
END_VAR
VAR_OUTPUT
    Output  : BOOL;
    Error   : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax  : FB_MotionAxis;
    pos : LREAL;
    p   : LREAL;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 6; ax();
    Error := FALSE; ErrorID := 0; Output := FALSE;
    IF ax.STATUS < 0 THEN
        Error := TRUE; ErrorID := ax.ERRID;
    ELSE
        pos := DINT_TO_LREAL(ax.POS_ACTUAL);
        p := pos;
        IF Modulo AND ModuloFactor > 0.0 THEN
            p := pos - DINT_TO_LREAL(LREAL_TO_DINT(pos / ModuloFactor)) * ModuloFactor;
        END_IF;
        IF p >= FirstOnPosition AND p <= LastOnPosition THEN
            Output := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// XFC_TimeCam — output ON from FirstOnPosition for Duration
// seconds (position-triggered, time-limited).
FUNCTION_BLOCK XFC_TimeCam
VAR_INPUT
    FirstOnPosition : LREAL;
    Duration        : LREAL;
    Modulo          : BOOL;
    ModuloFactor    : LREAL;
    Axis            : UDINT;
END_VAR
VAR_OUTPUT
    Output  : BOOL;
    Error   : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    clk   : XFC_Clock;
    ax    : FB_MotionAxis;
    pos   : LREAL;
    p     : LREAL;
    armed : BOOL;
    onAt  : LINT;
END_VAR
    clk();
    ax.AXIS := Axis;
    ax.OP := 6; ax();
    Error := FALSE; ErrorID := 0; Output := FALSE;
    IF ax.STATUS < 0 THEN
        Error := TRUE; ErrorID := ax.ERRID;
    ELSE
        pos := DINT_TO_LREAL(ax.POS_ACTUAL);
        p := pos;
        IF Modulo AND ModuloFactor > 0.0 THEN
            p := pos - DINT_TO_LREAL(LREAL_TO_DINT(pos / ModuloFactor)) * ModuloFactor;
        END_IF;
        // one shot per crossing: re-arms only when position drops
        // back below FirstOnPosition
        IF p < FirstOnPosition THEN
            armed := FALSE;
        ELSIF p >= FirstOnPosition AND NOT armed THEN
            armed := TRUE;
            onAt := clk.NowNs;
        END_IF;
        IF armed AND (LINT_TO_LREAL(clk.NowNs - onAt) / 1000000000.0) < Duration THEN
            Output := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// XFC_BreakCam — brake cam: Output follows BreakRelease while the
// axis is valid (the position-dependent release curve of the real
// block is not modeled).
FUNCTION_BLOCK XFC_BreakCam
VAR_INPUT
    BreakRelease : BOOL;
    Axis         : UDINT;
END_VAR
VAR_OUTPUT
    Output  : BOOL;
    Error   : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    ax.AXIS := Axis;
    ax.OP := 6; ax();
    Error := FALSE; ErrorID := 0;
    IF ax.STATUS < 0 THEN
        Error := TRUE; ErrorID := ax.ERRID;
    ELSE
        Output := BreakRelease;
    END_IF;
END_FUNCTION_BLOCK

// MC_DigitalCamSwitch — single-cam digital switch (CAMSWITCH_REF/
// OUTPUT_REF/TRACK_REF flattened to FirstOn/LastOn/Modulo + BOOL).
// MC_DigitalCamSwitch_MultiEdge shares this implementation.
FUNCTION_BLOCK MC_DigitalCamSwitch
VAR_INPUT
    Enable          : BOOL;
    FirstOnPosition : LREAL;
    LastOnPosition  : LREAL;
    Modulo          : BOOL;
    ModuloFactor    : LREAL;
    Axis            : UDINT;
END_VAR
VAR_OUTPUT
    InOperation : BOOL;
    Output      : BOOL;
    Error       : BOOL;
    ErrorID     : UDINT;
END_VAR
VAR
    cam : XFC_PositionCam;
END_VAR
    InOperation := Enable;
    cam(FirstOnPosition := FirstOnPosition,
        LastOnPosition  := LastOnPosition,
        Modulo          := Modulo,
        ModuloFactor    := ModuloFactor,
        Axis            := Axis);
    Output := Enable AND cam.Output;
    Error := cam.Error;
    ErrorID := cam.ErrorID;
END_FUNCTION_BLOCK

// MC_DigitalCamSwitch_MultiEdge — same flattened single-cam model;
// the multi-edge/EL5131 track variant is not modeled.
FUNCTION_BLOCK MC_DigitalCamSwitch_MultiEdge
VAR_INPUT
    Enable          : BOOL;
    FirstOnPosition : LREAL;
    LastOnPosition  : LREAL;
    Modulo          : BOOL;
    ModuloFactor    : LREAL;
    Axis            : UDINT;
END_VAR
VAR_OUTPUT
    InOperation : BOOL;
    Output      : BOOL;
    Error       : BOOL;
    ErrorID     : UDINT;
END_VAR
VAR
    cam : XFC_PositionCam;
END_VAR
    InOperation := Enable;
    cam(FirstOnPosition := FirstOnPosition,
        LastOnPosition  := LastOnPosition,
        Modulo          := Modulo,
        ModuloFactor    := ModuloFactor,
        Axis            := Axis);
    Output := Enable AND cam.Output;
    Error := cam.Error;
    ErrorID := cam.ErrorID;
END_FUNCTION_BLOCK

// XFC_TouchProbe — record PositionActualValue on the TriggerInput
// rising edge while Execute is active (XFC_TRIGGER_REF flattened
// to BOOL).
FUNCTION_BLOCK XFC_TouchProbe
VAR_INPUT
    Execute       : BOOL;
    TriggerInput  : BOOL;
    Axis          : UDINT;
END_VAR
VAR_OUTPUT
    RecordedPosition : LREAL;
    Done             : BOOL;
    Busy             : BOOL;
    Error            : BOOL;
    ErrorID          : UDINT;
END_VAR
VAR
    ax        : FB_MotionAxis;
    exPrev    : BOOL;
    exEdge    : BOOL;
    trgPrev   : BOOL;
    armed     : BOOL;
END_VAR
    ax.AXIS := Axis;
    exEdge := Execute AND NOT exPrev; exPrev := Execute;
    Busy := FALSE; Done := FALSE; Error := FALSE; ErrorID := 0;
    IF exEdge THEN
        armed := TRUE;
    END_IF;
    IF NOT Execute THEN armed := FALSE; END_IF;
    IF armed AND TriggerInput AND NOT trgPrev THEN
        ax.OP := 6; ax();
        IF ax.STATUS < 0 THEN
            Error := TRUE; ErrorID := ax.ERRID;
        ELSE
            RecordedPosition := DINT_TO_LREAL(ax.POS_ACTUAL);
            Done := TRUE;
            armed := FALSE;
        END_IF;
    END_IF;
    Busy := armed;
    trgPrev := TriggerInput;
END_FUNCTION_BLOCK

// ---- version --------------------------------------------------------

// F_GetVersionTcXfc — library version placeholder.
FUNCTION F_GetVersionTcXfc : UDINT
F_GetVersionTcXfc := 16#0100;
END_FUNCTION
