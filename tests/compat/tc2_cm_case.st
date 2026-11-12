// tc2_cm_case.st — CM e2e: function generator, linear interp,
// cam profile — pure-ST curve/math.
PROGRAM tc2_cm_case
VAR
    fg : FB_FuncGen;
    li : FB_LinInterp;
    cam : FB_CamProfile;
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        fg.Execute := TRUE;
        li.Execute := TRUE;
        cam.Execute := TRUE;
        fg.Shape := 1;            // triangle
        fg.Period := 1.0;
        fg.Amplitude := 2.0;
        fg.Offset := 0.0;
        fg.CycleTime := 0.1;
        li.X := 1.5;
        li.X0 := 0.0; li.Y0 := 0.0;
        li.X1 := 1.0; li.Y1 := 2.0;
        li.X2 := 2.0; li.Y2 := 4.0;
        li.X3 := 3.0; li.Y3 := 8.0;
        cam.MasterEnd := 10.0;
        cam.SlaveAmp := 100.0;
        cam.Master := 5.0;
        st := 1;
    END_IF;

    // funcgen runs — phase advances
    IF st = 1 THEN
        IF fg.Done THEN
            n := n + 1;
            IF n > 8 THEN
                IF fg.Phase > 0.0 THEN mask := mask + 1; END_IF;
                n := 0;
                st := 2;
            END_IF;
        END_IF;
    END_IF;
    // lininterp X=1.5 between (1,2)->(2,4) -> Y=3
    IF st = 2 THEN
        IF li.Done THEN
            IF li.Y > 2.9 AND li.Y < 3.1 THEN mask := mask + 2; END_IF;
            st := 3;
        END_IF;
    END_IF;
    // cam: master 5 of 10 -> slave ~50 of 100
    IF st = 3 THEN
        IF cam.Done THEN
            IF cam.Slave > 49.0 AND cam.Slave < 51.0 THEN
                mask := mask + 4;
            END_IF;
            IF cam.InSync THEN mask := mask + 8; END_IF;
            st := 4;
        END_IF;
    END_IF;
    // funcgen output in range
    IF st = 4 THEN
        IF fg.Done THEN
            IF fg.Output >= 0.0 AND fg.Output <= 2.0 THEN
                mask := mask + 16;
            END_IF;
            st := 5;
        END_IF;
    END_IF;

    fg(); li(); cam();
END_PROGRAM
