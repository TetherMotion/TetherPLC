PROGRAM LD02_MULTI_COIL
VAR
    a_in  AT %IX0.0 : BOOL;
    b_in  AT %IX0.1 : BOOL;
    c_out AT %QX0.0 : BOOL;
    d_out AT %QX0.1 : BOOL;
    e_out AT %QX0.2 : BOOL;
END_VAR
c_out := a_in AND b_in;
d_out := a_in AND b_in;
e_out := NOT a_in;
END_PROGRAM
