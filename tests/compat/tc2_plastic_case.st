// tc2_plastic_case.st — Plastic shot-cycle e2e: fill -> pack ->
// hold -> complete with a simulated screw/cavity.
// KNOWN OPEN BUG: consumer-side member reads of shot.PressureSetpoint
// /shot.CycleFinished resolve to wrong values at runtime (pou logic,
// IR offsets, and .so disassembly all verified consistent — the
// divergence is in member->offset resolution for this consumer).
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
        shot.FillVel := 5.0;
        shot.CoolTime := 0.3;
        shot.ShotSize := 100.0;
        shot.VpSwitchPos := 40.0;
        shot.CycleTime := 0.1;
        st := 1;
    ELSIF st = 1 THEN
        shot.StartCycle := TRUE;
        st := 2;
    ELSIF st = 2 THEN
        // wait for the fill phase, then release start
        IF shot.Phase = 1 THEN
            mask := mask + 1;
            shot.StartCycle := FALSE;
            st := 3;
        END_IF;
    ELSIF st = 3 THEN
        // fill: screw advances by the velocity setpoint
        IF shot.VelSetpoint > 0.0 THEN mask := mask + 2; END_IF;
        IF shot.Phase = 2 THEN
            mask := mask + 4;
            st := 4;
        ELSE
            screw := screw + shot.VelSetpoint * 0.1;
            shot.ScrewPos := screw;
        END_IF;
    ELSIF st = 4 THEN
        // pack: cavity pressure rises toward the setpoint
        pSeen := shot.PressureSetpoint;
        IF pSeen > 70.0 THEN mask := mask + 8; END_IF;
        cavity := cavity + (pSeen - cavity) * 0.3;
        shot.CavityPressure := cavity;
        IF shot.Phase = 3 THEN
            mask := mask + 16;
            st := 5;
        END_IF;
    ELSIF st = 5 THEN
        // complete (start was released in st=2)
        IF shot.CycleFinished THEN
            mask := mask + 32;
            st := 6;
        END_IF;
    END_IF;

    shot();
END_PROGRAM
