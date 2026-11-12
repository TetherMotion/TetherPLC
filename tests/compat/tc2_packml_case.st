// tc2_packml_case.st — PackML state machine e2e: Idle -> Start ->
// Execute -> Hold -> Held -> Unhold -> Execute -> Abort -> Aborted
// -> Clear -> Idle.  Command constants per tc2_packml.st header.
PROGRAM tc2_packml_case
VAR
    pml : FB_PackML;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        pml.Execute := TRUE;
        pml.Busy := FALSE;
        pml.Command := 0;
        st := 1;
    END_IF;

    // Idle -> Start -> Execute
    IF st = 1 THEN
        IF pml.State = 1 THEN              // Idle
            mask := mask + 1;
            pml.Command := 1;              // Start
            st := 2;
        END_IF;
    END_IF;
    IF st = 2 THEN
        IF pml.State = 3 THEN              // Execute
            mask := mask + 2;
            pml.Command := 4;              // Hold
            st := 3;
        END_IF;
    END_IF;
    // Execute -> Hold -> Held -> Unhold -> Execute
    IF st = 3 THEN
        IF pml.State = 8 THEN              // Held
            mask := mask + 4;
            pml.Command := 5;              // Unhold
            st := 4;
        END_IF;
    END_IF;
    IF st = 4 THEN
        IF pml.State = 3 THEN              // Execute again
            mask := mask + 8;
            pml.Command := 8;              // Abort
            st := 5;
        END_IF;
    END_IF;
    // Execute -> Abort -> Aborted -> Clear -> Idle
    IF st = 5 THEN
        IF pml.State = 14 THEN             // Aborted
            mask := mask + 16;
            pml.Command := 9;              // Clear
            st := 6;
        END_IF;
    END_IF;
    IF st = 6 THEN
        IF pml.State = 1 THEN              // back to Idle
            mask := mask + 32;
            pml.Command := 0;
            st := 7;
        END_IF;
    END_IF;

    pml();
END_PROGRAM
