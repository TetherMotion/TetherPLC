PROGRAM RTrigCase
VAR
    sig  AT %IX0.0 : BOOL;
    rise AT %QX0.0 : BOOL;
    fall AT %QX0.1 : BOOL;
    rt1  : R_TRIG;
    ft1  : F_TRIG;
END_VAR
rt1(CLK := sig);
ft1(CLK := sig);
rise := rt1.Q;
fall := ft1.Q;
END_PROGRAM
