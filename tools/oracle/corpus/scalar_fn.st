PROGRAM Demo
VAR
    a AT %QD0  : DINT;
    b AT %QD4  : DINT;
    c AT %QD8  : DINT;
    d AT %QD12 : DINT;
    e AT %QD16 : DINT;
    f AT %QD20 : DINT;
END_VAR
a := ABS(-5);
b := MIN(3, 7);
c := MAX(3, 7);
d := SEL(FALSE, 11, 22);
e := LIMIT(10, 42, 20);
f := ABS(4 - 9);
END_PROGRAM
