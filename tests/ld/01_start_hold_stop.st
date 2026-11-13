PROGRAM LD01_START_HOLD_STOP
VAR
    start_in AT %IX0.0 : BOOL;
    hold_in  AT %IX0.1 : BOOL;
    stop_in  AT %IX0.2 : BOOL;
    run_out  AT %QX0.0 : BOOL;
    lamp_out AT %QX0.1 : BOOL;
END_VAR
run_out := start_in AND hold_in AND NOT stop_in;
lamp_out := run_out;
END_PROGRAM
