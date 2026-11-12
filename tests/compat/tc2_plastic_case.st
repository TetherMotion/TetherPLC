// tc2_plastic_case.st — Plastic shot-cycle e2e: fill -> pack ->
// hold -> complete with a simulated screw/cavity.
PROGRAM tc2_plastic_case
VAR
    shot : FB_PlasticShot;
    screw : LREAL;
    cavity : LREAL;
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
END_VAR
    // diagnostic: phase in b0, Done b8, Execute b9, StartCycle b10
    mask := shot.Phase;
    IF shot.Done THEN mask := mask + 256; END_IF;
    IF shot.Execute THEN mask := mask + 512; END_IF;
    IF shot.StartCycle THEN mask := mask + 1024; END_IF;
    IF st = 0 THEN
        screw := 0.0;
        cavity := 0.0;
        shot.Execute := TRUE;
        shot.StartCycle := FALSE;
        shot.FillVel := 5.0;
        shot.PackPressure := 80.0;
        shot.HoldTime := 0.5;
        shot.ShotSize := 50.0;
        shot.VpSwitchPos := 40.0;
        shot.CycleTime := 0.1;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF shot.Done THEN
            shot.StartCycle := TRUE;
            IF shot.Phase = 1 THEN st := 2; END_IF;
        END_IF;
    END_IF;
    IF st = 2 THEN
        IF shot.Done THEN
            screw := screw + shot.VelSetpoint * shot.CycleTime;
            shot.ScrewPos := screw;
            IF shot.Phase = 2 THEN st := 3; END_IF;
        END_IF;
    END_IF;
    IF st = 3 THEN
        IF shot.Done THEN
            cavity := cavity + (shot.PressureSetpoint - cavity) * 0.3;
            shot.CavityPressure := cavity;
            IF shot.Phase = 3 THEN st := 4; END_IF;
        END_IF;
    END_IF;
    IF st = 4 THEN
        IF shot.Done THEN
            n := n + 1;
            IF shot.ShotDone THEN st := 5; END_IF;
            IF n > 20 THEN st := 5; END_IF;
        END_IF;
    END_IF;

    shot();
END_PROGRAM
