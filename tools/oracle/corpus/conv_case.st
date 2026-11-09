PROGRAM Demo
VAR
    t1 AT %QD0  : DINT;
    ro AT %QD4  : DINT;
    m1 AT %QD8  : DINT;
    m2 AT %QD12 : DINT;
    u1 AT %QD16 : UDINT;
END_VAR
t1 := TRUNC(-3.9);
ro := ROUND(2.5);
m1 := MUX(1, 10, 20, 30);
m2 := MUX(2, 10, 20, 30);
u1 := DINT_TO_UDINT(7);
END_PROGRAM
