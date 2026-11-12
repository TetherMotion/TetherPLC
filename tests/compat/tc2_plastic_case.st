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
    IF st = 0 THEN
        mask := 0;
        screw := 0.0;
        cavity := 0.0;
        shot.Execute := TRUE;
        shot.StartCycle := FALSE;
        shot.FillVel := 5.0;
        shot.BarrelPrs := 80.0;
        shot.CoolTime := 0.5;
        shot.ShotSize := 50.0;
        shot.VpSwitchPos := 40.0;
        shot.CycleTime := 0.1;
        st := 1;
    END_IF;

    // idle -> start -> fill
    IF st = 1 THEN
        IF shot.Done THEN
            shot.StartCycle := TRUE;
            IF shot.Phase = 1 THEN
                mask := mask + 1;
                st := 2;
            END_IF;
        END_IF;
    END_IF;
    // fill: screw advances by velocity
    IF st = 2 THEN
        IF shot.Done THEN
            screw := screw + shot.VelSetpoint * shot.CycleTime;
            shot.ScrewPos := screw;
            IF shot.VelSetpoint > 0.0 THEN mask := mask + 2; END_IF;
            IF shot.Phase = 2 THEN
                mask := mask + 4;
                st := 3;
            END_IF;
        END_IF;
    END_IF;
    // pack: cavity pressure rises to pack setpoint
    IF st = 3 THEN
        IF shot.Done THEN
            cavity := cavity + (shot.PressureSetpoint - cavity) * 0.3;
            shot.CavityPressure := cavity;
            IF shot.PressureSetpoint > 70.0 THEN mask := mask + 8; END_IF;
            IF shot.Phase = 3 THEN
                mask := mask + 16;
                st := 4;
            END_IF;
        END_IF;
    END_IF;
    // hold dwell -> complete
    IF st = 4 THEN
        IF shot.Done THEN
            n := n + 1;
            IF shot.ShotDone THEN
                mask := mask + 32;
                st := 5;
            END_IF;
            IF n > 20 THEN st := 5; END_IF;
        END_IF;
    END_IF;

    shot();
END_PROGRAM
