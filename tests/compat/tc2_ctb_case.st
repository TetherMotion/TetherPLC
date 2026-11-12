// tc2_ctb_case.st — Tc2_ControllerToolbox e2e: hysteresis, two-
// position, deadtime, statistics, PWM — pure-ST blocks.
PROGRAM tc2_ctb_case
VAR
    hys : FB_Hysteresis;
    two : FB_TwoPosition;
    dt  : FB_DeadTime;
    stt : FB_Statistics;
    pwm : FB_Pwm;
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        hys.Execute := TRUE;
        two.Execute := TRUE;
        dt.Execute := TRUE;
        stt.Execute := TRUE;
        pwm.Execute := TRUE;
        hys.Input := 0.5; hys.OnLevel := 0.3; hys.OffLevel := 0.8;
        two.Setpoint := 1.0; two.Actual := 0.0; two.Hysteresis := 0.2;
        dt.Input := 7.0; dt.DelayCycles := 3;
        stt.Input := 2.0; stt.N := 4;
        pwm.Duty := 50.0; pwm.Period := 1.0; pwm.CycleTime := 0.1;
        st := 1;
    END_IF;

    // hysteresis: 0.5 is between 0.3..0.8 — stays off (started off)
    IF st = 1 THEN
        IF hys.Done THEN
            IF hys.Out = FALSE THEN mask := mask + 1; END_IF;
            st := 2;
        END_IF;
    END_IF;
    // two-position: Actual 0 < SP 1 - hyst -> drive on
    IF st = 2 THEN
        IF two.Done THEN
            IF two.Out = TRUE THEN mask := mask + 2; END_IF;
            st := 3;
        END_IF;
    END_IF;
    // deadtime: after DelayCycles the output lags input
    IF st = 3 THEN
        IF dt.Done THEN
            n := n + 1;
            IF n > 6 THEN
                IF dt.Output = 7.0 THEN mask := mask + 4; END_IF;
                n := 0;
                st := 4;
            END_IF;
        END_IF;
    END_IF;
    // statistics: mean of constant input ~= input, stddev ~0
    IF st = 4 THEN
        IF stt.Done THEN
            n := n + 1;
            IF n > 8 THEN
                IF stt.Mean > 1.9 AND stt.StdDev < 0.2 THEN
                    mask := mask + 8;
                END_IF;
                n := 0;
                st := 5;
            END_IF;
        END_IF;
    END_IF;
    // pwm: 50% duty -> Out toggles; just verify it ran
    IF st = 5 THEN
        IF pwm.Done THEN
            mask := mask + 16;
            st := 6;
        END_IF;
    END_IF;

    hys(); two(); dt(); stt(); pwm();
END_PROGRAM
