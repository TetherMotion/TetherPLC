PROGRAM P05_LD_DATA
VAR
    a AT %IX0.0 : BOOL;
    b AT %IX0.1 : BOOL;
    c AT %IX0.2 : BOOL;
    t AT %MX0.0 : BOOL;
    y AT %QX0.0 : BOOL;
END_VAR
t := a AND b;
y := t OR c;
END_PROGRAM
