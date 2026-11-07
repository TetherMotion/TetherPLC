PROGRAM P01_TON_DEBOUNCE
VAR
    start_in AT %IX0.0 : BOOL;
    stop_in AT %IX0.1 : BOOL;
    run AT %QX0.0 : BOOL;
    ton1 : TON;
END_VAR
ton1(IN := start_in, PT := T#200ms);
run := ton1.Q AND NOT stop_in;
END_PROGRAM
