// tc2_plastic_case.st — Plastic shot-cycle e2e: fill -> pack ->
// hold -> complete with a simulated screw/cavity.
PROGRAM tc2_plastic_case
VAR
    mask AT %QD0 : UDINT;
    shot : FB_PlasticShot;
    screw : LREAL;
    cavity : LREAL;
    st : INT;
    n : INT;
    pSeen : LREAL;
END_VAR
    // One state per scan (ELSIF): outputs read here are the
    // previous call's, inputs written here reach the call below.
    IF st = 0 THEN
        mask := 0;
        screw := 0.0;
        cavity := 0.0;
        n := 0;
        shot.Execute := TRUE;
        shot.StartCycle := FALSE;
        shot.MoldTarget := 80.0;
        shot.FillVel := 40.0;
        shot.CoolTime := 0.3;
        shot.ShotSize := 100.0;
        shot.VpSwitchPos := 20.0;
        shot.CycleTime := 0.1;
        st := 1;
    ELSIF st = 1 THEN
        shot.StartCycle := TRUE;
        st := 2;
    ELSIF st = 2 THEN
        // wait for the fill phase, then release start
        IF shot.Phase = 1 THEN
            mask := mask OR 1;
            shot.StartCycle := FALSE;
            st := 3;
        END_IF;
    ELSIF st = 3 THEN
        // fill: velocity setpoint drives the screw to V/P
        IF shot.VelSetpoint > 0.0 THEN mask := mask OR 2; END_IF;
        shot.ScrewPos := shot.ScrewPos + 4.0;
        IF shot.Phase = 2 THEN
            mask := mask OR 4;
            st := 4;
        END_IF;
    ELSIF st = 4 THEN
        // pack: cavity pressure rises to the pack setpoint
        pSeen := shot.PressureSetpoint;
        IF pSeen > 70.0 THEN mask := mask OR 8; END_IF;
        shot.CavityPressure := 100.0;
        IF shot.Phase = 3 THEN
            mask := mask OR 16;
            st := 5;
        END_IF;
    ELSIF st = 5 THEN
        // complete (start was released in st=2)
        IF shot.CycleFinished THEN
            mask := mask OR 32;
            st := 6;
        END_IF;
    END_IF;

    shot();
END_PROGRAM
