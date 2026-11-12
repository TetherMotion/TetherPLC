// tc2_plastic.st — Tc2_Plastic surface — injection-molding
// control blocks, pure-ST, no service or hardware.
//
// FB_PlasticShot wraps a molding cycle: fill (velocity-profiled
// screw) -> pack (pressure hold) -> hold (cooling dwell) ->
// complete.  The caller supplies screw position, cavity pressure
// and cycle time; the block sequences the phases and outputs the
// velocity/pressure setpoints per ph.

// ---- FB_PlasticShot: injection-molding cycle controller ----
// Phases: 0 idle, 1 fill, 2 pack, 3 hold, 4 complete
FUNCTION_BLOCK FB_PlasticShot
VAR_INPUT
    Execute : BOOL;
    StartCycle : BOOL;     // cycle start command
    ScrewPos : LREAL;       // screw position (mm)
    CavityPressure : LREAL; // cavity pressure (bar)
    FillVel : LREAL;        // fill velocity setpoint (mm/s)
    BarrelPrs : LREAL;   // pack pressure setpoint (bar)
    CoolTime : LREAL;       // hold/cooling dwell (s)
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
    VelSetpoint : LREAL;    // velocity output (fill ph)
    PressureSetpoint : LREAL; // pressure output (pack ph)
    Busy : BOOL;            // cycle in progress
    ShotDone : BOOL;        // cycle complete
END_VAR
VAR
    ph : UDINT;
    tHold : LREAL;
    vel : LREAL;
    prs : LREAL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Reset THEN
        ph := 0;
        tHold := 0.0;
        vel := 0.0;
        prs := 0.0;
    END_IF;
    IF NOT Execute THEN RETURN; END_IF;

    // ph sequencing
    IF ph = 0 THEN
        vel := 0.0; prs := 0.0;
        IF StartCycle THEN ph := 1; END_IF;   // -> fill
    ELSIF ph = 1 THEN
        // fill: velocity-profiled screw to the V/P switch position
        vel := FillVel;
        prs := 0.0;
        IF ScrewPos >= VpSwitchPos THEN
            ph := 2;                     // -> pack
        END_IF;
    ELSIF ph = 2 THEN
        // pack: hold pressure to pack setpoint
        vel := 0.0;
        prs := BarrelPrs;
        IF CavityPressure >= BarrelPrs * 0.9 THEN
            ph := 3;                     // -> hold
            tHold := 0.0;
        END_IF;
    ELSIF ph = 3 THEN
        // hold: cooling dwell
        vel := 0.0;
        prs := 0.0;
        tHold := tHold + CycleTime;
        IF tHold >= CoolTime THEN
            ph := 4;                     // -> complete
        END_IF;
    ELSIF ph = 4 THEN
        vel := 0.0; prs := 0.0;
        IF NOT StartCycle THEN
            ph := 0;                     // -> idle for next shot
        END_IF;
    END_IF;

    Phase := ph;
    VelSetpoint := vel;
    PressureSetpoint := prs;
    Busy := (ph >= 1) AND (ph <= 3);
    ShotDone := (ph = 4);
    Done := TRUE;
END_FUNCTION_BLOCK
