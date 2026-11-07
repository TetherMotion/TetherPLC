PROGRAM TonCase
VAR
    start_in AT %IX0.0 : BOOL;
    lamp AT %QX0.0 : BOOL;
    ton1 : TON;
    et_seen : TIME;
END_VAR
ton1(IN := start_in, PT := T#500ms);
lamp := ton1.Q;
et_seen := ton1.ET;
END_PROGRAM
