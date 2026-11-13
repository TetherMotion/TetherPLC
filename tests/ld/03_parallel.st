PROGRAM LD03_PARALLEL
VAR
    start_in AT %IX0.0 : BOOL;
    hold_in  AT %IX0.1 : BOOL;
    stop_in  AT %IX0.2 : BOOL;
    run_out  AT %QX0.0 : BOOL;
END_VAR
run_out := (start_in OR hold_in) AND NOT stop_in;
END_PROGRAM
