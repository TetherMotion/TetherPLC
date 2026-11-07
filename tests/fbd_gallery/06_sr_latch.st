PROGRAM P06_SR_LATCH
VAR
    s AT %IX0.0 : BOOL;
    r AT %IX0.1 : BOOL;
    q AT %QX0.0 : BOOL;
    sr1 : SR;
END_VAR
sr1(S1 := s, R := r);
q := sr1.Q1;
END_PROGRAM
