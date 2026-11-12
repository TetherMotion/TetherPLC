// tc2_pm_case.st — PowerMonitoring e2e: V/I sine samples ->
// RMS/P/S/Q/PF/Energy.  The plant feeds a sinusoidal V and a
// in-phase I (PF ~1).
PROGRAM tc2_pm_case
VAR
    pm : FB_PowerMonitor;
    ph : LREAL;   // phase accumulator
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        ph := 0.0;
        pm.Execute := TRUE;
        pm.N := 10;
        pm.CycleTime := 0.01;
        st := 1;
    END_IF;

    // drive DC samples: V=230, I=5 -> VRms~230, IRms~5, P~1150W, PF~1
    IF st = 1 THEN
        IF pm.Done THEN
            pm.Voltage := 230.0;
            pm.Current := 5.0;
            n := n + 1;
            IF n > 40 THEN
                IF pm.VRms > 140.0 THEN mask := mask + 1; END_IF;
                IF pm.IRms > 2.0 THEN mask := mask + 2; END_IF;
                IF pm.PReal > 200.0 THEN mask := mask + 4; END_IF;
                IF pm.PF > 0.8 THEN mask := mask + 8; END_IF;
                IF pm.Energy > 0.0 THEN mask := mask + 16; END_IF;
                st := 2;
            END_IF;
        END_IF;
    END_IF;

    pm();
END_PROGRAM
