PROGRAM P07_TWO_NETS
VAR
    a AT %IX0.0 : BOOL;
    b AT %IX0.1 : BOOL;
    q AT %QX0.0 : BOOL;
    gate : BOOL;
END_VAR
gate := a AND b;
q := gate;
END_PROGRAM
