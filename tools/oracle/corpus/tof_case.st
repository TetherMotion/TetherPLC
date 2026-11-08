PROGRAM TofCase
VAR
    sig  AT %IX0.0 : BOOL;
    outp AT %QX0.0 : BOOL;
    tof1 : TOF;
END_VAR
tof1(IN := sig, PT := T#500ms);
outp := tof1.Q;
END_PROGRAM
