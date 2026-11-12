// tc2_filter_case.st — Tc2_Filter surface e2e: PT1 step response,
// PID tracking, limit clamp, average, ramp — pure-ST blocks.
PROGRAM tc2_filter_case
VAR
    lim : FB_Limit;
    pt1 : FB_PT1;
    pid : FB_PID;
    avg : FB_Average;
    rmp : FB_Ramp;
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        n := 0;
        lim.Execute := TRUE;
        pt1.Execute := TRUE;
        pid.Execute := TRUE;
        avg.Execute := TRUE;
        rmp.Execute := TRUE;
        pt1.Input := 1.0;   pt1.Kp := 1.0;  pt1.Tn := 0.5;  pt1.CycleTime := 0.1;
        pid.Setpoint := 1.0; pid.Actual := 0.0;
        pid.Kp := 2.0; pid.Tn := 0.5; pid.Tv := 0.0;
        pid.OutMin := -10.0; pid.OutMax := 10.0; pid.CycleTime := 0.1;
        avg.Input := 2.0; avg.N := 4;
        rmp.Setpoint := 3.0; rmp.SlewRate := 2.0; rmp.CycleTime := 0.1;
        lim.Input := 5.0; lim.Low := 0.0; lim.High := 3.0;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF lim.Done THEN
            IF lim.Output = 3.0 THEN mask := mask + 1; END_IF; // clamped
            st := 2;
        END_IF;
    END_IF;
    IF st = 2 THEN
        IF pt1.Done THEN
            n := n + 1;
            IF n > 20 THEN
                IF pt1.Output > 0.8 THEN mask := mask + 2; END_IF; // settles ~1
                n := 0;
                st := 3;
            END_IF;
        END_IF;
    END_IF;
    IF st = 3 THEN
        IF pid.Done THEN
            n := n + 1;
            IF n > 20 THEN
                IF pid.Output > 0.0 THEN mask := mask + 4; END_IF; // drives +
                n := 0;
                st := 4;
            END_IF;
        END_IF;
    END_IF;
    IF st = 4 THEN
        IF avg.Done THEN
            n := n + 1;
            IF n > 6 THEN
                IF avg.Output > 1.9 THEN mask := mask + 8; END_IF; // ~2.0
                n := 0;
                st := 5;
            END_IF;
        END_IF;
    END_IF;
    IF st = 5 THEN
        IF rmp.Done THEN
            n := n + 1;
            IF n > 40 THEN
                IF rmp.Output > 2.0 THEN mask := mask + 16; END_IF; // ramps
                n := 0;
                st := 6;
            END_IF;
        END_IF;
    END_IF;

    lim(); pt1(); pid(); avg(); rmp();
END_PROGRAM
