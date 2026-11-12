// tc2_weighing_case.st — Weighing e2e: simulated load-cell signal,
// tare, net, stability, dosing coarse->fine->done.
PROGRAM tc2_weighing_case
VAR
    wgh : FB_Weigh;
    cell : LREAL;   // simulated load-cell signal
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        cell := 5.0;                      // empty cell = tare
        wgh.Execute := TRUE;
        wgh.FilterTn := 0.2;
        wgh.CycleTime := 0.1;
        wgh.DoTare := FALSE;              // tare captured in st=1
        wgh.StableBand := 0.05;
        wgh.StableCycles := 5;
        wgh.DoseSetpoint := 10.0;
        wgh.CoarseLimit := 0.8;
        st := 1;
    END_IF;

    // tare captured -> Net ~0
    IF st = 1 THEN
        IF wgh.Done THEN
            n := n + 1;
            cell := 5.0;                  // still empty
            wgh.Raw := cell;
            wgh.DoTare := TRUE;           // keep capturing tare
            IF n > 5 THEN
                IF wgh.TareWeight > 4.0 THEN mask := mask + 1; END_IF;
                IF wgh.Net < 1.0 THEN mask := mask + 2; END_IF;
                n := 0;
                wgh.DoTare := FALSE;
                st := 2;
            END_IF;
        END_IF;
    END_IF;
    // fill: cell signal ramps up
    IF st = 2 THEN
        IF wgh.Done THEN
            n := n + 1;
            cell := cell + 0.3;           // product filling
            wgh.Raw := cell;
            IF n > 30 THEN
                IF wgh.Net > 3.0 THEN mask := mask + 4; END_IF;
                n := 0;
                st := 3;
            END_IF;
        END_IF;
    END_IF;
    // dosing coarse->fine->done as Net approaches Setpoint
    IF st = 3 THEN
        IF wgh.Done THEN
            n := n + 1;
            cell := cell + 0.3;
            wgh.Raw := cell;
            IF wgh.DoseDone THEN
                mask := mask + 8;
                n := 0;
                st := 4;
            END_IF;
            IF n > 80 THEN st := 4; END_IF;
        END_IF;
    END_IF;
    // hold — verify stable flag settles
    IF st = 4 THEN
        IF wgh.Done THEN
            n := n + 1;
            IF n > 20 THEN
                IF wgh.Stable THEN mask := mask + 16; END_IF;
                st := 5;
            END_IF;
        END_IF;
    END_IF;

    wgh();
END_PROGRAM
