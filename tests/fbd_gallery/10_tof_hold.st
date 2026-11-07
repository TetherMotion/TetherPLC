PROGRAM P10_TOF_HOLD
VAR
    x AT %IX0.0 : BOOL;
    q AT %QX0.0 : BOOL;
    tof1 : TOF;
END_VAR
tof1(IN := x, PT := T#200ms);
q := tof1.Q;
END_PROGRAM
