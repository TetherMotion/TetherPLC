PROGRAM Demo
VAR
    a  : STRING[16];
    b  : STRING[16];
    eq1 AT %QX0.0 : BOOL;
    eq2 AT %QX0.1 : BOOL;
    eq3 AT %QX0.2 : BOOL;
    eq4 AT %QX0.3 : BOOL;
    eq5 AT %QX0.4 : BOOL;
END_VAR
a := 'abc';
b := 'abd';
eq1 := (a = b);
eq2 := (a = 'abc');
eq3 := (a <> b);
eq4 := ('ab' = 'ab');
eq5 := ('ab' = 'abc');
END_PROGRAM
