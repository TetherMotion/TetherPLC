// tc2_tc_case.st — TempController e2e: a simulated thermal plant
// (heater -> PT1 lag -> sensor) driven by the PID+PWM controller.
PROGRAM tc2_tc_case
VAR
    ctl : FB_TempController;
    plant : LREAL;   // simulated thermal mass
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        plant := 20.0;                    // ambient
        ctl.Execute := TRUE;
        ctl.Setpoint := 100.0;
        ctl.Kp := 5.0; ctl.Tn := 1.0; ctl.Tv := 0.0;
        ctl.FilterTn := 0.5;
        ctl.PwmPeriod := 1.0;
        ctl.CycleTime := 0.1;
        st := 1;
    END_IF;

    // thermal plant: duty heats the mass, ambient cools it
    IF st = 1 THEN
        IF ctl.Done THEN
            n := n + 1;
            plant := plant +
                     (ctl.Duty * 0.02) - (plant - 20.0) * 0.001;
            ctl.Actual := plant;
            IF n > 50 THEN
                IF ctl.Duty > 10.0 THEN mask := mask + 1; END_IF;  // heating
                IF plant > 25.0 THEN mask := mask + 2; END_IF;     // rising
                IF ctl.Filtered > 20.0 THEN mask := mask + 4; END_IF;
                st := 2;
            END_IF;
        END_IF;
    END_IF;
    // keep driving — the plant settles toward SP
    IF st = 2 THEN
        IF ctl.Done THEN
            n := n + 1;
            plant := plant +
                     (ctl.Duty * 0.02) - (plant - 20.0) * 0.001;
            ctl.Actual := plant;
            IF n > 100 THEN
                IF plant > 30.0 THEN mask := mask + 8; END_IF;     // hot
                IF ctl.E < 90.0 THEN mask := mask + 16; END_IF;    // converging
                st := 3;
            END_IF;
        END_IF;
    END_IF;

    ctl();
END_PROGRAM
