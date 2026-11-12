// tc2_plastic.st — Tc2_Plastic surface — injection-molding
// control blocks, pure-ST, no service or hardware.
//
// FB_PlasticShot wraps a molding cycle: fill (velocity-profiled
// screw) -> pack (pressure hold) -> hold (cooling dwell) ->
// complete.  The caller supplies screw position, cavity pressure
// and cycle time; the block sequences the phases and outputs the
// velocity/pressure setpoints per phase.

// ---- FB_PlasticShot: injection-molding cycle controller ----
// Phases: 0 idle, 1 fill, 2 pack, 3 hold, 4 complete
FUNCTION_BLOCK FB_PlasticShot
VAR_INPUT
    Execute : BOOL;
    StartCycle : BOOL;     // cycle start command
    ScrewPos : LREAL;       // screw position (mm)
    CavityPressure : LREAL; // cavity pressure (bar)
    FillVel : LREAL;        // fill velocity setpoint (mm/s)
    PackPressure : LREAL;   // pack pressure setpoint (bar)
    HoldTime : LREAL;       // hold/cooling dwell (s)
    ShotSize : LREAL;       // target shot (mm)
    VpSwitchPos : LREAL;    // fill->pack switch position (mm)
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Phase : UDINT;          // 0 idle 1 fill 2 pack 3 hold 4 complete
    VelSetpoint : LREAL;    // velocity output (fill phase)
    PressureSetpoint : LREAL; // pressure output (pack phase)
    Busy : BOOL;            // cycle in progress
    ShotDone : BOOL;        // cycle complete
END_VAR
VAR
    phase : UDINT;
    tHold : LREAL;
    vel : LREAL;
    prs : LREAL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Reset THEN
        phase := 0;
        tHold := 0.0;
        vel := 0.0;
        prs := 0.0;
    END_IF;
    IF NOT Execute THEN RETURN; END_IF;

    // phase sequencing
    IF phase = 0 THEN
        vel := 0.0; prs := 0.0;
        IF StartCycle THEN phase := 1; END_IF;   // -> fill
    ELSIF phase = 1 THEN
        // fill: velocity-profiled screw to the V/P switch position
        vel := FillVel;
        prs := 0.0;
        IF ScrewPos >= VpSwitchPos THEN
            phase := 2;                     // -> pack
        END_IF;
    ELSIF phase = 2 THEN
        // pack: hold pressure to pack setpoint
        vel := 0.0;
        prs := PackPressure;
        IF CavityPressure >= PackPressure * 0.9 THEN
            phase := 3;                     // -> hold
            tHold := 0.0;
        END_IF;
    ELSIF phase = 3 THEN
        // hold: cooling dwell
        vel := 0.0;
        prs := 0.0;
        tHold := tHold + CycleTime;
        IF tHold >= HoldTime THEN
            phase := 4;                     // -> complete
        END_IF;
    ELSIF phase = 4 THEN
        vel := 0.0; prs := 0.0;
        IF NOT StartCycle THEN
            phase := 0;                     // -> idle for next shot
        END_IF;
    END_IF;

    Phase := phase;
    VelSetpoint := vel;
    PressureSetpoint := prs;
    Busy := (phase >= 1) AND (phase <= 3);
    ShotDone := (phase = 4);
    Done := TRUE;
END_FUNCTION_BLOCK
