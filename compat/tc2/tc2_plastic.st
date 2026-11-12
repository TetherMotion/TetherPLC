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
    MoldTarget : LREAL;   // pack pressure setpoint (bar)
    Execute : BOOL;
    StartCycle : BOOL;     // cycle start command
    ScrewPos : LREAL;       // screw position (mm)
    CavityPressure : LREAL; // cavity pressure (bar)
    FillVel : LREAL;        // fill velocity setpoint (mm/s)
    CoolTime : LREAL;       // hold/cooling dwell (s)
    ShotSize : LREAL;       // target shot (mm)
    VpSwitchPos : LREAL;    // fill->pack switch position (mm)
    CycleTime : LREAL;
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    CycleActive : BOOL;            // cycle in progress
    CycleFinished : BOOL;        // cycle complete
    ErrorID : UDINT;
    Phase : UDINT;          // 0 idle 1 fill 2 pack 3 hold 4 complete
    PressureSetpoint : LREAL; // pressure output (pack ph)
    VelSetpoint : LREAL;    // velocity output (fill ph)
END_VAR
VAR
    ph : UDINT;
    fin : BOOL;
    tHold : LREAL;
    vel : LREAL;
    pOut : LREAL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Reset THEN
        ph := 0;
        fin := FALSE;
        tHold := 0.0;
        vel := 0.0;
        pOut := 0.0;
    END_IF;
    IF NOT Execute THEN RETURN; END_IF;

    // ph sequencing
    IF ph = 0 THEN
        vel := 0.0; pOut := 0.0;
        IF StartCycle THEN ph := 1; fin := FALSE; END_IF;   // -> fill
    ELSIF ph = 1 THEN
        // fill: velocity-profiled screw to the V/P switch position
        vel := FillVel;
        pOut := 0.0;
        IF ScrewPos >= VpSwitchPos THEN
            ph := 2;                     // -> pack
        END_IF;
    ELSIF ph = 2 THEN
        // pack: hold pressure to pack setpoint
        vel := 0.0;
        pOut := MoldTarget;
        IF CavityPressure >= MoldTarget * 0.9 THEN
            ph := 3;                     // -> hold
            tHold := 0.0;
        END_IF;
    ELSIF ph = 3 THEN
        // hold: cooling dwell
        vel := 0.0;
        pOut := 0.0;
        tHold := tHold + CycleTime;
        IF tHold >= CoolTime THEN
            ph := 4;                     // -> complete
            fin := TRUE;
        END_IF;
    ELSIF ph = 4 THEN
        vel := 0.0; pOut := 0.0;
        IF NOT StartCycle THEN
            ph := 0;                     // -> idle for next shot
        END_IF;
    END_IF;

    Phase := ph;
    VelSetpoint := vel;
    PressureSetpoint := pOut;
    CycleActive := (ph >= 1) AND (ph <= 3);
    CycleFinished := fin;
    Done := TRUE;
END_FUNCTION_BLOCK
