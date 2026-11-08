PROGRAM SrRsCase
VAR
    a   AT %IX0.0 : BOOL;
    b   AT %IX0.1 : BOOL;
    qsr AT %QX0.0 : BOOL;
    qrs AT %QX0.1 : BOOL;
    sr1 : SR;
    rs1 : RS;
END_VAR
sr1(S1 := a, R := b);
rs1(S := a, R1 := b);
qsr := sr1.Q1;
qrs := rs1.Q1;
END_PROGRAM
